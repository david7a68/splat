//! F1 a malformed World value is flagged on its line, and reading never changes the file.
use super::*;
use std::fs;

pub fn run(d: &mut Driver<'_>) {
    let root = d.root.join("campaigns/emberfall");
    let path = root.join("NPCs/Malformed.md");
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
    // Round 24 FE1: the row's warning icon has the warning tone before muted words, and the
    // header's count equals the marked rows.
    let (marked, detail) = d.at_size(LARGE, |d| super::world_build::world_fault_marks(d, false));
    d.observe("iter-24-D-FE1", marked, detail);
    d.state("flagged-malformed-value", Sizes::All);
    assert_eq!(
        fs::read_to_string(&path).unwrap(),
        before,
        "Reading never changes a file"
    );
}
