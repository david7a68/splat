//! The layout parts prove their rules here, once. Callers do not repeat these proofs.
use super::*;

/// The ruling's order, by measure, at each boundary: on the title line, else one row under the
/// title, else rows that wrap at whole actions. Every action once, in order; never a wrap while
/// one row fits; an action wider than the line has its own row.
#[test]
// One row of all three actions is the point of these cases.
#[allow(clippy::single_range_in_vec_init)]
fn title_placement_follows_the_ruling_at_each_boundary() {
    let widths = [60.0, 100.0, 80.0];
    let row = 60.0 + 100.0 + 80.0 + 2.0 * ACTION_GAP;
    let (title, lead) = (200.0, L - S);
    let place = |beside: f32, under: f32| title_placement(title, lead, &widths, beside, under);
    assert_eq!(place(title + lead + row, 1.0), TitlePlacement::OnTitle);
    assert_eq!(
        place(title + lead + row - 0.5, 1000.0),
        TitlePlacement::Under(vec![0..3])
    );
    assert_eq!(place(10.0, row), TitlePlacement::Under(vec![0..3]));
    assert_eq!(
        place(10.0, row - 0.5),
        TitlePlacement::Under(vec![0..2, 2..3])
    );
    assert_eq!(
        place(10.0, 100.0),
        TitlePlacement::Under(vec![0..1, 1..2, 2..3])
    );
    // Wider than the line: its own row, never dropped.
    assert_eq!(
        place(10.0, 70.0),
        TitlePlacement::Under(vec![0..1, 1..2, 2..3])
    );
    // No actions: nothing moves under the title.
    assert_eq!(
        title_placement(title, lead, &[], 10.0, 10.0),
        TitlePlacement::OnTitle
    );
}
