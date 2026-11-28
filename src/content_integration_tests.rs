use super::*;
use crate::fixtures::{Seed, World};
use crate::state_context::StateContext;

#[test]
fn package_copy_keeps_original_entries_encounters_and_byte_exact_undo() {
    let temp = tempfile::tempdir().unwrap();
    let world = World::build(
        temp.path(),
        &temp.path().join("cache"),
        "copy-test",
        Seed::Campaign,
    )
    .unwrap();
    let app = world.app();
    content_workspace::package_copy_keeps_original_entries_encounters_and_byte_exact_undo(
        &temp,
        &app.campaigns.modules()[0],
        app.campaigns.active_campaign().unwrap(),
    );
}

#[test]
fn copy_registration_and_undo_keep_campaign_choices_and_later_edits() {
    let temp = tempfile::tempdir().unwrap();
    let world = World::build(
        temp.path(),
        &temp.path().join("cache"),
        "copy-use",
        Seed::Campaign,
    )
    .unwrap();
    let mut app = world.app();
    library::copy_registration_and_undo_keep_campaign_choices_and_later_edits(
        &temp,
        &mut app.campaigns,
    );
}

#[test]
fn iter14_e_changed_files_end_every_undo_result() {
    let temp = tempfile::tempdir().unwrap();
    let world = World::build(
        temp.path(),
        &temp.path().join("cache"),
        "undo-ends",
        Seed::Campaign,
    )
    .unwrap();
    let mut app = world.app();
    let mut campaigns = std::mem::take(&mut app.campaigns);
    let frame_context = StateContext::new();
    library::iter14_e_changed_files_end_every_undo_result(
        &temp,
        &mut campaigns,
        |campaigns, library, ctx| {
            app.campaigns = std::mem::take(campaigns);
            app.library = library;
            app.route(ctx);
            *campaigns = std::mem::take(&mut app.campaigns);
            std::mem::take(&mut app.library)
        },
        |library, campaigns| library.begin(&frame_context, campaigns),
    );
}

#[test]
fn applied_changes_reload_the_catalogue_and_failures_are_shown() {
    campaign_content::applied_changes_reload_the_catalogue_and_failures_are_shown(|content| {
        let app = Splat {
            campaign_content: content,
            ..Default::default()
        };
        assert_eq!(app.blocked_switch(), Some("Wait until Apply has finished."));
        app.campaign_content
    });
}
