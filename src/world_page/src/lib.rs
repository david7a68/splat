//! The World page and its notes and repairs.
extern crate self as splat;
use splat_content::content;

use splat_content_ui::{
    content_workspace, design, library, lookup, npc_generator, persistence, world,
};
use splat_design::sentence;

#[cfg(test)]
use splat_character::character;
#[cfg(test)]
#[path = "../../trackers/reader.rs"]
mod tracker_reader;

#[path = "../../world/page.rs"]
pub mod page;
