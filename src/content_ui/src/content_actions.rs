use crate::design::{self};
/// `N content issues to repair`: the words of a page's collapsing content issue count.
pub fn content_issue_count(count: usize) -> String {
    design::count(count, "content issue to repair", "content issues to repair")
}

pub fn draft_switch_reason() -> &'static str {
    "Save or discard to switch campaign."
}

/// The campaign switch lock of a draft whose Save is blocked: Discard is the one way out, so
/// the reason does not name Save (round 25 FB6).
pub fn blocked_draft_switch_reason() -> &'static str {
    "Discard to switch campaign."
}
