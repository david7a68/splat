//! F1 conversion is explicit, previewed, and recoverable without replacing edits.
use super::*;
use std::fs;

pub fn run(d: &mut Driver<'_>) {
    let root = d.root.join("campaigns/emberfall");
    let path = root.join("NPCs/Rook.md");
    let before = fs::read_to_string(&path).unwrap();
    d.click(NAV_WORLD);
    d.click_containing("Malformed · ");
    d.expect("Line 7: splat_status: value not read");
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
    d.state("saved-in-new-layout", Sizes::All);
    let after = fs::read_to_string(&path).unwrap();
    assert!(after.contains("splat_schema: 2"));
    assert!(after.contains("splat_in:"));
    d.click("Undo");
    d.settle();
    assert_eq!(fs::read_to_string(&path).unwrap(), before);

    d.click(NAV_CAMPAIGN);
    d.click("Convert files…");
    d.click("File to convert");
    d.click("NPCs/Rook.md");
    d.expect("Before");
    d.expect("After");
    d.state("convert-files-preview", Sizes::All);
    d.click("Cancel");
    assert_eq!(fs::read_to_string(&path).unwrap(), before);
}
