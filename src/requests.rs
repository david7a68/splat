//! The requests that one part of the app leaves for another in the state context
//! (`state_context::StateContext`), each a typed slot. A page or a reader sets one when the DM
//! presses something whose answer belongs to another page; the frame logic takes it
//! (`Splat::route`, `Splat::poll_background`, `Splat::follow_world`) and acts.
use crate::state_context::Slot;
use std::{path::PathBuf, sync::Arc};
use uuid::Uuid;

/// The content index that readers resolve their links in (set by `Splat::poll_background`).
pub const LINK_INDEX: Slot<Arc<crate::lookup::ContentIndex>> = Slot::new("reader_link_index");

/// The note a reader showed when the DM pressed a note link in it: `None` for a reader with no
/// note of its own.
pub const CLICKED_READER_DOCUMENT: Slot<Option<Uuid>> = Slot::new("clicked_reader_document");

/// Use a package's entry in the campaign: the package and the entry.
pub const USE_PACKAGE_ENTRY: Slot<(Uuid, Uuid)> = Slot::new("use_package_entry");

/// Open a file that needs repair: in the World page when it is a campaign note, else in the
/// Library.
pub const OPEN_LIBRARY_REPAIR: Slot<PathBuf> = Slot::new("open_library_repair");

/// Show an encounter's NPC on the World page.
pub const REVEAL_ENCOUNTER_NPC: Slot<Uuid> = Slot::new("reveal_encounter_npc");

/// Edit the run's encounter on the Encounters page.
pub const EDIT_RUN_ENCOUNTER: Slot<Uuid> = Slot::new("edit_run_encounter");

/// Open the campaign's Packages tab.
pub const OPEN_CAMPAIGN_PACKAGES: Slot<bool> = Slot::new("open_campaign_packages");

/// Open the account settings.
pub const OPEN_ACCOUNT_SETTINGS: Slot<bool> = Slot::new("open_account_settings");

/// An encounter's identity changed on disk, so the run and the plan read it again.
pub const ENCOUNTER_IDENTITY_CHANGED: Slot<bool> = Slot::new("encounter_identity_changed");
