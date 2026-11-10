//! Read-only bundled bytes, separate from the code that parses or displays them.
use include_dir::{Dir, include_dir};

pub static SRD: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../builtin/srd-5.2.1");
pub static DEFINITIONS: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/../../builtin/dnd2024-definitions");
pub static MECHANICS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../builtin/dnd2024-mechanics");

pub static FONTS: [(&str, &[u8]); 10] = [
    (
        "plex-regular",
        include_bytes!("../../../assets/fonts/IBMPlexSans-Regular.ttf"),
    ),
    (
        "plex-medium",
        include_bytes!("../../../assets/fonts/IBMPlexSans-Medium.ttf"),
    ),
    (
        "plex-semibold",
        include_bytes!("../../../assets/fonts/IBMPlexSans-SemiBold.ttf"),
    ),
    (
        "plex-bold",
        include_bytes!("../../../assets/fonts/IBMPlexSans-Bold.ttf"),
    ),
    (
        "serif-regular",
        include_bytes!("../../../assets/fonts/SourceSerif4-Regular.ttf"),
    ),
    (
        "serif-italic",
        include_bytes!("../../../assets/fonts/SourceSerif4-It.ttf"),
    ),
    (
        "serif-semibold",
        include_bytes!("../../../assets/fonts/SourceSerif4-Semibold.ttf"),
    ),
    (
        "serif-title",
        include_bytes!("../../../assets/fonts/SourceSerif4Subhead-Semibold.ttf"),
    ),
    (
        "serif-small",
        include_bytes!("../../../assets/fonts/SourceSerif4SmText-Regular.ttf"),
    ),
    (
        "serif-small-strong",
        include_bytes!("../../../assets/fonts/SourceSerif4SmText-Semibold.ttf"),
    ),
];

/// Built-in reports without layout whitespace.
pub static COVERAGE: &str = include_str!(concat!(env!("OUT_DIR"), "/coverage.json"));
pub static CONTRACTS: &str = include_str!(concat!(env!("OUT_DIR"), "/contracts.json"));
