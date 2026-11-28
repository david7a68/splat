//! Campaign package selection and repair.
extern crate self as splat;

use splat_character::character;
use splat_content::content;
use splat_content_ui::hermetic;
use splat_content_ui::requests;
use splat_content_ui::{
    campaigns, content_actions, content_workspace, design, library, persistence, world,
};
use splat_design::{choice, sentence, state_context};

#[path = "../../campaign_content.rs"]
pub mod campaign_content;
