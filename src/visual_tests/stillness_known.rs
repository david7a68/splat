//! The known lines of the blocking G and N checks (`stillness::BLOCKING`): each a check, a state
//! and its exact count of findings, in one block per scenario. This list is code, not
//! `baseline.json`, so no refresh can add to it, and it may only shrink: a state with more
//! findings than listed fails the run, and so does a listed state with fewer, until its line is
//! lowered or removed. To lower your lines, run your scenarios (`visual-test <label> --only a,b`),
//! then replace each of their blocks here with the same block of
//! `artifacts/screenshots/<label>/stillness-known.txt` and run `cargo fmt`. A check's constant is
//! imported here only while it has a line (G7 has none). Fix a line in its screen, or add
//! `d.expect_moves("reason")` before the state when a G1a or G1b move is the point of the step.
use super::stillness::{G1A, G1B, G1C, G3, G6, KnownLine, N2, N5};

pub const KNOWN: &[KnownLine] = &[
    // campaign-content
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
    (G6, ("campaign-content", "11-repair-needed", "960x640"), 1),
    (G6, ("campaign-content", "13-repair-dirty", "1200x800"), 1),
    (G6, ("campaign-content", "13-repair-dirty", "1920x1080"), 1),
    (
        G6,
        (
            "campaign-content",
            "16-campaign-manifest-written-with-undo",
            "1200x800",
        ),
        1,
    ),
    (
        G6,
        (
            "campaign-content",
            "16-campaign-manifest-written-with-undo",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        (
            "campaign-content",
            "16-campaign-manifest-written-with-undo",
            "960x640",
        ),
        1,
    ),
    // campaign-copy-registered
    (
        G1B,
        ("campaign-copy-registered", "01-copy-registered", "960x640"),
        1,
    ),
    (
        G6,
        ("campaign-copy-registered", "01-copy-registered", "1200x800"),
        1,
    ),
    (
        G6,
        (
            "campaign-copy-registered",
            "01-copy-registered",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        (
            "campaign-copy-registered",
            "02-copy-registered-after-build",
            "1200x800",
        ),
        1,
    ),
    (
        G6,
        (
            "campaign-copy-registered",
            "02-copy-registered-after-build",
            "1920x1080",
        ),
        1,
    ),
    // column-resize
    (
        G1B,
        ("column-resize", "06-run-rail-keeps-roster-room", "1200x800"),
        1,
    ),
    (
        G6,
        (
            "column-resize",
            "13-session-events-fill-their-column",
            "1200x800",
        ),
        1,
    ),
    (
        G6,
        (
            "column-resize",
            "13-session-events-fill-their-column",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        (
            "column-resize",
            "13-session-events-fill-their-column",
            "960x640",
        ),
        1,
    ),
    // data-integrity
    (G3, ("data-integrity", "06-run-folder-gone", "1920x1080"), 2),
    (
        N2,
        ("data-integrity", "04-recording-audio-removed", "1200x800"),
        1,
    ),
    (
        N2,
        ("data-integrity", "04-recording-audio-removed", "1920x1080"),
        1,
    ),
    (
        N2,
        ("data-integrity", "04-recording-audio-removed", "960x640"),
        1,
    ),
    // dialogs
    (
        G6,
        ("dialogs", "06-unsaved-changes-on-close", "1200x800"),
        1,
    ),
    (
        G6,
        ("dialogs", "06-unsaved-changes-on-close", "1920x1080"),
        1,
    ),
    (G6, ("dialogs", "06-unsaved-changes-on-close", "960x640"), 1),
    // dm-audit
    (G1B, ("dm-audit", "(after the last state)", "1200x800"), 1),
    // empty
    (G1B, ("empty", "07-campaign-added", "960x640"), 1),
    (G6, ("empty", "07-campaign-added", "960x640"), 1),
    // encounter-build
    (
        G1B,
        ("encounter-build", "(after the last state)", "960x640"),
        1,
    ),
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
    // encounter-difficulty
    (
        G3,
        ("encounter-difficulty", "01-ready-to-run", "1920x1080"),
        5,
    ),
    // encounter-groups
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
        G1C,
        (
            "encounter-groups",
            "25-tied-arrival-moved-before",
            "960x640",
        ),
        1,
    ),
    (
        G3,
        ("encounter-groups", "01-groups-in-the-list", "1920x1080"),
        7,
    ),
    (G3, ("encounter-groups", "14-group-removed", "1920x1080"), 7),
    (
        G6,
        ("encounter-groups", "16-invalid-arrival-total", "960x640"),
        1,
    ),
    (
        G6,
        (
            "encounter-groups",
            "25-tied-arrival-moved-before",
            "960x640",
        ),
        1,
    ),
    (
        G6,
        ("encounter-groups", "27-group-added-in-setup", "960x640"),
        1,
    ),
    (N5, ("encounter-groups", "15-group-panel", "960x640"), 1),
    (
        N5,
        ("encounter-groups", "16-invalid-arrival-total", "960x640"),
        1,
    ),
    (
        N5,
        ("encounter-groups", "17-available-member-added", "960x640"),
        1,
    ),
    (
        N5,
        ("encounter-groups", "19-group-total-typed", "960x640"),
        1,
    ),
    (
        N5,
        ("encounter-groups", "26-group-panel-in-setup", "960x640"),
        1,
    ),
    // encounter-history
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
    // encounter-owned-groups
    (
        G1B,
        (
            "encounter-owned-groups",
            "05-new-group-in-the-encounter",
            "960x640",
        ),
        1,
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
        N5,
        (
            "encounter-owned-groups",
            "16-group-panel-of-an-older-save",
            "960x640",
        ),
        1,
    ),
    // encounter-run
    (G1B, ("encounter-run", "03-next-turn", "960x640"), 1),
    // generator-tables
    (
        G1A,
        (
            "generator-tables",
            "06-generator-world-keeps-its-npc",
            "960x640",
        ),
        6,
    ),
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
    // journey-j2
    (G1B, ("journey-j2", "(after the last state)", "1200x800"), 1),
    // journey-j3
    (G1B, ("journey-j3", "(after the last state)", "960x640"), 9),
    // journey-j4
    (G1B, ("journey-j4", "(after the last state)", "1200x800"), 1),
    // library-navigation
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
        G6,
        ("library-navigation", "01-homebrew-reference", "1920x1080"),
        1,
    ),
    (
        G6,
        ("library-navigation", "01-homebrew-reference", "960x640"),
        1,
    ),
    (
        G6,
        (
            "library-navigation",
            "03-dependency-copy-reconnected",
            "1200x800",
        ),
        1,
    ),
    (
        G6,
        (
            "library-navigation",
            "03-dependency-copy-reconnected",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        (
            "library-navigation",
            "03-dependency-copy-reconnected",
            "960x640",
        ),
        1,
    ),
    // list-formats
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
        G3,
        (
            "list-formats",
            "06-flagged-group-in-groups-list",
            "1920x1080",
        ),
        8,
    ),
    (
        G6,
        (
            "list-formats",
            "04-rules-and-runs-flagged-choice",
            "1200x800",
        ),
        1,
    ),
    (
        G6,
        (
            "list-formats",
            "04-rules-and-runs-flagged-choice",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        (
            "list-formats",
            "04-rules-and-runs-flagged-choice",
            "960x640",
        ),
        1,
    ),
    (
        G6,
        (
            "list-formats",
            "15-timeline-item-edited-and-flagged-in-obsidian",
            "1200x800",
        ),
        1,
    ),
    (
        G6,
        (
            "list-formats",
            "15-timeline-item-edited-and-flagged-in-obsidian",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        (
            "list-formats",
            "15-timeline-item-edited-and-flagged-in-obsidian",
            "960x640",
        ),
        1,
    ),
    // log-from-play
    (
        G1B,
        ("log-from-play", "02-log-line-added-in-lookup", "1200x800"),
        1,
    ),
    (
        N2,
        ("log-from-play", "01-done-scene-add-to-log", "1200x800"),
        1,
    ),
    (
        N2,
        ("log-from-play", "01-done-scene-add-to-log", "1920x1080"),
        1,
    ),
    (
        N2,
        ("log-from-play", "02-log-line-added-in-lookup", "1920x1080"),
        1,
    ),
    (
        N2,
        ("log-from-play", "04-log-field-from-scene", "1920x1080"),
        1,
    ),
    (
        N2,
        ("log-from-play", "04-log-field-from-scene", "960x640"),
        1,
    ),
    (
        N2,
        ("log-from-play", "05-log-header-long-title", "1920x1080"),
        1,
    ),
    (
        N2,
        ("log-from-play", "06-log-field-blocked-by-edit", "960x640"),
        2,
    ),
    (
        N2,
        (
            "log-from-play",
            "07-log-words-of-a-deleted-entry",
            "960x640",
        ),
        1,
    ),
    (
        N2,
        ("log-from-play", "08-log-words-without-an-entry", "960x640"),
        1,
    ),
    (
        N5,
        (
            "log-from-play",
            "07-log-words-of-a-deleted-entry",
            "960x640",
        ),
        1,
    ),
    (
        N5,
        ("log-from-play", "08-log-words-without-an-entry", "960x640"),
        1,
    ),
    // nav-table
    (G1B, ("nav-table", "(after the last state)", "960x640"), 1),
    // npc-in-lookup
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
    // package-about
    (
        G6,
        ("package-about", "02-package-every-entry-used", "1200x800"),
        1,
    ),
    (
        G6,
        ("package-about", "02-package-every-entry-used", "1920x1080"),
        1,
    ),
    (
        G6,
        ("package-about", "02-package-every-entry-used", "960x640"),
        1,
    ),
    (
        G6,
        ("package-about", "04-package-about-edited", "1200x800"),
        1,
    ),
    (
        G6,
        ("package-about", "04-package-about-edited", "1920x1080"),
        1,
    ),
    (
        G6,
        ("package-about", "04-package-about-edited", "960x640"),
        1,
    ),
    // package-delete-entry
    (
        N5,
        (
            "package-delete-entry",
            "02-deleted-package-entry-in-encounter",
            "960x640",
        ),
        2,
    ),
    (
        N5,
        (
            "package-delete-entry",
            "03-deleted-package-blocks-new-fight",
            "960x640",
        ),
        2,
    ),
    (
        N5,
        (
            "package-delete-entry",
            "05-deleted-package-entry-choose-replacement",
            "960x640",
        ),
        1,
    ),
    (
        N5,
        (
            "package-delete-entry",
            "07-deleted-package-entry-replacement-undone",
            "960x640",
        ),
        2,
    ),
    // package-duplicate-id
    (
        G6,
        (
            "package-duplicate-id",
            "01-duplicate-package-library-list",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        (
            "package-duplicate-id",
            "04-duplicate-copy-reader",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        (
            "package-duplicate-id",
            "05-duplicate-copy-new-identity",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        (
            "package-duplicate-id",
            "06-duplicate-identity-restored",
            "1920x1080",
        ),
        1,
    ),
    // package-file-changed
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
        G6,
        (
            "package-file-changed",
            "05-package-disk-version-kept",
            "1200x800",
        ),
        1,
    ),
    (
        G6,
        (
            "package-file-changed",
            "05-package-disk-version-kept",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        (
            "package-file-changed",
            "13-invalid-package-in-paused-run",
            "1200x800",
        ),
        1,
    ),
    (
        G6,
        (
            "package-file-changed",
            "13-invalid-package-in-paused-run",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        (
            "package-file-changed",
            "15-repaired-package-in-paused-run",
            "1200x800",
        ),
        1,
    ),
    (
        G6,
        (
            "package-file-changed",
            "15-repaired-package-in-paused-run",
            "1920x1080",
        ),
        1,
    ),
    (
        N5,
        (
            "package-file-changed",
            "10-invalid-package-in-encounter",
            "960x640",
        ),
        2,
    ),
    (
        N5,
        (
            "package-file-changed",
            "11-invalid-package-blocks-new-fight",
            "960x640",
        ),
        2,
    ),
    // package-folder-copied
    (
        G6,
        (
            "package-folder-copied",
            "06-copied-package-registered-alongside-original",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        (
            "package-folder-copied",
            "06-copied-package-registered-alongside-original",
            "960x640",
        ),
        1,
    ),
    (
        G6,
        ("package-folder-copied", "09-copy-entries-used", "1920x1080"),
        1,
    ),
    (
        G6,
        ("package-folder-copied", "09-copy-entries-used", "960x640"),
        1,
    ),
    (
        G6,
        (
            "package-folder-copied",
            "12-manifest-only-hand-repair-keeps-both-files",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        (
            "package-folder-copied",
            "13-same-filenames-name-the-folder",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        (
            "package-folder-copied",
            "14-copied-folder-identities-repaired-together",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        (
            "package-folder-copied",
            "14-copied-folder-identities-repaired-together",
            "960x640",
        ),
        1,
    ),
    (
        G6,
        (
            "package-folder-copied",
            "15-copied-folder-identities-undone",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        (
            "package-folder-copied",
            "16-one-entry-copied-between-registered-packages",
            "1920x1080",
        ),
        1,
    ),
    // package-manifest-broken
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
        G6,
        (
            "package-manifest-broken",
            "11-manifest-written-again-library",
            "1200x800",
        ),
        1,
    ),
    (
        G6,
        (
            "package-manifest-broken",
            "11-manifest-written-again-library",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        (
            "package-manifest-broken",
            "11-manifest-written-again-library",
            "960x640",
        ),
        1,
    ),
    (
        G6,
        (
            "package-manifest-broken",
            "14-manifest-written-beside-readme",
            "1200x800",
        ),
        1,
    ),
    (
        G6,
        (
            "package-manifest-broken",
            "14-manifest-written-beside-readme",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        (
            "package-manifest-broken",
            "14-manifest-written-beside-readme",
            "960x640",
        ),
        1,
    ),
    // package-rename
    (
        G6,
        (
            "package-rename",
            "02-linked-entry-renamed-on-disk",
            "1200x800",
        ),
        1,
    ),
    (
        G6,
        (
            "package-rename",
            "02-linked-entry-renamed-on-disk",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        (
            "package-rename",
            "02-linked-entry-renamed-on-disk",
            "960x640",
        ),
        1,
    ),
    (
        G6,
        (
            "package-rename",
            "03-renamed-entry-heading-fixed",
            "1200x800",
        ),
        1,
    ),
    (
        G6,
        (
            "package-rename",
            "03-renamed-entry-heading-fixed",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        (
            "package-rename",
            "03-renamed-entry-heading-fixed",
            "960x640",
        ),
        1,
    ),
    (
        G6,
        (
            "package-rename",
            "05-renamed-creature-offers-link-update",
            "1200x800",
        ),
        1,
    ),
    (
        G6,
        (
            "package-rename",
            "05-renamed-creature-offers-link-update",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        (
            "package-rename",
            "06-renamed-creature-links-updated",
            "1200x800",
        ),
        1,
    ),
    (
        G6,
        (
            "package-rename",
            "06-renamed-creature-links-updated",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        (
            "package-rename",
            "07-renamed-creature-link-update-undone",
            "1200x800",
        ),
        1,
    ),
    (
        G6,
        (
            "package-rename",
            "07-renamed-creature-link-update-undone",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        (
            "package-rename",
            "11-package-folder-restored-with-links",
            "1200x800",
        ),
        1,
    ),
    (
        G6,
        (
            "package-rename",
            "11-package-folder-restored-with-links",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        (
            "package-rename",
            "11-package-folder-restored-with-links",
            "960x640",
        ),
        1,
    ),
    // party
    (N2, ("party", "14-force-read-replaced", "960x640"), 1),
    // plain-evening
    (
        G1B,
        ("plain-evening", "(after the last state)", "1200x800"),
        1,
    ),
    (
        G6,
        (
            "plain-evening",
            "04-session-last: played fight open",
            "1200x800",
        ),
        1,
    ),
    (
        G6,
        (
            "plain-evening",
            "04-session-last: played fight open",
            "960x640",
        ),
        1,
    ),
    (
        N2,
        (
            "plain-evening",
            "04-session-last: played fight open",
            "1200x800",
        ),
        1,
    ),
    // recording-banner-priority
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
        G6,
        (
            "recording-banner-priority",
            "03-blocked-banner-retry",
            "1200x800",
        ),
        1,
    ),
    (
        G6,
        (
            "recording-banner-priority",
            "03-blocked-banner-retry",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        (
            "recording-banner-priority",
            "03-blocked-banner-retry",
            "960x640",
        ),
        1,
    ),
    (
        G6,
        (
            "recording-banner-priority",
            "05-recording-retry-with-input",
            "1920x1080",
        ),
        1,
    ),
    (
        N2,
        (
            "recording-banner-priority",
            "01-failed-elsewhere-with-unprocessed-row",
            "1200x800",
        ),
        2,
    ),
    (
        N2,
        (
            "recording-banner-priority",
            "01-failed-elsewhere-with-unprocessed-row",
            "1920x1080",
        ),
        2,
    ),
    (
        N2,
        (
            "recording-banner-priority",
            "01-failed-elsewhere-with-unprocessed-row",
            "960x640",
        ),
        2,
    ),
    (
        N2,
        (
            "recording-banner-priority",
            "02-failed-processing-and-stopped-recording",
            "1200x800",
        ),
        3,
    ),
    (
        N2,
        (
            "recording-banner-priority",
            "02-failed-processing-and-stopped-recording",
            "1920x1080",
        ),
        3,
    ),
    (
        N2,
        (
            "recording-banner-priority",
            "02-failed-processing-and-stopped-recording",
            "960x640",
        ),
        3,
    ),
    (
        N2,
        (
            "recording-banner-priority",
            "03-blocked-banner-retry",
            "1200x800",
        ),
        3,
    ),
    (
        N2,
        (
            "recording-banner-priority",
            "03-blocked-banner-retry",
            "1920x1080",
        ),
        3,
    ),
    (
        N2,
        (
            "recording-banner-priority",
            "03-blocked-banner-retry",
            "960x640",
        ),
        3,
    ),
    (
        N2,
        (
            "recording-banner-priority",
            "04-stalled-elsewhere-with-unprocessed-row",
            "1200x800",
        ),
        1,
    ),
    (
        N2,
        (
            "recording-banner-priority",
            "04-stalled-elsewhere-with-unprocessed-row",
            "1920x1080",
        ),
        1,
    ),
    (
        N2,
        (
            "recording-banner-priority",
            "04-stalled-elsewhere-with-unprocessed-row",
            "960x640",
        ),
        1,
    ),
    (
        N2,
        (
            "recording-banner-priority",
            "05-recording-retry-with-input",
            "1200x800",
        ),
        3,
    ),
    (
        N2,
        (
            "recording-banner-priority",
            "05-recording-retry-with-input",
            "1920x1080",
        ),
        3,
    ),
    (
        N2,
        (
            "recording-banner-priority",
            "05-recording-retry-with-input",
            "960x640",
        ),
        2,
    ),
    // remove-package-in-use
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
    (
        G6,
        (
            "remove-package-in-use",
            "02-package-unregistered-with-undo",
            "1200x800",
        ),
        1,
    ),
    (
        G6,
        (
            "remove-package-in-use",
            "02-package-unregistered-with-undo",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        (
            "remove-package-in-use",
            "07-paused-run-after-package-removal",
            "1200x800",
        ),
        1,
    ),
    (
        G6,
        (
            "remove-package-in-use",
            "07-paused-run-after-package-removal",
            "1920x1080",
        ),
        1,
    ),
    // run-absent-player
    (
        N2,
        (
            "run-absent-player",
            "05-initiative-set-in-combat",
            "960x640",
        ),
        1,
    ),
    // run-add-creature
    (N5, ("run-add-creature", "01-arrivals-empty", "960x640"), 1),
    (
        N5,
        ("run-add-creature", "04-arrivals-made-up-added", "960x640"),
        1,
    ),
    // run-continue
    (G1B, ("run-continue", "14-undo-steps-unread", "960x640"), 1),
    (
        G1B,
        ("run-continue", "16-history-not-the-last-fight", "960x640"),
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
    (G6, ("run-continue", "14-undo-steps-unread", "1920x1080"), 1),
    (
        G6,
        (
            "run-continue",
            "15-undo-steps-unread-beside-lookup",
            "1920x1080",
        ),
        1,
    ),
    // run-edge-states
    (
        G1B,
        ("run-edge-states", "01-long-effect-text", "960x640"),
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
        G6,
        ("run-edge-states", "07-close-while-not-saved", "960x640"),
        1,
    ),
    (
        G6,
        (
            "run-edge-states",
            "08-save-new-run-before-close",
            "1200x800",
        ),
        1,
    ),
    (
        G6,
        ("run-edge-states", "08-save-new-run-before-close", "960x640"),
        1,
    ),
    (
        N2,
        ("run-edge-states", "01-long-effect-text", "1200x800"),
        1,
    ),
    (N2, ("run-edge-states", "01-long-effect-text", "960x640"), 1),
    (
        N2,
        ("run-edge-states", "02-start-concentration-undo", "1200x800"),
        1,
    ),
    (
        N2,
        ("run-edge-states", "02-start-concentration-undo", "960x640"),
        1,
    ),
    (
        N2,
        ("run-edge-states", "03-stop-concentration-undo", "1200x800"),
        1,
    ),
    (
        N2,
        ("run-edge-states", "03-stop-concentration-undo", "960x640"),
        1,
    ),
    (
        N2,
        ("run-edge-states", "04-full-effect-in-conditions", "960x640"),
        1,
    ),
    // run-effects
    (
        G1B,
        ("run-effects", "03-effects-beside-lookup", "960x640"),
        1,
    ),
    (G6, ("run-effects", "02-effects-on-rows", "960x640"), 1),
    // run-folder-defect
    (
        G6,
        ("run-folder-defect", "01-run-folder-edited", "1200x800"),
        1,
    ),
    (
        G6,
        ("run-folder-defect", "01-run-folder-edited", "1920x1080"),
        1,
    ),
    (
        G6,
        ("run-folder-defect", "01-run-folder-edited", "960x640"),
        1,
    ),
    // run-large-roster
    (
        G1B,
        ("run-large-roster", "(after the last state)", "960x640"),
        4,
    ),
    (G1C, ("run-large-roster", "02-roster-of-20", "960x640"), 1),
    // run-long-names
    (
        G6,
        ("run-long-names", "06-creature-changed-since", "1200x800"),
        1,
    ),
    (
        G6,
        ("run-long-names", "06-creature-changed-since", "1920x1080"),
        1,
    ),
    // run-loot
    (G1B, ("run-loot", "07-rewards-for-loot", "960x640"), 1),
    (
        G6,
        ("run-loot", "09-rewards-with-loot-added", "1200x800"),
        1,
    ),
    (
        G6,
        ("run-loot", "09-rewards-with-loot-added", "1920x1080"),
        1,
    ),
    (G6, ("run-loot", "09-rewards-with-loot-added", "960x640"), 1),
    (N2, ("run-loot", "05-loot-in-the-editor", "1200x800"), 2),
    (N2, ("run-loot", "05-loot-in-the-editor", "1920x1080"), 2),
    // run-overrides
    (G1B, ("run-overrides", "03-slot-changes-open", "960x640"), 1),
    (N2, ("run-overrides", "03-slot-changes-open", "960x640"), 1),
    // run-restart
    (G1B, ("run-restart", "01-before-restart", "960x640"), 1),
    (
        G3,
        ("run-restart", "04-run-closed-in-one-step", "1920x1080"),
        2,
    ),
    // run-rolls
    (G1B, ("run-rolls", "02-attack-armed", "960x640"), 1),
    (G1B, ("run-rolls", "04-tile-menu", "960x640"), 1),
    (G1C, ("run-rolls", "02-attack-armed", "960x640"), 1),
    (G1C, ("run-rolls", "04-tile-menu", "960x640"), 1),
    (G1C, ("run-rolls", "05-row-menu", "960x640"), 1),
    (N2, ("run-rolls", "07-save-targets", "1200x800"), 2),
    (N2, ("run-rolls", "07-save-targets", "1920x1080"), 2),
    (N2, ("run-rolls", "07-save-targets", "960x640"), 1),
    // run-roster-conflict
    (
        G1B,
        ("run-roster-conflict", "05-after-full-round", "960x640"),
        4,
    ),
    // run-sync-conflict
    (
        N5,
        ("run-sync-conflict", "06-conflict-resolved", "960x640"),
        1,
    ),
    (N5, ("run-sync-conflict", "07-party-sync-on", "960x640"), 1),
    (
        N5,
        (
            "run-sync-conflict",
            "08-undo-boundary-for-one-character",
            "960x640",
        ),
        1,
    ),
    // run-sync-states
    (
        G1B,
        ("run-sync-states", "04-conflict-on-strip", "960x640"),
        1,
    ),
    // session-carry-over
    (G6, ("session-carry-over", "03-carried-over", "960x640"), 1),
    (
        G6,
        (
            "session-carry-over",
            "04-carried-over-beside-lookup",
            "960x640",
        ),
        1,
    ),
    (
        G6,
        ("session-carry-over", "05-carried-recap-in-read", "1200x800"),
        1,
    ),
    // session-notes
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
    (
        G6,
        ("session-notes", "10-session-event-added", "960x640"),
        1,
    ),
    (G6, ("session-notes", "11-plan-event-title", "960x640"), 1),
    (G6, ("session-notes", "12-session-events", "960x640"), 1),
    (
        G6,
        ("session-notes", "16-timeline-day-added", "1200x800"),
        1,
    ),
    (
        G6,
        ("session-notes", "16-timeline-day-added", "1920x1080"),
        1,
    ),
    (G6, ("session-notes", "16-timeline-day-added", "960x640"), 1),
    (G6, ("session-notes", "17-timeline-event", "1200x800"), 1),
    (G6, ("session-notes", "17-timeline-event", "1920x1080"), 1),
    (G6, ("session-notes", "17-timeline-event", "960x640"), 1),
    (
        G6,
        ("session-notes", "18-event-link-update-offered", "1200x800"),
        1,
    ),
    (
        G6,
        ("session-notes", "18-event-link-update-offered", "1920x1080"),
        1,
    ),
    (
        G6,
        ("session-notes", "18-event-link-update-offered", "960x640"),
        1,
    ),
    (
        G6,
        ("session-notes", "19-event-links-updated", "960x640"),
        1,
    ),
    (G6, ("session-notes", "21-timeline-removed", "960x640"), 1),
    (
        G6,
        (
            "session-notes",
            "33-session-folder-move-offered",
            "1200x800",
        ),
        1,
    ),
    (
        G6,
        (
            "session-notes",
            "33-session-folder-move-offered",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        ("session-notes", "34-session-folder-moved", "1200x800"),
        1,
    ),
    (
        G6,
        ("session-notes", "34-session-folder-moved", "1920x1080"),
        1,
    ),
    (
        G6,
        ("session-notes", "35-session-events-older", "960x640"),
        1,
    ),
    (
        G6,
        ("session-notes", "48-timeline-disk-conflict", "1200x800"),
        1,
    ),
    (
        G6,
        ("session-notes", "48-timeline-disk-conflict", "1920x1080"),
        1,
    ),
    (
        G6,
        ("session-notes", "48-timeline-disk-conflict", "960x640"),
        1,
    ),
    (
        N2,
        ("session-notes", "27-session-long-title", "1200x800"),
        1,
    ),
    (
        N2,
        ("session-notes", "27-session-long-title", "1920x1080"),
        1,
    ),
    // session-plan
    (G1B, ("session-plan", "12-rule-from-the-row", "960x640"), 1),
    (G1B, ("session-plan", "14-correct-initiative", "960x640"), 1),
    (
        G3,
        ("session-plan", "38-saved-runs-history", "1920x1080"),
        3,
    ),
    (G6, ("session-plan", "19-rewards-added", "1200x800"), 1),
    (G6, ("session-plan", "19-rewards-added", "1920x1080"), 1),
    (G6, ("session-plan", "23-setup-carries-hp", "1200x800"), 1),
    (G6, ("session-plan", "23-setup-carries-hp", "1920x1080"), 1),
    (G6, ("session-plan", "26-long-rest-restores", "1200x800"), 1),
    (
        G6,
        ("session-plan", "26-long-rest-restores", "1920x1080"),
        1,
    ),
    (G6, ("session-plan", "26-long-rest-restores", "960x640"), 1),
    (
        G6,
        (
            "session-plan",
            "30-resumed-setup-after-long-rest",
            "1200x800",
        ),
        1,
    ),
    (
        G6,
        (
            "session-plan",
            "30-resumed-setup-after-long-rest",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        (
            "session-plan",
            "30-resumed-setup-after-long-rest",
            "960x640",
        ),
        1,
    ),
    (G6, ("session-plan", "31-saved-hp-restored", "1200x800"), 1),
    (G6, ("session-plan", "31-saved-hp-restored", "1920x1080"), 1),
    (G6, ("session-plan", "31-saved-hp-restored", "960x640"), 1),
    (
        G6,
        ("session-plan", "32-timeline-long-rest-restores", "1200x800"),
        1,
    ),
    (
        G6,
        (
            "session-plan",
            "32-timeline-long-rest-restores",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        ("session-plan", "32-timeline-long-rest-restores", "960x640"),
        1,
    ),
    (
        G6,
        ("session-plan", "33-timeline-rest-ruling-off", "1200x800"),
        1,
    ),
    (
        G6,
        ("session-plan", "33-timeline-rest-ruling-off", "1920x1080"),
        1,
    ),
    (
        G6,
        ("session-plan", "33-timeline-rest-ruling-off", "960x640"),
        1,
    ),
    (
        G6,
        ("session-plan", "34-timeline-rest-ruling-on", "1200x800"),
        1,
    ),
    (
        G6,
        ("session-plan", "34-timeline-rest-ruling-on", "1920x1080"),
        1,
    ),
    (
        G6,
        ("session-plan", "34-timeline-rest-ruling-on", "960x640"),
        1,
    ),
    (N2, ("session-plan", "09-several-targets", "1200x800"), 2),
    (N2, ("session-plan", "09-several-targets", "1920x1080"), 2),
    (N2, ("session-plan", "09-several-targets", "960x640"), 2),
    // session-plan-groups
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
        G6,
        (
            "session-plan-groups",
            "06-group-removed-from-plan",
            "960x640",
        ),
        1,
    ),
    (
        G6,
        (
            "session-plan-groups",
            "10-plan-group-in-another-session",
            "960x640",
        ),
        1,
    ),
    (
        G6,
        (
            "session-plan-groups",
            "15-plan-group-no-member-can-join",
            "1920x1080",
        ),
        1,
    ),
    (
        N2,
        (
            "session-plan-groups",
            "15-plan-group-no-member-can-join",
            "960x640",
        ),
        1,
    ),
    (
        N5,
        (
            "session-plan-groups",
            "08-arrivals-tonight-first",
            "960x640",
        ),
        1,
    ),
    // session-plan-scenes
    (
        G1B,
        ("session-plan-scenes", "08-scenes-folded", "960x640"),
        2,
    ),
    (
        G6,
        ("session-plan-scenes", "08-scenes-folded", "960x640"),
        1,
    ),
    // session-recording
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
    (
        G6,
        ("session-recording", "11-recording-saved-no-key", "960x640"),
        1,
    ),
    (
        G6,
        ("session-recording", "12-processing-settings", "1920x1080"),
        1,
    ),
    (
        G6,
        (
            "session-recording",
            "14-processing-settings-anthropic",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        ("session-recording", "16-recording-ready", "1200x800"),
        1,
    ),
    (
        G6,
        ("session-recording", "16-recording-ready", "1920x1080"),
        1,
    ),
    (
        G6,
        ("session-recording", "16-recording-ready", "960x640"),
        1,
    ),
    (G6, ("session-recording", "24-marker-added", "1200x800"), 1),
    (G6, ("session-recording", "24-marker-added", "1920x1080"), 1),
    (G6, ("session-recording", "24-marker-added", "960x640"), 1),
    (
        G6,
        ("session-recording", "25-marked-transcript", "1200x800"),
        1,
    ),
    (
        G6,
        ("session-recording", "25-marked-transcript", "1920x1080"),
        1,
    ),
    (
        G6,
        ("session-recording", "25-marked-transcript", "960x640"),
        1,
    ),
    (
        G6,
        (
            "session-recording",
            "40-recording-summary-narrow",
            "1200x800",
        ),
        1,
    ),
    (
        G6,
        (
            "session-recording",
            "40-recording-summary-narrow",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        (
            "session-recording",
            "40-recording-summary-narrow",
            "960x640",
        ),
        1,
    ),
    (
        G6,
        (
            "session-recording",
            "41-processing-custom-speech-service",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        ("session-recording", "44-processing-with-no-run", "1200x800"),
        1,
    ),
    (
        G6,
        (
            "session-recording",
            "44-processing-with-no-run",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        ("session-recording", "44-processing-with-no-run", "960x640"),
        1,
    ),
    (
        G6,
        (
            "session-recording",
            "45-processing-with-setup-run",
            "1200x800",
        ),
        1,
    ),
    (
        G6,
        (
            "session-recording",
            "45-processing-with-setup-run",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        (
            "session-recording",
            "46-processing-with-running-run",
            "1200x800",
        ),
        1,
    ),
    (
        G6,
        (
            "session-recording",
            "46-processing-with-running-run",
            "1920x1080",
        ),
        1,
    ),
    (
        G6,
        (
            "session-recording",
            "47-processing-with-ended-run",
            "1200x800",
        ),
        1,
    ),
    (
        G6,
        (
            "session-recording",
            "47-processing-with-ended-run",
            "1920x1080",
        ),
        1,
    ),
    (
        N2,
        ("session-recording", "04-recording-listed", "960x640"),
        1,
    ),
    (
        N2,
        ("session-recording", "06-recording-silent", "1200x800"),
        1,
    ),
    (
        N2,
        ("session-recording", "06-recording-silent", "1920x1080"),
        1,
    ),
    (
        N2,
        ("session-recording", "06-recording-silent", "960x640"),
        1,
    ),
    (
        N2,
        ("session-recording", "07-recording-stalled", "1200x800"),
        1,
    ),
    (
        N2,
        ("session-recording", "07-recording-stalled", "1920x1080"),
        1,
    ),
    (
        N2,
        ("session-recording", "07-recording-stalled", "960x640"),
        1,
    ),
    (
        N2,
        ("session-recording", "11-recording-saved-no-key", "960x640"),
        2,
    ),
    (
        N2,
        ("session-recording", "16-recording-ready", "1200x800"),
        2,
    ),
    (
        N2,
        ("session-recording", "16-recording-ready", "1920x1080"),
        2,
    ),
    (
        N2,
        ("session-recording", "16-recording-ready", "960x640"),
        2,
    ),
    (
        N2,
        ("session-recording", "17-processing-speech", "960x640"),
        1,
    ),
    (
        N2,
        ("session-recording", "19-processing-cancelled", "1200x800"),
        1,
    ),
    (
        N2,
        ("session-recording", "19-processing-cancelled", "1920x1080"),
        1,
    ),
    (
        N2,
        ("session-recording", "19-processing-cancelled", "960x640"),
        1,
    ),
    (
        N2,
        ("session-recording", "20-processing-second-pass", "960x640"),
        1,
    ),
    (
        N2,
        (
            "session-recording",
            "21-processing-failed-elsewhere",
            "1200x800",
        ),
        1,
    ),
    (
        N2,
        (
            "session-recording",
            "21-processing-failed-elsewhere",
            "1920x1080",
        ),
        1,
    ),
    (
        N2,
        (
            "session-recording",
            "21-processing-failed-elsewhere",
            "960x640",
        ),
        1,
    ),
    (
        N2,
        ("session-recording", "22-processing-failed", "1200x800"),
        1,
    ),
    (
        N2,
        ("session-recording", "22-processing-failed", "1920x1080"),
        1,
    ),
    (
        N2,
        ("session-recording", "22-processing-failed", "960x640"),
        1,
    ),
    (
        N2,
        ("session-recording", "23-recording-processed", "1200x800"),
        1,
    ),
    (
        N2,
        ("session-recording", "23-recording-processed", "1920x1080"),
        1,
    ),
    (
        N2,
        ("session-recording", "23-recording-processed", "960x640"),
        1,
    ),
    (N2, ("session-recording", "24-marker-added", "1200x800"), 1),
    (N2, ("session-recording", "24-marker-added", "1920x1080"), 1),
    (N2, ("session-recording", "24-marker-added", "960x640"), 1),
    (
        N2,
        ("session-recording", "30-recording-warned", "1200x800"),
        1,
    ),
    (
        N2,
        ("session-recording", "30-recording-warned", "1920x1080"),
        1,
    ),
    (
        N2,
        ("session-recording", "30-recording-warned", "960x640"),
        1,
    ),
    (
        N2,
        ("session-recording", "31-process-again-confirm", "960x640"),
        1,
    ),
    (
        N2,
        (
            "session-recording",
            "32-recording-processing-again",
            "1200x800",
        ),
        1,
    ),
    (
        N2,
        (
            "session-recording",
            "32-recording-processing-again",
            "1920x1080",
        ),
        1,
    ),
    (
        N2,
        (
            "session-recording",
            "32-recording-processing-again",
            "960x640",
        ),
        1,
    ),
    (
        N2,
        ("session-recording", "33-recording-row-menu", "1200x800"),
        1,
    ),
    (
        N2,
        ("session-recording", "33-recording-row-menu", "1920x1080"),
        1,
    ),
    (
        N2,
        ("session-recording", "33-recording-row-menu", "960x640"),
        1,
    ),
    (
        N2,
        (
            "session-recording",
            "35-recording-stopped-elsewhere",
            "960x640",
        ),
        1,
    ),
    (
        N2,
        ("session-recording", "36-recording-stopped", "1200x800"),
        1,
    ),
    (
        N2,
        ("session-recording", "36-recording-stopped", "1920x1080"),
        1,
    ),
    (
        N2,
        ("session-recording", "36-recording-stopped", "960x640"),
        1,
    ),
    (
        N2,
        (
            "session-recording",
            "43-processing-while-recording-consent",
            "1200x800",
        ),
        1,
    ),
    (
        N2,
        (
            "session-recording",
            "43-processing-while-recording-consent",
            "1920x1080",
        ),
        1,
    ),
    (
        N2,
        (
            "session-recording",
            "43-processing-while-recording-consent",
            "960x640",
        ),
        1,
    ),
    (
        N2,
        ("session-recording", "44-processing-with-no-run", "1200x800"),
        3,
    ),
    (
        N2,
        (
            "session-recording",
            "44-processing-with-no-run",
            "1920x1080",
        ),
        3,
    ),
    (
        N2,
        ("session-recording", "44-processing-with-no-run", "960x640"),
        3,
    ),
    (
        N2,
        (
            "session-recording",
            "45-processing-with-setup-run",
            "1200x800",
        ),
        3,
    ),
    (
        N2,
        (
            "session-recording",
            "45-processing-with-setup-run",
            "1920x1080",
        ),
        3,
    ),
    (
        N2,
        (
            "session-recording",
            "45-processing-with-setup-run",
            "960x640",
        ),
        3,
    ),
    (
        N2,
        (
            "session-recording",
            "46-processing-with-running-run",
            "1200x800",
        ),
        3,
    ),
    (
        N2,
        (
            "session-recording",
            "46-processing-with-running-run",
            "1920x1080",
        ),
        3,
    ),
    (
        N2,
        (
            "session-recording",
            "46-processing-with-running-run",
            "960x640",
        ),
        3,
    ),
    (
        N2,
        (
            "session-recording",
            "47-processing-with-ended-run",
            "1200x800",
        ),
        3,
    ),
    (
        N2,
        (
            "session-recording",
            "47-processing-with-ended-run",
            "1920x1080",
        ),
        3,
    ),
    (
        N2,
        (
            "session-recording",
            "47-processing-with-ended-run",
            "960x640",
        ),
        3,
    ),
    // settings
    (G1B, ("settings", "(after the last state)", "960x640"), 1),
    (G1B, ("settings", "09-edit-campaign", "960x640"), 1),
    (
        G1B,
        ("settings", "11-campaign-details-discarded", "960x640"),
        1,
    ),
    (G1B, ("settings", "16-touchpad-settings", "960x640"), 1),
    (N2, ("settings", "15-sync-stopped-recovery", "1200x800"), 1),
    (N2, ("settings", "15-sync-stopped-recovery", "1920x1080"), 1),
    (N2, ("settings", "15-sync-stopped-recovery", "960x640"), 1),
    // signed-in
    (G1B, ("signed-in", "15-link-tab-during-run", "960x640"), 2),
    // trackers
    (
        G6,
        ("trackers", "03-trackers-three-notices", "1920x1080"),
        1,
    ),
    (
        G6,
        ("trackers", "07-tracker-history-in-the-reader", "1200x800"),
        1,
    ),
    (
        G6,
        ("trackers", "07-tracker-history-in-the-reader", "1920x1080"),
        1,
    ),
    (
        G6,
        ("trackers", "07-tracker-history-in-the-reader", "960x640"),
        1,
    ),
    (
        G6,
        ("trackers", "08-tracker-history-removed", "1200x800"),
        1,
    ),
    (G6, ("trackers", "08-tracker-history-removed", "960x640"), 1),
    // travel-checks
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
    // world
    (G1B, ("world", "08-world-generated-npc-saved", "960x640"), 1),
    (G1B, ("world", "10-npc-add-to-run-form", "960x640"), 1),
    // world-facts
    (
        G6,
        ("world-facts", "07-group-joined-with-log", "960x640"),
        1,
    ),
    (G6, ("world-facts", "09-log-line-added", "960x640"), 1),
    (
        G6,
        ("world-facts", "11-group-joined-with-log-in-prep", "960x640"),
        1,
    ),
    // world-note-duplicated
    (
        G6,
        (
            "world-note-duplicated",
            "02-duplicate-npc-repaired-from-encounter",
            "1920x1080",
        ),
        1,
    ),
    // world-part-of-safety
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
    // world-places
    (
        G1C,
        ("world-places", "07-place-change-keeps-disk-edit", "960x640"),
        2,
    ),
];
