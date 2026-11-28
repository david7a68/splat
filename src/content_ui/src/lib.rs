//! World, Lookup, Library, and Campaign screens and their shared readers.
extern crate self as splat;
pub use splat_character::{character, memo};
pub use splat_content::content;
pub use splat_design::{choice, create_reasons, frame_clock, lead, sentence, state_context, word};
pub use splat_execution::execution;
pub mod file_batch {
    pub use splat_campaign::file_batch::*;
}
#[path = "../../art.rs"]
pub mod art;
#[path = "../../auth.rs"]
pub mod auth;
#[path = "../../campaigns.rs"]
pub mod campaigns;
#[path = "../../content_workspace.rs"]
pub mod content_workspace;
#[path = "../../copy_dialog.rs"]
pub mod copy_dialog;
#[path = "../../creature_filter.rs"]
pub mod creature_filter;
#[path = "../../dndbeyond.rs"]
pub mod dndbeyond;
#[path = "../../hermetic.rs"]
pub mod hermetic;
#[path = "../../history.rs"]
pub mod history;
#[path = "../../library/catalogue.rs"]
pub mod library;
#[path = "../../lookup.rs"]
pub mod lookup;
#[path = "../../markdown_source.rs"]
pub mod markdown_source;
#[path = "../../markdown_view.rs"]
pub mod markdown_view;
#[path = "../../matching.rs"]
pub mod matching;
#[path = "../../npc_generator.rs"]
pub mod npc_generator;
#[path = "../../persistence.rs"]
pub mod persistence;
#[path = "../../requests.rs"]
pub mod requests;
#[path = "../../roll_table.rs"]
pub mod roll_table;
#[path = "../../rolls.rs"]
pub mod rolls;
#[path = "../../sheet_writer.rs"]
pub mod sheet_writer;
#[path = "../../world.rs"]
pub mod world;

pub mod content_actions;
#[path = "../../design/content.rs"]
pub mod design;
pub mod roster;
pub mod session_notes;

#[cfg(any(test, feature = "test-support"))]
#[path = "../../test_support.rs"]
pub mod test_support;

#[cfg(test)]
fn workspace() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace folder")
}
