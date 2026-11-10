//! F1 conversion is explicit, previewed, and recoverable without replacing edits.
use super::*;
use std::fs;

pub fn run(d: &mut Driver<'_>) {
    let root = d.root.join("campaigns/emberfall");
    let path = root.join("NPCs/Rook.md");
    let before = fs::read_to_string(&path).unwrap();
    d.click(NAV_WORLD);
    d.click_containing("Malformed · ");
    let warning = "Status “[alive]” not read (line 7). Use a short word, such as alive.";
    d.expect(warning);
    d.expect("A note for tonight.");
    d.observe_both(
        "iter-11-D-FD6",
        "The unread Status warning sits once below the file line, above the values",
        |d| {
            for name in [warning, "Show file", "Change Status"] {
                d.expect(name);
            }
            let warnings = d
                .nodes()
                .into_iter()
                .filter(|node| node.name == warning && node.role == egui::accesskit::Role::Label)
                .count();
            let file = d.place_of("Show file").unwrap();
            let message = d.place_of(warning).unwrap();
            let status = d.place_of("Change Status").unwrap();
            // Neighbour: the Status row still says not read.
            warnings == 1
                && message.top() >= file.bottom()
                && message.bottom() <= status.top()
                && d.painted_in(egui::Rect::from_min_max(
                    egui::pos2(message.left(), status.top()),
                    status.max,
                ))
                .contains("not read")
        },
    );
    d.state("flagged-malformed-value", Sizes::All);
    assert_eq!(
        fs::read_to_string(&path).unwrap(),
        before,
        "Reading never converts a file"
    );

    go_library(d);
    search_library(d, "NPCs/Rook.md");
    d.click_containing("NPCs/Rook.md");
    d.expect("NPCs/Rook.md uses the old layout. Splat no longer reads it.");
    d.observe_both(
        "Old-layout reader",
        "Front matter stays out of prose and the unreadable label appears once",
        |d| {
            !d.nodes()
                .iter()
                .any(|node| node.name.contains("splat_schema:"))
                && d.nodes()
                    .iter()
                    .filter(|node| {
                        node.role == egui::accesskit::Role::Label
                            && node.in_view()
                            && node.name.contains("Cannot be read")
                    })
                    .count()
                    == 1
        },
    );
    d.state("old-layout-flagged-in-library", Sizes::All);
    assert_eq!(fs::read_to_string(&path).unwrap(), before);
    d.click("Convert files…");
    d.expect("NPCs/Rook.md");
    d.expect(
        before
            .lines()
            .find(|line| line.starts_with("splat_id:"))
            .unwrap(),
    );
    d.click("Cancel");
    d.click(NAV_CAMPAIGN);
    d.expect("4 files use the old layout.");
    d.expect("Convert Cinder Wolf.md first.");
    assert!(d.is_disabled("Save selections"));
    d.observe_both(
        "Old-layout campaign",
        "One conversion notice replaces repair rows and empty package separators",
        |d| {
            !d.has("Repair")
                && d.nodes()
                    .iter()
                    .filter(|node| {
                        node.role == egui::accesskit::Role::Label
                            && node.in_view()
                            && node.name == "4 files use the old layout."
                    })
                    .count()
                    == 1
                && !d
                    .nodes()
                    .iter()
                    .any(|node| node.name.trim_end().ends_with('·'))
        },
    );
    d.state_prepared("old-layout-flagged-in-campaign", Sizes::All, |d| {
        let actions: Vec<_> = d
            .nodes()
            .into_iter()
            .filter(|node| node.name == "Convert files…")
            .collect();
        d.observe(
            "iter-6-C-SD1",
            actions.len() == 1 && actions[0].in_view(),
            "Campaign keeps exactly one conversion action beside its combined warning",
        );
    });
    d.click_beside_exact("4 files use the old layout.", "Convert files…");
    d.click("File to convert");
    d.click("NPCs/Rook.md");
    d.expect("BEFORE");
    d.expect("AFTER");
    let identity = before
        .lines()
        .find(|line| line.starts_with("splat_id:"))
        .unwrap();
    let world = before
        .lines()
        .find(|line| line.starts_with("splat_world:"))
        .unwrap();
    d.observe_both(
        "F1 preview",
        "The comparison opens on the schema change with full identity and World lines",
        |d| {
            d.visible_text("splat_schema: 1")
                && d.visible_text("splat_schema: 2")
                && d.visible_text(identity)
                && d.nodes().iter().any(|node| {
                    node.name == world
                        && node.in_view()
                        && node.rect.height() <= crate::design::Type::Mono.line()
                })
                && d.visible_text("1 rule · 1 World note")
                && d.visible_text("Next difference")
                && d.visible_text("Convert files")
                && d.visible_text("Cancel")
        },
    );
    let converted = splat::content::formats::convert("NPCs/Rook.md", &before).unwrap();
    let source_lines = before.lines().chain(converted.lines()).collect();
    super::list_formats::check_preview(
        d,
        &source_lines,
        [&before, &converted],
        false,
        "iter-5-A-short",
    );
    d.state("convert-files-preview", Sizes::All);
    d.click("Cancel");
    assert_eq!(fs::read_to_string(&path).unwrap(), before);
    // Preview is read-only. A race at conversion is reported beside the result.
    d.click(NAV_LIBRARY);
    d.click_containing("files use the old layout");
    fs::write(&path, format!("{before}Outside edit after preview.\n")).unwrap();
    d.click("Convert files");
    d.expect("1 file now uses the new layout. 1 file was skipped.");
    d.expect("Undo convert");
    d.expect(
        "NPCs/Rook.md changed on disk after the preview. Open Convert files again to convert it.",
    );
    d.observe_both(
        "W11 conversion result",
        "One batch Undo and the skipped file stay with the conversion result",
        |d| {
            d.nodes()
                .iter()
                .filter(|node| node.in_view() && node.name.starts_with("Undo"))
                .count()
                == 1
                && d.visible_text("Undo convert")
                && d.visible_text("NPCs/Rook.md changed on disk after the preview. Open Convert files again to convert it.")
        },
    );
    d.expect("Convert files…");
    assert!(!d.has("Engine support"));
    assert!(!d.has("Source and identifiers"));
    d.state_prepared("converted-with-skipped-file-and-one-undo", Sizes::All, |d| {
        let converts = d
            .nodes()
            .iter()
            .filter(|node| node.in_view() && node.name == "Convert files…")
            .count();
        d.observe(
            "iter-13-E-FD5",
            converts == 1
                && d.visible_text("NPCs/Rook.md uses the old layout. Splat no longer reads it."),
            format!("One Convert files… for the one skipped file, and the reader keeps its sentence: {converts} buttons"),
        );
    });
    d.click_beside_exact(
        "NPCs/Rook.md changed on disk after the preview. Open Convert files again to convert it.",
        "Convert files…",
    );
    d.expect("File to convert");
    d.state("skipped-file-conversion-reopened", Sizes::All);
    d.click("Cancel");
    d.click("Undo convert");
    d.expect("Conversion undone.");
    d.observe_both(
        "W12 Undo convert",
        "Undo restores the old layouts and clears the skipped result",
        |d| !d.has("Undo convert") && !d.has("FILES SKIPPED"),
    );
    d.state("conversion-undone", Sizes::All);
    assert!(
        fs::read_to_string(&path)
            .unwrap()
            .contains("Outside edit after preview.")
    );
    go_encounters(d);
    resume_run_from_cold_start(d);
    go_library(d);
    let old = d
        .nodes()
        .into_iter()
        .find(|node| node.name.ends_with("files use the old layout"))
        .unwrap()
        .name;
    assert!(d.is_disabled(&old));
    assert!(d.tooltip_contains(&old, "Close the run before converting files."));
    assert!(d.is_disabled("Convert files…"));
    assert!(d.tooltip_contains("Convert files…", "Close the run before converting files."));
    d.state("old-layout-notice-blocked-by-run", Sizes::All);
    // Test the package-menu action without a second Convert button behind its popup.
    search_library(d, "Aboleth");
    d.click("Packages");
    assert!(d.is_disabled("Convert files…"));
    assert!(d.tooltip_contains("Convert files…", "Close the run before converting files."));
    d.state("convert-files-blocked-by-run", Sizes::Standard);
}
