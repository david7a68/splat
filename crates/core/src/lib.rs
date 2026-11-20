//! Shared content, character, and execution interfaces for the app and verification CLI.
pub use splat_character::{character, memo};
pub use splat_execution::execution;

pub use splat_campaign::{
    encounter_run, file_batch, folder_check, loot, note, roll_table, timeline, trackers,
};

pub mod matching;

pub mod content {
    pub use splat_content::content::*;

    pub mod definitions {
        pub use splat_content::content::definitions::*;
        pub use splat_execution::execution::catalogue::{CatalogueReport, verify};
    }
}
