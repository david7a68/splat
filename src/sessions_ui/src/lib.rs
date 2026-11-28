//! Session notes, the Timeline page, and recording and processing.
extern crate self as splat;
use splat_content::content;

pub use splat_design::design::layout;
pub use splat_design::{choice, create_reasons, frame_clock, lead, sentence, state_context, word};

use splat_content_ui::{
    auth, content_workspace, design, hermetic, library, lookup, npc_generator, persistence,
    roll_table, world,
};
use splat_session_records::{difficulty, encounter_run, encounters};

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
