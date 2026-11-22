//! The World page and its notes and repairs.
extern crate self as splat;
use splat_content::content;

use splat_content_ui::{
    content_workspace, design, library, lookup, markdown_view, npc_generator, persistence, world,
};
use splat_design::{field_id, sentence, word};

#[cfg(test)]
use splat_character::character;
#[cfg(test)]
#[path = "../../trackers/reader.rs"]
mod tracker_reader;

#[path = "../../world/page.rs"]
pub mod page;
