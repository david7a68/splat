//! Settings: screenshots of the egui `settings` and `empty` scenarios' main states, and the
//! account tab's flows (Connect's reason, Edit details with Discard and its Undo, Remove with
//! its confirm, Add campaign).
use crate::gui::harness::Shell;
use crate::settings::Account;
use crate::visual_tests::fixture::Seed;
use crate::{Page, SettingsTab};
use gpui::{px, size};

fn settings(name: &str, seed: Seed) -> Shell {
    let mut shell = Shell::new(name, seed);
    shell.click("Settings");
    assert_eq!(shell.read(|root| root.app.page), Page::Settings);
    shell
}

/// The connect form: Connect waits for a cookie and says why; a typed cookie makes it the
/// primary action; each account state has its words.
#[test]
fn account_states() {
    let mut shell = settings("settings", Seed::Campaign);
    shell.update(|root, _, _| root.app.settings.simulate(Account::NotConnected));
    shell.capture("settings__01-connect-form__1200x800");
    shell.click("Paste sign-in cookie");
    shell.type_text("synthetic-not-a-real-cookie");
    assert_eq!(
        shell.read(|root| root.app.settings.cookie().to_owned()),
        "synthetic-not-a-real-cookie"
    );
    shell.capture("settings__03-cookie-entered__1200x800");
    shell.click("How to get your sign-in cookie");
    assert!(shell.read(|root| root.pages.settings.cookie_help_open));
    shell.capture("settings__14-cookie-help__1200x800");
    shell.click("How to get your sign-in cookie");
    for (state, step) in [
        (Account::Busy, "05-account-connecting"),
        (Account::Connected, "06-account-connected"),
        (Account::KeyringWarning, "07-account-keyring-warning"),
        (Account::Failed, "04-connect-failed"),
    ] {
        shell.update(|root, _, _| root.app.settings.simulate(state));
        shell.capture(&format!("settings__{step}__1200x800"));
    }
    // Connected: Disconnect is there, and the sign-in form is not.
    shell.update(|root, _, _| root.app.settings.simulate(Account::Connected));
    assert!(shell.bounds("Disconnect").is_some());
    assert!(shell.bounds("Paste sign-in cookie").is_none());
    let mut shell = Shell::with_size("settings", Seed::Campaign, size(px(960.0), px(640.0)));
    shell.click("Settings");
    shell.update(|root, _, _| root.app.settings.simulate(Account::NotConnected));
    shell.capture("settings__01-connect-form__960x640");
    // Narrow, the account is a compact line until the DM asks to connect.
    assert!(shell.bounds("Paste sign-in cookie").is_none());
    shell.click("Connect…");
    assert!(shell.bounds("Paste sign-in cookie").is_some());
}

/// The campaign list: Edit details opens the form under its row; an edit says Unsaved; Discard
/// brings the saved words back with an Undo in the footer, whose press puts the edit back.
#[test]
fn edit_details_discard_and_undo() {
    let mut shell = settings("settings", Seed::Campaign);
    shell.update(|root, _, _| root.app.settings.simulate(Account::NotConnected));
    shell.capture("settings__08-campaign-list__1200x800");
    let first = shell.read(|root| root.app.campaigns.all()[0].clone());
    shell.click(&format!("id:campaign-edit-{}", first.id));
    assert_eq!(
        shell.read(|root| root.app.campaigns.editing()),
        Some(first.id)
    );
    assert!(shell.read(|root| root.app.campaigns.form_unchanged()));
    shell.capture("settings__09-edit-campaign__1200x800");
    shell.click(&format!("id:campaign_name-{}", first.id));
    shell.type_text(" Draft");
    assert!(!shell.read(|root| root.app.campaigns.form_unchanged()));
    shell.click("Discard");
    assert_eq!(
        shell.read(|root| root.app.campaigns.form_words().0.to_owned()),
        first.name
    );
    assert!(shell.bounds("Undo discard of Campaign details").is_some());
    shell.capture("settings__11-campaign-details-discarded__1200x800");
    shell.click("Undo discard of Campaign details");
    assert_eq!(
        shell.read(|root| root.app.campaigns.form_words().0.to_owned()),
        format!("{} Draft", first.name)
    );
    assert!(shell.bounds("Undo discard of Campaign details").is_none());
    // Cancel ends the edit (after Discard brings the saved words back).
    shell.click("Discard");
    shell.click("Cancel");
    assert_eq!(shell.read(|root| root.app.campaigns.editing()), None);
}

/// Remove… opens the confirm dialog; Escape keeps the campaign; Remove from Splat forgets it.
#[test]
fn remove_asks_first() {
    let mut shell = settings("settings", Seed::Campaign);
    let second = shell.read(|root| root.app.campaigns.all()[1].clone());
    let count = shell.read(|root| root.app.campaigns.all().len());
    shell.click(&format!("id:campaign-more-{}", second.id));
    shell.capture("settings__13-campaign-row-menu__1200x800");
    shell.click("Remove…");
    assert!(shell.bounds("Remove from Splat").is_some());
    shell.capture("settings__10-confirm-remove-campaign__1200x800");
    shell.press("escape");
    assert!(shell.bounds("Remove from Splat").is_none());
    assert_eq!(shell.read(|root| root.app.campaigns.all().len()), count);
    shell.click(&format!("id:campaign-more-{}", second.id));
    shell.click("Remove…");
    shell.click("Remove from Splat");
    assert_eq!(shell.read(|root| root.app.campaigns.all().len()), count - 1);
    assert!(
        shell.read(|root| !root.app.campaigns.all().iter().any(|c| c.id == second.id))
    );
}

/// With no campaign, the add form is open; Add waits for a name and a folder, then adds the
/// campaign and says so in the footer.
#[test]
fn add_the_first_campaign() {
    let mut shell = settings("empty", Seed::Empty);
    assert!(shell.read(|root| root.app.settings_tab == SettingsTab::Account));
    shell.capture("empty__06-settings-no-campaigns__1200x800");
    assert_eq!(
        shell.read(|root| root.app.campaigns.form_reason_words()),
        Some("Enter a campaign name and a folder first.")
    );
    let folder = tempfile::tempdir().expect("a folder");
    shell.click("Campaign name");
    shell.type_text("Salt Road");
    assert_eq!(
        shell.read(|root| root.app.campaigns.form_reason_words()),
        Some("Choose a folder first.")
    );
    let path = folder.path().to_string_lossy().into_owned();
    shell.update(|root, _, _| root.app.campaigns.set_form_path(&path));
    shell.click("Add campaign");
    assert_eq!(shell.read(|root| root.app.campaigns.all().len()), 1);
    assert_eq!(
        shell.read(|root| root.app.campaigns.active_campaign().map(|c| c.name)),
        Some("Salt Road".to_owned())
    );
    assert!(shell.bounds("Build an encounter").is_some());
    shell.capture("empty__07-campaign-added__1200x800");
}

/// Settings → Recording and processing draws its groups.
#[test]
fn recording_tab() {
    let mut shell = settings("settings-recording", Seed::Campaign);
    shell.click("Recording and processing");
    assert!(shell.read(|root| root.app.settings_tab == SettingsTab::Recording));
    assert!(shell.bounds("Speech API key").is_some());
    shell.capture("settings-recording__01-tab__1200x800");
}
