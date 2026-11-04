//! F1 conversion is explicit, previewed, and recoverable without replacing edits.
use super::*;
use std::fs;

pub fn run(d: &mut Driver<'_>) {
    let root = d.root.join("campaigns/emberfall");
    let path = root.join("NPCs/Rook.md");
    let before = fs::read_to_string(&path).unwrap();
    d.click(NAV_WORLD);
    d.click_containing("Malformed · ");
    d.expect("Status \"[alive]\" not read (line 7). Use a short word, such as alive.");
    d.expect("A note for tonight.");
    d.state("flagged-malformed-value", Sizes::All);
    assert_eq!(
        fs::read_to_string(&path).unwrap(),
        before,
        "Reading never converts a file"
    );

    d.click_containing("Rook · ");
    d.click("Change In");
    d.type_text("Undercroft");
    d.click("Undercroft");
    d.expect("Saved in the new layout.");
    d.observe_both(
        "H1 World",
        "One named Undo restores the place change and the old layout",
        |d| {
            d.nodes()
                .iter()
                .filter(|node| node.in_view() && node.name.starts_with("Undo"))
                .count()
                == 1
                && d.visible_text("In: not set → Undercroft. Saved in the new layout.")
        },
    );
    d.state("saved-in-new-layout", Sizes::All);
    let after = fs::read_to_string(&path).unwrap();
    assert!(after.contains("splat_schema: 2"));
    assert!(after.contains("splat_in:"));
    d.click("Undo place change");
    d.settle();
    assert_eq!(fs::read_to_string(&path).unwrap(), before);

    d.click(NAV_CAMPAIGN);
    d.click("Convert files…");
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
    d.state("convert-files-preview", Sizes::All);
    d.click("Cancel");
    assert_eq!(fs::read_to_string(&path).unwrap(), before);
    // Preview is read-only. A race at conversion is reported beside the result.
    d.click(NAV_LIBRARY);
    d.click_containing("files use the old layout");
    fs::write(&path, format!("{before}Outside edit after preview.\n")).unwrap();
    d.click("Convert files");
    d.expect("Saved in the new layout.");
    d.expect("Undo convert");
    d.click("Files skipped");
    d.expect("NPCs/Rook.md: This entry changed on disk.");
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
        },
    );
    d.state("converted-with-skipped-file-and-one-undo", Sizes::All);
    d.click("Undo convert");
    assert!(
        fs::read_to_string(&path)
            .unwrap()
            .contains("Outside edit after preview.")
    );
}
