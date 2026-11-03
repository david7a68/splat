//! External edits keep their bytes and leave a visible recovery choice.
use super::*;
use std::fs;

pub fn run(d: &mut Driver<'_>) {
    open_saved_encounter(d, "Shrine of Cinders");
    d.clear_text_in("Encounter name");
    d.type_into("Encounter name", "Cinders at dawn");
    let campaign = d.app_mut().campaigns.active_campaign().unwrap();
    let entries = splat::content::read_directory(&campaign.path).unwrap().0;
    let entry = entries
        .iter()
        .find(|entry| entry.name == "Shrine of Cinders")
        .unwrap();
    let path = campaign.path.join(&entry.path);
    let disk = fs::read_to_string(&path)
        .unwrap()
        .replace("splat_schema: 1", "tags: [dawn]\nsplat_schema: 1");
    fs::write(&path, format!("{disk}\nDM's outside account.\n")).unwrap();
    d.press_ctrl(Key::S);
    d.expect("Overwrite with my draft");
    d.expect("Reload and lose my draft");
    d.state("encounter-disk-comparison", Sizes::All);
    d.click("Overwrite with my draft");
    d.expect("Saved");
    d.observe(
        "D8",
        fs::read_to_string(&path).unwrap().contains("tags: [dawn]"),
        "Keeping the draft keeps the disk's unknown properties",
    );
    d.click("Back to encounters");

    d.click(NAV_SESSIONS);
    new_session(d, "A long meeting name");
    let folder = crate::sessions::folder(&campaign).unwrap();
    let note = crate::sessions::list_notes(&folder).remove(0);
    let copy = folder.join("Session 0.md");
    fs::copy(&note.path, &copy).unwrap();
    d.app_mut().sessions.refresh();
    d.settle();
    d.expect("Give the copy a new identity");
    d.state("session-copy", Sizes::All);
    d.click_starting("New identity:");
    d.expect("Undo new identity");
    d.observe(
        "D9",
        crate::sessions::list_notes(&folder).len() == 2,
        "Both copies have a session after New identity",
    );
    d.click("Undo new identity");
    d.observe(
        "D9",
        crate::sessions::list_notes(&folder)[0].path == note.path,
        "Undo keeps the original as the known session",
    );
    fs::remove_file(copy).unwrap();
    d.app_mut().sessions.refresh();
    recording_without_audio(d, &folder, note.id);
    timeline_body(d, &folder);
    missing_folders(d, &campaign, &folder);
}

fn recording_without_audio(d: &mut Driver<'_>, folder: &std::path::Path, session: uuid::Uuid) {
    let made = crate::sessions::recordings_folder(folder, session);
    fs::create_dir_all(&made).unwrap();
    fs::write(
        made.join("Evening.transcript.md"),
        "# Raw transcript\n\nThe party crossed the bridge.",
    )
    .unwrap();
    fs::write(
        made.join("Evening.summary.md"),
        "# Summary\n\nThe party reached the mill.",
    )
    .unwrap();
    d.app_mut().sessions.select(None);
    d.app_mut().sessions.select(Some(session));
    d.settle();
    d.click_starting("Recordings");
    d.expect("Audio removed");
    d.expect("Summary");
    d.expect("Raw transcript");
    d.state("recording-audio-removed", Sizes::All);
}

fn timeline_body(d: &mut Driver<'_>, folder: &std::path::Path) {
    d.click(NAV_TIMELINE);
    d.app_mut()
        .timeline
        .add(crate::timeline::Kind::Travel, None);
    d.app_mut().timeline.events[0].notes = (1..=48)
        .map(|day| format!("Day {day}: The party followed the river."))
        .collect::<Vec<_>>()
        .join("\n");
    d.app_mut().timeline.save();
    let path = folder.join("Timeline.md");
    let text = format!("{}\nDM's paragraph.\n", fs::read_to_string(&path).unwrap());
    fs::write(&path, &text).unwrap();
    d.app_mut().timeline.reload();
    d.app_mut().timeline.touch();
    d.app_mut().timeline.save();
    d.settle();
    d.expect("Overwrite with my timeline");
    d.expect("Reload and lose my timeline");
    d.observe_both(
        "D1 comparison",
        "The disk body is visible at the first difference beside the draft",
        |d| {
            d.visible_text("YOUR DRAFT")
                && d.visible_text("IN THE FILE NOW")
                && d.visible_text("DM's paragraph.")
                && d.painted_left("Overwrite with my timeline")
                    .zip(d.painted_left("Reload and lose my timeline"))
                    .is_some_and(|(overwrite, reload)| overwrite < reload)
        },
    );
    d.observe(
        "D1",
        fs::read_to_string(&path).unwrap() == text,
        "An edited body stays on disk until the DM chooses",
    );
    d.state("timeline-body-conflict", Sizes::All);
    d.click("Overwrite with my timeline");
}

fn missing_folders(
    d: &mut Driver<'_>,
    campaign: &crate::persistence::Campaign,
    folder: &std::path::Path,
) {
    let runs = d.app_mut().encounters.saved_runs();
    let run_folder = runs[0].path.parent().unwrap().to_owned();
    let moved = campaign.path.join("Moved runs");
    fs::rename(&run_folder, &moved).unwrap();
    d.click(NAV_ENCOUNTERS);
    d.app_mut().encounters.read_runs_again(campaign);
    d.settle();
    d.expect("Run folder not found");
    d.expect("Choose folder");
    d.state("run-folder-gone", Sizes::All);
    fs::rename(&moved, run_folder).unwrap();
    d.click(NAV_SESSIONS);
    fs::rename(folder, campaign.path.join("Moved sessions")).unwrap();
    d.app_mut().sessions.refresh();
    d.settle();
    d.expect("Session folder not found");
    d.expect("Choose folder");
    d.observe_both(
        "D10 recovery",
        "A missing folder has no session count or repeated action",
        |d| {
            let nodes = d.nodes();
            let message = nodes
                .iter()
                .find(|node| node.name.starts_with("Session folder not found:"));
            let action = nodes.iter().find(|node| node.name == "Choose folder…");
            !d.has_containing("1 session")
                && d.visible_text("It may have been moved or renamed.")
                && !d.has_containing("Choose folder.")
                && message.zip(action).is_some_and(|(message, action)| {
                    message.rect.y_range().contains(action.rect.center().y)
                })
        },
    );
    d.state("session-folder-gone", Sizes::All);
}
