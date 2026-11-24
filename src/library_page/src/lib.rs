//! Library browsing and authoring.
extern crate self as splat;
use splat_content::content;
use splat_content_ui::{
    campaigns, content_workspace, design, hermetic, history, lookup, matching, persistence,
    roll_table, world,
};
use splat_design::{choice, create_reasons, sentence};

#[cfg(test)]
use splat_content_ui::test_support;

#[path = "../../library.rs"]
pub mod library;

#[cfg(test)]
fn workspace() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace folder")
}
