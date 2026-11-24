//! Shared widgets, layout, and messages for desktop features.
#[path = "../mod.rs"]
pub mod design;
#[path = "../../frame_clock.rs"]
pub mod frame_clock;

pub use design::{extras, fact, layout, outcome, pane, text_draft, value_block};
#[cfg(test)]
pub use egui;

#[doc(hidden)]
pub mod ownership {
    pub use crate::design::components::{Footer, ResultOrder};
    pub use crate::design::memo::{Changed, Key, Measure, Memo, Memos, Revised, Revision};
    pub use crate::design::results::*;
    pub use crate::design::save_row::{
        Diagnosis, DraftOrigin, DraftState, Primary, SaveWeight, SaveWord,
    };
}

#[cfg(test)]
#[path = "../input_frame.rs"]
mod input_frame;

#[cfg(test)]
fn workspace() -> &'static std::path::Path {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("workspace root")
}
