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
    crowded_banner(d, &folder, &note.path);
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
    // AB1: no step below may change a saved run's bytes.
    let run_bytes = |folder: &std::path::Path| {
        let mut files: Vec<_> = fs::read_dir(folder)
            .map(|entries| {
                entries
                    .filter_map(Result::ok)
                    .map(|entry| (entry.file_name(), fs::read(entry.path()).ok()))
                    .collect()
            })
            .unwrap_or_default();
        files.sort();
        files
    };
    let bytes_before = run_bytes(&run_folder);
    assert!(!bytes_before.is_empty());
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
    // AB1: a list that cannot be read says so, never that it is empty.
    let unread = |d: &Driver<'_>| {
        d.has(crate::encounters::UNFINISHED_UNREAD)
            && d.has(crate::encounters::ENDED_UNREAD)
            && !d.has_containing("No unfinished runs")
            && !d.has_containing("No ended fights yet")
    };
    d.observe(
        "iter-25-B-AB1",
        unread(d),
        "In progress and History say the run folder cannot be read",
    );
    d.state("run-folder-gone", Sizes::All);
    // A full reload reads the folder again, and still cannot.
    d.click("Reload from disk");
    d.settle();
    d.observe(
        "iter-25-B-AB1-reload",
        unread(d),
        "After a reload, the lists still say the run folder cannot be read",
    );
    // The synchronous refresh (Make it yours, a creature save, a group removal) says the same.
    let modules = d.app_mut().campaigns.modules().to_vec();
    d.app_mut().encounters.refresh_now(campaign, &modules);
    d.settle();
    d.observe(
        "iter-25-B-AB1-refresh",
        unread(d),
        "After a synchronous refresh, the lists still say the run folder cannot be read",
    );
    fs::rename(&moved, &run_folder).unwrap();
    d.app_mut().encounters.read_runs_again(campaign);
    d.settle();
    d.observe(
        "iter-25-B-AB1-back",
        !d.has(crate::encounters::UNFINISHED_UNREAD)
            && !d.has(crate::encounters::ENDED_UNREAD)
            && d.has_containing(&runs[0].encounter),
        "When the folder is back, its runs show again",
    );
    // A run folder whose parent cannot be looked in is not a folder that was never made: the
    // lists say it cannot be read, through a reload and a synchronous refresh. The parent here is
    // a plain file, so the system answers "not a directory", not "not found" (a permission
    // change would not hold in this sandbox).
    #[cfg(unix)]
    {
        let blocker = d.root.join("Run folder parent");
        fs::write(&blocker, "not a folder").unwrap();
        let moved_out = d.root.join("Runs kept aside");
        fs::rename(&run_folder, &moved_out).unwrap();
        std::os::unix::fs::symlink(blocker.join("Encounter Runs"), &run_folder).unwrap();
        d.click("Reload from disk");
        d.settle();
        let reloaded = unread(d);
        d.app_mut().encounters.refresh_now(campaign, &modules);
        d.settle();
        d.observe(
            "iter-25-B-AB1-parent",
            reloaded && unread(d),
            format!("A run folder under a parent that is not a folder says it cannot be read after a reload ({reloaded}) and a refresh"),
        );
        fs::remove_file(&run_folder).unwrap();
        fs::rename(&moved_out, &run_folder).unwrap();
        fs::remove_file(&blocker).unwrap();
        d.app_mut().encounters.read_runs_again(campaign);
        d.settle();
    }
    d.observe(
        "iter-25-B-AB1-bytes",
        run_bytes(&run_folder) == bytes_before && d.has_containing(&runs[0].encounter),
        "Every saved run's bytes are unchanged, and the runs show again",
    );
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

/// Review of PR 255, item 3: a recording alert and many session notices at once at 960x640.
/// The banner stays within a third of the window, the alert stays at its top, and the last
/// recovery choice is reached by the wheel and by the keyboard.
fn crowded_banner(d: &mut Driver<'_>, folder: &std::path::Path, note: &std::path::Path) {
    const ALERT: &str = "Processing “crowded” failed. What is done is kept.";
    const RELOAD: &str = "Reload and lose my draft";
    let base = d.window_size();
    d.set_base(SMALL);
    let copies = ["Session 00.md", "Session 000.md"].map(|name| folder.join(name));
    for copy in &copies {
        fs::copy(note, copy).unwrap();
    }
    fs::write(
        folder.join("Broken.md"),
        "---\nsession_id: not an id\n---\n",
    )
    .unwrap();
    d.app_mut().sessions.refresh();
    d.app_mut().sessions.simulate_alert(ALERT);
    d.app_mut().sessions.simulate_conflict();
    d.settle();
    d.expect(RELOAD);
    let (_, height) = d.window_size();
    let alert = d.place_of(ALERT);
    let reload_before = d.place_of(RELOAD);
    d.observe(
        "iter-25-S-banner-crowded",
        alert.is_some_and(|alert| alert.top() >= height * 2.0 / 3.0 - crate::design::M)
            && !d.in_view(RELOAD)
            && d.has_containing("claim to be the same session")
            && d.has_containing("cannot read the properties"),
        format!("At 960x640 the crowded banner holds the alert at its top {alert:?}, and Reload is below its edge {reload_before:?}"),
    );
    let at = alert.map_or(pos2(400.0, height - 40.0), |alert| alert.center());
    d.scroll_at(at, 10);
    let reload_after = d.place_of(RELOAD);
    d.observe(
        "iter-25-S-banner-wheel",
        d.in_view(RELOAD)
            && reload_before
                .zip(reload_after)
                .is_some_and(|(before, after)| after.top() < before.top() - 1.0),
        format!("The wheel moves the banner: Reload {reload_before:?} then {reload_after:?}"),
    );
    d.scroll_up_at(at, 10);
    assert!(d.in_view(ALERT) && !d.in_view(RELOAD));
    let reached = d.tab_to(RELOAD);
    d.observe(
        "iter-25-S-banner-keyboard",
        reached && d.in_view(RELOAD),
        "Tab reaches Reload and lose my draft, and it scrolls into view",
    );
    d.click("Overwrite with my draft");
    d.click("Dismiss this error");
    for copy in &copies {
        fs::remove_file(copy).unwrap();
    }
    fs::remove_file(folder.join("Broken.md")).unwrap();
    d.app_mut().sessions.refresh();
    d.settle();
    d.set_base(base);
}
