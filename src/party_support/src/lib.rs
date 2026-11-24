//! Live party sheets and their shared presentation.
extern crate self as splat;
use splat_character::character;
use splat_content::content;
pub use splat_design::{field_id, sentence};

#[cfg(test)]
use splat_content_ui::test_support;
use splat_content_ui::{
    auth, content_workspace, design, dndbeyond, hermetic, persistence, rolls, sheet_writer,
};

#[path = "../../combat_sync.rs"]
pub mod combat_sync;
#[path = "../sheets.rs"]
pub mod party;
#[path = "../../party_bar.rs"]
pub mod party_bar;
#[path = "../../party_stream.rs"]
mod party_stream;
#[path = "../../party_sync.rs"]
pub mod party_sync;
