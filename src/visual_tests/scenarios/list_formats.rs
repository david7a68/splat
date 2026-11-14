//! F3 files remain readable after Obsidian edits. Reading never changes them.
use super::*;
use std::fs;

/// FB2: the flagged member in the groups list, the plan and Arrivals, with its line and cause.
const SCOUT_FAULT: &str =
    "Scout (as Goblin Warrior) · line\u{a0}15: the change “HP lots” was not read.";

/// FB2: each summary of the flagged member is the cause alone; the example `Use a number, such
/// as HP +5.` belongs to the editor. The found texts also hold each painted row of a wrapped
/// summary, so a row must be the start of the cause.
fn scout_fault_alone(d: &mut Driver<'_>, id: &str, place: &str) {
    let names: Vec<_> = d
        .nodes()
        .into_iter()
        .filter(|node| node.name.starts_with("Scout (as Goblin Warrior) · line"))
        .map(|node| node.name)
        .collect();
    d.observe(
        id,
        names.iter().any(|name| name == SCOUT_FAULT)
            && names
                .iter()
                .all(|name| SCOUT_FAULT.starts_with(name.as_str())),
        format!("{place} names Scout's cause without the example: {names:?}"),
    );
}

pub fn run(d: &mut Driver<'_>) {
    let campaign = d.app_mut().campaigns.active_campaign().unwrap();
    let folder = crate::sessions::folder(&campaign).unwrap();
    let path = folder.join(crate::timeline::FILE);
    let before = fs::read_to_string(&path).unwrap();
    let after = format!("{before}\nThe toll keeper has a silver bell.\n");
    fs::write(&path, &after).unwrap();
    d.app_mut().timeline.reload();
    d.click(NAV_TIMELINE);
    d.click_containing("Old bridge · ");
    d.expect("The party argued about the toll.");
    d.scroll_until(
        egui::pos2(900.0, 500.0),
        "The party argued about the toll.",
        12,
    );
    d.state("timeline-v2-dm-prose", Sizes::All);
    assert_eq!(fs::read_to_string(&path).unwrap(), after);
    assert!(after.contains("The toll keeper has a silver bell."));
    assert!(after.contains("%% splat id=t-f3-day %%"));

    // This is the exact one-character edit an Obsidian task toggle makes.
    let manifest = campaign.path.join("Campaign.md");
    let before = fs::read_to_string(&manifest).unwrap();
    let module = d.app_mut().campaigns.modules()[0].clone();
    let task = before
        .lines()
        .find(|l| l.ends_with(&format!(" ^{}", module.id)))
        .unwrap();
    let after = before.replace(task, &task.replacen("[x]", "[ ]", 1));
    assert_ne!(before, after);
    fs::write(&manifest, &after).unwrap();
    d.click(NAV_CAMPAIGN);
    d.click("Reload from disk");
    d.settle();
    let document = crate::content_workspace::CampaignDocument::load(&campaign).unwrap();
    d.observe(
        "F3 package toggle",
        !document
            .selections
            .iter()
            .any(|s| s.package.id == Some(module.id)),
        "An Obsidian task toggle removes the package from the active campaign selection.",
    );
    d.expect("No changes");
    // Round 27 FA6: the three entries other choices need are ticked, so they count as chosen.
    d.expect("3 of 6 chosen");
    d.expect("Arena Final, Bridge Ambush, Lost Patrol, Rival Team and Shrine of Cinders use its creatures.");
    // Round 22 FD4: a fault count is a status, the warning icon and colour before the words,
    // and it keeps its name and collapse.
    let warning = crate::design::colors().result.warning;
    let heading = format!("{} 1 content issue to repair", crate::design::icons::WARN);
    let mut colours = Vec::new();
    d.each_painted_text(|text, _| {
        if text.galley.job.text == heading {
            colours = text
                .galley
                .job
                .sections
                .iter()
                .map(|section| section.format.color)
                .collect();
        }
    });
    d.observe(
        "iter-22-C-FD4",
        !colours.is_empty()
            && colours.iter().all(|colour| *colour == warning)
            && d.in_view("1 content issue to repair"),
        format!("The issue count paints {colours:?}"),
    );
    // Round 28 FC12: the note starts at its row's name, not at the row's checkbox.
    let mut lefts = (None, None);
    d.each_painted_text(|text, _| {
        let words = &text.galley.job.text;
        if words == "Ashen Marches" {
            lefts.0 = Some(text.pos.x);
        } else if words.starts_with("Arena Final, ") {
            lefts.1 = Some(text.pos.x);
        }
    });
    d.observe(
        "iter-28-C-FC12",
        matches!(lefts, (Some(name), Some(note)) if (name - note).abs() <= 1.0),
        format!("Package name and its note start at x: {lefts:?}"),
    );
    d.state("campaign-package-off-in-obsidian", Sizes::All);
    assert_eq!(fs::read_to_string(&manifest).unwrap(), after);

    let flagged = after.replace("| --- | --- |\n", "| --- | --- |\n| Combat initiative | Imaginary initiative %% splat rule=dnd2024:rules/combat_initiative id=00000000-0000-0000-0000-000000000002 %% |\n");
    assert_ne!(flagged, after);
    fs::write(&manifest, &flagged).unwrap();
    d.focus_window(false);
    d.focus_window(true);
    d.click("Reload from disk");
    d.settle();
    d.expect("Imaginary initiative");
    d.expect("Combat initiative choice");
    d.expect("Splat uses the D&D default");
    d.observe_both(
        "iter-1-C-FD4-13",
        "The save row stays above the campaign warning and its repair actions",
        |d| {
            let nodes = d.nodes();
            nodes
                .iter()
                .find(|node| node.name == "Save selections")
                .zip(
                    nodes
                        .iter()
                        .find(|node| node.name == "Choose in Rules and runs"),
                )
                .is_some_and(|(save, repair)| save.rect.bottom() < repair.rect.top())
                && d.has("2 content issues to repair")
        },
    );
    // Round 22 FD1 family: Campaign's flag keeps its repair choice and file line at its words.
    for size in [SMALL, DEFAULT, (1920.0, 1080.0)] {
        let placed = d.at_size(size, |d| {
            super::file_links_at_message_words(
                d,
                "Line 23: Combat initiative",
                &manifest,
                "Campaign.md",
            )
            .and_then(|()| {
                let words = d
                    .nodes()
                    .into_iter()
                    .find(|node| node.name.starts_with("Line 23: Combat initiative"))
                    .map(|node| node.rect);
                let choice = d.place_of("Choose in Rules and runs");
                let left = choice.and_then(|choice| super::words_left_in(d, choice));
                words
                    .zip(left)
                    .is_some_and(|(words, left)| left >= words.left() - 1.0)
                    .then_some(())
                    .ok_or(format!("choice words at {left:?}, message {words:?}"))
            })
        });
        d.observe(
            "iter-22-C-FD1-campaign",
            placed.is_ok(),
            format!("At {size:?}, the Campaign flag's choice and file line: {placed:?}"),
        );
    }
    d.state("campaign-flagged-rule-choice", Sizes::All);
    assert_eq!(fs::read_to_string(&manifest).unwrap(), flagged);
    // Round 22 review: two flags about Campaign.md give one file line, after the last flag.
    let two_flags = flagged.replace(
        " id=00000000-0000-0000-0000-000000000002 %% |\n",
        " id=00000000-0000-0000-0000-000000000002 %% |\n| Flanking | Imaginary flanking %% splat rule=dnd2024:rules/flanking %% |\n",
    );
    assert_ne!(two_flags, flagged);
    fs::write(&manifest, &two_flags).unwrap();
    d.click("Reload from disk");
    d.settle();
    d.expect("3 content issues to repair");
    for size in [SMALL, DEFAULT, (1920.0, 1080.0)] {
        let placed = d.at_size(size, |d| {
            let last = d
                .nodes()
                .into_iter()
                .find(|node| node.name.contains("Flanking choice"))
                .ok_or("the second flag is not shown".to_owned())?;
            let first = d
                .nodes()
                .into_iter()
                .find(|node| node.name.starts_with("Line 23: Combat initiative"))
                .map(|node| node.rect)
                .ok_or("the first flag is not shown".to_owned())?;
            let files: Vec<_> = d
                .nodes()
                .into_iter()
                .filter(|node| node.name == manifest.display().to_string())
                .map(|node| node.rect)
                .collect();
            super::file_links_at_message_words(d, &last.name, &manifest, "Campaign.md").and_then(
                |()| {
                    (files.len() == 1 && first.bottom() <= last.rect.top())
                        .then_some(())
                        .ok_or(format!(
                            "file lines {files:?}; flags {first:?}, {:?}",
                            last.rect
                        ))
                },
            )
        });
        d.observe(
            "iter-22-C-FD1-campaign-two-flags",
            placed.is_ok(),
            format!("At {size:?}, two Campaign flags and their file line: {placed:?}"),
        );
    }
    fs::write(&manifest, &flagged).unwrap();
    d.click("Reload from disk");
    d.settle();
    d.expect("2 content issues to repair");
    d.click("Choose in Rules and runs");
    d.expect("Combat initiative choice");
    d.expect("Roll NPC initiative");
    d.state("rules-and-runs-flagged-choice", Sizes::All);
    // An unchanged form with an unread rule choice says what its enabled Save writes.
    const REPAIR_STATE: &str = "Save replaces the unread choice on line 23 with these settings";
    d.observe(
        "iter-23-B-AA1",
        d.visible_text(REPAIR_STATE) && !d.visible_text("No changes"),
        "The save row names the write instead of No changes",
    );
    // FD3 family: the file line starts at the warning's words.
    for size in [SMALL, DEFAULT, (1920.0, 1080.0)] {
        let placed = d.at_size(size, |d| {
            // FD5: the file line names the file, so the warning names its line only.
            let named = d
                .nodes()
                .into_iter()
                .filter(|node| node.name.contains("Combat initiative choice") && node.in_view())
                .map(|node| node.name)
                .collect::<Vec<_>>();
            super::file_links_at_message_words(
                d,
                "Line 23: Combat initiative",
                &campaign.path.join("Campaign.md"),
                "Campaign.md",
            )
            .and_then(|()| {
                (!named.is_empty()
                    && named
                        .iter()
                        .all(|name| name.starts_with("Line ") && !name.contains("Campaign.md")))
                .then_some(())
                .ok_or(format!("warnings {named:?}"))
            })
        });
        d.observe(
            "iter-21-B-FD3-rules",
            placed.is_ok(),
            format!("At {size:?}, the Rules and runs warning's file line: {placed:?}"),
        );
    }
    // iter-26-B FC5: the warning sits in the tab's footer, so Roll NPC initiative and Save keep
    // their y when Save clears it, at 960, 1200 and 1920.
    let form_tops = |d: &mut Driver<'_>| {
        [SMALL, DEFAULT, (1920.0, 1080.0)].map(|size| {
            d.at_size(size, |d| {
                ["Roll NPC initiative", "Save run settings"].map(|name| {
                    d.nodes()
                        .into_iter()
                        .filter(|node| node.name.starts_with(name))
                        .map(|node| node.rect.top().round())
                        .reduce(f32::max)
                })
            })
        })
    };
    let with_warning = form_tops(d);
    // The Ctrl+S that Save shows saves exactly when Save can.
    d.press_ctrl(Key::S);
    d.expect("No changes");
    let repaired = form_tops(d);
    d.observe(
        "iter-26-B-FC5",
        with_warning.iter().flatten().all(Option::is_some) && repaired == with_warning,
        format!(
            "Initiative and Save tops with the warning {with_warning:?} and after Save {repaired:?}"
        ),
    );
    d.observe(
        "iter-23-B-AA2",
        !d.has_containing("Imaginary initiative") && !d.visible_text(REPAIR_STATE),
        "Ctrl+S writes the shown settings over the unread choice, as Save does",
    );
    // Put the unread choice back for the pointer's repair below. The changed file makes a run
    // settings draft; discard it, then read the file again.
    fs::write(&manifest, &flagged).unwrap();
    d.settle();
    if d.has("Discard") {
        d.click("Discard");
    }
    d.click(NAV_ENCOUNTERS);
    d.click("Reload from disk");
    d.settle();
    d.click(NAV_CAMPAIGN);
    d.click("Rules and runs");
    d.expect(REPAIR_STATE);
    // The drop-down's name says its value too (`Roll NPC initiative: …`).
    d.click_starting("Roll NPC initiative: ");
    d.click("Individually");
    d.click("Save run settings");
    d.expect("No changes");
    d.observe_both(
        "W12 rule repair",
        "Saving the chosen rule clears its warning",
        |d| !d.has_containing("Imaginary initiative") && d.visible_text("Individually"),
    );
    d.state("rules-and-runs-choice-repaired", Sizes::All);
    // Astra review of PR 235: the reader takes the heading in any case, so a lowercase
    // `combat initiative` row is a rule choice too. Campaign's repair link opens Rules and
    // runs, and the warning is there.
    let saved = fs::read_to_string(&manifest).unwrap();
    let lowercase = saved.replacen(
        "| --- | --- |\n",
        "| --- | --- |\n| combat initiative | Lost initiative %% splat rule=dnd2024:rules/combat_initiative id=00000000-0000-0000-0000-000000000003 %% |\n",
        1,
    );
    assert_ne!(lowercase, saved);
    fs::write(&manifest, &lowercase).unwrap();
    d.click("Packages");
    d.focus_window(false);
    d.focus_window(true);
    d.click("Reload from disk");
    d.settle();
    d.expect("Lost initiative");
    d.click("Choose in Rules and runs");
    d.settle();
    d.observe_both(
        "iter-23-Q-lowercase-rule-choice",
        "A lowercase rule heading keeps its warning on the page its repair link opens",
        |d| {
            d.visible_text("Roll NPC initiative")
                && d.has_containing("combat initiative choice “Lost initiative” is not a rule.")
        },
    );
    fs::write(&manifest, &saved).unwrap();
    d.click("Packages");
    d.focus_window(false);
    d.focus_window(true);
    d.click("Reload from disk");
    d.settle();
    fs::write(&manifest, &after).unwrap();

    d.click(NAV_ENCOUNTERS);
    d.click_starting("GROUPS");
    d.expect("2 of 3 can join");
    // FB2: every member fault says its cause, as a roster row does.
    d.expect(SCOUT_FAULT);
    scout_fault_alone(d, "iter-24-F-FB2-list", "The groups list");
    d.state("flagged-group-in-groups-list", Sizes::All);
    d.click(NAV_SESSIONS);
    new_session(d, "Bridge watch");
    show_plan(d);
    d.click("Add a group");
    d.click("Plan Bridge guards");
    d.expect("2 of 3 can join");
    d.state_prepared("flagged-group-in-the-plan", Sizes::All, |d| {
        scout_fault_alone(d, "iter-24-F-FB2-plan", "The plan");
        // AD2: the row names the member left out, its base and its line, beside Edit group.
        let nodes = d.nodes();
        let warning = nodes
            .iter()
            .find(|node| node.name.starts_with(SCOUT_FAULT))
            .map(|node| node.rect);
        let link = d.place_of("Bridge guards");
        let edit = d.place_of("Edit group Bridge guards");
        d.observe(
            "iter-10-C-AD2",
            match (warning, link, edit) {
                (Some(warning), Some(link), Some(edit)) => {
                    warning.top() >= link.bottom()
                        && warning.left() >= link.left() - 0.5
                        && (edit.center().y - link.center().y).abs() <= crate::design::Density::Read.control()
                        && d.visible_text(SCOUT_FAULT)
                }
                _ => false,
            },
            format!("The plan row names Scout, its base and line below the row: {warning:?} {link:?} {edit:?}"),
        );
        // `line 15` wraps whole: no painted row of the warning starts inside it.
        let mut rows = Vec::new();
        d.each_painted_text(|text, _| {
            if text.galley.job.text.starts_with(SCOUT_FAULT) {
                rows.extend(text.galley.rows.iter().map(|row| {
                    row.glyphs.iter().map(|glyph| glyph.chr).collect::<String>()
                }));
            }
        });
        d.observe(
            "iter-23-B-FA12",
            !rows.is_empty()
                && rows.iter().any(|row| row.contains("line\u{a0}15:"))
                && rows.iter().all(|row| !row.trim_start().starts_with("15:")),
            format!("The plan warning's rows: {rows:?}"),
        );
        d.observe(
            "iter-10-C-AD2-summary",
            d.visible_text("2 of 3 can join"),
            "The row keeps its summary above the warning",
        );
        // Round 22 AD3: a flagged group's summary keeps each `·` with the words after it, and
        // shows every fact.
        let name = d
            .nodes()
            .into_iter()
            .find(|node| node.name.contains("2 of 3 can join") && node.in_view())
            .map(|node| node.name);
        let mut rows = Vec::new();
        let mut painted = String::new();
        d.each_painted_text(|text, _| {
            if text.galley.job.text.contains("2 of 3 can join") {
                painted = text.galley.job.text.replace('\u{a0}', " ");
                rows = text
                    .galley
                    .rows
                    .iter()
                    .map(|row| row.glyphs.iter().map(|glyph| glyph.chr).collect::<String>())
                    .collect();
            }
        });
        d.observe(
            "iter-22-C-AD3",
            !rows.is_empty()
                && rows.iter().all(|row| !row.trim_end().ends_with('·'))
                // iter-26-B FI7: no row starts with a loose `·` either; a line break stands for
                // the separator it dropped, and the name keeps it.
                && rows.iter().all(|row| !row.trim_start().starts_with('·'))
                && !rows.concat().contains('…')
                && name.as_deref() == Some(painted.replace('\n', " · ").as_str()),
            format!("Flagged summary {name:?} paints rows {rows:?}"),
        );
        // FC6: `Will add:` stays on a line with its first member.
        let plain: Vec<String> = rows.iter().map(|row| row.replace('\u{a0}', " ")).collect();
        d.observe(
            "iter-25-B-FC6",
            plain.iter().any(|row| row.contains("Will add: "))
                && plain.iter().all(|row| !row.trim_end().ends_with("Will add:")),
            format!("Flagged summary rows {plain:?}"),
        );
    });
    d.click(NAV_ENCOUNTERS);
    run_saved_encounter(d, "Shrine of Cinders");
    open_arrivals(d);
    // FB2: every member fault says its cause, as a roster row does.
    d.expect(SCOUT_FAULT);
    d.expect("Edit group");
    scout_fault_alone(d, "iter-24-F-FB2-arrivals", "Arrivals");
    d.state("flagged-group-in-arrivals", Sizes::All);
    close_run(d);
    d.click(NAV_ENCOUNTERS);
    d.click("Edit group Bridge guards");
    // FA1 (Astra review of PR 259): the editor says the member fault in the list's own words,
    // and its help on a line of its own.
    d.expect(SCOUT_FAULT);
    d.expect("Use a number, such as HP +5.");
    d.observe(
        "iter-25-B-FA1-editor",
        !d.has_containing("Line 15: the change"),
        "The group editor's member fault is the one form, not the flag's own sentence",
    );
    d.expect("Goblin Warrior");
    d.observe_both(
        "F3 member warning",
        "The named warning is below its member summary and before the next member",
        |d| {
            let nodes = d.nodes();
            let find = |name: &str| nodes.iter().find(|node| node.name == name);
            let first = find("1. Scout");
            let next = find("2. Goblin Warrior");
            let warning = find(SCOUT_FAULT);
            let summary = find("CR 1/4 · HP 10 · AC 15 · as Goblin Warrior");
            first.zip(summary).zip(warning).zip(next).is_some_and(
                |(((first, summary), warning), next)| {
                    first.rect.bottom() <= summary.rect.top()
                        && summary.rect.bottom() <= warning.rect.top()
                        && warning.rect.bottom() < next.rect.top()
                        && warning.rect.left() >= first.rect.left()
                        && warning.in_view()
                },
            ) && d.visible_text("This member cannot join a fight until its Changes cell is fixed. Other members can join.")
        },
    );
    d.expect("XP 100");
    // Scout cannot join, so the heading's XP is for the two who can, and it says so.
    d.observe(
        "iter-23-B-FC3",
        d.has_containing("2 can join · XP 100"),
        "The members heading says how many can join before its XP",
    );
    d.expect("Edit in Obsidian");
    d.expect("Show file");
    d.expect("Changes for Scout");
    d.state("group-flagged-changes-cell", Sizes::All);
    // FD3 family: what the fault blocks and the file line start at the warning's words.
    for size in [SMALL, DEFAULT, (1920.0, 1080.0)] {
        let placed = d.at_size(size, |d| {
            let words = d
                .nodes()
                .into_iter()
                .find(|node| node.name == SCOUT_FAULT && node.in_view())
                .map(|node| node.rect);
            let blocked = super::glyph_spans(d, "This member cannot join a fight", "This member");
            let blocked_at = words
                .zip(blocked.first())
                .is_some_and(|(words, (left, _, _))| (left - words.left()).abs() <= 1.0);
            super::file_links_at_message_words(
                d,
                "Scout (as Goblin Warrior) · line",
                &campaign.path.join("Encounter Groups/Bridge guards.md"),
                "Encounter Groups/Bridge guards.md",
            )
            .and_then(|()| {
                blocked_at
                    .then_some(())
                    .ok_or(format!("blocked line {blocked:?}, message words {words:?}"))
            })
        });
        d.observe(
            "iter-21-B-FD3-group",
            placed.is_ok(),
            format!("At {size:?}, the group member warning's lines: {placed:?}"),
        );
    }
    let text = fs::read_to_string(campaign.path.join("Encounter Groups/Bridge guards.md")).unwrap();
    assert!(text.contains("HP lots | Keep this row"));
    d.click("More actions for Scout");
    d.expect("Remove member");
    d.state("flagged-member-menu", Sizes::All);
    // FT2: the group editor's menu uses the same note helper as the roster menu.
    d.press(Key::Escape);
    d.click("More actions for Goblin Warrior");
    d.expect("Already last.");
    for size in [SMALL, DEFAULT, (1920.0, 1080.0)] {
        let note = d.at_size(size, |d| {
            super::note_starts_at_choice_words(d, "Move down", "Already last.")
        });
        d.observe(
            "iter-21-B-FT2-group",
            note.is_ok(),
            format!("At {size:?}, the group member menu's note: {note:?}"),
        );
    }
    d.press(Key::Escape);
    d.click("Changes for Scout");
    d.expect("Display name");
    d.expect("For example: HP +5, AC +2, Speed 40 ft.");
    d.state("group-repair-changes-cell", Sizes::All);
    // iter-24-B FB6: the change form has one right edge: Display name, Note, the Other effect
    // row's Add and the Loot field end at one x (`Roll on a table` follows the Loot field).
    for size in [DEFAULT, (1920.0, 1080.0)] {
        let edges = d.at_size(size, |d| {
            let other = d.place_of("Other effect")?;
            let add = d
                .nodes()
                .into_iter()
                .filter(|node| {
                    node.name == "Add"
                        && node.rect.left() >= other.right()
                        && node.rect.top() < other.bottom()
                        && other.top() < node.rect.bottom()
                })
                .map(|node| node.rect)
                .min_by(|a, b| a.left().total_cmp(&b.left()))?
                .right();
            Some([
                d.place_of("Display name")?.right(),
                d.place_of("Note")?.right(),
                add,
                d.place_of("Add loot to Scout")?.right(),
            ])
        });
        d.observe(
            "iter-24-B-FB6",
            edges.is_some_and(|edges| edges.iter().all(|x| (x - edges[0]).abs() <= 1.0)),
            format!(
                "At {size:?}, Display name, Note, Other effect's Add and Loot end at {edges:?}"
            ),
        );
    }
    // FD2: the unread value is a warning message, and its example starts at its words.
    for size in [SMALL, DEFAULT, (1920.0, 1080.0)] {
        let (tones, words, example) = d.at_size(size, |d| {
            let words = super::glyph_spans(d, "Changes “HP lots” not read.", "Changes");
            let example = super::glyph_spans(d, "For example: HP +5", "For");
            (
                super::tones_beside(d, "Changes “HP lots” not read."),
                words,
                example,
            )
        });
        let colour = d.painted_color(
            "Changes “HP lots” not read. Set the changes below, then save the group.",
        );
        d.observe(
            "iter-22-B-FD2",
            tones == ["warning"]
                && colour == Some(crate::design::colors().text)
                && words
                    .first()
                    .zip(example.first())
                    .is_some_and(|(words, example)| (words.0 - example.0).abs() <= 1.0),
            format!("At {size:?}: tones {tones:?}, colour {colour:?}, words {words:?}, example {example:?}"),
        );
    }
    d.click("Maximum HP: how it changes");
    d.click("Maximum HP: Add");
    d.click("Maximum HP: amount");
    d.press_ctrl(Key::A);
    d.type_text("5");
    d.press(Key::Enter);
    d.click("Done with changes");
    d.click("Save group");
    let saved =
        fs::read_to_string(campaign.path.join("Encounter Groups/Bridge guards.md")).unwrap();
    assert_eq!(saved, text.replace("HP lots", "HP +5"));
    // iter-28-B FC9: the repaired member's `Changed` has the muted pencil before it, on the
    // member's detail line (the line that holds its CR), as the encounter editor and the list
    // show a change.
    for size in [SMALL, DEFAULT, LARGE] {
        let placed = d.at_size(size, |d| {
            let link = d.place_of("Changed")?;
            let mut pencil = None;
            let mut detail = None;
            d.each_painted_text(|text, clip| {
                let rect = egui::Rect::from_min_size(text.pos, text.galley.size());
                if !clip.intersects(rect) {
                    return;
                }
                let words = &text.galley.job.text;
                // Review of PR 327: the pencil is muted, as in the list.
                if words.starts_with(crate::design::icons::CHANGED)
                    && text.galley.job.sections.first().is_some_and(|section| {
                        section.format.color == crate::design::colors().muted
                    })
                    && rect.right() <= link.left() + 0.5
                    && (rect.center().y - link.center().y).abs() <= 1.0
                {
                    pencil = Some(rect);
                }
                // One centre line for the detail and the link.
                if words.replace('\u{a0}', " ").contains("CR ")
                    && (rect.center().y - link.center().y).abs() <= 1.0
                {
                    detail = Some(rect);
                }
            });
            Some((pencil.is_some(), detail.is_some()))
        });
        d.observe(
            "iter-28-B-FC9",
            placed == Some((true, true)),
            format!("At {size:?}, Changed has its pencil, on the detail line: {placed:?}"),
        );
    }
    d.state("group-changes-cell-repaired", Sizes::All);

    d.click("Back to encounters");
    let catalogue =
        crate::content_workspace::Catalogue::load(d.app_mut().campaigns.modules(), Some(&campaign))
            .unwrap();
    let encounter = catalogue
        .entries
        .values()
        .find(|entry| entry.entry.name == "Shrine of Cinders" && entry.entry.kind == "encounter")
        .unwrap();
    let roster_path = campaign.path.join(&encounter.entry.path);
    let roster_text = fs::read_to_string(&roster_path).unwrap();
    // FD3: the header's place without a fault, to compare with the flagged roster below.
    let header = |d: &Driver<'_>| {
        [
            "Encounter name: Shrine of Cinders",
            "Save encounter",
            "Start fight",
        ]
        .map(|name| d.place_of(name).map(|rect| rect.top()))
    };
    let sizes = [SMALL, DEFAULT, (1920.0, 1080.0)];
    open_saved_encounter(d, "Shrine of Cinders");
    d.expect("Encounter name: Shrine of Cinders");
    assert!(!d.has_containing("was not read"));
    let clean = sizes.map(|size| d.at_size(size, header));
    d.click("Back to encounters");
    let mut flagged_roster = roster_text.clone();
    let table = splat::content::layout::Document::new(&roster_text)
        .table(splat::content::list_formats::ROSTER)
        .unwrap();
    let cell = &table.rows.last().unwrap()[3];
    flagged_roster.replace_range(cell.range.clone(), "HP lots");
    fs::write(&roster_path, &flagged_roster).unwrap();
    d.click("Reload from disk");
    open_saved_encounter(d, "Shrine of Cinders");
    d.expect("the change “HP lots” was not read");
    assert!(d.is_disabled("Start fight"));
    assert!(!d.app_mut().encounters.has_run());
    d.expect("Edit in Obsidian");
    d.expect("Show file");
    for (size, clean) in sizes.into_iter().zip(clean) {
        let (flagged, placed) = d.at_size(size, |d| {
            let flagged = header(d);
            let message = d
                .nodes()
                .into_iter()
                .find(|node| node.name.starts_with("Ember Cultist · ") && node.in_view())
                .map(|node| node.rect);
            let below = message
                .zip(d.place_of("Save encounter"))
                .is_some_and(|(message, save)| message.top() >= save.bottom());
            // FD5: the warning names the row and its line; the file line below holds the path.
            let named = d
                .nodes()
                .into_iter()
                .find(|node| node.name.starts_with("Ember Cultist · ") && node.in_view())
                .map(|node| node.name)
                .unwrap_or_default();
            let placed = super::file_links_at_message_words(
                d,
                "Ember Cultist · line 20: ",
                &roster_path,
                &encounter.entry.path,
            )
            .and_then(|()| {
                (below
                    && d.visible_text("Fix this roster row or remove it in Obsidian")
                    && !named.contains(&encounter.entry.path))
                    .then_some(())
                    .ok_or(format!(
                        "message {message:?} is above the header, has no instruction or names the path: {named:?}"
                    ))
            });
            (flagged, placed)
        });
        let kept = clean.iter().all(Option::is_some)
            && clean.iter().zip(&flagged).all(|(clean, flagged)| {
                clean
                    .zip(*flagged)
                    .is_some_and(|(a, b)| (a - b).abs() <= 0.5)
            });
        d.observe(
            "iter-21-B-FD3",
            kept && placed.is_ok(),
            format!(
                "At {size:?}, header tops without a fault {clean:?}, with one {flagged:?}; warning: {placed:?}"
            ),
        );
    }
    // FC3: the row that blocks Start fight shows in its place, after the last row that reads,
    // and marks itself once, in the warning tone; no other row does.
    // One label can give two nodes with one rect; a row is one place.
    let mut marked: Vec<(String, egui::Rect)> = d
        .nodes()
        .into_iter()
        .filter(|node| node.name.contains("not read") && node.name.contains("Line 20"))
        .map(|node| (node.name, node.rect))
        .collect();
    marked.dedup_by(|a, b| a.0 == b.0 && a.1.min.distance(b.1.min) < 1.0);
    let mut tones = Vec::new();
    d.each_painted_text(|text, _| {
        if text.galley.job.text.contains("Line 20 not read") {
            tones.extend(
                text.galley
                    .job
                    .sections
                    .iter()
                    .filter(|section| {
                        !text.galley.job.text[section.byte_range.clone()]
                            .trim()
                            .is_empty()
                    })
                    .map(|section| section.format.color),
            );
        }
    });
    let mark = d
        .nodes()
        .into_iter()
        .find(|node| node.name.ends_with("Line 20 not read"))
        .map(|node| node.rect);
    let last_read = d.place_of("6. Ember Cultist");
    d.observe(
        "iter-25-B-FC3",
        marked.len() == 1
            && marked[0].0 == format!("{} Line 20 not read", crate::design::icons::WARN)
            // iter-26-B FA5, AA2: the icon and the words, every painted part of the mark.
            && !tones.is_empty()
            && tones.iter().all(|tone| *tone == crate::design::colors().result.warning)
            && mark
                .zip(last_read)
                .is_some_and(|(mark, row)| mark.top() > row.bottom()),
        format!("Marked roster rows {marked:?}, mark colours {tones:?}, mark {mark:?} after row 6 {last_read:?}"),
    );
    d.state("flagged-roster-at-start-fight", Sizes::All);
    assert_eq!(fs::read_to_string(&roster_path).unwrap(), flagged_roster);
    fs::write(&roster_path, &roster_text).unwrap();
    d.click("Back to encounters");
    d.click("Reload from disk");
    run_saved_encounter(d, "Shrine of Cinders");
    fs::write(&roster_path, &flagged_roster).unwrap();
    d.app_mut()
        .encounters
        .run_mut()
        .unwrap()
        .guard_campaign(&campaign.path);
    d.settle();
    assert!(d.is_disabled("Begin combat"));
    d.expect("the change “HP lots” was not read");
    for size in [SMALL, DEFAULT, (1920.0, 1080.0)] {
        let (fits, placed) = d.at_size(size, |d| {
            let warning = d
                .nodes()
                .into_iter()
                .find(|node| node.name.contains("the change “HP lots” was not read"));
            let header = d.place_of("Begin combat").unwrap();
            let fits = warning
                .as_ref()
                .is_some_and(|node| node.rect.top() > header.bottom())
                && d.visible_text("Fix the unread row first.")
                && d.in_view("Initiative for Ember Cultist 2")
                && !d.visible_text("below, 1 needs initiative");
            // Round 22 AD2: a file line below the warning starts at its words. A wide column
            // keeps it beside the words instead, so 960 must take the line below.
            let file = d
                .nodes()
                .into_iter()
                .find(|node| node.name == roster_path.display().to_string() && node.in_view())
                .map(|node| node.rect);
            let placed = match (warning, file) {
                (Some(warning), Some(file)) if file.top() >= warning.rect.bottom() - 1.0 => {
                    super::file_links_at_message_words(
                        d,
                        "Ember Cultist · line 20: the change “HP lots”",
                        &roster_path,
                        "Encounters/Shrine of Cinders.md",
                    )
                }
                (Some(warning), Some(file)) if size != SMALL => (file.left()
                    >= warning.rect.right())
                .then_some(())
                .ok_or(format!("file {file:?} overlaps warning {:?}", warning.rect)),
                (warning, file) => Err(format!(
                    "warning {:?} and file line {file:?}",
                    warning.map(|node| node.rect)
                )),
            };
            (fits, placed)
        });
        d.observe(
            "iter-5-B-FD8",
            fits,
            format!("Setup warning follows the header and keeps roster room at {size:?}"),
        );
        d.observe(
            "iter-22-C-AD2",
            placed.is_ok(),
            format!("At {size:?}, the setup warning's file line: {placed:?}"),
        );
    }
    d.expect(
        "Ember Cultist · line 20: the change “HP lots” was not read. Use a number, such as HP +5.",
    );
    for size in [SMALL, DEFAULT, (1920.0, 1080.0)] {
        let fits = d.at_size(size, |d| {
            ["Kestrel Dunmoor", "Sable Harrowgate", "Ember Cultist 2"]
                .iter()
                .all(|name| {
                    let rows: Vec<_> = d
                        .nodes()
                        .into_iter()
                        .filter(|node| {
                            node.role == egui::accesskit::Role::Button
                                && node.name.starts_with(&format!("{name} ·"))
                        })
                        .collect();
                    !rows.is_empty() && rows.iter().all(|row| row.fully_in_view())
                })
        });
        d.observe(
            "iter7-B-AD1-FD1",
            fits,
            format!("The whole final setup row and the long party names fit at {size:?}"),
        );
    }
    // iter-25-R FF1: a row from the unread line marks its own fault; a clean row does not.
    let rows: Vec<String> = d
        .nodes()
        .into_iter()
        .filter(|node| node.role == egui::accesskit::Role::Button && node.name.contains(" · HP "))
        .map(|node| node.name)
        .collect();
    let marked: Vec<_> = rows
        .iter()
        .filter(|row| row.contains("Line 20 not read"))
        .collect();
    d.observe(
        "iter-25-R-FF1",
        !marked.is_empty()
            && marked.iter().all(|row| row.starts_with("Ember Cultist"))
            && rows
                .iter()
                .any(|row| row.starts_with("Kestrel Dunmoor") && !row.contains("not read"))
            && d.visible_text("Line\u{a0}20\u{a0}not\u{a0}read"),
        format!("Rows from line 20 say so, others do not: {marked:?} of {rows:?}"),
    );
    // iter-26-R FA11: the setup file line names the file by its path in the campaign, as the
    // encounter editor does.
    d.observe_both(
        "iter-26-R-FA11",
        "The setup file line says Encounters/Shrine of Cinders.md",
        |d| d.visible_text("Encounters/Shrine of Cinders.md"),
    );
    d.state("unread-roster-kept-in-setup", Sizes::All);
    assert_eq!(fs::read_to_string(&roster_path).unwrap(), flagged_roster);
    // Two unread rows name one file: its name and actions show once.
    let one_row = d
        .nodes()
        .iter()
        .filter(|node| node.name.contains("the change “HP lots” was not read"))
        .count();
    let mut two_flagged = flagged_roster.clone();
    let first = &splat::content::layout::Document::new(&flagged_roster)
        .table(splat::content::list_formats::ROSTER)
        .unwrap()
        .rows[0][3];
    two_flagged.replace_range(first.range.clone(), "HP lots");
    fs::write(&roster_path, &two_flagged).unwrap();
    d.app_mut()
        .encounters
        .run_mut()
        .unwrap()
        .guard_campaign(&campaign.path);
    d.settle();
    let warnings = d
        .nodes()
        .iter()
        .filter(|node| node.name.contains("the change “HP lots” was not read"))
        .count();
    let show_file = d
        .nodes()
        .iter()
        .filter(|node| node.name == "Show file")
        .count();
    d.observe(
        "iter-5-B-FD8-two-rows",
        one_row > 0 && warnings == 2 * one_row && show_file == 1,
        format!("Two unread rows: {warnings} warnings, {show_file} Show file"),
    );
    // Round 22 review: the one file line follows the last warning about the file, at its
    // words (or beside them, when a wide column has room).
    for size in [SMALL, DEFAULT, (1920.0, 1080.0)] {
        let placed = d.at_size(size, |d| {
            let mut faults: Vec<_> = d
                .nodes()
                .into_iter()
                .filter(|node| {
                    node.role == egui::accesskit::Role::Label
                        && node.name.contains("the change “HP lots” was not read")
                        && node.in_view()
                })
                .collect();
            faults.sort_by(|a, b| a.rect.top().total_cmp(&b.rect.top()));
            faults.dedup_by(|a, b| a.rect == b.rect);
            let files: Vec<_> = d
                .nodes()
                .into_iter()
                .filter(|node| node.name == roster_path.display().to_string())
                .map(|node| node.rect)
                .collect();
            let (Some(last), [file]) = (faults.last(), files.as_slice()) else {
                return Err(format!("{} warnings, file lines {files:?}", faults.len()));
            };
            if faults.len() != 2 {
                return Err(format!("{} warnings", faults.len()));
            }
            if file.top() >= last.rect.bottom() - 1.0 {
                super::file_links_at_message_words(
                    d,
                    &last.name,
                    &roster_path,
                    "Encounters/Shrine of Cinders.md",
                )
            } else if size != SMALL
                && file.left() >= last.rect.right()
                && file.top() >= last.rect.top() - 1.0
                && file.top() < last.rect.bottom()
            {
                Ok(())
            } else {
                Err(format!("file line {file:?}, last warning {:?}", last.rect))
            }
        });
        d.observe(
            "iter-22-C-AD2-two-rows",
            placed.is_ok(),
            format!("At {size:?}, two unread rows and their one file line: {placed:?}"),
        );
    }
    fs::write(&roster_path, &roster_text).unwrap();
    close_run(d);

    // iter-25-S FF2: the warning arrives while the DM is on Timeline, and the Add strip and the
    // list's first row keep their places.
    d.click(NAV_TIMELINE);
    let strip_top = |d: &Driver<'_>| d.place_of("Add Fight").map(|rect| rect.top());
    let strip_before = [SMALL, DEFAULT, (1920.0, 1080.0)].map(|size| d.at_size(size, strip_top));
    assert!(!d.has_containing("Line 9: "));
    let text = fs::read_to_string(&path).unwrap();
    fs::write(&path, text.replace("## Day 1:", "## Day 5:")).unwrap();
    d.app_mut().timeline.reload();
    d.click(NAV_TIMELINE);
    d.expect("Line 9: “Day 5” is day 1 in order.");
    let strip_after = [SMALL, DEFAULT, (1920.0, 1080.0)].map(|size| {
        d.at_size(size, |d| {
            // The warning sits below the strip, in the list column.
            let below = d
                .place_of("Line 9: “Day 5” is day 1 in order. Splat renumbers it when it saves.")
                .or_else(|| {
                    d.nodes()
                        .into_iter()
                        .find(|node| node.name.starts_with("Line 9: "))
                        .map(|node| node.rect)
                })
                .zip(d.place_of("Add Fight"))
                .is_some_and(|(warning, add)| warning.top() > add.bottom());
            (strip_top(d), below)
        })
    });
    d.observe(
        "iter-25-S-FF2",
        strip_before.iter().zip(&strip_after).all(|(before, (after, below))| {
            *below && before.zip(*after).is_some_and(|(a, b)| (a - b).abs() < 1.0)
        }),
        format!("The Add strip keeps its y with the warning below it: {strip_before:?} then {strip_after:?}"),
    );
    d.expect("Splat renumbers it when it saves.");
    d.observe_both(
        "i20-S3",
        "A Timeline flag names the file, displayed day, and repair routes",
        |d| {
            d.has("Edit in Obsidian")
                && d.has("Show file")
                && d.visible_text("Line 9: “Day 5”")
                && d.visible_text("Splat renumbers it when it saves.")
        },
    );
    // Round 22 FD1, FD5: the file line starts at the warning's words, and only it names the file.
    for size in [SMALL, DEFAULT, (1920.0, 1080.0)] {
        let placed = d.at_size(size, |d| {
            super::file_links_at_message_words(d, "Line 9: ", &path, crate::timeline::FILE)
                .and_then(|()| {
                    let named = d
                        .nodes()
                        .into_iter()
                        .filter(|node| node.name.starts_with("Line 9: "))
                        .map(|node| node.name)
                        .collect::<Vec<_>>();
                    (!named.is_empty()
                        && named
                            .iter()
                            .all(|name| !name.contains(crate::timeline::FILE)))
                    .then_some(())
                    .ok_or(format!("warnings {named:?} name the file"))
                })
        });
        d.observe(
            "iter-22-C-FD1",
            placed.is_ok(),
            format!("At {size:?}, the Timeline warning's file line: {placed:?}"),
        );
    }
    // Review of PR 340: over long content, the warning and its file line stay in view at the top
    // of the Timeline list and of the plan: they are below the pane, outside its scroll.
    d.aside(|d| {
        let flagged = fs::read_to_string(&path).unwrap();
        let session = d.app_mut().sessions.current();
        for _ in 0..40 {
            d.app_mut()
                .timeline
                .add(crate::timeline::Kind::Event, session);
        }
        d.app_mut().timeline.save();
        let long = fs::read_to_string(&path).unwrap();
        fs::write(&path, long.replace("## Day 1:", "## Day 5:")).unwrap();
        d.app_mut().timeline.reload();
        d.settle();
        d.click(NAV_TIMELINE);
        d.scroll_up_at(pos2(500.0, 300.0), 30);
        let list = super::file_links_at_message_words(d, "Line ", &path, crate::timeline::FILE);
        d.click(NAV_SESSIONS);
        show_plan(d);
        d.scroll_up_at(pos2(500.0, 300.0), 30);
        let plan = super::file_links_at_message_words(d, "Line ", &path, crate::timeline::FILE);
        d.observe(
            "round-28-S-flags-in-view",
            list.is_ok() && plan.is_ok(),
            format!("Over 40 more events, at the top: the list's file line {list:?}, the plan's {plan:?}"),
        );
        fs::write(&path, flagged).unwrap();
        d.app_mut().timeline.reload();
        d.settle();
        d.click(NAV_TIMELINE);
    });
    // Round 13 FD6: the flagged day keeps its Day icon and says what its heading was written as.
    d.observe_both(
        "iter-13-C-FD6",
        "The flagged day row keeps the Day icon and adds a muted warning word",
        |d| {
            let rows: Vec<_> = d
                .nodes()
                .into_iter()
                .filter(|node| node.name.ends_with(" · Written as Day 5"))
                .collect();
            let warn = crate::design::icons::WARN;
            rows.len() == 1
                && rows[0].name.starts_with("Day 1: ")
                && d.visible_text(&format!("{warn} Written as Day 5"))
                && {
                    let painted = d.painted_in(rows[0].rect);
                    painted.contains(crate::timeline::Kind::Day.icon())
                        && painted.matches(warn).count() == 1
                }
        },
    );
    // AD1: the clock shows minutes, so a ten-minute event ends at a different time.
    d.observe_both(
        "iter-12-C-AD1",
        "Starts, Ends and the header's Now show hours and minutes",
        |d| {
            d.visible_text("Starts: Day 1, 00:00")
                && d.visible_text("Ends: Day 1, 00:10")
                && d.visible_text("Now: Day 1, 00:10")
        },
    );
    d.state("timeline-item-edited-and-flagged-in-obsidian", Sizes::All);
    // Round 22 FD1 edge: with two flags, one file line follows the last flag, at its words.
    let one_flag = fs::read_to_string(&path).unwrap();
    fs::write(
        &path,
        format!("{one_flag}\n## Day 9: Later\n%% splat id=t-iter22-extra %%\n"),
    )
    .unwrap();
    d.app_mut().timeline.reload();
    d.wait_for_text("“Day 9” is day 2 in order.");
    for size in [SMALL, DEFAULT] {
        let placed = d.at_size(size, |d| {
            let last = d
                .nodes()
                .into_iter()
                .find(|node| node.name.contains("“Day 9” is day 2 in order."))
                .and_then(|node| {
                    node.name
                        .split_once(": ")
                        .map(|(line, _)| format!("{line}: "))
                })
                .ok_or("the second flag is not shown".to_owned())?;
            let files = d
                .nodes()
                .into_iter()
                .filter(|node| node.name == path.display().to_string())
                .count();
            super::file_links_at_message_words(d, &last, &path, crate::timeline::FILE).and_then(
                |()| {
                    (files == 1 && d.visible_text("Line 9: “Day 5”"))
                        .then_some(())
                        .ok_or(format!("{files} file lines"))
                },
            )
        });
        d.observe(
            "iter-22-C-FD1-two-flags",
            placed.is_ok(),
            format!("At {size:?}, two Timeline flags and their file line: {placed:?}"),
        );
    }
    fs::write(&path, &one_flag).unwrap();
    d.app_mut().timeline.reload();
    d.settle();
    assert!(!d.visible_text("“Day 9”"));
    d.app_mut().timeline.touch();
    d.app_mut().timeline.save();
    d.settle();
    d.observe_both(
        "i20-X23",
        "Saving renumbers the day and clears its flag",
        |d| !d.has_containing("Splat renumbers it when it saves."),
    );
    assert!(
        fs::read_to_string(&path)
            .unwrap()
            .contains("## Day 1: The crossing")
    );
    d.expect_same("Saving the corrected day returns to the original unflagged Timeline.");
    d.state("timeline-day-renumbered-on-save", Sizes::All);
}
