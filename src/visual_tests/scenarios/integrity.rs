//! External edits keep their bytes and leave a visible recovery choice.
use super::*;
use std::fs;

pub fn run(d: &mut Driver<'_>) {
    let campaign = d.app_mut().campaigns.active_campaign().unwrap();
    let entries = splat::content::read_directory(&campaign.path).unwrap().0;
    let entry = entries
        .iter()
        .find(|entry| entry.name == "Shrine of Cinders")
        .unwrap();
    let path = campaign.path.join(&entry.path);
    let before = fs::read_to_string(&path).unwrap();
    let catalogue =
        crate::content_workspace::Catalogue::load(d.app_mut().campaigns.modules(), Some(&campaign))
            .unwrap();
    // Start with saved references so the comparison isolates the name edit.
    let source = splat::content::list_formats::write_references(
        &before,
        catalogue.entries.values().map(|entry| &entry.entry),
    )
    .unwrap();
    fs::write(&path, &source).unwrap();
    open_saved_encounter(d, "Shrine of Cinders");
    d.clear_text_in("Encounter name");
    d.type_into("Encounter name", "Cinders at dawn");
    let disk = source.replace("splat_schema: 2", "tags: [dawn]\nsplat_schema: 2");
    fs::write(&path, format!("{disk}\nDM's outside account.\n")).unwrap();
    d.press_ctrl(Key::S);
    d.expect("Overwrite with my draft");
    d.expect("Reload and lose my draft");
    d.observe_both(
        "i19-E comparison",
        "Only the encounter name differs in the disk's layout",
        |d| {
            d.visible_text("1 line differs")
                && d.visible_text("YOUR DRAFT")
                && d.visible_text("IN THE FILE NOW")
        },
    );
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
    d.expect(&format!(
        "{} and Session 0.md claim to be the same session.",
        note.path.file_name().unwrap().to_string_lossy()
    ));
    d.state("session-copy", Sizes::All);
    d.click("Make Session 0.md a separate session");
    d.expect("Undo separate session");
    d.expect("Session 0.md is now a separate session.");
    d.state("session-copy-repaired", Sizes::All);
    // iter-25-S AB2 caller: at 1920 the open-session warning and its action stay inside the
    // list measure, and the action starts at the warning's words.
    let capped = d.at_size((1920.0, 1080.0), |d| {
        let nodes = d.nodes();
        let words = nodes
            .iter()
            .find(|node| node.name.contains("is not the session being played") && node.in_view())
            .map(|node| node.rect);
        let action = nodes
            .iter()
            .find(|node| {
                node.in_view()
                    && (node.name == "Return to the session being played"
                        || node.name.starts_with("Open "))
                    && words.is_some_and(|words| node.rect.top() >= words.bottom() - 1.0)
            })
            .map(|node| node.rect);
        (words, action)
    });
    d.observe(
        "iter-25-S-AB2",
        capped.0.zip(capped.1).is_some_and(|(words, action)| {
            words.width() <= crate::design::LIST_MEASURE + 1.0
                && (action.left() - words.left()).abs() < 1.0
                && action.right() <= words.left() + crate::design::LIST_MEASURE + 1.0
        }),
        format!("The open-session warning and its action at 1920: {capped:?}"),
    );
    d.observe(
        "D9",
        crate::sessions::list_notes(&folder).len() == 2,
        "Both copies have a session after making a separate entry",
    );
    d.click("Undo separate session");
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
    d.observe_both("i19-S6", "Missing audio has no size", |d| {
        !d.has_containing("Audio removed ·")
    });
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
    // Outside prose stays in the file when timeline values save.
    d.app_mut().timeline.run_open = true;
    d.app_mut().timeline.save();
    d.app_mut().timeline.run_open = false;
    let path = folder.join("Timeline.md");
    let text = format!("{}DM's paragraph.\n", fs::read_to_string(&path).unwrap());
    fs::write(&path, &text).unwrap();
    d.app_mut().timeline.reload();
    d.app_mut().timeline.touch();
    d.app_mut().timeline.save();
    d.settle();
    assert_eq!(fs::read_to_string(&path).unwrap(), text);
    d.state("timeline-keeps-outside-prose", Sizes::All);
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
    // FB6: the run folder's fault has the session folder's wording.
    d.observe(
        "iter-23-Q-FB6",
        d.has_containing("Run folder not found:")
            && d.has_containing("It may have been moved or renamed."),
        "A missing run folder says it may have been moved or renamed",
    );
    d.observe_both(
        "i19-P2",
        "A missing run folder leaves no Resume or paused run",
        |d| !d.has("Resume") && !d.has_containing("Paused run ·"),
    );
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
            let action = nodes.iter().find(|node| node.name == "Choose folder");
            !d.has_containing("1 session")
                && d.visible_text("It may have been moved or renamed.")
                && !d.has_containing("Choose folder.")
                && message.zip(action).is_some_and(|(message, action)| {
                    // Beside the words, or right below them and aligned with them, as
                    // `message_with_action` places it when the line is too narrow.
                    let (m, a) = (message.rect, action.rect);
                    m.y_range().contains(a.center().y)
                        || (a.top() >= m.bottom()
                            && a.top() - m.bottom() <= crate::design::M
                            && (a.left() - m.left()).abs() <= 1.0)
                })
        },
    );
    d.observe_both(
        "iter-13-C-FD3",
        "Choose folder after the session folder warning is a bordered secondary action",
        |d| {
            d.in_view("Choose folder")
                && d.control_frame("Choose folder").is_some_and(|frame| {
                    frame.stroke.width > 0.0
                        && frame.stroke.color == crate::design::colors().field_border
                })
        },
    );
    d.observe_both(
        "i19-S5",
        "The missing session folder owns its rail link and recovery",
        |d| {
            d.in_view("Session folder not found")
                && !d.has("Timeline not readable")
                && !d.has("New session")
        },
    );
    d.state("session-folder-gone", Sizes::All);
}
