//! Party page for the desktop app.
pub use splat_design::{field_id, sentence, word};

use splat_content_ui::{art, auth, design, hermetic};
use splat_party::{combat_sync, party_sync};

#[path = "../../party.rs"]
mod page;
pub use page::PartyPage;
