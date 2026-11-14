//! Desk work in World, including a real disk edit while the app stays open.
use super::*;
use crate::design;
use crate::world::edit::ALREADY_SAVED;
use egui::accesskit::Role;
use std::fs;
use uuid::Uuid;

/// A free-text History date longer than a third of the reader, short enough for the list.
const LONG_WHEN: &str = "In the winter before the founding";
const LONG_SUMMARY: &str = "The guild held the Arena through the winter siege, and fed the miners' families from its own stores until the thaw came and the soldiers went home.";

fn root(d: &Driver<'_>) -> std::path::PathBuf {
    d.root.join("campaigns/emberfall")
}

fn entry(d: &Driver<'_>, path: &str, name: &str, kind: &str, fields: &str) -> Uuid {
    let id = splat::character::stable_id(path);
    let path = root(d).join(path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, splat::content::formats::fixture("World.md", &format!("splat_id: {id}\nsplat_kind: world\nsplat_name: {name}\nsplat_world:\n  type: {kind}\n{fields}"), &format!("\n# {name}\n\nA note for tonight.\n")).unwrap()).unwrap();
    id
}

fn reload(d: &mut Driver<'_>) {
    d.app_mut().lookup.refresh();
    d.settle();
}

fn reveal_editor_passage(d: &mut Driver<'_>, words: &str) {
    d.scroll_up_at(READER, 200);
    for _ in 0..200 {
        if d.visible_passage(words) {
            return;
        }
        d.scroll_at(READER, 1);
    }
    panic!("The editor did not show {words}");
}

pub fn broken(d: &mut Driver<'_>) {
    let path = root(d).join("Notes/Broken.md");
    fs::write(&path, "---\nsplat_schema: 2\nsplat_id: [\n---\n").unwrap();
    reload(d);
    d.click(NAV_WORLD);
    d.expect("1 note cannot be read");
    d.observe(
        "W5",
        d.visible_text("4 notes"),
        "The World total includes the note that cannot be read",
    );
    for size in [SMALL, DEFAULT, LARGE] {
        let cause = d.at_size(size, |d| {
            d.expect("Notes/Broken.md · Line 3 opens a list with [ that never closes.");
            let row = d
                .place_of("Notes/Broken.md · Line 3 opens a list with [ that never closes.")
                .unwrap();
            d.painted_in(row).contains("Line 3") && !d.painted_in(row).contains("Cannot be read")
        });
        d.observe(
            "iter-7-D-FD9",
            cause,
            format!("The unreadable row names the cause at {size:?}"),
        );
        let heading_gap = d.at_size(size, |d| {
            d.expect("CANNOT BE READ 1");
            d.expect("Powers/");
            let nodes = d.nodes();
            let unreadable = nodes
                .iter()
                .find(|node| node.role == Role::Label && node.name == "CANNOT BE READ 1")
                .unwrap()
                .rect;
            let folder = nodes
                .iter()
                .find(|node| node.role == Role::Label && node.name.contains("Powers/"))
                .unwrap()
                .rect;
            let gap = |heading: egui::Rect| {
                let previous = nodes
                    .iter()
                    .filter(|node| {
                        node.role == Role::Button
                            && (node.rect.left() - heading.left()).abs() < 1.0
                            && node.rect.bottom() <= heading.top()
                    })
                    .max_by(|a, b| a.rect.bottom().total_cmp(&b.rect.bottom()))
                    .expect("A heading has a preceding row");
                heading.top() - previous.rect.bottom()
            };
            (gap(unreadable) - gap(folder)).abs() < 1.0
        });
        d.observe(
            "iter-7-D-FD9-heading",
            heading_gap,
            format!("The unreadable group uses the folder heading gap at {size:?}"),
        );
    }
    // Round 27 FH3, FF4: the wrapped cause starts its second line at its first word, right of
    // the error icon, not under it.
    for size in [SMALL, DEFAULT, LARGE] {
        let (icon, words) = d.at_size(size, |d| {
            let words = d.painted_rect_in(
                "Line 3 opens a list with [ that never closes.",
                egui::Rect::EVERYTHING,
            );
            // The icon on the cause's first line, just left of it.
            let icon = words.and_then(|words| {
                d.painted_rect_in(
                    design::icons::ERROR,
                    egui::Rect::from_min_max(
                        egui::pos2(words.left() - design::XL, words.top() - design::XS),
                        egui::pos2(words.left() + design::XS, words.top() + design::XL),
                    ),
                )
            });
            (icon, words)
        });
        d.observe(
            "iter-27-D-FH3",
            icon.zip(words)
                .is_some_and(|(icon, words)| words.left() >= icon.right() - 0.5),
            format!(
                "At {size:?}, the cause {words:?} starts right of the icon {icon:?} on every line"
            ),
        );
    }
    d.state("broken-note-listed", Sizes::All);
    d.click("1 note cannot be read");
    d.expect("Save repaired note");
    // The row, not the editor, which now names the file too.
    d.click_starting("Notes/Broken.md · ");
    d.expect("Save repaired note");
    d.observe(
        "W5",
        d.visible_text("Line 3 opens a list") && d.has("Edit in Obsidian"),
        "The broken note opens the shared repair editor and its file actions",
    );
    // Round 24 AC3, FC3, FD2, SC2: Save's check refuses the unchanged text, so the save row says
    // Invalid with the error icon, as Campaign's does; `No changes` is for a readable draft.
    d.observe(
        "iter-24-D-FD2",
        d.visible_text(&format!("{} Invalid", design::icons::ERROR))
            && !d.visible_text("No changes")
            && d.is_disabled("Save repaired note"),
        "The unchanged unreadable repair says Invalid, not No changes",
    );
    // iter-27-A FE2: the 4-line repair's footer follows its save row, `S` below it, instead of
    // the pane's bottom; a footer that cuts no body has no hairline at its top.
    d.state_prepared("broken-note-repair", Sizes::All, |d| {
        repair_footer_follows(d, "Line 3 opens a list", "iter-27-A-FE2");
    });
    let external = "---\nsplat_schema: 2\nsplat_id: [\n---\n\nOutside edit kept.\n";
    fs::write(&path, external).unwrap();
    d.settle();
    assert_eq!(d.editor_text().as_deref(), Some(external));
    d.state("clean-repair-follows-disk", Sizes::All);
    let unfinished = format!("{external}\nMy unfinished draft.\n");
    let outside = format!("{external}\nNew words on disk.\n");
    d.replace_text_in("Note text", &unfinished);
    d.observe(
        "iter-8-D-FD2-repair",
        d.visible_text("World repair draft") && !d.visible_text("World draft"),
        "A repair-only draft has one repair rail status",
    );
    d.click("1 note cannot be read");
    assert_eq!(d.editor_text().as_deref(), Some(unfinished.as_str()));
    fs::write(&path, &outside).unwrap();
    d.settle();
    d.click("Overwrite with my draft");
    assert_eq!(fs::read_to_string(&path).unwrap(), outside);
    assert_eq!(d.editor_text().as_deref(), Some(unfinished.as_str()));
    // Round 25 FD1: the repair editor's top at each size, in every message state below.
    let editor_tops = |d: &mut Driver<'_>| {
        [SMALL, DEFAULT, LARGE].map(|size| {
            d.at_size(size, |d| {
                d.place_of(&crate::world::repair::editor_name("Notes/Broken.md"))
                    .map(|editor| editor.top())
            })
        })
    };
    let mut tops = vec![("invalid draft, reason", editor_tops(d))];
    // Round 24 SA1, FA1: the disk did not fail, so the message is the cause alone; `Not saved.`
    // is only for a disk failure.
    let combined = "Line 3 opens a list with [ that never closes.";
    d.observe(
        "iter-25-D-FB6",
        d.is_disabled("Save repaired note")
            && d.visible_text(crate::encounters::blocked_draft_switch_reason())
            && !d.has(crate::encounters::draft_switch_reason()),
        "Save is blocked, so the switch lock says `Discard to switch campaign.`",
    );
    for size in [SMALL, DEFAULT, LARGE] {
        let (one_message, ordered, (state_word, within)) = d.at_size(size, |d| {
            let errors: Vec<_> = d
                .nodes()
                .into_iter()
                .filter(|node| {
                    node.role == Role::Label && node.name.contains("opens a list with [")
                })
                .collect();
            let one_message = !errors.is_empty()
                && errors.len() == 1
                && errors
                    .iter()
                    .all(|node| node.name == combined && node.in_view())
                && !d.has_containing("Not saved")
                && !d.has("Not saved: fix the problem above, then save.");
            // Round 23 FF2: a changed draft that still cannot be read is Invalid, as in Library.
            let state = d.place_of(&format!("{} Invalid", design::icons::ERROR));
            let state_word =
                state.is_some() && !d.has(&format!("{} Unsaved", design::icons::DRAFT));
            let discard = d.place_of("Discard").unwrap();
            // Round 25 FB6: Save refuses this draft, so the lock names Discard only.
            let reason = d
                .place_of(crate::encounters::blocked_draft_switch_reason())
                .unwrap();
            let save = d.place_of("Save repaired note").unwrap();
            // Round 23 AF1: the editor and its save row end together within `LIST_MEASURE`.
            let editor = d
                .place_of(&crate::world::repair::editor_name("Notes/Broken.md"))
                .unwrap();
            (
                one_message,
                state.is_some_and(|state| state.right() <= discard.left())
                    && discard.right() <= reason.left()
                    && reason.right() <= save.left()
                    && reason.top() < save.bottom()
                    && reason.bottom() > save.top(),
                (
                    state_word,
                    (
                        save.right() - editor.left() <= design::LIST_MEASURE + 1.0,
                        format!("editor {editor:?}, Save {save:?}"),
                    ),
                ),
            )
        });
        d.observe(
            "iter-23-D-FF2-world",
            state_word,
            format!("The invalid changed repair says Invalid, not Unsaved, at {size:?}"),
        );
        d.observe(
            "iter-23-D-AF1",
            within.0,
            format!(
                "The repair editor and Save end within the list measure at {size:?}: {}",
                within.1
            ),
        );
        d.observe(
            "iter-5-D-FD9",
            one_message,
            format!("One failed-save message at {size:?}"),
        );
        d.observe(
            "iter-5-D-FD2",
            ordered,
            format!("State, Discard, reason, then Save at {size:?}"),
        );
    }
    d.state("invalid-conflict-draft-kept-for-repair", Sizes::All);
    d.click(&crate::world::repair::editor_name("Notes/Broken.md"));
    for size in [SMALL, LARGE, SMALL, DEFAULT] {
        d.set_base(size);
        d.observe(
            "iter-5-D-draft-resize",
            d.editor_text().as_deref() == Some(unfinished.as_str())
                && d.focused_name() == Some(crate::world::repair::editor_name("Notes/Broken.md"))
                && d.visible_passage("My unfinished draft."),
            format!("The same unsent draft, focus and visible passage survive resize to {size:?}"),
        );
    }
    let stale_before = d.has_containing("My unfinished draft.");
    d.click("Discard");
    assert_eq!(d.editor_text().as_deref(), Some(outside.as_str()));
    // Round 24 FD2, SC2: Discard leaves the unchanged file text, which still cannot be read. Save
    // refuses it, so the state is Invalid, never a check-marked `No changes`.
    d.observe(
        "iter-24-D-FD2-discard",
        stale_before
            && d.visible_text(&format!("{} Invalid", design::icons::ERROR))
            && !d.visible_text("No changes")
            && d.is_disabled("Save repaired note"),
        "After Discard the unchanged unreadable draft says Invalid, and Save is blocked",
    );
    // Round 25 FD1, lead's ruling: a result with a live Undo comes first, so after Discard of a
    // broken repair `Undo discard` leads and Save's reason follows it, at every size. Round 26
    // FA1: both are in the pane's footer, below Save, and the reason shows whole (it was cut at
    // half height in the two-line space).
    let undo_first = |d: &mut Driver<'_>, undo_name: &str| {
        for size in [SMALL, DEFAULT, LARGE] {
            let (undo, reason, save, whole) = d.at_size(size, |d| {
                (
                    d.place_of(undo_name),
                    d.nodes()
                        .into_iter()
                        .find(|node| node.role == Role::Label && node.name == combined)
                        .map(|node| node.rect),
                    d.place_of("Save repaired note"),
                    // The reason shows whole in the footer, not cut by it.
                    d.nodes().iter().any(|node| {
                        node.role == Role::Label && node.name == combined && node.fully_in_view()
                    }),
                )
            });
            d.observe(
                "iter-25-D-FD1-undo-first",
                undo.zip(reason).zip(save).is_some_and(|((undo, reason), save)| {
                    undo.top() >= save.bottom()
                        && reason.top() >= undo.center().y
                        && reason.bottom() <= size.1
                        && whole
                }),
                format!("At {size:?}, {undo_name} {undo:?} leads, then the reason {reason:?}, below Save {save:?}"),
            );
        }
    };
    undo_first(d, "Undo discard of Broken.md");
    tops.push(("discarded, Undo and reason", editor_tops(d)));
    // Round 25 FD1 review: the Undo row and the reason overflow the two-line part. Tab reaches
    // the part by its whole message, the keyboard and the wheel scroll it, its focus ring keeps
    // clear of its scroll bar at every size, and none of it moves the editor.
    let part_reachable = |d: &mut Driver<'_>, message: &str, state: &str| {
        let before = editor_tops(d);
        // Re-check 1: the gestures themselves run at each size, with the window at that size.
        for size in [SMALL, DEFAULT, LARGE] {
            d.set_base(size);
            // Round 26: the footer is as tall as what it holds, so the message is reached,
            // with its Undo, whether or not it needs to scroll.
            let (tabbed, _, _, ring) = fixed_part_scrolls(d, message);
            let owner = d.focused_name();
            // The discard's Undo is named for its draft.
            let undo = if state == "Undo discard" {
                "Undo discard of Broken.md"
            } else {
                state
            };
            // Each message is reached on its own: the reason's words, with the result's Undo.
            let reached = space_reaches(d, combined, Some(undo));
            let (keyed, wheeled) = (
                reached.iter().all(|(keyboard, _)| *keyboard),
                reached.iter().all(|(_, pointer)| *pointer),
            );
            d.observe(
                "iter-25-D-FD1-part-focus",
                tabbed && owner.as_deref() == Some(message),
                format!(
                    "{state} at {size:?}: Tab reaches the part named {message:?}; focus is on {owner:?}"
                ),
            );
            d.observe(
                "iter-25-D-FD1-part-scroll",
                keyed && wheeled,
                format!(
                    "{state} at {size:?}: the keys reach the part {keyed}, the wheel reaches it {wheeled}"
                ),
            );
            d.observe(
                "iter-25-D-FD1-part-ring",
                ring.iter().all(|clear| *clear),
                format!(
                    "{state} at {size:?}: the focus ring keeps clear of the scroll bar at 960, 1200, 1920: {ring:?}"
                ),
            );
            // Back to the part's top, so its Undo is in view again.
            d.press(Key::PageUp);
        }
        d.set_base(DEFAULT);
        let after = editor_tops(d);
        d.observe(
            "iter-25-D-FD1-part-editor",
            before.iter().all(Option::is_some) && before == after,
            format!("{state}: the editor stays at {before:?} → {after:?}"),
        );
    };
    part_reachable(
        d,
        &format!("Broken.md draft discarded. {combined}"),
        "Undo discard",
    );
    d.click("Undo discard of Broken.md");
    assert_eq!(d.editor_text().as_deref(), Some(unfinished.as_str()));
    tops.push(("Undo discard, reason", editor_tops(d)));
    // The discard result's owner is the discarded draft: an edit ends it, and the reason leads
    // again. The edit types the same words back, so the draft is the one before Discard.
    d.click("Discard");
    d.expect("Undo discard of Broken.md");
    d.replace_text_in("Note text", &unfinished);
    d.observe(
        "iter-25-D-FD1-owner",
        !d.has("Undo discard of Broken.md")
            && d.nodes()
                .into_iter()
                .any(|node| node.role == Role::Label && node.name == combined && node.in_view()),
        "An edit after Discard ends Undo discard; Save's reason leads the message space",
    );
    let fixed = format!(
        "---\nsplat_schema: 2\nsplat_id: {}\nsplat_kind: world\nsplat_name: Repaired watchkeeper\nsplat_type: npc\n---\n\nThe watchkeeper is back.\n",
        splat::character::stable_id("repaired-watchkeeper")
    );
    d.replace_text_in("Note text", &fixed);
    d.expect(&format!("{} Unsaved", design::icons::DRAFT));
    tops.push(("valid draft, no message", editor_tops(d)));
    fs::write(&path, format!("{external}\nAnother outside edit.\n")).unwrap();
    d.settle();
    d.expect("This note changed on disk. Your draft is safe until you choose.");
    d.state_prepared("dirty-repair-compares-disk", Sizes::All, |d| {
        choices_at_text(
            d,
            "iter-15-E-AD3-world-repair",
            "This note changed on disk",
            "Overwrite with my draft",
            "Reload and lose my draft",
            Some("YOUR DRAFT"),
        );
    });
    d.click("Reload and lose my draft");
    d.expect("Undo reload");
    assert_eq!(
        d.editor_text().as_deref(),
        Some(format!("{external}\nAnother outside edit.\n").as_str())
    );
    assert_eq!(
        fs::read_to_string(&path).unwrap(),
        format!("{external}\nAnother outside edit.\n")
    );
    undo_first(d, "Undo reload");
    tops.push(("reloaded, Undo and reason", editor_tops(d)));
    part_reachable(
        d,
        &format!("Reloaded from disk. Your draft is kept until you edit. {combined}"),
        "Undo reload",
    );
    // iter-27-A FA2: with Undo reload and the reason the footer still follows the save row.
    d.state_prepared("repair-reloaded-with-undo", Sizes::All, |d| {
        repair_footer_follows(d, "Reloaded from disk.", "iter-27-A-FA2");
    });
    d.click("Undo reload");
    assert_eq!(d.editor_text().as_deref(), Some(fixed.as_str()));
    tops.push(("reload undone, no message", editor_tops(d)));
    // Round 25 FD1: no reason, result or Undo moves the editor, at any size.
    let first = tops[0].1;
    d.observe(
        "iter-25-D-FD1",
        first.iter().all(Option::is_some)
            && tops.iter().all(|(_, at)| {
                at.iter()
                    .zip(&first)
                    .all(|(at, first)| at.zip(*first).is_some_and(|(a, f)| (a - f).abs() <= 0.5))
            }),
        format!("The repair editor's top in each state (960, 1200, 1920): {tops:?}"),
    );
    d.state("repair-reload-undone", Sizes::All);
    fs::write(&path, format!("{external}\nA later outside edit.\n")).unwrap();
    d.settle();
    d.click("Reload and lose my draft");
    d.replace_text_in("Note text", &fixed);
    d.observe(
        "iter-1-A-reload-next-edit",
        !d.has("Undo reload") && d.editor_text().as_deref() == Some(fixed.as_str()),
        "A new edit ends Undo reload and keeps the newly typed draft",
    );
    // Round 25 FD1: the editor no longer moves when the message below it ends, so it keeps the
    // keyboard after the edit (it lost it before). Escape gives the keyboard back, as after Undo
    // reload, so the two states compare without the focus ring.
    d.press(Key::Escape);
    d.expect_same("Editing after reload restores the same draft as Undo reload; the recovery control is gone in both states.");
    d.assert_state_matches("repair-edited-after-reload", "repair-reload-undone");
    d.observe_both(
        "iter-18-A-world-lock",
        "A World repair states the campaign switch lock",
        |d| d.visible_text(crate::encounters::draft_switch_reason()),
    );
    d.click("Setting · ");
    d.click_starting("Notes/Broken.md · ");
    d.observe(
        "W5",
        d.editor_text().as_deref() == Some(&fixed),
        "Reading another note keeps the repair draft",
    );
    // A second disk edit lets the DM save the draft in one explicit choice.
    fs::write(&path, format!("{external}\nOne more outside edit.\n")).unwrap();
    d.settle();
    d.click("Overwrite with my draft");
    d.settle();
    d.observe(
        "W5",
        !d.has("1 note cannot be read") && fs::read_to_string(&path).unwrap() == fixed,
        "Saving repairs the note and removes the warning",
    );
    d.wait_for_text("Note repaired.");
    d.expect("The watchkeeper is back.");
    // FD2: `Note repaired.` is a success message below the file line, so the title does not
    // move. Another note's title shows where a title sits with no result.
    let (below, detail) = result_below_file_line(
        d,
        "Note repaired.",
        "Notes/Broken.md",
        design::colors().result.success,
    );
    let repaired = title_rect(d, "Repaired watchkeeper");
    // Round 24 FD5: two safe choices start at the warning's words, `L` apart.
    let choices = d
        .place_of("Rename the note to Broken")
        .zip(d.place_of("Rename the file to Repaired watchkeeper.md"));
    d.observe(
        "iter-24-D-FD5-choices",
        choices.is_some_and(|(note, file)| {
            (note.top() - file.top()).abs() < 0.5
                && (file.left() - note.right() - design::L).abs() <= 0.5
        }),
        format!("The rename choices are {choices:?}"),
    );
    d.state("broken-note-repaired", Sizes::Default);
    d.click("Setting · ");
    d.settle();
    let plain = title_rect(d, "Setting");
    d.observe("iter-16-D-FD2-repaired", below, detail);
    d.observe(
        "iter-16-D-FD2-repaired-title",
        repaired
            .zip(plain)
            .is_some_and(|(repaired, plain)| (repaired.top() - plain.top()).abs() <= 0.5),
        format!("The repaired title at {repaired:?}, another note's title at {plain:?}"),
    );
    d.click_starting("Repaired watchkeeper · ");
    d.expect("Note repaired.");
    // FD1: an edit-form save result takes the same place on the same note. It replaces
    // `Note repaired.`, the older result.
    // Round 26 FG1: the value rows keep their place from `Note repaired.` to `Changes saved.`
    // and its Undo; both results sit in the footer.
    let rows = tops_at_sizes(d, "Change Stat block");
    d.click("Edit Repaired watchkeeper");
    d.type_into("Status", "on watch");
    d.click("Save changes");
    d.wait_for_text("Changes saved.");
    // FD5 (round 20): a finished change has the success tone, as `Note repaired.` has.
    let (below, detail) = result_below_file_line(
        d,
        "Changes saved.",
        "Notes/Broken.md",
        design::colors().result.success,
    );
    d.observe(
        "iter-17-D-FD1-same-place",
        below && !d.visible_text("Note repaired."),
        format!("`Changes saved.` replaces `Note repaired.`; {detail}"),
    );
    body_kept(
        d,
        "iter-26-D-FG1",
        "Change Stat block",
        &rows,
        Some("Undo changes"),
    );
    d.state("repaired-note-saved", Sizes::Default);
    d.click("Undo changes");
    d.observe(
        "iter-17-D-FD1-undo",
        fs::read_to_string(&path).unwrap() == fixed && !d.has("Undo changes"),
        "Undo changes restores the repaired note",
    );
    // An edit's Undo from before the note broke does not hide the later repair's result. The
    // note breaks while Problems shows, so the reader does not see the file change, and the
    // note leaves the index with its Undo still kept.
    d.click("Edit Repaired watchkeeper");
    d.type_into("Status", "on watch");
    d.click("Save changes");
    d.wait_for_text("Changes saved.");
    d.click_containing("Things to check · ");
    fs::write(&path, "---\nsplat_schema: 2\nsplat_id: [\n---\n").unwrap();
    reload(d);
    d.click("1 note cannot be read");
    d.click_starting("Notes/Broken.md · ");
    d.expect("Save repaired note");
    let clean = super::session_recording::accent_filled(d);
    d.replace_text_in("Note text", &fixed);
    // Round 22 FD3: one weight for a save row. The changed repair's save is the screen's one
    // primary action; unchanged, it is not primary.
    let changed = super::session_recording::accent_filled(d);
    d.observe(
        "iter-22-C-FD3-world",
        !clean.iter().any(|name| name == "Save repaired note") && changed == ["Save repaired note"],
        format!("Accent-filled unchanged {clean:?}, changed {changed:?}"),
    );
    d.click("Save repaired note");
    d.settle();
    d.observe(
        "iter-17-D-repaired-after-edit",
        fs::read_to_string(&path).unwrap() == fixed
            && d.visible_text("Note repaired.")
            && !d.has("Undo changes"),
        "A repair after an edit shows Note repaired., and the old edit's Undo is gone",
    );
    // Another note's rename Undo survives this repair.
    let warden = root(d).join("NPCs/Gate Warden.md");
    entry(d, "NPCs/Gate Warden.md", "Old Warden", "npc", "");
    reload(d);
    d.click_containing("Old Warden · ");
    d.click("Rename the note to Gate Warden");
    d.wait_for_text("Undo name change");
    let renamed = fs::read_to_string(&warden).unwrap();
    d.click_containing("Things to check · ");
    fs::write(&path, "---\nsplat_schema: 2\nsplat_id: [\n---\n").unwrap();
    reload(d);
    d.click("1 note cannot be read");
    d.click_starting("Notes/Broken.md · ");
    d.expect("Save repaired note");
    d.replace_text_in("Note text", &fixed);
    d.click("Save repaired note");
    d.settle();
    d.expect("Note repaired.");
    d.click_containing("Gate Warden · ");
    d.observe(
        "iter-17-D-rename-undo-kept",
        fs::read_to_string(&warden).unwrap() == renamed && d.has("Undo name change"),
        "Repairing another note keeps this note's Undo name change",
    );
    // The note goes after this check, so its Undo is only pressed when it is there.
    if d.has("Undo name change") {
        d.click("Undo name change");
    }
    fs::remove_file(&warden).unwrap();
    reload(d);
    d.click_starting("Repaired watchkeeper · ");
    // A draft whose note is gone: its choices start at the message text.
    d.click("Edit Repaired watchkeeper");
    d.type_into("Status", "lost words");
    // Round 23 FF3: the warning sits below the file line, so the file line keeps its place.
    d.click("Setting · ");
    // Round 27 FE6: the warning is about another note's draft, so it is in the footer and
    // Setting's own text keeps its place too.
    let prose = "A cold war around an inner sea.";
    let everywhere = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(f32::MAX, f32::MAX));
    let file_line: Vec<_> = [SMALL, DEFAULT, LARGE]
        .map(|size| {
            d.at_size(size, |d| {
                (
                    d.place_of("Show file"),
                    d.painted_rect_in(prose, everywhere),
                )
            })
        })
        .into();
    fs::remove_file(&path).unwrap();
    reload(d);
    d.click("Setting · ");
    let lost = "The note for your unsaved changes to Repaired watchkeeper is gone.";
    for (size, (before, text_before)) in [SMALL, DEFAULT, LARGE].into_iter().zip(file_line) {
        let (after, warning, text_after) = d.at_size(size, |d| {
            (
                d.place_of("Show file"),
                d.painted_rect_in(lost, everywhere),
                d.painted_rect_in(prose, everywhere),
            )
        });
        d.observe(
            "iter-23-D-FF3",
            warning.is_some()
                && before
                    .zip(after)
                    .is_some_and(|(before, after)| (before.top() - after.top()).abs() < 0.5),
            format!("The file line stays at {before:?} → {after:?} with the lost-draft warning at {size:?}"),
        );
        d.observe(
            "iter-27-D-FE6",
            text_before
                .zip(text_after)
                .is_some_and(|(before, after)| (before.top() - after.top()).abs() < 0.5)
                && warning
                    .zip(text_after)
                    .is_some_and(|(warning, text)| warning.top() > text.bottom()),
            format!(
                "Setting's text stays at {text_before:?} → {text_after:?}; the warning {warning:?} is in the footer below it at {size:?}"
            ),
        );
    }
    d.expect("Copy the text of Repaired watchkeeper");
    // Revision 19: with the warning in the footer at 960, the last body action still takes the
    // press, and only it: the type section opens, and the warning and its choices stay.
    let base = d.window_size();
    d.set_base(SMALL);
    d.click("Give this note a type");
    let opened = d.has_label(crate::world::TYPE_HELP)
        && d.has("Copy the text of Repaired watchkeeper")
        && d.has("Discard the draft of Repaired watchkeeper");
    d.click("Give this note a type");
    let opened = opened && !d.has_label(crate::world::TYPE_HELP);
    d.set_base(base);
    d.observe(
        "iter-27-D-FE6-press",
        opened,
        "At 960 the body's last action opens and closes its section under the footer's warning",
    );
    action_at_text_edge(
        d,
        "iter-17-D-copy-text",
        "Copy the text of Repaired watchkeeper",
        "Resize World notes column",
        &[SMALL, DEFAULT, LARGE],
        |d| d.painted_left(lost),
    );
    // Discard draft has no Undo: an `L` gap separates its target from Copy text's.
    for size in [SMALL, DEFAULT, LARGE] {
        let (copy, discard) = d.at_size(size, |d| {
            (
                d.place_of("Copy the text of Repaired watchkeeper"),
                d.place_of("Discard the draft of Repaired watchkeeper"),
            )
        });
        d.observe(
            "iter-17-D-discard-gap",
            copy.zip(discard).is_some_and(|(copy, discard)| {
                (copy.top() - discard.top()).abs() < 0.5
                    && (discard.left() - copy.right() - design::L).abs() <= 0.5
            }),
            format!("At {size:?}, Copy text {copy:?} and Discard draft {discard:?} share a line, {} px apart", copy.zip(discard).map_or(f32::NAN, |(copy, discard)| discard.left() - copy.right())),
        );
    }
    // Round 25 AA1: Discard draft is a message action without an icon, so it has the secondary
    // border. Copy text keeps its icon and stays quiet.
    let bordered = |d: &Driver<'_>, name: &str| {
        d.control_frame(name).is_some_and(|frame| {
            frame.stroke.width > 0.0 && frame.stroke.color == design::colors().field_border
        })
    };
    d.observe(
        "iter-25-D-AA1",
        bordered(d, "Discard the draft of Repaired watchkeeper")
            && !bordered(d, "Copy the text of Repaired watchkeeper"),
        "Discard draft has the secondary border; Copy text with its icon does not",
    );
    // Round 24 FA7: the warning says what happened in the DM's words, not `identity`.
    d.observe(
        "iter-24-D-FA7",
        d.has_containing("It was deleted, or another note took its place.")
            && !d.has_containing("identity"),
        "The lost draft's warning names no identity",
    );
    d.state("lost-draft-choices", Sizes::Default);
    d.click("Discard the draft of Repaired watchkeeper");
    d.observe(
        "iter-17-D-copy-text-discard",
        !d.has("Copy the text of Repaired watchkeeper"),
        "Discard draft removes the lost draft and its message",
    );
    // Round 23 FF1: Discard draft keeps the words for one Undo discard, whatever note is open.
    // A: the discard. B: an independent change to another note. Undo A brings the draft back
    // and keeps B's change and B's Undo.
    let undo_lost = "Undo discard of Repaired watchkeeper";
    d.observe(
        "iter-23-D-FF1-offered",
        d.visible_text("Repaired watchkeeper draft discarded.") && d.has(undo_lost),
        "Discard draft offers Undo discard in the shared Undo message",
    );
    let keeper = root(d).join("NPCs/Lamp Keeper.md");
    entry(
        d,
        "NPCs/Lamp Keeper.md",
        "Lamp Keeper",
        "npc",
        "  in: '[[Setting]]'\n",
    );
    reload(d);
    d.click_starting("Lamp Keeper · ");
    let keeper_before = fs::read_to_string(&keeper).unwrap();
    d.click("Change In");
    d.click("Clear value");
    d.expect("Undo place change");
    let keeper_after = fs::read_to_string(&keeper).unwrap();
    d.observe(
        "iter-23-D-FF1-b-changed",
        keeper_after != keeper_before && d.has(undo_lost),
        "B clears Lamp Keeper's place, and the lost draft's Undo stays",
    );
    d.click(undo_lost);
    d.observe(
        "iter-23-D-FF1-undo",
        d.has("Copy the text of Repaired watchkeeper")
            && !d.has(undo_lost)
            && fs::read_to_string(&keeper).unwrap() == keeper_after
            && d.has("Undo place change"),
        "Undo discard brings the lost draft back; B's change and its Undo stay",
    );
    // The next Edit ends the Undo: it is offered again after a second Discard, then goes.
    d.click("Discard the draft of Repaired watchkeeper");
    d.expect(undo_lost);
    d.click("Edit Lamp Keeper");
    d.observe(
        "iter-23-D-FF1-ends",
        !d.has(undo_lost) && !d.has("Copy the text of Repaired watchkeeper"),
        "The next Edit ends the lost draft's Undo discard",
    );
    d.click("Cancel");
    fs::remove_file(&keeper).unwrap();
    reload(d);
    // A file that cannot be opened for repair. The pane names the file, the recovery and, in
    // Technical details, the system's words. Its bytes never change. A second unreadable note
    // with the longest realistic path crowds the list beside it.
    let unreadable = [0xff, 0xfe, b'\n'];
    let long_path = "Notes/Letters from the Northern Shore/The watchkeepers' unread letters to the Harbour Council.md";
    let long = root(d).join(long_path);
    fs::create_dir_all(long.parent().unwrap()).unwrap();
    fs::write(&path, unreadable).unwrap();
    fs::write(&long, b"# Letters\n\xe9t\xe9\n").unwrap();
    // A note with a warning beside the two errors, so the page shows both counts (FG1).
    entry(
        d,
        "NPCs/Lost Scout.md",
        "Lost Scout",
        "npc",
        "  in: '[[Nowhere]]'\n",
    );
    reload(d);
    d.click("2 notes cannot be read");
    let row = "Notes/Broken.md · Not a text file";
    d.observe(
        "iter-18-D-AD2-list-cause",
        d.has(row) && d.has(&format!("{long_path} · Not a text file")),
        "Each unreadable row states the short cause, not the system's words",
    );
    // The click finds the row by its path, so a wrong cause fails only the check above.
    d.click_starting("Notes/Broken.md · ");
    // Round 24 FG6: the title and the file line name the file, so the error does not.
    let headline = "Splat cannot read this note as text.";
    d.expect(headline);
    d.observe(
        "iter-24-D-FG6",
        !d.has_containing("Splat cannot read Notes/Broken.md")
            && d.visible_text("Broken.md")
            && d.has_label(headline),
        "The unreadable pane's error says this note; its title and file line name the file",
    );
    // Round 24 AG2: the recovery is prose at the reader measure, at every captured size.
    let recovery = "Open the file in a text editor and save it as UTF-8 text";
    for size in [SMALL, DEFAULT, LARGE] {
        let width = d.at_size(size, |d| {
            d.painted_rect_in(recovery, egui::Rect::EVERYTHING)
                .map(|rect| rect.width())
        });
        d.observe(
            "iter-24-D-AG2",
            width.is_some_and(|width| width <= design::READER_MEASURE + 0.5),
            format!("The recovery paragraph is {width:?} wide at {size:?}"),
        );
    }
    d.observe(
        "iter-18-D-AD2-headline",
        d.has_label(headline)
            && !d.has_containing("stream did not contain")
            && !d.has_containing("valid UTF-8 ·"),
        "The pane leads with the file; the system's words wait in Technical details",
    );
    d.observe(
        "iter-18-D-AD1-recovery",
        d.has_containing("Open the file in a text editor and save it as UTF-8 text, or restore it from a backup. Then choose Try again."),
        "The pane explains the text format and the recovery",
    );
    // FD6: no editor can open a file that Splat cannot read as text, so its file line offers
    // only Show file.
    d.observe(
        "iter-19-D-FD6-no-editor",
        d.has("Show file") && !d.has("Edit in Obsidian"),
        "The file line of a file that is not text offers Show file and no editor link",
    );
    d.observe(
        "iter-18-D-AD1-file-line",
        d.has("Show file")
            && d.place_of("Show file")
                .zip(d.place_of(headline))
                .is_some_and(|(show, words)| show.bottom() <= words.top()),
        "The file line with Show file sits above the message",
    );
    // Round 25 FG3 (replaces round 18's quiet `line_start` edge): Try again and Close note are
    // message actions without an icon, so each has the secondary border. Try again's border, not
    // its words, starts at the message text; its words sit inside the border. Close note follows on
    // the same line, `L` after it.
    for size in [SMALL, DEFAULT, LARGE] {
        let (again, cancel, edge, words, splitter) = d.at_size(size, |d| {
            let again = d.place_of("Try again");
            let words = again.and_then(|again| words_left_in(d, again));
            (
                again,
                d.place_of("Close note"),
                d.painted_left(headline),
                words,
                d.place_of("Resize World notes column"),
            )
        });
        d.observe(
            "iter-25-D-FG3",
            again
                .zip(edge)
                .zip(words)
                .is_some_and(|((again, edge), words)| {
                    (again.left() - edge).abs() <= 1.0 && words >= edge + design::S
                })
                && again
                    .zip(splitter)
                    .is_some_and(|(again, splitter)| again.left() >= splitter.right()),
            format!(
                "At {size:?}, Try again's border {again:?} starts at the text edge {edge:?}, its \
                 words at {words:?}"
            ),
        );
        let border = |name: &str| {
            d.control_frame(name).is_some_and(|frame| {
                frame.stroke.width > 0.0 && frame.stroke.color == design::colors().field_border
            })
        };
        let framed = border("Try again") && border("Close note");
        d.observe(
            "iter-25-D-FG3-border",
            framed,
            format!("At {size:?}, Try again and Close note have the secondary border"),
        );
        d.observe(
            "iter-18-D-cancel-follows",
            again.zip(cancel).is_some_and(|(again, cancel)| {
                (again.top() - cancel.top()).abs() < 0.5
                    && (cancel.left() - again.right() - design::L).abs() <= 1.0
            }),
            format!("At {size:?}, Close note {cancel:?} follows Try again {again:?}, L apart"),
        );
    }
    // Round 24 FG1, FE1, AG1 (L, D contract): a row marks every fault the header counts. The two
    // notes that cannot be read are errors in their rows and in their count; each warning row
    // has the warning icon in its tone before muted words; each count equals its marks.
    let (counts, detail) = d.at_size(LARGE, |d| world_fault_marks(d, true));
    d.observe("iter-24-D-FG1", counts, detail);
    d.state("repair-cannot-open", Sizes::All);
    // iter-25-A (carried): with both header counts beside Lookup at 960, the page header keeps
    // its note count whole; the counts' links wrap in the width it leaves.
    d.click(NAV_LOOKUP);
    let count = d.at_size(SMALL, |d| {
        let header = d
            .nodes()
            .into_iter()
            .find(|node| node.name.ends_with(" notes") && node.rect.top() < 60.0);
        let links = d.has("1 note needs fixing") && d.has("2 notes cannot be read");
        header.map(|node| {
            (
                node.name.clone(),
                d.painted_in(node.rect.expand(1.0)),
                links,
            )
        })
    });
    d.observe(
        "iter-25-A-world-header-count",
        count
            .as_ref()
            .is_some_and(|(name, painted, links)| *links && painted.contains(name.as_str())),
        format!("Beside Lookup at 960, the header's count is painted whole: {count:?}"),
    );
    d.state("both-counts-beside-lookup", Sizes::Small);
    d.click("Close Lookup");
    // The warning note served the checks above, so the later Lookup step runs without it.
    fs::remove_file(root(d).join("NPCs/Lost Scout.md")).unwrap();
    reload(d);
    d.click("Technical details");
    d.observe(
        "iter-18-D-AD2-details",
        d.has_containing("stream did not contain valid UTF-8"),
        "Technical details keeps the system's words",
    );
    d.expect_same(
        "Technical details opens one short line of the system's words under its heading.",
    );
    d.state("repair-cannot-open-details", Sizes::Default);
    d.click("Close note");
    d.observe(
        "iter-17-D-error-cancel-closes",
        !d.has(headline),
        "Close note removes the error",
    );
    d.observe(
        "iter-18-D-AD1-bytes-cancel",
        fs::read(&path).unwrap() == unreadable,
        "Opening and cancelling leave the file's bytes as they were",
    );
    // Another note's row takes the pane, as it does from an open repair.
    d.click_starting("Notes/Broken.md · ");
    d.expect(headline);
    d.click("Gate Mechanics · ");
    d.observe(
        "iter-18-D-row-yields",
        !d.has(headline) && !d.has("Try again") && d.has_label("Gate Mechanics"),
        "A note row opens that note in the reader and ends the unreadable pane",
    );
    // The longest path wraps in the list at 960, whole, with its cause below it.
    let long_row = format!("{long_path} · Not a text file");
    let whole = d.at_size(SMALL, |d| {
        d.place_of(&long_row).is_some_and(|row| {
            let painted = d.painted_in(row);
            painted.contains("Harbour Council.md") && painted.contains("Not a text file")
        })
    });
    d.observe(
        "iter-18-D-long-row-wraps",
        whole,
        "At 960, the longest unreadable path wraps whole in its row, above its cause",
    );
    // The longest path wraps at 960 and keeps its choices in view.
    d.click_starting(&format!("{long_path} · "));
    // The pane names the long file in its file line; its error says this note.
    let long_headline = headline.to_owned();
    d.expect(&long_headline);
    d.expect("The watchkeepers' unread letters to the Harbour Council.md");
    let fits = d.at_size(SMALL, |d| {
        d.nodes()
            .iter()
            .any(|node| node.name == long_headline && node.fully_in_view())
            && d.in_view("Try again")
            && d.in_view("Close note")
    });
    d.observe(
        "iter-18-D-long-path-960",
        fits,
        "At 960, the longest path's message wraps whole and its choices stay in view",
    );
    // Round 28 FH9: the title is the file's name, so the file line shows the folder, not the
    // file name again.
    let line = d.at_size(SMALL, |d| {
        d.nodes()
            .into_iter()
            .find(|node| node.name.ends_with(long_path))
            .map(|node| d.painted_in(node.rect))
    });
    d.observe(
        "iter-28-D-FH9",
        line.as_deref().is_some_and(|words| {
            words.trim_end().ends_with('/') && !words.contains("Harbour Council.md")
        }),
        format!("The file line under the file-name title says {line:?}"),
    );
    d.state("repair-cannot-open-long", Sizes::Small);
    // A file deleted outside Splat while its pane is open ends the pane on the next load.
    fs::remove_file(&long).unwrap();
    fs::remove_dir(long.parent().unwrap()).unwrap();
    reload(d);
    d.observe(
        "iter-18-D-deleted-closes",
        !d.has(&long_headline) && !d.has("Try again"),
        "A deleted file's pane closes with its row",
    );
    // Saved again as UTF-8 text outside Splat: Try again opens the note in the reader.
    d.click_starting("Notes/Broken.md · ");
    d.expect(headline);
    fs::write(&path, &fixed).unwrap();
    d.click("Try again");
    d.observe(
        "iter-18-D-try-again-opens",
        !d.has(headline) && d.has("Edit Repaired watchkeeper"),
        "Try again reads the folder again and opens the repaired note",
    );
    // Must stay true: a note that is text again gets its editor link back.
    d.observe(
        "iter-19-D-FD6-text-keeps-editor",
        d.has("Show file") && d.has("Edit in Obsidian"),
        "A note read as text again offers Edit in Obsidian beside Show file",
    );
    // FD6, Part of: an open typed note that Splat can no longer read as text. The outside edit
    // keeps the file's length and time, so nothing reloads. Part of reads the file again first;
    // its error offers no editor link, and neither does the file line. (The visual suite runs
    // as root in its namespace, so a permission change cannot make a file unreadable here.)
    {
        flat_entry(
            d,
            "Factions/Lantern Guild.md",
            "Lantern Guild",
            "splat_type: faction",
            "Keeps the lamps.\n",
        );
        flat_entry(
            d,
            "Factions/Ash Court.md",
            "Ash Court",
            "splat_type: faction",
            "Judges the ash.\n",
        );
        flat_entry(
            d,
            "NPCs/Mira Vell.md",
            "Mira Vell",
            "splat_type: npc\nsplat_part_of: ['[[Lantern Guild]]']\nsplat_status: alive",
            "Trims the wicks.\n",
        );
        reload(d);
        d.click_containing("Mira Vell · ");
        d.expect("Change Part of");
        d.expect("Edit in Obsidian");
        let npc = root(d).join("NPCs/Mira Vell.md");
        let text = fs::read(&npc).unwrap();
        let time = fs::metadata(&npc).unwrap().modified().unwrap();
        // Same length, same time: the reader cannot tell that the bytes changed.
        let outside = |bytes: &[u8]| {
            fs::write(&npc, bytes).unwrap();
            fs::File::options()
                .write(true)
                .open(&npc)
                .unwrap()
                .set_modified(time)
                .unwrap();
        };
        let mut not_text = text.clone();
        let wick = not_text.len() - 2;
        not_text[wick] = 0xff;
        outside(&not_text);
        d.click("Change Part of");
        d.type_into("Find a group", "Ash");
        d.click("Ash Court");
        let refused = "Splat cannot read NPCs/Mira Vell.md as text.";
        d.observe(
            "iter-19-D-FD6-part-of-unread",
            d.has_containing(refused) && !d.has("Edit in Obsidian") && d.has("Show file"),
            "A Part of change that cannot read the note says so and offers no editor link",
        );
        // Must stay true: once the file is text again, the same change saves, its error goes,
        // and the editor link comes back.
        outside(&text);
        d.click("Change Part of");
        d.type_into("Find a group", "Ash");
        d.click("Ash Court");
        d.observe(
            "iter-19-D-FD6-part-of-recovers",
            !d.has_containing(refused)
                && d.has("Edit in Obsidian")
                && fs::read_to_string(&npc).unwrap().contains("Ash Court"),
            "With the file text again, the change saves and the editor link returns",
        );
        // Leave the page as the steps below expect it.
        d.click("Undo group change");
        for path in [
            "NPCs/Mira Vell.md",
            "Factions/Lantern Guild.md",
            "Factions/Ash Court.md",
        ] {
            fs::remove_file(root(d).join(path)).unwrap();
        }
        // Folders this step made go too; a seed folder that still holds notes stays.
        for folder in ["NPCs", "Factions"] {
            let _ = fs::remove_dir(root(d).join(folder));
        }
        reload(d);
        d.click_containing("Repaired watchkeeper");
        d.expect("Edit Repaired watchkeeper");
    }

    fs::write(&path, "---\nsplat_schema: 2\nsplat_id: [\n---\n").unwrap();
    reload(d);
    look_up(d, "watchkeeper");
    d.expect("1 note cannot be read");
    d.state("broken-note-in-lookup", Sizes::All);
    d.click("Show");
    d.expect("Save repaired note");
    d.observe(
        "W5",
        d.current_page() == "World" && d.has(&crate::world::repair::editor_name("Notes/Broken.md")),
        "Lookup opens the broken file on World",
    );
    d.replace_editor_text(&fixed);
    d.click("Save repaired note");
    d.settle();
    d.observe(
        "W5",
        !d.has("1 note cannot be read") && fs::read_to_string(&path).unwrap() == fixed,
        "Save repaired note writes the repair and removes the warning",
    );
    // Round 24 FD5: the DM typed a new name in the repair. Beside `Rename the note to Broken`,
    // the second safe choice keeps that name and renames the file, with Undo.
    d.expect("Rename the note to Broken");
    // Review re-check 2: the DM saves a link into the note. The reader's body has its links
    // converted, and the rename compares the file's own text, so it still works after the DM's
    // own Save (below), while an outside edit still blocks it.
    d.click("Edit Repaired watchkeeper");
    d.click("Text");
    d.press_ctrl(Key::End);
    d.type_text(" See [[Verori]].");
    d.click("Save changes");
    d.wait_for_text("Changes saved.");
    let fixed = fs::read_to_string(&path).unwrap();
    assert!(
        fixed.contains("See [[Verori]]."),
        "fixture: the note has a link"
    );
    d.expect("Rename the file to Repaired watchkeeper.md");
    // Re-check 3: directly after the DM's own Save, with no forced reload, the rename and its
    // Undo work, so Save itself refreshes what the rename compares with.
    let after_save = root(d).join("Notes/Repaired watchkeeper.md");
    d.click("Rename the file to Repaired watchkeeper.md");
    d.settle();
    d.observe(
        "re-check-3-D-after-save",
        !path.exists()
            && fs::read_to_string(&after_save).ok().as_deref() == Some(fixed.as_str())
            && d.has("Undo file rename"),
        "Right after the DM's Save, Rename the file moves the saved note",
    );
    if d.has("Undo file rename") {
        d.click("Undo file rename");
        d.settle();
    }
    d.observe(
        "re-check-3-D-after-save-undo",
        fs::read_to_string(&path).ok().as_deref() == Some(fixed.as_str()) && !after_save.exists(),
        "Undo file rename moves the saved note back",
    );
    d.expect("Rename the file to Repaired watchkeeper.md");
    // Re-check 2, repair 2: the rename compares the whole file with the whole file the index
    // read. An unchanged note with a byte order mark and CRLF line ends renames.
    let crlf_path = root(d).join("NPCs/Crlf Note.md");
    let crlf_text = format!(
        "\u{feff}---\r\nsplat_schema: 2\r\nsplat_id: {}\r\nsplat_kind: world\r\nsplat_name: Crlf Keeper\r\nsplat_type: npc\r\n---\r\n\r\nKept with Windows line ends.\r\n",
        splat::character::stable_id("crlf-keeper")
    );
    fs::create_dir_all(crlf_path.parent().unwrap()).unwrap();
    fs::write(&crlf_path, &crlf_text).unwrap();
    reload(d);
    d.click_starting("Crlf Keeper · ");
    d.click("Rename the file to Crlf Keeper.md");
    d.wait_for_text("The file is now Crlf Keeper.md.");
    d.observe(
        "re-check-2-2-crlf",
        !crlf_path.exists()
            && fs::read_to_string(root(d).join("NPCs/Crlf Keeper.md"))
                .ok()
                .as_deref()
                == Some(crlf_text.as_str()),
        "An unchanged note with a byte order mark and CRLF line ends renames, bytes kept",
    );
    // A YAML comment that changes outside Splat before the reader reads the file. The index
    // reads the note with its comment while another note is open, so no reader has seen it.
    // Then the comment changes, with the same length and file time, and the DM opens the note.
    let with_comment = fixed.replacen("---\n", "---\n# watch notes\n", 1);
    assert_ne!(with_comment, fixed, "fixture: a comment line");
    d.click_containing("Setting · ");
    fs::write(&path, &with_comment).unwrap();
    reload(d);
    let comment_time = fs::metadata(&path).unwrap().modified().unwrap();
    let keep_time = |text: &str| {
        fs::write(&path, text).unwrap();
        fs::File::options()
            .write(true)
            .open(&path)
            .unwrap()
            .set_modified(comment_time)
            .unwrap();
    };
    let changed_comment = with_comment.replace("# watch notes", "# watch NOTES");
    keep_time(&changed_comment);
    d.click_starting("Repaired watchkeeper · ");
    d.click("Rename the file to Repaired watchkeeper.md");
    d.settle();
    d.observe(
        "re-check-2-2-comment",
        fs::read_to_string(&path).ok().as_deref() == Some(changed_comment.as_str())
            && !root(d).join("Notes/Repaired watchkeeper.md").exists()
            && d.has_containing("Notes/Broken.md changed on disk. Nothing was renamed."),
        "A comment changed before the reader read the file blocks the rename",
    );
    keep_time(&with_comment);
    let fixed = with_comment;
    let moved = root(d).join("Notes/Repaired watchkeeper.md");
    // Review repair 1: the file changes outside Splat while the choice shows. The change keeps
    // the file's length and time, so nothing reloads. The move compares with the text the
    // reader came from, so it refuses and the changed file stays where it is.
    let seen_time = fs::metadata(&path).unwrap().modified().unwrap();
    let outside_edit = fixed.replace("is back.", "is home.");
    assert_eq!(outside_edit.len(), fixed.len(), "fixture: same length");
    let write_kept_time = |text: &str| {
        fs::write(&path, text).unwrap();
        fs::File::options()
            .write(true)
            .open(&path)
            .unwrap()
            .set_modified(seen_time)
            .unwrap();
    };
    write_kept_time(&outside_edit);
    d.click("Rename the file to Repaired watchkeeper.md");
    d.settle();
    d.observe(
        "review-1-D-seen-version",
        fs::read_to_string(&path).ok().as_deref() == Some(outside_edit.as_str())
            && !moved.exists()
            && d.has_containing("Notes/Broken.md changed on disk. Nothing was renamed."),
        "Rename the file refuses a file that changed after the reader read it",
    );
    write_kept_time(&fixed);
    d.click("Rename the file to Repaired watchkeeper.md");
    d.wait_for_text("The file is now Repaired watchkeeper.md.");
    d.observe(
        "iter-24-D-FD5",
        !path.exists()
            && fs::read_to_string(&moved).ok().as_deref() == Some(fixed.as_str())
            && d.has("Undo file rename")
            && !d.has_containing("but the name is"),
        "Rename the file moves the note's bytes to its name, and the warning goes",
    );
    // An independent change between the rename and its Undo: another note's name.
    let lamp = root(d).join("NPCs/Lamp.md");
    entry(d, "NPCs/Lamp.md", "Lamp Warden", "npc", "");
    reload(d);
    d.click_containing("Lamp Warden · ");
    let lamp_original = fs::read_to_string(&lamp).unwrap();
    // Round 26 review 3: results with a live Undo are listed newest first, whatever their kind.
    // Order 1: an edit's Discard, then the name change. Order 2: the name change, then a Discard.
    let newer_above = |d: &mut Driver<'_>, newer: &str, older: &str| {
        let (newer_at, older_at) = (d.place_of(newer), d.place_of(older));
        (
            newer_at
                .zip(older_at)
                .is_some_and(|(newer, older)| newer.top() < older.top()),
            format!("{newer:?} at {newer_at:?}, {older:?} at {older_at:?}"),
        )
    };
    d.click("Edit Lamp Warden");
    d.type_into("Status", "awake");
    d.click("Discard");
    let discard = d.name_starting("Undo discard of ");
    d.click("Rename the note to Lamp");
    d.wait_for_text("Undo name change");
    let first = newer_above(d, "Undo name change", &discard);
    d.click("Undo name change");
    d.settle();
    assert_eq!(
        fs::read_to_string(&lamp).unwrap(),
        lamp_original,
        "A's rename undone"
    );
    d.click("Rename the note to Lamp");
    d.wait_for_text("Undo name change");
    d.click("Edit Lamp");
    d.type_into("Status", "asleep");
    d.click("Discard");
    let discard = d.name_starting("Undo discard of ");
    let second = newer_above(d, &discard, "Undo name change");
    d.observe(
        "iter-26-D-newest-first",
        first.0 && second.0,
        format!(
            "Discard then rename: {}; rename then Discard: {}",
            first.1, second.1
        ),
    );
    // The next Edit ends the discard's Undo; the clean form closes with Cancel.
    d.click("Edit Lamp");
    d.click("Cancel");
    d.expect("Undo name change");
    let lamp_renamed = fs::read_to_string(&lamp).unwrap();
    assert!(
        lamp_renamed.contains("splat_name: Lamp\n"),
        "B changed state"
    );
    // Round 26 contract (revision 18): a newer result on the same note joins the older one in
    // the footer, newest Undo first in the footer's order. The older Undo refuses while the file
    // holds the newer write: the bytes stay and the refusal is a reason; then each Undo, newest
    // first, takes its own write back.
    d.click("Change Stat block");
    d.type_text("Guard");
    d.click("Guard");
    d.wait_for_text("Stat block set to Guard.");
    let lamp_both = fs::read_to_string(&lamp).unwrap();
    let listed = d.in_view("Undo name change") && d.in_view("Undo stat block change");
    d.click("Undo name change");
    d.settle();
    let refused = fs::read_to_string(&lamp).unwrap() == lamp_both
        && d.has_containing("changed on disk")
        && d.in_view("Undo stat block change")
        && d.in_view("Undo name change");
    d.click("Undo stat block change");
    d.settle();
    let stat_back = fs::read_to_string(&lamp).unwrap() == lamp_renamed;
    d.click("Undo name change");
    d.settle();
    let restored = fs::read_to_string(&lamp).unwrap() == lamp_original;
    let undo_left = d.has("Undo name change");
    let refusal_left = d.has_containing("changed on disk");
    d.observe(
        "iter-26-D-two-results",
        listed
            && lamp_both != lamp_renamed
            && refused
            && stat_back
            && fs::read_to_string(&lamp).unwrap() == lamp_original
            && !d.has("Undo name change")
            && !d.has_containing("changed on disk"),
        format!(
            "Both listed {listed}; older refused {refused}; newer undone {stat_back}; restored \
             {restored}, Undo name change left {undo_left}, refusal left {refusal_left}"
        ),
    );
    d.click("Rename the note to Lamp");
    d.wait_for_text("Undo name change");
    assert_eq!(
        fs::read_to_string(&lamp).unwrap(),
        lamp_renamed,
        "B changed state again"
    );
    d.click_containing("Repaired watchkeeper · ");
    d.click("Undo file rename");
    d.settle();
    d.observe(
        "iter-24-D-FD5-undo",
        fs::read_to_string(&path).ok().as_deref() == Some(fixed.as_str())
            && !moved.exists()
            && fs::read_to_string(&lamp).unwrap() == lamp_renamed
            && d.has("Rename the file to Repaired watchkeeper.md"),
        "Undo file rename moves the file back; the other note keeps its new name",
    );
    // Review repair 3: the rename's Undo belongs to its campaign. Lookup keeps its reader across
    // a campaign change, and Quiet Harbor has a note at the same relative path. A (rename in
    // Emberfall from Lookup), then a campaign change and B (a name change there), then Undo A.
    let quiet_note = d
        .root
        .join("campaigns/quiet-harbor/Notes/Repaired watchkeeper.md");
    fs::create_dir_all(quiet_note.parent().unwrap()).unwrap();
    fs::write(
        &quiet_note,
        format!(
            "---\nsplat_schema: 2\nsplat_id: {}\nsplat_kind: world\nsplat_name: Quiet keeper\nsplat_type: npc\n---\n\nKeeps the harbour light.\n",
            splat::character::stable_id("quiet-keeper")
        ),
    )
    .unwrap();
    let quiet_before = fs::read_to_string(&quiet_note).unwrap();
    d.click(NAV_ENCOUNTERS);
    if d.has(SEARCH_LOOKUP) {
        look_up_again(d, "Repaired watchkeeper");
    } else {
        look_up(d, "Repaired watchkeeper");
    }
    d.click("Rename the file to Repaired watchkeeper.md");
    d.wait_for_text("The file is now Repaired watchkeeper.md.");
    assert!(moved.exists() && !path.exists(), "A moved the file");
    switch_campaign(d, QUIET_CAMPAIGN);
    d.settle();
    look_up_again(d, "Quiet keeper");
    d.wait_for_text("Keeps the harbour light.");
    d.observe(
        "review-3-D-campaign-owner",
        !d.has("Undo file rename") && quiet_note.exists(),
        "Emberfall's file rename offers no Undo on Quiet Harbor's note at the same path",
    );
    d.click("Rename the note to Repaired watchkeeper");
    d.wait_for_text("Undo name change");
    let quiet_after = fs::read_to_string(&quiet_note).unwrap();
    assert!(
        quiet_after.contains("splat_name: Repaired watchkeeper\n"),
        "B changed state"
    );
    switch_campaign(d, CAMPAIGN);
    d.settle();
    look_up_again(d, "Repaired watchkeeper");
    d.wait_for_text("The watchkeeper is back.");
    // Lead ruling: B's `Undo name change` belongs to Quiet Harbor. Emberfall's note now sits at
    // the same relative path, and it does not offer B's Undo; it offers its own A.
    d.observe(
        "review-3-D-name-undo-owner",
        !d.has("Undo name change") && d.has("Undo file rename"),
        "Quiet Harbor's Undo name change does not show on Emberfall's note at the same path",
    );
    d.click("Undo file rename");
    d.settle();
    d.observe(
        "review-3-D-undo-after-b",
        fs::read_to_string(&path).ok().as_deref() == Some(fixed.as_str())
            && !moved.exists()
            && fs::read_to_string(&quiet_note).ok().as_deref() == Some(quiet_after.as_str()),
        "Undo A back in Emberfall moves its file back; Quiet Harbor keeps B",
    );
    // B's Undo still works in its own campaign, and changes only its own note.
    switch_campaign(d, QUIET_CAMPAIGN);
    d.settle();
    // New words, so Lookup searches Quiet Harbor's notes again.
    look_up_again(d, "Repaired");
    d.wait_for_text("Keeps the harbour light.");
    d.click("Undo name change");
    d.settle();
    d.observe(
        "review-3-D-name-undo-home",
        fs::read_to_string(&quiet_note).ok().as_deref() == Some(quiet_before.as_str())
            && fs::read_to_string(&path).ok().as_deref() == Some(fixed.as_str()),
        "Undo name change in Quiet Harbor restores its note; Emberfall's note stays",
    );
    // Review re-check 1: an edit draft belongs to its campaign. In Lookup, Emberfall's form
    // reloads after an outside change, so its draft is clean and the switch is allowed. Quiet
    // Harbor has a copy of the note (same id, same disk text). It never offers Emberfall's
    // form or `Undo reload`, and keeps its bytes.
    switch_campaign(d, CAMPAIGN);
    d.settle();
    look_up_again(d, "Repaired watchkeeper");
    d.wait_for_text("The watchkeeper is back.");
    d.click("Edit Repaired watchkeeper");
    d.replace_text_in("Status", "ready");
    // The form's Status field, not a Status label elsewhere on screen.
    let status_field = |d: &Driver<'_>| {
        let fields: Vec<_> = d
            .nodes()
            .into_iter()
            .filter(|node| node.name == "Status" && node.role == Role::TextInput)
            .map(|node| node.rect)
            .collect();
        (fields.len() == 1).then(|| fields[0])
    };
    let status_before = status_field(d);
    let outside = format!("{fixed}\nAn outside line.\n");
    fs::write(&path, &outside).unwrap();
    d.click("Save changes");
    d.expect("Repaired watchkeeper changed on disk. Your draft is safe until you choose.");
    d.settle();
    d.click("Reload and lose my draft");
    d.settle();
    d.expect("Undo reload");
    // Round 25 results family: the edit form's reload result sits below its save row, so it does
    // not move the fields; the first edit ends it without moving them back.
    let (status, undo, save) = (
        status_field(d),
        d.place_of("Undo reload"),
        d.place_of("Save changes"),
    );
    d.observe(
        "iter-25-D-edit-reload-below",
        status_before
            .zip(status)
            .is_some_and(|(before, after)| (before.top() - after.top()).abs() <= 0.5)
            && undo
                .zip(save)
                .is_some_and(|(undo, save)| undo.top() >= save.bottom()),
        format!("Status {status_before:?} → {status:?}; Undo reload {undo:?} below Save {save:?}"),
    );
    let copy = d.root.join("campaigns/quiet-harbor/Notes/Broken.md");
    fs::write(&copy, &outside).unwrap();
    switch_campaign(d, QUIET_CAMPAIGN);
    d.settle();
    look_up_again(d, "Repaired watchkeeper");
    d.wait_for_text("The watchkeeper is back.");
    d.observe(
        "re-check-1-D-draft-owner",
        !d.has("Undo reload")
            && !d.has("Save changes")
            && d.has("Edit Repaired watchkeeper")
            && fs::read_to_string(&copy).ok().as_deref() == Some(outside.as_str()),
        "Quiet Harbor's copy offers neither Emberfall's form nor its Undo reload, and keeps its bytes",
    );
}

/// The file each generated NPC card will save, one label per card. A text run repeats its
/// label's name, so it is not counted.
fn npc_saves(d: &Driver<'_>) -> Vec<String> {
    d.nodes()
        .into_iter()
        .filter(|n| n.role == Role::Label)
        .filter_map(|n| n.name.strip_prefix("Will save NPCs/").map(str::to_owned))
        .collect()
}

fn flat_entry(d: &Driver<'_>, path: &str, name: &str, fields: &str, body: &str) {
    let id = splat::character::stable_id(path);
    let path = root(d).join(path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(
        path,
        format!(
            "---\nsplat_schema: 2\nsplat_id: {id}\nsplat_kind: world\nsplat_name: {name}\n{fields}\n---\n\n# {name}\n\n{body}"
        ),
    )
    .unwrap();
}

/// One home for each fact: a page shows what other pages say about it, from their lines,
/// their relations, their dates and their Logs. See `docs/design/world-data.md`.
pub fn facts(d: &mut Driver<'_>) {
    flat_entry(
        d,
        "Lore/Horn Rings.md",
        "Horn Rings",
        "splat_type: lore\nsplat_in: ''\nsplat_status: ''\nsplat_part_of: []",
        "Tieflings wear one ring on their horns for each kept ancestor.\n\n- [ ] Can a horn ring be sold?\n- [x] A horn ring marks an ancestor.\n",
    );
    flat_entry(
        d,
        "Factions/Arena Guild.md",
        "Arena Guild",
        "splat_type: faction",
        "The organizers of the race, under court supervision.\n",
    );
    flat_entry(
        d,
        "Places/The Arena.md",
        "The Arena",
        "splat_type: location",
        "The arena at the capital, where the race starts.\n",
    );
    flat_entry(
        d,
        "NPCs/Zarel.md",
        "Zarel",
        "splat_type: npc\nsplat_status: alive\nsplat_part_of:\n- '[[Arena Guild]]'",
        "Tiefling. The Games-Master, who runs the race.\n\n- **Look:** Gilded [[Horn Rings|horn rings]], loud and new.\n- **Says:** \"Filed!\"\n\nSee [[Horn Rings]].\n",
    );
    flat_entry(
        d,
        "NPCs/Haltha.md",
        "Haltha",
        "splat_type: npc\nsplat_status: alive",
        "- **Look:** Dull iron [[Horn Rings|horn rings]], and old scars that look a little too even.\n",
    );
    flat_entry(
        d,
        "NPCs/Tordra.md",
        "Tordra",
        "splat_type: npc\nsplat_status: alive",
        "An elf convict, who frees other prisoners.\n",
    );
    flat_entry(
        d,
        "History/The Revolt.md",
        "The Revolt",
        "splat_type: event\nsplat_when: 90 years ago\nsplat_until: 85 years ago",
        "The miners took the Arena and held it for five years.\n",
    );
    // A free-text date longer than the date column's limit, with a summary that wraps.
    flat_entry(
        d,
        "History/The Long Siege.md",
        "The Long Siege",
        &format!("splat_type: event\nsplat_when: {LONG_WHEN}"),
        &format!("{LONG_SUMMARY}\n"),
    );
    flat_entry(
        d,
        "History/The Massacre.md",
        "The Massacre",
        "splat_type: event\nsplat_when: 120 years ago",
        "Soldiers fired on protesting miners at [[The Arena]].\n",
    );
    fs::write(
        root(d).join("Visitors.md"),
        "# Visitors\n\nSee [[Horn Rings]].\n",
    )
    .unwrap();
    let sessions = root(d).join(crate::sessions::DEFAULT_FOLDER);
    fs::create_dir_all(&sessions).unwrap();
    fs::write(
        sessions.join("Session 1.md"),
        format!(
            "---\nsession_id: {}\nsession_number: 1\nsession_title: ''\nsession_date: {}\n---\n\nThe race began.\n",
            splat::character::stable_id("visual:session:facts"),
            // A change goes in the Log only while a session is played (W14): a fight, a
            // recording, or a current session that is not dated before today.
            crate::hermetic::today()
        ),
    )
    .unwrap();
    d.app_mut().sessions.refresh();
    reload(d);
    d.click(NAV_WORLD);

    d.click_containing("Horn Rings · ");
    d.expect("From other notes");
    d.observe(
        "T15.1",
        d.has("Look: Gilded horn rings, loud and new.")
            && d.has_containing("Look: Dull iron horn rings")
            && d.visible_text("Also linked from")
            && d.has("Visitors"),
        "A page shows each line that names it, under its page, and a line of links adds only its page",
    );
    for size in [SMALL, DEFAULT, LARGE] {
        let markers = d.at_size(size, |d| {
            d.expect("Open");
            d.expect("Done");
            d.visible_text(egui_phosphor::regular::SQUARE)
                && d.visible_text(egui_phosphor::regular::CHECK_SQUARE)
                && !d.visible_text("[ ] Can")
                && !d.visible_text("[x] A horn")
        });
        d.observe(
            "iter-7-D-FD2",
            markers,
            format!("Task marks have box icons and accessible names at {size:?}"),
        );
        let empty_fields = d.at_size(size, |d| {
            d.expect("Edit Horn Rings");
            !d.has("Change In")
                && !d.has("Change Part of")
                && !d.has("Change Status")
                && !d.visible_text("not set")
        });
        d.observe(
            "iter-7-D-FD6",
            empty_fields,
            format!("A lore reader omits every empty field at {size:?}"),
        );
    }
    d.observe(
        "iter-8-D-FD1",
        d.has("Horn Rings · ")
            && !d.visible_text("Place not found")
            && !d.has_containing("note needs fixing")
            && !d.has_containing("notes need fixing"),
        "An empty optional place has no list warning",
    );
    // FD3: a typed note with no set values shows its prose as a plain note does, with the same
    // gap from the file line to the first prose line.
    let mut typed = Vec::new();
    for size in [SMALL, DEFAULT, LARGE] {
        typed.push(d.at_size(size, |d| {
            prose_gap(d, "Horn Rings.md", "Tieflings wear one ring")
        }));
    }
    d.click("Setting · ");
    d.expect("Edit in Obsidian");
    for (size, typed) in [SMALL, DEFAULT, LARGE].into_iter().zip(typed) {
        let plain = d.at_size(size, |d| {
            prose_gap(d, "Setting.md", "A cold war around an inner sea.")
        });
        d.observe(
            "iter-17-D-FD3",
            typed
                .zip(plain)
                .is_some_and(|(typed, plain)| (typed - plain).abs() <= 1.0),
            format!(
                "At {size:?}, Horn Rings has {typed:?} px from its file line to its prose, Setting {plain:?} px"
            ),
        );
    }
    // A plain note's type choices start at the section's text edge.
    d.click("Give this note a type");
    let help = crate::world::TYPE_HELP;
    // SD2: the help says what the choice does, in the DM's words; storage waits in Technical
    // details, closed.
    d.observe(
        "iter-19-D-SD2-help",
        d.has_label(help) && !d.has_containing("front matter"),
        "The type help states its purpose and names no storage format",
    );
    // FD1 (round 20): the help states what a type gives the DM, and does not repeat the
    // heading `Give this note a type` above it.
    d.observe(
        "iter-20-D-FD1",
        d.has("Give this note a type")
            && d.has_label(help)
            && !help.to_lowercase().contains("this note a type")
            && help.contains("fields")
            && help.contains("stat block"),
        format!("The type help says {help:?}"),
    );
    // Round 25 FG1, AG1: help keeps the reader measure (`design::help`) at every size.
    for size in [SMALL, DEFAULT, LARGE] {
        let width = d.at_size(size, |d| {
            d.painted_rect_in(help, egui::Rect::EVERYTHING)
                .map(|rect| rect.width())
        });
        d.observe(
            "iter-25-D-FG1",
            width.is_some_and(|width| width <= design::READER_MEASURE + 0.5),
            format!("The help is {width:?} wide at {size:?}"),
        );
    }
    action_at_text_edge(
        d,
        "iter-17-D-type-choice",
        "NPC",
        "Resize World notes column",
        &[SMALL, DEFAULT, LARGE],
        |d| d.painted_left(help),
    );
    d.type_into("Another type", "faction");
    // Round 27 FF7: the field ends at the edit form's edge, `FORM_WIDTH` from the reader's text
    // edge (the note's title), not `L` further for the section's indent.
    for size in [DEFAULT, LARGE] {
        let (field, title) = d.at_size(size, |d| {
            let field = d
                .nodes()
                .into_iter()
                .find(|node| node.name == "Another type" && node.role == Role::TextInput)
                .map(|node| node.rect);
            // The title, right of the notes column (the list has a row of the same name).
            let title = d
                .place_of("Resize World notes column")
                .and_then(|splitter| {
                    d.painted_word_rect(
                        "Setting",
                        egui::Rect::from_min_max(
                            egui::pos2(splitter.right(), 0.0),
                            egui::pos2(f32::MAX, f32::MAX),
                        ),
                    )
                });
            (field, title)
        });
        d.observe(
            "iter-27-D-FF7",
            field.zip(title).is_some_and(|(field, title)| {
                (field.right() - (title.left() + design::FORM_WIDTH)).abs() <= 1.0
            }),
            format!(
                "At {size:?}, Another type {field:?} ends at the form edge from the title {title:?}"
            ),
        );
    }
    action_at_text_edge(
        d,
        "iter-17-D-make-type",
        "Make faction",
        "Resize World notes column",
        &[SMALL, DEFAULT, LARGE],
        |d| d.painted_left(help),
    );
    d.state("type-choices", Sizes::Default);
    d.click("Technical details");
    d.observe(
        "iter-19-D-SD2-storage",
        d.has_label(crate::world::TYPE_STORAGE),
        "Technical details at the end of the type section says how Splat stores the type",
    );
    // Must stay true: the open details do not move the choices off the section's text edge.
    action_at_text_edge(
        d,
        "iter-19-D-SD2-choice-edge",
        "NPC",
        "Resize World notes column",
        &[DEFAULT],
        |d| d.painted_left(help),
    );
    d.click("Technical details");
    d.replace_text_in("Another type", "");
    d.click("Give this note a type");
    // Must stay true: a typed note with values keeps its rows between the file line and its
    // prose.
    d.click_containing("Zarel · ");
    d.expect("Change Status");
    for size in [SMALL, DEFAULT, LARGE] {
        let (file_line, first, last, prose) = d.at_size(size, |d| {
            let pane = reader_pane(d);
            (
                d.painted_rect_in("Zarel.md", pane),
                d.place_of("Change Part of"),
                d.place_of("Change Status"),
                d.painted_rect_in("Tiefling. The Games-Master", pane),
            )
        });
        d.observe(
            "iter-17-D-FD3-rows",
            file_line
                .zip(first)
                .zip(last.zip(prose))
                .is_some_and(|((file_line, first), (last, prose))| {
                    first.top() >= file_line.bottom() && prose.top() >= last.bottom()
                }),
            format!(
                "At {size:?}, Zarel's rows {first:?} to {last:?} sit below the file line {file_line:?} and above the prose {prose:?}"
            ),
        );
    }
    d.click_containing("Horn Rings · ");
    d.expect("Edit Horn Rings");
    d.state("lines-from-other-pages", Sizes::All);
    d.expect("Edit Horn Rings");
    d.click("Edit Horn Rings");
    for name in ["Status", "Change In", "Change Part of", "Text"] {
        d.expect(name);
    }
    d.observe_both(
        "iter-9-D-SD3-AD3",
        "An empty optional place says not set in Edit",
        |d| {
            d.expect("Change In");
            let change = d.place_of("Change In").unwrap();
            let label = d.place_of("In").unwrap();
            d.painted_in(egui::Rect::from_min_max(
                egui::pos2(label.left(), change.top()),
                change.max,
            ))
            .contains("not set")
        },
    );
    d.observe_both(
        "iter-11-D-FD4",
        "The empty Status field shows a faint example of its rule",
        |d| {
            d.expect("Status");
            let field = d
                .nodes()
                .into_iter()
                .filter(|node| node.name == "Status" && node.role == Role::TextInput)
                .map(|node| node.rect)
                .collect::<Vec<_>>();
            // Neighbour: the field stays empty and editable; the example is not a value.
            field.len() == 1
                && d.field_text("Status").unwrap_or_default().is_empty()
                && d.painted_in(field[0])
                    .contains(crate::world::edit::STATUS_HINT)
                && !d.is_disabled("Status")
        },
    );
    d.state("empty-world-fields-in-edit", Sizes::All);
    let lore_path = root(d).join("Lore/Horn Rings.md");
    let unchanged = fs::read_to_string(&lore_path).unwrap();
    d.click("Change Part of");
    d.type_into("Find a group", "Arena Guild");
    d.click("Arena Guild");
    d.click("Change Part of");
    d.type_into("Find a group", "Arena");
    d.click("The Arena");
    let group_actions: Vec<_> = d
        .nodes()
        .into_iter()
        .filter(|node| node.role == Role::Button && node.name == "Change Part of")
        .collect();
    d.observe(
        "iter-7-D-FD6-groups",
        !group_actions.is_empty()
            && group_actions.len() == 1
            && d.visible_text("Arena Guild, The Arena"),
        "Two draft groups share one Change action",
    );
    assert_eq!(
        fs::read_to_string(&lore_path).unwrap(),
        unchanged,
        "iter-7-D-FD6-unsent: a group stays in the draft"
    );
    d.type_into_editor("\nWords kept across pane sizes.\n");
    let draft_text = d.editor_text().unwrap();
    for size in [SMALL, LARGE, SMALL, DEFAULT, SMALL] {
        d.set_base(size);
        d.observe(
            "iter-7-D-FD6-draft",
            d.editor_text().as_ref() == Some(&draft_text)
                && d.focused_name().as_deref() == Some("Text"),
            format!("The text and focus survive the pane boundary at {size:?}"),
        );
    }
    d.click("Discard");
    d.observe(
        "iter-8-D-FD11-discard",
        !d.has("Save changes") && !d.visible_text("World draft") && !d.visible_text("Unsaved"),
        "Discard closes the form and clears its draft state",
    );
    assert_eq!(fs::read_to_string(&lore_path).unwrap(), unchanged);
    d.expect("Undo discard of Horn Rings");
    d.click("Undo discard of Horn Rings");
    assert_eq!(d.editor_text().as_ref(), Some(&draft_text));
    d.expect("Arena Guild, The Arena");
    d.observe(
        "iter-8-D-FD11-undo",
        d.has("Save changes") && d.visible_text("Unsaved") && d.visible_text("World draft"),
        "Undo discard restores the form and draft state",
    );
    // Round 27 FF2, FF3: the edit form draws values as Read does. A set group is `Body` in the
    // text colour; `not set` is muted `Small`.
    let style = |d: &Driver<'_>, words: &str| {
        let mut found = None;
        d.each_painted_text(|text, _| {
            if text.galley.job.text == words
                && let Some(section) = text.galley.job.sections.first()
            {
                found = Some((section.format.color, section.format.font_id.size));
            }
        });
        found
    };
    let (group, unset) = (style(d, "Arena Guild, The Arena"), style(d, "not set"));
    d.observe(
        "iter-27-D-FF2-FF3",
        group == Some((design::colors().text, design::Type::Body.size()))
            && unset == Some((design::colors().muted, design::Type::Small.size())),
        format!("The set group draws as {group:?}, `not set` as {unset:?}"),
    );
    d.state("group-draft-restored", Sizes::All);
    d.click("Status");
    d.press_ctrl(Key::S);
    d.wait_for_text("Changes saved.");
    let saved = fs::read_to_string(&lore_path).unwrap();
    assert!(
        saved.contains("splat_part_of:") && saved.contains("Words kept across pane sizes."),
        "iter-7-D-FD6-save: text and groups save together"
    );
    d.click("Undo changes");
    assert_eq!(
        fs::read_to_string(&lore_path).unwrap(),
        unchanged,
        "iter-7-D-FD6-undo: one Undo restores every field"
    );
    // A group-only draft must keep a simultaneous disk edit until the DM chooses.
    d.click("Edit Horn Rings");
    d.click("Change Part of");
    d.type_into("Find a group", "Arena Guild");
    d.click("Arena Guild");
    let external = format!("{unchanged}\nWritten in Obsidian while the group draft was open.\n");
    fs::write(&lore_path, &external).unwrap();
    d.click("Save changes");
    d.expect("Horn Rings changed on disk");
    assert_eq!(fs::read_to_string(&lore_path).unwrap(), external);
    d.click("Overwrite with my draft");
    d.wait_for_text("Changes saved.");
    let merged = fs::read_to_string(&lore_path).unwrap();
    assert!(
        merged.contains("splat_part_of:")
            && crate::sessions::split_note(&merged)
                .unwrap()
                .1
                .starts_with(crate::sessions::split_note(&external).unwrap().1),
        "iter-7-D-FD6-conflict: a group-only overwrite keeps the disk text"
    );
    d.click("Undo changes");
    assert_eq!(fs::read_to_string(&lore_path).unwrap(), external);
    fs::write(&lore_path, &unchanged).unwrap();
    reload(d);
    d.click("Look: Gilded horn rings, loud and new.");
    d.observe(
        "T15.1",
        d.visible_text("Tiefling. The Games-Master, who runs the race."),
        "Pressing a line opens the page that holds it",
    );

    d.click_containing("Arena Guild · ");
    d.expect("Members");
    d.observe(
        "T15.2",
        d.has("Zarel") && d.visible_text("Tiefling. The Games-Master, who runs the race."),
        "A group lists its members, each with what it is",
    );
    d.state("group-members", Sizes::All);

    d.click_containing("The Arena · ");
    d.expect("History");
    let (first, second) = (
        d.painted_rect_in("120 years ago", egui::Rect::EVERYTHING),
        d.painted_rect_in("90 to 85 years ago", egui::Rect::EVERYTHING),
    );
    d.observe(
        "T15.3",
        first
            .zip(second)
            .is_some_and(|(first, second)| first.top() < second.top()),
        "A place shows the events that concern it, oldest first, each with its date",
    );
    // FD3: the dates share one column, as wide as the widest date plus an `S` gap, but never
    // more than a third of the reader. A longer date wraps inside its cell.
    for size in [SMALL, DEFAULT, LARGE] {
        let (column, bound, kept, level) = d
            .at_size(size, |d| {
                let body = d.painted_left("The arena at the capital, where the race starts.")?;
                let reader = egui::Rect::from_min_max(
                    egui::pos2(body - 1.0, 0.0),
                    egui::pos2(f32::INFINITY, f32::INFINITY),
                );
                let short = d.painted_rect_in("120 years ago", reader)?;
                let wide = d.painted_rect_in("90 to 85 years ago", reader)?;
                let long = d.painted_rect_in(LONG_WHEN, reader)?;
                let names = [
                    d.painted_rect_in("The Massacre", reader)?,
                    d.painted_rect_in("The Revolt", reader)?,
                    d.painted_rect_in("The Long Siege", reader)?,
                ];
                let edge = names[0].left();
                // The rule under the History heading spans the reader's own width.
                let heading = d.painted_rect_in("History", reader)?;
                let whole = d
                    .hairlines_in(reader)
                    .into_iter()
                    .filter(|rule| rule.top() > heading.top() && rule.bottom() < short.top())
                    .min_by(|a, b| a.top().total_cmp(&b.top()))?;
                let summary = d.painted_word_rect(
                    "The miners took the Arena and held it for five years.",
                    reader,
                )?;
                let wrapped = d.painted_rect_in(LONG_SUMMARY, reader)?;
                let last_row = d.painted_word_rect(LONG_SUMMARY, reader)?;
                Some((
                    names.iter().all(|name| (name.left() - edge).abs() <= 0.5)
                        && edge - body >= wide.width() + design::S - 1.0
                        && [short, wide, long]
                            .iter()
                            .all(|date| date.right() <= edge - design::S + 1.0),
                    // The long date wraps in a column of at most a third of the reader's own
                    // width, and the name after it stays inside the reader.
                    (
                        (whole.left() - body).abs() <= 2.0
                            && long.height() > short.height() * 1.5
                            && edge - body <= whole.width() / 3.0 + design::S + 1.0
                            && names[2].right() <= whole.right(),
                        edge - body,
                        whole.width(),
                    ),
                    // Must stay true: dates start at the text edge, and a summary, also on its
                    // wrapped lines, never sits under the date column.
                    (short.left() - body).abs() <= 1.0
                        && (wide.left() - body).abs() <= 1.0
                        && (long.left() - body).abs() <= 1.0
                        && summary.left() >= edge - 0.5
                        && last_row.top() > wrapped.top() + 1.0
                        && last_row.left() >= edge - 0.5,
                    // Each date's first line sits on its name's line, also when the date wraps.
                    [(short, names[0]), (wide, names[1]), (long, names[2])]
                        .iter()
                        .all(|(date, name)| {
                            (date.top() + design::Type::Small.line() / 2.0 - name.center().y).abs()
                                <= 1.5
                        }),
                ))
            })
            .unwrap_or_default();
        d.observe(
            "iter-14-D-FD3",
            column,
            format!(
                "History names share one left edge after the widest date and an S gap at {size:?}"
            ),
        );
        let (bound, column_width, reader_width) = bound;
        d.observe(
            "iter-14-D2-FD3-bound",
            bound,
            format!(
                "Date column {column_width:.0} px plus S, reader {reader_width:.0} px at {size:?}. "
            ) + &format!(
                "A long History date wraps in a column of at most a third of the reader at {size:?}"
            ),
        );
        d.observe(
            "iter-14-D2-FD3-level",
            level,
            format!("Each History date's first line sits on its name's line at {size:?}"),
        );
        d.observe(
            "iter-14-D2-FD3-wrap",
            kept,
            format!(
                "History dates start at the text edge, and wrapped summaries stay right of them at {size:?}"
            ),
        );
    }
    // Round 24 FG2: a History row is one paragraph with one line pitch. The summary's first
    // line sits on the name's line, and each wrap after it is one `Small` line lower.
    for size in [SMALL, DEFAULT, LARGE] {
        let (even, detail) = d.at_size(size, |d| {
            let mut tops = Vec::new();
            d.each_painted_text(|text, _| {
                if text.galley.job.text == LONG_SUMMARY {
                    tops = text
                        .galley
                        .rows
                        .iter()
                        .map(|row| text.pos.y + row.rect().center().y)
                        .collect();
                }
            });
            let name = d.painted_rect_in("The Long Siege", egui::Rect::EVERYTHING);
            let pitch = design::Type::Small.line();
            (
                tops.len() >= 2
                    && tops
                        .windows(2)
                        .all(|pair| (pair[1] - pair[0] - pitch).abs() <= 1.0)
                    && name.is_some_and(|name| (name.center().y - tops[0]).abs() <= 1.0),
                format!("name {name:?}, summary line centres {tops:?}"),
            )
        });
        d.observe(
            "iter-24-D-FG2",
            even,
            format!("The Long Siege's lines keep one pitch at {size:?}: {detail}"),
        );
    }
    d.state("place-history", Sizes::All);

    d.click_containing("Zarel · ");
    let path = root(d).join("NPCs/Zarel.md");
    let before = fs::read_to_string(&path).unwrap();
    d.click("Change Part of");
    d.type_into("Find a group", "Arena");
    d.click("The Arena");
    d.expect("Part of set to Arena Guild, The Arena. It was Arena Guild.");
    let after = fs::read_to_string(&path).unwrap();
    d.observe(
        "T15.4",
        after.contains("splat_part_of:\n- '[[Arena Guild]]'\n- '[[The Arena]]'")
            && after.ends_with(
                "## Log\n\n- [[Session 1]]: Part of [[Arena Guild]] → [[Arena Guild]], [[The Arena]].\n",
            ),
        "A change in play writes the value and one Log line in the entry",
    );
    d.state("group-joined-with-log", Sizes::All);
    d.click("Undo group change");
    d.observe(
        "T15.4",
        fs::read_to_string(&path).unwrap() == before,
        "One undo removes the value and its Log line",
    );
    // The index reads the entry again in the background, and the picker offers what it knows.
    reload(d);
    d.click("Change Part of");
    d.type_into("Find a group", "Arena");
    d.click("The Arena");
    d.expect("Part of set to Arena Guild, The Arena. It was Arena Guild.");
    assert_eq!(fs::read_to_string(&path).unwrap(), after);
    // F6: `Add to Log` beside Edit opens a field with the line's stamp. Enter writes one line
    // in the DM's words, stamped as the change above, and its Undo takes back that line alone.
    reload(d);
    d.click("Add to Log");
    d.observe(
        "T18.2",
        d.focused_name().as_deref() == Some("Log line for Zarel") && d.visible_text("Session 1:"),
        "Add to Log opens the Log field with the keyboard in it, after the line's stamp",
    );
    // X31 of app round 2: the stamp and the field end at the reader's measure, as its rules do.
    for size in [DEFAULT, LARGE] {
        let (stamp, field) = d.at_size(size, |d| {
            (d.place_of("Session 1:"), d.place_of("Log line for Zarel"))
        });
        d.observe(
            "T18.2",
            stamp.zip(field).is_some_and(|(stamp, field)| {
                field.right() - stamp.left() <= design::READER_MEASURE + 0.5
            }),
            format!("At {size:?} the stamp {stamp:?} and the field {field:?} end at the measure"),
        );
    }
    // Round 28 AC3, FA6, AA4: beside the field, the stamp is centred on the field's line.
    for size in [DEFAULT, LARGE] {
        let (stamp, field) = d.at_size(size, |d| {
            (
                d.place_of("Session 1:"),
                d.nodes()
                    .into_iter()
                    .find(|node| node.name == "Log line for Zarel" && node.role == Role::TextInput)
                    .map(|node| node.rect),
            )
        });
        d.observe(
            "iter-28-D-FA6",
            stamp.zip(field).is_some_and(|(stamp, field)| {
                stamp.right() < field.left() && (stamp.center().y - field.center().y).abs() <= 1.0
            }),
            format!("At {size:?} the stamp {stamp:?} beside the field {field:?}"),
        );
    }
    d.state("log-field-open", Sizes::All);
    d.type_text("Gave the gate key to Iselle.");
    d.press(Key::Enter);
    d.expect("Added to Zarel’s Log.");
    d.observe(
        "T18.2",
        fs::read_to_string(&path).unwrap()
            == format!("{after}- [[Session 1]]: Gave the gate key to Iselle.\n")
            && !d.has("Log line for Zarel"),
        "Enter adds one line at the end of the Log, with the change's stamp, and closes the field",
    );
    d.state("log-line-added", Sizes::All);
    d.click("Undo Log line");
    d.observe(
        "T18.2",
        fs::read_to_string(&path).unwrap() == after && !d.has("Undo Log line"),
        "Undo Log line takes back that one line",
    );
    reload(d);
    d.click(NAV_SESSIONS);
    d.click_containing("Session 1 · ");
    d.click("Note");
    if d.has("Read") {
        d.click("Read");
    }
    d.expect("World changes");
    d.observe(
        "T15.4",
        d.has("Zarel") && d.visible_text("Part of Arena Guild → Arena Guild, The Arena."),
        "The session lists what play changed, from the entry's Log",
    );
    d.state("session-world-changes", Sizes::All);
    d.click(NAV_WORLD);
    // Row 19 of the session-cycle design: the Log stamp asks W14's one test of the session being
    // played. In prep for a session dated later, a Change now writes its Log line; before, only
    // a session dated today did. Beside Lookup as without it. Dated before today, it writes none.
    let note = sessions.join("Session 1.md");
    let dated = fs::read_to_string(&note).unwrap();
    let change_part_of = |d: &mut Driver<'_>, date: &str| {
        fs::write(&note, dated.replace(&crate::hermetic::today(), date)).unwrap();
        fs::write(&path, &before).unwrap();
        d.app_mut().sessions.refresh();
        reload(d);
        d.click_containing("Zarel · ");
        d.click("Change Part of");
        d.type_into("Find a group", "Arena");
        d.click("The Arena");
        d.expect("Part of set to Arena Guild, The Arena. It was Arena Guild.");
        fs::read_to_string(&path).unwrap()
    };
    let later = change_part_of(d, "2999-01-01");
    d.observe(
        "P1c-row-19",
        later == after,
        "In prep for a session dated later, a Change writes its Log line, with the session's link",
    );
    // The same view as on the session's date.
    d.assert_state_matches("group-joined-with-log-in-prep", "group-joined-with-log");
    // The session lists that line from the entry's Log, beside Lookup as without it.
    d.click(NAV_SESSIONS);
    d.click_containing("Session 1 · ");
    d.click("Note");
    if d.has("Read") {
        d.click("Read");
    }
    d.expect("World changes");
    d.click(NAV_LOOKUP);
    d.observe(
        "P1c-row-19",
        d.has("Close Lookup") && d.visible_text("Part of Arena Guild → Arena Guild, The Arena."),
        "Beside Lookup, the session in prep lists the change from the entry's Log",
    );
    d.state(
        "session-world-changes-in-prep-beside-lookup",
        Sizes::Default,
    );
    d.click("Close Lookup");
    d.click(NAV_WORLD);
    let older = change_part_of(d, "2000-01-01");
    d.observe(
        "P1c-row-19",
        older.contains("- '[[The Arena]]'") && !older.contains("## Log"),
        "After the session's date, with no fight or recording, a Change writes no Log line",
    );
    d.click("Undo group change");
    fs::write(&note, &dated).unwrap();
    fs::write(&path, &after).unwrap();
    d.app_mut().sessions.refresh();
    reload(d);

    d.click_containing("Open questions · ");
    d.expect("Can a horn ring be sold?");
    for size in [SMALL, DEFAULT, LARGE] {
        let unselected = d.at_size(size, |d| {
            d.expect("Zarel · alive");
            d.control_frame("Zarel · alive")
                .is_none_or(|frame| frame.fill != design::colors().selected)
        });
        d.observe(
            "iter-8-D-SD2",
            unselected,
            format!("Open questions leaves the remembered note without selected fill at {size:?}"),
        );
    }
    // Round 26 FE4: the question is `Body` and comes first; its note's link follows in `Small`.
    let (question, source) = d.at_size(DEFAULT, |d| {
        (
            d.place_of("Can a horn ring be sold?"),
            d.place_of("Horn Rings"),
        )
    });
    let sizes = question.zip(source).map(|(question, source)| {
        (
            d.painted_size(question, "Can a horn ring be sold?"),
            d.painted_size(source, "Horn Rings"),
        )
    });
    d.observe(
        "iter-26-D-FE4",
        question
            .zip(source)
            .is_some_and(|(question, source)| question.bottom() <= source.top() + 0.5)
            && sizes
                == Some((
                    Some(design::Type::Body.size()),
                    Some(design::Type::Small.size()),
                )),
        format!("The question {question:?} is Body above its note {source:?}: {sizes:?}"),
    );
    d.state("open-questions", Sizes::All);
    // X45 of app round 2: a question is ticked where the DM reads it. Its box writes `[x]` to
    // that line of its note at once, with `Undo tick` in the pane's footer.
    let horn = root(d).join("Lore/Horn Rings.md");
    let open = fs::read_to_string(&horn).unwrap();
    let question = "Open: Can a horn ring be sold?";
    // The list row just below the Open questions row.
    let row_below = |d: &Driver<'_>| {
        let nodes = d.nodes();
        let open = nodes
            .iter()
            .find(|node| node.name.starts_with("Open questions · "))?
            .rect;
        nodes
            .iter()
            .filter(|node| {
                node.name.contains(" · ")
                    && node.rect.top() >= open.bottom() - 0.5
                    && node.rect.left() < open.right()
                    && node.rect.right() > open.left()
            })
            .map(|node| node.rect)
            .min_by(|a, b| a.top().total_cmp(&b.top()))
    };
    let below_before = row_below(d);
    d.click(question);
    d.expect("Undo tick");
    // Round 28 FE4: the last question ticked, the row stays while its page is open, says that
    // none is open, and the row below keeps its place; the page says so too.
    let below_after = row_below(d);
    d.observe(
        "iter-28-D-FE4",
        d.has("Open questions · No open questions")
            && d.visible_text("No open questions.")
            && below_before.is_some()
            && below_before == below_after,
        format!("The row below: {below_before:?} → {below_after:?}"),
    );
    d.observe(
        "T15.5",
        fs::read_to_string(&horn).unwrap()
            == open.replacen(
                "- [ ] Can a horn ring be sold?",
                "- [x] Can a horn ring be sold?",
                1,
            )
            && !d.has(question)
            && d.visible_text("“Can a horn ring be sold?” ticked in Horn Rings."),
        "The box ticks that line of its note, and the question leaves the list",
    );
    d.state("question-ticked", Sizes::Default);
    d.click("Undo tick");
    d.observe(
        "T15.5",
        fs::read_to_string(&horn).unwrap() == open && d.has(question) && !d.has("Undo tick"),
        "Undo tick opens that box again",
    );
    // The line changed on disk since it was read: the tick writes nothing and says so.
    let changed = open.replace(
        "Can a horn ring be sold?",
        "Can a horn ring be sold or lent?",
    );
    fs::write(&horn, &changed).unwrap();
    d.click(question);
    d.observe(
        "T15.5",
        fs::read_to_string(&horn).unwrap() == changed
            && d.visible_text("This question changed in Horn Rings. Nothing was written.")
            && !d.has("Undo tick"),
        "A question whose line changed on disk is not ticked, and the footer says why",
    );
    // Another note took the file's place outside Splat, with the same question on the same
    // line: the tick writes nothing to it (Astra's review of PR 287).
    let id = open
        .lines()
        .find(|line| line.starts_with("splat_id: "))
        .unwrap();
    let other = open.replace(id, "splat_id: 00000000-0000-4000-8000-0000000000aa");
    fs::write(&horn, &other).unwrap();
    d.click(question);
    d.observe(
        "T15.5",
        fs::read_to_string(&horn).unwrap() == other
            && d.visible_text("Horn Rings changed on disk. Nothing was written.")
            && !d.has("Undo tick"),
        "A note that took the question's file is not ticked",
    );
    fs::write(&horn, &open).unwrap();
    reload(d);
    d.click("Can a horn ring be sold?");
    d.observe(
        "T15.5",
        d.visible_text("Tieflings wear one ring on their horns for each kept ancestor."),
        "An open question opens the page that holds it",
    );
    // Round 26 FF3, FH1: the advice row names suggestions, not a fault count, so it never
    // reads as a second `needs fixing` count. Its page has the same title.
    let row = d.name_starting("Things to check · ");
    d.observe(
        "iter-26-D-FF3-row",
        (row.ends_with(" suggestion") || row.ends_with(" suggestions"))
            && !d.has_containing("Problems · "),
        format!("The advice row says {row:?}"),
    );
    d.click_containing("Things to check · ");
    d.observe(
        "iter-26-D-FF3-title",
        d.has_label("Things to check") && !d.has_label("Problems"),
        "The advice page's title is Things to check",
    );
    d.expect("No summary");
    d.observe(
        "T15.5",
        d.has("Haltha") && d.visible_text("Nothing points here") && d.has("Tordra"),
        "The problems name an entry with no summary and an entry that shows on no other page",
    );
    d.state("world-problems", Sizes::All);
}

pub fn places(d: &mut Driver<'_>) {
    entry(
        d,
        "Locations/Ashfall Keep.md",
        "Ashfall Keep",
        "location",
        "  aliases: [The Keep]\n",
    );
    entry(
        d,
        "NPCs/Rook Ambry.md",
        "Rook Ambry",
        "npc",
        "  in: '[[The Keep]]'\n",
    );
    entry(
        d,
        "Locations/Loop One.md",
        "Loop One",
        "location",
        "  in: '[[Loop Two]]'\n",
    );
    entry(
        d,
        "Locations/Loop Two.md",
        "Loop Two",
        "location",
        "  in: '[[Loop One]]'\n",
    );
    fs::write(
        root(d).join("Visitors.md"),
        "# Visitors\n\nRook Ambry guards [[Ashfall Keep]].\n",
    )
    .unwrap();
    reload(d);
    d.click(NAV_WORLD);
    d.click_containing("Rook Ambry · ");
    d.observe(
        "W3",
        d.has("Ashfall Keep") && d.has("Change Stat block") && d.has("Change In"),
        "An NPC can use a place's alias in its in link",
    );
    d.state("npc-place", Sizes::All);
    let original = fs::read(root(d).join("NPCs/Rook Ambry.md")).unwrap();
    // Round 26 SH1: the In row below the Stat block row keeps its place while the result shows.
    let rows = tops_at_sizes(d, "Change In");
    let before = d.interaction_count();
    d.click("Change Stat block");
    d.type_text("Guard");
    d.click("Guard");
    d.observe(
        "W3",
        d.interaction_count() - before <= 3,
        "Give an NPC a stat block in three interactions",
    );
    assert!(
        fs::read_to_string(root(d).join("NPCs/Rook Ambry.md"))
            .unwrap()
            .contains("splat_stat_block:")
    );
    body_kept(
        d,
        "iter-26-D-SH1",
        "Change In",
        &rows,
        Some("Undo stat block change"),
    );
    d.expect_same("Adding the stat block keeps the note's content and offers Undo.");
    d.state("npc-stat-block-changed", Sizes::All);
    d.click("Undo stat block change");
    body_kept(d, "iter-26-D-SH1-undo", "Change In", &rows, None);
    d.observe(
        "W3",
        fs::read(root(d).join("NPCs/Rook Ambry.md")).unwrap() == original,
        "Undo restores the previous content byte for byte",
    );
    // Round 26 FH2: the rows above keep their place while the cleared place's result shows in
    // the footer. The cleared In row goes with its value, so the prose moves up by exactly one
    // row pitch, and the result adds nothing to that.
    let prose_at = |d: &mut Driver<'_>| {
        [SMALL, DEFAULT, LARGE].map(|size| {
            d.at_size(size, |d| {
                d.painted_rect_in("A note for tonight.", egui::Rect::EVERYTHING)
                    .map(|words| words.top())
            })
        })
    };
    let rows = tops_at_sizes(d, "Change Stat block");
    let prose = prose_at(d);
    d.click("Change In");
    action_at_text_edge(
        d,
        "iter-17-D-clear-value",
        "Clear value",
        "Resize World notes column",
        &[SMALL, DEFAULT, LARGE],
        |d| d.place_of("Find a location").map(|field| field.left()),
    );
    d.click("Clear value");
    d.expect("Undo place change");
    // Round 23 FG3: the message names the place as its row did (`Ashfall Keep`), not the words
    // in the file (`The Keep`).
    d.observe(
        "iter-23-D-FG3-cleared",
        d.visible_text("Place cleared. It was Ashfall Keep.") && !d.visible_text("The Keep"),
        "The cleared place's message names the note the In row showed",
    );
    d.observe(
        "iter-7-D-FD6-cleared",
        !d.has("Change In") && d.in_view("Undo place change"),
        "A cleared place row disappears and its Undo stays visible",
    );
    // FD1, must stay true: a change that names a row value, with no row to sit beside, sits
    // below the row block, not in the page-result place.
    let (undo, row) = (
        d.place_of("Undo place change"),
        d.place_of("Change Stat block"),
    );
    d.observe(
        "iter-17-D-FD1-row-change",
        undo.zip(row)
            .is_some_and(|(undo, row)| undo.top() >= row.bottom()),
        format!("Undo place change at {undo:?}, below the last row at {row:?}"),
    );
    body_kept(
        d,
        "iter-26-D-FH2",
        "Change Stat block",
        &rows,
        Some("Undo place change"),
    );
    let cleared = prose_at(d);
    d.observe(
        "iter-26-D-FH2-prose",
        prose.iter().zip(&cleared).all(|pair| match pair {
            (Some(before), Some(after)) => {
                (before - design::Density::Scan.row() - after).abs() <= 0.5
            }
            _ => false,
        }),
        format!("The prose moves up one row pitch with the In row: {prose:?} → {cleared:?}"),
    );
    d.state("cleared-place-keeps-undo", Sizes::All);
    d.click("Undo place change");
    assert_eq!(
        fs::read(root(d).join("NPCs/Rook Ambry.md")).unwrap(),
        original
    );
    d.click("Change In");
    d.click("New location…");
    d.type_into("Location name", "Watch House");
    d.click("Create location here");
    d.expect("Place set to Watch House. It was Ashfall Keep.");
    let (success, detail) =
        finished_change_tone(d, "Place set to Watch House. It was Ashfall Keep.");
    d.observe("iter-20-D-FD5-place", success, detail);
    d.state("npc-new-place", Sizes::All);
    d.click("Undo place change");
    d.click("Ashfall Keep");
    d.observe(
        "World places",
        d.has("Rook Ambry") && d.has("Visitors"),
        "Here and Mentioned in show the people and notes at a place",
    );
    d.state("place-here-and-mentions", Sizes::All);
    d.click_containing("Loop One · ");
    d.observe(
        "World places",
        d.visible_text("Loop One is in Loop Two"),
        "A place loop remains visible and says why",
    );
    // Round 23 FG2: each note in the loop says so on its list row, and the header counts both.
    // Lookalike: Ashfall Keep holds Rook Ambry but leads back to nothing, so its row is clear.
    d.observe(
        "iter-23-D-FG2-rows",
        ["Loop One", "Loop Two"].iter().all(|name| {
            d.nodes().iter().any(|node| {
                node.name.starts_with(&format!("{name} · ")) && node.name.contains("Place loop")
            })
        }) && d.in_view_containing("2 notes need fixing")
            && !d.nodes().iter().any(|node| {
                node.name.starts_with("Ashfall Keep · ") && node.name.contains("Place loop")
            }),
        "Both loop notes show Place loop on their rows, and the header counts 2 notes",
    );
    // Round 23 FG5: the loop warning wraps at the reader measure, the right edge of the rows.
    let loop_warning = "Loop One is in Loop Two, and Loop Two is in Loop One. Change one of them.";
    for size in [SMALL, DEFAULT, LARGE] {
        let (inside, detail) = d.at_size(size, |d| {
            let warning = d.place_of(loop_warning);
            let pane = d
                .place_of("Resize World notes column")
                .map(|pane| pane.right());
            match warning.zip(pane) {
                Some((warning, pane)) => {
                    let measure = pane + design::S + design::READER_MEASURE;
                    (
                        warning.right() <= measure + 1.0,
                        format!("warning ends at {}, measure at {measure}", warning.right()),
                    )
                }
                None => (false, format!("warning {warning:?}, pane {pane:?}")),
            }
        });
        d.observe(
            "iter-23-D-FG5",
            inside,
            format!("The loop warning wraps at the reader measure at {size:?}: {detail}"),
        );
    }
    d.state("place-loop", Sizes::All);

    d.click_containing("Rook Ambry · ");
    let path = root(d).join("NPCs/Rook Ambry.md");
    let external = fs::read_to_string(&path)
        .unwrap()
        .replace("A note for tonight.", "## Watch schedule\n\nObsidian added the watch schedule.\n\nRook takes the first watch at the east gate. At midnight, the relief patrol leaves Watch House and checks the bridge.\n\nThe captain keeps a spare key in the guard room. Visitors must give their names before they enter the keep.");
    fs::write(&path, &external).unwrap();
    // No focus event: Change must re-read the file before it patches this one key.
    d.click("Change In");
    d.type_text("Watch House");
    d.click("Watch House");
    let after = fs::read_to_string(&path).unwrap();
    assert!(after.contains("Obsidian added the watch schedule."));
    assert_ne!(
        splat::content::Entry::parse("NPCs/Rook Ambry.md", &after)
            .unwrap()
            .fields["splat_world"]["in"],
        splat::content::Entry::parse("NPCs/Rook Ambry.md", &external)
            .unwrap()
            .fields["splat_world"]["in"]
    );
    d.wait_for_text("Obsidian added the watch schedule.");
    d.state("place-change-keeps-disk-edit", Sizes::All);
    d.click("Undo place change");
    assert_eq!(fs::read_to_string(&path).unwrap(), external);
    // Review repair 5: the place picker names a shared name's folder (`Bell House (Ruins)`)
    // with no search on each frame, even when many notes give that name as an alias. The
    // shared names are found when the index loads; the count is kept in that search.
    use crate::test_support::CachedWork;
    entry(d, "Locations/Bell House.md", "Bell House", "location", "");
    entry(d, "Ruins/Bell House.md", "Bell House", "location", "");
    for n in 0..150 {
        entry(
            d,
            &format!("Lanterns/Lantern {n}.md"),
            &format!("Lantern {n}"),
            "lore",
            "  aliases: [Bell House]\n",
        );
    }
    let folder = root(d);
    let names = CachedWork::start("Shared World names", d, move |_: &mut Driver<'_>| {
        crate::lookup::shown_name_counts(&folder)
    });
    reload(d);
    d.click("Change In");
    d.type_text("Bell House");
    let shown = |d: &Driver<'_>| d.has("Bell House (Locations)") && d.has("Bell House (Ruins)");
    let warm = names.warm(d, shown(d), 1);
    d.observe(
        "review-5-D-warm",
        warm.is_ok(),
        format!("The index found the shared names once: {warm:?}"),
    );
    let steady = names.steady(d, |d| {
        d.settle();
        d.at_size(LARGE, |_| ());
        d.settle();
        shown(d)
    });
    d.observe(
        "review-5-D-steady",
        steady.is_ok(),
        format!("Picker frames and a resize do no shared-name search: {steady:?}"),
    );
    // Lookalike: the Ruins note gets its own name, the index reads it again, and both names
    // show without a folder.
    let refresh = names.refresh(d, 1, |d| {
        entry(d, "Ruins/Bell House.md", "Bell House Ruins", "location", "");
        reload(d);
        d.has("Bell House") && d.has("Bell House Ruins") && !d.has_containing("Bell House (")
    });
    d.observe(
        "review-5-D-refresh",
        refresh.is_ok(),
        format!("A new name is found when the index reads again: {refresh:?}"),
    );
}

pub fn round_trip(d: &mut Driver<'_>) {
    d.click(NAV_WORLD);
    d.click("Setting · ");
    d.expect("Edit in Obsidian");
    d.state("before-disk-edit", Sizes::All);
    let path = root(d).join("Setting.md");
    fs::write(&path, "# Setting\n\nThe cold war ended this morning.\n").unwrap();
    d.focus_window(false);
    d.focus_window(true);
    d.wait_for_text("The cold war ended this morning.");
    d.observe(
        "World round trip",
        d.visible_text("The cold war ended this morning.")
            && fs::read_to_string(path).unwrap()
                == "# Setting\n\nThe cold war ended this morning.\n",
        "Returning to the app reloads the selected note without changing its text",
    );
    d.state("after-disk-edit", Sizes::All);
    d.click("New ▾");
    d.click("Note…");
    d.type_into("Name", "Ashfall Faction");
    d.type_into("Type (optional)", "faction");
    d.click("Create note");
    d.expect("Edit in Obsidian");
    d.observe(
        "W12",
        fs::read_to_string(root(d).join("Factions/Ashfall Faction.md"))
            .unwrap()
            .contains("type: faction"),
        "New Note accepts an optional World type and offers the file editor",
    );
    d.expect("Nothing written yet.");
    for size in [SMALL, DEFAULT, LARGE] {
        let one_link = d.at_size(size, |d| {
            let links: Vec<_> = d
                .nodes()
                .into_iter()
                .filter(|node| node.name == "Edit in Obsidian" && node.role == Role::Link)
                .collect();
            !links.is_empty() && links.len() == 1 && links.iter().all(|node| node.in_view())
        });
        d.observe(
            "iter-5-D-FD12",
            one_link,
            format!("An empty World note has one file-edit link at {size:?}"),
        );
    }
    d.state("new-faction-note", Sizes::All);
    let faction_path = root(d).join("Factions/Ashfall Faction.md");
    let faction_before = fs::read_to_string(&faction_path).unwrap();
    d.expect("Edit Ashfall Faction");
    d.click("Edit Ashfall Faction");
    d.type_into("Status", "ready");
    for size in [SMALL, DEFAULT, LARGE] {
        let primary = d.at_size(size, |d| {
            d.expect("Save changes");
            d.expect("Discard");
            d.expect("Unsaved");
            !d.has(NEXT_TURN)
                && d.control_frame("Save changes").is_some_and(|frame| {
                    frame.fill == design::colors().accent_fill
                        || frame.fill == design::colors().accent_hover
                })
        });
        d.observe(
            "iter-6-D-ST1-desk",
            primary,
            format!("World Save owns primary without a live run at {size:?}"),
        );
    }
    for size in [SMALL, DEFAULT, LARGE] {
        let aligned = d.at_size(size, |d| {
            for name in ["Status", "Text", "Text field", "Save changes", "Discard"] {
                d.expect(name);
            }
            let nodes = d.nodes();
            let status = nodes
                .iter()
                .find(|node| node.name == "Status" && node.role == Role::TextInput)
                .expect("Status input")
                .rect;
            let text = d.place_of("Text field").expect("Text field frame");
            let save = d.place_of("Save changes").unwrap();
            let discard = d.place_of("Discard").unwrap();
            let unsaved = d
                .painted_rect_in("Unsaved", egui::Rect::EVERYTHING)
                .unwrap();
            (status.right() - text.right()).abs() <= 1.0
                && (save.right() - status.right()).abs() <= 1.0
                && unsaved.left() < discard.left()
                && discard.right() < save.left()
        });
        d.observe("iter-8-D-FD11", aligned, format!("The World form shares one right edge and puts state and Discard before Save at {size:?}"));
    }
    d.observe(
        "iter-8-D-FD2-edit",
        d.visible_text("World draft") && !d.visible_text("World repair draft"),
        "An unsaved World edit has its own rail status",
    );
    // Round 24 FD7: the changed form locks the campaign switcher, and its save row says why,
    // after Discard and before Save, as every other draft's does.
    for size in [SMALL, DEFAULT, LARGE] {
        let (said, detail) = d.at_size(size, |d| {
            let reason = d.place_of(crate::encounters::draft_switch_reason());
            let discard = d.place_of("Discard");
            let save = d.place_of("Save changes");
            let state = d.painted_rect_in("Unsaved", egui::Rect::EVERYTHING);
            // Between Discard and Save where it fits; else on its own line below the row,
            // starting at the state.
            (
                reason.zip(discard).zip(save).zip(state).is_some_and(
                    |(((reason, discard), save), state)| {
                        (reason.left() >= discard.right()
                            && reason.right() <= save.left()
                            && reason.top() < save.bottom()
                            && reason.bottom() > save.top())
                            || (reason.top() >= save.bottom() - 0.5
                                && (reason.left() - state.left()).abs() <= 1.0)
                    },
                ) && d.visible_text(crate::encounters::draft_switch_reason()),
                format!("reason {reason:?}, Discard {discard:?}, Save {save:?}, state {state:?}"),
            )
        });
        d.observe(
            "iter-24-D-FD7",
            said,
            format!("The World edit save row states the switch reason at {size:?}: {detail}"),
        );
    }
    d.state("world-edit-without-run", Sizes::All);
    // A field keeps its cursor and its text history under its id, and the id carries the note's
    // key: Ctrl+Z in Rook's Status field must not bring back words from Ashfall Faction's draft.
    assert_eq!(d.field_text("Status").as_deref(), Some("ready"));
    let rook_path = root(d).join("NPCs/Rook.md");
    fs::create_dir_all(rook_path.parent().unwrap()).unwrap();
    fs::write(
        &rook_path,
        format!(
            "---\nsplat_schema: 2\nsplat_id: {}\nsplat_kind: world\nsplat_name: Rook\nsplat_type: npc\nsplat_status: alive\n---\n",
            crate::hermetic::new_uuid()
        ),
    )
    .unwrap();
    reload(d);
    d.click_containing("Rook · ");
    d.expect("Edit Rook");
    d.click("Edit Rook");
    d.expect("Save changes");
    assert_eq!(d.field_text("Status").as_deref(), Some("alive"));
    d.type_into("Status", "");
    d.press_ctrl(Key::Z);
    assert_eq!(
        d.field_text("Status").as_deref(),
        Some("alive"),
        "ctrl-z-other-note: Rook keeps its own Status after Ctrl+Z"
    );
    d.click("Cancel");
    fs::remove_file(&rook_path).unwrap();
    reload(d);
    d.click_containing("Ashfall Faction · ");
    assert_eq!(
        d.field_text("Status").as_deref(),
        Some("ready"),
        "ctrl-z-other-note: Ashfall Faction's draft is unchanged"
    );
    // One entry, two pickers: the group search must not take the location search's text
    // history, so each picker kind has its own field name.
    d.click("Change In");
    d.expect("Find a location");
    d.type_into("Find a location", "Gate");
    d.click("Cancel");
    d.click("Change Part of");
    d.expect("Find a group");
    d.type_into("Find a group", "");
    d.press_ctrl(Key::Z);
    assert_eq!(
        d.field_text("Find a group").as_deref(),
        Some(""),
        "ctrl-z-other-picker: the group search keeps its own text after Ctrl+Z"
    );
    d.click("Cancel");
    let repair_path = root(d).join("Notes/Rail repair.md");
    fs::write(&repair_path, "---\nsplat_schema: 2\nsplat_id: [\n---\n").unwrap();
    reload(d);
    d.click_containing("Notes/Rail repair.md · ");
    let repair_text = format!("{}\nUnsent repair words.\n", d.editor_text().unwrap());
    d.replace_text_in("Note text", &repair_text);
    for size in [SMALL, DEFAULT, LARGE] {
        let one_status = d.at_size(size, |d| {
            let statuses: Vec<_> = d
                .nodes()
                .into_iter()
                // iter-26-B FF2: a rail draft status is a link to its draft.
                .filter(|node| {
                    matches!(node.role, Role::Label | Role::Link)
                        && node.name.contains("World repair draft")
                })
                .collect();
            !statuses.is_empty()
                && statuses.len() == 1
                && statuses.iter().all(|node| node.in_view())
                && !d.visible_text("World draft")
        });
        d.observe(
            "iter-8-D-FD2-both",
            one_status,
            format!("A repair and an edit share one repair rail status at {size:?}"),
        );
    }
    d.state("world-edit-and-repair-drafts", Sizes::All);
    d.click("Discard");
    d.expect("World draft");
    fs::remove_file(repair_path).unwrap();
    reload(d);
    d.click_containing("Ashfall Faction · ");
    d.expect("Unsaved");
    d.click("Status");
    d.press_ctrl(Key::S);
    d.wait_for_text("Changes saved.");
    assert!(
        fs::read_to_string(&faction_path).unwrap().contains("ready"),
        "iter-6-D-ST1-shortcut: Ctrl+S must save the focused form",
    );
    d.click("Undo changes");
    assert_eq!(fs::read_to_string(&faction_path).unwrap(), faction_before);
    go_campaign(d);
    d.click(TAB_ENTRIES);
    d.click_containing("Captain Varra · Creature");
    d.click("Make NPC note");
    d.expect("Change Stat block");
    d.observe(
        "W13",
        d.current_page() == "World" && d.has("Captain Varra"),
        "Make NPC note opens the linked person in World",
    );
    d.state("campaign-creature-as-npc", Sizes::All);

    // An unread list must not turn into an empty editable list.
    let id = crate::hermetic::new_uuid();
    let unread_groups = format!(
        "splat_part_of: # Keep the stored groups.\n- '[[Zarel|the Archdevil]]'\n- {{id: {}, link: '[[Lost Guild|the lost]]'}}\n",
        crate::hermetic::new_uuid()
    );
    let unread_path = root(d).join("NPCs/Unread groups.md");
    let before = format!(
        "---\nsplat_schema: 2\nsplat_id: {id}\nsplat_kind: world\nsplat_name: Unread groups\nsplat_type: npc\n{unread_groups}---\n\n# Unread groups\n\nKeeps the gate.\n"
    );
    fs::write(&unread_path, &before).unwrap();
    let reason = unread_reason(&unread_groups);
    reload(d);
    d.click_containing("Unread groups · ");
    d.click("Edit Unread groups");
    d.expect(&reason);
    d.observe_both(
        "iter-8-D2-unread-groups",
        "An unread Part of list blocks only its picker and explains the repair",
        |d| {
            for name in ["Change Part of", "Status", "Text", "Change In"] {
                d.expect(name);
            }
            d.is_disabled("Change Part of")
                && !d.is_disabled("Status")
                && !d.is_disabled("Text")
                && !d.is_disabled("Change In")
        },
    );
    d.observe_both(
        "iter-9-D-FD4-link",
        "The unread Edit field keeps its source line and quiet Obsidian repair link",
        |d| {
            d.expect(&reason);
            d.has("Edit in Obsidian")
                && d.nodes().iter().any(|node| {
                    node.name == "Edit in Obsidian" && node.role == Role::Link && node.in_view()
                })
        },
    );
    d.type_into("Status", "ready");
    for size in [SMALL, DEFAULT, LARGE] {
        let (inside, detail) = d.at_size(size, |d| {
            d.expect(&reason);
            d.expect("Status");
            let warning = d.place_of(&reason).unwrap();
            let form = d
                .nodes()
                .into_iter()
                .find(|node| node.name == "Status" && node.role == Role::TextInput)
                .unwrap()
                .rect;
            (
                warning.right() <= form.right() + 1.0,
                format!(
                    "warning ends at {}, form at {}",
                    warning.right(),
                    form.right()
                ),
            )
        });
        d.observe(
            "iter-15-D-FD6-edit",
            inside,
            format!(
                "The unread warning wraps inside the Edit form's right edge at {size:?}: {detail}"
            ),
        );
        // FD6 (round 20): the warning's action starts at its words, on the line below them.
        let (starts, detail) = d.at_size(size, |d| {
            d.expect(&reason);
            let message = d.painted_rect_in(&reason, egui::Rect::EVERYTHING);
            let links: Vec<egui::Rect> = message
                .map(|message| {
                    d.nodes()
                        .into_iter()
                        .filter(|node| {
                            node.name == "Edit in Obsidian"
                                && node.role == Role::Link
                                && node.in_view()
                                && node.rect.top() >= message.bottom() - 1.0
                                && node.rect.top() <= message.bottom() + design::XL
                        })
                        .map(|node| node.rect)
                        .collect()
                })
                .unwrap_or_default();
            let start = links.first().and_then(|link| words_left_in(d, *link));
            (
                links.len() == 1
                    && start
                        .zip(message)
                        .is_some_and(|(start, message)| (start - message.left()).abs() <= 1.0),
                format!("message at {message:?}; links below it {links:?}, words from {start:?}"),
            )
        });
        d.observe(
            "iter-20-D-FD6",
            starts,
            format!("At {size:?}, Edit in Obsidian starts at the warning's words: {detail}"),
        );
    }
    d.state("unread-part-of-edit", Sizes::All);
    d.click("Save changes");
    d.wait_for_text("Changes saved.");
    let saved = fs::read_to_string(&unread_path).unwrap();
    assert!(saved.contains("splat_status: ready"));
    assert!(
        saved.contains(&unread_groups),
        "iter-8-D2-group-bytes: another field's save must keep the raw group block"
    );
    d.click("Undo changes");
    assert_eq!(fs::read_to_string(&unread_path).unwrap(), before);
    // Removing the unread property leaves the parsed fields unchanged. The open form
    // must still notice its cleared flag and make the group picker available again.
    d.click("Edit Unread groups");
    d.expect(&reason);
    fs::write(&unread_path, before.replace(&unread_groups, "")).unwrap();
    reload(d);
    d.expect("Change Part of");
    assert!(
        !d.is_disabled("Change Part of"),
        "iter-8-D2-repaired: a repaired list is editable"
    );
    d.click("Cancel");
}

pub fn tables(d: &mut Driver<'_>) {
    let body = "# Table Workshop\n\n<!-- splat:rolls key=valid dice=1d6 per=fortnight -->\n\n| d6 | Result |\n|---|---|\n| 1 | Rain |\n| 2-5 | Fog |\n| 6 | Sun |\n\n<!-- splat:rolls key=gap dice=1d4 -->\n\n| d4 | Gap |\n|---|---|\n| 1 | One |\n| 3-4 | Later |\n\n<!-- splat:rolls key=overlap dice=1d4 -->\n\n| d4 | Overlap |\n|---|---|\n| 1-3 | First |\n| 3-4 | Second |\n\n<!-- splat:rolls key=words dice=1d6 -->\n\n| d6 | Words |\n|---|---|\n| 1 | One |\n| 2 to 5 | Fog |\n| 6 | Six |\n";
    let body = body.replace(
        "<!-- splat:rolls key=words",
        &format!(
            "{}<!-- splat:rolls key=words",
            "\nThe next table checks a range written as words.\n".repeat(8)
        ),
    );
    fs::write(root(d).join("Table Workshop.md"), body).unwrap();
    reload(d);
    d.click(NAV_WORLD);
    d.click_containing("Table Workshop · ");
    d.observe_both(
        "W6",
        "Table faults appear in the World list and warning count",
        |d| d.has_containing("4 table faults") && d.visible_text("1 note needs fixing"),
    );
    // F9: a `per` word that Splat cannot read warns on its table, and the table still rolls.
    // Round 27 FE13: the warning quotes the word the DM wrote and names the words that work.
    let per_warning = crate::roll_table::per_not_read("fortnight");
    d.observe(
        "T11.9",
        d.visible_text(&per_warning)
            && per_warning.contains("per fortnight")
            && per_warning.contains("Use day, watch or hours such as 4h.")
            && d.has("Roll 1d6"),
        "An interval that cannot be read warns on its table, with the word and what to write",
    );
    // Round 27 FE4: every warning under a table has one size: the interval warning is the size
    // of a gap in the rows.
    let sizes = [per_warning.as_str(), "1d4 needs a row for 2."].map(|words| {
        d.painted_rect_in(words, egui::Rect::EVERYTHING)
            .and_then(|rect| d.painted_size(rect, words))
    });
    d.observe(
        "iter-27-L-FE4",
        sizes.iter().all(Option::is_some) && sizes[0] == sizes[1],
        format!("Table warnings share one size (interval, gap): {sizes:?}"),
    );
    d.state("authored-table-validation", Sizes::All);
    d.observe_both(
        "World typed roll field",
        "The typed roll field shares the Roll button edges and has an S gap",
        |d| {
            let Some(button) = d.place_of("Roll 1d6") else {
                return false;
            };
            let Some(field) = d.place_of("Typed roll for Valid") else {
                return false;
            };
            (button.top() - field.top()).abs() <= 1.0
                && (button.bottom() - field.bottom()).abs() <= 1.0
                && (field.left() - button.right() - design::S).abs() <= 1.0
                && (design::XXL * 3.0..=design::XXL * 3.0 + design::S).contains(&field.width())
        },
    );
    // Round 25 FD2: the next table's header, at each size, before the roll.
    let gap_header = |d: &Driver<'_>| {
        let mut top = None;
        d.each_painted_text(|text, _| {
            if text.galley.job.text == "Gap" {
                top = Some(text.pos.y);
            }
        });
        top
    };
    let before = [SMALL, DEFAULT, LARGE].map(|size| d.at_size(size, |d| gap_header(d)));
    d.type_into("Typed roll for Valid", "4");
    d.press(Key::Enter);
    d.observe(
        "World table author",
        d.visible_text("Fog") && d.visible_text("Typed 4"),
        "A valid range accepts a typed roll and shows the matched row",
    );
    // Round 25 FD2: the result's line is reserved, so the roll moves nothing below the table.
    // The result is new (it was not on screen before), and the header keeps its y at each size.
    let after = [SMALL, DEFAULT, LARGE].map(|size| d.at_size(size, |d| gap_header(d)));
    d.observe(
        "iter-25-D-FD2",
        d.visible_text("Typed 4 on 1d6: Fog")
            && before.iter().all(Option::is_some)
            && before.iter().zip(&after).all(|(before, after)| {
                before
                    .zip(*after)
                    .is_some_and(|(b, a)| (a - b).abs() <= 0.5)
            }),
        format!("The d4 Gap header stays put after the roll: before {before:?}, after {after:?}"),
    );
    // Round 24 FG3: the rolled row's range (2-5, marked) starts at the x of the others.
    // `1` and `6` are found nearest the rolled row, in the same table.
    let rolled = d.place_of("2-5");
    let ranges = ["1", "2-5", "6"].map(|range| {
        let rolled = rolled?;
        d.nodes()
            .into_iter()
            .filter(|node| node.name == range)
            .map(|node| node.rect)
            .min_by(|a, b| {
                (a.center().y - rolled.center().y)
                    .abs()
                    .total_cmp(&(b.center().y - rolled.center().y).abs())
            })
    });
    d.observe(
        "iter-24-L-FG3",
        ranges.iter().all(Option::is_some)
            && ranges
                .iter()
                .flatten()
                .all(|rect| (rect.left() - ranges[0].unwrap().left()).abs() <= 0.5),
        format!("Every range of the rolled table starts at one x: {ranges:?}"),
    );
    d.state("authored-table-typed-roll", Sizes::All);
    d.set_base(SMALL);
    d.scroll_until(READER, "The first column must be a number", 24);
    d.wait_for_text("The first column must be a number");
    d.observe(
        "World table author",
        d.visible_text("The first column must be a number"),
        "An invalid table says what needs an edit",
    );
    d.state_prepared("authored-table-gap-and-overlap", Sizes::All, |d| {
        d.scroll_until(READER, "The first column must be a number", 24);
    });
    // P1c: one descriptor names a marked table outside its note: its note, its heading, its key
    // and its dice. Every reader of the campaign's tables takes it from the index. The Reader's
    // own button is as before, in Lookup too.
    let tables = d
        .app()
        .lookup
        .index()
        .map(|index| index.loot_sources().tables.clone())
        .unwrap_or_default();
    let named = |key: &str| {
        tables.iter().find(|table| table.key == key).map(|table| {
            (
                table.path.clone(),
                table.heading.clone(),
                table.dice.clone(),
                table.fault(),
            )
        })
    };
    // F9: the descriptor holds the interval. With no `per`, a journey checks each day; a word
    // that cannot be read also checks each day.
    let per = |key: &str| {
        tables
            .iter()
            .find(|table| table.key == key)
            .map(|table| table.per.clone())
    };
    d.observe(
        "P5-table-interval",
        per("forced_trip") == Some(crate::roll_table::Per::Day)
            && per("valid") == Some(crate::roll_table::Per::NotRead("fortnight".into())),
        format!("Each table names how often a journey rolls it: {tables:?}"),
    );
    // A note's title is its name, not a heading above the table; a `##` heading is one.
    d.observe(
        "P1c-table-descriptor",
        named("forced_trip")
            == Some((
                "Gate Mechanics.md".into(),
                Some("Forced trips".into()),
                "1d20".into(),
                None,
            ))
            && named("valid") == Some(("Table Workshop.md".into(), None, "1d6".into(), None))
            && named("gap").and_then(|table| table.3).is_some_and(|fault| {
                fault.starts_with("Table Workshop.md: 1d4 needs a row for 2.")
            }),
        format!("The campaign's tables name their note and heading: {tables:?}"),
    );
    look_up(d, "Table Workshop");
    d.wait_for_text("Roll 1d6");
    d.observe(
        "P1c-table-descriptor",
        d.has("Roll 1d6") && d.has("Close Lookup"),
        "In Lookup, the table's own button says Roll 1d6, as before",
    );
    d.state_prepared("authored-table-in-lookup", Sizes::Default, |d| {
        // Round 26 AE1: World and Lookup show the same table. Each reader has its own field id
        // (egui paints no `Second use` id warning) and its own roll state: World's typed 4 does
        // not show in Lookup. Lookup is the right pane, so its Roll button is the rightmost.
        let rolls: Vec<_> = d
            .nodes()
            .into_iter()
            .filter(|node| node.name == "Roll 1d6")
            .map(|node| node.rect)
            .collect();
        let fields = d
            .nodes()
            .into_iter()
            .filter(|node| node.name == "Typed roll for Valid")
            .count();
        let lookup = rolls
            .iter()
            .max_by(|a, b| a.left().total_cmp(&b.left()))
            .map(|roll| {
                egui::Rect::from_min_max(
                    roll.left_top(),
                    egui::pos2(roll.left() + 600.0, roll.bottom() + 3.0 * design::XL),
                )
            });
        let everything = d.painted_in(egui::Rect::EVERYTHING);
        d.observe(
            "iter-26-L-AE1",
            rolls.len() == 2
                && fields == 2
                && !everything.contains("Second use")
                && !everything.contains("First use")
                && lookup.is_some_and(|area| !d.painted_in(area).contains("Typed 4")),
            format!(
                "World and Lookup each show the table ({} Roll buttons, {fields} fields) with no widget id warning, and Lookup does not show World's typed roll",
                rolls.len()
            ),
        );
    });
}

pub fn generator(d: &mut Driver<'_>) {
    let path = root(d).join("Names.md");
    fs::write(&path, "# Names\n\n<!-- splat:rolls key=npc-name dice=1d2 -->\n\n| d2 | Name |\n|---|---|\n| 1-2 | Tally |\n").unwrap();
    reload(d);
    d.click(NAV_WORLD);
    d.click("New ▾");
    d.click("Generate NPC");
    d.expect("Tally");
    d.observe(
        "W14",
        d.visible_text("Names from") && d.has("Names.md"),
        "The generator names the campaign table used for names",
    );
    d.state("generator-campaign-table", Sizes::All);
    d.click("Discard");
    fs::write(path, "# Names\n\n<!-- splat:rolls key=npc-name dice=1d6 -->\n\n| d6 | Name |\n|---|---|\n| 1 | Tally |\n").unwrap();
    reload(d);
    d.click("New ▾");
    d.click("Generate NPC");
    d.observe(
        "W14",
        d.visible_text("the name table stops at 1, but 1d6 rolls up to 6.")
            && d.visible_text("Built-in names stand in until it is fixed."),
        "A bad campaign table states why the built-in list stands in",
    );
    d.state("generator-bad-table", Sizes::All);
    for size in [SMALL, DEFAULT, LARGE] {
        let aligned = d.at_size(size, |d| {
            d.expect("Open Names.md");
            let action = d.place_of("Open Names.md").unwrap();
            let warning = d
                .nodes()
                .into_iter()
                .find(|n| n.role == Role::Label && n.name.contains("the name table stops at 1"))
                .unwrap();
            action.top() >= warning.rect.bottom()
                && (action.left() - warning.rect.left()).abs() <= 1.0
                && d.control_frame("Open Names.md")
                    .is_some_and(|frame| frame.stroke.color == design::colors().field_border)
        });
        d.observe(
            "iter-6-D-FD11",
            aligned,
            format!("The generator repair action is bordered and aligned at {size:?}"),
        );
    }
    d.expect("Open Names.md");
    d.click("Open Names.md");
    d.wait_for_text("Names");
    d.observe(
        "W9",
        d.has("Close Lookup") && !d.has("New NPC “Names.md”"),
        "The warning opens its source note in Lookup",
    );

    // FD5: with no names table, Lookup says so in the DM's words. Its link hands this NPC to
    // the World page, which keeps the table marker for the DM who prepares.
    fs::remove_file(root(d).join("Names.md")).unwrap();
    reload(d);
    d.click("Close Lookup");
    d.click("Discard");
    d.observe(
        "iter-14-D-FD5-start",
        !d.has("Save NPC"),
        "The World page has no generated NPC before Lookup rolls one",
    );
    // Round 26 FF1: with no note open, the discard result and its Undo sit in the pane's
    // footer, as below a note, not under the empty state's advice.
    let empty = tops_at_sizes(d, "No note open");
    let undo = d.name_starting("Undo discard of ");
    body_kept(d, "iter-26-D-FF1", "No note open", &empty, Some(&undo));
    look_up(d, "Tally");
    d.click("Clear");
    d.click("Generate NPC");
    d.expect(crate::lookup::USE_OWN_NAMES);
    // Review of PR 323: an NPC that Lookup holds belongs to this campaign (its Save writes into
    // the active one). The switch waits for it as for World's, with the same reason, and opens
    // after Discard.
    let switch = "Save or discard your changes to change campaigns.";
    d.click_starting("Active campaign: ");
    let held = (d.app().blocked_switch(), d.in_view_containing(switch));
    d.press(Key::Escape);
    d.click("Discard");
    let discarded = d.app().blocked_switch();
    d.observe(
        "iter-28-C-lookup-npc-lock",
        held == (Some(switch), true) && discarded.is_none(),
        format!("Held in Lookup: (lock, reason in view) {held:?}; after Discard: {discarded:?}"),
    );
    d.click("Generate NPC");
    d.expect(crate::lookup::USE_OWN_NAMES);
    // One label per card. A text run repeats its label's name, so it is not counted.
    let saves = npc_saves;
    let rolled = saves(d);
    d.observe(
        "iter-14-D-FD5-lookup",
        rolled.len() == 1
            && d.visible_text(crate::lookup::BUILT_IN_NAMES)
            && !d.visible_text("splat:rolls"),
        "Lookup's generated NPC says Built-in names. and shows no table marker",
    );
    d.state("generator-built-in-names-in-lookup", Sizes::All);
    d.click(crate::lookup::USE_OWN_NAMES);
    d.settle();
    d.observe(
        "iter-14-D-FD5-world",
        d.current_page() == "World" && !d.has("Close Lookup") && saves(d) == rolled,
        "Use your own names opens the same NPC on the World page",
    );
    d.observe(
        "iter-15-D-FD3",
        d.visible_text(crate::lookup::OWN_NAMES_HELP)
            && d.in_view(crate::lookup::COPY_MARKER)
            && !d.visible_text("splat:rolls"),
        "The World card says how to use your own names in words, and copies the marker",
    );
    // Round 25 AE1, FE2: help keeps the reader measure (`design::help`) at every size.
    for size in [SMALL, DEFAULT, LARGE] {
        let width = d.at_size(size, |d| {
            d.painted_rect_in(crate::lookup::OWN_NAMES_HELP, egui::Rect::EVERYTHING)
                .map(|rect| rect.width())
        });
        d.observe(
            "iter-25-D-AE1",
            width.is_some_and(|width| width <= design::READER_MEASURE + 0.5),
            format!("The help is {width:?} wide at {size:?}"),
        );
    }

    // FD4, ST2 ruling: the action starts a line outside a message, so it has no icon and its words
    // start at the card's text edge, with the help above it and `Will save` below it.
    let icon = super::words_left(d, crate::lookup::COPY_MARKER);
    let help = d.painted_left(crate::lookup::OWN_NAMES_HELP);
    let save = d.painted_left("Will save ");
    d.observe(
        "iter-16-D-FD4",
        icon.zip(help)
            .zip(save)
            .is_some_and(|((icon, help), save)| {
                (icon - help).abs() <= 1.0 && (save - help).abs() <= 1.0
            }),
        format!("Copy the marker's words start at the card's text edge: words {icon:?}, help {help:?}, Will save {save:?}"),
    );
    // The press area reaches into the pane's gutter, never into the World column's splitter.
    for size in [SMALL, DEFAULT, LARGE] {
        let (copy, splitter) = d.at_size(size, |d| {
            (
                d.place_of(crate::lookup::COPY_MARKER),
                d.place_of("Resize World notes column"),
            )
        });
        d.observe(
            "iter-16-D-FD4",
            copy.zip(splitter)
                .is_some_and(|(copy, splitter)| copy.left() >= splitter.right()),
            format!(
                "At {size:?}, Copy the marker's press area {copy:?} starts right of the splitter {splitter:?}"
            ),
        );
    }
    d.state("generator-own-names-on-world", Sizes::Default);
    // FD3: after the copy, the page says which dice to change, and the card where the line goes.
    // Round 27 FG2: the result is in World's footer, below the card, and `Copy the marker`
    // stays, so the DM can copy again; the card's Save row keeps its place.
    let save_before = d.place_of("Save NPC");
    d.click(crate::lookup::COPY_MARKER);
    let everywhere = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(f32::MAX, f32::MAX));
    let copied = d.painted_rect_in(crate::lookup::MARKER_COPIED, everywhere);
    let save_after = d.place_of("Save NPC");
    d.observe(
        "iter-16-D-FD3-copied",
        d.visible_text(crate::lookup::MARKER_COPIED)
            && d.visible_text("Paste the marker on the line above the table.")
            && d.in_view(crate::lookup::COPY_MARKER),
        "The page says Copied and which dice to change, the card where the marker goes, and Copy the marker stays",
    );
    d.observe(
        "iter-27-D-FG2",
        save_before
            .zip(save_after)
            .is_some_and(|(before, after)| (before.top() - after.top()).abs() < 0.5)
            && copied
                .zip(save_after)
                .is_some_and(|(copied, save)| copied.top() > save.bottom()),
        format!(
            "The result {copied:?} is in the footer below Save NPC {save_before:?} → {save_after:?}"
        ),
    );
    d.state("generator-marker-copied", Sizes::Default);
    // Must stay true: the NPC moved, so Lookup cannot save it twice, and World still saves it.
    d.click(NAV_LOOKUP);
    d.observe(
        "iter-14-D-FD5-moved",
        d.has("Close Lookup") && !d.has(crate::lookup::USE_OWN_NAMES) && saves(d) == rolled,
        "Lookup no longer holds the NPC that moved to World",
    );
    d.click("Close Lookup");
    d.click("Save NPC");
    d.settle();
    let file = root(d).join(format!("NPCs/{}", rolled[0]));
    d.observe(
        "iter-14-D-FD5-saved",
        file.exists() && !d.has("Save NPC"),
        "Save NPC on the World page writes the NPC that Lookup rolled",
    );
    // The copy's result belongs to the NPC on the card: the save ends it.
    d.observe(
        "iter-27-D-FG2-ends",
        !d.visible_text(crate::lookup::MARKER_COPIED),
        "Saving the NPC ends the marker's result",
    );

    // An unsaved NPC on the World page is never replaced by Lookup's. The link opens World,
    // and Lookup keeps its own NPC.
    d.click("New ▾");
    d.click("Generate NPC");
    let world = saves(d);
    look_up(d, "Tally");
    d.click("Clear");
    d.click("Generate NPC");
    d.expect(crate::lookup::USE_OWN_NAMES);
    let both = saves(d);
    let lookup: Vec<String> = both
        .iter()
        .filter(|n| !world.contains(n))
        .cloned()
        .collect();
    d.observe(
        "iter-14-D2-FD5-two",
        world.len() == 1 && both.len() == 2 && lookup.len() == 1,
        "World and Lookup each hold a different unsaved NPC",
    );
    d.click(crate::lookup::USE_OWN_NAMES);
    d.settle();
    d.observe(
        "iter-14-D2-FD5-kept",
        d.current_page() == "World" && !d.has("Close Lookup") && saves(d) == world,
        "Use your own names opens World, which keeps its own unsaved NPC",
    );
    // Round 24 FE3: both NPCs have their names, the one Lookup keeps and the one on this card.
    let kept_line = |file: &str| {
        let name = file.strip_suffix(".md").unwrap_or(file);
        let card = world[0].strip_suffix(".md").unwrap_or(&world[0]);
        crate::world::page::kept_in_lookup(name, card)
    };
    d.observe(
        "iter-14-D3-FD5-reason",
        d.visible_text(&kept_line(&lookup[0])),
        "World's card says why it kept its own NPC, naming Lookup's",
    );
    // Round 26 FB8: the line is a state, so it is an info message with its icon and bar.
    let line = kept_line(&lookup[0]);
    let info = d
        .place_of(&line)
        .is_some_and(|words| d.has_tone_bar(words, design::colors().result.info));
    d.observe(
        "iter-26-D-FB8",
        line.contains(" is not saved. It is in Lookup.") && info,
        format!("World's card line {line:?} is an info message with its bar ({info})"),
    );
    d.state("generator-world-keeps-its-npc", Sizes::All);
    d.click(NAV_LOOKUP);
    d.observe(
        "iter-14-D2-FD5-lookup-kept",
        d.has("Close Lookup") && d.has(crate::lookup::USE_OWN_NAMES) && saves(d) == both,
        "Lookup still holds its NPC after the link",
    );
    // The line follows Lookup's NPC: a reroll there renames it on World.
    d.click_below(&format!("Will save NPCs/{}", lookup[0]), "Reroll");
    d.settle();
    let rerolled: Vec<String> = saves(d)
        .into_iter()
        .filter(|n| !world.contains(n))
        .collect();
    d.click("Close Lookup");
    d.observe(
        "iter-14-D3-FD5-reason-live",
        rerolled.len() == 1
            && rerolled != lookup
            && d.visible_text(&kept_line(&rerolled[0]))
            && !d.visible_text(&kept_line(&lookup[0])),
        "World's line names the NPC Lookup holds now",
    );
    let lookup = rerolled;
    d.click("Save NPC");
    d.settle();
    d.click(NAV_LOOKUP);
    if d.has("Save NPC") {
        d.click("Save NPC");
        d.settle();
    }
    let written = [&world, &lookup].map(|names| {
        names
            .first()
            .is_some_and(|name| root(d).join(format!("NPCs/{name}")).exists())
    });
    d.observe(
        "iter-14-D2-FD5-both-saved",
        written == [true, true],
        "Saving both NPCs writes two files",
    );
}

/// AD1/SD1/FD1: an unsaved generated NPC is a held draft. Each path to another view keeps it in
/// one row, Show brings its card back, Discard has Undo, and Save writes it once. AD2: the
/// card names the campaign's own NPC folder.
pub fn held_npc(d: &mut Driver<'_>) {
    // Ilsa is shorter than a link's minimum target.
    for name in ["Wren", "Oskar", "Ilsa"] {
        entry(d, &format!("People/{name}.md"), name, "npc", "");
    }
    fs::write(
        root(d).join("Bells.md"),
        "# Bells\n\nThe bells ring at dusk.\n\n- [ ] Who rings the bells?\n",
    )
    .unwrap();
    fs::write(
        root(d).join("Broken.md"),
        "---\nsplat_schema: 2\nsplat_id: [\n---\n",
    )
    .unwrap();
    reload(d);
    d.click(NAV_WORLD);
    let people_before = fs::read_dir(root(d).join("People")).unwrap().count();
    // Where Wren's title sits with no result above or below it.
    d.click_starting("Wren · ");
    d.expect("People/Wren.md");
    let plain_title = title_rect(d, "Wren");
    d.observe(
        "iter-16-D-FD1-none",
        !d.rail_words().contains(crate::world::page::UNSAVED_NPC),
        "With no generated NPC, the rail has no Unsaved NPC",
    );
    d.click("New ▾");
    d.click("Generate NPC");
    d.expect("Save NPC");
    let promised: Vec<String> = d
        .nodes()
        .into_iter()
        .filter(|node| node.role == Role::Label)
        .filter_map(|node| node.name.strip_prefix("Will save ").map(str::to_owned))
        .collect();
    d.observe(
        "iter-15-D-AD2",
        promised.len() == 1 && promised[0].starts_with("People/"),
        format!("The card names the campaign's NPC folder: {promised:?}"),
    );
    let file = promised[0].clone();
    let npc = file
        .trim_start_matches("People/")
        .trim_end_matches(".md")
        .to_owned();
    let row = format!("Unsaved NPC: {npc}");
    d.observe(
        "iter-15-D-FD3-sizes",
        d.visible_text(crate::lookup::OWN_NAMES_HELP)
            && d.in_view(crate::lookup::COPY_MARKER)
            && !d.visible_text("splat:rolls"),
        "The World card asks for a marked table of names in words",
    );
    // Round 25 AD2: help keeps the reader measure (`design::help`) at every size.
    for size in [SMALL, DEFAULT, LARGE] {
        let width = d.at_size(size, |d| {
            d.painted_rect_in(crate::lookup::OWN_NAMES_HELP, egui::Rect::EVERYTHING)
                .map(|rect| rect.width())
        });
        d.observe(
            "iter-25-D-AD2",
            width.is_some_and(|width| width <= design::READER_MEASURE + 0.5),
            format!("The help is {width:?} wide at {size:?}"),
        );
    }

    d.state("held-npc-card", Sizes::All);

    // Each path to another view keeps the NPC in one row above that view.
    let kept = |d: &mut Driver<'_>, id: &str, view: &str| {
        d.expect(view);
        d.observe(
            id,
            d.in_view(&row) && d.in_view("Show") && !d.has("Save NPC"),
            format!("{view} opens below one row that keeps the unsaved NPC"),
        );
    };
    d.click_containing("Things to check · ");
    kept(d, "iter-15-D-AD1-problems", "Things to check");
    let short = d
        .nodes()
        .into_iter()
        .find(|node| node.name == "Ilsa" && node.role == Role::Link && node.in_view());
    d.observe(
        "iter-15-D2-short-link",
        short
            .as_ref()
            .is_some_and(|node| node.rect.width() >= design::XL),
        format!(
            "A short link's press target is at least XL wide: {:?}",
            short.as_ref().map(|node| node.rect)
        ),
    );
    d.state("held-npc-row-over-problems", Sizes::All);
    // FD1: the rail names the held NPC once, with the draft word, and leads to its card from
    // another page.
    let rail = d.rail_words();
    d.observe(
        "iter-16-D-FD1",
        rail.matches(crate::world::page::UNSAVED_NPC).count() == 1 && !rail.contains("World draft"),
        format!("The rail names the held NPC once: {rail:?}"),
    );
    d.click(NAV_ENCOUNTERS);
    let away = d.current_page() == "Encounters" && !d.has("Save NPC");
    d.click(&format!(
        "{}: {npc}. Show its card on World.",
        crate::world::page::UNSAVED_NPC
    ));
    d.observe(
        "iter-16-D-FD1-link",
        away && d.current_page() == "World"
            && d.in_view("Save NPC")
            && d.has(&format!("Will save {file}")),
        "From Encounters, the rail's Unsaved NPC opens the same NPC's card on World",
    );
    d.click_containing("Open questions · ");
    kept(d, "iter-15-D-AD1-questions", "Who rings the bells?");
    d.click_starting("Wren · ");
    kept(d, "iter-15-D-AD1-other-note", "Show file");
    d.state("held-npc-row-over-note", Sizes::Default);
    d.click_starting("Broken.md · ");
    kept(d, "iter-15-D-AD1-repair", "Broken.md");
    d.click("New ▾");
    d.click("NPC…");
    kept(d, "iter-15-D-AD1-new-npc", "Create NPC");
    d.click("Cancel");

    // Show brings back the same NPC, whose save writes the promised file.
    d.click("Show");
    d.observe(
        "iter-15-D-AD1-show",
        d.has("Save NPC") && !d.has(&row) && d.has(&format!("Will save {file}")),
        "Show brings back the same NPC's card",
    );
    let notice = format!("{npc} draft discarded.");
    d.click("Discard");
    d.observe(
        "iter-15-D-AD1-discard",
        d.visible_text(&notice) && !d.has("Save NPC"),
        "Discard lets the NPC go with Undo discard",
    );
    // FD2: the result is a message below the file line, and the title stays where it was.
    let (below, detail) =
        result_below_file_line(d, &notice, "People/Wren.md", design::colors().result.info);
    d.observe("iter-16-D-FD2-discard", below, detail);
    let title = title_rect(d, "Wren");
    d.observe(
        "iter-16-D-FD2-discard-title",
        title
            .zip(plain_title)
            .is_some_and(|(title, plain)| (title.top() - plain.top()).abs() <= 0.5),
        format!("Wren's title at {title:?}, with no result at {plain_title:?}"),
    );
    // A discarded NPC is no draft: the rail word goes with it.
    d.observe(
        "iter-16-D-FD1-discarded",
        !d.rail_words().contains(crate::world::page::UNSAVED_NPC),
        "After Discard, the rail no longer says Unsaved NPC",
    );
    // FD5: Edit's icon sits one `L` after a short title, whatever the kind line's width.
    let gap = edit_gap(d, "Wren");
    d.observe(
        "iter-16-D-FD5-short",
        gap.is_some_and(|gap| (gap - design::L).abs() <= 1.0),
        format!("Edit's icon is {gap:?} px after the title Wren"),
    );
    d.state("held-npc-discarded", Sizes::All);
    d.click(&format!("Undo discard of {npc}"));
    d.observe(
        "iter-15-D-AD1-undo",
        d.has(&format!("Will save {file}")) && !d.visible_text(&notice),
        "Undo discard brings back the same NPC",
    );
    // The NPC that Undo brings back is held again, so the rail names it again, once.
    let rail = d.rail_words();
    d.observe(
        "iter-16-D-FD1-undo",
        rail.matches(crate::world::page::UNSAVED_NPC).count() == 1,
        format!("After Undo discard, the rail names the held NPC once: {rail:?}"),
    );

    // Closing the app or changing the campaign asks about the held NPC, as for other drafts.
    let unsaved = format!("The generated NPC \u{201c}{npc}\u{201d} is not saved.");
    d.request_close();
    d.observe(
        "iter-15-D-AD1-close",
        d.in_view_containing(&unsaved) && d.in_view("Keep editing"),
        "The close dialog names the unsaved NPC",
    );
    // The check above requires the dialog. Without it, the rest still reports its own checks.
    if d.has("Keep editing") {
        d.click("Keep editing");
    }
    d.observe(
        "iter-15-D-AD1-close-kept",
        !d.has_containing(&unsaved) && d.has(&format!("Will save {file}")),
        "Keep editing leaves the same NPC on its card",
    );
    let switch = "Save or discard your changes to change campaigns.";
    d.observe(
        "iter-15-D-AD1-switch-start",
        !d.in_view_containing(switch),
        "The campaign list is closed before the check",
    );
    d.click_starting("Active campaign: ");
    d.observe(
        "iter-15-D-AD1-switch",
        d.in_view_containing(switch),
        "The campaign selector is blocked while the NPC is unsaved",
    );
    d.press(Key::Escape);
    d.click("Save NPC");
    d.settle();
    let people_after = fs::read_dir(root(d).join("People")).unwrap().count();
    d.observe(
        "iter-15-D-AD1-saved",
        root(d).join(&file).is_file() && people_after == people_before + 1 && !d.has(&row),
        "Save NPC writes the held NPC to the promised file, once",
    );
    // Must stay true: a saved NPC is no draft, so the rail word goes with it.
    d.observe(
        "iter-16-D-FD1-saved",
        !d.rail_words().contains(crate::world::page::UNSAVED_NPC),
        "After Save NPC, the rail no longer says Unsaved NPC",
    );
}

/// The reader pane: right of the World list at its default width.
fn reader_pane(d: &Driver<'_>) -> egui::Rect {
    let (width, height) = d.window_size();
    egui::Rect::from_min_max(egui::pos2(405.0, 0.0), egui::pos2(width, height))
}

/// Astra re-check 2: a fixed part below Edit Markdown's save row whose message is longer than
/// the part. Tab reaches the part (one widget named with the whole message), Down scrolls its
/// words up, and the wheel does too. Returns (Tab reached it, a key scrolled it, the wheel
/// scrolled it), each from a changed painted position, and whether its focus ring keeps clear of
/// every scroll bar and splitter at 960, 1200 and 1920.
fn fixed_part_scrolls(d: &mut Driver<'_>, message: &str) -> (bool, bool, bool, [bool; 3]) {
    let top = |d: &Driver<'_>| {
        let mut found = None;
        d.each_painted_text(|text, clip| {
            // A message's words may follow its icon in one galley.
            let start = message.chars().take(24).collect::<String>();
            if text.galley.job.text.contains(&start) && clip.intersects(text.visual_bounding_rect())
            {
                found = Some(text.pos.y);
            }
        });
        found
    };
    let tabbed = d.tab_to(message);
    // Astra re-check 3: the part's focus ring keeps clear of its own scroll bar at each size.
    let ring_clear = [SMALL, DEFAULT, LARGE].map(|size| {
        d.at_size(size, |d| {
            let lanes = d.chrome_lanes();
            let rings = d.focus_rings();
            !rings.is_empty()
                && rings
                    .iter()
                    // Painted over: an overlap with area, not a shared edge.
                    .all(|(ring, _)| {
                        lanes
                            .iter()
                            .all(|lane| !lane.intersect(*ring).is_positive())
                    })
        })
    });
    // Tab passes the part's own Undo first, and egui scrolls the part to show it: start from the
    // part's top.
    d.press(egui::Key::PageUp);
    let before = top(d);
    d.press(egui::Key::ArrowDown);
    let keyed = top(d);
    let key_scrolled = before
        .zip(keyed)
        .is_some_and(|(before, keyed)| keyed < before - 1.0);
    // Back to the top, then the wheel over the part.
    d.press(egui::Key::PageUp);
    let back = top(d);
    let part = d
        .nodes()
        .into_iter()
        .filter(|node| node.name == message)
        .map(|node| node.rect)
        .max_by(|a, b| a.area().total_cmp(&b.area()));
    let wheel_scrolled = match (back, part) {
        (Some(back), Some(part)) => {
            d.scroll_at(part.center(), 1);
            // A short first message can scroll out of the part entirely.
            match top(d) {
                Some(after) => after < back - 1.0,
                None => d.has(message),
            }
        }
        _ => false,
    };
    (tabbed, key_scrolled, wheel_scrolled, ring_clear)
}

/// Wheel the message space that holds `words` until `control` is fully in view, at the current size.
fn reveal_in_space(d: &mut Driver<'_>, words: &str, control: &str) {
    let in_view = |d: &Driver<'_>| {
        d.nodes()
            .iter()
            .any(|node| node.name == control && node.fully_in_view())
    };
    let Some(space) = d
        .nodes()
        .into_iter()
        .find(|node| node.role == Role::GenericContainer && node.name.contains(words))
    else {
        return;
    };
    for _ in 0..8 {
        if in_view(d) {
            return;
        }
        d.scroll_at(space.rect.center(), 1);
    }
}

/// Astra review of round 25: a message in the one message space below the save row is drawn and
/// reachable, not only in the space's accessible name. At 960, 1200 and 1920, from the space's top:
/// Tab to the space and step down with the arrow key, then from the top again scroll the pointer
/// over the space a line at a time. Each traversal must bring every painted row that holds `words` fully into view inside
/// that space, one time or another, and `undo` (when given) fully into view. Focus goes back to
/// exactly the widget that had it (or to none), with no click, and the step checks that the app
/// state did not change. Returns whether the keyboard and the pointer each reached it, per size.
pub(super) fn space_reaches(
    d: &mut Driver<'_>,
    words: &str,
    undo: Option<&str>,
) -> [(bool, bool); 3] {
    // Marks each row of the message that holds `words` once it is fully in view in the space.
    fn see_rows(d: &Driver<'_>, space: egui::Rect, words: &str, seen: &mut Vec<bool>) {
        d.each_painted_text(|text, clip| {
            let galley = &text.galley;
            let Some(byte) = galley.job.text.find(words) else {
                return;
            };
            if !clip.intersects(space) {
                return;
            }
            let first = galley.job.text[..byte].chars().count();
            let last = first + words.chars().count();
            let view = clip.intersect(space).expand(1.0);
            let mut at = 0;
            let mut index = 0;
            for row in &galley.rows {
                let count = row.glyphs.len() + usize::from(row.ends_with_newline);
                let (start, end) = (at, at + count);
                at = end;
                if end <= first || start >= last {
                    continue;
                }
                if seen.len() <= index {
                    seen.push(false);
                }
                // In view: most of the row's height (a wheel notch is larger than the space, so a
                // middle row can stop a point or two short of its edge). A row's rect can run past
                // the wrap width with its trailing space, so only its height is measured.
                let rect = row.rect().translate(text.pos.to_vec2());
                let shown = rect.intersect(view);
                seen[index] |= rect.is_positive()
                    && shown.is_positive()
                    && shown.height() >= rect.height() * 0.75;
                index += 1;
            }
        });
    }
    let undo_in_view = |d: &Driver<'_>| {
        undo.is_none_or(|undo| {
            d.nodes()
                .iter()
                .any(|node| node.name == undo && node.fully_in_view())
        })
    };
    let reach = |d: &mut Driver<'_>, space: egui::Rect, step: &dyn Fn(&mut Driver<'_>)| {
        let mut seen = Vec::new();
        let mut action = false;
        for _ in 0..12 {
            see_rows(d, space, words, &mut seen);
            action |= undo_in_view(d);
            if !seen.is_empty() && seen.iter().all(|row| *row) && action {
                return true;
            }
            step(d);
        }
        see_rows(d, space, words, &mut seen);
        !seen.is_empty() && seen.iter().all(|row| *row) && action
    };
    let focus = d.focus_id();
    let editor = d.editor_text();
    let dirty = d.app_mut().library.dirty();
    let reached = [SMALL, DEFAULT, LARGE].map(|size| {
        d.act_at_size(size, |d| {
            let Some(space) = d
                .nodes()
                .into_iter()
                .find(|node| node.role == Role::GenericContainer && node.name.contains(words))
            else {
                return (false, false);
            };
            let keyboard = d.tab_to(&space.name) && {
                d.press(egui::Key::PageUp);
                reach(d, space.rect, &|d| d.press(egui::Key::ArrowDown))
            };
            d.press(egui::Key::PageUp);
            let pointer = {
                d.scroll_up_at(space.rect.center(), 3);
                // A line at a time, as a touchpad scrolls: a 100-point wheel notch is larger than
                // the two-line space and can pass over a middle row (a limit of `fixed_part`).
                reach(d, space.rect, &|d| {
                    d.scroll_points_at(space.rect.center(), design::Type::Body.line())
                })
            };
            d.scroll_up_at(space.rect.center(), 3);
            (keyboard, pointer)
        })
    });
    d.restore_focus(focus);
    let unchanged =
        d.focus_id() == focus && d.editor_text() == editor && d.app_mut().library.dirty() == dirty;
    d.observe(
        "iter-25-L-space-reach-restores",
        unchanged,
        format!("Reaching {words:?} gives focus back to {focus:?} and changes no draft"),
    );
    reached
}

/// Round 26 FD2: the y of the wolf's kind line, the first line of its stat block, in the
/// Library reader at 960, 1200 and 1920.
fn kind_line_tops(d: &mut Driver<'_>) -> [Option<f32>; 3] {
    [SMALL, DEFAULT, LARGE].map(|size| {
        d.at_size(size, |d| {
            d.painted_rect_in("Medium · Beast · Unaligned", egui::Rect::EVERYTHING)
                .map(|rect| rect.top())
        })
    })
}

/// Round 24 FC4: the y of the Read tab, `Show file` on the file line and the reader's title, at
/// 960, 1200 and 1920. A result in the reader's result place moves none of them.
pub(super) fn reader_frame(d: &mut Driver<'_>, title: &str) -> Vec<Option<(f32, f32, f32)>> {
    [SMALL, DEFAULT, LARGE]
        .into_iter()
        .map(|size| {
            d.at_size(size, |d| {
                Some((
                    d.place_of("Read")?.top(),
                    d.place_of("Show file")?.top(),
                    reader_title(d, title)?.top(),
                ))
            })
        })
        .collect()
}

/// A World reader's title, painted in the reader pane.
fn title_rect(d: &Driver<'_>, title: &str) -> Option<egui::Rect> {
    d.painted_word_rect(title, reader_pane(d))
}

/// From the end of the painted title to Edit's icon, which sits `S` inside its target.
fn edit_gap(d: &Driver<'_>, title: &str) -> Option<f32> {
    let edit = d.place_of(&format!("Edit {title}"))?;
    Some(edit.left() + design::S - title_rect(d, title)?.right())
}

/// FD2: a World result is a message with its tone bar, below the file line.
fn result_below_file_line(
    d: &Driver<'_>,
    result: &str,
    file: &str,
    tone: egui::Color32,
) -> (bool, String) {
    let pane = reader_pane(d);
    let message = d.painted_rect_in(result, pane);
    let file_line = d.painted_rect_in(file, pane);
    let bar = message.is_some_and(|message| d.has_tone_bar(message, tone));
    (
        bar && message
            .zip(file_line)
            .is_some_and(|(message, file_line)| message.top() >= file_line.bottom()),
        format!(
            "{result:?} at {message:?} with a tone bar {bar}, below the file line {file:?} at {file_line:?}"
        ),
    )
}

/// Round 26 footer: `anchor`'s top at 960, 1200 and 1920.
fn tops_at_sizes(d: &mut Driver<'_>, anchor: &str) -> [Option<f32>; 3] {
    [SMALL, DEFAULT, LARGE]
        .map(|size| d.at_size(size, |d| d.place_of(anchor).map(|rect| rect.top())))
}

/// Round 26 footer: a pane's results, reasons and Undo sit in its footer, so `anchor` (what the
/// DM works on) keeps the top it had in `before` at 960, 1200 and 1920. When `result` is given,
/// it is in view at each size in the footer: below `anchor`, in the pane's lower part.
fn body_kept(
    d: &mut Driver<'_>,
    id: &str,
    anchor: &str,
    before: &[Option<f32>; 3],
    result: Option<&str>,
) {
    let now = tops_at_sizes(d, anchor);
    let kept = before.iter().zip(&now).all(|pair| match pair {
        (Some(before), Some(now)) => (before - now).abs() <= 0.5,
        _ => false,
    });
    let mut placed = Vec::new();
    if let Some(result) = result {
        for size in [SMALL, DEFAULT, LARGE] {
            placed.push(d.at_size(size, |d| {
                let found = d.place_of(result).filter(|_| d.in_view(result));
                // iter-27-A: the footer follows a short body, so the result is no longer in the
                // window's lower half; it is below the body, inside the pane's footer group.
                let in_footer = found.is_some_and(|found| {
                    d.nodes().into_iter().any(|node| {
                        node.role == Role::GenericContainer
                            && !node.name.is_empty()
                            && node.rect.contains_rect(found)
                    })
                });
                let below = in_footer
                    && found
                        .zip(d.place_of(anchor))
                        .is_some_and(|(found, anchor)| found.top() >= anchor.bottom());
                (size, found, below)
            }));
        }
    }
    d.observe(
        id,
        kept && placed.iter().all(|(_, _, below)| *below),
        format!(
            "{anchor} keeps its top {before:?} → {now:?}; {result:?} in the footer: {placed:?}"
        ),
    );
}

/// FD5 (round 20): a finished change is a success. Its words have the success tone's bar, and
/// no info bar.
pub(super) fn finished_change_tone(d: &Driver<'_>, words: &str) -> (bool, String) {
    let result = design::colors().result;
    let message = d.painted_rect_in(words, egui::Rect::EVERYTHING);
    let success = message.is_some_and(|message| d.has_tone_bar(message, result.success));
    let info = message.is_some_and(|message| d.has_tone_bar(message, result.info));
    (
        success && !info,
        format!("{words:?} at {message:?}: success bar {success}, info bar {info}"),
    )
}

/// FD5: one tone for one fault. The message above the roster has this tone's bar, and every
/// roster row that states the fault marks it once, with this tone's icon and colour.
/// Round 23 AE4: the tone bar beside a manifest fault's words, as `error` or `warning`.
fn manifest_fault_tone(d: &Driver<'_>, words: &str) -> Option<&'static str> {
    let rect = d.painted_rect_in(words, egui::Rect::EVERYTHING)?;
    let tones = design::colors().result;
    [("error", tones.error), ("warning", tones.warning)]
        .into_iter()
        .find(|(_, color)| d.has_tone_bar(rect, *color))
        .map(|(name, _)| name)
}

fn fault_tone(
    d: &Driver<'_>,
    message: &str,
    row_words: &str,
    (icon, color): (&str, egui::Color32),
) -> (bool, String) {
    let bar = d
        .painted_rect_in(message, egui::Rect::EVERYTHING)
        .is_some_and(|words| d.has_tone_bar(words, color));
    let rows = row_marks(d, row_words);
    let marked = rows.len() == 2
        && rows
            .iter()
            .all(|marks| marks.len() == 1 && marks[0].0 == icon && marks[0].1 == color);
    (
        bar && marked,
        format!("message {message:?} bar in {color:?}: {bar}; rows: {rows:?}"),
    )
}

/// The reader title's distance below `Show file` on the file line, at each captured size.
fn title_below_file_line(d: &mut Driver<'_>, title: &str) -> Vec<Option<f32>> {
    [SMALL, DEFAULT, LARGE]
        .into_iter()
        .map(|size| {
            d.at_size(size, |d| {
                let file = d.place_of("Show file")?;
                reader_title(d, title).map(|rect| rect.top() - file.bottom())
            })
        })
        .collect()
}

/// The Library header's file count, as painted (`2 files need fixing`), if any.
fn files_to_fix(d: &Driver<'_>) -> Option<String> {
    let mut found = None;
    d.each_painted_text(|text, clip| {
        let words = &text.galley.job.text;
        if clip.intersects(text.visual_bounding_rect())
            && (words.ends_with(" files need fixing") || words.ends_with(" file needs fixing"))
        {
            found = Some(words.clone());
        }
    });
    found
}

/// A kept result whose Undo is blocked: the colour of its words, and whether the reason shows
/// in view with its words starting at the result's words.
fn blocked_reason_shown(d: &Driver<'_>, words: &str) -> (Option<egui::Color32>, bool) {
    let message = d.painted_rect_in(words, egui::Rect::EVERYTHING);
    let reason = d.painted_rect_in(crate::content_workspace::UNDO_ENDED, egui::Rect::EVERYTHING);
    (
        d.painted_color(words),
        message.zip(reason).is_some_and(|(message, reason)| {
            (reason.left() - message.left()).abs() <= 1.0 && reason.top() >= message.bottom() - 1.0
        }),
    )
}

/// The marks in each visible row line that holds `row_words`: every painted part that is not in
/// the muted colour, with its colour. A row's detail line has ` · `; a message has none.
pub(super) fn row_marks(d: &Driver<'_>, row_words: &str) -> Vec<Vec<(String, egui::Color32)>> {
    let mut rows = Vec::new();
    d.each_painted_text(|text, clip| {
        let job = &text.galley.job;
        if job.text.contains(row_words)
            && job.text.contains(" · ")
            && clip.intersects(text.visual_bounding_rect())
        {
            rows.push(painted_marks(job));
        }
    });
    rows
}

/// Every part of painted words that is in neither the muted nor the text colour, with its colour.
fn painted_marks(job: &egui::text::LayoutJob) -> Vec<(String, egui::Color32)> {
    job.sections
        .iter()
        .filter(|section| {
            ![design::colors().muted, design::colors().text].contains(&section.format.color)
        })
        .map(|section| {
            (
                job.text[section.byte_range.clone()].trim().to_owned(),
                section.format.color,
            )
        })
        .collect()
}

/// World's list rows and header counts: each count's icon and words in its tone (the error
/// tone for notes that cannot be read), and as many marked rows as the count says.
/// `both`: the page must show both counts, as with one unreadable and one warning note.
pub(super) fn world_fault_marks(d: &Driver<'_>, both: bool) -> (bool, String) {
    let c = design::colors();
    let search = d
        .place_of("Search the world")
        .unwrap_or(egui::Rect::NOTHING);
    let list = egui::Rect::from_x_y_ranges(search.x_range(), search.bottom()..=f32::INFINITY);
    let mut errors = 0;
    let mut warnings = 0;
    let mut unmarked = Vec::new();
    let mut counts = Vec::new();
    d.each_painted_text(|text, clip| {
        let job = &text.galley.job;
        let rect = egui::Rect::from_min_size(text.pos, text.galley.size());
        if !clip.intersects(rect) {
            return;
        }
        if !list.contains(rect.center()) {
            // The page header's counts sit above the list's search field.
            if rect.bottom() <= search.top()
                && (job.text.ends_with("cannot be read") || job.text.contains("fixing"))
            {
                counts.push((job.text.clone(), painted_marks(job)));
            }
            return;
        }
        let marks = painted_marks(job);
        let first = job
            .text
            .chars()
            .next()
            .map(String::from)
            .unwrap_or_default();
        match marks.as_slice() {
            [(mark, colour)] if *mark == design::icons::ERROR && *colour == c.result.error => {
                errors += 1
            }
            [(mark, colour)] if *mark == design::icons::WARN && *colour == c.result.warning => {
                warnings += 1
            }
            _ if first == design::icons::WARN || first == design::icons::ERROR => {
                unmarked.push(job.text.clone())
            }
            _ => {}
        }
    });
    let number = |words: &str| {
        words
            .split_whitespace()
            .find_map(|word| word.parse::<usize>().ok())
            .unwrap_or(0)
    };
    let toned = |words: &str, icon: &str, colour: egui::Color32| {
        counts.iter().any(|(text, marks)| {
            text.contains(words)
                && marks.first().is_some_and(|(mark, _)| mark == icon)
                && marks.iter().all(|(_, painted)| *painted == colour)
        })
    };
    let unread = counts
        .iter()
        .find(|(text, _)| text.ends_with("cannot be read"))
        .map_or(0, |(text, _)| number(text));
    let fixing = counts
        .iter()
        .find(|(text, _)| text.contains("fixing"))
        .map_or(0, |(text, _)| number(text));
    let ok = (if both {
        unread > 0 && fixing > 0
    } else {
        unread + fixing > 0
    }) && (unread == 0 || toned("cannot be read", design::icons::ERROR, c.result.error))
        && (fixing == 0 || toned("fixing", design::icons::WARN, c.result.warning))
        && errors == unread
        && warnings == fixing
        && unmarked.is_empty();
    (
        ok,
        format!(
            "counts {counts:?}; error rows {errors}, warning rows {warnings}, marks in the row colour {unmarked:?}"
        ),
    )
}

/// Campaign's package rows in view (each second line says `chosen`), and the marks of each row
/// that has any.
fn package_row_marks(d: &Driver<'_>) -> (usize, Vec<Vec<(String, egui::Color32)>>) {
    let mut rows = 0;
    let mut marked = Vec::new();
    d.each_painted_text(|text, clip| {
        let job = &text.galley.job;
        if (job.text.contains("chosen ·") || job.text.ends_with(" chosen"))
            && !job.text.contains("entries chosen")
            && clip.intersects(text.visual_bounding_rect())
        {
            rows += 1;
            let marks = painted_marks(job);
            if !marks.is_empty() {
                marked.push(marks);
            }
        }
    });
    (rows, marked)
}

/// True when there is at least one such row and each has exactly one mark: the warning icon in
/// the warning colour.
fn warning_marked(rows: &[Vec<(String, egui::Color32)>]) -> bool {
    !rows.is_empty()
        && rows.iter().all(|marks| {
            marks.len() == 1
                && marks[0].0 == design::icons::WARN
                && marks[0].1 == design::colors().result.warning
        })
}

/// FD3: from the bottom of the painted file line to the top of the first prose line.
fn prose_gap(d: &Driver<'_>, file: &str, prose: &str) -> Option<f32> {
    let pane = reader_pane(d);
    let file_line = d.painted_rect_in(file, pane)?;
    Some(d.painted_rect_in(prose, pane)?.top() - file_line.bottom())
}

/// In Lookup, the gap from the reader's kind line to the first text below it (the prose).
pub(super) fn lookup_prose_gap(d: &Driver<'_>, title: &str, kind: &str) -> Option<f32> {
    let search = d.place_of(SEARCH_LOOKUP)?;
    let (width, height) = d.window_size();
    let pane = egui::Rect::from_min_max(
        egui::pos2(search.left() - 1.0, search.bottom()),
        egui::pos2(width, height),
    );
    let title = d.painted_word_rect(title, pane)?;
    let kind = d.painted_rect_in(
        kind,
        egui::Rect::from_min_max(
            egui::pos2(pane.left(), title.bottom()),
            egui::pos2(width, title.bottom() + design::XL),
        ),
    )?;
    // Round 27: when the title-line actions do not fit beside the title, they take one line
    // under the kind line, and the head ends at that line.
    let head = d
        .nodes()
        .into_iter()
        .filter(|node| {
            pane.contains(node.rect.center())
                && node.rect.top() >= kind.bottom()
                && node.rect.top() <= kind.bottom() + design::XL
                && (node.name.starts_with("Edit ")
                    || node.name == "Add to Log"
                    || node.name == "Open this session"
                    || node.name == "Open on the timeline"
                    || node.name.starts_with("Start a fight with "))
        })
        .map(|node| node.rect.bottom())
        .fold(kind.bottom(), f32::max);
    let mut first: Option<f32> = None;
    d.each_painted_text(|text, clip| {
        let rect = egui::Rect::from_min_size(text.pos, text.galley.size());
        if pane.contains(rect.center())
            && clip.intersects(rect)
            && rect.top() >= head
            && !text.galley.job.text.trim().is_empty()
        {
            first = Some(first.map_or(rect.top(), |top| top.min(rect.top())));
        }
    });
    Some(first? - head)
}

/// The run's words when an ended fight refuses `Add to fight` (`Run::add_to_fight_reason`).
const ADD_REFUSED: &str = "This encounter has ended. Undo the ending to continue.";

/// End the open fight from the run's footer, with no outcome.
fn end_fight(d: &mut Driver<'_>) {
    if !d.has("End encounter…") && d.has("Close menu") {
        d.click("Close menu");
    }
    d.click("End encounter…");
    d.click("End encounter");
    d.expect("Outcome: Undecided");
}

/// The NPC keeps both reference links beside a live fight.
pub fn npc_in_lookup(d: &mut Driver<'_>) {
    let modules = d.app_mut().campaigns.modules().to_vec();
    let campaign = d.app_mut().campaigns.active_campaign();
    let catalogue = crate::content_workspace::Catalogue::load(&modules, campaign.as_ref()).unwrap();
    let guard = catalogue
        .entries
        .values()
        .find(|i| i.entry.name == "Guard")
        .unwrap()
        .entry
        .id;
    let place = entry(d, "Locations/Watch House.md", "Watch House", "location", "");
    entry(
        d,
        "NPCs/Rook Ambry.md",
        "Rook Ambry",
        "npc",
        &format!("  creature: {{id: {guard}}}\n  in: {{id: {place}}}\n"),
    );
    reload(d);
    resume_run_from_cold_start(d);
    look_up(d, "Rook Ambry");
    d.wait_for_text("Stat block");
    d.expect("Watch House");
    d.observe_both(
        "iter-1-A-single-lookup-result",
        "The one open Lookup result does not repeat its row above the reader",
        |d| !d.has("Rook Ambry · NPC · Campaign"),
    );
    d.observe(
        "W2",
        d.in_view(NEXT_TURN) && d.has("Guard") && !d.has("Change Stat block"),
        "The compact NPC reader shows its stat block and place beside the run",
    );
    // Round 24 FI6: the one match is the entry on screen, so no `Show 1 result` line shows.
    for size in [SMALL, DEFAULT, LARGE] {
        let words = d.at_size(size, super::lookup_results_words);
        d.observe(
            "iter-24-D-FI6",
            words.is_none() && !d.has("Show the results") && d.has_label("Rook Ambry"),
            format!("At {size:?}, the open single result offers {words:?}"),
        );
    }
    // Round 23 FI6, FJ5: Back keeps the title line's height, so the search field keeps its place.
    let fields_before: Vec<_> = [SMALL, DEFAULT, LARGE]
        .map(|size| d.at_size(size, |d| d.place_of(SEARCH_LOOKUP)))
        .into();
    let back_before = d.has("Back");
    d.state("npc-with-stat-block-and-place", Sizes::All);
    d.begin_task();
    d.click("Guard");
    d.wait_for_text("ARMOR CLASS");
    d.audit_goal_within("W2", "read an NPC stat block beside the run", 1);
    for (size, before) in [SMALL, DEFAULT, LARGE].into_iter().zip(fields_before) {
        let after = d.at_size(size, |d| d.place_of(SEARCH_LOOKUP));
        d.observe(
            "iter-23-D-FI6",
            !back_before
                && d.has("Back")
                && before
                    .zip(after)
                    .is_some_and(|(before, after)| (before.top() - after.top()).abs() < 0.5),
            format!("The search field stays at {before:?} → {after:?} when Back shows at {size:?}"),
        );
    }
    d.state("npc-stat-block-link", Sizes::All);
    // Round 27 FJ5: during a run, a creature's reader has `Add to fight` with its word, as an
    // NPC's has. One press adds one Guard through the run's own add; the run's Undo takes it
    // away again.
    let add = "Add Guard to the fight";
    let shown: Vec<_> = [SMALL, DEFAULT, LARGE]
        .map(|size| {
            d.at_size(size, |d| {
                d.place_of(add).is_some_and(|rect| {
                    rect.width() > design::Density::Read.control() + 0.5 && d.in_view(add)
                })
            })
        })
        .into();
    let guards = |d: &Driver<'_>| {
        roster_places(d)
            .iter()
            .filter(|(name, _)| name.starts_with("Guard"))
            .count()
    };
    let before = guards(d);
    d.click(add);
    let after = guards(d);
    d.observe(
        "iter-27-D-FJ5",
        shown.iter().all(|shown| *shown)
            && after == before + 1
            && d.has_containing("Undo: add Guard")
            && d.has("Back"),
        format!("Add to fight shows with its word at {shown:?}; Guards {before} → {after}, and Lookup keeps Guard"),
    );
    d.click_containing("Undo: add Guard");
    d.observe(
        "iter-27-D-FJ5-undo",
        guards(d) == before,
        format!("Undo: add Guard takes the Guard back: {} Guards", guards(d)),
    );
    // Review of PR 300: the run's refusal belongs to that run. An ended fight refuses; after
    // `Undo: end encounter` the same press adds the Guard, and the refusal is gone.
    end_fight(d);
    d.click(add);
    let refused = d.visible_text(ADD_REFUSED) && guards(d) == before;
    d.click_containing("Undo: end encounter");
    d.click(add);
    let joined = guards(d);
    d.observe(
        "iter-27-D-FJ5-refusal",
        refused && joined == before + 1 && !d.visible_text(ADD_REFUSED),
        format!(
            "Refused while ended {refused}; after Undo of the ending, Guards {before} → {joined}, refusal shown {}",
            d.visible_text(ADD_REFUSED)
        ),
    );
    d.click_containing("Undo: add Guard");
    d.observe(
        "iter-27-D-FJ5-refusal-undo",
        guards(d) == before && !d.visible_text(ADD_REFUSED) && d.in_view(add),
        "The run's Undo takes the Guard back, and no refusal comes back",
    );
    d.click("Back");
    d.click("Watch House");
    d.wait_for_text("Rook Ambry");
    // FT2: a location with no set values draws no value row in Lookup, so its prose has the
    // gap below the kind line that a spell (no value block) has. Compared with Bless below.
    let place_gaps: Vec<_> = [SMALL, DEFAULT, LARGE]
        .into_iter()
        .map(|size| {
            d.at_size(size, |d| {
                lookup_prose_gap(d, "Watch House", "Location · Campaign")
            })
        })
        .collect();
    d.state("npc-place-link", Sizes::All);

    // Rook moves and makes a promise. The DM writes both down without leaving the fight.
    entry(d, "Locations/North Gate.md", "North Gate", "location", "");
    reload(d);
    d.click("Back");
    d.wait_for_text("Watch House");
    let path = root(d).join("NPCs/Rook Ambry.md");
    let before = fs::read_to_string(&path).unwrap();
    let turn = |d: &mut Driver<'_>| {
        let strip = d.app_mut().encounters.run().unwrap().strip();
        (strip.round, strip.current)
    };
    let at_start = turn(d);
    d.begin_task();
    for size in [SMALL, DEFAULT, LARGE] {
        let named = d.at_size(size, |d| {
            d.expect("Edit Rook Ambry");
            let rect = d.place_of("Edit Rook Ambry").unwrap();
            let words = d.painted_in(rect);
            // X32 of app round 2: `Add to Log` has its own glyph, a list with a plus, in its
            // word form and its icon form, not the bare `+` of `Add to fight`.
            let log = d.place_of("Add to Log").map(|rect| d.painted_in(rect));
            words.contains("Edit")
                && words.contains(design::icons::EDIT)
                && log.is_some_and(|log| {
                    log.contains(design::icons::ADD_TO_LOG) && !log.contains(design::icons::ADD)
                })
        });
        d.observe(
            "iter-6-D-FD7",
            named,
            format!(
                "Lookup's Edit action has its pencil and word, and Add to Log its own glyph, at {size:?}"
            ),
        );
    }
    d.expect("Edit Rook Ambry");
    // Round 26 FJ3: `Add to fight` keeps its place in Lookup while the save result shows and
    // after it goes; the result sits in Lookup's footer.
    let add = tops_at_sizes(d, "Add to fight");
    d.click("Edit Rook Ambry");
    d.wait_for_text("Save changes");
    d.click("Change In");
    d.type_into("Find a location", "North");
    d.click("North Gate");
    d.type_into_editor("\nNever names the captain. Owes Nell a favour.\n");
    let same_turn = turn(d) == at_start;
    d.observe(
        "W2",
        d.in_view(NEXT_TURN) && d.visible_text("Unsaved") && same_turn,
        "Editing an NPC in Lookup leaves the run's turn as it was",
    );
    // Round 24 FD7 in the narrow Lookup form: the reason is whole on its own line below the
    // save row, not squeezed between Discard and Save.
    for size in [SMALL, DEFAULT, LARGE] {
        let (whole, detail) = d.at_size(size, |d| {
            let reason = d.place_of(crate::encounters::draft_switch_reason());
            let save = d.place_of("Save changes");
            // The pane scrolls, so the line may sit below the window's edge at 960.
            (
                d.has(crate::encounters::draft_switch_reason())
                    && reason.zip(save).is_some_and(|(reason, save)| {
                        reason.height() <= design::Type::Small.line() + 1.0
                            && (reason.right() <= save.left()
                                || reason.top() >= save.bottom() - 0.5)
                    }),
                format!("reason {reason:?}, Save {save:?}"),
            )
        });
        d.observe(
            "iter-24-D-FD7-lookup",
            whole,
            format!("The switch reason is whole on one line at {size:?}: {detail}"),
        );
    }
    // Round 24 AI1: Stat block, In and Part of keep one Change column, an `L` gap after the
    // widest value.
    for size in [SMALL, DEFAULT, LARGE] {
        let (column, detail) = d.at_size(size, |d| {
            let changes: Vec<_> = ["Change Stat block", "Change In", "Change Part of"]
                .into_iter()
                .map(|name| d.place_of(name))
                .collect();
            (
                changes.iter().all(Option::is_some)
                    && changes
                        .iter()
                        .flatten()
                        .all(|change| (change.left() - changes[0].unwrap().left()).abs() < 0.5),
                format!("{changes:?}"),
            )
        });
        d.observe(
            "iter-24-D-AI1",
            column,
            format!("Every Change in the Lookup edit form starts at one x at {size:?}: {detail}"),
        );
    }
    // R3f X50: the save row and the reason under it end inside the pane, whole.
    d.state_prepared("npc-edit-form-in-lookup", Sizes::All, |d| {
        let reason = crate::encounters::draft_switch_reason();
        d.observe(
            "r3f-X50-reason-whole",
            d.visible_text(reason),
            format!("`{reason}` is painted whole under the save row"),
        );
    });
    for size in [SMALL, DEFAULT, LARGE] {
        let one_primary = d.at_size(size, |d| {
            d.expect("Save changes");
            d.expect(NEXT_TURN);
            d.expect("Discard");
            d.expect("Unsaved");
            d.control_frame("Save changes").is_some_and(|frame| {
                frame.stroke.color == design::colors().field_border
                    && frame.fill != design::colors().accent_fill
            }) && d.control_frame(NEXT_TURN).is_some_and(|frame| {
                frame.fill == design::colors().accent_fill
                    || frame.fill == design::colors().accent_hover
            })
        });
        d.observe(
            "iter-6-D-ST1",
            one_primary,
            format!("Next turn owns primary while NPC Save stays secondary at {size:?}"),
        );
        let shared_edge = d.at_size(size, |d| {
            for name in ["Status", "Text", "Text field", "Save changes"] {
                d.expect(name);
            }
            let nodes = d.nodes();
            let status = nodes
                .iter()
                .find(|node| node.name == "Status" && node.role == Role::TextInput)
                .expect("Status input")
                .rect;
            let text = d.place_of("Text field").expect("Text field frame");
            let save = d.place_of("Save changes").unwrap();
            (status.right() - text.right()).abs() <= 1.0
                && (status.right() - save.right()).abs() <= 1.0
        });
        d.observe(
            "iter-8-D-FD11-scroll",
            shared_edge,
            format!("The scrolling Text field shares Status and Save's right edge at {size:?}"),
        );
    }
    d.click("Save changes");
    d.wait_for_text("Changes saved.");
    d.audit_goal_within("W2", "change an NPC's place and text beside the run", 9);
    let saved = fs::read_to_string(&path).unwrap();
    assert!(saved.contains("North Gate"), "{saved}");
    assert!(saved.ends_with("Owes Nell a favour.\n"), "{saved}");
    d.observe(
        "W2",
        d.has("North Gate")
            && d.visible_text("Never names the captain. Owes Nell a favour.")
            && !d.has("Change In"),
        "After Save, Lookup reads the NPC again with the new place and text",
    );
    body_kept(
        d,
        "iter-26-D-FJ3",
        "Add to fight",
        &add,
        Some("Undo changes"),
    );
    d.state("npc-edit-saved-in-lookup", Sizes::All);
    d.click("Undo changes");
    d.wait_for_text("Watch House");
    assert_eq!(fs::read_to_string(&path).unwrap(), before);
    body_kept(d, "iter-26-D-FJ3-undo", "Add to fight", &add, None);

    // FT2, the other half: Bless has no value block. Its prose sets the gap that Watch House
    // must match.
    look_up_again(d, "bless");
    d.wait_for_text("Spell · Level 1 · Campaign");
    let spell_gaps: Vec<_> = [SMALL, DEFAULT, LARGE]
        .into_iter()
        .map(|size| {
            d.at_size(size, |d| {
                lookup_prose_gap(d, "Bless", "Spell · Level 1 · Campaign")
            })
        })
        .collect();
    for ((size, place), spell) in [SMALL, DEFAULT, LARGE]
        .into_iter()
        .zip(place_gaps)
        .zip(spell_gaps.iter().copied())
    {
        d.observe(
            "iter-18-E-FT2",
            place
                .zip(spell)
                .is_some_and(|(place, spell)| (place - spell).abs() <= 1.0),
            format!(
                "At {size:?}, Lookup's Watch House has {place:?} px from its kind line to its prose, Bless {spell:?} px"
            ),
        );
    }
    // FT2, plain notes: a note with no front matter keeps the same gap in Lookup.
    fs::create_dir_all(root(d).join("Notes")).unwrap();
    fs::write(
        root(d).join("Notes/Harbour Rumours.md"),
        "# Harbour Rumours\n\nThe ferry master hears every rumour first.\n",
    )
    .unwrap();
    reload(d);
    look_up_again(d, "harbour rumours");
    d.wait_for_text("The ferry master hears every rumour first.");
    for (size, spell) in [SMALL, DEFAULT, LARGE].into_iter().zip(spell_gaps) {
        let note = d.at_size(size, |d| {
            lookup_prose_gap(d, "Harbour Rumours", "World note · Campaign")
        });
        d.observe(
            "iter-18-E-FT2-note",
            note.zip(spell)
                .is_some_and(|(note, spell)| (note - spell).abs() <= 1.0),
            format!(
                "At {size:?}, Lookup's plain note Harbour Rumours has {note:?} px from its kind line to its prose, Bless {spell:?} px"
            ),
        );
    }
    // Row 17 of the session-cycle design: while Lookup holds an unsaved generated NPC, an entry
    // that opens shows under one row, `Unsaved NPC: <name> · Show`. Before, the card hid it.
    d.click("Clear");
    d.click("Generate NPC");
    d.expect("Save NPC");
    let npc = d.app().lookup.generated_name().unwrap().to_owned();
    let row = format!("{}: {npc}", crate::world::page::UNSAVED_NPC);
    d.observe(
        "P1c-row-17",
        !d.has(&row),
        "While the NPC's card is the view, no row says so",
    );
    look_up_again(d, "Watch House");
    d.wait_for_text("Location · Campaign");
    for size in [SMALL, DEFAULT, LARGE] {
        let (above, detail) = d.at_size(size, |d| {
            let held = d.place_of(&row);
            let kind = d.painted_rect_in("Location · Campaign", egui::Rect::EVERYTHING);
            (
                d.in_view("Show")
                    && !d.has("Save NPC")
                    && held
                        .zip(kind)
                        .is_some_and(|(held, kind)| held.bottom() <= kind.top()),
                format!("row {held:?}, the entry's kind line {kind:?}"),
            )
        });
        d.observe(
            "P1c-row-17",
            above,
            format!("At {size:?}, the opened entry shows under the NPC's one row: {detail}"),
        );
    }
    d.state("npc-held-under-an-entry", Sizes::All);
    d.click("Rook Ambry");
    d.wait_for_text("Watch House");
    d.click("Show");
    d.expect("Save NPC");
    d.observe(
        "P1c-row-17",
        !d.has(&row) && d.has("Back"),
        "Show makes the NPC's card the view again",
    );
    // Back is as before: it opens the entry that was open, under the NPC's row.
    d.click("Back");
    d.wait_for_text("Location · Campaign");
    d.observe(
        "P1c-row-17",
        d.has(&row) && !d.has("Save NPC"),
        "Back opens the last entry under the NPC's row, and the NPC stays held",
    );
    // Round 28 FJ6: the rail names the NPC that Lookup holds, also with Lookup closed, and its
    // status opens Lookup again, where the NPC's row is.
    let in_rail = format!(
        "{}: {npc}. Show it in Lookup.",
        crate::world::page::UNSAVED_NPC
    );
    d.observe(
        "iter-28-C-FJ6",
        d.has(&in_rail),
        "With Lookup open, the rail names the NPC it holds",
    );
    d.click(NAV_LOOKUP);
    d.observe(
        "iter-28-C-FJ6",
        !d.has(&row) && d.has(&in_rail),
        "With Lookup closed, the rail still names the NPC that waits to be saved",
    );
    d.click(&in_rail);
    d.observe(
        "iter-28-C-FJ6",
        d.in_view("Save NPC") && !d.has(&row) && d.has(&in_rail),
        "The rail's status opens Lookup on the NPC's card",
    );
    // Review of PR 323: from Trackers too, open or closed. The status leaves Trackers and shows
    // the NPC's card.
    d.press_ctrl(Key::T);
    let on_trackers = d.app().lookup.on_trackers();
    d.click(&in_rail);
    let open_case = (
        on_trackers,
        d.app().lookup.on_trackers(),
        d.in_view("Save NPC"),
    );
    d.press_ctrl(Key::T);
    d.click(NAV_LOOKUP);
    let closed = !d.app().lookup.open;
    d.click(&in_rail);
    let closed_case = (closed, d.app().lookup.on_trackers(), d.in_view("Save NPC"));
    d.observe(
        "iter-28-C-FJ6",
        open_case == (true, false, true) && closed_case == (true, false, true),
        format!(
            "(on Trackers before, on Trackers after, the card in view): with Lookup on Trackers {open_case:?}; Lookup closed after Trackers {closed_case:?}"
        ),
    );
    // Closing the app names the NPC that Lookup holds, as for World's.
    let unsaved = format!("The generated NPC \u{201c}{npc}\u{201d} is not saved.");
    d.request_close();
    d.observe(
        "iter-28-C-lookup-npc-close",
        d.in_view_containing(&unsaved) && d.in_view("Keep editing"),
        "The close dialog names the NPC that Lookup holds",
    );
    if d.has("Keep editing") {
        d.click("Keep editing");
    }
    // Section 7, recorded as it is: Lookup's place is not kept across a restart, and the held
    // NPC goes with it.
    d.restart();
    d.click(NAV_LOOKUP);
    d.observe(
        "P1c-limits",
        !d.has_containing(crate::world::page::UNSAVED_NPC)
            && !d.has("Back")
            && !d.visible_text("Location · Campaign"),
        "After a restart, Lookup opens with no entry, no trail and no held NPC",
    );

    // F3 of the session-cycle design, rows 9 and 21 of its section 2.5: with no run open, an
    // NPC's and a package creature's reader has a quiet Start fight after Edit, in the room the
    // title leaves. It starts `Fight with <name>`, and after an untouched Setup is closed, the
    // same click starts that encounter again.
    if d.app().encounters.run().is_some() {
        if !d.has(CLOSE_RUN) {
            d.click(BACK_TO_RUN);
        }
        // Review of PR 300: a refusal ends with its run. Lookup keeps Guard open while the
        // ended run refuses it and then closes.
        if !d.has(SEARCH_LOOKUP) {
            d.click(NAV_LOOKUP);
        }
        look_up_again(d, "Guard");
        d.click("Guard · Creature · CR 1/8");
        d.wait_for_text("ARMOR CLASS");
        end_fight(d);
        d.click("Add Guard to the fight");
        let refused = d.visible_text(ADD_REFUSED);
        close_run(d);
        d.observe(
            "iter-27-D-FJ5-refusal-close",
            refused
                && !d.visible_text(ADD_REFUSED)
                && d.visible_text("ARMOR CLASS")
                && d.has("Start a fight with Guard"),
            format!("The ended run refused ({refused}); Close run ends the refusal while Lookup keeps Guard"),
        );
    }
    let start = "Start a fight with Rook Ambry";
    let show = |d: &mut Driver<'_>, name: &str| {
        if !d.has(SEARCH_LOOKUP) {
            d.click(NAV_LOOKUP);
        }
        look_up_again(d, name);
    };
    show(d, "Rook Ambry");
    d.wait_for_text("Watch House");
    for size in [SMALL, DEFAULT, LARGE] {
        let (placed, detail) = d.at_size(size, |d| {
            let (log, start, order) = reader_header(d, "Rook Ambry");
            // Round 27 FC4, FJ3: at every size, also 960, every action shows with its word.
            (
                start.is_some() && log.is_some() && order,
                format!("Add to Log {log:?}, Start fight {start:?}"),
            )
        });
        d.observe(
            "T12.12",
            placed,
            format!(
                "At {size:?}, the NPC's Start fight follows Edit, or Add to Log next to it, by XS: {detail}"
            ),
        );
    }
    d.state("npc-start-fight-in-lookup", Sizes::All);
    show(d, "Guard");
    d.click("Guard · Creature · CR 1/8");
    d.wait_for_text("ARMOR CLASS");
    let package = d.at_size(DEFAULT, |d| {
        d.has("Start a fight with Guard") && !d.has_containing("Edit Guard")
    });
    d.observe(
        "T12.12",
        package,
        "A package creature's reader, which has no Edit, has Start fight",
    );
    d.state("package-creature-start-fight", Sizes::All);
    show(d, "Rook Ambry");
    d.wait_for_text("Watch House");
    d.set_base(DEFAULT);
    d.begin_task();
    d.click(start);
    d.expect(&format!("{RUN_HEADING}Fight with Rook Ambry"));
    d.audit_goal_within(
        "T12.12",
        "start a fight that nobody planned from a reader",
        1,
    );
    let encounters = root(d).join("Encounters");
    let count = || fs::read_dir(&encounters).unwrap().count();
    let before = count();
    close_run(d);
    show(d, "Rook Ambry");
    d.wait_for_text("Watch House");
    d.click(start);
    d.expect(&format!("{RUN_HEADING}Fight with Rook Ambry"));
    d.observe(
        "T12.12",
        count() == before && !encounters.join("Fight with Rook Ambry 2.md").exists(),
        "After Close run, Start fight starts the same encounter, and no second one is saved",
    );
    close_run(d);
}

/// AD3: a result's Undo follows its message, drawn by `undo_message`, at each size. It has the
/// undo icon, sits on the message line after the words, or starts the next line with its words
/// at the message words and its icon in the message's icon column, and its press area is clear.
fn undo_follows_message(d: &mut Driver<'_>, id: &str, message: &str, undo: &str) {
    for size in [SMALL, DEFAULT, LARGE] {
        let (words, icon, target, clear) = d.at_size(size, |d| {
            let target = d.place_of(undo);
            let mut words = None;
            let mut icon = None;
            d.each_painted_text(|text, clip| {
                let rect = egui::Rect::from_min_size(text.pos, text.galley.size());
                if text.galley.job.text == message && clip.intersects(rect) {
                    words = Some(rect);
                }
                if target.is_some_and(|target| target.contains(rect.center()))
                    && text.galley.job.text == design::icons::UNDO
                {
                    icon = Some(rect);
                }
            });
            let clear = target.map_or_else(
                || Err(format!("{undo} is not in view")),
                |target| super::press_area_clear(d, target, undo, None),
            );
            (
                words,
                icon,
                target
                    .and_then(|target| super::words_left_in(d, target).map(|left| (left, target))),
                clear,
            )
        });
        let placed = words
            .zip(icon)
            .zip(target)
            .is_some_and(|((words, icon), (left, target))| {
                if target.top() >= words.bottom() - 1.0 {
                    (left - words.left()).abs() <= 1.0 && icon.right() <= words.left()
                } else {
                    (target.center().y - words.center().y).abs() <= design::XS
                        && icon.left() > words.right()
                        && left > icon.right()
                }
            });
        d.observe(
            id,
            placed && clear.is_ok(),
            format!(
                "At {size:?}, {undo} {target:?} with its icon {icon:?} follows the message {message:?} {words:?}; press area: {clear:?}"
            ),
        );
    }
}

/// A rename on disk keeps an identity-based package link working.
pub fn package_rename(d: &mut Driver<'_>) {
    let package = d.root.join("modules/ashen-marches");
    let (entries, _) = splat::content::read_directory(&package).unwrap();
    let target = entries.iter().find(|e| e.name == "Ashfall").unwrap();
    let path = package.join("Rules/Ash Exposure.md");
    let original = fs::read_to_string(&path).unwrap() + "\nSee [[Cinder Wolf]].\n";
    let reading = serde_yaml_ng::to_value(serde_json::json!({"title": "Ash Exposure", "summary": "The watch knows this spell.", "related": { target.id.to_string(): "Read the watch spell" }})).unwrap();
    fs::write(
        &path,
        crate::content_workspace::patch_fields(&original, &[("splat_reading", Some(reading))])
            .unwrap(),
    )
    .unwrap();
    go_library(d);
    d.open_combo_at(LIBRARY_PACKAGE_DROPDOWN);
    d.click(MODULE);
    d.click_starting("Ash Exposure · ");
    d.scroll_until(READER, "Read the watch spell", 12);
    d.scroll_at(READER, 1);
    d.wait_for_text("Read the watch spell");
    d.state("before-linked-entry-rename", Sizes::All);
    let old_path = package.join(&target.path);
    let renamed = crate::content_workspace::patch_fields(
        &fs::read_to_string(&old_path).unwrap(),
        &[("splat_name", Some("Watch Ashfall".into()))],
    )
    .unwrap();
    fs::write(package.join("Spells/Watch Ashfall.md"), renamed).unwrap();
    fs::remove_file(old_path).unwrap();
    d.click("Packages");
    d.click("Reload from disk");
    d.scroll_until(READER, "Read the watch spell", 12);
    d.scroll_at(READER, 1);
    d.wait_for_text("Read the watch spell");
    d.click("Read the watch spell");
    d.wait_for_text("Watch Ashfall");
    d.observe_both(
        "iter-16-E-FD7-alone",
        "Without a duplicate warning, the heading warning still follows the reader title",
        |d| {
            reader_title(d, "Watch Ashfall")
                .zip(d.place_of("Use Watch Ashfall"))
                .is_some_and(|(title, fix)| fix.top() > title.bottom())
        },
    );
    d.state("linked-entry-renamed-on-disk", Sizes::All);
    let warning_left = d
        .painted_word_rect(
            "The first heading says Ashfall, but the name is Watch Ashfall.",
            egui::Rect::EVERYTHING,
        )
        .map(|words| words.left());
    let renamed_path = package.join("Spells/Watch Ashfall.md");
    assert!(
        !fs::read_to_string(&renamed_path)
            .unwrap()
            .contains("# Watch Ashfall")
    );
    d.click("Use Watch Ashfall");
    d.wait_for_text("Undo heading change");
    // An unsaved fix changes every later step, so this check stops the scenario on failure.
    let saved = fs::read_to_string(&renamed_path)
        .unwrap()
        .contains("# Watch Ashfall");
    assert!(
        saved,
        "iter-16-E-FD7-saved: Use Watch Ashfall in the reader did not save the fixed heading"
    );
    d.observe(
        "iter-16-E-FD7-saved",
        saved,
        "Use Watch Ashfall in the reader saves the fixed heading to the file",
    );
    d.observe_both(
        "iter-16-E-FD7-undo-place",
        "Undo heading change takes the warning's place after the reader title",
        |d| {
            reader_title(d, "Watch Ashfall")
                .zip(d.place_of("Undo heading change"))
                .is_some_and(|(title, undo)| undo.top() > title.bottom())
        },
    );
    // AD3: the fix's result is a message, and its Undo follows it with its icon. The message
    // words start at the reader text edge, as the warning's did.
    undo_follows_message(
        d,
        "iter-19-E-AD3-heading",
        "The first heading now says Watch Ashfall.",
        "Undo heading change",
    );
    let result_left = d
        .painted_word_rect(
            "The first heading now says Watch Ashfall.",
            egui::Rect::EVERYTHING,
        )
        .map(|words| words.left());
    d.observe(
        "iter-19-E-AD3-heading-edge",
        result_left
            .zip(warning_left)
            .is_some_and(|(result, warning)| (result - warning).abs() <= 1.0),
        format!(
            "The heading result's words start at {result_left:?}, where the warning's words started ({warning_left:?})"
        ),
    );
    d.state("renamed-entry-heading-fixed", Sizes::All);
    d.click("Undo heading change");
    d.wait_for_text("Use Watch Ashfall");
    d.observe(
        "iter-19-E-AD3-heading-undone",
        !d.has_containing("The first heading now says"),
        "After Undo heading change, its result message goes away and the warning comes back",
    );
    assert!(
        fs::read_to_string(package.join("Spells/Watch Ashfall.md"))
            .unwrap()
            .contains("# Ashfall")
    );
    d.click("Use Watch Ashfall");

    search_library(d, "Cinder Wolf");
    d.click_starting("Cinder Wolf · ");
    // Round 26 FD2: the stat block's kind line with no message in the reader.
    let clean_body = kind_line_tops(d);
    d.click(EDIT_MARKDOWN);
    let draft = d.editor_text().unwrap();
    // Round 25 AC3, FC1, SC2: the editor's place with nothing in the message space.
    let clean_tops = editor_frame_tops(d);
    // AD3: a stat fix's result is a message, and its Undo follows it with its icon.
    // The DM types a Hit Points line that disagrees with the stat block (22). The warning and
    // the result use the DM's words, not the schema key.
    let typed = format!("{draft}\nHit Points 24 (4d8 + 4)\n");
    d.replace_text_in("Markdown editor", &typed);
    d.settle();
    d.expect("Use 24");
    let warning_tops = editor_frame_tops(d);
    // Astra review of round 25: the warning keeps its consequence, the value the fight uses, and
    // the wrapped warning and `Use 24` are drawn and reached in the message space at each size.
    let stat_reach = space_reaches(d, "The fight uses 22.", Some("Use 24"));
    let stat_reached = stat_reach
        .iter()
        .all(|(keyboard, pointer)| *keyboard && *pointer);
    d.observe(
        "iter-19-E-AD3-stat-words",
        d.has("Hit Points line says 24; stat block says 22. The fight uses 22.")
            && stat_reached
            && !d.has_containing("max_hp"),
        format!(
            "The stat warning names Hit Points and the stat block, not max_hp, and says the fight uses 22; reached by (keyboard, pointer) at 960, 1200, 1920: {stat_reach:?}"
        ),
    );
    assert!(!d.has_containing("The stat block now says"));
    // The warning wraps at 960, so `Use 24` is below the first two lines: the wheel over the
    // message space brings it into view, as a DM would scroll there.
    reveal_in_space(d, "Hit Points line says 24", "Use 24");
    d.click("Use 24");
    d.wait_for_text("Undo stat change");
    let fixed = d.editor_text().unwrap();
    assert_ne!(fixed, typed, "Use 24 did not change the draft");
    undo_follows_message(
        d,
        "iter-19-E-AD3-stat",
        "The stat block now says Hit Points 24.",
        "Undo stat change",
    );
    d.state("stat-fix-undo-message", Sizes::All);
    // Round 25 AC3, FC1, SC2 and the message-space contract: the stat warning, its result, a
    // reason that hides the result, Discard, Undo discard and the result again all keep the
    // editor still. The hidden result comes back while its owner (this draft) lives.
    let result_tops = editor_frame_tops(d);
    let invalid = fixed.replacen("splat_name: Cinder Wolf", "splat_name: [", 1);
    assert_ne!(invalid, fixed);
    d.replace_editor_text(&invalid);
    d.wait_for_text("Invalid");
    let reason_hides = !d.has("Undo stat change");
    let reason_tops = editor_frame_tops(d);
    d.click("Discard");
    d.expect("Undo discard of Cinder Wolf");
    let discard_tops = editor_frame_tops(d);
    d.click("Undo discard of Cinder Wolf");
    assert_eq!(d.editor_text().as_deref(), Some(invalid.as_str()));
    let undo_discard_tops = editor_frame_tops(d);
    let still_hidden = !d.has("Undo stat change");
    d.replace_editor_text(&fixed);
    d.wait_for_text("Undo stat change");
    let back_tops = editor_frame_tops(d);
    let all = [
        warning_tops,
        result_tops,
        reason_tops,
        discard_tops,
        undo_discard_tops,
        back_tops,
    ];
    d.observe(
        "iter-25-L-AC3-editor-still",
        clean_tops.iter().all(Option::is_some) && all.iter().all(|tops| *tops == clean_tops),
        format!("The editor top is {clean_tops:?} with no message and in every state: {all:?}"),
    );
    d.observe(
        "iter-25-L-message-space-result-returns",
        reason_hides && still_hidden && d.has("Undo stat change"),
        "A reason hides the stat result; the result comes back with its own draft",
    );
    d.click("Undo stat change");
    d.settle();
    d.expect("Use 24");
    let restored = d.editor_text().unwrap();
    d.observe(
        "iter-19-E-AD3-stat-undone",
        !d.has_containing("The stat block now says") && restored == typed,
        "Undo stat change gives back the draft before the fix, and its result goes away",
    );
    let renamed =
        crate::content_workspace::patch_fields(&draft, &[("splat_name", Some("Ash Wolf".into()))])
            .unwrap()
            .replace("# Cinder Wolf", "# Ash Wolf");
    d.replace_text_in("Markdown editor", &renamed);
    d.click("Save");
    d.wait_for_text("Update 1 link");
    // Astra review of round 24: the offer and its result belong to the renamed entry (A, Ash
    // Wolf) and show in its reader only.
    let ash_exposure = fs::read_to_string(&path).unwrap();
    search_library(d, "Ash Exposure");
    d.click_starting("Ash Exposure · ");
    d.observe(
        "iter-24-L-link-owner-offer-hidden",
        d.visible_text("The watch knows this spell.")
            && !d.has_containing("Change links from")
            && !d.has_containing("Update 1 link"),
        "Another entry's reader does not show the rename's offer",
    );
    search_library(d, "Ash Wolf");
    d.click_starting("Ash Wolf · ");
    d.wait_for_text("Update 1 link");
    d.state("renamed-creature-offers-link-update", Sizes::All);
    // Round 24 FC4: the offer and its result sit in the reader's result place, below the title.
    let with_offer = reader_frame(d, "Ash Wolf");
    let body_with_offer = kind_line_tops(d);
    // Round 23 AE2: the offer names the old and the new name, so the DM can check the words
    // before the write.
    let offer = d.painted_word_rect(
        "Change links from “Cinder Wolf” to “Ash Wolf”.",
        egui::Rect::EVERYTHING,
    );
    d.observe(
        "iter-23-L-AE2",
        offer.is_some() && d.visible_text("Update 1 link"),
        "The link offer names both names, and its action keeps the count",
    );
    assert_eq!(
        fs::read_to_string(&path).unwrap(),
        ash_exposure,
        "the offer writes nothing"
    );
    d.click("Update 1 link");
    d.wait_for_text("Undo link update");
    assert!(fs::read_to_string(&path).unwrap().contains("[[Ash Wolf]]"));
    // Review of PR 298, item 2: A's link result keeps the order its own update issued.
    let a_order = d.app().library.open_link_result_order();
    // AD3: the update's result is a message, and its Undo follows it with its icon. The result
    // takes the offer's place, so nothing below it moves.
    undo_follows_message(
        d,
        "iter-19-E-AD3-link",
        "Links now say “Ash Wolf”.",
        "Undo link update",
    );
    let result = d.painted_word_rect("Links now say “Ash Wolf”.", egui::Rect::EVERYTHING);
    d.observe(
        "iter-19-E-AD3-link-place",
        !d.has("Update 1 link")
            && offer.zip(result).is_some_and(|(offer, result)| {
                (offer.top() - result.top()).abs() <= 1.0
                    && (offer.left() - result.left()).abs() <= 1.0
            }),
        format!("The result {result:?} takes the offer's place {offer:?}, and the offer has gone"),
    );
    d.state("renamed-creature-links-updated", Sizes::All);
    let with_result = reader_frame(d, "Ash Wolf");
    let body_with_result = kind_line_tops(d);

    // B (Veilglass Lantern), linked from another file (Ash Wraith), is renamed while A's result
    // waits. A's result is hidden in B's reader, not ended; B's offer and result are B's.
    let (entries, _) = splat::content::read_directory(&package).unwrap();
    let wraith = package.join(
        &entries
            .iter()
            .find(|e| e.name == "Ash Wraith")
            .unwrap()
            .path,
    );
    let wraith_before = fs::read_to_string(&wraith).unwrap();
    fs::write(
        &wraith,
        format!("{wraith_before}\nSee [[Veilglass Lantern]].\n"),
    )
    .unwrap();
    d.click("Packages");
    d.click("Reload from disk");
    search_library(d, "Veilglass Lantern");
    d.click_starting("Veilglass Lantern · ");
    d.wait_for_text("Veilglass Lantern");
    let a_hidden_in_b = !d.has_containing("Links now say") && !d.has("Undo link update");
    let lantern_path = package.join(
        &entries
            .iter()
            .find(|e| e.name == "Veilglass Lantern")
            .unwrap()
            .path,
    );
    d.click(EDIT_MARKDOWN);
    // Astra re-check: B's long name makes its offer and its result wrap. They scroll inside the
    // two-line result part, so the editor keeps its place before, with the offer and with the
    // result, at 960, 1200 and 1920.
    const LAMP: &str = "Veilglass Lantern of the Ninth Ember Watch at Cinderfall Gate";
    // From Save's row to the editor: the space that holds the results and the reason.
    let editor_below_read = |d: &mut Driver<'_>| {
        [SMALL, DEFAULT, LARGE].map(|size| {
            d.at_size(size, |d| {
                // The editor's frame: its accessible rect follows its scrolled text.
                Some(d.input_frame("Markdown editor")?.0.top() - d.place_of("Save")?.bottom())
            })
        })
    };
    let lantern = d.editor_text().unwrap();
    let lamp =
        crate::content_workspace::patch_fields(&lantern, &[("splat_name", Some(LAMP.into()))])
            .unwrap()
            .replace("# Veilglass Lantern", &format!("# {LAMP}"));
    d.replace_text_in("Markdown editor", &lamp);
    let b_unsaved = editor_below_read(d);
    d.click("Save");
    d.settle();
    let b_offer_shown = d.has_containing(&format!(
        "Change links from “Veilglass Lantern” to “{LAMP}”."
    ));
    let b_offer_alone = !d.has_containing("Links now say “Ash Wolf”");
    let b_with_offer = editor_below_read(d);
    d.click("Update 1 link");
    d.settle();
    let b_result_shown = d.has_containing(&format!("Links now say “{LAMP}”."));
    let b_order = d.app().library.open_link_result_order();
    let b_result_alone = !d.has_containing("Links now say “Ash Wolf”");
    let b_with_result = editor_below_read(d);
    // Astra re-check 2: B's long result is reached by keyboard and by wheel, and its Undo keeps
    // clear of every scroll bar, splitter and other target at 960, 1200 and 1920. Round 26: the
    // message footer is as tall as what it holds, so a result that fits needs no scroll; it must
    // still be reached, with its Undo, at each size.
    let b_message = format!("Links now say “{LAMP}”.");
    let (tabbed, _, _, ring) = fixed_part_scrolls(d, &b_message);
    let reached = space_reaches(d, &b_message, Some("Undo link update"));
    let (keyed, wheeled) = (
        reached.iter().all(|(keyboard, _)| *keyboard),
        reached.iter().all(|(_, pointer)| *pointer),
    );
    let undo_clear = [SMALL, DEFAULT, LARGE].map(|size| {
        d.at_size(size, |d| {
            let lanes = d.chrome_lanes();
            d.nodes()
                .into_iter()
                .find(|node| node.name == "Undo link update")
                .is_some_and(|undo| {
                    !undo.shown.is_positive()
                        || (lanes.iter().all(|lane| !lane.intersects(undo.shown))
                            && super::press_area_clear(d, undo.rect, "Undo link update", None)
                                .is_ok())
                })
        })
    });
    d.observe(
        "iter-24-L-part-keyboard-result",
        tabbed && keyed && wheeled && undo_clear.iter().all(|clear| *clear) && ring.iter().all(|r| *r),
        format!(
            "B's result part: Tab {tabbed}, the keys reach it {keyed}, the wheel reaches it {wheeled}, Undo clear at 960, 1200, 1920 {undo_clear:?}, its focus ring clear {ring:?}"
        ),
    );
    d.observe(
        "iter-24-L-wrapped-result",
        b_offer_shown
            && b_result_shown
            && b_unsaved.iter().all(Option::is_some)
            && b_unsaved == b_with_offer
            && b_unsaved == b_with_result,
        format!(
            "B's wrapped offer and result keep its editor below Read: unsaved {b_unsaved:?}, offer {b_with_offer:?}, result {b_with_result:?}"
        ),
    );
    let b_saved = fs::read_to_string(&wraith)
        .unwrap()
        .contains(&format!("[[{LAMP}]]"));
    search_library(d, "Ash Wolf");
    d.click_starting("Ash Wolf · ");
    d.settle();
    let a_back = d.visible_text("Links now say “Ash Wolf”.") && !d.has_containing("Veilglass");
    let a_order_back = d.app().library.open_link_result_order();
    d.observe(
        "review-298-2-link-order",
        a_order.is_some()
            && a_order_back == a_order
            && b_order.zip(a_order).is_some_and(|(b, a)| b > a),
        format!(
            "Each link result keeps its own update's order: A {a_order:?}, B {b_order:?}, A again {a_order_back:?}"
        ),
    );
    d.observe(
        "iter-24-L-link-owner",
        a_hidden_in_b && b_offer_alone && b_result_alone && b_saved && a_back,
        format!(
            "Each link offer and result shows in its own entry's reader only: A hidden in B {a_hidden_in_b}, B's offer alone {b_offer_alone}, B's result alone {b_result_alone}, B saved {b_saved}, A's result back in A {a_back}"
        ),
    );
    if d.has("Undo link update") {
        d.click("Undo link update");
    }
    let a_undone = fs::read_to_string(&path)
        .unwrap()
        .contains("[[Cinder Wolf]]");
    let b_kept = fs::read_to_string(&wraith)
        .unwrap()
        .contains(&format!("[[{LAMP}]]"));
    d.observe(
        "iter-24-L-link-owner-undo",
        a_undone && b_kept,
        format!("Undo of A's link update restores A's links ({a_undone}) and keeps B's ({b_kept})"),
    );
    d.observe(
        "iter-19-E-AD3-link-undone",
        !d.has_containing("Links now say") && !d.has("Undo link update"),
        "After Undo link update, its result message goes away",
    );
    // Round 26 FD6: the links say the old name again, so the rename's offer comes back in the
    // result's place. B's update stays on disk (checked above).
    d.settle();
    d.observe(
        "iter-26-L-FD6-offer",
        d.has_containing("Change links from “Cinder Wolf” to “Ash Wolf”.")
            && d.has("Update 1 link"),
        "After Undo link update, the rename's offer comes back",
    );
    let without = reader_frame(d, "Ash Wolf");
    // Round 26 FD2: the offer and its result sit in the reader's message footer, so the stat
    // block keeps the place it has with no message: with the offer, with the result, and after
    // Undo (the offer again).
    let body_after_undo = kind_line_tops(d);
    d.observe(
        "iter-26-L-FD2",
        clean_body.iter().all(Option::is_some)
            && [body_with_offer, body_with_result, body_after_undo]
                .iter()
                .all(|tops| *tops == clean_body),
        format!(
            "The kind line is at {clean_body:?} with no message, {body_with_offer:?} with the offer, {body_with_result:?} with the result and {body_after_undo:?} after Undo"
        ),
    );
    d.observe(
        "iter-24-L-FC4-links",
        without.iter().all(Option::is_some) && with_offer == without && with_result == without,
        format!(
            "The tabs, the file line and the title keep their y with the link offer {with_offer:?}, its result {with_result:?} and neither {without:?}"
        ),
    );
    d.assert_state_matches(
        "renamed-creature-link-update-undone",
        "renamed-creature-offers-link-update",
    );
    // Round 26 FD6: the offer after an Undo is a new action. It writes again, and its own Undo
    // gives the offer back again. B's update stays throughout.
    d.click("Update 1 link");
    d.wait_for_text("Undo link update");
    let again = fs::read_to_string(&path).unwrap().contains("[[Ash Wolf]]");
    d.click("Undo link update");
    d.settle();
    d.observe(
        "iter-26-L-FD6-again",
        again
            && fs::read_to_string(&path).unwrap().contains("[[Cinder Wolf]]")
            && fs::read_to_string(&wraith)
                .unwrap()
                .contains(&format!("[[{LAMP}]]"))
            && d.has("Update 1 link"),
        format!("The offer writes again ({again}), its Undo restores A's links and gives the offer back, and B's links stay"),
    );
    // Review of PR 298 (re-check): an older discard result is kept, then the links are updated:
    // the newer `Undo link update` is above the older `Undo discard`, in Edit Markdown and in
    // Read, as each result keeps the order its own action issued.
    const DISCARD_A: &str = "Undo discard of Ash Wolf";
    d.click(EDIT_MARKDOWN);
    d.type_into_editor("A line to let go.\n");
    d.click("Discard");
    d.expect(DISCARD_A);
    d.click("Update 1 link");
    d.wait_for_text("Undo link update");
    let newest_first = |d: &mut Driver<'_>| {
        let (links, discard) = (d.place_of("Undo link update"), d.place_of(DISCARD_A));
        (
            d.in_view("Undo link update")
                && d.in_view(DISCARD_A)
                && links
                    .zip(discard)
                    .is_some_and(|(links, discard)| links.top() < discard.top()),
            format!("Undo link update {links:?}, Undo discard {discard:?}"),
        )
    };
    let (edit, edit_detail) = newest_first(d);
    d.click("Read");
    let (read, read_detail) = newest_first(d);
    d.observe(
        "review-298-3-link-above-discard",
        edit && read,
        format!("The newer link update leads the older discard: Edit Markdown {edit_detail}; Read {read_detail}"),
    );
    // Back to the offer, as before this check.
    d.click("Undo link update");
    d.settle();
    d.expect("Update 1 link");
    // Round 26 re-check: in Read with an unsaved draft, the save row's footer holds the result;
    // its Undo restores the links, keeps the draft and ends the result. The Library reads the
    // files again once the draft ends, and then the offer shows again.
    d.click("Update 1 link");
    d.wait_for_text("Undo link update");
    let updated = fs::read_to_string(&path).unwrap().contains("[[Ash Wolf]]");
    d.click(EDIT_MARKDOWN);
    let saved_draft = d.editor_text().unwrap();
    d.type_into_editor("Unsaved ruling.\n");
    d.click("Read");
    d.click("Undo link update");
    d.settle();
    let links_back = fs::read_to_string(&path)
        .unwrap()
        .contains("[[Cinder Wolf]]");
    let result_ended = !d.has("Undo link update");
    d.click(EDIT_MARKDOWN);
    let draft_kept = d
        .editor_text()
        .is_some_and(|text| text.contains("Unsaved ruling."));
    // A fault that keeps the result fails the observation below; its Undo puts the links back.
    if d.has("Undo link update") {
        d.click("Undo link update");
        d.settle();
    }
    d.replace_editor_text(&saved_draft);
    d.click("Read");
    d.settle();
    let offer_back = d.has("Update 1 link");
    d.observe(
        "iter-26-L-undo-unsaved",
        updated && links_back && result_ended && draft_kept && offer_back,
        format!(
            "With an unsaved draft in Read, Undo link update restores the links ({links_back}), ends its result ({result_ended}) and keeps the draft ({draft_kept}); the offer is back after the draft ({offer_back})"
        ),
    );
    // Round 26 review: the offer that Undo gives back keeps the version of Ash Wolf's file it
    // was made from. An outside edit of the body, with no reload, makes it refuse: the linking
    // file keeps its bytes. The file then gets its bytes back for the later steps.
    let wolf = package.join("Creatures/Cinder Wolf.md");
    let wolf_saved = fs::read(&wolf).unwrap();
    let mut edited = wolf_saved.clone();
    edited.extend_from_slice(b"\nA note added outside Splat.\n");
    fs::write(&wolf, &edited).unwrap();
    let linking = fs::read(&path).unwrap();
    d.click("Update 1 link");
    d.settle();
    let refused = d.has_containing(crate::library::LINKS_NOT_CHANGED)
        && fs::read(&path).unwrap() == linking
        && !d.has("Undo link update");
    d.observe(
        "iter-26-L-offer-version",
        refused,
        "After an outside body edit, the offer Undo gave back writes nothing and says why",
    );
    // A fault that writes fails the observation above; its Undo puts the links back for the
    // later steps.
    if d.has("Undo link update") {
        d.click("Undo link update");
        d.settle();
    }
    fs::write(&wolf, &wolf_saved).unwrap();

    // Astra re-check: a stale offer goes. B is renamed (offer), then renamed again to a name
    // another entry has: the earlier offer ends with that save, though the new rename gives no
    // offer. Then a valid offer whose entry is renamed outside Splat goes on reload. The linking
    // file keeps its bytes throughout.
    let rename_b = |d: &mut Driver<'_>, from: &str, to: &str| {
        d.click(EDIT_MARKDOWN);
        let text = d.editor_text().unwrap();
        let renamed =
            crate::content_workspace::patch_fields(&text, &[("splat_name", Some(to.into()))])
                .unwrap()
                .replace(&format!("# {from}"), &format!("# {to}"));
        d.replace_text_in("Markdown editor", &renamed);
        d.click("Save");
        d.settle();
    };
    let rename_outside = |to: &str| {
        let text = fs::read_to_string(&lantern_path).unwrap();
        let name = splat::content::Entry::parse("Item.md", &text).unwrap().name;
        fs::write(
            &lantern_path,
            crate::content_workspace::patch_fields(&text, &[("splat_name", Some(to.into()))])
                .unwrap()
                .replace(&format!("# {name}"), &format!("# {to}")),
        )
        .unwrap();
    };
    let linking = fs::read(&wraith).unwrap();
    search_library(d, "Veilglass");
    d.click_starting(&format!("{LAMP} · "));
    rename_b(d, LAMP, "Veilglass Beacon");
    let first_offer = d.has_containing(&format!(
        "Change links from “{LAMP}” to “Veilglass Beacon”."
    ));
    rename_b(d, "Veilglass Beacon", "Ash Wraith");
    let second_rename_gone =
        !d.has_containing("Change links from") && !d.has_containing("Update 1 link");
    rename_outside(LAMP);
    d.click("Packages");
    d.click("Reload from disk");
    d.settle();
    search_library(d, "Veilglass");
    d.click_starting(&format!("{LAMP} · "));
    rename_b(d, LAMP, "Veilglass Beacon");
    let second_offer = d.has_containing(&format!(
        "Change links from “{LAMP}” to “Veilglass Beacon”."
    ));
    // Astra re-check 3: the entry is renamed outside Splat at the last moment, after the scan and
    // after each linking file is checked, directly before the write. Update writes nothing and
    // says why.
    let hooked = std::rc::Rc::new(std::cell::Cell::new(false));
    {
        let hooked = hooked.clone();
        let owner = lantern_path.clone();
        crate::content_workspace::before_next_write(move || {
            let text = fs::read_to_string(&owner).unwrap();
            let name = splat::content::Entry::parse("Item.md", &text).unwrap().name;
            fs::write(
                &owner,
                crate::content_workspace::patch_fields(
                    &text,
                    &[("splat_name", Some("Veilglass Torch".into()))],
                )
                .unwrap()
                .replace(&format!("# {name}"), "# Veilglass Torch"),
            )
            .unwrap();
            hooked.set(true);
        });
    }
    let pressed = d.has("Update 1 link");
    if pressed {
        d.click("Update 1 link");
    }
    d.settle();
    // A hook that did not run must not fire on a later write.
    crate::content_workspace::before_next_write(|| {});
    let last_moment_kept = pressed
        && hooked.get()
        && fs::read(&wraith).unwrap() == linking
        && d.has_containing(crate::library::LINKS_NOT_CHANGED);
    // Back to a fresh offer: B named LAMP on disk, reloaded, then renamed in Splat again.
    rename_outside(LAMP);
    d.click("Packages");
    d.click("Reload from disk");
    d.settle();
    search_library(d, "Veilglass");
    d.click_starting(&format!("{LAMP} · "));
    rename_b(d, LAMP, "Veilglass Beacon");
    let third_offer = d.has_containing(&format!(
        "Change links from “{LAMP}” to “Veilglass Beacon”."
    ));
    // Round 26 re-check: a new file with B's identity and its old name, made after the rename
    // with no reload, is a conflict on disk. Only B's own file is left out of the check, so the
    // update refuses, writes nothing and gives no result.
    let twin = lantern_path.with_file_name("Old lamp.md");
    let beacon = fs::read_to_string(&lantern_path).unwrap();
    fs::write(
        &twin,
        crate::content_workspace::patch_fields(&beacon, &[("splat_name", Some(LAMP.into()))])
            .unwrap()
            .replace("# Veilglass Beacon", &format!("# {LAMP}")),
    )
    .unwrap();
    let linking_before = fs::read(&wraith).unwrap();
    let twin_pressed = d.has("Update 1 link");
    if twin_pressed {
        d.click("Update 1 link");
    }
    d.settle();
    let twin_refused = twin_pressed
        && fs::read(&wraith).unwrap() == linking_before
        && d.has_containing(crate::library::LINKS_NOT_CHANGED)
        && !d.has("Undo link update");
    d.observe(
        "iter-26-L-offer-identity",
        twin_refused,
        "A file made after the rename with B's identity and old name makes the update refuse; the linking file keeps its bytes and no result shows",
    );
    // A fault that writes fails the observation above; its Undo puts the links back.
    if d.has("Undo link update") {
        d.click("Undo link update");
        d.settle();
    }
    fs::remove_file(&twin).unwrap();
    // A fresh offer for the next case: B back to its old name, then renamed again. Each Save
    // also ends the refusal's error.
    rename_b(d, "Veilglass Beacon", LAMP);
    rename_b(d, LAMP, "Veilglass Beacon");
    // Astra re-check 2: renamed outside Splat with no reload, Update reads the entry's file
    // first, writes nothing and says why.
    rename_outside("Veilglass Torch");
    // Round 26: the page's error is a reason in the pane's message footer. Save and the editor
    // keep their places when it comes, at 960, 1200 and 1920.
    let save_and_editor = |d: &mut Driver<'_>| {
        [SMALL, DEFAULT, LARGE].map(|size| {
            d.at_size(size, |d| {
                Some((
                    d.place_of("Save")?.top(),
                    d.input_frame("Markdown editor")?.0.top(),
                ))
            })
        })
    };
    let before_error = save_and_editor(d);
    let pressed = d.has("Update 1 link");
    if pressed {
        d.click("Update 1 link");
    }
    d.settle();
    let no_reload_kept = pressed
        && fs::read(&wraith).unwrap() == linking
        && d.has_containing(crate::library::LINKS_NOT_CHANGED);
    let with_error = save_and_editor(d);
    let error_below = d
        .input_frame("Markdown editor")
        .zip(d.painted_rect_in("No links were changed.", egui::Rect::EVERYTHING))
        .is_some_and(|((editor, _), error)| error.top() > editor.bottom());
    d.observe(
        "iter-26-L-error-footer",
        no_reload_kept
            && error_below
            && before_error.iter().all(Option::is_some)
            && before_error == with_error,
        format!(
            "The error sits below the editor ({error_below}); Save and the editor stay at {before_error:?} → {with_error:?}"
        ),
    );
    d.click("Packages");
    d.click("Reload from disk");
    d.settle();
    search_library(d, "Veilglass");
    d.click_starting("Veilglass Torch · ");
    d.settle();
    let outside_gone = !d.has_containing("Change links from") && !d.has_containing("Update 1 link");
    let bytes_kept = fs::read(&wraith).unwrap() == linking;
    d.observe(
        "iter-24-L-stale-offer",
        first_offer
            && second_rename_gone
            && second_offer
            && last_moment_kept
            && third_offer
            && no_reload_kept
            && outside_gone
            && bytes_kept,
        format!(
            "A stale offer goes: first offer {first_offer}, gone after a rename to a taken name {second_rename_gone}, second offer {second_offer}, Update after an outside rename directly before the write writes nothing {last_moment_kept}, third offer {third_offer}, Update after an outside rename with no reload writes nothing {no_reload_kept}, gone after reload {outside_gone}; the linking file kept its bytes {bytes_kept}"
        ),
    );
    // The later steps compare with the package before B's renames: give B and Ash Wraith their
    // first text back.
    fs::write(&wraith, &wraith_before).unwrap();
    fs::write(&lantern_path, &lantern).unwrap();
    d.click("Packages");
    d.click("Reload from disk");
    d.settle();

    let moved = d.root.join("modules/watch-package");
    fs::rename(&package, &moved).unwrap();
    go_campaign(d);
    d.click("Reload from disk");
    d.expect("Find the folder…");
    d.expect("Ashen Marches moved from modules/ashen-marches/ to modules/watch-package/.");
    assert!(!d.has_containing("Splat cannot find Ashen Marches"));
    assert!(!d.has_containing("Ashen Marches is now in"));
    d.state_prepared("package-folder-moved-on-disk", Sizes::All, |d| {
        let warning = d.place_of("Ashen Marches moved from modules/ashen-marches/ to modules/watch-package/.");
        let action = d.place_of("Use this folder");
        let first = d.place_of("Bridge Ambush");
        let second = d.place_of("Shrine of Cinders");
        let and = d.nodes().into_iter().find(|node| node.name == "and");
        check_link_spaces(d);
        let aligned = warning.zip(action).is_some_and(|(text, action)| {
            (text.left() - action.left()).abs() < 1.0 && action.top() > text.bottom()
        });
        let spaced = first.zip(second).zip(and).is_some_and(|((first, second), and)| {
            (and.rect.left() - first.right() - (second.left() - and.rect.right())).abs() < 1.0
                && and.rect.left() - first.right() < crate::design::S
        });
        d.observe("iter-4-C-FD1-FD14", aligned && spaced,
            "One moved-folder warning aligns its recovery action and uses word spaces between affected links");
        // Round 25 family: every action on the message has the secondary border.
        let borders = ["Use this folder", "Find the folder…", "Remove from this campaign"]
            .map(|name| d.control_frame(name).map(|frame| frame.stroke.width > 0.0));
        d.observe(
            "iter-25-C-message-actions",
            borders.iter().all(|border| *border == Some(true)),
            format!("Moved-folder actions bordered: {borders:?}"),
        );
    });
    let wrong = d.root.join("modules/wrong-package");
    fs::create_dir(&wrong).unwrap();
    fs::write(
        wrong.join("Package.md"),
        crate::library::note_text("Wrong package", "package", Some("wrong")),
    )
    .unwrap();
    crate::hermetic::closed::next_folder(wrong);
    d.click("Find the folder…");
    // Round 23 FE3: the fault comes first, and it names the picked folder and the package.
    d.expect(
        "wrong-package/ holds a different package. Choose the folder that holds Ashen Marches.",
    );
    d.state("package-wrong-folder-picked", Sizes::All);
    crate::hermetic::closed::next_folder(moved.clone());
    d.click("Find the folder…");
    d.wait_for_text("Undo folder recovery");
    d.expect("Ashen Marches found in modules/watch-package/.");
    d.state_prepared("package-correct-folder-picked", Sizes::All, |d| {
        check_undo_icon(d, "Undo folder recovery", "iter-6-C-AD3");
    });
    d.click("Undo folder recovery");
    d.wait_for_text("Use this folder");
    d.click("Use this folder");
    d.wait_for_text("Undo folder recovery");
    go_library(d);
    d.click("Packages");
    d.click("Reload from disk");
    search_library(d, "Ash Exposure");
    d.click_starting("Ash Exposure · ");
    d.scroll_until(READER, "Read the watch spell", 12);
    d.scroll_at(READER, 1);
    d.wait_for_text("Read the watch spell");
    d.click("Read the watch spell");
    d.wait_for_text("Watch Ashfall");
    d.expect_same("Restoring the moved package preserves the same linked reader content.");
    d.assert_state_matches(
        "package-folder-restored-with-links",
        "renamed-entry-heading-fixed",
    );
    let package = moved;
    search_library(d, "Ash Exposure");
    d.click_starting("Ash Exposure · ");
    d.click(EDIT_MARKDOWN);
    d.type_into_editor("My moved draft.\n");
    let old = package.join("Rules/Ash Exposure.md");
    let moved = package.join("Moved/Ash Exposure.md");
    fs::create_dir_all(moved.parent().unwrap()).unwrap();
    fs::rename(&old, &moved).unwrap();
    d.focus_window(false);
    d.focus_window(true);
    d.wait_for_text("Save my draft to Moved/Ash Exposure.md");
    // Round 27 FD7: the old folder follows the path, and Show file follows both, on one line.
    d.observe_both(
        "W8",
        "The old folder follows the new path, then Show file, on one line",
        |d| {
            let from = d.place_of("moved from Rules/");
            let file = d.place_of("Show file");
            // On the file line only: Save's words name the path too.
            let path = from.and_then(|from| {
                let line = egui::Rect::from_x_y_ranges(
                    0.0..=f32::INFINITY,
                    from.top() - crate::design::S..=from.bottom() + crate::design::S,
                );
                d.painted_rect_in("Moved/Ash Exposure.md", line)
            });
            match (path, from, file) {
                (Some(path), Some(from), Some(file)) => {
                    from.left() >= path.right()
                        && file.left() >= from.right()
                        && (file.center().y - from.center().y).abs() < crate::design::XS
                        && (path.center().y - from.center().y).abs() < crate::design::XS
                }
                _ => false,
            }
        },
    );
    // Round 24 FF6: the warning offers no choice that no control names; its buttons do.
    d.observe(
        "iter-24-L-FF6",
        d.has(crate::library::MOVED_ON_DISK) && !d.has_containing("keep the disk version"),
        "The moved-file warning says the draft is safe, and the two buttons name the choices",
    );
    d.state_prepared("draft-file-moved-on-disk", Sizes::All, |d| {
        choices_at_text(
            d,
            "iter-15-E-AD3-library-moved",
            "This entry moved on disk",
            "Save my draft to Moved/Ash Exposure.md",
            "Reload and lose my draft",
            None,
        );
    });
    let before_save = fs::read_to_string(&moved).unwrap();
    d.click("Save my draft to Moved/Ash Exposure.md");
    d.wait_for_text("Saved");
    assert!(!old.exists());
    let saved = fs::read_to_string(&moved).unwrap();
    assert!(saved.contains("My moved draft."));
    let id = crate::content_workspace::note_id(&saved).unwrap();
    let (entries, errors) = splat::content::read_directory(&package).unwrap();
    assert!(errors.is_empty());
    assert_eq!(entries.iter().filter(|entry| entry.id == id).count(), 1);
    reveal_editor_passage(d, "My moved draft.");
    d.state("draft-saved-at-moved-path", Sizes::All);
    assert_ne!(saved, before_save);
    assert!(!d.has("Undo save"));
}

fn check_read_repair(d: &mut Driver<'_>, title: &str) {
    let read = d.place_of("Read").expect("Read tab is visible");
    let area = egui::Rect::from_min_max(read.min, egui::pos2(10000.0, 10000.0));
    let title = d
        .painted_rect_in(title, area)
        .expect("Reader title is visible");
    let error = d
        .painted_rect_in(
            if d.has("Write the manifest again") {
                "The package manifest was deleted."
            } else {
                "Line 4 opens a list"
            },
            area,
        )
        .expect("Specific repair error is visible");
    d.observe(
        "iter-9-E-FD10",
        error.top() >= title.bottom(),
        "The specific error follows the reader title",
    );
}

fn check_repair_reason(d: &mut Driver<'_>, manifest: bool) {
    let save = d.place_of("Save").expect("Save row is visible");
    let error = d
        .painted_rect_in("Line 4 opens a list", egui::Rect::EVERYTHING)
        .expect("The error is visible");
    let editor = d
        .input_frame("Markdown editor")
        .expect("The source editor is visible")
        .0;
    // Round 26 lead ruling: the error is in the pane's message footer, below the source, so it
    // never moves the source, which starts directly below Save.
    d.observe(
        "iter-9-E-FD9",
        editor.top() >= save.bottom()
            && editor.top() - save.bottom() <= design::S * 2.0
            && error.top() >= editor.bottom() + design::S - 1.0,
        "The source starts directly below Save, and the error is in the footer below it",
    );
    if manifest {
        let action = d
            .place_of("Replace with a basic Package.md")
            .expect("Replacement is visible");
        let consequence = d
            .painted_rect_in(
                &crate::content_workspace::manifest_replace_consequence("Package.md"),
                egui::Rect::EVERYTHING,
            )
            .expect("The consequence is visible");
        d.observe(
            "iter-9-E-FD2",
            action.top() >= error.bottom()
                && consequence.top() >= action.bottom()
                && (action.left() - error.left()).abs() <= 1.0
                && (consequence.left() - error.left()).abs() <= 1.0
                && d.control_frame("Replace with a basic Package.md")
                    .is_some_and(|frame| frame.stroke.width > 0.0),
            "Manifest error, bordered repair and muted consequence share the text edge in order",
        );
        // Round 23 AE3: the consequence is Campaign's longer sentence, so it may take two lines,
        // but it wraps at the pane measure, not at the action's width.
        d.observe(
            "iter-10-E-FD8",
            consequence.height() <= design::Type::Small.line() * 2.5
                && consequence.right() > action.right(),
            "The replacement consequence wraps at the pane measure, not the action width",
        );
    }
}

fn check_unreadable_owner(d: &mut Driver<'_>, id: &str) {
    let read = d.place_of("Read").expect("The Library reader is visible");
    let reader = egui::Rect::from_min_max(read.min, egui::pos2(10000.0, 10000.0));
    d.observe(
        id,
        d.painted_in(reader).matches("Cannot be read").count() == 1,
        "The reader says Cannot be read once, on the owner line",
    );
}

pub fn package_about(d: &mut Driver<'_>) {
    go_library(d);
    d.open_combo_at(LIBRARY_PACKAGE_DROPDOWN);
    d.click(MODULE);
    d.click("About Ashen Marches");
    d.wait_for_text("5 of 6 entries chosen");
    d.observe(
        "W2 fixture",
        d.visible_text("5 of 6 entries chosen")
            && !d.visible_text("Engine support")
            && !d.visible_text("Library / Other entries"),
        "Four chosen entries and the one entry they need count as chosen (round 27 FA6)",
    );
    d.state("package-about", Sizes::All);
    let title_before = reader_title_tops(d, MODULE);
    let read_tabs = |d: &mut Driver<'_>| {
        [SMALL, DEFAULT, LARGE].map(|size| d.at_size(size, |d| d.place_of("Read")))
    };
    let read_before = read_tabs(d);
    d.click("Choose all");
    d.wait_for_text("All entries chosen");
    let title_after = reader_title_tops(d, MODULE);
    let read_after = read_tabs(d);
    d.state_prepared("package-every-entry-used", Sizes::All, |d| {
        check_undo_icon(d, "Undo package choice", "iter-8-E-FD3-use");
        // Round 23 FD4: one result place per reader. The result sits after the title, where
        // the warnings sit, so the tabs, the file line and the title never move.
        let read = d.place_of("Read").expect("Read tab is visible");
        let title = reader_title(d, MODULE).expect("The About title is visible");
        let result = d
            .painted_rect_in(crate::library::PACKAGE_USE_RESULT, egui::Rect::EVERYTHING)
            .expect("Package use result is visible");
        d.observe(
            "iter-23-L-FD4-use",
            result.left() >= read.left() - 1.0
                && result.top() > title.bottom()
                && read_before.iter().all(Option::is_some)
                && read_before == read_after
                && title_before.iter().all(Option::is_some)
                && title_before == title_after,
            format!(
                "The package-use result {result:?} sits below the title {title:?}; the tabs and the title keep their places ({read_before:?} → {read_after:?}, {title_before:?} → {title_after:?})"
            ),
        );
    });
    d.click("Undo package choice");
    d.wait_for_text("5 of 6 entries chosen");
    d.expect_same("Undo package choice restores the original package selection and About reader.");
    d.assert_state_matches("package-use-undone", "package-about");
    d.click(EDIT_MARKDOWN);
    // Round 24 FB3: the manifest's editor below Read with no result, at each size.
    let editor_below_read = |d: &mut Driver<'_>| {
        [SMALL, DEFAULT, LARGE].map(|size| {
            d.at_size(size, |d| {
                // The editor's frame: its accessible rect follows its scrolled text.
                Some(d.input_frame("Markdown editor")?.0.top() - d.place_of("Read")?.bottom())
            })
        })
    };
    let no_result = editor_below_read(d);
    // Round 26 A: with nothing to say, the editor has no footer and runs to the pane's bottom.
    let footer_empty = footer_in_order(d, &[]);
    let original = d.editor_text().unwrap();
    let renamed = crate::content_workspace::patch_fields(
        &original,
        &[("splat_name", Some("Emberfall Watch".into()))],
    )
    .unwrap();
    d.replace_text_in(
        "Markdown editor",
        &format!("{renamed}\nA package for the watch at Emberfall.\n"),
    );
    d.click("Save");
    d.click("Read");
    d.wait_for_text("A package for the watch at Emberfall.");
    d.state_prepared("package-about-edited", Sizes::All, |d| {
        // The manifest's first heading still says the old name, and the header counts it.
        d.observe(
            "iter-13-E-FD1",
            d.visible_text("1 file needs fixing")
                && d.has(&format!("{} About Emberfall Watch", crate::icons::WARN))
                && !d.has("About Emberfall Watch"),
            "The About row warns for the manifest problem the header counts",
        );
        // Round 23 FD8: the About row's warning icon has the warning colour too.
        // Ruling R13 (row 151): the row keeps `About`; the name may shorten by whole words.
        let about = format!("{} About Emberfall", crate::icons::WARN);
        let mut marks = Vec::new();
        d.each_painted_text(|text, clip| {
            let job = &text.galley.job;
            if job.text.starts_with(&about) && clip.intersects(text.visual_bounding_rect()) {
                marks.push(painted_marks(job));
            }
        });
        d.observe(
            "iter-23-L-FD8-about",
            warning_marked(&marks),
            format!("The About row's mark has the warning colour: {marks:?}"),
        );
    });
    go_campaign(d);
    d.wait_for_text("Emberfall Watch");
    d.state("renamed-package-in-campaign", Sizes::All);
    let folder = d.root.join("modules/ashen-marches");
    let path = folder.join("Module.md");
    let manifest = fs::read_to_string(&path).unwrap();
    let broken = manifest.replace("splat_kind: package", "splat_kind: [");
    fs::write(&path, &broken).unwrap();
    d.click("Reload from disk");
    d.expect("Module.md cannot be read (line 4)");
    d.click("Replace with a basic Module.md");
    d.wait_for_text("Undo write Module.md");
    d.state_prepared("legacy-manifest-replaced-in-campaign", Sizes::All, |d| {
        check_undo_icon(d, "Undo write Module.md", "iter-8-E-FD7-campaign-undo");
        d.observe(
            "iter-8-E-FD7-campaign-name",
            d.visible_text("Module.md was written again.")
                && !d.visible_text("Package.md was written again."),
            "Campaign names the existing manifest file",
        );
        // Round 27 FC5: the result says what the file holds now, and that Undo puts the DM's
        // replaced text back.
        d.observe(
            "iter-27-C-FC5",
            d.visible_text(
                "Module.md was written again. It now has the package’s name and ID in the current \
                 format. Undo puts your old text back.",
            ),
            "The replaced manifest's result says what changed and that Undo restores the old text",
        );
    });
    assert!(!folder.join("Package.md").exists());
    d.click("Undo write Module.md");
    d.wait_for_text("Module.md cannot be read (line 4)");
    assert_eq!(fs::read_to_string(&path).unwrap(), broken);
    d.click("Open in Library");
    d.expect("Markdown editor");
    d.click("Replace with a basic Module.md");
    d.wait_for_text("Undo write Module.md");
    // The result takes the slot below the save row, so the editor keeps its place.
    let with_result = editor_below_read(d);
    d.observe(
        "iter-24-L-FB3",
        no_result.iter().all(Option::is_some) && no_result == with_result,
        format!(
            "Edit Markdown's editor is as far below Read with the write result {with_result:?} as with none {no_result:?}"
        ),
    );
    d.state_prepared("legacy-manifest-replaced-in-library", Sizes::All, |d| {
        check_undo_icon(d, "Undo write Module.md", "iter-8-E-FD7-library-undo");
        d.observe(
            "iter-8-E-FD7-library-name",
            d.visible_text("Module.md was written again.")
                && !d.visible_text("Package.md was written again."),
            "Library names the existing manifest file",
        );
    });
    // Astra review of round 24: with the write result kept on screen, Save's refusal of invalid
    // text, Discard and Undo discard move nothing either. The result and the reason share one
    // space of fixed height below the save row.
    let written = d.editor_text().unwrap();
    let invalid = written.replacen("splat_kind: package", "splat_kind: [", 1);
    assert_ne!(invalid, written, "fixture: the manifest names its kind");
    d.replace_editor_text(&invalid);
    d.wait_for_text("Invalid");
    // Round 25 lead ruling: two results with a live Undo (Discard's and the write's) share the
    // one two-line space, newest first; the older one stays in it, reachable by scroll.
    // Astra review: kept means drawn and reachable, by keyboard and by pointer, with its Undo, at
    // 960, 1200 and 1920; the space's accessible name alone does not count.
    let kept = |d: &mut Driver<'_>| {
        space_reaches(
            d,
            "Module.md was written again.",
            Some("Undo write Module.md"),
        )
        .iter()
        .all(|(keyboard, pointer)| *keyboard && *pointer)
    };
    let refused = (kept(d), editor_below_read(d));
    // Round 26 A: the footer's one order. The write result's live Undo first, then Save's reason.
    const WRITTEN: &str = "Module.md was written again.";
    const REASON: &str = "opens a list";
    let footer_refused = footer_in_order(d, &[WRITTEN, REASON]);
    d.state("manifest-footer-undo-then-reason", Sizes::All);
    d.click("Discard");
    assert_eq!(d.editor_text().as_deref(), Some(written.as_str()));
    let discarded = (
        kept(d) && d.has_containing("Undo discard"),
        editor_below_read(d),
    );
    // Two live Undo results, newest first: Discard's, then the write's. No reason is left.
    let footer_discarded = footer_in_order(d, &["draft discarded.", WRITTEN]);
    let reason_gone = !d.has_containing(REASON);
    d.state("manifest-footer-two-undo-results", Sizes::All);
    d.click_starting("Undo discard");
    assert_eq!(d.editor_text().as_deref(), Some(invalid.as_str()));
    let undone = (kept(d), editor_below_read(d));
    let footer_undone = footer_in_order(d, &[WRITTEN, REASON]);
    d.observe(
        "iter-26-A-footer-order",
        [&footer_empty, &footer_refused, &footer_discarded, &footer_undone]
            .iter()
            .all(|sizes| sizes.iter().all(|ok| *ok))
            && reason_gone,
        format!(
            "Edit Markdown's footer at 960, 1200, 1920: none with nothing to say {footer_empty:?}; the write's Undo, then the reason {footer_refused:?}; Discard's Undo, then the write's {footer_discarded:?}; after Undo discard {footer_undone:?}"
        ),
    );
    // Round 26 follow-up (found by package L): with the footer on screen, Lookup opened from the
    // rail keeps its search field's focus, and the DM's typed words arrive there, not in the
    // editor behind it.
    let draft_before = d.editor_text();
    d.click("Lookup");
    d.type_text("Cinder");
    let focus = d.focused_name();
    let typed = d.field_text(SEARCH_LOOKUP);
    let draft_kept = d.editor_text() == draft_before;
    d.press_ctrl(egui::Key::K);
    d.observe(
        "iter-26-A2-lookup-keeps-focus",
        focus.as_deref() == Some(SEARCH_LOOKUP) && typed.as_deref() == Some("Cinder") && draft_kept,
        format!(
            "Lookup from the rail with the footer on screen: focus {focus:?}, search {typed:?}, draft unchanged {draft_kept}"
        ),
    );
    d.observe(
        "iter-24-L-FB3-states",
        [&refused, &discarded, &undone]
            .iter()
            .all(|(kept, tops)| *kept && *tops == no_result),
        format!(
            "With the write result on screen the editor stays {no_result:?} below Read: Invalid {refused:?}, discarded {discarded:?}, Undo discard {undone:?}"
        ),
    );
    d.click("Discard");
    assert_eq!(d.editor_text().as_deref(), Some(written.as_str()));
    // Astra re-check: a Save reason that wraps (a long unknown kind) scrolls inside its two-line
    // slot; before it, with it and after Discard the editor keeps its place.
    let unsaved = (kept(d), editor_below_read(d));
    let long_kind = written.replacen(
        "splat_kind: package",
        "splat_kind: package of the watch that keeps the ember gate beyond the long ash road and the old cinder bridge where the patrol waits for the lantern signal at dusk",
        1,
    );
    assert_ne!(long_kind, written, "fixture: the manifest names its kind");
    d.replace_editor_text(&long_kind);
    d.wait_for_text("Invalid");
    let reason = d
        .nodes()
        .into_iter()
        .find(|node| {
            node.name.contains("ember gate beyond the long ash road")
                && node.role != egui::accesskit::Role::GenericContainer
        })
        .map(|node| node.name);
    // The one message space is named with all it holds: the write result, then the reason.
    let space = d
        .nodes()
        .into_iter()
        .find(|node| {
            node.name.contains("ember gate beyond the long ash road")
                && node.role == egui::accesskit::Role::GenericContainer
        })
        .map(|node| node.name)
        .unwrap_or_default();
    let failed = (kept(d), editor_below_read(d));
    // Astra re-check 2: the long reason scrolls by keyboard and by wheel, and its words keep
    // clear of every scroll bar and splitter at 960, 1200 and 1920.
    let message = reason.clone().unwrap_or_default();
    let mut sizes = Vec::new();
    d.each_painted_text(|text, clip| {
        if text.galley.job.text == message {
            sizes.push((text.pos, text.galley.size(), clip));
        }
    });
    // Round 26: the footer is as tall as what it holds, so the reason is reached whether or not
    // it needs to scroll.
    let (tabbed, _, _, ring) = fixed_part_scrolls(d, &space);
    let reached = space_reaches(d, &message, None);
    let (keyed, wheeled) = (
        reached.iter().all(|(keyboard, _)| *keyboard),
        reached.iter().all(|(_, pointer)| *pointer),
    );
    let clear = [SMALL, DEFAULT, LARGE].map(|size| {
        d.at_size(size, |d| {
            let lanes = d.chrome_lanes();
            let mut words = Vec::new();
            d.each_painted_text(|text, clip| {
                if text.galley.job.text == message {
                    words.push(text.visual_bounding_rect().intersect(clip));
                }
            });
            !words.is_empty()
                && words.iter().all(|rect| {
                    !rect.is_positive() || lanes.iter().all(|lane| !lane.intersects(*rect))
                })
        })
    });
    d.observe(
        "iter-24-L-part-keyboard-reason",
        tabbed && keyed && wheeled && clear.iter().all(|clear| *clear) && ring.iter().all(|r| *r),
        format!(
            "The reason part: Tab {tabbed}, the keys reach it {keyed}, the wheel reaches it {wheeled}, its words clear of bars and splitters at 960, 1200, 1920 {clear:?}, its focus ring clear {ring:?}; painted {sizes:?}"
        ),
    );
    d.click("Discard");
    assert_eq!(d.editor_text().as_deref(), Some(written.as_str()));
    let after = (kept(d), editor_below_read(d));
    d.observe(
        "iter-24-L-FB3-wrapped-reason",
        reason.is_some()
            && [&unsaved, &failed, &after]
                .iter()
                .all(|(kept, tops)| *kept && *tops == no_result),
        format!(
            "Save's reason {reason:?} keeps the editor {no_result:?} below Read: unsaved {unsaved:?}, failed {failed:?}, discarded {after:?}"
        ),
    );
    assert!(!folder.join("Package.md").exists());
    d.click("Undo write Module.md");
    // Round 25 lead ruling: Discard's live Undo leads the message space; the reason follows it.
    d.settle();
    d.expect("Line 4 opens a list");
    assert_eq!(fs::read_to_string(&path).unwrap(), broken);
}

/// External edits refresh all catalogue users; a draft still needs a choice.
pub fn package_manifest_broken(d: &mut Driver<'_>) {
    let folder = d.root.join("modules/ashen-marches");
    let path = folder.join("Package.md");
    fs::rename(folder.join("Module.md"), &path).unwrap();
    let valid = fs::read_to_string(&path).unwrap();
    fs::write(&path, valid.replace("splat_kind: package", "splat_kind: [")).unwrap();
    go_library(d);
    open_local_package(d, MODULE);
    search_library(d, "Cinder Wolf");
    d.click_starting("Cinder Wolf · ");
    d.expect("22 (4d8 + 4)");
    // Round 24 FC2: a manifest that cannot be read is an error on its About row too.
    let warned_about = format!("{} About Ashen Marches", design::icons::ERROR);
    d.state_prepared("manifest-invalid-library-keeps-entries", Sizes::All, |d| {
        let size = d.window_size();
        let about = d
            .place_of(&warned_about)
            .expect("The warning About row keeps its full accessible name");
        // FD5: one line, the same height as the About row without a warning.
        d.observe(
            "iter-12-E-FD5",
            about.height() <= design::Density::Read.row() + 0.5,
            format!("The warning About row stays one line at {size:?}: {about:?}"),
        );
        // Must stay true: the mark is still painted at the start of the row.
        d.observe(
            "iter-12-E-FD5-icon",
            d.painted_in(about)
                .starts_with(&format!("{} ", design::icons::ERROR)),
            format!("The About row still shows its mark at {size:?}"),
        );
        // Ruling R13 (row 151; it replaces round 25 FF4's "drop `About` first"): the marked row
        // keeps `About` and its mark; the name is whole or cut by whole words.
        let painted = d.painted_in(about);
        let after = painted
            .trim()
            .strip_prefix(&format!("{} About ", design::icons::ERROR))
            .unwrap_or_default();
        d.observe(
            "iter-28-L-151",
            after == "Ashen Marches" || after == "Ashen…",
            format!("The marked About row keeps `About` at {size:?}: {painted:?}"),
        );
        // Round 24 FC2: the mark is the error icon in the error colour.
        let mut marks = Vec::new();
        d.each_painted_text(|text, clip| {
            if text
                .galley
                .job
                .text
                .starts_with(&format!("{} About Ashen", design::icons::ERROR))
                && clip.intersects(text.visual_bounding_rect())
            {
                marks.push(painted_marks(&text.galley.job));
            }
        });
        d.observe(
            "iter-24-L-FC2",
            !marks.is_empty()
                && marks.iter().all(|marks| {
                    marks
                        == &[(
                            design::icons::ERROR.to_owned(),
                            design::colors().result.error,
                        )]
                }),
            format!("The invalid manifest's About mark is an error at {size:?}: {marks:?}"),
        );
        // At 1200x800 the name does not fit beside the icon, so it shortens and hover gives it whole.
        if size == DEFAULT {
            let shortened = !d.visible_text(&warned_about);
            let hover = d.painted_tooltip_shows(
                &format!("{} About Ashen", design::icons::ERROR),
                &warned_about,
            );
            d.observe(
                "iter-12-E-FD5-hover",
                shortened && hover,
                "The shortened About row shows the full package name on hover",
            );
        }
    });
    go_campaign(d);
    d.click("Reload from disk");
    d.expect("Package.md cannot be read (line 4)");
    assert!(!d.has("Find the folder…"));
    let invalid_tops = package_list_tops(d);
    d.state_prepared("manifest-invalid-campaign", Sizes::All, |d| {
        let open = d.place_of("Open in Library").expect("Safe manifest repair is present");
        let replace = d.place_of("Replace with a basic Package.md").expect("Manifest replacement is present");
        let notice = d.painted_left("Ashen Marches: Package.md cannot be read").expect("Manifest warning is visible");
        let help = d.painted_left("Keeps the package name and its links.").expect("Replacement consequence is visible");
        d.observe("iter-7-C-FD4-order", open.top() <= replace.top() && (open.top() < replace.top() || open.right() <= replace.left()),
            "Open in Library precedes manifest replacement");
        d.observe("iter-7-C-FD4-edge", (open.left() - notice).abs() <= 1.0 && (help - notice).abs() <= 1.0,
            "Manifest warning, first action and replacement consequence share a left edge");
        d.observe("iter-7-C-FD4-border", d.control_frame("Open in Library").is_some_and(|frame| frame.stroke.width > 0.0 && frame.stroke.color == design::colors().field_border),
            "Open in Library has a secondary border");
        d.observe("iter-7-C-FD5", d.visible_text("Package.md cannot be read (line 4). Entries are still available."),
            "The manifest fault and availability sentence have a full stop between them");
        d.observe("iter-6-C-AD1", d.visible_text("Keeps the package name and its links.")
            && d.visible_text("including its description.")
            && d.visible_text("The entries stay unchanged."),
            "Manifest replacement states the package name, links, description and unchanged entries");
        let tone = manifest_fault_tone(d, "Ashen Marches: Package.md cannot be read");
        d.observe("iter-23-C-AE4-campaign", tone == Some("error"),
            format!("An invalid manifest on disk is an error in Campaign, as in Library: {tone:?}"));
        // Round 24 FF4, outside the deleted case: the unreadable manifest marks its row with the
        // error icon in the error colour, and only that row.
        let (rows, marked) = package_row_marks(d);
        d.observe(
            "iter-24-C-FF4-invalid",
            rows >= 2
                && marked
                    == [vec![(design::icons::ERROR.to_owned(), design::colors().result.error)]]
                && d.has_containing("Ashen Marches · Manifest cannot be read · "),
            format!("{rows} package rows; marked rows {marked:?}"),
        );
    });
    let broken = fs::read_to_string(&path).unwrap();
    let creatures = folder.join("Creatures/Cinder Wolf.md");
    let creature_before = fs::read_to_string(&creatures).unwrap();
    d.click("Replace with a basic Package.md");
    d.wait_for_text("Undo write Package.md");
    let basic = fs::read_to_string(&path).unwrap();
    assert_eq!(
        crate::content_workspace::note_id(&basic),
        crate::content_workspace::note_id(&valid)
    );
    assert!(basic.contains("splat_name: Ashen Marches"));
    assert_eq!(fs::read_to_string(&creatures).unwrap(), creature_before);
    d.state_prepared("manifest-replaced-with-undo", Sizes::All, |d| {
        check_undo_icon(d, "Undo write Package.md", "iter-6-C-SD3-Replaced");
        // Round 24 FF5: the result and its Undo share one verb.
        d.observe(
            "iter-24-C-FF5",
            d.visible_text("Package.md was written again.") && !d.visible_text("was replaced"),
            "A replaced manifest says it was written again, as its Undo write does",
        );
    });
    // Round 25 review: an independent action (B, a saved package choice) shows its own result
    // and leaves A's Undo with A. Undo A brings back A's exact bytes and keeps B's choices.
    let campaign_md = d.root.join("campaigns/emberfall/Campaign.md");
    d.click_starting("Ashen Marches · ");
    d.click("Use Ash Wraith");
    d.click("Save selections");
    d.settle();
    let b_saved = fs::read(&campaign_md).unwrap();
    let a_kept = d.in_view("Undo write Package.md")
        && d.visible_text("Package.md was written again.")
        // B's plain result is listed after A's live Undo, in the same part (its name).
        && d.has_containing("Campaign selections saved.");
    d.click("Undo write Package.md");
    d.wait_for_text("Package.md cannot be read (line 4)");
    let a_back = fs::read_to_string(&path).unwrap() == broken;
    let b_kept = fs::read(&campaign_md).unwrap() == b_saved;
    d.observe(
        "iter-25-C-undo-owner",
        a_kept && a_back && b_kept && !d.has("Undo write Package.md"),
        format!(
            "A's Undo after B's Save {a_kept}; A's bytes back {a_back}; B's choices kept {b_kept}"
        ),
    );
    // B again, and the needed entries in the right pane, so the steps below start as before.
    d.click("Use Ash Wraith");
    d.click("Save selections");
    d.wait_for_text("Campaign selections saved.");
    d.click_starting("Needed by your entries");
    assert_eq!(fs::read_to_string(&path).unwrap(), broken);
    // Round 25 review: two packages' manifests, A then B. Each keeps its own result and Undo,
    // newest first; Undo A brings back A's exact bytes and leaves B repaired.
    let b_folder = d.root.join("modules/ember-reach");
    fs::create_dir_all(&b_folder).unwrap();
    let b_path = b_folder.join("Package.md");
    let b_valid = crate::library::note_text("Ember Reach", "package", Some("ember_reach"));
    fs::write(&b_path, &b_valid).unwrap();
    let b_module = crate::library::local_module(&b_folder).unwrap();
    let b_id = b_module.id;
    d.app_mut().campaigns.register_module(b_module).unwrap();
    let b_broken = b_valid.replace("splat_kind: package", "splat_kind: [");
    fs::write(&b_path, &b_broken).unwrap();
    d.click("Reload from disk");
    d.wait_for_text("Ember Reach: Package.md cannot be read");
    d.click_below(
        "Ashen Marches: Package.md cannot be read",
        "Replace with a basic Package.md",
    );
    d.wait_for_text("Undo write Package.md");
    d.click_below(
        "Ember Reach: Package.md cannot be read",
        "Replace with a basic Package.md",
    );
    d.settle();
    let b_written = fs::read_to_string(&b_path).unwrap();
    let a_written = fs::read_to_string(&path).unwrap() != broken;
    let undos = d
        .nodes()
        .iter()
        .filter(|node| node.name == "Undo write Package.md")
        .count();
    // Newest first: B's result, then A's, in the one part's name.
    let part = d
        .nodes()
        .into_iter()
        .find(|node| {
            node.name
                .contains("Ember Reach: Package.md was written again.")
                && node
                    .name
                    .contains("Ashen Marches: Package.md was written again.")
        })
        .map(|node| node.name)
        .unwrap_or_default();
    let newest_first = part
        .find("Ember Reach: Package.md was written again.")
        .zip(part.find("Ashen Marches: Package.md was written again."))
        .is_some_and(|(b, a)| b < a);
    // A fault that loses A's result fails the observation below, not this click.
    if d.has_containing("Ashen Marches: Package.md was written again.") {
        d.click_below(
            "Ashen Marches: Package.md was written again.",
            "Undo write Package.md",
        );
        d.settle();
    }
    let a_back = fs::read_to_string(&path).unwrap() == broken;
    let b_kept = fs::read_to_string(&b_path).unwrap() == b_written;
    let b_undo = d.has_containing("Ember Reach: Package.md was written again.")
        && !d.has_containing("Ashen Marches: Package.md was written again.");
    d.observe(
        "iter-25-C-undo-per-package",
        b_written != b_broken && a_written && undos == 2 && newest_first && a_back && b_kept && b_undo,
        format!(
            "B written {}, A written {a_written}, Undo buttons {undos}, newest first {newest_first} in {part:?}; after Undo A: A's bytes back {a_back}, B kept {b_kept}, only B's result {b_undo}",
            b_written != b_broken
        ),
    );
    // Back to one package, with A broken, as the steps below expect.
    if d.has_containing("Ember Reach: Package.md was written again.") {
        d.click_below(
            "Ember Reach: Package.md was written again.",
            "Undo write Package.md",
        );
        d.settle();
    }
    fs::write(&path, &broken).unwrap();
    d.app_mut().campaigns.unregister_module(b_id).unwrap();
    fs::remove_dir_all(&b_folder).unwrap();
    d.click("Reload from disk");
    d.wait_for_text("Package.md cannot be read (line 4)");
    assert!(!d.has_containing("Ember Reach"));
    assert_eq!(fs::read_to_string(&path).unwrap(), broken);
    d.click("Open in Library");
    d.expect("Markdown editor");
    d.expect("Line 4 opens a list");
    d.expect("Replace with a basic Package.md");
    d.expect(&crate::content_workspace::manifest_replace_consequence(
        "Package.md",
    ));
    d.observe(
        "iter-23-C-AE3",
        !d.has_containing("Keeps the id") && !d.has_containing("Drops the rest"),
        "Library states the replacement in Campaign’s words, without field names",
    );
    d.state_prepared("manifest-opened-for-repair", Sizes::All, |d| {
        let tone = manifest_fault_tone(d, "Line 4 opens a list");
        d.observe(
            "iter-23-C-AE4-library",
            tone == Some("error"),
            format!("The invalid manifest keeps its error tone in Library repair: {tone:?}"),
        );
        let about = d
            // Round 24 FC2: the invalid manifest's About row has the error mark.
            .place_of(&format!("{} About Ashen Marches", design::icons::ERROR))
            .expect("The marked About row is visible");
        let about_selected = d.app_mut().library.repairing_package_about();
        d.observe(
            "iter-9-E-FD3",
            about_selected && !d.visible_text("No entries in this section.") && about.width() > 0.0,
            "Manifest repair opens the package About row with its warning and content list",
        );
        check_repair_reason(d, true);
    });
    go_encounters(d);
    open_saved_encounter(d, "Bridge Ambush");
    d.expect("Cinder Wolf");
    assert!(!d.has_containing("Not found"));
    d.state("manifest-invalid-encounter-keeps-creatures", Sizes::All);
    fs::write(&path, &valid).unwrap();
    d.focus_window(false);
    d.focus_window(true);
    go_campaign(d);
    d.click("Reload from disk");
    assert!(!d.has_containing("Package.md cannot be read"));
    d.state("manifest-repaired", Sizes::All);
    fs::remove_file(&path).unwrap();
    d.focus_window(false);
    d.focus_window(true);
    d.click("Reload from disk");
    d.expect("The package manifest was deleted");
    assert!(!d.has("Find the folder…"));
    let deleted_tops = package_list_tops(d);
    d.state_prepared("manifest-deleted-campaign", Sizes::All, |d| {
        // Round 23 AE4, outside its scope: a deleted manifest stays a warning.
        let tone = manifest_fault_tone(d, "The package manifest was deleted");
        d.observe(
            "iter-23-C-AE4-deleted",
            tone == Some("warning"),
            format!("A deleted manifest is a warning in Campaign: {tone:?}"),
        );
        // Round 24 FF4: the package the warning names has the warning mark on its row, and
        // only that row is marked.
        let (rows, marked) = package_row_marks(d);
        d.observe(
            "iter-24-C-FF4",
            rows >= 2
                && warning_marked(&marked)
                && marked.len() == 1
                && d.has_containing("Ashen Marches · Manifest deleted · "),
            format!("{rows} package rows; marked rows {marked:?}"),
        );
        let search = d
            .place_of("Find a needed entry")
            .expect("The entry search is present");
        let save = d
            .place_of("Save selections")
            .expect("The save action is present");
        assert!(
            search.width() <= crate::design::LIST_MEASURE,
            "Search: {search:?}"
        );
        assert!(
            save.right() <= search.left() + crate::design::LIST_MEASURE,
            "Save: {save:?}; search: {search:?}"
        );
    });
    assert!(
        !path.exists(),
        "The manifest is deleted before it is written again"
    );
    d.click("Write the manifest again");
    d.settle();
    assert!(path.is_file(), "Write the manifest again writes the file");
    assert!(!d.has_containing("The package manifest was deleted"));
    assert_eq!(
        crate::library::local_module(&folder).unwrap().id,
        crate::content_workspace::note_id(&valid).unwrap()
    );
    d.expect("Package.md was written again.");
    d.expect("Undo write Package.md");
    // Round 26 FD1: package warnings and results sit in the page's footer, so the package list
    // keeps one y with an invalid manifest, a deleted one, and the written result with its Undo.
    let written_tops = package_list_tops(d);
    d.observe(
        "iter-26-C-FD1",
        invalid_tops.iter().all(Option::is_some)
            && deleted_tops == invalid_tops
            && written_tops == invalid_tops,
        format!(
            "Package list tops at 960, 1200 and 1920: invalid {invalid_tops:?}, deleted \
             {deleted_tops:?}, written again {written_tops:?}"
        ),
    );
    // Round 24 FF5: a missing and a replaced manifest give one result sentence and one Undo, so
    // this capture matches `manifest-replaced-with-undo`. The file above went from missing to
    // written; what differs is only what Undo restores.
    d.expect_same(
        "Writing a deleted manifest again says what replacing an invalid one says; only Undo's target differs: here it deletes the file again.",
    );
    d.state_prepared("manifest-written-again-campaign", Sizes::All, |d| {
        check_undo_icon(d, "Undo write Package.md", "iter-6-C-SD3-Package");
    });
    d.click("Undo write Package.md");
    d.wait_for_text("The package manifest was deleted");
    assert!(!path.exists());
    d.click("Write the manifest again");
    d.settle();
    fs::remove_file(&path).unwrap();
    d.focus_window(false);
    d.focus_window(true);
    go_library(d);
    search_library(d, "Cinder Wolf");
    d.click_starting("Cinder Wolf · ");
    d.expect("22 (4d8 + 4)");
    assert!(!path.exists());
    d.expect_same("Deleting the manifest keeps the creature reader and its repair count.");
    d.state("manifest-deleted-library-keeps-entries", Sizes::All);
    search_library(d, "");
    open_local_package(d, MODULE);
    d.click_containing("About Ashen Marches");
    d.expect("Write the manifest again");
    d.state_prepared("manifest-deleted-library-repair", Sizes::All, |d| {
        let read = d.place_of("Read").expect("Read tab is visible");
        let reader = egui::Rect::from_min_max(read.min, egui::pos2(10000.0, 10000.0));
        let words = d.painted_in(reader);
        d.observe(
            "iter-9-E-SD2-AD2-FD1",
            words.contains("Ashen Marches · Manifest deleted")
                && !words.contains("Package.md")
                && !words.contains("Module.md")
                && !words.contains("Show file")
                && words.contains("Show folder")
                && words.matches("Ashen Marches").count() >= 2,
            "A missing manifest shows its registered name and folder without a guessed file",
        );
        check_read_repair(d, "Ashen Marches");
        let tone = manifest_fault_tone(d, "The package manifest was deleted.");
        d.observe(
            "iter-23-C-AE4-deleted-library",
            tone == Some("warning"),
            format!("A deleted manifest is a warning in Library: {tone:?}"),
        );
        d.observe(
            "iter-8-E-FD8-deleted",
            !d.visible_text("Missing YAML frontmatter"),
            "A deleted manifest has only its missing-file warning",
        );
        let action = d
            .place_of("Write the manifest again")
            .expect("Manifest repair is visible");
        let text_edge = d
            .painted_left("The package manifest was deleted.")
            .expect("Missing-file warning is visible");
        let warning = d
            .painted_rect_in("The package manifest was deleted.", egui::Rect::EVERYTHING)
            .expect("The warning text is visible");
        // Round 27 FH9: `L` between the kind line and the warning below it.
        let kind = d.painted_word_rect("Package", reader);
        d.observe(
            "iter-27-L-FH9",
            kind.is_some_and(|kind| warning.top() - kind.bottom() >= design::L - 1.0),
            format!("The warning starts L below the kind line: kind {kind:?}, warning {warning:?}"),
        );
        let aligned = if action.top() >= warning.bottom() {
            (action.left() - text_edge).abs() <= 1.0
        } else {
            action.left() >= warning.right() && action.top() <= warning.bottom()
        };
        d.observe(
            "iter-8-E-FD9-edge",
            aligned,
            "Manifest repair sits beside the warning, or below at its text edge",
        );
        d.observe(
            "iter-8-E-FD9-border",
            d.control_frame("Write the manifest again")
                .is_some_and(|frame| {
                    frame.stroke.width > 0.0 && frame.stroke.color == design::colors().field_border
                }),
            "Manifest repair has a secondary border",
        );
    });
    d.click("Write the manifest again");
    d.settle();
    assert!(!d.has_containing("The package manifest was deleted"));
    d.expect("Package.md was written again.");
    d.expect("Undo write Package.md");
    d.state_prepared("manifest-written-again-library", Sizes::All, |d| {
        check_undo_icon(d, "Undo write Package.md", "iter-8-E-SD3-library");
    });
    go_encounters(d);
    d.expect("Cinder Wolf");
    assert!(!d.has_containing("Not found"));
    d.expect_same("Deleting the manifest keeps the encounter's creature entries as before.");
    d.assert_state_matches(
        "manifest-restored-encounter-keeps-creatures",
        "manifest-invalid-encounter-keeps-creatures",
    );
    // A plain note at a manifest name stays user data, through both UI write routes.
    fs::remove_file(&path).unwrap();
    let readme = "Notes on this adventure.\n";
    fs::write(&path, readme).unwrap();
    d.focus_window(false);
    d.focus_window(true);
    go_campaign(d);
    d.click("Reload from disk");
    d.expect("The package manifest was deleted");
    assert!(!d.has_containing("Replace with a basic"));
    d.click("Write the manifest again");
    d.wait_for_text("Module.md was written again.");
    assert_eq!(fs::read(&path).unwrap(), readme.as_bytes());
    d.click("Undo write Module.md");
    d.wait_for_text("The package manifest was deleted");
    assert!(!folder.join("Module.md").exists());
    go_library(d);
    open_local_package(d, MODULE);
    d.click_containing("About Ashen Marches");
    d.expect("Write the manifest again");
    assert!(!d.has_containing("Replace with a basic"));
    d.observe(
        "iter-10-E-SD3",
        !d.has_containing("Package.md was written again")
            && !d.has_containing("Undo write Package.md"),
        "A write result whose file became a README clears with its Undo",
    );
    d.observe(
        "iter-10-E-SD3-readme",
        fs::read(&path).is_ok_and(|bytes| bytes == readme.as_bytes()),
        "The README at the manifest name keeps its bytes",
    );
    // Without the stale result, the view is the deleted-manifest repair again.
    d.assert_state_matches(
        "plain-readme-missing-manifest",
        "manifest-deleted-library-repair",
    );
    d.click("Write the manifest again");
    d.wait_for_text("Module.md was written again.");
    assert_eq!(fs::read(&path).unwrap(), readme.as_bytes());
    d.state("manifest-written-beside-readme", Sizes::All);
    d.click("Undo write Module.md");
    d.wait_for_text("The package manifest was deleted");
    assert!(!folder.join("Module.md").exists());
    assert_eq!(fs::read(&path).unwrap(), readme.as_bytes());
}

pub fn package_file_changed(d: &mut Driver<'_>) {
    let path = d
        .root
        .join("modules/ashen-marches/Creatures/Cinder Wolf.md");
    go_library(d);
    search_library(d, "Cinder Wolf");
    d.click_starting("Cinder Wolf · ");
    d.wait_for_text("Bite");
    d.state("package-before-disk-edit", Sizes::All);
    let before = fs::read_to_string(&path).unwrap();
    let changed = before
        .replace("max_hp: {value: 22}", "max_hp: {value: 28}")
        .replace(
            "hit_points_text: '22 (4d8 + 4)'",
            "hit_points_text: '28 (4d8 + 10)'",
        )
        .replace("HP 22 (4d8 + 4)", "HP 28 (4d8 + 10)")
        .replace("Bite.", "Cinder Bite.");
    fs::write(&path, &changed).unwrap();
    d.focus_window(false);
    d.focus_window(true);
    d.wait_for_text("Cinder Bite");
    d.expect_same("The disk edit renames the Bite action, and only its tile's name changes.");
    d.state("package-edit-round-trip", Sizes::All);
    look_up(d, "Cinder Wolf");
    d.wait_for_text("Cinder Bite");
    d.state("changed-package-in-lookup", Sizes::All);
    d.press(Key::Escape);
    d.click(EDIT_MARKDOWN);
    d.type_into_editor("The draft stays safe.\n");
    fs::write(&path, changed.replace("Cinder Bite.", "Ash Bite.")).unwrap();
    d.focus_window(false);
    d.focus_window(true);
    d.wait_for_text("This entry changed on disk. Your draft is safe until you choose.");
    d.expect("Overwrite with my draft");
    d.state_prepared("package-draft-changed-on-disk", Sizes::All, |d| {
        choices_at_text(
            d,
            "iter-15-E-AD3-library-package",
            "This entry changed on disk",
            "Overwrite with my draft",
            "Reload and lose my draft",
            None,
        );
    });
    d.click("Reload and lose my draft");
    d.click("Read");
    // The attribute table's rows are pointer targets, so the actions can start lower.
    d.scroll_at(super::READER, 3);
    d.wait_for_text("Ash Bite");
    d.expect("Undo discard of Cinder Wolf");
    assert!(!d.app_mut().library.dirty());
    d.state("package-disk-version-kept", Sizes::All);

    d.click("What the Library shows: Can use");
    d.click("Ashen Marches");
    let valid = fs::read_to_string(&path).unwrap();
    let broken = valid.replace("splat_kind: creature", "splat_kind: [");
    assert_ne!(broken, valid);
    fs::write(&path, &broken).unwrap();
    d.focus_window(false);
    d.focus_window(true);
    d.wait_for_text("Cannot be read");
    d.state_prepared("package-yaml-broken-on-disk", Sizes::All, |d| {
        check_unreadable_owner(d, "iter-8-E-FD10-entry");
        check_read_repair(d, "Creatures/Cinder Wolf.md");
        let fix = d
            .place_of("Fix in Edit Markdown")
            .expect("The direct repair is visible");
        let error = d
            .painted_rect_in("Line 4 opens a list", egui::Rect::EVERYTHING)
            .expect("The error is visible");
        d.observe(
            "iter-9-E-FD5",
            fix.top() >= error.bottom()
                && (fix.left() - error.left()).abs() <= 1.0
                && d.control_frame("Fix in Edit Markdown")
                    .is_some_and(|frame| frame.stroke.width > 0.0),
            "The error offers bordered repair below at its text edge",
        );
    });
    d.click("Fix in Edit Markdown");
    d.expect("Save");
    assert!(
        !d.has_containing("Undo discard"),
        "a later disk edit clears the discarded draft"
    );
    let visible_frame = |d: &Driver<'_>| {
        d.input_frame("Markdown editor")
            .is_some_and(|(frame, clip)| clip.expand(1.0).contains_rect(frame))
    };
    d.observe_both(
        "AD2",
        "All source frame edges remain visible",
        visible_frame,
    );
    assert!(d.at_size((1920.0, 1080.0), visible_frame));
    d.state_prepared("package-yaml-repair-editor", Sizes::All, |d| {
        check_repair_reason(d, false);
        d.observe(
            "iter-8-E-FD4-visible",
            d.visible_passage("splat_kind: ["),
            "Edit Markdown opens with the faulty line in view",
        );
        d.observe(
            "iter-8-E-FD4-focus",
            d.focused_name().as_deref() == Some("Markdown editor"),
            "The repair editor has keyboard focus",
        );
    });
    // Typing without a helper click proves that the caret is at the error line.
    d.type_text("repair ");
    d.observe(
        "iter-8-E-FD4-caret",
        d.editor_text().as_deref()
            == Some(
                broken
                    .replacen("splat_kind: [", "repair splat_kind: [", 1)
                    .as_str(),
            ),
        "Typing lands at the start of the error line",
    );
    d.press_ctrl(Key::Z);
    assert_eq!(d.editor_text().as_deref(), Some(broken.as_str()));
    d.type_text("draft ");
    let unsent_repair = d.editor_text().unwrap();
    assert_ne!(unsent_repair, broken);
    // Round 24 AF1, FF1, SF1: the discard result takes the slot reserved below the parse error,
    // so the editor keeps its y before Discard, after it and after Undo discard.
    let editor_tops = |d: &mut Driver<'_>| {
        [SMALL, DEFAULT, LARGE]
            .map(|size| d.at_size(size, |d| Some(d.place_of("Markdown editor")?.top())))
    };
    let typed_tops = editor_tops(d);
    d.click("Discard");
    assert_eq!(d.editor_text().as_deref(), Some(broken.as_str()));
    d.expect("Invalid");
    d.expect("Line 4 opens a list");
    d.expect("Undo discard of Creatures/Cinder Wolf.md");
    let discarded_tops = editor_tops(d);
    d.state("repair-draft-discarded", Sizes::All);
    d.click("Undo discard of Creatures/Cinder Wolf.md");
    assert_eq!(d.editor_text().as_deref(), Some(unsent_repair.as_str()));
    d.expect("Invalid");
    d.expect("Line 4 opens a list");
    let undone_tops = editor_tops(d);
    d.observe(
        "iter-24-L-AF1",
        typed_tops.iter().all(Option::is_some)
            && typed_tops == discarded_tops
            && discarded_tops == undone_tops,
        format!(
            "The repair editor keeps its y: typed {typed_tops:?}, discarded {discarded_tops:?}, undone {undone_tops:?}"
        ),
    );
    d.replace_editor_text(&broken);
    let before_scroll = d.input_frame("Markdown editor").unwrap().0;
    d.scroll_at(before_scroll.center(), 3);
    assert_eq!(
        d.input_frame("Markdown editor").unwrap().0,
        before_scroll,
        "Only the source text scrolls"
    );
    d.replace_editor_text(&valid);
    d.click("Save");
    d.click("Read");
    // The attribute table's rows are pointer targets, so the actions can start lower.
    d.scroll_at(super::READER, 3);
    d.wait_for_text("Ash Bite");
    d.expect("Ashen Marches · Chosen for this campaign");
    d.state("package-yaml-repaired", Sizes::All);

    // Keep the original repair proof at step 08; repeat the break for its consumers.
    fs::write(&path, &broken).unwrap();
    d.focus_window(false);
    d.focus_window(true);
    go_encounters(d);
    open_saved_encounter(d, "Bridge Ambush");
    d.expect("Cinder Wolf.md cannot be read (line 4)");
    assert!(!d.has("Replacement creature"));
    d.state("invalid-package-in-encounter", Sizes::All);
    d.click("More actions for Bridge Ambush");
    d.click("Start another fight");
    d.expect("Cinder Wolf");
    assert!(!d.app_mut().encounters.has_run());
    d.state("invalid-package-blocks-new-fight", Sizes::All);
    // iter-19-B-FD5: the message and the two rows that state the same fault share its error tone.
    let (one_tone, detail) = fault_tone(
        d,
        "Cinder Wolf.md cannot be read (line 4). Open in Library to fix it.",
        "Cinder Wolf.md cannot be read (line 4)",
        (design::icons::ERROR, design::colors().result.error),
    );
    d.observe("iter-19-B-FD5", one_tone, detail);
    d.click("Open in Library");
    d.expect("Markdown editor");
    d.expect("Line 4 opens a list");
    d.state("invalid-package-opened-for-repair", Sizes::All);
    go_encounters(d);
    d.click("Back to encounters");
    resume_run_from_cold_start(d);
    d.click_row("Cinder Wolf 1");
    d.click("Stat block");
    d.expect("Bite");
    // iter-27-R FD3: a file that cannot be read now is an error in the run's panel too.
    let mut error_icon = false;
    let mut unread = false;
    d.each_painted_text(|text, _| {
        let job = &text.galley.job;
        if job.text == design::icons::ERROR
            && job.sections.first().map(|s| s.format.color) == Some(design::colors().result.error)
        {
            error_icon = true;
        }
        unread |= job.text.contains("Cinder Wolf.md cannot be read now.");
    });
    d.observe(
        "iter-27-R-FD3",
        error_icon && unread,
        format!("The run's notice is an error: icon {error_icon}, words {unread}"),
    );
    // iter-27-R FD5: the notice is in the panel's footer, so the stat block keeps its top when
    // the notice grows by `Read the new version` after the repair.
    let bite_y = |d: &mut Driver<'_>| {
        [(960.0, 640.0), (1200.0, 800.0), (1920.0, 1080.0)].map(|size| {
            d.at_size(size, |d| {
                d.painted_rect_in("Bite", egui::Rect::EVERYTHING)
                    .map(|rect| rect.top().round())
            })
        })
    };
    let unreadable_y = bite_y(d);
    d.state("invalid-package-in-paused-run", Sizes::All);
    footer_presses(
        d,
        "iter-27-R-FD5-press-appears",
        Some("Cinder Wolf.md cannot be read now."),
        false,
    );
    close_run(d);
    go_library(d);
    search_library(d, "Cinder Wolf");
    d.click_starting("Creatures/Cinder Wolf.md · ");
    d.click(EDIT_MARKDOWN);
    d.replace_editor_text(&valid);
    d.click("Save");
    d.click("Read");
    d.wait_for_text("Ashen Marches · Chosen for this campaign");
    go_encounters(d);
    open_saved_encounter(d, "Bridge Ambush");
    assert!(!d.has_containing("Not found. Its package entry was deleted."));
    d.state("repaired-package-in-encounter", Sizes::All);
    d.click("Back to encounters");
    resume_run_from_cold_start(d);
    d.click_row("Cinder Wolf 1");
    d.click("Stat block");
    d.expect("Bite");
    d.expect_same("The paused run keeps its saved creature snapshot across a package repair.");
    let repaired_y = bite_y(d);
    d.observe(
        "iter-27-R-FD5",
        unreadable_y.iter().all(Option::is_some)
            && unreadable_y == repaired_y
            && d.has("Read the new version"),
        format!(
            "Bite's y with the unreadable notice {unreadable_y:?}, after the repair {repaired_y:?}"
        ),
    );
    d.state("repaired-package-in-paused-run", Sizes::All);
    footer_presses(
        d,
        "iter-27-R-FD5-press-grows",
        Some("Cinder Wolf.md has changed."),
        true,
    );
    // Another row's stat block, from a file that did not change: the footer is empty.
    d.click_row("Ember Cultist");
    if !d.has(SOURCE_SECTION) {
        d.click("Stat block");
    }
    footer_presses(d, "iter-27-R-FD5-press-clears", None, false);

    // iter-22-E-SD1: a repair draft's notice and save-row state come from its current parse. The
    // saved file stays unreadable on its owner line until Save or a newer file.
    close_run(d);
    fs::write(&path, &broken).unwrap();
    d.focus_window(false);
    d.focus_window(true);
    go_library(d);
    search_library(d, "Cinder Wolf");
    d.click_starting("Creatures/Cinder Wolf.md · ");
    d.click(EDIT_MARKDOWN);
    d.expect("Line 4 opens a list");
    d.expect("Invalid");
    // A new fault on another line, so each step's notice is one no earlier step showed.
    let other_fault = valid.replace("splat_name: Cinder Wolf", "splat_name: [");
    assert_ne!(other_fault, valid);
    // The saved file's stored error, as the folder read gives it: the old code fell back to it.
    let stored = splat::content::Entry::parse("Creatures/Cinder Wolf.md", &broken).unwrap_err();
    let stored = stored.split(" at line").next().unwrap().to_owned();
    assert!(!stored.contains("opens a list"), "{stored}");
    fn sd1(
        d: &mut Driver<'_>,
        step: &str,
        notice: Option<&str>,
        word: &str,
        owner: &str,
        stored: &str,
    ) {
        let shown = notice.is_none_or(|notice| d.has_containing(notice));
        let gone = ["Line 4 opens a list", "Line 6 opens a list", stored]
            .into_iter()
            .filter(|other| notice != Some(*other))
            .all(|other| !d.has_containing(other));
        d.observe(
            &format!("iter-22-E-SD1-{step}-notice"),
            shown && gone,
            format!("The notice is {notice:?}; no other line fault and no stored error shows"),
        );
        // A status node is its icon, a space and its word.
        let state = |word: &str| {
            d.nodes()
                .iter()
                .any(|node| node.name.ends_with(&format!(" {word}")))
        };
        let others = ["Invalid", "Unsaved", "No changes"]
            .into_iter()
            .filter(|other| *other != word)
            .all(|other| !state(other));
        d.observe(
            &format!("iter-22-E-SD1-{step}-state"),
            state(word) && others,
            format!("The save row says only {word}"),
        );
        d.observe(
            &format!("iter-22-E-SD1-{step}-owner"),
            d.has_containing(owner),
            format!("The owner line says {owner}"),
        );
    }
    const UNREADABLE: &str = "Ashen Marches · Cannot be read";
    // Round 25 AC1, SC1: the parse error is in the one message space below the save row, so
    // the editor keeps its top when the error clears, comes back, or Discard and its Undo run.
    let mut repair_tops = vec![editor_frame_tops(d)];
    d.replace_editor_text(&valid);
    sd1(d, "valid", None, "Unsaved", UNREADABLE, &stored);
    repair_tops.push(editor_frame_tops(d));
    d.state("repair-draft-parses", Sizes::All);
    d.replace_editor_text(&other_fault);
    sd1(
        d,
        "invalid-again",
        Some("Line 6 opens a list"),
        "Invalid",
        UNREADABLE,
        &stored,
    );
    repair_tops.push(editor_frame_tops(d));
    d.press_ctrl(Key::Z);
    assert_eq!(d.editor_text().as_deref(), Some(valid.as_str()));
    sd1(d, "editor-undo", None, "Unsaved", UNREADABLE, &stored);
    d.click("Discard");
    assert_eq!(d.editor_text().as_deref(), Some(broken.as_str()));
    sd1(
        d,
        "discard",
        Some("Line 4 opens a list"),
        "Invalid",
        UNREADABLE,
        &stored,
    );
    repair_tops.push(editor_frame_tops(d));
    d.click("Undo discard of Creatures/Cinder Wolf.md");
    assert_eq!(d.editor_text().as_deref(), Some(valid.as_str()));
    sd1(d, "undo-discard", None, "Unsaved", UNREADABLE, &stored);
    repair_tops.push(editor_frame_tops(d));
    d.observe(
        "iter-25-L-AC1-editor-still",
        repair_tops[0].iter().all(Option::is_some)
            && repair_tops.iter().all(|tops| *tops == repair_tops[0]),
        format!(
            "The editor top stays with the error, without it, after Discard and Undo discard: {repair_tops:?}"
        ),
    );
    // An outside change under the valid draft: Reload takes the file's own fault.
    fs::write(&path, &other_fault).unwrap();
    d.focus_window(false);
    d.focus_window(true);
    d.wait_for_text("This entry changed on disk. Your draft is safe until you choose.");
    d.click("Reload and lose my draft");
    assert_eq!(d.editor_text().as_deref(), Some(other_fault.as_str()));
    sd1(
        d,
        "reload",
        Some("Line 6 opens a list"),
        "Invalid",
        UNREADABLE,
        &stored,
    );
    // An outside repair with no draft: the file reads again, and nothing is left to fix.
    fs::write(&path, &valid).unwrap();
    d.focus_window(false);
    d.focus_window(true);
    d.wait_for_text("Ashen Marches · Chosen for this campaign");
    sd1(
        d,
        "outside-repair",
        None,
        "No changes",
        "Ashen Marches · Chosen for this campaign",
        &stored,
    );
}

/// Deleting a file leaves its uses visible and its reader link unresolved.
pub fn package_delete_entry(d: &mut Driver<'_>) {
    let package = d.root.join("modules/ashen-marches");
    let rules = package.join("Rules/Ash Exposure.md");
    fs::write(
        &rules,
        format!(
            "{}\n\n[[Cinder Wolf]]\n",
            fs::read_to_string(&rules).unwrap()
        ),
    )
    .unwrap();
    // A's file as the scenario starts, to bring A back between the Remove cases at the end.
    let a_text = fs::read_to_string(&rules).unwrap();
    go_library(d);
    search_library(d, "Ash Exposure");
    d.click_starting("Ash Exposure · ");
    d.wait_for_text("Cinder Wolf");
    d.state("package-entry-before-delete", Sizes::All);
    fs::remove_file(package.join("Creatures/Cinder Wolf.md")).unwrap();
    d.focus_window(false);
    d.focus_window(true);
    go_encounters(d);
    open_saved_encounter(d, "Bridge Ambush");
    d.wait_for_text("Not found. Its package entry was deleted.");
    d.observe(
        "Missing creature names",
        d.has_containing("Cinder Wolf") && !d.has_containing("actor_wolf_"),
        "Deleted entries keep their last known name and hide reference keys",
    );
    // iter-28-B FC3: the roster heading's reason names the deleted creature as its rows do.
    d.observe(
        "iter-28-B-FC3",
        d.has_containing("XP not known: Cinder Wolf") && !d.has_containing("Unknown creature"),
        "The difficulty reason names the rows' creature, Cinder Wolf",
    );
    d.state("deleted-package-entry-in-encounter", Sizes::All);
    d.click("More actions for Bridge Ambush");
    d.click("Start another fight");
    assert!(!d.app_mut().encounters.has_run());
    d.state("deleted-package-blocks-new-fight", Sizes::All);
    // iter-19-B-FD5 must stay true: a deleted entry stays a warning in the message and the rows.
    let (warned, detail) = fault_tone(
        d,
        "2 Cinder Wolf rows have no creature.",
        "Not found. Its package entry was deleted.",
        (design::icons::WARN, design::colors().result.warning),
    );
    d.observe("iter-19-B-FD5-deleted", warned, detail);
    go_library(d);
    search_library(d, "Ash Exposure");
    d.click_starting("Ash Exposure · ");
    d.wait_for_text("Cinder Wolf");
    let unresolved = d
        .app_mut()
        .lookup
        .index()
        .unwrap()
        .reader_markdown("[[Cinder Wolf]]", None)
        .contains("splat-missing:");
    d.observe(
        "Deleted package link",
        unresolved,
        "The deleted creature's words remain without a working link",
    );
    d.expect_same(
        "The note keeps its words and layout; only the deleted creature link becomes unavailable.",
    );
    d.state("deleted-package-entry-reader-link", Sizes::All);
    // Round 23 FD8: the row's warning is a status: its icon has the warning colour, before the
    // muted words, as in the reader and the roster.
    let rows = row_marks(d, "1 broken link");
    d.observe(
        "iter-23-L-FD8",
        warning_marked(&rows),
        format!("The Library row marks its broken link with the warning icon in the warning colour: {rows:?}"),
    );
    go_encounters(d);
    d.click_beside(
        "6. Cinder Wolf",
        "Options: starting HP, temporary HP, initiative group and changes",
    );
    d.expect("Replacement creature");
    d.expect("Replace all 2 Cinder Wolf rows");
    d.type_into("Replacement creature", "Guard");
    d.expect("Use Guard");
    d.state("deleted-package-entry-choose-replacement", Sizes::All);
    // iter-26-B FA2: the replacement's result sits in the editor's footer, so the roster's
    // heading and its first row keep their y with the result, at 960, 1200 and 1920.
    let roster_tops = |d: &mut Driver<'_>| {
        [SMALL, DEFAULT, LARGE].map(|size| {
            d.at_size(size, |d| {
                // The heading, or the roster's tab where the panes are tabs.
                ["ROSTER", "1. "].map(|start| {
                    d.nodes()
                        .into_iter()
                        .find(|node| node.name.to_uppercase().starts_with(start))
                        .map(|node| node.rect.top().round())
                })
            })
        })
    };
    let before_result = roster_tops(d);
    d.click("Use Guard");
    d.expect("as\u{a0}Guard");
    d.expect("Undo replacement");
    let with_result = roster_tops(d);
    d.observe(
        "iter-26-B-FA2",
        before_result.iter().flatten().all(Option::is_some) && with_result == before_result,
        format!("Roster heading and row 1 tops before {before_result:?} and with the result {with_result:?}"),
    );
    assert!(!d.has_containing("Not found. Its package entry was deleted."));
    assert!(
        !d.has_containing("have no creature")
            && !d.has_containing("Choose a replacement in the roster.")
    );
    let dirty = d.app_mut().encounters.draft_dirty();
    d.observe(
        "Replace deleted creature",
        dirty,
        "Choosing a replacement edits the draft; it does not save it",
    );
    d.state("deleted-package-entry-replaced", Sizes::All);
    // iter-23 FD10: on a replaced row, the separator before the change link has the same gap on
    // each side as the separators between the stats.
    for size in [SMALL, DEFAULT, LARGE] {
        let gaps = d.at_size(size, |d| {
            let mut line = None;
            d.each_painted_text(|text, clip| {
                let words = text.galley.job.text.replace('\u{a0}', " ");
                if line.is_none()
                    && words.ends_with("as Guard")
                    && words.contains('·')
                    && clip.intersects(text.visual_bounding_rect())
                {
                    line = Some(text.visual_bounding_rect());
                }
            });
            let line = line?;
            let mut glyphs = Vec::new();
            d.each_painted_text(|text, clip| {
                if !clip.intersects(text.visual_bounding_rect()) {
                    return;
                }
                for row in &text.galley.rows {
                    for glyph in &row.glyphs {
                        let x = text.pos.x + row.pos.x + glyph.pos.x;
                        let y = text.pos.y + row.pos.y + glyph.pos.y;
                        if x >= line.left() - 0.5 && (y - line.bottom()).abs() <= line.height() {
                            glyphs.push((x, x + glyph.advance_width, glyph.chr));
                        }
                    }
                }
            });
            glyphs.sort_by(|a, b| a.0.total_cmp(&b.0));
            let solid: Vec<_> = glyphs
                .into_iter()
                .filter(|(_, _, chr)| !chr.is_whitespace())
                .collect();
            Some(
                solid
                    .windows(3)
                    .filter(|three| three[1].2 == '·')
                    .map(|three| (three[1].0 - three[0].1, three[2].0 - three[1].1))
                    .collect::<Vec<_>>(),
            )
        });
        // The last separator is the link's; it matches the mean of the stats' separators on
        // each side within 1 px (glyph bearings differ by a little from digit to letter).
        let even = gaps.as_ref().is_some_and(|gaps| {
            let Some(((left, right), text)) = gaps.split_last() else {
                return false;
            };
            let mean = |side: fn(&(f32, f32)) -> f32| {
                text.iter().map(side).sum::<f32>() / text.len() as f32
            };
            text.len() >= 2
                && (left - mean(|gap| gap.0)).abs() <= 1.0
                && (right - mean(|gap| gap.1)).abs() <= 1.0
        });
        d.observe(
            "iter-23-B-FD10",
            even,
            format!("At {size:?}, the gaps around each separator of the replaced row: {gaps:?}"),
        );
    }
    // iter-18-B: the replacement's result is a short message, so its Undo keeps the undo icon as
    // every other Undo does, and its press area stays clear of the splitter.
    d.observe_both(
        "iter-18-B-replacement-message",
        "The replacement says what changed, with Undo replacement beside it",
        |d| d.has_containing("2 Cinder Wolf rows replaced with Guard."),
    );
    // FC4: a finished change is a success message, with the success bar.
    let replaced = d.painted_rect_in(
        "2 Cinder Wolf rows replaced with Guard.",
        egui::Rect::EVERYTHING,
    );
    d.observe(
        "iter-25-B-FC4",
        replaced.is_some_and(|words| d.has_tone_bar(words, design::colors().result.success)),
        format!("The replacement result {replaced:?} has the success bar"),
    );
    for size in [SMALL, DEFAULT, LARGE] {
        let (icon, clear) = d.at_size(size, |d| {
            let target = d.place_of("Undo replacement");
            (
                target.map(|rect| d.painted_in(rect)),
                target.map_or_else(
                    || Err("Undo replacement is not in view".to_owned()),
                    |target| super::press_area_clear(d, target, "Undo replacement", None),
                ),
            )
        });
        d.observe(
            "iter-18-B-undo-replacement-icon",
            icon.as_deref()
                .is_some_and(|painted| painted.contains(crate::design::icons::UNDO)),
            format!("At {size:?}, Undo replacement paints {icon:?}"),
        );
        d.observe(
            "iter-17-B-undo-replacement-clear",
            clear.is_ok(),
            format!("At {size:?}, Undo replacement's press area: {clear:?}"),
        );
    }
    d.observe_both("W8", "Changed keeps the same roster row pitch", |d| {
        let top = |name: &str| {
            d.nodes()
                .into_iter()
                .find(|node| node.name.starts_with(name))
                .unwrap()
                .rect
                .top()
        };
        let pitch = top("6. Cinder Wolf") - top("5. Sable Harrowgate");
        (top("7. Cinder Wolf") - top("6. Cinder Wolf") - pitch).abs() < 1.0
            && (top("8. Ember Cultist") - top("7. Cinder Wolf") - pitch).abs() < 1.0
    });
    // iter-26-B FC3: Undo before any Save gives back the file's rows, so the draft is clean:
    // `No changes`, no Discard, and the campaign switch is open.
    let bridge = d
        .app_mut()
        .campaigns
        .active_campaign()
        .unwrap()
        .path
        .join("Encounters/Bridge Ambush.md");
    let on_disk = fs::read(&bridge).unwrap();
    let replaced_dirty = d.app_mut().encounters.draft_dirty();
    d.click("Undo replacement");
    d.expect("Not found. Its package entry was deleted.");
    let clean = !d.app_mut().encounters.draft_dirty();
    d.observe(
        "iter-26-B-FC3",
        replaced_dirty
            && clean
            && d.visible_text("No changes")
            && !d.visible_text("Invalid")
            && !d.visible_text("Unsaved")
            && !d.has("Discard")
            && fs::read(&bridge).unwrap() == on_disk,
        format!(
            "Undo replacement before Save: dirty before {replaced_dirty}, clean after {clean}; the save row says No changes"
        ),
    );
    // The same roster in the same state as before the replacement: one roster, one state.
    d.assert_state_matches(
        "deleted-package-entry-replacement-undone",
        "deleted-package-blocks-new-fight",
    );
    // iter-24-B FC1: Save would refuse a changed draft with these rows (2 rows have no
    // creature), so it says Invalid, not Unsaved, and Save and Ctrl+S are blocked: the file keeps
    // its bytes. Putting the name back makes the draft clean again.
    d.type_into("Encounter name", " at dusk");
    let invalid =
        d.visible_text("Invalid") && !d.visible_text("Unsaved") && d.is_disabled(SAVE_ENCOUNTER);
    d.press_ctrl(Key::S);
    d.observe(
        "iter-24-B-FC1",
        invalid
            && fs::read(&bridge).unwrap() == on_disk
            && d.visible_text("Invalid")
            && d.has_containing("2 Cinder Wolf rows have no creature."),
        format!("Invalid before Ctrl+S: {invalid}; the file and the state stay after it"),
    );
    d.replace_text_in("Encounter name", "Bridge Ambush");
    d.expect("No changes");
    // A start saves the replacement, and that Save ends the replacement's result and Undo: after
    // it, Undo would give a draft that differs from the file while it looks like it.
    d.click_beside(
        "6. Cinder Wolf",
        "Options: starting HP, temporary HP, initiative group and changes",
    );
    d.expect("Replacement creature");
    d.replace_text_in("Replacement creature", "Guard");
    d.click("Use Guard");
    d.expect("Undo replacement");
    d.click("More actions for Bridge Ambush");
    d.click("Start another fight");
    d.expect("Begin combat");
    d.state("replacement-starts-another-fight", Sizes::All);
    close_run(d);
    go_encounters(d);
    d.expect("as\u{a0}Guard");
    let written = fs::read_to_string(&bridge).unwrap();
    let (changed, undo, clean) = (
        written != String::from_utf8_lossy(&on_disk),
        d.has("Undo replacement"),
        d.visible_text("Saved") && !d.visible_text("Unsaved") && !d.visible_text("Invalid"),
    );
    d.observe(
        "iter-26-B-FC3-save-ends",
        changed && !undo && clean,
        format!(
            "The start saved the replacement ({changed}), and the Save ended its result and Undo (Undo shown: {undo}; Saved and clean: {clean})"
        ),
    );
    go_library(d);
    search_library(d, "Ash Exposure");
    d.click_starting("Ash Exposure · ");
    d.click(EDIT_MARKDOWN);
    d.type_into_editor("The draft survives a deleted file.\n");
    fs::remove_file(&rules).unwrap();
    d.focus_window(false);
    d.focus_window(true);
    d.wait_for_text("This entry’s file was deleted on disk. Your draft is safe until you choose.");
    d.expect("Ember Cultist needs this entry.");
    d.state_prepared("draft-file-deleted-on-disk", Sizes::All, |d| {
        choices_at_text(
            d,
            "iter-15-E-AD3-library-gone",
            "This entry’s file was deleted on disk",
            "Save my draft again",
            "Remove entry and lose draft",
            None,
        );
        // The note about who needs the entry sits with the message, at its text edge.
        let text = d.painted_rect_in("This entry’s file was deleted", egui::Rect::EVERYTHING);
        let needs = d.painted_rect_in("Ember Cultist needs this entry.", egui::Rect::EVERYTHING);
        d.observe(
            "iter-15-E-AD3-library-gone-needs",
            text.zip(needs)
                .is_some_and(|(text, needs)| (needs.left() - text.left()).abs() < 1.0),
            format!("The dependents line starts at the message text: {text:?}, {needs:?}"),
        );
        // Round 23 FD2: no link offers the file that has gone; the line offers its folder.
        d.observe(
            "iter-23-L-FD2",
            !d.has("Show file") && d.has("Show folder") && !d.has("Edit in Obsidian"),
            "The deleted draft's file line offers Show folder, not Show file",
        );
        let draft = d.place_of("Your draft").expect("The lone draft is visible");
        d.observe(
            "iter-13-E-AD3",
            draft.width() <= crate::library::EDITOR_MAX_WIDTH + 1.0
                && draft.width() >= design::XXL * 4.0,
            format!(
                "A deleted file's lone draft keeps the source editor width: {:.0} px",
                draft.width()
            ),
        );
    });
    d.expect("YOUR DRAFT");
    d.click("Save my draft again");
    d.wait_for_text("Saved");
    reveal_editor_passage(d, "The draft survives a deleted file.");
    assert!(
        fs::read_to_string(&rules)
            .unwrap()
            .contains("The draft survives a deleted file.")
    );
    d.state("deleted-draft-file-saved", Sizes::All);
    d.click("Read");
    d.expect("The draft survives a deleted file.");
    d.click(EDIT_MARKDOWN);
    d.type_into_editor("A draft to let go.\n");
    fs::remove_file(&rules).unwrap();
    d.focus_window(false);
    d.focus_window(true);
    d.wait_for_text("Remove entry and lose draft");
    let gone_count = files_to_fix(d);
    let gone_used = d.visible_text("Ashen Marches · Chosen for this campaign");
    d.click("Remove entry and lose draft");
    d.wait_for_text("No entry open");
    d.expect(
        "Ash Exposure removed. Its draft was discarded. Ember Cultist now needs a missing entry.",
    );
    assert!(!d.has_containing("Ash Exposure · "));
    d.expect("Undo remove");
    d.state("deleted-draft-let-go", Sizes::All);
    let let_go_count = files_to_fix(d);
    d.click("Undo remove");
    d.expect("Remove entry and lose draft");
    d.expect("A draft to let go.");
    // Round 23 FD1: Undo remove gives back the entry's campaign use and the header count with
    // the draft, as one step.
    let restored_count = files_to_fix(d);
    d.observe(
        "iter-23-L-FD1",
        gone_used
            && d.visible_text("Ashen Marches · Chosen for this campaign")
            && !d.visible_text("Ashen Marches · Not chosen")
            && !d.has("Choose for this campaign")
            && gone_count.is_some()
            && restored_count == gone_count,
        format!(
            "Undo remove restores Chosen for this campaign and the count {gone_count:?} (after Remove entry and lose draft {let_go_count:?}, after Undo {restored_count:?})"
        ),
    );
    d.expect_same(
        "Undo remove gives back the deleted-draft state of the first deletion: only the draft's words differ.",
    );
    d.state("deleted-draft-restored", Sizes::All);
    d.click("Remove entry and lose draft");
    d.wait_for_text("No entry open");
    d.focus_window(false);
    d.focus_window(true);
    d.expect("No entry open");
    assert!(!d.has_containing("Ash Exposure · "));
    d.expect_same("The deleted entry stays absent after focus returns.");
    d.assert_state_matches("deleted-draft-after-focus", "deleted-draft-let-go");
    // Astra review of round 25: an unrelated choice B between Remove (A) and Undo remove. B
    // writes the campaign file and refreshes the Library; A's Undo belongs to A's file and stays.
    let (entries, _) = splat::content::read_directory(&package).unwrap();
    let wraith = entries.iter().find(|e| e.name == "Ash Wraith").unwrap().id;
    let wraith_chosen = |d: &mut Driver<'_>| {
        let campaign = d.app_mut().campaigns.active_campaign().unwrap();
        crate::content_workspace::CampaignDocument::load(&campaign)
            .unwrap()
            .selections
            .iter()
            .any(|selection| match &selection.selection {
                crate::content_workspace::Selection::Selected { entries } => {
                    entries.iter().any(|entry| entry.id == Some(wraith))
                }
                _ => false,
            })
    };
    let b_before = wraith_chosen(d);
    // B is the campaign choice that `Use it` in encounter search and `Choose for this campaign`
    // both send, routed by the app to `use_in_campaign` and then `refresh_destination` for the
    // campaign folder. The search cannot offer `Use it` for Ash Wraith here: the built-in Wraith
    // matches as typed, and the offer shows only when nothing does. The Library cannot open Ash
    // Wraith either: opening another entry is Remove's own end.
    go_encounters(d);
    let module = d.app_mut().campaigns.modules()[0].id;
    d.app_mut().library.use_entry = Some((module, wraith));
    d.settle();
    let b_after = wraith_chosen(d);
    go_library(d);
    d.wait_for_text("Undo remove");
    d.click("Undo remove");
    d.expect("Remove entry and lose draft");
    let a_back = d.has_containing("A draft to let go.")
        && d.visible_text("Ashen Marches · Chosen for this campaign");
    let b_kept = wraith_chosen(d);
    d.observe(
        "iter-25-L-undo-remove-after-b",
        !b_before && b_after && b_kept && a_back,
        format!(
            "B (Ash Wraith) chosen between Remove and Undo remove: before {b_before}, after {b_after}, after Undo {b_kept}; A's draft and chosen state back: {a_back}"
        ),
    );
    // Astra re-check of round 25: Remove's Undo belongs to A in this campaign. Selecting any
    // entry (Lookup, the list) or opening another campaign ends it, so Undo can never put A back
    // into another catalogue or over another selection. A comes back between the cases: its
    // file is written again, a draft is typed, and the file is deleted under it.
    let gone_draft = |d: &mut Driver<'_>| {
        fs::write(&rules, &a_text).unwrap();
        d.focus_window(false);
        d.focus_window(true);
        go_library(d);
        search_library(d, "Ash Exposure");
        d.click_starting("Ash Exposure · ");
        d.click(EDIT_MARKDOWN);
        d.type_into_editor("A draft to let go.\n");
        fs::remove_file(&rules).unwrap();
        d.focus_window(false);
        d.focus_window(true);
        d.wait_for_text("Remove entry and lose draft");
    };
    let removed = |d: &mut Driver<'_>| {
        d.click("Remove entry and lose draft");
        d.wait_for_text("No entry open");
        d.has("Undo remove")
    };
    // Lookup: Open in Library selects another entry.
    let lookup_offered = removed(d);
    look_up(d, "Ash Wraith");
    d.settle();
    d.click("Open in Library");
    d.settle();
    if d.has(SEARCH_LOOKUP) {
        d.press_ctrl(Key::K);
    }
    go_library(d);
    let lookup_ended = !d.has("Undo remove") && !d.has_containing("Ash Exposure · ");
    // The list: a row selects another entry.
    gone_draft(d);
    let list_offered = removed(d);
    search_library(d, "Ash Wraith");
    d.click_starting("Ash Wraith · ");
    d.settle();
    let list_ended = !d.has("Undo remove") && !d.has_containing("Ash Exposure · ");
    // Another campaign: Undo would put A into its catalogue with this campaign's choice. The
    // encounter draft from earlier in this scenario would block the switch: let it go first.
    go_encounters(d);
    if d.has("Discard") {
        d.click("Discard");
    }
    gone_draft(d);
    let switch_offered = removed(d);
    switch_campaign(d, QUIET_CAMPAIGN);
    go_library(d);
    d.settle();
    let in_other = d.has("Undo remove");
    switch_campaign(d, CAMPAIGN);
    go_library(d);
    d.settle();
    let switch_ended = !in_other && !d.has("Undo remove") && !d.has_containing("Ash Exposure · ");
    // Astra re-check 2: the same round trip while the Library is hidden. The switch itself ends
    // the Undo; coming back to the first campaign does not bring it back.
    gone_draft(d);
    let hidden_offered = removed(d);
    go_encounters(d);
    switch_campaign(d, QUIET_CAMPAIGN);
    d.settle();
    switch_campaign(d, CAMPAIGN);
    d.settle();
    go_library(d);
    d.settle();
    let hidden_ended = !d.has("Undo remove") && !d.has_containing("Ash Exposure · ");
    d.observe(
        "iter-25-L-undo-remove-owner",
        lookup_offered
            && lookup_ended
            && list_offered
            && list_ended
            && switch_offered
            && switch_ended
            && hidden_offered
            && hidden_ended,
        format!(
            "Undo remove offered then ended by Lookup ({lookup_offered}, {lookup_ended}), the list ({list_offered}, {list_ended}), a campaign switch ({switch_offered}, shown in the other campaign {in_other}, {switch_ended}) and a switch there and back with the Library hidden ({hidden_offered}, {hidden_ended})"
        ),
    );
}

/// The copied wolf's heading warning in `package_duplicate_id`.
const COPY_HEADING_WARNING: &str =
    "The first heading says Cinder Wolf, but the name is Cinder Wolf copy.";

/// The Library reader's title: the topmost painted text that is exactly `title`, below the
/// `Read` tab and right of its edge, so the list row and the body heading do not count.
pub(super) fn reader_title(d: &Driver<'_>, title: &str) -> Option<egui::Rect> {
    let read = d.place_of("Read")?;
    let mut found: Option<egui::Rect> = None;
    d.each_painted_text(|text, clip| {
        let rect = egui::Rect::from_min_size(text.pos, text.galley.size());
        if text.galley.job.text == title
            && rect.left() >= read.left() - 1.0
            && rect.top() > read.bottom()
            && clip.intersects(rect)
            && found.is_none_or(|top| rect.top() < top.top())
        {
            found = Some(rect);
        }
    });
    found
}

/// The reader title's top at each captured size.
pub(super) fn reader_title_tops(d: &mut Driver<'_>, title: &str) -> Vec<Option<f32>> {
    [SMALL, DEFAULT, (1920.0, 1080.0)]
        .into_iter()
        .map(|size| d.at_size(size, |d| reader_title(d, title).map(|rect| rect.top())))
        .collect()
}

/// The Markdown editor's message footer at 960, 1200 and 1920 (round 26 lead ruling: one footer
/// per pane, pinned below the editor). With `words`, at each size: one footer holds them all, it
/// starts at least `S` below the editor frame and ends in the window, it is at most a third of
/// the window high, and each of `words` is painted inside it, in this order from the top. With
/// no `words`: no footer, and the editor runs to the pane's bottom.
fn footer_in_order(d: &mut Driver<'_>, words: &[&str]) -> [bool; 3] {
    [SMALL, DEFAULT, LARGE].map(|size| {
        d.at_size(size, |d| {
            let Some((editor, _)) = d.input_frame("Markdown editor") else {
                return false;
            };
            let below: Vec<_> = d
                .nodes()
                .into_iter()
                .filter(|node| {
                    node.role == Role::GenericContainer
                        && !node.name.is_empty()
                        && node.rect.is_positive()
                        && node.rect.top() > editor.bottom()
                })
                .collect();
            if words.is_empty() {
                return below.is_empty() && editor.bottom() > size.1 - crate::design::XXL * 2.0;
            }
            let Some(footer) = below
                .iter()
                .find(|node| words.iter().all(|word| node.name.contains(word)))
            else {
                return false;
            };
            let tops: Vec<_> = words
                .iter()
                .map(|word| d.painted_rect_in(word, footer.rect).map(|rect| rect.top()))
                .collect();
            footer.rect.top() >= editor.bottom() + crate::design::S - 1.0
                && footer.rect.bottom() <= size.1
                && footer.rect.height() <= size.1 / 3.0
                && tops.iter().all(Option::is_some)
                && tops.windows(2).all(|pair| pair[0] < pair[1])
        })
    })
}

/// The Markdown editor frame's top at 960, 1200 and 1920. Round 25: the one message space below
/// the save row keeps it still whatever the space holds.
fn editor_frame_tops(d: &mut Driver<'_>) -> [Option<f32>; 3] {
    // From Save's row, as `editor_below_read` measures: the page can scroll to the caret.
    [SMALL, DEFAULT, LARGE].map(|size| {
        d.at_size(size, |d| {
            Some(d.input_frame("Markdown editor")?.0.top() - d.place_of("Save")?.bottom())
        })
    })
}

/// A copied file keeps its own save target until the DM repairs its identity.
pub fn package_duplicate_id(d: &mut Driver<'_>) {
    let package = d.root.join("modules/ashen-marches");
    let original = package.join("Creatures/Cinder Wolf.md");
    let copy = package.join("Creatures/Cinder Wolf copy.md");
    let before = fs::read_to_string(&original).unwrap();
    let original_id = crate::content_workspace::note_id(&before);
    let copy_text = before
        .replace("splat_name: Cinder Wolf", "splat_name: Cinder Wolf copy")
        .replace("max_hp: {value: 22}", "max_hp: {value: 44}")
        .replace("HP 22 (", "HP 44 (")
        .replace("| Hit Points | 22 (", "| Hit Points | 44 (");
    fs::write(&copy, &copy_text).unwrap();
    go_library(d);
    search_library(d, "Cinder Wolf");
    d.wait_for_text("Same entry in two files");
    d.expect("Bridge Ambush keeps Cinder Wolf.md.");
    d.expect("Bridge Ambush will then use Cinder Wolf copy.md.");
    d.observe_both(
        "iter-3-D-FD4",
        "Each repair consequence stays under its own choice",
        |d| {
            let first = d
                .place_of("Make Cinder Wolf copy.md a separate entry")
                .unwrap();
            let keeps = d.place_of("Bridge Ambush keeps Cinder Wolf.md.").unwrap();
            let second = d.place_of("Make Cinder Wolf.md a separate entry").unwrap();
            let changes = d
                .place_of("Bridge Ambush will then use Cinder Wolf copy.md.")
                .unwrap();
            (keeps.top() - first.bottom() - crate::design::XS).abs() < 1.0
                && second.top() - keeps.bottom() >= crate::design::L
                && (changes.top() - second.bottom() - crate::design::XS).abs() < 1.0
        },
    );
    // Round 24 AC1: both files of the duplicate identity are marked, and the header counts both.
    let marked = row_marks(d, crate::library::SAME_ENTRY);
    d.observe(
        "iter-24-L-AC1",
        marked.len() == 2 && d.visible_text("2 files need fixing"),
        format!("Two rows mark the duplicate, and the header counts two files: {marked:?}"),
    );
    d.state("duplicate-package-library-list", Sizes::All);
    d.click_starting("Cinder Wolf · ");
    d.expect("Make Cinder Wolf copy.md a separate entry");
    // The original has the duplicate warning and no heading warning: its title height is the
    // bar for the copy, whose file has both (iter-16-E-FD7-title).
    assert!(!d.has_containing("The first heading says"));
    let plain_tops = reader_title_tops(d, "Cinder Wolf");
    assert!(
        plain_tops.iter().all(Option::is_some),
        "The original's reader title is found at each size: {plain_tops:?}"
    );
    d.click(EDIT_MARKDOWN);
    // Round 26 FC4: Edit Markdown puts the duplicate warning and its choices in the pane's
    // message footer, below the editor. Must stay true with iter-14-E-FD4: both choices are
    // there, in view.
    d.observe_both(
        "iter-14-E-FD4-source",
        "Edit Markdown keeps both repair choices in the footer below the Markdown editor",
        |d| {
            let Some(editor) = d.input_frame("Markdown editor").map(|(frame, _)| frame) else {
                return false;
            };
            [
                "Make Cinder Wolf copy.md a separate entry",
                "Make Cinder Wolf.md a separate entry",
            ]
            .iter()
            .all(|name| {
                d.place_of(name)
                    .is_some_and(|choice| d.in_view(name) && choice.top() > editor.bottom())
            })
        },
    );
    d.state("duplicate-original-editor", Sizes::All);

    d.type_into_editor("Saved only to the original.\n");
    d.click("Save");
    d.wait_for_text("Saved");
    assert!(
        fs::read_to_string(&original)
            .unwrap()
            .contains("Saved only to the original.")
    );
    assert_eq!(fs::read_to_string(&copy).unwrap(), copy_text);
    go_encounters(d);
    open_saved_encounter(d, "Bridge Ambush");
    d.expect("Changes apply to the next fight. To add to this fight, use Add to fight in the run.");
    // Round 24 FA3: each roster row says the short cause, as Library rows do; the sentence with
    // the file names is on the row's hover, not repeated in every row.
    let long =
        "These two files claim to be the same entry: Cinder Wolf copy.md and Cinder Wolf.md.";
    let short = format!("{} {}", crate::icons::WARN, crate::library::SAME_ENTRY);
    let mut painted = Vec::new();
    d.each_painted_text(|text, clip| {
        if text.galley.job.text.contains(crate::library::SAME_ENTRY)
            && clip.intersects(text.visual_bounding_rect())
        {
            painted.push(text.galley.job.text.clone());
        }
    });
    d.observe(
        "iter-24-L-FA3",
        painted.len() == 2
            && painted.iter().all(|row| row.ends_with(&short))
            && !d.has_containing(long),
        format!("Both Cinder Wolf rows say the short cause, and no row repeats the file names: {painted:?}"),
    );
    d.state("duplicate-package-in-roster", Sizes::All);
    // Round 23 AD2: a duplicate identity is a warning in the roster too, as in Library; only a
    // file that cannot be read is an error. Round 24 FA3: the rows say the short cause.
    let rows = row_marks(d, crate::library::SAME_ENTRY);
    d.observe(
        "iter-23-L-AD2",
        rows.len() == 2 && warning_marked(&rows),
        format!("Both Cinder Wolf rows mark the duplicate identity as a warning: {rows:?}"),
    );
    d.click("Open in Library");
    // The copy's footer holds its heading warning first, so at 960 the duplicate's choices are
    // reached by scrolling the footer.
    let heading_fix = d
        .place_of("Use Cinder Wolf copy")
        .expect("The heading fix is in the footer");
    d.scroll_until(
        heading_fix.center(),
        "Make Cinder Wolf copy.md a separate entry",
        12,
    );
    d.expect("Make Cinder Wolf copy.md a separate entry");
    assert_eq!(
        d.app_mut().library.selected_name(),
        Some("Cinder Wolf copy")
    );
    d.click("Read");
    // Round 25 AC2: the copy's row names both causes its reader reports, the duplicate and the
    // first heading; the original's row names only the duplicate.
    // At 960 the row is cut inside the second cause, so its first words are the proof.
    let causes = format!(
        "{} {} · First heading",
        crate::icons::WARN,
        crate::library::SAME_ENTRY
    );
    let mut lines = Vec::new();
    d.each_painted_text(|text, clip| {
        let words = &text.galley.job.text;
        if words.contains(crate::library::SAME_ENTRY)
            && words.contains(" · ")
            && clip.intersects(text.visual_bounding_rect())
        {
            lines.push(words.clone());
        }
    });
    d.observe(
        "iter-25-L-AC2",
        lines.len() == 2
            && lines
                .iter()
                .filter(|line| line.starts_with(&causes))
                .count()
                == 1
            && lines
                .iter()
                .filter(|line| !line.contains("First heading"))
                .count()
                == 1,
        format!("One row names both causes, the other only the duplicate: {lines:?}"),
    );
    d.state_prepared("duplicate-copy-reader", Sizes::All, |d| {
        let text = d
            .painted_rect_in(crate::world::DUPLICATE_ENTRY, egui::Rect::EVERYTHING)
            .expect("The duplicate warning is visible");
        let choices: Vec<_> = [
            "Make Cinder Wolf copy.md a separate entry",
            "Make Cinder Wolf.md a separate entry",
        ]
        .into_iter()
        .map(|name| d.place_of(name).expect("Both repair choices are visible"))
        .collect();
        d.observe(
            "iter-13-E-FD4",
            choices
                .iter()
                .all(|choice| (choice.left() - text.left()).abs() < 1.0),
            format!(
                "The repair choices start at the message text edge: text {:.0}, choices {:.0} and {:.0}",
                text.left(),
                choices[0].left(),
                choices[1].left()
            ),
        );
        // Must stay true: the second choice still sits a large gap below the first.
        d.observe(
            "iter-13-E-FD4-gap",
            choices[1].top() - choices[0].bottom() >= design::L,
            "The two repair choices keep their gap",
        );
        // One place for a file's warnings: the heading warning and its action follow the
        // title, directly above the duplicate warning.
        let title =
            reader_title(d, "Cinder Wolf copy").expect("The copy's reader title is visible");
        let heading = d
            .painted_rect_in(COPY_HEADING_WARNING, egui::Rect::EVERYTHING)
            .expect("The heading warning is visible");
        let fix = d
            .place_of("Use Cinder Wolf copy")
            .expect("The heading fix is visible");
        d.observe(
            "iter-16-E-FD7-order",
            title.bottom() < heading.top()
                && heading.bottom() <= fix.bottom()
                && fix.bottom() < text.top()
                && text.top() - fix.bottom() <= design::L,
            format!(
                "The heading warning follows the title and sits directly above the duplicate warning: title bottom {:.0}, heading {:.0} to {:.0}, its action bottom {:.0}, duplicate warning top {:.0}",
                title.bottom(),
                heading.top(),
                heading.bottom(),
                fix.bottom(),
                text.top()
            ),
        );
        d.observe(
            "iter-16-E-FD7-fit",
            d.nodes()
                .iter()
                .any(|node| node.name == "Use Cinder Wolf copy" && node.fully_in_view()),
            "The heading fix is fully in view in the reader",
        );
    });
    // The copy's title sits where the original's does, though only the copy has the heading
    // warning: the warning no longer pushes the title down.
    let copy_tops = reader_title_tops(d, "Cinder Wolf copy");
    d.observe(
        "iter-16-E-FD7-title",
        copy_tops.iter().zip(&plain_tops).all(|(copy, plain)| {
            copy.zip(*plain)
                .is_some_and(|(copy, plain)| (copy - plain).abs() < 1.0)
        }),
        format!(
            "The title top with the heading warning ({copy_tops:?}) matches the title top without it ({plain_tops:?}) at 960, 1200 and 1920"
        ),
    );
    // Must stay true: Edit Markdown draws the heading warning in the same footer, below the
    // editor (round 26).
    d.click(EDIT_MARKDOWN);
    d.observe_both(
        "iter-16-E-FD7-source",
        "Edit Markdown keeps the heading warning in the footer below the Markdown editor",
        |d| {
            let Some(editor) = d.input_frame("Markdown editor").map(|(frame, _)| frame) else {
                return false;
            };
            d.painted_rect_in(COPY_HEADING_WARNING, egui::Rect::EVERYTHING)
                .is_some_and(|heading| heading.top() > editor.bottom())
                && d.place_of("Use Cinder Wolf copy").is_some_and(|fix| {
                    d.in_view("Use Cinder Wolf copy") && fix.top() > editor.bottom()
                })
        },
    );
    // Round 26 FC4: Save and the editor's top, from the Read tab, at 960, 1200 and 1920. The
    // copy has the duplicate warning now, and none after `Make … a separate entry` below.
    let save_and_editor = |d: &mut Driver<'_>| {
        [SMALL, DEFAULT, LARGE].map(|size| {
            d.at_size(size, |d| {
                let read = d.place_of("Read")?;
                Some((
                    d.place_of("Save")?.top() - read.top(),
                    d.input_frame("Markdown editor")?.0.top() - read.top(),
                ))
            })
        })
    };
    let with_duplicate = save_and_editor(d);
    d.click("Read");
    d.expect("Creature · CR 1");
    let title_before = title_below_file_line(d, "Cinder Wolf copy");
    d.click("Make Cinder Wolf copy.md a separate entry");
    d.wait_for_text("Undo separate entry");
    d.expect("Cinder Wolf copy.md is now a separate entry.");
    assert_eq!(
        crate::content_workspace::note_id(&fs::read_to_string(&original).unwrap()),
        original_id
    );
    let copy_id = crate::content_workspace::note_id(&fs::read_to_string(&copy).unwrap());
    assert_ne!(original_id, copy_id);
    d.state("duplicate-copy-new-identity", Sizes::All);
    // Round 23 FD4, FE6: the result takes the warnings' place after the title, so the title
    // stays where it was with the warning.
    // The title is measured from the file line (FD5).
    let title_after = title_below_file_line(d, "Cinder Wolf copy");
    let below = d.at_size(DEFAULT, |d| {
        let title = reader_title(d, "Cinder Wolf copy")?;
        let result = d.painted_rect_in(
            "Cinder Wolf copy.md is now a separate entry.",
            egui::Rect::EVERYTHING,
        )?;
        Some(result.top() > title.bottom())
    });
    d.observe(
        "iter-23-L-FD4-identity",
        title_before.iter().all(Option::is_some)
            && title_before == title_after
            && below == Some(true),
        format!(
            "The title keeps its place when the result shows ({title_before:?} → {title_after:?}), and the result is below it ({below:?})"
        ),
    );
    // Round 23 FD5: the file line keeps the whole file name, which tells the copy from its
    // twin, at every size. It shortens only from the front.
    for size in [SMALL, DEFAULT, LARGE] {
        let shown = d.at_size(size, |d| {
            let mut shown = Vec::new();
            d.each_painted_text(|text, clip| {
                let words = &text.galley.job.text;
                if clip.intersects(text.visual_bounding_rect())
                    && (words.starts_with("Cinder Wolf") || words.contains("/Cinder Wolf"))
                    && (words.ends_with(".md") || words.ends_with('…'))
                {
                    shown.push(words.clone());
                }
            });
            shown
        });
        d.observe(
            "iter-23-L-FD5",
            shown.iter().any(|words| {
                words == "Creatures/Cinder Wolf copy.md" || words == "…/Cinder Wolf copy.md"
            }) && !shown.iter().any(|words| words.ends_with('…')),
            format!("At {size:?} the file line shows the whole file name: {shown:?}"),
        );
    }
    // Round 23 FD7: the header counts the copy's first-heading fault, and its row says so.
    let rows = row_marks(d, "First heading differs");
    d.observe(
        "iter-23-L-FD7",
        d.visible_text("1 file needs fixing") && rows.len() == 1 && warning_marked(&rows),
        format!("The copy's row marks the fault the header counts: {rows:?}"),
    );
    // Round 26 FC4: the copy with no duplicate has Save and the editor where it had them with
    // the duplicate warning: the warning gives Save no second place.
    d.click(EDIT_MARKDOWN);
    assert!(
        !d.has_containing(crate::world::DUPLICATE_ENTRY),
        "the copy is a separate entry now"
    );
    let without_duplicate = save_and_editor(d);
    d.observe(
        "iter-26-L-FC4",
        with_duplicate.iter().all(Option::is_some) && with_duplicate == without_duplicate,
        format!(
            "Save and the editor keep their places with the duplicate warning {with_duplicate:?} and without it {without_duplicate:?}"
        ),
    );
    d.click("Read");
    d.click("Undo separate entry");
    d.wait_for_text("Same entry in two files");
    assert_eq!(
        crate::content_workspace::note_id(&fs::read_to_string(&copy).unwrap()),
        original_id
    );
    d.expect_same("Undo restores both identity warnings and the original copy reader.");
    d.assert_state_matches("duplicate-identity-restored", "duplicate-copy-reader");
    d.click("Make Cinder Wolf copy.md a separate entry");
    d.wait_for_text("Undo separate entry");
    d.expect("Cinder Wolf copy.md is now a separate entry.");
    // Round 25 FF3: `Choose for this campaign` is a toolbar action, so the title of an entry that
    // is not chosen sits where a chosen entry's does, at every size.
    let not_chosen = reader_title_tops(d, "Cinder Wolf copy");
    // At 960 the toolbar folds Copy to… and the choice into More.
    if !d.has("Choose for this campaign") {
        d.click("More");
    }
    d.click("Choose for this campaign");
    d.wait_for_text("Ashen Marches · Chosen for this campaign");
    let chosen = reader_title_tops(d, "Cinder Wolf copy");
    d.observe(
        "iter-25-L-FF3",
        !d.has("Choose for this campaign")
            && not_chosen.iter().all(Option::is_some)
            && not_chosen == chosen,
        format!("The title keeps its top when the entry is chosen: {not_chosen:?} → {chosen:?}"),
    );
    go_encounters(d);
    d.expect("Bridge Ambush");
    d.observe_both(
        "W10",
        "The repaired roster keeps the original wolf stats",
        |d| d.nodes().iter().any(|node| node.name == "HP\u{a0}22"),
    );
    d.state("repaired-encounter-keeps-original-wolves", Sizes::All);
    d.type_into(FIND_CREATURE, "Cinder Wolf");
    d.expect("Cinder Wolf copy");
    d.expect("HP 44");
    d.state("repaired-creatures-are-separate-results", Sizes::All);
}

/// Copy a book creature into a shared package, then change its HP.
pub fn book_to_package(d: &mut Driver<'_>) {
    go_library(d);
    search_library(d, "Aboleth");
    d.click_starting("Aboleth · ");
    d.click(COPY_TO);
    d.click_starting("Destination");
    d.click(&format!("Package · {MODULE}"));
    d.replace_text_in("Copy name", "Watch Aboleth");
    d.type_into("Folder", "Creatures");
    d.wait_for_text("Creatures/Watch Aboleth.md");
    // Round 25 FE1: with the caret in Folder, the focus ring goes round the whole field, its
    // suggestions button included, as every text field's does.
    for size in [SMALL, DEFAULT] {
        let (focused, field, rings) = d.at_size(size, |d| {
            let field = d
                .nodes()
                .into_iter()
                .find(|node| node.name == "Folder" && node.role == egui::accesskit::Role::TextInput)
                .map(|node| node.rect);
            (d.focused_name(), field, d.focus_rings())
        });
        d.observe(
            "iter-25-L-FE1",
            focused.as_deref() == Some("Folder")
                && field.is_some_and(|field| {
                    rings.iter().any(|(ring, _)| {
                        ring.expand(0.5).contains_rect(field)
                            && ring.width() >= field.width() + design::XL
                    })
                }),
            format!("At {size:?} the focused Folder field ({focused:?}, {field:?}) has a ring round the whole field: {rings:?}"),
        );
    }
    d.state("book-copy-to-package", Sizes::All);
    d.click(CREATE_COPY);
    d.expect("Copied to Ashen Marches as Watch Aboleth.");
    d.click("Edit the copy");
    d.expect("Markdown editor");
    let original = d.editor_text().unwrap();
    assert!(original.contains("> [!quote]- Printed stat block (not read; edit the table above)"));
    let archived = original.replace("HP 150 (20d10 + 40)", "HP 180 (20d10 + 70)");
    assert_ne!(original, archived);
    d.replace_text_in("Markdown editor", &archived);
    d.observe_both("X2", "Archived printed HP is not used or checked", |d| {
        !d.has_containing("Hit Points line says") && !d.has("Use 180")
    });
    reveal_editor_passage(d, "> [!quote]- Printed stat block");
    d.state("book-creature-printed-record", Sizes::All);
    d.click("Read");
    d.scroll_up_at(READER, 30);
    d.expect("150 (20d10 + 40)");
    d.observe_both(
        "X2",
        "The reader uses the table and hides the printed record",
        |d| !d.has_containing("Printed stat block") && !d.has("180 (20d10 + 70)"),
    );
    d.state("book-creature-printed-record-ignored", Sizes::All);
    d.click("Edit Markdown");
    let edited = archived.replace(
        "| Hit Points | 150 (20d10 + 40) |",
        "| Hit Points | 180 (20d10 + 40) |",
    );
    assert_ne!(archived, edited);
    d.replace_text_in("Markdown editor", &edited);
    assert!(!edited.contains("    value:"));
    d.observe_both(
        "W1/C1",
        "A book copy has no false broken-link warning",
        |d| !d.has_containing("broken link"),
    );
    d.click("Save");
    d.click("Read");
    d.expect("Watch Aboleth");
    d.scroll_up_at(READER, 30);
    d.expect("180 (20d10 + 40)");
    assert!(
        fs::read_to_string(
            d.root
                .join("modules/ashen-marches/Creatures/Watch Aboleth.md")
        )
        .unwrap()
        .contains("# Watch Aboleth")
    );
    d.state("book-creature-changed-in-package", Sizes::All);
}

/// Rook Ambry's World list rows, by the name the row starts with.
fn rook_rows(d: &Driver<'_>) -> Vec<crate::visual_tests::driver::Found> {
    d.nodes()
        .into_iter()
        .filter(|node| node.name.starts_with("Rook Ambry · "))
        .collect()
}

pub fn world_note_moved(d: &mut Driver<'_>) {
    let folder = root(d);
    fs::create_dir_all(folder.join("Locations")).unwrap();
    fs::create_dir_all(folder.join("Other")).unwrap();
    fs::write(
        folder.join("Locations/Watch House.md"),
        "# Watch House\n\nThe original watch house.\n",
    )
    .unwrap();
    fs::copy(
        folder.join("Locations/Watch House.md"),
        folder.join("Other/Watch House.md"),
    )
    .unwrap();
    let npc = entry(
        d,
        "NPCs/Rook Ambry.md",
        "Rook Ambry",
        "npc",
        "  in: '[[Watch House]]'\n",
    );
    entry(d, "Locations/North Gate.md", "North Gate", "location", "");
    reload(d);
    d.click(NAV_WORLD);
    d.click_containing("Rook Ambry · ");
    d.click_containing("Watch House · 2 matches");
    d.expect("This link matches more than one note. Choose one.");
    d.observe_both(
        "iter-3-D-FD9-10",
        "Each place choice pairs its file and bordered action",
        |d| {
            ["Locations/Watch House.md", "Other/Watch House.md"]
                .iter()
                .all(|path| {
                    let link = d.place_of(path).unwrap();
                    let action = d.place_of(&format!("Always use {path}")).unwrap();
                    action.left() >= link.right() + crate::design::L - 1.0
                        && action.top() < link.bottom()
                        && link.top() < action.bottom()
                })
        },
    );
    d.observe_both(
        "iter-4-D-FD11",
        "Place choices share one action column in World",
        |d| {
            let first = d.place_of("Always use Locations/Watch House.md").unwrap();
            let second = d.place_of("Always use Other/Watch House.md").unwrap();
            (first.left() - second.left()).abs() < 1.0
        },
    );
    d.observe_both(
        "iter-11-D-FD7",
        "Change and each place choice's action share one column",
        |d| {
            for name in [
                "Change In",
                "Always use Locations/Watch House.md",
                "Always use Other/Watch House.md",
            ] {
                d.expect(name);
            }
            let change = d.place_of("Change In").unwrap();
            ["Locations/Watch House.md", "Other/Watch House.md"]
                .iter()
                .all(|path| {
                    let link = d.place_of(path).unwrap();
                    let action = d.place_of(&format!("Always use {path}")).unwrap();
                    // Neighbour: each file keeps at least an L gap before its action.
                    (action.left() - change.left()).abs() < 1.0
                        && action.left() >= link.right() + crate::design::L - 1.0
                })
        },
    );
    d.observe_both(
        "iter-12-D-FD1",
        "A place with two matches warns on the list row and counts its note in the header",
        |d| {
            let rows = rook_rows(d);
            // Neighbour: the place waits for a choice. It is not missing.
            !rows.is_empty()
                && rows
                    .iter()
                    .all(|row| row.name.contains("Place has 2 matches") && row.in_view())
                && d.in_view_containing("1 note needs fixing")
                && !d.has_containing("Place not found")
        },
    );
    // FD4: the warning icon is its own glyph before the link words. Only the words are
    // underlined, and the accessible name keeps the warning.
    for size in [SMALL, DEFAULT, LARGE] {
        let parts = d.at_size(size, |d| {
            d.expect(&format!("{} Watch House · 2 matches", design::icons::WARN));
            icon_link_parts(d, design::icons::WARN, "Watch House · 2 matches")
        });
        d.observe(
            "iter-17-D-FD4",
            parts == Some((false, true, design::colors().result.warning)),
            format!("At {size:?}, (icon underlined, words underlined, icon colour) is {parts:?}"),
        );
    }
    // Round 24 FG4: each file link shares its action's line: the link's words and the words of
    // `Always use this one` centre within 1 px.
    for size in [SMALL, DEFAULT, LARGE] {
        let (level, detail) = d.at_size(size, |d| {
            let pairs: Vec<_> = ["Locations/Watch House.md", "Other/Watch House.md"]
                .into_iter()
                .map(|path| {
                    let action = d.place_of(&format!("Always use {path}"));
                    let words = action.and_then(|action| {
                        d.painted_rect_in("Always use this one", action.expand(1.0))
                    });
                    let link = action.and_then(|action| {
                        d.painted_rect_in(
                            path,
                            egui::Rect::from_x_y_ranges(
                                f32::NEG_INFINITY..=action.left(),
                                action.top() - design::S..=action.bottom() + design::S,
                            ),
                        )
                    });
                    (link, words)
                })
                .collect();
            (
                pairs.iter().all(|(link, words)| {
                    link.zip(*words).is_some_and(|(link, words)| {
                        (link.center().y - words.center().y).abs() <= 1.0
                    })
                }),
                format!("{pairs:?}"),
            )
        });
        d.observe(
            "iter-24-D-FG4",
            level,
            format!("Each chooser link centres on its action's words at {size:?}: {detail}"),
        );
    }
    d.state("duplicate-place-chooser-in-world", Sizes::All);
    d.click("Always use Locations/Watch House.md");
    d.expect("Undo place change");
    d.observe_both(
        "iter-12-D-FD1-cleared",
        "After Always use this one, the list row and the header count no longer warn",
        |d| {
            let rows = rook_rows(d);
            // Neighbour: the choice keeps its Undo.
            !rows.is_empty()
                && rows
                    .iter()
                    .all(|row| !row.name.contains("Place has") && row.in_view())
                && !d.has_containing("note needs fixing")
                && !d.has_containing("notes need fixing")
                && d.in_view("Undo place change")
        },
    );
    // Round 24 AD2, FD3, SD2: two notes have the name, so the result names the chosen one's
    // folder as the list shows it. The other folder is not named.
    d.observe(
        "iter-24-D-AD2",
        d.has_containing("Place set to Watch House (Locations). It was Watch House.")
            && !d.has_containing("Watch House (Other)"),
        "The place result names the folder of the file the DM chose",
    );
    d.state("place-choice-kept-with-undo", Sizes::All);
    d.click("Undo place change");
    // From another note, the header count's menu opens the note whose place waits.
    d.click_containing("North Gate · ");
    d.expect("Edit North Gate");
    assert!(!d.has("Edit Rook Ambry"), "fixture: another note is open");
    d.click("1 note needs fixing");
    d.click("Rook Ambry");
    d.expect("Edit Rook Ambry");
    d.observe(
        "iter-12-D-FD1-menu",
        d.in_view("Edit Rook Ambry") && !d.has("Edit North Gate"),
        "The header count's repair menu opens the note whose place has two matches",
    );
    // iter-13-D-SD2: a place that waits for a choice is not a broken link, even beside one.
    let npc_path = folder.join("NPCs/Rook Ambry.md");
    let original = fs::read_to_string(&npc_path).unwrap();
    for (fields, words, other) in [
        (
            "splat_in: '[[Watch House]]'\nsplat_part_of: ['[[Lost Guild]]']\n",
            "2 links need fixing",
            "broken link",
        ),
        (
            "splat_in: '[[Nowhere]]'\nsplat_part_of: ['[[Lost Guild]]']\n",
            "2 broken links",
            "need fixing",
        ),
    ] {
        fs::write(
            &npc_path,
            format!("---\nsplat_schema: 2\nsplat_id: {npc}\nsplat_kind: world\nsplat_name: Rook Ambry\nsplat_type: npc\n{fields}---\n\n# Rook Ambry\n\nA note for tonight.\n"),
        )
        .unwrap();
        reload(d);
        d.observe_both(
            "iter-13-D-SD2",
            &format!("A row with two link problems says {words}"),
            |d| {
                let rows = rook_rows(d);
                // Neighbour: the header still counts the note once.
                !rows.is_empty()
                    && rows.iter().all(|row| {
                        row.name.contains(words) && !row.name.contains(other) && row.in_view()
                    })
                    && d.in_view_containing("1 note needs fixing")
            },
        );
        if words == "2 links need fixing" {
            d.state("mixed-link-problems-need-fixing", Sizes::All);
        }
    }
    fs::write(&npc_path, &original).unwrap();
    reload(d);
    d.expect("Edit Rook Ambry");
    d.click_containing("Watch House · 2 matches");
    d.click("Locations/Watch House.md");
    look_up(d, "Rook Ambry");
    d.click_containing("Watch House · 2 matches");
    d.expect("This link matches more than one note. Choose one.");
    d.state("duplicate-place-chooser-in-lookup", Sizes::All);
    d.click("Locations/Watch House.md");
    d.press(Key::Escape);
    fs::remove_file(folder.join("Other/Watch House.md")).unwrap();
    fs::create_dir_all(folder.join("Locations/Moved")).unwrap();
    fs::rename(
        folder.join("Locations/Watch House.md"),
        folder.join("Locations/Moved/Watch House.md"),
    )
    .unwrap();
    fs::rename(
        folder.join("NPCs/Rook Ambry.md"),
        folder.join("Locations/Moved/Rook Ambry.md"),
    )
    .unwrap();
    d.focus_window(false);
    d.focus_window(true);
    d.click_containing("Rook Ambry · ");
    d.expect("Watch House");
    d.state("moved-note-keeps-place-in-world", Sizes::All);
    look_up(d, "Rook Ambry");
    d.expect("Watch House");
    d.state("moved-note-keeps-place-in-lookup", Sizes::All);
    d.press(Key::Escape);
    let moved_npc = folder.join("Locations/Moved/Rook Ambry.md");
    let before_change = fs::read_to_string(&moved_npc).unwrap();
    let external = format!(
        "{before_change}\n## Patrol after the move\n\nRook keeps the east gate key in the watch room. The relief patrol leaves at midnight and checks the bridge before it returns to the north gate.\n\nKeep this note from Obsidian when the place changes.\n"
    );
    fs::write(&moved_npc, &external).unwrap();
    d.click("Change In");
    d.type_text("North Gate");
    d.click("North Gate");
    assert!(
        fs::read_to_string(&moved_npc)
            .unwrap()
            .contains("Keep this note from Obsidian")
    );
    assert_eq!(
        crate::content_workspace::note_id(
            &fs::read_to_string(folder.join("Locations/Moved/Rook Ambry.md")).unwrap()
        ),
        Some(npc)
    );
    assert!(!folder.join("NPCs/Rook Ambry.md").exists());
    d.state("change-writes-moved-npc", Sizes::All);
}

pub fn world_note_deleted(d: &mut Driver<'_>) {
    entry(d, "Locations/Watch House.md", "Watch House", "location", "");
    let block_text = crate::library::note_text("Watchkeeper Guard", "creature", None);
    let block = splat::content::Entry::parse("Watchkeeper Guard.md", &block_text).unwrap();
    fs::write(root(d).join("Watchkeeper Guard.md"), block_text).unwrap();
    entry(
        d,
        "NPCs/Watchkeeper.md",
        "Watchkeeper",
        "npc",
        &format!(
            "  in: '[[Watch House]]'\n  creature: {{id: {}, link: '[[Watchkeeper Guard]]'}}\n",
            block.id
        ),
    );
    reload(d);
    d.click(NAV_WORLD);
    d.click_containing("Watchkeeper · ");
    d.wait_for_text("Watch House");
    for size in [SMALL, DEFAULT, LARGE] {
        let named = d.at_size(size, |d| {
            d.expect("Edit Watchkeeper");
            let rect = d.place_of("Edit Watchkeeper").unwrap();
            let words = d.painted_in(rect);
            words.contains("Edit") && words.contains(design::icons::EDIT)
        });
        d.observe(
            "iter-6-D-FD7-world",
            named,
            format!("World's Edit action has its pencil and word at {size:?}"),
        );
    }
    d.state("npc-before-place-deleted", Sizes::All);
    fs::remove_file(root(d).join("Locations/Watch House.md")).unwrap();
    d.focus_window(false);
    d.focus_window(true);
    d.wait_for_text("Watch House · Not found");
    // Round 24 AG1: `⚠ Place not found` on the row has the warning tone, as the reader's does.
    let (marked, detail) = d.at_size(LARGE, |d| world_fault_marks(d, false));
    d.observe("iter-24-D-AG1", marked, detail);
    d.state("npc-place-deleted-in-world", Sizes::All);
    look_up(d, "Watchkeeper");
    d.wait_for_text("Watch House · Not found");
    // Round 24 FD6: the compact reader's link fault reads as its other values: the muted label
    // `In`, then the value in the warning tone. The label is not in the warning colour.
    let missing = format!("{} Watch House · Not found", design::icons::WARN);
    d.observe(
        "iter-24-D-FD6",
        d.painted_color("In") == Some(design::colors().muted)
            && d.painted_color(&missing) == Some(design::colors().result.warning)
            && !d.has_containing("In Watch House"),
        format!(
            "In is {:?} and its value is {:?}",
            d.painted_color("In"),
            d.painted_color(&missing)
        ),
    );
    // Round 26 AE2: Lookup's result says the fault that World's row says, with its warning
    // mark, before the note's summary. Neighbour: the stat block's result has no mark.
    let mark = format!("{} Place not found", design::icons::WARN);
    let row = d
        .nodes()
        .into_iter()
        .find(|node| node.name.starts_with("Watchkeeper · ") && node.name.contains("NPC"))
        .map(|node| node.name);
    // The neighbour is required: the stat block's own result row, a creature, with no mark.
    let guard = d
        .nodes()
        .into_iter()
        .find(|node| node.name.starts_with("Watchkeeper Guard · "))
        .map(|node| node.name);
    d.observe(
        "iter-26-D-AE2",
        row.as_ref().is_some_and(|row| {
            row.find(&mark)
                .zip(row.find("A note for tonight"))
                .is_some_and(|(mark, summary)| mark < summary)
        }) && guard.as_ref().is_some_and(|guard| {
            guard.contains("Creature") && !guard.contains(design::icons::WARN)
        }),
        format!("Lookup's result row is {row:?}; the stat block's row is {guard:?}"),
    );
    d.state("npc-place-deleted-in-lookup", Sizes::All);
    d.press(Key::Escape);
    fs::remove_file(root(d).join("Watchkeeper Guard.md")).unwrap();
    d.focus_window(false);
    d.focus_window(true);
    d.wait_for_text("Watchkeeper Guard · Not found");
    d.type_into("Search the world", "Watchkeeper");
    d.state("npc-stat-block-deleted-in-world", Sizes::All);
    look_up(d, "Watchkeeper");
    d.wait_for_text("Watchkeeper Guard · Not found");
    // Round 23 FG1, SG1: with Lookup open, World's rows stack each label above its value. The
    // value keeps the label column's room, so each missing link stays on one line. Must stay
    // true: Change still starts an `L` gap after its value.
    // At 960 the pane is too narrow for the whole value: the name keeps its words on one line
    // and `Not found` takes its own line, whole and visible, directly under the name, as the
    // missing-link rule says.
    for size in [SMALL, DEFAULT, LARGE] {
        let (whole, detail, sized, level) = d.at_size(size, |d| {
            // Each painted text with exactly these words, wholly visible: its rows and its rect.
            let painted = |words: &str| {
                let mut found = Vec::new();
                // World's pane only: Lookup, to the right, names the same links.
                let lookup = d
                    .place_of(SEARCH_LOOKUP)
                    .map_or(f32::INFINITY, |field| field.left());
                d.each_painted_text(|text, clip| {
                    let rect = text.visual_bounding_rect();
                    if text.galley.job.text == words
                        && clip.expand(1.0).contains_rect(rect)
                        && rect.right() < lookup
                    {
                        found.push((text.galley.rows.len(), rect));
                    }
                });
                found
            };
            // Round 25 FA3: a missing link's icon and name are in `Body`, as a set link is; only
            // `Not found` may be `Small`. The first section's size is the name's size.
            let mut name_sizes = Vec::new();
            d.each_painted_text(|text, _| {
                let job = &text.galley.job;
                if job.text.starts_with(design::icons::WARN)
                    && ["Watchkeeper Guard", "Watch House"]
                        .iter()
                        .any(|name| job.text.contains(name))
                    && let Some(section) = job.sections.first()
                {
                    name_sizes.push((job.text.clone(), section.format.font_id.size));
                }
            });
            let body = design::Type::Body.size();
            let sized = (
                name_sizes.len() >= 2 && name_sizes.iter().all(|(_, size)| *size == body),
                format!("Missing link names are in Body ({body}) at {size:?}: {name_sizes:?}"),
            );
            let not_found = painted("Not found");
            let mut detail = Vec::new();
            let mut whole = true;
            for name in ["Watchkeeper Guard", "Watch House"] {
                let full = painted(&format!("{} {name} · Not found", design::icons::WARN));
                let alone = painted(&format!("{} {name}", design::icons::WARN));
                // When the value wraps, `Not found` is its own whole line directly under its
                // name, starting at the name's left edge.
                let under = match alone.as_slice() {
                    [(1, name)] => not_found.iter().any(|(rows, state)| {
                        *rows == 1
                            && (state.left() - name.left()).abs() <= 1.0
                            && state.top() >= name.bottom() - 0.5
                            && state.top() - name.bottom() <= design::S
                    }),
                    _ => false,
                };
                // Round 24 SG1: at 960 the label stands above the value, and `Not found` has its
                // own line under the name even where the whole value would fit.
                whole &= if size == SMALL {
                    full.is_empty() && under
                } else {
                    (full.len() == 1 && full[0].0 == 1) || (full.is_empty() && under)
                };
                detail.push(format!(
                    "{name}: one galley {full:?}, name alone {alone:?}, under {under}"
                ));
            }
            detail.push(format!("Not found texts {not_found:?}"));
            let change = d.place_of("Change Stat block");
            // Round 25 AA2: Change lines up with the name's line, not between the name and
            // `Not found`. The name line is the one-galley value, or the name alone above
            // `Not found` (stacked), each one row; Lookup's copy is right of the pane.
            let mut level = Vec::new();
            for (name, label) in [
                ("Watchkeeper Guard", "Change Stat block"),
                ("Watch House", "Change In"),
            ] {
                let line = painted(&format!("{} {name} · Not found", design::icons::WARN))
                    .into_iter()
                    .chain(painted(&format!("{} {name}", design::icons::WARN)))
                    .find(|(rows, _)| *rows == 1)
                    .map(|(_, rect)| rect.center().y);
                let target = d.place_of(label).map(|rect| rect.center().y);
                level.push((name, line, target));
            }
            let level = (
                level.iter().all(|(_, line, target)| {
                    line.zip(*target)
                        .is_some_and(|(line, target)| (line - target).abs() <= 2.0)
                }),
                format!("Change is level with the missing name's line at {size:?}: {level:?}"),
            );
            // World's value only: Lookup, to the right, names the same stat block.
            let value = change.and_then(|change| {
                d.painted_rect_in(
                    "Watchkeeper Guard",
                    egui::Rect::from_min_max(
                        egui::pos2(0.0, change.top() - 1.0),
                        egui::pos2(change.right(), f32::INFINITY),
                    ),
                )
            });
            (
                whole
                    && value
                        .zip(change)
                        .is_some_and(|(value, change)| change.left() >= value.right()),
                format!("{detail:?}; value {value:?}, Change Stat block {change:?}"),
                sized,
                level,
            )
        });
        d.observe("iter-25-D-FA3", sized.0, sized.1);
        d.observe("iter-25-D-AA2", level.0, level.1);
        d.observe(
            "iter-23-D-FG1",
            whole,
            format!("Each missing link's name stays whole before Change at {size:?}: {detail}"),
        );
        // Round 24 FA4: the file line never cuts the file name; when `…/<name>` does not fit
        // beside the links, the path takes its own row above them.
        let (named, detail) = d.at_size(size, |d| {
            let file = d.painted_rect_in("Watchkeeper.md", egui::Rect::EVERYTHING);
            let show = d.place_of("Show file");
            (
                file.is_some()
                    && !d.visible_text("Watchkee…")
                    && file.zip(show).is_some_and(|(file, show)| {
                        show.left() >= file.right() || show.top() >= file.bottom() - 0.5
                    }),
                format!("file {file:?}, Show file {show:?}"),
            )
        });
        d.observe(
            "iter-24-D-FA4",
            named,
            format!("The file line keeps Watchkeeper.md whole at {size:?}: {detail}"),
        );
    }
    // Round 27 FF5 and the lead's ruling on PR 300: with Lookup beside it, World's header keeps
    // every action with its word, as Lookup's does. Per reader: exactly one Edit, Start fight
    // and Add to Log, each with its word and inside its own pane. They share one row (equal
    // centres) when the measured row fits the pane; only when it does not, the row wraps at a
    // whole action, and the first row holds as many as fit.
    for size in [SMALL, DEFAULT, LARGE] {
        let (fine, detail) = d.at_size(size, |d| {
            let Some(splitter) = d.place_of("Resize Lookup column") else {
                return (false, "no Lookup splitter".to_owned());
            };
            let lookup_right = d
                .place_of("Close Lookup")
                .map_or(d.window_size().0, |close| close.right());
            let names = [
                "Edit Watchkeeper",
                "Start a fight with Watchkeeper",
                "Add to Log",
            ];
            let mut fine = true;
            let mut detail = Vec::new();
            for (pane, left, right) in [
                ("World", 0.0, splitter.left()),
                ("Lookup", splitter.right(), lookup_right),
            ] {
                let found: Vec<Vec<egui::Rect>> = names
                    .iter()
                    .map(|name| {
                        d.nodes()
                            .into_iter()
                            .filter(|node| {
                                node.name == *name
                                    && node.in_view()
                                    && node.rect.width() > design::Density::Read.control() + 0.5
                                    && node.rect.left() >= left
                                    && node.rect.right() <= right
                            })
                            .map(|node| node.rect)
                            .collect()
                    })
                    .collect();
                if found.iter().any(|rects| rects.len() != 1) {
                    fine = false;
                    detail.push(format!(
                        "{pane}: counts {:?}",
                        found.iter().map(Vec::len).collect::<Vec<_>>()
                    ));
                    continue;
                }
                let rects: Vec<egui::Rect> = found.into_iter().map(|rects| rects[0]).collect();
                let same =
                    |a: egui::Rect, b: egui::Rect| (a.center().y - b.center().y).abs() <= 0.5;
                let row = rects.iter().map(egui::Rect::width).sum::<f32>()
                    + design::XS * (rects.len() - 1) as f32;
                let fits = rects[0].left() + row <= right + 0.5;
                let first = rects
                    .iter()
                    .take_while(|rect| same(**rect, rects[0]))
                    .count();
                let pane_fine = if fits {
                    first == rects.len()
                } else {
                    // A wrap at a whole action, after as many as fit: the next one would not.
                    (1..rects.len()).contains(&first)
                        && rects[first].top() >= rects[0].bottom() - 0.5
                        && rects[first - 1].right() + design::XS + rects[first].width()
                            > right + 0.5
                };
                fine &= pane_fine;
                detail.push(format!(
                    "{pane}: right {right}, row fits {fits}, first row {first} of {}, {rects:?}",
                    rects.len()
                ));
            }
            (fine, detail.join("; "))
        });
        d.observe(
            "iter-27-D-FF5",
            fine,
            format!("At {size:?}, each reader's three actions: {detail}"),
        );
    }
    d.state("npc-stat-block-deleted-in-lookup", Sizes::All);
    d.press(Key::Escape);
    go_encounters(d);
    d.click(NEW_ENCOUNTER);
    d.type_into(FIND_CREATURE, "Watchkeeper");
    d.expect("Watchkeeper Guard stat block not found");
    assert!(d.is_disabled("Add"));
    assert!(!d.has("No creatures match"));
    d.state("npc-deleted-stat-block-encounter-search", Sizes::All);
    d.click("Open in World");
    d.expect("Watchkeeper Guard · Not found");
    // Only the rail's encounter draft and the focus differ, since the account line no longer
    // moves when a status is added above it.
    d.expect_same("Open in World returns to the World view of npc-stat-block-deleted-in-world");
    d.state("npc-stat-block-repair-from-search", Sizes::All);
}

/// Copying a typed note must keep both files reachable.
/// Do the duplicate repair choices and consequences start at the warning's text edge? Only nodes
/// that start right of `pane_left` count, so a check of one pane cannot pass on another's copies.
fn duplicate_choices_at_text(d: &Driver<'_>, pane_left: f32) -> (bool, String) {
    let warning = "These two files claim to be the same note.";
    let parts = [
        "Make Rook Ambry copy.md a separate note",
        "Existing links keep NPCs/Rook Ambry.md.",
        "Make Rook Ambry.md a separate note",
        "Existing links will then use NPCs/Rook Ambry copy.md.",
    ];
    for name in parts.iter().chain([&warning]) {
        d.expect(name);
    }
    let nodes = d.nodes();
    let texts: Vec<_> = nodes
        .iter()
        .filter(|node| node.name == warning && node.in_view() && node.rect.left() >= pane_left)
        .map(|node| node.rect)
        .collect();
    let found: Vec<_> = nodes
        .iter()
        .filter(|node| {
            parts.contains(&node.name.as_str()) && node.in_view() && node.rect.left() >= pane_left
        })
        .collect();
    let every_part = parts
        .iter()
        .all(|part| found.iter().any(|node| node.name == *part));
    // Each choice and consequence sits below a warning and starts at its text edge.
    let aligned = found.iter().all(|node| {
        texts.iter().any(|text| {
            text.top() < node.rect.top() && (text.left() - node.rect.left()).abs() < 1.0
        })
    });
    (
        !texts.is_empty() && every_part && aligned,
        format!(
            "texts at {:?}, parts at {:?}",
            texts.iter().map(|text| text.left()).collect::<Vec<_>>(),
            found
                .iter()
                .map(|node| (node.name.clone(), node.rect.left()))
                .collect::<Vec<_>>()
        ),
    )
}

pub fn world_note_duplicated(d: &mut Driver<'_>) {
    let campaign = d.app_mut().campaigns.active_campaign().unwrap();
    let catalogue = crate::content_workspace::Catalogue::load(&[], Some(&campaign)).unwrap();
    let guard = catalogue
        .entries
        .values()
        .find(|entry| entry.entry.name == "Guard" && entry.entry.kind == "creature")
        .unwrap()
        .entry
        .id;
    let id = entry(
        d,
        "NPCs/Rook Ambry.md",
        "Rook Ambry",
        "npc",
        &format!("  creature: {{id: {guard}}}\n"),
    );
    go_encounters(d);
    d.click("Reload from disk");
    d.click(NEW_ENCOUNTER);
    d.type_text("Rook's watch");
    d.type_into(FIND_CREATURE, "Rook Ambry");
    d.click_beside_exact("Rook Ambry", "Add");
    d.click(SAVE_ENCOUNTER);
    d.wait_for_text("Saved");
    let original = root(d).join("NPCs/Rook Ambry.md");
    let copy = root(d).join("NPCs/Rook Ambry copy.md");
    let before = fs::read_to_string(&original).unwrap();
    fs::copy(&original, &copy).unwrap();
    d.focus_window(false);
    d.focus_window(true);
    d.expect("6. Rook Ambry");
    d.replace_text_in(FIND_CREATURE, "Rook Ambry");
    d.expect("These two files claim to be the same note.");
    d.expect("Make Rook Ambry copy.md a separate note");
    assert!(
        !d.has("Changed"),
        "A copied NPC note does not change its roster values"
    );
    d.expect("Rook's watch keeps Rook Ambry.md.");
    d.expect("Rook's watch will then use Rook Ambry copy.md.");
    d.state("copied-world-npc-encounter-keeps-guard", Sizes::All);
    let tops = |d: &Driver<'_>| {
        [FIND_CREATURE, "Filter creatures"].map(|name| d.place_of(name).map(|rect| rect.top()))
    };
    let search_before = tops(d);
    d.click("Make Rook Ambry copy.md a separate note");
    d.wait_for_text("Undo separate note");
    d.expect("Rook Ambry copy.md is now a separate note.");
    // iter-27-B FF8, AF1, SF1: the result and its Undo are in the editor's footer, below the
    // search's results, and the search and its filter do not move.
    let result = d.painted_rect_in(
        "Rook Ambry copy.md is now a separate note.",
        egui::Rect::EVERYTHING,
    );
    let adds: Vec<f32> = d
        .nodes()
        .into_iter()
        .filter(|node| node.name == "Add" && node.in_view())
        .map(|node| node.rect.bottom())
        .collect();
    d.observe(
        "iter-27-B-FF8",
        search_before.iter().all(Option::is_some)
            && tops(d) == search_before
            && !adds.is_empty()
            && result.is_some_and(|result| adds.iter().all(|add| result.top() > *add)),
        format!(
            "Search tops before {search_before:?}, after {:?}; result {result:?} below Add {adds:?}",
            tops(d)
        ),
    );
    d.state("duplicate-npc-repaired-from-encounter", Sizes::All);
    // An unrelated change between the separation and its Undo stays: another search and a
    // Guard added from it. The result stays in the footer through the new search.
    d.replace_text_in(FIND_CREATURE, "Guard");
    d.click_beside_exact("Guard", "Add");
    d.expect("7. Guard");
    let kept = d.has("Undo separate note");
    d.click("Undo separate note");
    d.replace_text_in(FIND_CREATURE, "Rook Ambry");
    d.wait_for_text("Make Rook Ambry copy.md a separate note");
    d.observe(
        "iter-27-B-FF8-undo",
        kept && d.has("7. Guard") && !d.has("Rook Ambry copy.md is now a separate note."),
        format!(
            "Undo separate note ends its result and keeps the Guard added after it (kept through the search: {kept})"
        ),
    );
    // The rest of the scenario starts from the saved encounter.
    d.click("Discard");
    d.expect("6. Rook Ambry");
    assert!(!d.has("7. Guard"), "Discard takes back the Guard");
    d.click(NAV_WORLD);
    d.click_starting("Rook Ambry.md ·");
    d.expect("Make Rook Ambry copy.md a separate note");
    d.expect("Make Rook Ambry.md a separate note");
    for size in [SMALL, DEFAULT, LARGE] {
        let consequences = d.at_size(size, |d| {
            let first = "Existing links keep NPCs/Rook Ambry.md.";
            let second = "Existing links will then use NPCs/Rook Ambry copy.md.";
            for name in [
                "Make Rook Ambry copy.md a separate note",
                "Make Rook Ambry.md a separate note",
                first,
                second,
            ] {
                d.expect(name);
            }
            let mut matched = 0;
            let mut muted_small = true;
            d.each_painted_text(|text, clip| {
                if [first, second].contains(&text.galley.job.text.as_str())
                    && clip.intersects(text.visual_bounding_rect())
                {
                    matched += 1;
                    muted_small &= text.galley.job.sections.iter().all(|section| {
                        section.format.color == design::colors().muted
                            && section.format.font_id.size == design::Type::Small.size()
                    });
                }
            });
            let actions: Vec<_> = d
                .nodes()
                .into_iter()
                .filter(|node| {
                    node.role == Role::Button
                        && [
                            "Make Rook Ambry copy.md a separate note",
                            "Make Rook Ambry.md a separate note",
                        ]
                        .contains(&node.name.as_str())
                })
                .collect();
            matched == 2
                && muted_small
                && actions.len() == 2
                && actions.iter().all(|node| node.in_view())
        });
        d.observe(
            "iter-9-D-FD7",
            consequences,
            format!("Each duplicate repair has a muted consequence at {size:?}"),
        );
    }
    for size in [SMALL, DEFAULT, LARGE] {
        let gaps = d.at_size(size, |d| {
            let names = [
                "Make Rook Ambry copy.md a separate note",
                "Existing links keep NPCs/Rook Ambry.md.",
                "Make Rook Ambry.md a separate note",
                "Existing links will then use NPCs/Rook Ambry copy.md.",
                "Stat block",
            ];
            for name in names {
                d.expect(name);
            }
            // The stat block's label is its own node; a missing one fails this check.
            let [
                Some(action),
                Some(first),
                Some(second_action),
                Some(second),
                Some(field),
            ] = names.map(|name| d.place_of(name))
            else {
                return false;
            };
            // Neighbour: each consequence stays close below its own action.
            first.top() - action.bottom() < design::S
                && second_action.top() - first.bottom() >= design::L - 0.5
                && field.top() - second.bottom() >= design::M - 0.5
        });
        d.observe(
            "iter-10-D-FD5",
            gaps,
            format!("An L gap separates the two choices and an M gap ends them at {size:?}"),
        );
    }
    for size in [SMALL, DEFAULT, LARGE] {
        let (aligned, detail) = d.at_size(size, |d| duplicate_choices_at_text(d, 0.0));
        d.observe(
            "iter-13-D-FD4-world",
            aligned,
            format!("World's duplicate choices start at the message text at {size:?}: {detail}"),
        );
    }
    // Round 23 FG6: a duplicate note's stat block reads as every reader's does: a muted label,
    // then the value as a link that opens it. Only Change waits for the repair.
    let guard_links = |d: &Driver<'_>| {
        d.nodes()
            .into_iter()
            .filter(|node| node.role == Role::Link && node.name == "Guard" && node.in_view())
            .count()
    };
    for size in [SMALL, DEFAULT, LARGE] {
        let (read, detail) = d.at_size(size, |d| {
            let links = guard_links(d);
            let label = d.place_of("Stat block");
            let link = d
                .nodes()
                .into_iter()
                .find(|node| node.role == Role::Link && node.name == "Guard")
                .map(|node| node.rect);
            (
                links == 1
                    && !d.visible_text("Stat block: Guard")
                    && !d.has("Change Stat block")
                    // Must stay true: the label stays with its value, above it or beside it.
                    && label.zip(link).is_some_and(|(label, link)| {
                        link.center().y >= label.top() - 0.5
                            && link.top() - label.bottom() < design::M
                    }),
                format!("{links} Guard links, label {label:?}, link {link:?}"),
            )
        });
        d.observe(
            "iter-23-D-FG6-world",
            read,
            format!(
                "The duplicate's stat block is a labelled link with no Change at {size:?}: {detail}"
            ),
        );
    }
    d.state("duplicate-world-files-and-repair", Sizes::All);
    look_up(d, "Rook Ambry");
    d.expect("Make Rook Ambry copy.md a separate note");
    d.observe(
        "iter-23-D-FG6-lookup",
        guard_links(d) == 2 && !d.visible_text("Stat block: Guard"),
        "Lookup's duplicate reader links its stat block too, beside World's",
    );
    for size in [SMALL, DEFAULT, LARGE] {
        let rows = d.at_size(size, |d| {
            let rows: Vec<_> = d
                .nodes()
                .into_iter()
                .filter(|node| node.name.starts_with("Rook Ambry · NPCs/Rook Ambry"))
                .collect();
            let painted: Vec<_> = rows
                .iter()
                .map(|row| d.painted_lines_in(row.rect).concat())
                .collect();
            (
                rows.len() == 2
                    && rows.iter().all(|row| {
                        let file = row
                            .name
                            .split(" · ")
                            .nth(1)
                            .and_then(|path| path.rsplit('/').next())
                            .unwrap_or_default()
                            .to_owned();
                        // The file comes right after the name, before the cut.
                        row.in_view()
                            && d.painted_lines_in(row.rect)
                                .concat()
                                .starts_with(&format!("Rook Ambry · NPCs/{file} · "))
                    }),
                format!("{painted:?}"),
            )
        });
        d.observe(
            "iter-10-D-FD7",
            rows.0,
            format!(
                "Each duplicate Lookup row shows its file right after the name at {size:?}: {}",
                rows.1
            ),
        );
    }
    d.observe_both(
        "iter-11-D-FD3",
        "The Lookup reader's kind line names its file, not the fault its message names",
        |d| {
            let kinds: Vec<_> = d
                .nodes()
                .into_iter()
                .filter(|node| node.in_view() && node.name.starts_with("NPC · "))
                .map(|node| node.name)
                .collect();
            let rows: Vec<_> = d
                .nodes()
                .into_iter()
                .filter(|node| node.name.starts_with("Rook Ambry · NPCs/Rook Ambry"))
                .collect();
            // Neighbour: each result row keeps the fault.
            kinds.contains(&"NPC · Campaign · NPCs/Rook Ambry copy.md".to_owned())
                && kinds
                    .iter()
                    .all(|kind| !kind.contains("Same note in two files"))
                && rows.len() == 2
                && rows
                    .iter()
                    .all(|row| row.name.contains("Same note in two files"))
        },
    );
    for size in [SMALL, DEFAULT, LARGE] {
        let (aligned, detail) = d.at_size(size, |d| {
            // Lookup's own copies only: the World pane beside it shows the same message.
            let pane_left = d
                .place_of("Resize Lookup column")
                .map_or(f32::INFINITY, |splitter| splitter.right());
            let (aligned, detail) = duplicate_choices_at_text(d, pane_left);
            (aligned, format!("Lookup pane from x {pane_left}; {detail}"))
        });
        d.observe(
            "iter-13-D-FD4-lookup",
            aligned,
            format!("Lookup's duplicate choices start at the message text at {size:?}: {detail}"),
        );
    }
    // Round 24 FA4: at 960 beside Lookup, World's file line puts the path on its own row, so
    // World's `Also linked from` line sits just below the window. Scroll the World reader to it.
    let world_reader = egui::pos2(470.0, 400.0);
    d.scroll_at(world_reader, 2);
    d.observe_both(
        "iter-15-D-FD7",
        "Also linked from never names the note itself, and names a copy by its path",
        |d| {
            let links: Vec<String> = d
                .nodes()
                .into_iter()
                .filter(|node| node.role == Role::Link && node.in_view())
                .map(|node| node.name)
                .collect();
            // World reads Rook Ambry.md and Lookup reads the copy: each names only the other.
            let count = |path: &str| links.iter().filter(|name| *name == path).count();
            d.visible_text("Also linked from")
                && count("NPCs/Rook Ambry.md") == 1
                && count("NPCs/Rook Ambry copy.md") == 1
                && !links.iter().any(|name| name == "Rook Ambry")
        },
    );
    d.scroll_up_at(world_reader, 2);
    // FD6: a Lookup row shortens by whole parts. It drops the kind and keeps the whole file that
    // tells the rows apart. At 1200 the fault is whole; at 960 it ends on a whole word. No row
    // cuts inside a word or ends on a separator before `…`.
    for size in [SMALL, DEFAULT] {
        let (whole, detail) = d.at_size(size, |d| {
            let rows: Vec<_> = d
                .nodes()
                .into_iter()
                .filter(|node| node.name.starts_with("Rook Ambry · ") && node.in_view())
                .collect();
            let painted: Vec<String> = rows.iter().map(|row| d.painted_in(row.rect)).collect();
            let files = ["NPCs/Rook Ambry copy.md", "NPCs/Rook Ambry.md"];
            (
                rows.len() == 2
                    && painted.iter().all(|text| {
                        let fault = format!("{} Same note in two files", design::icons::WARN);
                        let tail = text.rsplit(" · ").next().unwrap_or_default();
                        (if size == DEFAULT {
                            tail == fault
                        } else {
                            tail.strip_suffix('…').is_some_and(|kept| {
                                fault.starts_with(kept) && fault[kept.len()..].starts_with(' ')
                            })
                        }) && !text.split(" · ").any(|part| part.trim() == "NPC")
                            && !text.contains("· …")
                    })
                    && files
                        .iter()
                        .all(|file| painted.iter().any(|text| text.contains(file))),
                format!("{size:?}: {painted:?}"),
            )
        });
        d.observe(
            "iter-16-D-FD6",
            whole,
            format!("Lookup rows drop the kind and keep the file and the whole fault: {detail}"),
        );
    }
    // Round 23 FD8: a Lookup row's fault mark has the warning colour, as on a Library row.
    let pane_left = d
        .place_of("Resize Lookup column")
        .map_or(f32::INFINITY, |splitter| splitter.right());
    let mut marks = Vec::new();
    d.each_painted_text(|text, clip| {
        let job = &text.galley.job;
        // At 960 the fault shortens to whole words and then to its mark; the file stays.
        if job.text.contains("NPCs/Rook Ambry")
            && job.text.contains(design::icons::WARN)
            && text.pos.x >= pane_left
            && clip.intersects(text.visual_bounding_rect())
        {
            marks.push(painted_marks(job));
        }
    });
    d.observe(
        "iter-23-L-FD8-lookup",
        marks.len() == 2 && warning_marked(&marks),
        format!("Both Lookup rows mark the fault in the warning colour: {marks:?}"),
    );
    d.state("duplicate-world-files-in-lookup", Sizes::All);
    d.press(Key::Escape);
    d.click("Make Rook Ambry copy.md a separate note");
    d.wait_for_text("Undo separate note");
    d.expect("Rook Ambry copy.md is now a separate note.");
    assert_eq!(
        crate::content_workspace::note_id(&fs::read_to_string(&original).unwrap()),
        Some(id)
    );
    assert_ne!(
        crate::content_workspace::note_id(&fs::read_to_string(&copy).unwrap()),
        Some(id)
    );
    assert_eq!(fs::read_to_string(&original).unwrap(), before);
    d.expect("Rook Ambry copy");
    d.state("world-copy-repaired", Sizes::All);
    d.click("Undo separate note");
    d.wait_for_text("Make Rook Ambry copy.md a separate note");
    assert_eq!(fs::read_to_string(&copy).unwrap(), before);
    d.expect_same("Undo restores the same copied files and identity warning.");
    d.state("world-copy-repair-undone", Sizes::All);
    // Round 23 FG6: the duplicate's stat block link opens Guard's stat block.
    let shown_before = d.has_containing("ARMOR CLASS");
    d.click("Guard");
    d.wait_for_text("ARMOR CLASS");
    d.observe(
        "iter-23-D-FG6-opens",
        !shown_before && d.has_containing("ARMOR CLASS"),
        "Guard on the duplicate note opens its stat block",
    );
    // World's reader opened Guard in place; the note's row brings the duplicate back.
    d.click_starting("Rook Ambry.md ·");
    d.expect("Make Rook Ambry copy.md a separate note");
    d.click("Make Rook Ambry copy.md a separate note");
    d.wait_for_text("Undo separate note");
    d.expect("Rook Ambry copy.md is now a separate note.");
    let same_name = before.replace("splat_name: Rook Ambry", "splat_name: Rook Ambry copy");
    fs::write(&original, same_name).unwrap();
    reload(d);
    d.expect("Rook Ambry.md");
    d.expect("Rook Ambry copy.md");
    // FD5, must stay true: after a long title, Edit's icon also sits one `L` after it.
    let gap = edit_gap(d, "Rook Ambry copy");
    d.observe(
        "iter-16-D-FD5-long",
        gap.is_some_and(|gap| (gap - design::L).abs() <= 1.0),
        format!("Edit's icon is {gap:?} px after the title Rook Ambry copy"),
    );
    d.state("same-name-npcs-show-file-names", Sizes::All);
}

pub fn world_note_renamed(d: &mut Driver<'_>) {
    entry(d, "NPCs/Rook Ambry.md", "Rook Ambry", "npc", "");
    fs::rename(
        root(d).join("NPCs/Rook Ambry.md"),
        root(d).join("NPCs/Mara Vell.md"),
    )
    .unwrap();
    reload(d);
    d.click(NAV_WORLD);
    d.click_starting("Rook Ambry ·");
    d.expect("NPCs/Mara Vell.md");
    d.expect("The file is Mara Vell.md, but the name is Rook Ambry.");
    d.state("renamed-npc-keeps-old-name-in-world", Sizes::All);
    look_up(d, "Rook Ambry");
    d.expect("Rook Ambry");
    d.state("renamed-npc-keeps-old-name-in-lookup", Sizes::All);
    d.press(Key::Escape);
    d.click("Rename the note to Mara Vell");
    d.wait_for_text("Undo name change");
    let (success, detail) = finished_change_tone(d, "Name changed to Mara Vell.");
    d.observe("iter-20-D-FD5-name", success, detail);
    for size in [SMALL, DEFAULT, LARGE] {
        let icon = d.at_size(size, |d| {
            d.place_of("Undo name change")
                .is_some_and(|rect| d.painted_in(rect).contains(design::icons::UNDO))
        });
        d.observe(
            "iter-5-D-name-undo",
            icon,
            format!("Name Undo has the shared icon at {size:?}"),
        );
    }
    d.state("renamed-npc-name-updated", Sizes::All);
    look_up(d, "Mara Vell");
    d.expect("Mara Vell");
    d.state("renamed-npc-new-name-in-lookup", Sizes::All);
    d.press(Key::Escape);
    d.click("Undo name change");
    d.wait_for_text("Rename the note to Mara Vell");
}

pub fn package_folder_copied(d: &mut Driver<'_>) {
    fn copy_folder(from: &std::path::Path, to: &std::path::Path) {
        fs::create_dir_all(to).unwrap();
        for file in fs::read_dir(from).unwrap() {
            let file = file.unwrap();
            let target = to.join(file.file_name());
            if file.path().is_dir() {
                copy_folder(&file.path(), &target);
            } else {
                fs::copy(file.path(), target).unwrap();
            }
        }
    }
    let copied = d.root.join("modules/ashen-marches-copy");
    copy_folder(&d.root.join("modules/ashen-marches"), &copied);
    let before = fs::read(copied.join("Module.md")).unwrap();
    go_library(d);
    d.click("Packages");
    d.click("Manage packages…");
    d.type_into("Package folder", &copied.to_string_lossy());
    d.click("Add folder");
    d.expect("This folder is a copy of Ashen Marches.");
    assert!(d.is_disabled("Add folder"));
    assert_eq!(d.app_mut().campaigns.modules().len(), 1);
    // Round 27 FA7, FD6: the warning is the reason. The kept reason line under Add folder stays
    // empty; Add folder's accessible description and hover keep `This folder is a copy.`
    let copy_reason_once = |d: &mut Driver<'_>, id: &str| {
        let line_empty = !d.visible_text("This folder is a copy.");
        let described = d
            .descriptions()
            .iter()
            .any(|words| words == "This folder is a copy.");
        let hover = d.tooltip_contains("Add folder", "This folder is a copy.");
        d.observe(
            id,
            d.visible_text("This folder is a copy of Ashen Marches.") && line_empty && described && hover,
            format!(
                "The copy fact shows once, as the warning; Add folder keeps the reason (line empty {line_empty}, description {described}, hover {hover})"
            ),
        );
    };
    copy_reason_once(d, "iter-27-L-FA7");
    d.state("copied-package-names-duplicate-identity", Sizes::All);
    d.click("Register as a new package");
    d.clear_text_in("New package name");
    d.expect("Type a name first.");
    assert!(d.is_disabled("Register"));
    // Round 27 FD2: the inline creation form's one order. The choice first, `Register` at the
    // end of the name field's line, then quiet Cancel at the start of the kept line below with
    // the reason after it. Typing a name moves neither action.
    let form_places = |d: &mut Driver<'_>, reason: Option<&str>| {
        [SMALL, DEFAULT, LARGE].map(|size| {
            d.at_size(size, |d| {
                let field = d
                    .nodes()
                    .into_iter()
                    .find(|node| {
                        node.name == "New package name"
                            && node.role == egui::accesskit::Role::TextInput
                    })?
                    .rect;
                let choice = d.place_of("Choose for this campaign")?;
                let register = d.place_of("Register")?;
                let cancel = d.place_of("Cancel")?;
                let reason_ok = reason.is_none_or(|reason| {
                    d.painted_rect_in(reason, egui::Rect::EVERYTHING)
                        .is_some_and(|words| {
                            words.left() >= cancel.right() + design::L - 1.0
                                && (words.center().y - cancel.center().y).abs() <= 1.0
                        })
                });
                (choice.bottom() < field.top()
                    && (register.center().y - field.center().y).abs() <= 1.0
                    && register.left() > field.right()
                    && cancel.top() >= field.bottom() - 0.5
                    && (cancel.left() - field.left()).abs() <= design::S
                    && reason_ok)
                    .then_some((register, cancel))
            })
        })
    };
    let empty = form_places(d, Some("Type a name first."));
    d.observe(
        "iter-27-L-FD2-order",
        empty.iter().all(Option::is_some),
        format!("Choice, then the name with Register on its line, then Cancel and the reason below: {empty:?}"),
    );
    d.state("copied-package-needs-a-name", Sizes::All);
    d.replace_text_in("New package name", "Ashen Reaches");
    let named = form_places(d, None);
    d.observe(
        "iter-27-L-FD2-still",
        empty.iter().all(Option::is_some) && empty == named,
        format!(
            "Register and Cancel keep their place when the name is typed: {empty:?} → {named:?}"
        ),
    );
    d.expect("REGISTER A COPY");
    d.expect("This campaign keeps the original. The copy is registered but not chosen.");
    d.expect("Cancel");
    d.observe_both(
        "iter-4-D-FD2",
        "The copy warning stays above the named registration form and Cancel",
        |d| {
            let warning = d
                .place_of("This folder is a copy of Ashen Marches.")
                .unwrap();
            let title = d.place_of("REGISTER A COPY").unwrap();
            let register = d.place_of("Register").unwrap();
            let cancel = d.place_of("Cancel").unwrap();
            // Round 27 FD2: Cancel starts the kept line below Register's line.
            warning.bottom() < title.top()
                && title.bottom() < register.top()
                && cancel.top() >= register.bottom() - 0.5
                && cancel.right() < register.left()
        },
    );
    // Round 24 FF3: the ready form's commit is primary, as `Create package` is, and it is the
    // only accent-filled control on the screen.
    let primary = super::session_recording::accent_filled(d);
    d.observe(
        "iter-24-L-FF3",
        primary == ["Register"],
        format!("The ready registration form's commit is the one primary: {primary:?}"),
    );
    d.state("copied-package-new-name", Sizes::All);
    d.click("Choose for this campaign");
    d.expect("Uses every entry in this package, including future entries.");
    assert!(!d.has("This campaign keeps the original. The copy is registered but not chosen."));
    d.state("copied-package-use-enabled", Sizes::All);
    d.click("Cancel");
    d.expect("Register as a new package");
    assert!(!d.has("New package name"));
    assert_eq!(d.app_mut().campaigns.modules().len(), 1);
    assert_eq!(fs::read(copied.join("Module.md")).unwrap(), before);
    copy_reason_once(d, "iter-27-L-FD6");
    d.expect_same(
        "Cancel restores the copied-folder warning and its registration action without writing.",
    );
    d.state("copied-package-registration-cancelled", Sizes::All);
    d.click("Register as a new package");
    d.expect("Register");
    d.expect("Uses every entry in this package, including future entries.");
    d.click("Choose for this campaign");
    d.expect("This campaign keeps the original. The copy is registered but not chosen.");
    // Round 24 FC4: the search field's place below the page header, before any result.
    let columns_top = |d: &mut Driver<'_>| {
        [SMALL, DEFAULT, LARGE].map(|size| {
            d.at_size(size, |d| {
                Some(d.place_of("Search the Library")?.top() - d.place_of("Packages")?.top())
            })
        })
    };
    let columns_before = columns_top(d);
    d.click("Register");
    d.wait_for_text("Undo package registration");
    let columns_after = columns_top(d);
    d.observe(
        "iter-24-L-FC4-registration",
        columns_before.iter().all(Option::is_some) && columns_before == columns_after,
        format!(
            "The registration result sits in the reader, so the columns keep their place below the header ({columns_before:?} → {columns_after:?})"
        ),
    );
    // Round 21 FD6, round 23 AD1: the result says what the registration did, with no status
    // phrase. A later source change can make encounters use the copy, so it claims no use now.
    d.observe(
        "iter-23-L-AD1",
        d.visible_text("Ashen Reaches is registered, with no entries chosen for this campaign.")
            && !d.has_containing("is registered ·")
            && !d.has_containing("This campaign does not use it."),
        "The registration result states the completed action, not the campaign's current use",
    );
    search_library(d, "Cinder Wolf");
    d.expect("Ashen Marches");
    d.expect("Ashen Reaches");
    assert_eq!(d.app_mut().campaigns.modules().len(), 2);
    for size in [SMALL, DEFAULT, LARGE] {
        let icon = d.at_size(size, |d| {
            d.place_of("Undo package registration")
                .is_some_and(|rect| d.painted_in(rect).contains(design::icons::UNDO))
        });
        d.observe(
            "iter-5-D-package-undo",
            icon,
            format!("Registration Undo has the shared icon at {size:?}"),
        );
    }
    d.state("copied-package-registered-alongside-original", Sizes::All);
    // SD1: `Undo package registration` follows its message on the message line through
    // `undo_message`, with its icon before its words. When the line has no room it starts the
    // next line, with its words at the message's words and its icon hanging over the message's
    // icon column (the unit test `iter18_a_sd1_a_wrapped_undo_starts_its_words_at_the_message_words`
    // reaches that branch). Either way, its press area clears the entries column's splitter.
    for size in [SMALL, DEFAULT, LARGE] {
        let (undo, message, icon, clear) = d.at_size(size, |d| {
            let target = d.place_of("Undo package registration");
            let mut icon = None;
            if let Some(target) = target {
                d.each_painted_text(|text, _| {
                    let rect = egui::Rect::from_min_size(text.pos, text.galley.size());
                    if target.contains(rect.center()) && text.galley.job.text == design::icons::UNDO
                    {
                        icon = Some(rect);
                    }
                });
            }
            let mut message = None;
            d.each_painted_text(|text, clip| {
                let rect = egui::Rect::from_min_size(text.pos, text.galley.size());
                if text
                    .galley
                    .job
                    .text
                    .starts_with("Ashen Reaches is registered")
                    && clip.intersects(rect)
                {
                    message = Some(rect);
                }
            });
            (
                target
                    .and_then(|target| super::words_left_in(d, target).map(|left| (left, target))),
                message,
                icon,
                target.map_or_else(
                    || Err("Undo package registration is not in view".to_owned()),
                    |target| super::press_area_clear(d, target, "Undo package registration", None),
                ),
            )
        });
        let placed =
            undo.zip(message)
                .zip(icon)
                .is_some_and(|(((words, target), message), icon)| {
                    if target.top() >= message.bottom() - 1.0 {
                        (words - message.left()).abs() <= 1.0 && icon.right() <= message.left()
                    } else {
                        icon.left() > message.right() && words > icon.right()
                    }
                });
        d.observe(
            "iter-18-A-SD1",
            placed && clear.is_ok(),
            format!(
                "At {size:?}, Undo package registration's words and target {undo:?}, the message \
                 {message:?}, its icon {icon:?}; press area: {clear:?}"
            ),
        );
    }
    for size in [SMALL, DEFAULT, LARGE] {
        // Round 23 FB3: a message choice without an icon has the secondary border; only Undo
        // is quiet. The border starts at the message words, below the message.
        let (action, words, message, frame, icon, clear) = d.at_size(size, |d| {
            d.expect(crate::library::USE_COPY);
            let action = d.place_of(crate::library::USE_COPY);
            let words = action.and_then(|action| super::words_left_in(d, action));
            let message = d
                .nodes()
                .into_iter()
                .find(|n| {
                    n.role == Role::Label && n.name.starts_with("Ashen Reaches is registered")
                })
                .map(|n| n.rect);
            let frame = d
                .control_frame(crate::library::USE_COPY)
                .map(|frame| frame.stroke.color);
            let icon = action.is_some_and(|action| {
                d.painted_in(action)
                    .chars()
                    .any(|c| ('\u{E000}'..='\u{F8FF}').contains(&c))
            });
            let clear = action.map_or_else(
                || Err("Use the copy in encounters is not in view".to_owned()),
                |action| super::press_area_clear(d, action, crate::library::USE_COPY, None),
            );
            (action, words, message, frame, icon, clear)
        });
        d.observe(
            "iter-23-C-FB3-use-copy",
            action.zip(words).zip(message).is_some_and(|((action, words), message)| {
                action.top() >= message.bottom()
                    && (action.left() - message.left()).abs() <= 1.0
                    && words > action.left()
            }) && frame == Some(design::colors().field_border)
                && !icon
                && clear.is_ok(),
            format!(
                "At {size:?}, Use the copy in encounters {action:?} starts its words at {words:?}, the message words {message:?}; border {frame:?}; icon {icon}; press area: {clear:?}"
            ),
        );
        // FD1: the Undo sits on the message line, after the words, so the message takes two
        // lines (message, then Use the copy in encounters), not three.
        let (undo, message, action) = d.at_size(size, |d| {
            let message = d
                .nodes()
                .into_iter()
                .find(|n| {
                    n.role == Role::Label && n.name.starts_with("Ashen Reaches is registered")
                })
                .map(|n| n.rect);
            (
                d.place_of("Undo package registration"),
                message,
                d.place_of(crate::library::USE_COPY),
            )
        });
        // Round 24 FC4: the result sits in the reader's measure. Undo stays on the message line
        // where both fit; where they do not, it starts the next line (iter-18-A-SD1 checks its
        // words there). Use the copy in encounters is the one row below Undo either way.
        let measure = crate::design::READER_MEASURE;
        d.observe(
            "iter-18-E-FD1",
            undo.zip(message).zip(action).is_some_and(|((undo, message), action)| {
                let same_line = (undo.center().y - message.center().y).abs() <= 1.0
                    && undo.left() >= message.right();
                let wrapped = undo.top() >= message.bottom() - 1.0
                    && message.width() + undo.width() > measure;
                (same_line || wrapped) && action.top() >= undo.bottom()
            }),
            format!(
                "At {size:?}, Undo package registration {undo:?} is on the message line {message:?} where both fit in the measure ({measure}), and Use the copy in encounters {action:?} is the one row below"
            ),
        );
    }
    // AD1, a result with another action: an outside change to a copied file blocks Undo package
    // registration with its reason, and Use the copy in encounters stays. Restoring the bytes and
    // reloading brings Undo back, so the later steps see the usual state.
    {
        let reload_library = |d: &mut Driver<'_>| {
            d.click("Packages");
            d.click("Reload from disk");
            d.wait_for_text("Undo package registration");
        };
        let (entries, _) = splat::content::read_directory(&copied).unwrap();
        let file = copied.join(
            &entries
                .iter()
                .find(|entry| entry.kind != "package")
                .expect("fixture: a copied entry")
                .path,
        );
        let written = fs::read(&file).unwrap();
        assert!(
            !d.is_disabled("Undo package registration"),
            "fixture: Undo starts available"
        );
        let mut outside = written.clone();
        outside.extend_from_slice(b"\nAn outside note.\n");
        fs::write(&file, &outside).unwrap();
        reload_library(d);
        let blocked = d.is_disabled("Undo package registration");
        let use_kept =
            d.in_view(crate::library::USE_COPY) && !d.is_disabled(crate::library::USE_COPY);
        let reason = d.tooltip_contains(
            "Undo package registration",
            crate::content_workspace::UNDO_ENDED,
        );
        d.state("registration-undo-blocked", Sizes::Default);
        // Round 23 FD6: the change still happened, so its words keep the text colour. Only Undo
        // greys, and its reason shows at the message words.
        let words = "Ashen Reaches is registered, with no entries chosen for this campaign.";
        let (color, said) = blocked_reason_shown(d, words);
        d.observe(
            "iter-23-L-FD6-registration",
            color == Some(design::colors().text) && said,
            format!(
                "The blocked registration result keeps the text colour ({color:?}) and shows why Undo is blocked at its words ({said})"
            ),
        );
        d.observe(
            "iter-14-E-AD1-registration",
            blocked && use_kept && reason && fs::read(&file).unwrap() == outside,
            format!(
                "After an outside change, Undo package registration is blocked ({blocked}) and says why ({reason}); Use the copy in encounters stays ({use_kept}); the file keeps the outside bytes"
            ),
        );
        fs::write(&file, &written).unwrap();
        reload_library(d);
        d.observe(
            "iter-14-E-AD1-registration-back",
            !d.is_disabled("Undo package registration"),
            "With the written bytes back, a reload makes Undo package registration available again",
        );
    }
    go_campaign(d);
    d.expect("Ashen Reaches");
    d.state("campaign-after-copy-registration", Sizes::All);
    go_library(d);
    d.expect(crate::library::USE_COPY);
    // The campaign's notes before the change, to find the encounter files that it writes.
    fn notes_in(folder: &std::path::Path, out: &mut Vec<(std::path::PathBuf, Vec<u8>)>) {
        for item in fs::read_dir(folder).unwrap().flatten() {
            let path = item.path();
            if path.is_dir() {
                notes_in(&path, out);
            } else if path.extension().is_some_and(|ext| ext == "md") {
                out.push((path.clone(), fs::read(&path).unwrap()));
            }
        }
    }
    let campaign_folder = d.app_mut().campaigns.active_campaign().unwrap().path;
    let mut notes_before = Vec::new();
    notes_in(&campaign_folder, &mut notes_before);
    d.click(crate::library::USE_COPY);
    d.wait_for_text("Undo source change");
    // AD3: the result is a message, and its Undo follows it with its icon.
    undo_follows_message(
        d,
        "iter-19-E-AD3-source",
        crate::library::USE_COPY_RESULT,
        "Undo source change",
    );
    d.state("copy-entries-used", Sizes::All);
    // Round 26 review: each kept result is its own footer message, newest first: the source
    // change, then the registration it followed. Both Undos are reached; at 960 the footer
    // scrolls to the older one.
    for size in [SMALL, DEFAULT, LARGE] {
        d.set_base(size);
        let order = d
            .nodes()
            .into_iter()
            .find(|node| {
                node.name.contains(crate::library::USE_COPY_RESULT)
                    && node.name.contains("is registered, with")
            })
            .map(|node| node.name)
            .unwrap_or_default();
        let newest_first = order
            .find(crate::library::USE_COPY_RESULT)
            .zip(order.find("is registered, with"))
            .is_some_and(|(source, registration)| source < registration);
        let first_in_view = d.in_view("Undo source change");
        let footer = d.place_of("Undo source change").map(|undo| undo.center());
        let older_reached = footer.is_some_and(|at| {
            d.scroll_until(at, "Undo package registration", 8);
            d.in_view("Undo package registration")
        });
        if let Some(at) = footer {
            d.scroll_up_at(at, 8);
        }
        d.observe(
            "iter-26-L-results-order",
            newest_first && first_in_view && older_reached,
            format!(
                "At {size:?}: newest first {newest_first} in {order:?}; Undo source change in view {first_in_view}; Undo package registration reached {older_reached}"
            ),
        );
    }
    d.set_base(SMALL);
    // AD3, a result with another action: an outside change to an encounter that the change wrote
    // blocks Undo source change with its reason, inside its message. Restoring the bytes and
    // reloading brings it back, so the later steps see the usual state.
    {
        let reload_library = |d: &mut Driver<'_>| {
            d.click("Packages");
            d.click("Reload from disk");
            d.wait_for_text("Undo source change");
        };
        let (changed, written) = notes_before
            .iter()
            .find_map(|(path, before)| {
                let now = fs::read(path).unwrap();
                (now != *before).then(|| (path.clone(), now))
            })
            .expect("Use the copy in encounters wrote an encounter file");
        assert!(
            !d.is_disabled("Undo source change"),
            "fixture: Undo source change starts available"
        );
        let mut outside = written.clone();
        outside.extend_from_slice(b"\nAn outside note.\n");
        fs::write(&changed, &outside).unwrap();
        reload_library(d);
        let blocked = d.is_disabled("Undo source change");
        let reason = d.tooltip_contains("Undo source change", crate::content_workspace::UNDO_ENDED);
        let kept = d.has(crate::library::USE_COPY_RESULT);
        // Round 23 FD6: the same rule for the source change's result.
        let (color, said) = blocked_reason_shown(d, crate::library::USE_COPY_RESULT);
        d.observe(
            "iter-23-L-FD6-source",
            color == Some(design::colors().text) && said,
            format!(
                "The blocked source-change result keeps the text colour ({color:?}) and shows why Undo is blocked at its words ({said})"
            ),
        );
        d.observe(
            "iter-19-E-AD3-source-blocked",
            blocked && reason && kept && fs::read(&changed).unwrap() == outside,
            format!(
                "After an outside change to {}, Undo source change is blocked ({blocked}) and says why ({reason}); its message stays ({kept}); the file keeps the outside bytes",
                changed.display()
            ),
        );
        fs::write(&changed, &written).unwrap();
        reload_library(d);
        d.observe(
            "iter-19-E-AD3-source-back",
            !d.is_disabled("Undo source change"),
            "With the written bytes back, a reload makes Undo source change available again",
        );
    }
    // Round 26 (Astra review of A): in Edit Markdown each kept result is its own footer message,
    // and results with a live Undo are listed newest first: the source change, made after the
    // registration, comes above it. Both sit below the editor, at 960, 1200 and 1920.
    d.click(EDIT_MARKDOWN);
    const REGISTERED: &str = "Ashen Reaches is registered";
    let newest_first = [SMALL, DEFAULT, LARGE].map(|size| {
        d.at_size(size, |d| {
            let editor = d.input_frame("Markdown editor")?.0;
            let newer =
                d.painted_rect_in(crate::library::USE_COPY_RESULT, egui::Rect::EVERYTHING)?;
            let older = d.painted_rect_in(REGISTERED, egui::Rect::EVERYTHING)?;
            Some((newer.top() >= editor.bottom(), newer.top() < older.top()))
        })
    });
    d.observe(
        "iter-26-A-footer-newest-first",
        newest_first == [Some((true, true)); 3],
        format!(
            "Edit Markdown's footer lists the source change, then the older registration, below the editor at 960, 1200, 1920 (below, newer above): {newest_first:?}"
        ),
    );
    d.click("Read");
    go_encounters(d);
    if d.has("Back to encounters") {
        d.click("Back to encounters");
    }
    open_saved_encounter(d, RUN);
    d.type_into(FIND_CREATURE, "Cinder Wolf");
    d.expect("Ashen Reaches");
    d.state("encounter-uses-copy-with-kept-changes", Sizes::All);
    go_library(d);
    d.click("Undo source change");
    d.click("Undo package registration");
    d.wait_for_text("Ashen Marches");
    assert_eq!(d.app_mut().campaigns.modules().len(), 1);
    assert_eq!(fs::read(copied.join("Module.md")).unwrap(), before);
    d.state("copied-package-registration-undone", Sizes::All);
    let hand_edit = crate::content_workspace::patch_fields(
        std::str::from_utf8(&before).unwrap(),
        &[
            (
                "splat_id",
                Some(crate::hermetic::new_uuid().to_string().into()),
            ),
            ("splat_namespace", Some("manual_copy".into())),
            ("splat_key", Some("manual_copy:library".into())),
        ],
    )
    .unwrap();
    fs::write(copied.join("Module.md"), hand_edit).unwrap();
    let module = crate::library::local_module(&copied).unwrap();
    d.app_mut().campaigns.register_module(module).unwrap();
    d.settle();
    d.wait_for_text("Same entry in two files");
    // Round 26: the choices sit in the reader's message footer, which scrolls inside when they do
    // not fit. Each batch choice is reached by scrolling the footer, then checked.
    for size in [SMALL, DEFAULT, LARGE] {
        d.set_base(size);
        let footer = d
            .painted_rect_in(crate::world::DUPLICATE_ENTRY, egui::Rect::EVERYTHING)
            .expect("the duplicate warning is in the footer")
            .center();
        let mut failed = Vec::new();
        for (folder, result) in [
            (
                "ashen-marches-copy",
                "Bridge Ambush and Shrine of Cinders keep the files in ashen-marches/.",
            ),
            (
                "ashen-marches",
                "Bridge Ambush and Shrine of Cinders will then use the files in ashen-marches-copy/.",
            ),
        ] {
            d.scroll_until(footer, result, 12);
            // In view can mean its first line only: scroll on until all of it shows.
            for _ in 0..4 {
                if d.visible_text(result) {
                    break;
                }
                d.scroll_at(footer, 1);
            }
            let button = d.nodes().into_iter().find(|node| {
                node.name.starts_with("Make all ")
                    && node
                        .name
                        .ends_with(&format!("files in {folder}/ separate entries"))
            });
            let line = d.place_of(result);
            let fits = button.as_ref().zip(line).is_some_and(|(button, line)| {
                d.visible_text(result)
                    && line.top() >= button.rect.bottom()
                    && line.top() - button.rect.bottom() <= design::XS + 1.0
                    && (line.left() - button.rect.left()).abs() < 1.0
            });
            if !fits {
                failed.push(format!(
                    "{folder}: button {:?}, line {line:?}, painted {}",
                    button.map(|button| button.rect),
                    d.visible_text(result)
                ));
            }
        }
        d.scroll_up_at(footer, 12);
        d.observe(
            "iter-5-D-FD5",
            failed.is_empty(),
            format!(
                "Each batch choice names its encounter and remaining folder at {size:?}: {failed:?}"
            ),
        );
    }
    d.set_base(SMALL);
    d.state("manifest-only-hand-repair-keeps-both-files", Sizes::All);
    // FD4, round 26: the four choices sit in the reader's message footer, at most a third of the
    // pane, so the body above stays in reach: a scroll over the body moves the title and brings
    // the body into view, and the choices stay where they are. Checked at 960x640.
    {
        let single = "Make Cinder Wolf.md in ashen-marches-copy/ a separate entry";
        let last_choice = |d: &Driver<'_>| {
            d.nodes()
                .into_iter()
                .filter(|node| node.name.starts_with("Make all ") && node.in_view())
                .map(|node| node.rect)
                .max_by(|a, b| a.top().total_cmp(&b.top()))
        };
        d.expect(single);
        let title = d
            .scroll_position_of_reader("Cinder Wolf")
            .expect("the reader title is on the page");
        let choice = d.place_of(single).unwrap();
        let last = last_choice(d).expect("a folder choice is in view");
        let warning = d
            .painted_rect_in(crate::world::DUPLICATE_ENTRY, egui::Rect::EVERYTHING)
            .expect("the duplicate warning is painted");
        let at = egui::pos2(choice.center().x, title + 4.0);
        d.scroll_at(at, 2);
        let title_after = d
            .scroll_position_of_reader("Cinder Wolf")
            .unwrap_or(f32::MIN);
        let moved = title - title_after;
        let choice_moved = choice.top() - d.place_of(single).map_or(f32::MAX, |rect| rect.top());
        let last_moved = last_choice(d).map_or(0.0, |rect| last.top() - rect.top());
        d.scroll_up_at(at, 4);
        let restored = d.scroll_position_of_reader("Cinder Wolf") == Some(title);
        d.observe(
            "iter-14-E-FD4",
            title >= 0.0
                && title < choice.top()
                && moved > 0.0
                && choice_moved.abs() < 1.0
                && last_moved.abs() < 1.0
                && restored,
            format!(
                "A scroll over the body moves the reader title and not the copied-folder choices in the footer: title {title:.0} moved {moved:.0}, single choice moved {choice_moved:.0}, last folder choice moved {last_moved:.0}, restored {restored}"
            ),
        );
        // Must stay true: the choices still start at the message text edge.
        d.observe(
            "iter-14-E-FD4-edge",
            (choice.left() - warning.left()).abs() < 1.0,
            format!(
                "The first choice starts at the warning text: text {:.0}, choice {:.0}",
                warning.left(),
                choice.left()
            ),
        );
    }
    search_library(d, "Cinder Wolf");
    d.click_containing("· ashen-marches/");
    d.expect("Make Cinder Wolf.md in ashen-marches-copy/ a separate entry");
    d.expect_same("The duplicate identity view already selects this file and names both folders.");
    d.assert_state_matches(
        "same-filenames-name-the-folder",
        "manifest-only-hand-repair-keeps-both-files",
    );
    let (copied_entries, _) = splat::content::read_directory(&copied).unwrap();
    let count = copied_entries
        .iter()
        .filter(|entry| entry.kind != "package")
        .count();
    let title_before = reader_title_tops(d, "Cinder Wolf");
    d.click(&format!(
        "Make all {count} files in ashen-marches-copy/ separate entries"
    ));
    d.wait_for_text("Undo separate entries");
    d.expect(&format!(
        "{count} files in ashen-marches-copy/ are now separate entries."
    ));
    // Round 25 FF5: the folder that tells two Ashen Marches packages apart ends the row, as it
    // ends the reader's kind line; at every size it is whole, and a cut falls before it.
    for size in [SMALL, DEFAULT, LARGE] {
        let rows = d.at_size(size, |d| {
            let mut rows = Vec::new();
            d.each_painted_text(|text, clip| {
                let words = &text.galley.job.text;
                if words.starts_with("Creature · CR 1")
                    && words.contains("ashen-marches")
                    && clip.intersects(text.visual_bounding_rect())
                {
                    rows.push(words.clone());
                }
            });
            rows
        });
        d.observe(
            "iter-25-L-FF5",
            // The list rows, not the open reader's kind line, which has the folder alone.
            {
                let listed: Vec<_> = rows
                    .iter()
                    .filter(|row| row.as_str() != "Creature · CR 1 · ashen-marches/")
                    .collect();
                listed.len() == 2
                    && listed.iter().any(|row| row.ends_with(" · ashen-marches/"))
                    && listed
                        .iter()
                        .any(|row| row.ends_with(" · ashen-marches-copy/"))
            },
            format!("At {size:?} each row ends with its whole folder: {rows:?}"),
        );
    }
    d.state("copied-folder-identities-repaired-together", Sizes::All);
    // Round 23 FE6: the batch result takes the warning's place after the title, so the title
    // does not move when the DM presses the repair.
    let title_after = reader_title_tops(d, "Cinder Wolf");
    d.observe(
        "iter-23-L-FE6",
        title_before.iter().all(Option::is_some) && title_before == title_after,
        format!("The title keeps its place: {title_before:?} → {title_after:?}"),
    );
    // Round 23 FE8: the kind line under the owner line adds only the folder that tells the
    // same-name packages apart, not the package name again.
    d.observe(
        "iter-23-L-FE8",
        d.painted_rect_in("Creature · CR 1 · ashen-marches/", egui::Rect::EVERYTHING)
            .is_some()
            && reader_title(d, "Creature · CR 1 · ashen-marches/").is_some()
            && reader_title(d, "Creature · CR 1 · Ashen Marches · ashen-marches/").is_none(),
        "The reader's kind line names the folder without the package name",
    );
    d.click("Undo separate entries");
    d.wait_for_text("Make Cinder Wolf.md in ashen-marches-copy/ a separate entry");
    d.expect_same(
        "Undo restores the same duplicate identities and folder-specific repair actions.",
    );
    d.assert_state_matches(
        "copied-folder-identities-undone",
        "manifest-only-hand-repair-keeps-both-files",
    );
    d.click(&format!(
        "Make all {count} files in ashen-marches-copy/ separate entries"
    ));
    let batch_words = format!("{count} files in ashen-marches-copy/ are now separate entries.");
    d.wait_for_text("Undo separate entries");
    d.expect(&batch_words);
    let wolf = copied_entries
        .iter()
        .find(|entry| entry.name == "Cinder Wolf")
        .unwrap();
    fs::copy(
        d.root.join("modules/ashen-marches").join(&wolf.path),
        copied.join(&wolf.path),
    )
    .unwrap();
    // The reported fixture: the original's bytes over one repaired file, and the batch's other files.
    let on_disk = |copied: &std::path::Path| {
        copied_entries
            .iter()
            .map(|entry| fs::read(copied.join(&entry.path)).unwrap())
            .collect::<Vec<_>>()
    };
    let outside = on_disk(&copied);
    assert_eq!(
        fs::read(copied.join(&wolf.path)).unwrap(),
        fs::read(d.root.join("modules/ashen-marches").join(&wolf.path)).unwrap()
    );
    d.click("Packages");
    d.click("Reload from disk");
    search_library(d, "Cinder Wolf");
    d.click_containing("· ashen-marches/");
    d.expect("Make Cinder Wolf.md in ashen-marches-copy/ a separate entry");
    // AD1: the outside copy ended the batch result, so the message and its Undo go. The reload
    // writes nothing, and the duplicate repair choices stay.
    d.observe(
        "iter-14-E-AD1",
        !d.has("Undo separate entries")
            && !d.visible_text(&batch_words)
            && d.has("Make Cinder Wolf.md in ashen-marches-copy/ a separate entry")
            && on_disk(&copied) == outside,
        "After an outside copy over a repaired file, the batch result and Undo separate entries are gone, the choices stay and no file changed",
    );
    // Round 24 AC2, SC1: the open duplicate's kind line names its folder, as its row does.
    d.observe(
        "iter-24-L-AC2",
        d.visible_text("Creature · CR 1 · ashen-marches/"),
        "The duplicate reader's kind line keeps the folder that tells the two files apart",
    );
    d.state("one-entry-copied-between-registered-packages", Sizes::All);
    go_encounters(d);
    if d.has("Back to encounters") {
        d.click("Back to encounters");
    }
    open_saved_encounter(d, RUN);
    d.expect("Cinder Wolf");
    d.expect("Ember Cultist");
    d.state(
        "manifest-only-hand-repair-keeps-original-roster",
        Sizes::All,
    );
}

/// What the World reader says about an unread Part of list whose object value is on line 9.
/// Round 25 FD4: the object is longer than 40 characters, so the warning quotes its first and
/// last 12 characters around `…`.
fn unread_reason(groups: &str) -> String {
    let value: Vec<char> = format!(
        "{{id:{}",
        groups
            .lines()
            .find_map(|line| line.strip_prefix("- {id:"))
            .expect("fixture: an object group")
    )
    .chars()
    .collect();
    assert!(value.len() > 40, "fixture: a long object group");
    let start: String = value[..12].iter().collect();
    let end: String = value[value.len() - 12..].iter().collect();
    format!(
        "Line 9: Part of “{start}…{end}” was not read. Write each group as a link, such as [[Arena Guild]]."
    )
}

/// The same sentence with the object whole: the unread warning's hover and accessible
/// description (round 25 FD4 review).
fn unread_whole(groups: &str) -> String {
    let value = groups
        .lines()
        .find_map(|line| line.strip_prefix("- {id:"))
        .expect("fixture: an object group");
    format!(
        "Line 9: Part of “{{id:{value}” was not read. Write each group as a link, such as [[Arena Guild]]."
    )
}

/// Round 25 FD4 review: the cut warning keeps its whole sentence in its hover and in its
/// accessible description. Observes each apart, under `where_`.
fn whole_value_kept(d: &mut Driver<'_>, reason: &str, whole: &str, where_: &str) {
    assert_ne!(reason, whole, "fixture: the warning is cut");
    let described = d.descriptions();
    d.observe(
        "iter-25-D-FD4-description",
        described.iter().any(|text| text == whole),
        format!("{where_}: the warning's accessible description is the whole sentence; descriptions {described:?}"),
    );
    // The warning's own words are painted; resting on them paints the whole sentence once more.
    let start: String = reason.chars().take(30).collect();
    let hovered = d.painted_tooltip_shows(&start, whole);
    d.observe(
        "iter-25-D-FD4-hover",
        hovered,
        format!("{where_}: hovering the warning shows the whole sentence {whole:?}"),
    );
}

pub fn part_of_safety(d: &mut Driver<'_>) {
    flat_entry(
        d,
        "Factions/Added Guild.md",
        "Added Guild",
        "splat_type: faction",
        "A new group.\n",
    );
    flat_entry(
        d,
        "Factions/Outside Guild.md",
        "Outside Guild",
        "splat_type: faction",
        "A group added in Obsidian.\n",
    );
    let path = root(d).join("NPCs/Safe Rook.md");
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    let id = crate::hermetic::new_uuid();
    let readable = format!(
        "---\nsplat_schema: 2\nsplat_id: {id}\nsplat_kind: world\nsplat_name: Safe Rook\nsplat_type: npc\nsplat_part_of: # Keep the stored groups.\n- \"[[Zarel|the Archdevil]]\" # Keep the alias.\nsplat_status: alive\n---\n\n# Safe Rook\n\nKeeps the gate.\n"
    );
    let unread_groups = format!(
        "splat_part_of: # Keep the stored groups.\n- '[[Zarel|the Archdevil]]'\n- {{id: {}, link: '[[Lost Guild|the lost]]'}}\n",
        crate::hermetic::new_uuid()
    );
    let unread = readable.replace("splat_part_of: # Keep the stored groups.\n- \"[[Zarel|the Archdevil]]\" # Keep the alias.\n", &unread_groups);
    fs::write(&path, &readable).unwrap();
    reload(d);
    d.click(NAV_WORLD);
    d.click_containing("Safe Rook · ");
    d.expect("Change Part of");
    let missing = format!("{} the Archdevil · Not found", design::icons::WARN);
    for size in [SMALL, DEFAULT, LARGE] {
        let whole = d.at_size(size, |d| {
            for name in [
                "Part of",
                "Change Part of",
                "Change Stat block",
                "Change Status",
            ] {
                d.expect(name);
            }
            // A split value names only the group, so find it by its start.
            let Some(value) = d
                .nodes()
                .into_iter()
                .find(|node| {
                    node.name
                        .starts_with(&missing[..missing.find(" · ").unwrap()])
                })
                .map(|node| node.rect)
            else {
                return (false, "no missing group value".to_owned());
            };
            let label = d.place_of("Part of").unwrap();
            let change = d.place_of("Change Part of").unwrap();
            // Neighbour: every Change keeps one column at the widest value plus an L gap.
            let column = ["Change Stat block", "Change Status"]
                .iter()
                .all(|name| (d.place_of(name).unwrap().left() - change.left()).abs() < 0.5);
            let lines = d.painted_lines_in(value);
            (
                lines == [missing.clone()]
                    && label.height() > 0.0
                    && change.left() - value.right() >= design::L - 0.5
                    && column,
                format!("lines {lines:?}, value {value:?}, change {change:?}, column {column}"),
            )
        });
        d.observe(
            "iter-10-D-AD3",
            whole.0,
            format!("A missing group and Not found stay on one line, with one Change column, at {size:?}: {}", whole.1),
        );
    }
    d.observe_both(
        "iter-11-D-FD1",
        "A missing group warns on the list row and counts its note once in the header",
        |d| {
            let rows: Vec<_> = d
                .nodes()
                .into_iter()
                .filter(|node| node.name.starts_with("Safe Rook · "))
                .collect();
            // Neighbour: the reader's own warning stays whole.
            !rows.is_empty()
                && rows
                    .iter()
                    .all(|row| row.name.contains("Group not found") && row.in_view())
                && d.in_view_containing("1 note needs fixing")
                && d.has_containing("the Archdevil · Not found")
        },
    );
    // Round 26 FE5: the set Status value `alive` is `Body` in the text colour, as a set link is.
    // Neighbour: the `not set` placeholder in a row above stays muted `Small`.
    let status = |d: &Driver<'_>, words: &str| {
        let row = d.place_of("Change Status")?;
        let area = egui::Rect::from_x_y_ranges(
            0.0..=row.left(),
            row.top() - 4.0 * design::Density::Scan.row()..=row.bottom(),
        );
        Some((d.painted_size(area, words)?, d.painted_color(words)?))
    };
    let (alive, unset) = (status(d, "alive"), status(d, "not set"));
    d.observe(
        "iter-26-D-FE5",
        alive == Some((design::Type::Body.size(), design::colors().text))
            && unset == Some((design::Type::Small.size(), design::colors().muted)),
        format!("Status alive {alive:?} is Body text; not set {unset:?} stays Small muted"),
    );
    d.state("missing-group-on-one-line", Sizes::All);
    // From another note, the header count's menu opens the note that needs fixing.
    d.click_containing("Added Guild · ");
    d.expect("Edit Added Guild");
    assert!(!d.has("Edit Safe Rook"), "fixture: another note is open");
    d.click("1 note needs fixing");
    d.click("Safe Rook");
    d.expect("Edit Safe Rook");
    d.observe(
        "iter-11-D-FD1-menu",
        d.in_view("Edit Safe Rook") && !d.has("Edit Added Guild"),
        "The header count's repair menu opens the note that needs fixing",
    );
    d.click("Change Part of");
    d.type_into("Find a group", "Added Guild");
    fs::write(&path, &unread).unwrap();
    d.expect("Added Guild");
    d.click("Added Guild");
    assert_eq!(
        fs::read(&path).unwrap(),
        unread.as_bytes(),
        "iter-9-D-SD1-raw: Read picker preserves the reported disk fixture"
    );
    let reason = &unread_reason(&unread_groups);
    d.expect(reason);
    // Round 25 FD4: the warning cuts the long value, so its id is not on screen; the sentence
    // with the value whole stays the warning's hover.
    let id = unread_groups
        .split("{id: ")
        .nth(1)
        .and_then(|rest| rest.split(',').next())
        .expect("fixture: the group's id");
    d.observe(
        "iter-25-D-FD4",
        d.has(reason) && !d.has_containing(id) && !d.in_view_containing(id),
        format!("The unread warning quotes the value's start and end, not the id {id}"),
    );
    let whole = &unread_whole(&unread_groups);
    whole_value_kept(d, reason, whole, "World");
    d.observe_both(
        "iter-9-D-SD1-reader",
        "The refused write closes the picker and gives the source line and repair link",
        |d| {
            d.expect(reason);
            d.expect("Edit in Obsidian");
            !d.has("Find a group")
        },
    );
    // The click settles until the index has read the note again.
    d.settle();
    d.observe_both(
        "iter-10-D-SD2-FD1",
        "After a refused write, Part of says not read with Change blocked, and the list and header count the fault",
        |d| {
            d.expect("Change Part of");
            let row = d.place_of("Change Part of").unwrap();
            let label = d.place_of("Part of").unwrap();
            d.is_disabled("Change Part of")
                && d
                    .painted_in(egui::Rect::from_min_max(
                        egui::pos2(label.left(), row.top()),
                        row.max,
                    ))
                    .contains("not read")
                && !d.has_containing("the Archdevil · Not found")
                && d.in_view_containing("1 value not read")
                && d.in_view_containing("1 note needs fixing")
        },
    );
    d.observe_both(
        "iter-10-D-FD3-FD4",
        "The refusal is a warning below the file line, which keeps the one Edit in Obsidian link",
        |d| {
            d.expect(reason);
            let links: Vec<_> = d
                .nodes()
                .into_iter()
                .filter(|node| node.name == "Edit in Obsidian" && node.role == Role::Link)
                .collect();
            let file = d.place_of("Show file").unwrap();
            let message = d.place_of(reason).unwrap();
            // Warning tone, as in Edit: no error icon anywhere on the reader.
            links.len() == 1 && message.top() >= file.bottom() && !d.in_view(design::icons::ERROR)
        },
    );
    for size in [SMALL, DEFAULT, LARGE] {
        let (inside, detail) = d.at_size(size, |d| {
            d.expect(reason);
            let warning = d.place_of(reason).unwrap();
            // The painted words too: the label's rect and its words both end at the measure.
            let words = d.painted_right(reason).unwrap();
            let pane = d.place_of("Resize World notes column").unwrap().right();
            let measure = pane + design::S + design::READER_MEASURE;
            (
                warning.right() <= measure + 1.0 && words <= measure + 1.0,
                format!(
                    "warning ends at {}, its words at {words}, measure at {measure}",
                    warning.right()
                ),
            )
        });
        d.observe(
            "iter-15-D-FD6-read",
            inside,
            format!("The unread warning wraps at the reader measure at {size:?}: {detail}"),
        );
    }
    d.state("read-picker-unread-disk-kept", Sizes::All);

    fs::write(&path, &readable).unwrap();
    reload(d);
    d.observe(
        "iter-9-D-repaired-reader",
        !d.has(reason),
        "Repair clears the old refused-write reason",
    );

    // A readable list: the draft adds a group while Obsidian adds another.
    let conflict = "Safe Rook changed on disk. Your draft is safe until you choose.";
    let zarel = "- \"[[Zarel|the Archdevil]]\" # Keep the alias.\n";
    let outside_line = "- '[[Outside Guild]]' # Added in Obsidian.\n";
    let outside = readable.replace(zarel, &format!("{zarel}{outside_line}"));
    assert_ne!(outside, readable, "fixture: Obsidian adds one group line");
    d.click("Edit Safe Rook");
    d.observe_both(
        "iter-10-D-SD6",
        "A clean World form offers quiet Cancel, not Discard",
        |d| d.has("Cancel") && !d.has("Discard") && d.visible_text("No changes"),
    );
    d.click("Change Part of");
    d.type_into("Find a group", "Added Guild");
    d.click("Added Guild");
    d.observe(
        "iter-10-D-SD6-dirty",
        d.has("Discard") && !d.has("Cancel"),
        "A dirty World form says Discard",
    );
    fs::write(&path, &outside).unwrap();
    d.click("Save changes");
    d.expect(conflict);
    let added = "Overwrite adds Added Guild and keeps Outside Guild from the note.";
    d.observe_both(
        "iter-10-D-SD1-preview",
        "Before the choice, the comparison names the group Overwrite adds and the outside group it keeps",
        |d| d.in_view(added) && d.in_view("Overwrite with my draft"),
    );
    d.expect_moves("The disk-conflict choice takes the edit form's place until the DM chooses.");
    d.state("readable-group-conflict-names-each-change", Sizes::All);
    d.click("Overwrite with my draft");
    d.wait_for_text("Changes saved.");
    assert_eq!(
        fs::read(&path).unwrap(),
        outside
            .replace(
                outside_line,
                &format!("{outside_line}- '[[Added Guild]]'\n")
            )
            .as_bytes(),
        "iter-10-D-SD1-add-raw: Overwrite saves A, B and C; A and C keep their raw text"
    );
    d.click("Undo changes");
    assert_eq!(fs::read(&path).unwrap(), outside.as_bytes());
    // The draft removes A while Obsidian adds C.
    fs::write(&path, &readable).unwrap();
    reload(d);
    d.click("Edit Safe Rook");
    d.click("Change Part of");
    action_at_text_edge(
        d,
        "iter-17-D-remove-group",
        "Remove the Archdevil",
        "Resize World notes column",
        &[SMALL, DEFAULT, LARGE],
        |d| d.place_of("Find a group").map(|field| field.left()),
    );
    // The picker's Cancel starts its line for a group, after the Remove actions.
    for size in [SMALL, DEFAULT, LARGE] {
        let (cancel, field, splitter) = d.at_size(size, |d| {
            let remove = d.place_of("Remove the Archdevil");
            (
                remove.and_then(|remove| {
                    d.nodes()
                        .into_iter()
                        .filter(|node| node.name == "Cancel" && node.rect.top() >= remove.bottom())
                        .map(|node| node.rect)
                        .min_by(|a, b| a.top().total_cmp(&b.top()))
                }),
                d.place_of("Find a group"),
                d.place_of("Resize World notes column"),
            )
        });
        d.observe(
            "iter-17-D-picker-cancel",
            cancel.zip(field).zip(splitter).is_some_and(|((cancel, field), splitter)| {
                (cancel.left() + design::S - field.left()).abs() <= 1.0
                    && cancel.left() >= splitter.right()
            }),
            format!(
                "At {size:?}, the picker's Cancel {cancel:?} starts at the field's edge {field:?}, right of the splitter {splitter:?}"
            ),
        );
    }
    d.click("Remove the Archdevil");
    fs::write(&path, &outside).unwrap();
    d.click("Save changes");
    d.expect(conflict);
    let removed = "Overwrite removes the Archdevil and keeps Outside Guild from the note.";
    d.observe(
        "iter-10-D-SD1-preview-remove",
        d.in_view(removed),
        "Before the choice, the comparison names the group Overwrite removes",
    );
    d.click("Overwrite with my draft");
    d.wait_for_text("Changes saved.");
    assert_eq!(
        fs::read(&path).unwrap(),
        outside.replace(zarel, "").as_bytes(),
        "iter-10-D-SD1-remove-raw: Overwrite saves C alone, as raw text"
    );
    d.click("Undo changes");
    assert_eq!(fs::read(&path).unwrap(), outside.as_bytes());
    // The draft removes A while Obsidian respells it. Overwrite still removes it.
    let part_of_line = "splat_part_of: # Keep the stored groups.\n";
    let respelled_line = "- \"[[Zarel]]\" # Respelled in Obsidian.\n";
    let respelled = readable.replace(zarel, respelled_line);
    fs::write(&path, &readable).unwrap();
    reload(d);
    d.click("Edit Safe Rook");
    d.click("Change Part of");
    d.click("Remove the Archdevil");
    fs::write(&path, &respelled).unwrap();
    d.click("Save changes");
    d.expect(conflict);
    d.observe(
        "iter-11-D-SD1-preview-respelled",
        d.in_view("Overwrite removes Zarel."),
        "Before the choice, the comparison names the respelled group Overwrite removes",
    );
    d.click("Overwrite with my draft");
    d.wait_for_text("Changes saved.");
    assert_eq!(
        fs::read(&path).unwrap(),
        respelled
            .replace(&format!("{part_of_line}{respelled_line}"), "")
            .as_bytes(),
        "iter-11-D-SD1-respelled-raw: Overwrite removes the group Obsidian respelled"
    );
    d.click("Undo changes");
    assert_eq!(fs::read(&path).unwrap(), respelled.as_bytes());
    // The draft adds B while Obsidian adds B in another spelling: one line, the note's own.
    let other_line = "- \"[[Factions/Added Guild|Guild]]\" # Added in Obsidian.\n";
    let other = readable.replace(zarel, &format!("{zarel}{other_line}"));
    fs::write(&path, &readable).unwrap();
    reload(d);
    d.click("Edit Safe Rook");
    d.click("Change Part of");
    d.type_into("Find a group", "Added Guild");
    d.click("Added Guild");
    fs::write(&path, &other).unwrap();
    d.click("Save changes");
    d.expect(conflict);
    d.observe(
        "iter-11-D-SD1-preview-same",
        !d.has_containing("Overwrite adds"),
        "Before the choice, the comparison does not offer to add a group the note has",
    );
    // An old time on the file shows whether Overwrite wrote it, even with the same bytes.
    let old = std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1_000_000);
    fs::File::options()
        .write(true)
        .open(&path)
        .unwrap()
        .set_modified(old)
        .unwrap();
    d.click("Overwrite with my draft");
    d.wait_for_text(ALREADY_SAVED);
    assert_eq!(
        fs::read(&path).unwrap(),
        other.as_bytes(),
        "iter-11-D-SD1-same-raw: one line for the group, in the note's own spelling"
    );
    assert_eq!(
        fs::metadata(&path).unwrap().modified().unwrap(),
        old,
        "iter-12-D-AD2-no-write: Overwrite with nothing to change does not write the note"
    );
    d.observe_both(
        "iter-12-D-AD2",
        "Overwrite with nothing to change says so, offers no Undo and closes the form",
        |d| {
            d.expect(ALREADY_SAVED);
            // Neighbour: the note's groups still show in the reader.
            !d.has("Undo changes")
                && !d.has("Changes saved.")
                && !d.has("Save changes")
                && !d.has("Overwrite with my draft")
                && d.has_containing("Added Guild")
        },
    );
    // FD1, round 26: `Your changes are already in the note.` is a World result too, in the
    // pane's footer below the value rows, at every captured size.
    for size in [SMALL, DEFAULT, LARGE] {
        let (placed, detail) = d.at_size(size, |d| {
            d.expect("Change Part of");
            let (below, detail) = result_below_file_line(
                d,
                ALREADY_SAVED,
                "Safe Rook.md",
                design::colors().result.info,
            );
            let message = d.painted_rect_in(ALREADY_SAVED, reader_pane(d));
            let row = d.place_of("Change Part of");
            (
                below
                    && message
                        .zip(row)
                        .is_some_and(|(message, row)| message.top() >= row.bottom()),
                format!("{detail}; the Part of row at {row:?}"),
            )
        });
        d.observe(
            "iter-17-D-FD1-kept",
            placed,
            format!(
                "At {size:?}, the kept-changes message sits in the footer below the rows: {detail}"
            ),
        );
    }
    // Round 24 FG5: with Part of on two lines, Change keeps the column at the widest drawn line
    // of any value plus `L`, not at the width of every group on one line.
    for size in [SMALL, DEFAULT, LARGE] {
        let (column, detail) = d.at_size(size, |d| {
            let changes: Vec<_> = [
                "Change Stat block",
                "Change In",
                "Change Part of",
                "Change Status",
            ]
            .into_iter()
            .filter_map(|name| d.place_of(name))
            .collect();
            let Some(change) = d.place_of("Change Part of") else {
                return (false, "no Change Part of".to_owned());
            };
            let top = changes
                .iter()
                .map(|rect| rect.top())
                .fold(f32::INFINITY, f32::min);
            let bottom = changes
                .iter()
                .map(|rect| rect.bottom())
                .fold(0.0_f32, f32::max);
            let area = egui::Rect::from_min_max(
                egui::pos2(reader_pane(d).left(), top - 1.0),
                egui::pos2(change.left(), bottom + 1.0),
            );
            let mut widest = 0.0_f32;
            let mut lines = 0;
            d.each_painted_text(|text, clip| {
                if ["Stat block", "In", "Part of", "Status"]
                    .contains(&text.galley.job.text.as_str())
                {
                    return;
                }
                for row in &text.galley.rows {
                    let rect = row
                        .rect_without_leading_space()
                        .translate(text.pos.to_vec2());
                    if area.contains(rect.center()) && clip.intersects(rect) {
                        widest = widest.max(rect.right());
                        lines += 1;
                    }
                }
            });
            (
                lines >= 3 && (change.left() - widest - design::L).abs() <= 2.0,
                format!(
                    "{lines} value lines end by {widest}, Change at {}",
                    change.left()
                ),
            )
        });
        d.observe(
            "iter-24-D-FG5",
            column,
            format!("Change sits L after the widest drawn value at {size:?}: {detail}"),
        );
    }
    d.state("overwrite-already-in-note", Sizes::All);
    fs::write(&path, &readable).unwrap();
    reload(d);
    d.observe(
        "iter-12-D-AD2-ends",
        !d.has(ALREADY_SAVED),
        "The message goes when the note changes on disk",
    );
    d.click("Edit Safe Rook");
    d.replace_text_in("Status", "ready");
    d.click("Change Part of");
    d.type_into("Find a group", "Added Guild");
    d.click("Added Guild");
    let safe_rook_rows = |d: &Driver<'_>| -> Vec<String> {
        d.nodes()
            .into_iter()
            .filter(|node| node.name.starts_with("Safe Rook · ") && node.in_view())
            .map(|node| node.name)
            .collect()
    };
    let before = safe_rook_rows(d);
    d.observe(
        "iter-13-D-FD2-before",
        !before.is_empty() && before.iter().all(|row| row.contains("Group not found")),
        format!("fixture: before the outside change, the row names the missing group: {before:?}"),
    );
    fs::write(&path, &unread).unwrap();
    d.click("Save changes");
    d.expect("Safe Rook changed on disk. Your draft is safe until you choose.");
    // The save reads the note again, as a refused Part of write does.
    d.settle();
    d.observe_both(
        "iter-13-D-FD2",
        "A save conflict refreshes the note's list row and header count; the choice still waits",
        |d| {
            d.expect("Overwrite with my draft");
            let rows = safe_rook_rows(d);
            // Neighbour: the header still counts the note once.
            !rows.is_empty()
                && rows
                    .iter()
                    .all(|row| row.contains("1 value not read") && !row.contains("Group not found"))
                && d.in_view_containing("1 note needs fixing")
                && d.in_view("Reload and lose my draft")
        },
    );
    d.expect_moves("The disk-conflict choice takes the edit form's place until the DM chooses.");
    d.state_prepared("group-draft-disk-conflict", Sizes::All, |d| {
        choices_at_text(
            d,
            "iter-15-E-AD3-world-edit",
            "Safe Rook changed on disk",
            "Overwrite with my draft",
            "Reload and lose my draft",
            None,
        );
    });
    d.click("Overwrite with my draft");
    d.wait_for_text("Changes saved.");
    let saved = fs::read_to_string(&path).unwrap();
    assert!(
        saved.contains(&unread_groups),
        "iter-9-D-overwrite-raw: keep the entire unread Part of block"
    );
    assert!(saved.contains("splat_status: ready"));
    d.expect(reason);
    d.observe_both(
        "iter-9-D-overwrite",
        "Overwrite saves other fields, keeps disk groups, explains the refusal and closes the form",
        |d| {
            d.expect(reason);
            d.expect("Undo changes");
            !d.has("Save changes")
        },
    );
    // Round 26 FE2 (replaces round 17 FD1's place above the rows): the edit-form save result
    // is in the pane's footer, below the value rows, with the success tone.
    for size in [SMALL, DEFAULT, LARGE] {
        let (placed, detail) = d.at_size(size, |d| {
            d.expect("Change Status");
            // FD5 (round 20): with the success tone.
            let (below, detail) = result_below_file_line(
                d,
                "Changes saved.",
                "Safe Rook.md",
                design::colors().result.success,
            );
            let message = d.painted_rect_in("Changes saved.", reader_pane(d));
            let row = d.place_of("Change Status");
            (
                below
                    && message
                        .zip(row)
                        .is_some_and(|(message, row)| message.top() >= row.bottom()),
                format!("{detail}; status row at {row:?}"),
            )
        });
        d.observe(
            "iter-26-D-FE2-footer",
            placed,
            format!("At {size:?}, `Changes saved.` sits in the footer below the rows: {detail}"),
        );
    }
    let rows = tops_at_sizes(d, "Change Status");
    // The footer checks above assert the change: `Changes saved.` with `Undo changes` and the
    // saved Status. The rows keep their place now, so this state differs from the read picker's
    // only by those words at the pane's foot.
    d.expect_same(
        "Round 26: the save result sits in the footer and the rows keep their place, so the saved \
         note differs from the read picker state only by the footer's Changes saved. with Undo \
         changes and the Status value.",
    );
    d.expect_moves("Overwrite ends the disk-conflict choice, and the note's rows come back.");
    d.state("overwrite-keeps-unread-disk-groups", Sizes::All);
    d.click("Undo changes");
    // The rows keep their place when the result goes with its Undo.
    body_kept(d, "iter-26-D-FE2", "Change Status", &rows, None);
    assert_eq!(fs::read(&path).unwrap(), unread.as_bytes());
    reload(d);
    d.click("Edit Safe Rook");
    d.expect(reason);
    whole_value_kept(d, reason, whole, "World edit form");
    d.observe_both(
        "iter-9-D-FD4",
        "Unread groups name the source line and put a quiet repair link beside it",
        |d| {
            for name in [
                reason,
                "Edit in Obsidian",
                "Change Part of",
                "Change In",
                "Text",
            ] {
                d.expect(name);
            }
            let links: Vec<_> = d
                .nodes()
                .into_iter()
                .filter(|node| node.name == "Edit in Obsidian" && node.role == Role::Link)
                .collect();
            d.is_disabled("Change Part of")
                && !d.is_disabled("Change In")
                && !d.is_disabled("Text")
                && links.len() == 1
                && links.iter().all(|node| node.in_view())
        },
    );
    // Round 23 FA10: Edit shows `not read` in the warning colour, as the reader does.
    d.observe(
        "iter-23-D-FA10",
        d.painted_color("not read") == Some(design::colors().result.warning),
        format!(
            "Edit's Part of `not read` is {:?}",
            d.painted_color("not read")
        ),
    );
    d.state("unread-groups-edit-repair-link", Sizes::All);
    d.click("Cancel");

    // iter-13-D-FD2: the conflict's reload must not cost the Reload choice its Undo.
    fs::write(&path, &readable).unwrap();
    reload(d);
    d.click("Edit Safe Rook");
    d.replace_text_in("Status", "ready");
    let away = readable.replace("splat_status: alive", "splat_status: away");
    assert_ne!(away, readable, "fixture: Obsidian changes the status");
    fs::write(&path, &away).unwrap();
    d.click("Save changes");
    d.expect(conflict);
    d.settle();
    d.click("Reload and lose my draft");
    d.settle();
    d.observe_both(
        "iter-13-D-FD2-reload-undo",
        "After the conflict's reload, Reload shows the disk value and keeps Undo reload",
        |d| {
            d.expect("Undo reload");
            d.in_view("Undo reload") && d.field_text("Status").as_deref() == Some("away")
        },
    );
    d.click("Undo reload");
    d.observe(
        "iter-13-D-FD2-reload-undone",
        d.field_text("Status").as_deref() == Some("ready") && !d.has("Undo reload"),
        "Undo reload restores the draft",
    );
    d.click("Discard");
    assert_eq!(
        fs::read(&path).unwrap(),
        away.as_bytes(),
        "iter-13-D-FD2-no-write"
    );
    // iter-13-D-FD2-second: Obsidian writes again while the choice is open. Reload must not drop
    // Undo reload for a version the DM has not seen.
    fs::write(&path, &readable).unwrap();
    reload(d);
    d.click("Edit Safe Rook");
    d.replace_text_in("Status", "ready");
    fs::write(&path, &away).unwrap();
    d.click("Save changes");
    d.expect(conflict);
    d.settle();
    let gone = readable.replace("splat_status: alive", "splat_status: gone");
    fs::write(&path, &gone).unwrap();
    d.click("Reload and lose my draft");
    d.settle();
    d.observe_both(
        "iter-13-D-FD2-second",
        "A second outside change after Reload shows the choice again, not a silent reopen",
        |d| {
            d.expect(conflict);
            // Neighbour: the form waits for the choice, so no field shows yet.
            d.in_view(conflict)
                && d.in_view("Reload and lose my draft")
                && d.field_text("Status").is_none()
        },
    );
    d.click("Reload and lose my draft");
    d.settle();
    d.observe(
        "iter-13-D-FD2-second-reload",
        d.in_view("Undo reload") && d.field_text("Status").as_deref() == Some("gone"),
        "The second Reload shows the newest file and keeps Undo reload",
    );
    d.click("Undo reload");
    d.observe(
        "iter-13-D-FD2-second-undone",
        d.field_text("Status").as_deref() == Some("ready") && !d.has("Undo reload"),
        "Undo reload restores the DM's own draft, not the version between",
    );
    d.click("Discard");
    assert_eq!(
        fs::read(&path).unwrap(),
        gone.as_bytes(),
        "iter-13-D-FD2-second-no-write"
    );
    // iter-13-D-FD2-second-overwrite: the same start, then Overwrite at the second choice. The
    // DM's own draft goes in, with Undo, and the reopened choice counts as unsaved work.
    fs::write(&path, &readable).unwrap();
    reload(d);
    d.click("Edit Safe Rook");
    d.replace_text_in("Status", "ready");
    fs::write(&path, &away).unwrap();
    d.click("Save changes");
    d.expect(conflict);
    d.settle();
    fs::write(&path, &gone).unwrap();
    d.click("Reload and lose my draft");
    d.settle();
    d.expect(conflict);
    let unsaved = d.app_mut().any_unsaved_edits();
    d.observe(
        "iter-13-D-FD2-second-unsaved",
        unsaved && d.in_view("Overwrite with my draft"),
        "While the second choice shows, the DM's draft counts as unsaved for the close guard",
    );
    d.click("Overwrite with my draft");
    d.settle();
    let ready_over_gone = gone.replace("splat_status: gone", "splat_status: ready");
    assert_ne!(ready_over_gone, gone, "fixture: the draft's status differs");
    assert_eq!(
        fs::read(&path).unwrap(),
        ready_over_gone.as_bytes(),
        "iter-13-D-FD2-second-overwrite: Overwrite writes the DM's ready over gone"
    );
    d.expect("Undo changes");
    d.click("Undo changes");
    assert_eq!(
        fs::read(&path).unwrap(),
        gone.as_bytes(),
        "iter-13-D-FD2-second-overwrite-undo: Undo restores gone"
    );
    // iter-13-D-FD2-second-overwrite-outside: disk1 also changes another field. Overwrite at the
    // second choice changes only what the DM changed; disk2 keeps the rest.
    let status_line = "splat_status: alive\n";
    let paragraph = "\nA paragraph from Obsidian.\n";
    for (case, base, outside) in [
        (
            "in",
            readable.replace(
                status_line,
                &format!("splat_in: '[[Old Gate]]'\n{status_line}"),
            ),
            readable.replace(
                status_line,
                &format!("splat_in: '[[New Gate]]'\n{status_line}"),
            ),
        ),
        ("text", readable.clone(), format!("{readable}{paragraph}")),
    ] {
        let disk1 = outside.replace(status_line, "splat_status: away\n");
        let disk2 = outside.replace(status_line, "splat_status: gone\n");
        let expected = outside.replace(status_line, "splat_status: ready\n");
        assert!(
            base != outside && disk1 != outside && disk2 != outside && expected != outside,
            "fixture {case}: each version differs"
        );
        fs::write(&path, &base).unwrap();
        reload(d);
        d.click("Edit Safe Rook");
        d.replace_text_in("Status", "ready");
        fs::write(&path, &disk1).unwrap();
        d.click("Save changes");
        d.expect(conflict);
        d.settle();
        fs::write(&path, &disk2).unwrap();
        d.click("Reload and lose my draft");
        d.settle();
        d.expect(conflict);
        d.click("Overwrite with my draft");
        d.settle();
        assert_eq!(
            String::from_utf8(fs::read(&path).unwrap()).unwrap(),
            expected,
            "iter-13-D-FD2-second-overwrite-{case}: only the DM's status goes over disk2"
        );
        d.expect("Undo changes");
        d.click("Undo changes");
        assert_eq!(
            fs::read(&path).unwrap(),
            disk2.as_bytes(),
            "iter-13-D-FD2-second-overwrite-{case}-undo: Undo restores disk2"
        );
    }

    // Compact Read has no group Change. Use its real Edit → Change → Save route.
    fs::write(&path, &readable).unwrap();
    reload(d);
    go_encounters(d);
    resume_run_from_cold_start(d);
    look_up(d, "Safe Rook");
    d.expect("Edit Safe Rook");
    d.click("Edit Safe Rook");
    d.expect("Change Part of");
    d.click("Change Part of");
    d.type_into("Find a group", "Added Guild");
    fs::write(&path, &unread).unwrap();
    d.expect("Added Guild");
    d.click("Added Guild");
    d.click("Save changes");
    d.expect("Safe Rook changed on disk. Your draft is safe until you choose.");
    d.click("Overwrite with my draft");
    d.settle();
    assert_eq!(
        fs::read(&path).unwrap(),
        unread.as_bytes(),
        "iter-9-D-AT1-raw: live group change preserves every byte and writes no Log"
    );
    // The group change did not go in, so the form does not say it is in the note.
    d.observe(
        "iter-12-D-AD2-refused",
        !d.has(ALREADY_SAVED) && !d.has("Undo changes"),
        "A refused group change writes nothing, gives no Undo and does not say it is in the note",
    );
    d.expect(reason);
    whole_value_kept(d, reason, whole, "Lookup");
    d.observe_both("iter-9-D-AT1", "The live group change keeps unread disk groups, closes the form and keeps Next turn visible", |d| {
        d.expect(reason);
        d.expect(NEXT_TURN);
        !d.has("Find a group") && !d.has("Save changes")
    });
    d.state("live-picker-unread-disk-kept", Sizes::All);
}

pub fn empty_links(d: &mut Driver<'_>) {
    d.click(NAV_WORLD);
    for (label, value) in [("null", "null"), ("empty", "''"), ("whitespace", "'   '")] {
        let name = format!("Optional {label}");
        let path = format!("NPCs/{name}.md");
        flat_entry(
            d,
            &path,
            &name,
            &format!("splat_type: npc\nsplat_in: {value}\nsplat_stat_block: {value}"),
            "Keeps the gate.\n",
        );
        let before = fs::read(root(d).join(&path)).unwrap();
        reload(d);
        d.click_containing(&format!("{name} · "));
        d.observe_both(
            "iter-9-D-empty-reader",
            "An unset place stays hidden; an NPC keeps its empty Stat block row",
            |d| {
                d.expect("Change Stat block");
                !d.has("Change In") && !d.has_containing("Place not found")
            },
        );
        d.click(&format!("Edit {name}"));
        for size in [SMALL, DEFAULT, LARGE] {
            let empty = d.at_size(size, |d| {
                ["In", "Stat block"].iter().all(|label| {
                    let name = format!("Change {label}");
                    d.expect(&name);
                    let change = d.place_of(&name).unwrap();
                    let label = d.place_of(label).unwrap();
                    d.painted_in(egui::Rect::from_min_max(
                        egui::pos2(label.left(), change.top()),
                        change.max,
                    ))
                    .contains("not set")
                })
            });
            d.observe(
                "iter-9-D-SD3-AD3",
                empty,
                format!("Both {label} optional links say not set at {size:?}"),
            );
        }
        // Round 23 FF6: Edit keeps the Read order: Stat block, In, Part of, then Status. Must
        // stay true: Text still follows the fields.
        let order = [
            "Change Stat block",
            "Change In",
            "Change Part of",
            "Status",
            "Text",
        ]
        .map(|name| d.place_of(name).map(|rect| rect.top()));
        d.observe(
            "iter-23-D-FF6",
            order.iter().all(Option::is_some) && order.windows(2).all(|pair| pair[0] < pair[1]),
            format!("Edit fields top to bottom: {order:?}"),
        );
        d.replace_text_in("Status", "ready");
        d.state(&format!("{label}-links-in-edit"), Sizes::All);
        d.click("Save changes");
        d.wait_for_text("Changes saved.");
        let saved = fs::read_to_string(root(d).join(&path)).unwrap();
        assert!(
            saved.contains(&format!("splat_in: {value}\n"))
                && saved.contains(&format!("splat_stat_block: {value}\n")),
            "iter-9-D-empty-bytes: unrelated save keeps both raw optional fields"
        );
        d.click("Undo changes");
        assert_eq!(fs::read(root(d).join(&path)).unwrap(), before);
    }
}

/// Round 20 SD1: a Library file that stops reading as text while it is open, in a package and
/// in a campaign, with and without a draft. Its reader names the cause, offers Show file and
/// Try again, and no editor link or source repair. The file's bytes never change.
pub fn library_file_fault(d: &mut Driver<'_>) {
    // Round 24 FE5: below the file line, the error says `this file`; the file line names it.
    const HEADLINE: &str = "Splat cannot read this file as text.";
    let path = d
        .root
        .join("modules/ashen-marches/Creatures/Cinder Wolf.md");
    let text = fs::read(&path).unwrap();
    // One byte that no UTF-8 text holds, inside the stat block: the rest of the file is kept.
    let mut not_text = text.clone();
    let at = text.windows(4).position(|w| w == b"Bite").unwrap();
    not_text[at] = 0xff;
    let refocus = |d: &mut Driver<'_>| {
        d.focus_window(false);
        d.focus_window(true);
    };
    go_library(d);
    search_library(d, "Cinder Wolf");
    d.click_starting("Cinder Wolf · ");
    d.wait_for_text("Bite");
    assert!(
        !d.is_disabled(EDIT_MARKDOWN),
        "a readable file opens in Edit Markdown"
    );
    // A package entry with no ID is not one the campaign uses: show the package's own entries.
    d.click("What the Library shows: Can use");
    d.click("Ashen Marches");
    d.wait_for_text("Bite");

    // Action-time failure with no draft: the file stops reading after it was opened.
    fs::write(&path, &not_text).unwrap();
    refocus(d);
    d.wait_for_text(HEADLINE);
    // Round 25 FB3, SB1: the recovery steps are prose and keep the reader measure, with a
    // draft and without one.
    fn recovery_measure(d: &mut Driver<'_>, id: &str) {
        let recovery =
            d.painted_rect_in("Splat reads notes as UTF-8 text.", egui::Rect::EVERYTHING);
        d.observe(
            id,
            recovery.is_some_and(|rect| rect.width() <= design::READER_MEASURE + 1.0),
            format!(
                "The recovery paragraph wraps at {} px or less: {recovery:?}",
                design::READER_MEASURE
            ),
        );
    }
    d.state_prepared("package-file-not-text", Sizes::All, |d| {
        file_fault_reader(
            d,
            "iter20-E-SD1-package",
            HEADLINE,
            "Creatures/Cinder Wolf.md",
            false,
        );
        recovery_measure(d, "iter-25-L-SB1-no-draft");
        // Round 25 family: a message action without an icon is a bordered secondary button.
        d.observe(
            "iter-25-L-try-again-bordered",
            d.control_frame("Try again")
                .is_some_and(|frame| frame.stroke.width > 0.0),
            "Try again under the fault is bordered",
        );
    });
    let unread_count = files_to_fix(d);
    assert_eq!(
        fs::read(&path).unwrap(),
        not_text,
        "the file keeps its bytes"
    );

    // Try again while the file is still not text: the same fault, not a repair.
    d.click("Try again");
    d.wait_for_text(HEADLINE);
    d.observe(
        "iter20-E-SD1-try-again-still",
        d.has("Try again") && !d.has("Fix in Edit Markdown") && !d.visible_text("Bite"),
        "Try again on a file that is still not text keeps the fault",
    );
    // The file reads again outside Splat. Nothing notices until Try again.
    fs::write(&path, &text).unwrap();
    d.settle();
    let waiting = d.visible_text(HEADLINE);
    d.click("Try again");
    d.wait_for_text("Bite");
    d.observe(
        "iter20-E-SD1-try-again-reads",
        waiting && !d.has("Try again") && !d.is_disabled(EDIT_MARKDOWN),
        format!(
            "The fault stays until Try again ({waiting}); then the entry reads, with Edit Markdown"
        ),
    );

    // Action-time failure with a draft: the draft stays and cannot be saved over the file.
    d.click(EDIT_MARKDOWN);
    d.type_into_editor("The draft stays safe.\n");
    let readable_count = files_to_fix(d);
    fs::write(&path, &not_text).unwrap();
    refocus(d);
    let conflict = format!("{HEADLINE} Your draft is safe here until you choose.");
    d.wait_for_text(&conflict);
    // Round 23 FC2: one file, one state. With a draft open, the header counts the file that
    // stopped reading, as it does without a draft, and the owner line says Cannot be read.
    let draft_count = files_to_fix(d);
    d.observe(
        "iter-23-L-FC2",
        unread_count.is_some()
            && draft_count == unread_count
            && draft_count != readable_count
            && d.visible_text("Ashen Marches · Cannot be read")
            && !d.visible_text("Ashen Marches · Chosen for this campaign"),
        format!(
            "The header counts the unread file with a draft ({draft_count:?}) as without one ({unread_count:?}), not as when it read ({readable_count:?}), and the owner line says Cannot be read"
        ),
    );
    d.state_prepared("package-draft-file-not-text", Sizes::All, |d| {
        choices_at_text(
            d,
            "iter20-E-SD1-draft-choices",
            &conflict,
            "Try again",
            "Reload and lose my draft",
            None,
        );
        recovery_measure(d, "iter-25-L-FB3");
        d.observe(
            "iter20-E-SD1-draft-no-save",
            !d.has("Save my draft again")
                && !d.has("Overwrite with my draft")
                && !d.has("Save")
                && !d.has_containing("was deleted on disk")
                && d.has("Technical details"),
            "The draft's choices never save over the unreadable file",
        );
    });
    assert!(d.app_mut().library.dirty(), "the draft stays");
    assert_eq!(fs::read(&path).unwrap(), not_text);
    // The file reads as before again: Try again ends the fault and keeps the draft.
    fs::write(&path, &text).unwrap();
    d.click("Try again");
    d.settle();
    let kept = d.app_mut().library.dirty();
    d.observe(
        "iter20-E-SD1-draft-try-again",
        !d.has_containing("Your draft is safe here") && kept,
        "Try again on a file that reads again ends the fault and keeps the draft",
    );
    // The count and the owner line end with the fault.
    let back = files_to_fix(d);
    d.observe(
        "iter-23-L-FC2-ends",
        back == readable_count && !d.visible_text("Ashen Marches · Cannot be read"),
        format!("After Try again the header count is back to {readable_count:?}: {back:?}"),
    );
    // It stops reading again, and the DM lets the draft go: the reader shows the fault.
    fs::write(&path, &not_text).unwrap();
    refocus(d);
    d.wait_for_text(&conflict);
    d.click("Reload and lose my draft");
    d.wait_for_text(HEADLINE);
    let dirty = d.app_mut().library.dirty();
    d.observe(
        "iter20-E-SD1-draft-reload",
        !dirty && d.has("Try again") && !d.has_containing("Your draft is safe"),
        "Reload and lose my draft shows the file's fault in the reader",
    );
    assert_eq!(fs::read(&path).unwrap(), not_text);
    // Undo discard brings the draft back over the unreadable file; the fault stays.
    d.click_starting("Undo discard");
    d.settle();
    let restored = d.app_mut().library.dirty();
    d.observe(
        "iter20-E-SD1-undo-discard",
        restored && d.visible_text(HEADLINE) && d.has("Try again"),
        format!("Undo discard keeps the draft ({restored}) and the file's fault"),
    );
    // The file reads again. The reader's Try again reads it now, with the draft open: the
    // entry reads, or the changed file and the draft are compared.
    fs::write(&path, &text).unwrap();
    d.settle();
    d.click("Try again");
    d.settle();
    let changed = d.has_containing("This entry changed on disk");
    let read = changed || (!d.visible_text(HEADLINE) && d.visible_text("Bite"));
    let kept = d.app_mut().library.dirty();
    d.observe(
        "iter20-E-SD1-draft-reload-try-again",
        read && kept,
        format!(
            "With a draft open, Try again reads the file again (changed {changed}, read {read}) and keeps the draft ({kept})"
        ),
    );
    if !(read && kept) {
        // The steps below need the file read again; the failed check above is the report.
        return;
    }
    if changed {
        d.click("Reload and lose my draft");
        if d.has("Read") {
            d.click("Read");
        }
        d.wait_for_text("Bite");
    }

    // The campaign path: a note that has never read as text.
    let note = root(d).join("Notes/Not text.md");
    fs::create_dir_all(note.parent().unwrap()).unwrap();
    fs::write(&note, [0xff, 0xfe, b'\n']).unwrap();
    d.click_starting("What the Library shows: ");
    d.click("Can use");
    refocus(d);
    search_library(d, "Not text");
    d.click_starting("Notes/Not text.md");
    let campaign_headline = HEADLINE;
    d.wait_for_text(campaign_headline);
    d.state_prepared("campaign-file-not-text", Sizes::All, |d| {
        file_fault_reader(
            d,
            "iter20-E-SD1-campaign",
            campaign_headline,
            "Notes/Not text.md",
            true,
        );
    });
    assert_eq!(fs::read(&note).unwrap(), [0xff, 0xfe, b'\n']);
}

/// The reader of a file that cannot be read: the cause, Show file, Try again at the message
/// words, Technical details, and no editor link, repair or source editing.
fn file_fault_reader(d: &mut Driver<'_>, id: &str, headline: &str, path: &str, campaign: bool) {
    // Round 24 FE5: the reader names the file's path once, on its file line; its title is the
    // file name.
    let pane = d.place_of("Read").map(|read| {
        egui::Rect::from_min_max(
            egui::pos2(read.left() - 1.0, read.top()),
            egui::pos2(f32::INFINITY, f32::INFINITY),
        )
    });
    let words = pane.map(|pane| d.painted_in(pane)).unwrap_or_default();
    let name = path.rsplit('/').next().unwrap_or(path);
    d.observe(
        &format!(
            "iter-24-L-FE5-{}",
            if campaign { "campaign" } else { "package" }
        ),
        pane.is_some()
            && words.matches(path).count() <= 1
            && d.painted_word_rect(name, pane.unwrap()).is_some()
            && !words.contains(&format!("cannot read {path}")),
        format!(
            "The reader says {path} at most once, on its file line, and is titled {name}: {words}"
        ),
    );
    // Round 24 AE1, FE2: the row marks the fault the header counts with the error icon in the
    // error colour, and the header count has the error tone.
    let error = (
        design::icons::ERROR.to_owned(),
        design::colors().result.error,
    );
    let mut rows = Vec::new();
    let mut header = Vec::new();
    d.each_painted_text(|text, clip| {
        let job = &text.galley.job;
        if !clip.intersects(text.visual_bounding_rect()) {
            return;
        }
        if job.text.contains("Not a text file") {
            rows.push(painted_marks(job));
        }
        if job.text.contains(" need fixing") || job.text.contains(" needs fixing") {
            header.push(painted_marks(job));
        }
    });
    d.observe(
        &format!(
            "iter-24-L-AE1-{}",
            if campaign { "campaign" } else { "package" }
        ),
        !rows.is_empty()
            && rows
                .iter()
                .all(|marks| marks == std::slice::from_ref(&error)),
        format!("The unreadable row has one mark, the error icon in the error colour: {rows:?}"),
    );
    d.observe(
        &format!(
            "iter-24-L-FE2-header-{}",
            if campaign { "campaign" } else { "package" }
        ),
        header.len() == 1 && header[0].first() == Some(&error),
        format!("The header count leads with the error icon in the error colour: {header:?}"),
    );
    let message = d.painted_rect_in(headline, egui::Rect::EVERYTHING);
    let again = d.place_of("Try again");
    let words = again.and_then(|again| super::words_left_in(d, again));
    let clear = again.map_or_else(
        || Err("Try again is not in view".to_owned()),
        |again| super::press_area_clear(d, again, "Try again", None),
    );
    // Round 25: Try again is a bordered secondary button, so its border (not its words) starts
    // at the message words, as every message action without an icon does.
    let bordered = d
        .control_frame("Try again")
        .is_some_and(|frame| frame.stroke.width > 0.0);
    let placed = message.zip(again).is_some_and(|(message, again)| {
        again.top() >= message.bottom() - 1.0 && (again.left() - message.left()).abs() <= 1.0
    });
    d.observe(
        &format!("{id}-try-again"),
        placed && bordered && clear.is_ok(),
        format!(
            "Try again {again:?} (words {words:?}, bordered {bordered}) starts at the message words {message:?}; press area: {clear:?}"
        ),
    );
    d.observe(
        &format!("{id}-recovery"),
        d.has("Show file")
            && d.has("Technical details")
            && d.visible_text("Splat reads notes as UTF-8 text.")
            && d.has_containing("Cannot be read"),
        "The reader names the cause, the recovery, Show file and Technical details",
    );
    // Round 21 FD2: the Library row names the short cause, as World's row does. The owner line
    // keeps `Cannot be read` (checked above).
    d.observe(
        &format!(
            "iter21-E-FD2-{}",
            if campaign { "campaign" } else { "package" }
        ),
        d.visible_text("Not a text file"),
        "The Library row names the short cause, Not a text file",
    );
    d.observe(
        &format!("{id}-no-repair"),
        !d.has("Edit in Obsidian")
            && !d.has("Fix in Edit Markdown")
            && !d.has_containing("frontmatter")
            && d.is_disabled(EDIT_MARKDOWN),
        format!(
            "No editor link, no repair of empty text, and Edit Markdown is blocked (campaign {campaign})"
        ),
    );
}

/// Round 20 SD2: a copied campaign's registration result names the new campaign, with its Undo on
/// the message line and Build an encounter below at the message words. Build an encounter leaves
/// the result and its Undo; Undo ends both.
pub fn campaign_copy_registered(d: &mut Driver<'_>) {
    const NAME: &str = "Emberfall Again";
    let copy = d.root.join("campaigns/emberfall-copy");
    fs::create_dir_all(&copy).unwrap();
    let manifest = fs::read(root(d).join("Campaign.md")).unwrap();
    fs::write(copy.join("Campaign.md"), &manifest).unwrap();
    let before = d.app_mut().campaigns.active_campaign().unwrap().id;
    go_settings(d);
    if !d.has("Campaign name") {
        d.click("Add campaign");
    }
    d.type_into("Campaign name", NAME);
    d.type_into("Folder", &copy.display().to_string());
    // Wave 2 d (C), from the review of PR 320: with no account at 960, the import hint follows
    // the open form. It was drawn above the form once the account heading left the view (here
    // the footer's result takes the room), and moved the form and its pressed Add campaign by
    // its height. (The new campaign's row moves the form down by one row on purpose.)
    const IMPORT_HINT: &str = "To import from D&D Beyond,";
    let account_in_view = |d: &Driver<'_>| {
        d.nodes()
            .into_iter()
            .find(|node| node.name.eq_ignore_ascii_case("D&D Beyond account"))
            .map(|node| node.fully_in_view())
    };
    let header = |d: &Driver<'_>| d.place_of("Add campaign");
    let pressed = (header(d), account_in_view(d));
    d.click_below("Folder", "Add campaign");
    d.expect("Register as a new campaign");
    d.click("Register as a new campaign");
    let result = crate::campaigns::copy_registered(NAME);
    d.wait_for_text(&result);
    let create = d
        .nodes()
        .into_iter()
        .filter(|node| node.name == "Add campaign")
        .map(|node| node.rect)
        .nth(1);
    let hint = d.place_of(IMPORT_HINT);
    let shown = (header(d), account_in_view(d));
    d.observe(
        "iter-28-C-import-hint",
        pressed.1 == Some(true)
            && shown.1 == Some(false)
            && hint
                .zip(create)
                .is_some_and(|(hint, create)| hint.top() >= create.bottom()),
        format!(
            "Before Add campaign (its header, the account heading whole in view): {pressed:?}; after the result: {shown:?}; the hint {hint:?} under the form's Add campaign {create:?}"
        ),
    );
    undo_follows_message(d, "iter20-E-SD2-undo", &result, "Undo register campaign");
    for size in [SMALL, DEFAULT, LARGE] {
        let (message, build, words, clear) = d.at_size(size, |d| {
            let message = d.painted_rect_in(&result, egui::Rect::EVERYTHING);
            let build = d.place_of("Build an encounter");
            let words = build.and_then(|build| super::words_left_in(d, build));
            let clear = build.map_or_else(
                || Err("Build an encounter is not in view".to_owned()),
                |build| super::press_area_clear(d, build, "Build an encounter", None),
            );
            (message, build, words, clear)
        });
        let (frame, undo) = d.at_size(size, |d| {
            (
                d.control_frame("Build an encounter")
                    .map(|frame| frame.stroke.color),
                d.place_of("Undo register campaign"),
            )
        });
        // Round 23 FB3: the same bordered action as after a plain add, on its own line below
        // the Undo line, its border at the message words.
        let placed = message.zip(build).zip(words).zip(undo).is_some_and(
            |(((message, build), words), undo)| {
                build.top() >= message.bottom() - 1.0
                    && build.top() >= undo.bottom()
                    && (build.left() - message.left()).abs() <= 1.0
                    && words > build.left()
            },
        );
        d.observe(
            "iter-23-C-FB3-build",
            placed && frame == Some(design::colors().field_border) && clear.is_ok(),
            format!(
                "At {size:?}, Build an encounter {build:?} (border {frame:?}, words {words:?}) starts at the message words {message:?}, below Undo {undo:?}; press area: {clear:?}"
            ),
        );
    }
    d.state("copy-registered", Sizes::All);
    d.click("Build an encounter");
    assert_eq!(d.current_page(), "Encounters");
    go_settings(d);
    d.wait_for_text(&result);
    d.observe(
        "iter20-E-SD2-build-keeps",
        d.visible_text(&result) && d.has("Undo register campaign"),
        "After Build an encounter, the result and its Undo are still one message",
    );
    // The return shows the same result and Undo as before Build an encounter.
    d.assert_state_matches("copy-registered-after-build", "copy-registered");
    // Review round 26: Undo register campaign changes the registration and can change the active
    // campaign, so it waits for an unsaved campaign draft and for a run, with the reason; the
    // copy's bytes and the registration stay.
    const UNDO: &str = "Undo register campaign";
    let registered = |d: &mut Driver<'_>| {
        (
            d.app_mut().campaigns.all().len(),
            fs::read(copy.join("Campaign.md")).unwrap(),
        )
    };
    let start = registered(d);
    go_campaign(d);
    d.click_starting("Ashen Marches · ");
    d.click("Use Ash Wraith");
    go_settings(d);
    d.wait_for_text(&result);
    let draft_locked = d.is_disabled(UNDO);
    let draft_kept = registered(d) == start;
    go_campaign(d);
    d.click_below("Save selections", "Discard");
    go_settings(d);
    d.wait_for_text(&result);
    let free_again = !d.is_disabled(UNDO);
    d.click("Make The Emberfall Marches the active campaign");
    go_encounters(d);
    resume_run_from_cold_start(d);
    go_settings(d);
    d.wait_for_text(&result);
    let run_locked = d.is_disabled(UNDO) && d.visible_text("Close the run to change campaigns.");
    let run_kept = registered(d) == start;
    d.click("Back to run");
    close_run(d);
    go_settings(d);
    d.observe(
        "iter-26-C-undo-register-locked",
        draft_locked && draft_kept && free_again && run_locked && run_kept,
        format!(
            "Undo locked by a draft {draft_locked} (kept {draft_kept}), free after Discard \
             {free_again}, locked by a run {run_locked} (kept {run_kept})"
        ),
    );
    d.click(UNDO);
    d.settle();
    let active = d.app_mut().campaigns.active_campaign().map(|c| c.id);
    d.observe(
        "iter20-E-SD2-undo-ends",
        !d.has_containing("is registered as a new campaign")
            && !d.has("Undo register campaign")
            && active == Some(before)
            && fs::read(copy.join("Campaign.md")).unwrap() == manifest,
        "Undo ends the result and its Undo, and puts the copy's file and the active campaign back",
    );
}

/// Round 26 FD1: the top of Campaign's package list (its first row) at 960, 1200 and 1920.
fn package_list_tops(d: &mut Driver<'_>) -> [Option<f32>; 3] {
    [(960.0, 640.0), (1200.0, 800.0), (1920.0, 1080.0)].map(|size| {
        d.at_size(size, |d| {
            d.nodes()
                .into_iter()
                .find(|node| node.name.starts_with("D&D 2024"))
                .map(|node| node.rect.top())
        })
    })
}

/// iter-27-A: a short repair's footer (the one whose words hold `words`) sits `S` below its
/// save row, clear of the pane's bottom, and draws no hairline, as it cuts no body.
fn repair_footer_follows(d: &mut Driver<'_>, words: &str, id: &str) {
    let size = d.window_size();
    let save = d.place_of("Save repaired note");
    let footer = footer_with(d, words);
    let hairline = footer.map(|footer| {
        d.hairlines_in(egui::Rect::from_min_max(
            footer.left_top() - egui::vec2(0.0, 1.0),
            egui::pos2(footer.right(), footer.top() + 2.0),
        ))
    });
    d.observe(
        id,
        save.zip(footer).is_some_and(|(save, footer)| {
            (footer.top() - (save.bottom() + design::S)).abs() <= 1.0
                && footer.bottom() < size.1 - design::XXL
        }) && hairline.as_ref().is_some_and(Vec::is_empty),
        format!(
            "At {size:?}, the footer is `S` below the save row, with no hairline: save {save:?}, footer {footer:?}, hairlines {hairline:?}"
        ),
    );
}

/// The stat block's last section, next to the run panel's footer.
const SOURCE_SECTION: &str = "Source and identifiers";

/// iter-27-R FD5, review of PR 295 (revision 19 press proof): at 960, 1200 and 1920, with the
/// run's panel on the stat block, each press reaches only its own control while the panel's
/// footer holds `notice` (`None`: the footer is empty). The stat block's last action, `Source
/// and identifiers`, is scrolled to the body's end next to the footer and opens only itself;
/// the footer's `Read the new version`, when `action`, opens only Lookup and is reached by Tab;
/// and typing in an outside field (`Amount`) changes only that field.
fn footer_presses(d: &mut Driver<'_>, id: &str, notice: Option<&str>, action: bool) {
    const SOURCE: &str = SOURCE_SECTION;
    const OPENED: &str = "Starting snapshot, saved with this run";
    for size in [(960.0, 640.0), (1200.0, 800.0), (1920.0, 1080.0)] {
        let report = d.act_at_size(size, |d| {
            let footer = |d: &Driver<'_>| notice.is_none_or(|words| d.has_containing(words));
            let empty = |d: &Driver<'_>| {
                notice.is_some() || !d.has_containing("uses the copy from the start of the fight")
            };
            // The body's end: the stat block scrolled down, so its last action is the body's
            // lowest control, next to the footer.
            if let Some(tab) = d.place_of("Stat block") {
                d.scroll_at(egui::pos2(tab.center().x, tab.bottom() + 120.0), 12);
            }
            let source = d.place_of(SOURCE);
            d.click(SOURCE);
            let body = d.has_containing(OPENED) && !d.has("Close Lookup") && footer(d) && empty(d);
            // A press that went elsewhere is recorded, then put back, so each part still runs.
            if d.has("Close Lookup") {
                d.press_ctrl(egui::Key::K);
                if !d.has(SOURCE) {
                    d.click("Stat block");
                }
            }
            if d.has_containing(OPENED) {
                d.click(SOURCE);
            }
            let closed = !d.has_containing(OPENED);
            let (pressed, tab) = if action && d.has("Read the new version") {
                d.click("Read the new version");
                let pressed = d.has("Close Lookup") && !d.has_containing(OPENED);
                d.press_ctrl(egui::Key::K);
                if !d.has("Read the new version") {
                    d.click("Stat block");
                }
                let tab = d.tab_to("Read the new version")
                    && d.focused_name().as_deref() == Some("Read the new version");
                (pressed, tab)
            } else {
                (!action && !d.has("Read the new version"), true)
            };
            d.click("Amount");
            d.type_text("7");
            let typed = d.focused_name().as_deref() == Some("Amount")
                && !d.has("Close Lookup")
                && !d.has_containing(OPENED)
                && footer(d)
                && empty(d);
            d.clear_text_in("Amount");
            (
                body && closed && pressed && tab && typed && source.is_some(),
                format!(
                    "source {source:?}: body {body}, closed again {closed}, footer action {pressed}, \
                     Tab {tab}, outside field {typed}"
                ),
            )
        });
        d.observe(id, report.0, format!("At {size:?}: {}", report.1));
    }
}
