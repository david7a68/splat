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
    d.state("campaign-package-off-in-obsidian", Sizes::All);
    assert_eq!(fs::read_to_string(&manifest).unwrap(), after);

    d.click(NAV_ENCOUNTERS);
    d.click_starting("GROUPS");
    d.click("Edit group Bridge guards");
    d.expect("Changes: HP lots not read");
    d.expect("Goblin Warrior");
    d.state("group-flagged-changes-cell", Sizes::All);
    let text = fs::read_to_string(campaign.path.join("Encounter Groups/Bridge guards.md")).unwrap();
    assert!(text.contains("HP lots | Keep this row"));
}
