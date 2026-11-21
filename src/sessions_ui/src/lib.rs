//! Session notes, the Timeline page, and recording and processing.
extern crate self as splat;
use splat_content::content;

pub use splat_design::design::layout;
pub use splat_design::{choice, create_reasons, field_id, frame_clock, key, lead, sentence, word};

use splat_content_ui::{
    auth, content_workspace, design, hermetic, library, lookup, markdown_source, markdown_view,
    npc_generator, persistence, roll_table, world,
};
use splat_session_records::{difficulty, encounters};
mod encounter_run {
    #[cfg(test)]
    pub use splat_encounters_ui::encounter_run::write_ended_run_with;
    pub use splat_session_records::encounter_run::*;
}
use splat_campaign::loot as loot_editor;

#[cfg(any(test, feature = "test-support"))]
use splat_content_ui::test_support;

pub mod timeline;

#[path = "../../credentials.rs"]
pub mod credentials;
#[path = "../../processing.rs"]
pub mod processing;
#[path = "../../recording.rs"]
pub mod recording;
#[path = "../../sessions.rs"]
pub mod sessions;
