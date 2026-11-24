//! Party page for the desktop app.
pub use splat_design::{sentence, word};

use splat_content_ui::design;
#[cfg(test)]
use splat_party::party_sync;

#[path = "../../party.rs"]
mod page;
pub use page::{PartyHeader, PartyPage, PartyPress, tonight_words};
pub use page::{class_line, classes_and_level, compare_values, most_recent_sheet, technical};
