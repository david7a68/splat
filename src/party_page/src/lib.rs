//! Party page for the desktop app.
extern crate self as splat;
use splat_character::character;
use splat_content::content;
pub use splat_design::{field_id, sentence};

use splat_content_ui::{art, auth, design, hermetic};
use splat_party::{combat_sync, party_sync};

#[path = "../../party.rs"]
mod page;
pub use page::PartyPage;
