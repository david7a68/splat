//! F3 files remain readable after Obsidian edits. Reading never converts them.
use super::*;
use std::fs;

pub fn run(d: &mut Driver<'_>) {
    let campaign = d.app_mut().campaigns.active_campaign().unwrap();
    let folder = crate::sessions::folder(&campaign).unwrap();
    let path = folder.join(crate::timeline::FILE);
    let before = fs::read_to_string(&path).unwrap();
    d.click(NAV_CAMPAIGN);
    d.click("Convert files…");
    d.click("File to convert");
    d.click_containing("/Timeline.md");
    d.expect("BEFORE");
    d.expect("AFTER");
    let converted = crate::timeline::formats::convert(&before, &Default::default()).unwrap();
    let source_lines: std::collections::BTreeSet<_> =
        before.lines().chain(converted.lines()).collect();
    check_preview(
        d,
        &source_lines,
        [&before, &converted],
        true,
        "iter-5-A-timeline",
    );
    d.state("timeline-conversion-preview", Sizes::All);
    // Where the later change's line sits in the small window, before and after the click.
    let later = |d: &mut Driver<'_>| {
        d.at_size(SMALL, |d| {
            d.nodes()
                .iter()
                .filter(|node| node.name == "## Day 1: The crossing" && node.in_view())
                .map(|node| node.rect.top().round() as i32)
                .min()
        })
    };
    let before_click = later(d);
    d.click("Next difference");
    d.expect("## Day 1: The crossing");
    let after_click = later(d);
    d.observe(
        "iter-4-D-FD8-next",
        after_click.is_some() && after_click != before_click,
        format!(
            "Next difference moves the small preview to the later change: {before_click:?} -> {after_click:?}"
        ),
    );
    assert!(after_click.is_some() && after_click != before_click);
    d.expect_same("Next difference moves the small preview. At larger sizes all aligned rows already fit, so its new destination needs no scroll.");
    d.state("timeline-next-change-group", Sizes::All);
    d.click("Cancel");
    assert_eq!(fs::read_to_string(&path).unwrap(), before);

    let after = format!(
        "{}\nThe toll keeper has a silver bell.\n",
        crate::timeline::formats::convert(&before, &Default::default()).unwrap()
    );
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
    d.expect("None chosen · 3 needed");
    d.expect("Arena Final, Bridge Ambush, Lost Patrol, Rival Team and Shrine of Cinders use its creatures.");
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
    d.state("campaign-flagged-rule-choice", Sizes::All);
    assert_eq!(fs::read_to_string(&manifest).unwrap(), flagged);
    d.click("Choose in Rules and runs");
    d.expect("Combat initiative choice");
    d.expect("Roll NPC initiative");
    d.state("rules-and-runs-flagged-choice", Sizes::All);
    d.click("Roll NPC initiative");
    d.click("Individually");
    d.click("Save run settings");
    d.expect("No changes");
    d.observe_both(
        "W12 rule repair",
        "Saving the chosen rule clears its warning",
        |d| !d.has_containing("Imaginary initiative") && d.visible_text("Individually"),
    );
    d.state("rules-and-runs-choice-repaired", Sizes::All);
    fs::write(&manifest, &after).unwrap();

    d.click(NAV_ENCOUNTERS);
    d.click_starting("GROUPS");
    d.expect("2 of 3 can join");
    d.expect("Scout (as Goblin Warrior) needs fixing");
    d.state("flagged-group-in-groups-list", Sizes::All);
    d.click(NAV_SESSIONS);
    new_session(d, "Bridge watch");
    show_plan(d);
    d.click("Add a group");
    d.click("Plan Bridge guards");
    d.expect("2 of 3 can join");
    d.state("flagged-group-in-the-plan", Sizes::All);
    d.click(NAV_ENCOUNTERS);
    run_saved_encounter(d, "Shrine of Cinders");
    fight_tab(d, "Arrivals");
    d.expect("Scout (as Goblin Warrior) needs fixing");
    d.expect("Edit group");
    d.state("flagged-group-in-arrivals", Sizes::All);
    close_run(d);
    d.click(NAV_ENCOUNTERS);
    d.click("Edit group Bridge guards");
    d.expect("Line 15: Changes \"HP lots\" not read.");
    d.expect("Goblin Warrior");
    d.observe_both(
        "F3 member warning",
        "The named warning is below its member summary and before the next member",
        |d| {
            let nodes = d.nodes();
            let find = |name: &str| nodes.iter().find(|node| node.name == name);
            let first = find("1. Scout");
            let next = find("2. Goblin Warrior");
            let warning = find("Line 15: Changes \"HP lots\" not read.");
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
    d.expect("Edit in Obsidian");
    d.expect("Show file");
    d.expect("Changes for Scout");
    d.state("group-flagged-changes-cell", Sizes::All);
    let text = fs::read_to_string(campaign.path.join("Encounter Groups/Bridge guards.md")).unwrap();
    assert!(text.contains("HP lots | Keep this row"));
    d.click("More actions for Scout");
    d.expect("Remove member");
    d.state("flagged-member-menu", Sizes::All);
    d.press(Key::Escape);
    d.click("Changes for Scout");
    d.expect("Display name");
    d.expect("For example: HP +5, AC +2, Speed 40 ft.");
    d.state("group-repair-changes-cell", Sizes::All);
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
    let mut flagged_roster = roster_text.clone();
    let table = splat::content::layout::Document::new(&roster_text)
        .table(splat::content::list_formats::ROSTER)
        .unwrap();
    let cell = &table.rows.last().unwrap()[3];
    flagged_roster.replace_range(cell.range.clone(), "HP lots");
    fs::write(&roster_path, &flagged_roster).unwrap();
    d.click("Reload from disk");
    open_saved_encounter(d, "Shrine of Cinders");
    d.expect("Changes: HP lots not read");
    assert!(d.is_disabled("Start fight"));
    assert!(!d.app_mut().encounters.has_run());
    d.expect("Edit in Obsidian");
    d.expect("Show file");
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
    d.expect("Changes: HP lots not read");
    d.state("unread-roster-kept-in-setup", Sizes::All);
    assert_eq!(fs::read_to_string(&roster_path).unwrap(), flagged_roster);
    fs::write(&roster_path, &roster_text).unwrap();
    close_run(d);

    let text = fs::read_to_string(&path).unwrap();
    fs::write(&path, text.replace("## Day 1:", "## Day 5:")).unwrap();
    d.app_mut().timeline.reload();
    d.click(NAV_TIMELINE);
    d.expect("Timeline.md, line 9: \"Day 5\" is day 1 in order.");
    d.expect("Splat renumbers it when it saves.");
    d.observe_both(
        "i20-S3",
        "A Timeline flag names the file, displayed day, and repair routes",
        |d| {
            d.has("Edit in Obsidian")
                && d.has("Show file")
                && d.visible_text("Timeline.md, line 9:")
                && d.visible_text("Splat renumbers it when it saves.")
        },
    );
    d.state("timeline-item-edited-and-flagged-in-obsidian", Sizes::All);
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

/// Measure the painted source labels against the actual scroll view at every capture size.
pub(super) fn check_preview(
    d: &mut Driver<'_>,
    source: &std::collections::BTreeSet<&str>,
    text: [&str; 2],
    long: bool,
    id: &str,
) {
    for size in [SMALL, DEFAULT, LARGE] {
        let passed = d.at_size(size, |d| {
            let nodes = d.nodes();
            let panes: Vec<_> = nodes.iter().filter(|node| {
                ["Before source pane", "After source pane"].contains(&node.name.as_str())
            }).collect();
            assert_eq!(panes.len(), 2, "Both source viewports must exist");
            let results: Vec<_> = panes.iter().map(|pane| {
                let rows: Vec<_> = nodes.iter().filter(|node| {
                    node.role == egui::accesskit::Role::Label
                        && node.in_view()
                        && (source.contains(node.name.as_str()) || node.name == "No line")
                        && node.rect.left() >= pane.rect.left() - 1.0
                        && node.rect.left() < pane.rect.right()
                        && node.rect.center().y >= pane.rect.top()
                        && node.rect.top() < pane.rect.bottom()
                }).collect();
                assert!(!rows.is_empty(), "Each source pane must paint rows");
                let last = rows.iter().map(|node| node.rect.bottom()).fold(f32::MIN, f32::max);
                let bottom_gap = pane.rect.bottom() - last;
                let whole = rows.iter().all(|node| {
                    (node.shown.height() - node.rect.height()).abs() < 1.0
                });
                let side = usize::from(pane.name == "After source pane");
                let full = text[side].lines().filter(|line| !line.is_empty())
                    .all(|line| rows.iter().any(|row| row.name == line));
                let height = if long {
                    // At 960x640, the header and actions take precedence over the 60% goal.
                    full || pane.rect.height() >= if size == SMALL { size.1 * 0.4 } else { size.1 * 0.6 - 24.0 }
                } else {
                    // A short source ends immediately above the reserved scroll bar.
                    bottom_gap < 1.0
                };
                let converted_visible = !long || pane.name != "After source pane" || rows.iter().any(|row| {
                    row.name.starts_with("## Day ") || row.name.starts_with("- Travel:")
                });
                eprintln!("{id} {size:?} {}: height {}, bottom gap {bottom_gap}, whole {whole}, converted {converted_visible}", pane.name, pane.rect.height());
                whole && bottom_gap.abs() < 1.0 && height && converted_visible
            }).collect();
            results.iter().all(|passed| *passed) && d.visible_text("Cancel") && d.visible_text("Convert files")
        });
        d.observe(id, passed, format!("Aligned source rows fill both panes, with converted text and actions visible at {}x{}", size.0, size.1));
    }
}
