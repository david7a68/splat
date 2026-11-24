//! Reader and roll widgets shared by the content screens and desktop.
pub use splat_design::design::*;

pub mod reader {
    pub use splat_design::design::reader::*;
}

pub mod fact {
    pub use splat_design::design::fact::{line, tile_label, words};
}
