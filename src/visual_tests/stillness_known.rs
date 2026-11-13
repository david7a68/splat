//! The known lines of the blocking G checks (`stillness::BLOCKING`): each a check, a state and its
//! exact count of findings. This list is code, not `baseline.json`, so no refresh can add to it,
//! and it may only shrink: a state with fewer findings than listed fails the run until its line
//! is lowered or removed. Fix a line in its screen, or add `d.expect_moves("reason")` before the
//! state when a G1a or G1b move is the point of the step.
use super::stillness::{G1A, G1B, G1C, G3, KnownLine};

pub const KNOWN: &[KnownLine] = &[
    // G1A: 4 states, 29 findings.
    (
        G1A,
        (
            "generator-tables",
            "06-generator-world-keeps-its-npc",
            "960x640",
        ),
        5,
    ),
    (
        G1A,
        (
            "world-part-of-safety",
            "02-read-picker-unread-disk-kept",
            "1200x800",
        ),
        8,
    ),
    (
        G1A,
        (
            "world-part-of-safety",
            "02-read-picker-unread-disk-kept",
            "1920x1080",
        ),
        8,
    ),
    (
        G1A,
        (
            "world-part-of-safety",
            "02-read-picker-unread-disk-kept",
            "960x640",
        ),
        8,
    ),
    // G1B: 68 states, 102 findings.
    (
        G1B,
        (
            "campaign-content",
            "15-missing-campaign-manifest-needs-explicit-write",
            "1200x800",
        ),
        3,
    ),
    (
        G1B,
        (
            "campaign-content",
            "15-missing-campaign-manifest-needs-explicit-write",
            "1920x1080",
        ),
        2,
    ),
    (
        G1B,
        (
            "campaign-content",
            "15-missing-campaign-manifest-needs-explicit-write",
            "960x640",
        ),
        12,
    ),
    (
        G1B,
        (
            "campaign-content",
            "17-campaign-manifest-write-undone",
            "960x640",
        ),
        1,
    ),
    (
        G1B,
        ("column-resize", "06-run-rail-keeps-roster-room", "1200x800"),
        1,
    ),
    (G1B, ("dm-audit", "(after the last state)", "1200x800"), 1),
    (G1B, ("empty", "07-campaign-added", "960x640"), 1),
    (
        G1B,
        ("encounter-build", "(after the last state)", "960x640"),
        1,
    ),
    (
        G1B,
        ("encounter-groups", "(after the last state)", "960x640"),
        2,
    ),
    (
        G1B,
        ("encounter-groups", "03-group-member-changes", "960x640"),
        1,
    ),
    (
        G1B,
        ("encounter-groups", "06-group-invalid-change", "960x640"),
        1,
    ),
    (
        G1B,
        ("encounter-groups", "24-a-named-side-on-its-turn", "960x640"),
        2,
    ),
    (
        G1B,
        (
            "encounter-owned-groups",
            "05-new-group-in-the-encounter",
            "960x640",
        ),
        1,
    ),
    (G1B, ("encounter-run", "03-next-turn", "960x640"), 1),
    (
        G1B,
        ("generator-tables", "(after the last state)", "960x640"),
        1,
    ),
    (
        G1B,
        (
            "generator-tables",
            "03-generator-built-in-names-in-lookup",
            "960x640",
        ),
        1,
    ),
    (G1B, ("journey-j2", "(after the last state)", "1200x800"), 1),
    (G1B, ("journey-j3", "(after the last state)", "960x640"), 9),
    (G1B, ("journey-j4", "(after the last state)", "1200x800"), 1),
    (
        G1B,
        ("library-navigation", "(after the last state)", "960x640"),
        1,
    ),
    (
        G1B,
        (
            "library-navigation",
            "05-reference-outside-package",
            "960x640",
        ),
        1,
    ),
    (
        G1B,
        ("list-formats", "01-timeline-v2-dm-prose", "960x640"),
        1,
    ),
    (
        G1B,
        ("list-formats", "11-group-repair-changes-cell", "960x640"),
        1,
    ),
    (
        G1B,
        ("log-from-play", "02-log-line-added-in-lookup", "1200x800"),
        1,
    ),
    (G1B, ("nav-table", "(after the last state)", "960x640"), 1),
    (
        G1B,
        (
            "package-file-changed",
            "16-repair-draft-parses",
            "1920x1080",
        ),
        1,
    ),
    (
        G1B,
        (
            "package-manifest-broken",
            "04-manifest-opened-for-repair",
            "960x640",
        ),
        1,
    ),
    (
        G1B,
        ("plain-evening", "(after the last state)", "1200x800"),
        1,
    ),
    (
        G1B,
        (
            "recording-banner-priority",
            "(after the last state)",
            "960x640",
        ),
        1,
    ),
    (
        G1B,
        (
            "recording-banner-priority",
            "03-blocked-banner-retry",
            "960x640",
        ),
        1,
    ),
    (
        G1B,
        (
            "remove-package-in-use",
            "04-missing-package-technical-details",
            "960x640",
        ),
        1,
    ),
    (
        G1B,
        (
            "remove-package-in-use",
            "05-missing-package-removed-with-undo",
            "960x640",
        ),
        1,
    ),
    (G1B, ("run-continue", "14-undo-steps-unread", "960x640"), 1),
    (
        G1B,
        ("run-continue", "16-history-not-the-last-fight", "960x640"),
        1,
    ),
    (
        G1B,
        ("run-edge-states", "01-long-effect-text", "960x640"),
        1,
    ),
    (
        G1B,
        ("run-effects", "03-effects-beside-lookup", "960x640"),
        1,
    ),
    (
        G1B,
        ("run-large-roster", "(after the last state)", "960x640"),
        4,
    ),
    (G1B, ("run-loot", "07-rewards-for-loot", "960x640"), 1),
    (G1B, ("run-overrides", "03-slot-changes-open", "960x640"), 1),
    (G1B, ("run-restart", "01-before-restart", "960x640"), 1),
    (G1B, ("run-rolls", "02-attack-armed", "960x640"), 1),
    (G1B, ("run-rolls", "04-tile-menu", "960x640"), 1),
    (
        G1B,
        ("run-roster-conflict", "05-after-full-round", "960x640"),
        4,
    ),
    (
        G1B,
        ("run-sync-states", "04-conflict-on-strip", "960x640"),
        1,
    ),
    (
        G1B,
        ("session-notes", "22-timeline-time-not-counted", "960x640"),
        1,
    ),
    (
        G1B,
        ("session-notes", "27-session-long-title", "960x640"),
        1,
    ),
    (
        G1B,
        ("session-notes", "32-session-folder-in-campaign", "960x640"),
        1,
    ),
    (
        G1B,
        ("session-notes", "38-sessions-with-notes", "960x640"),
        1,
    ),
    (G1B, ("session-plan", "12-rule-from-the-row", "960x640"), 1),
    (G1B, ("session-plan", "14-correct-initiative", "960x640"), 1),
    (
        G1B,
        ("session-plan-groups", "02-plan-group-assembled", "960x640"),
        1,
    ),
    (
        G1B,
        (
            "session-plan-groups",
            "10-plan-group-in-another-session",
            "960x640",
        ),
        1,
    ),
    (
        G1B,
        ("session-plan-scenes", "08-scenes-folded", "960x640"),
        2,
    ),
    (
        G1B,
        ("session-recording", "04-recording-listed", "960x640"),
        1,
    ),
    (G1B, ("session-recording", "05-run-silent", "960x640"), 1),
    (
        G1B,
        ("session-recording", "15-key-for-another-service", "960x640"),
        2,
    ),
    (
        G1B,
        ("session-recording", "16-recording-ready", "960x640"),
        1,
    ),
    (
        G1B,
        ("session-recording", "27-second-pass-review", "960x640"),
        1,
    ),
    (
        G1B,
        ("session-recording", "37-input-during-recording", "960x640"),
        2,
    ),
    (G1B, ("settings", "(after the last state)", "960x640"), 1),
    (G1B, ("settings", "09-edit-campaign", "960x640"), 1),
    (
        G1B,
        ("settings", "11-campaign-details-discarded", "960x640"),
        1,
    ),
    (G1B, ("settings", "16-touchpad-settings", "960x640"), 1),
    (G1B, ("signed-in", "15-link-tab-during-run", "960x640"), 2),
    (
        G1B,
        ("travel-checks", "06-travel-checks-timeline", "960x640"),
        1,
    ),
    (
        G1B,
        ("travel-checks", "10-travel-checks-road-fight", "960x640"),
        1,
    ),
    (G1B, ("world", "08-world-generated-npc-saved", "960x640"), 1),
    (G1B, ("world", "10-npc-add-to-run-form", "960x640"), 1),
    // G1C: 7 states, 15 findings.
    (
        G1C,
        (
            "encounter-groups",
            "25-tied-arrival-moved-before",
            "960x640",
        ),
        1,
    ),
    (
        G1C,
        (
            "run-continue",
            "04-ended-with-loot-beside-lookup",
            "1200x800",
        ),
        8,
    ),
    (G1C, ("run-large-roster", "02-roster-of-20", "960x640"), 1),
    (G1C, ("run-rolls", "02-attack-armed", "960x640"), 1),
    (G1C, ("run-rolls", "04-tile-menu", "960x640"), 1),
    (G1C, ("run-rolls", "05-row-menu", "960x640"), 1),
    (
        G1C,
        ("world-places", "07-place-change-keeps-disk-edit", "960x640"),
        2,
    ),
    // G3: 24 states, 73 findings.
    (G3, ("data-integrity", "06-run-folder-gone", "1920x1080"), 2),
    (G3, ("encounter-build", "01-encounter-list", "1920x1080"), 2),
    (
        G3,
        ("encounter-build", "13-encounter-menu-remove", "1920x1080"),
        1,
    ),
    (
        G3,
        ("encounter-build", "14-encounter-removed", "1920x1080"),
        1,
    ),
    (
        G3,
        ("encounter-difficulty", "01-ready-to-run", "1920x1080"),
        5,
    ),
    (
        G3,
        ("encounter-groups", "01-groups-in-the-list", "1920x1080"),
        7,
    ),
    (G3, ("encounter-groups", "14-group-removed", "1920x1080"), 7),
    (
        G3,
        (
            "encounter-history",
            "01-encounters-with-history",
            "1920x1080",
        ),
        2,
    ),
    (G3, ("encounter-history", "02-history-open", "1920x1080"), 4),
    (
        G3,
        ("encounter-history", "04-ended-run-open", "1920x1080"),
        1,
    ),
    (
        G3,
        (
            "encounter-history",
            "05-history-beside-open-run",
            "1920x1080",
        ),
        1,
    ),
    (
        G3,
        ("encounter-history", "06-keep-in-ready-blocked", "1920x1080"),
        3,
    ),
    (
        G3,
        (
            "encounter-owned-groups",
            "01-groups-for-encounters-closed",
            "1920x1080",
        ),
        3,
    ),
    (
        G3,
        (
            "encounter-owned-groups",
            "02-groups-for-encounters-open",
            "1920x1080",
        ),
        10,
    ),
    (
        G3,
        (
            "list-formats",
            "06-flagged-group-in-groups-list",
            "1920x1080",
        ),
        8,
    ),
    (
        G3,
        ("npc-in-lookup", "07-npc-start-fight-in-lookup", "1920x1080"),
        2,
    ),
    (
        G3,
        (
            "npc-in-lookup",
            "08-package-creature-start-fight",
            "1920x1080",
        ),
        2,
    ),
    (
        G3,
        ("run-continue", "12-history-continue-fight", "1920x1080"),
        1,
    ),
    (
        G3,
        ("run-continue", "13-history-beside-lookup", "1920x1080"),
        1,
    ),
    (
        G3,
        ("run-continue", "16-history-not-the-last-fight", "1920x1080"),
        1,
    ),
    (
        G3,
        (
            "run-continue",
            "17-history-not-the-last-fight-beside-lookup",
            "1920x1080",
        ),
        1,
    ),
    (
        G3,
        (
            "run-edge-states",
            "09-saved-new-run-and-closed",
            "1920x1080",
        ),
        3,
    ),
    (
        G3,
        ("run-restart", "04-run-closed-in-one-step", "1920x1080"),
        2,
    ),
    (
        G3,
        ("session-plan", "38-saved-runs-history", "1920x1080"),
        3,
    ),
];
