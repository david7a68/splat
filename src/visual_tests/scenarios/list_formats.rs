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
    d.click("Timeline.md");
    d.expect("BEFORE");
    d.expect("AFTER");
    d.state("timeline-conversion-preview", Sizes::All);
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
    d.expect("is not a rule in this campaign");
    d.state("campaign-flagged-rule-choice", Sizes::All);
    assert_eq!(fs::read_to_string(&manifest).unwrap(), flagged);
    fs::write(&manifest, &after).unwrap();

    d.click(NAV_ENCOUNTERS);
    d.click_starting("GROUPS");
    d.click("Edit group Bridge guards");
    d.expect("Bridge guards.md, line 15: Changes \"HP lots\" not read.");
    d.expect("Goblin Warrior");
    d.observe_both(
        "F3 member warning",
        "The named warning is below its member summary and before the next member",
        |d| {
            let nodes = d.nodes();
            let find = |name: &str| nodes.iter().find(|node| node.name == name);
            let first = find("1. Goblin Warrior");
            let next = find("2. Goblin Warrior");
            let warning = find("Bridge guards.md, line 15: Changes \"HP lots\" not read.");
            let summary = find("Count 1 · CR 1/4 · HP 10 · AC 15");
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
    d.expect("XP 100 + unknown");
    d.expect("Edit in Obsidian");
    d.expect("Show file");
    d.expect("Edit flagged member");
    d.state("group-flagged-changes-cell", Sizes::All);
    let text = fs::read_to_string(campaign.path.join("Encounter Groups/Bridge guards.md")).unwrap();
    assert!(text.contains("HP lots | Keep this row"));
    d.click("More actions for flagged member");
    d.expect("Remove member");
    d.state("flagged-member-menu", Sizes::All);
    d.press(Key::Escape);
    d.click("Edit flagged member");
    d.expect("Changes cell");
    d.state("group-repair-changes-cell", Sizes::All);
    d.replace_text_in("Changes cell", "HP +5");
    d.click("Done");
    d.click("Save group");
    let saved =
        fs::read_to_string(campaign.path.join("Encounter Groups/Bridge guards.md")).unwrap();
    assert_eq!(saved, text.replace("HP lots", "HP +5"));
    d.state("group-changes-cell-repaired", Sizes::All);

    let text = fs::read_to_string(&path).unwrap();
    fs::write(&path, text.replace("## Day 1:", "## Day 5:")).unwrap();
    d.app_mut().timeline.reload();
    d.click(NAV_TIMELINE);
    d.expect("Day number comes from the order of days");
    d.state("timeline-item-edited-and-flagged-in-obsidian", Sizes::All);
}
