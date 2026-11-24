//! The design system's plain parts (tokens, messages, kept results, the save row), the frame
//! clock and the state context, shared by every crate of the app.
#[path = "../mod.rs"]
pub mod design;
#[path = "../../frame_clock.rs"]
pub mod frame_clock;
#[path = "../../state_context.rs"]
pub mod state_context;

pub use design::{fact, layout, outcome, text_draft};

#[doc(hidden)]
pub mod ownership {
    pub use crate::design::components::ResultOrder;
    pub use crate::design::memo::{Changed, Key, Measure, Memo, Memos, Revised, Revision};
    pub use crate::design::results::*;
    pub use crate::design::save_row::{
        Diagnosis, DraftOrigin, DraftState, Primary, SaveWeight, SaveWord,
    };
}

#[cfg(test)]
fn workspace() -> &'static std::path::Path {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("workspace root")
}
