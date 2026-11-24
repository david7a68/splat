//! Settings: screenshots of the `settings` and `empty` scenarios' main states, and the
//! account tab's flows (Connect's reason, Edit details with Discard and its Undo, Remove with
//! its confirm, Add campaign).
use crate::fixtures::Seed;
use crate::gui::harness::Shell;
use crate::settings::Account;
use crate::{Page, SettingsTab};
use gpui::{px, size};

fn settings(name: &str, seed: Seed) -> Shell {
    let mut shell = Shell::new(name, seed);
    shell.click("Settings");
    assert_eq!(shell.read(|root| root.app.page), Page::Settings);
    shell
}

/// The state the `settings` scenario's campaign steps start from (egui ran them after the
/// account steps): the paused run read, the cookie typed, and the account connected and then
/// lost, so the party's sync stopped and the rail says so.
fn after_the_account_steps(shell: &mut Shell) {
    for _ in 0..10 {
        if !shell.read(|root| root.app.encounters.busy()) {
            break;
        }
        shell.update(|root, _, _| root.app.encounters.wait_for_load());
    }
    shell.update(|root, _, _| root.app.settings.simulate(Account::Connected));
    // Combat sync starts once the party's sheet is read (on a thread).
    for _ in 0..200 {
        if shell.read(|root| root.app.party.combat.enabled) {
            break;
        }
        shell.wait(std::time::Duration::from_millis(50));
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    shell.update(|root, _, _| {
        root.app.settings.simulate(Account::NotConnected);
        root.app.settings.set_cookie("synthetic-not-a-real-cookie");
    });
    shell.settle();
    assert!(
        shell.read(|root| root.app.party.sync_stopped),
        "the party's sync stopped with the account"
    );
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
    // The keyring warning names the sign-in; its Connect again opens the sign-in form, whose
    // cookie field asks for the keyboard.
    shell.update(|root, _, _| root.app.settings.simulate(Account::KeyringWarning));
    assert!(shell.bounds("Paste sign-in cookie").is_none());
    shell.click("Connect again");
    assert!(shell.read(|root| root.app.settings.signing_in_again()));
    assert!(shell.bounds("Paste sign-in cookie").is_some());
    // The cookie's help, last in the scenario, after the account was lost.
    after_the_account_steps(&mut shell);
    shell.click("How to get your sign-in cookie");
    assert!(shell.read(|root| root.pages.settings.cookie_help_open));
    shell.capture("settings__14-cookie-help__1200x800");
    shell.click("How to get your sign-in cookie");
    assert!(!shell.read(|root| root.pages.settings.cookie_help_open));
    let mut shell = Shell::with_size("settings", Seed::Campaign, size(px(960.0), px(640.0)));
    shell.click("Settings");
    shell.update(|root, _, _| root.app.settings.simulate(Account::NotConnected));
    shell.capture("settings__01-connect-form__960x640");
    // Narrow, the account is a compact line until the DM asks to connect.
    assert!(shell.bounds("Paste sign-in cookie").is_none());
    shell.click("Connect…");
    assert!(shell.bounds("Paste sign-in cookie").is_some());
    // Connected, the narrow account opens its sign-in and Disconnect as a caret section.
    shell.update(|root, _, _| {
        root.pages.settings.compact_open = false;
        root.app.settings.simulate(Account::Connected);
    });
    assert!(shell.bounds("Disconnect").is_none());
    shell.click("Sign-in details");
    assert!(shell.read(|root| root.pages.settings.sign_in_details_open));
    assert!(shell.bounds("Disconnect").is_some());
}

/// The campaign list: Edit details opens the form under its row; an edit says Unsaved; Discard
/// brings the saved words back with an Undo in the footer, whose press puts the edit back.
#[test]
fn edit_details_discard_and_undo() {
    let mut shell = settings("settings", Seed::Campaign);
    after_the_account_steps(&mut shell);
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
    after_the_account_steps(&mut shell);
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
    assert!(shell.read(|root| !root.app.campaigns.all().iter().any(|c| c.id == second.id)));
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

/// Signed in (scenario `signed-in` 02, 03): Add from D&D Beyond lists the account's campaigns, an
/// added one as Added and the others with Add…, which opens the import form under its row.
/// On Link and folder, a D&D Beyond campaign linked to another campaign is a blocked choice
/// with its reason: choosing it changes nothing.
#[test]
fn signed_in_remote_list_and_link() {
    let mut shell = settings("signed-in", Seed::Campaign);
    shell.update(|root, _, _| root.app.settings.simulate(Account::Connected));
    if shell.bounds("Add Salt Road from D&D Beyond").is_none() {
        shell.click("Add from D&D Beyond");
    }
    for _ in 0..200 {
        if shell.read(|root| root.app.dndbeyond.loaded()) {
            break;
        }
        shell.wait(std::time::Duration::from_millis(50));
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    shell.settle();
    assert!(shell.bounds("Add Salt Road from D&D Beyond").is_some());
    // The added campaign is listed, as Added, with no Add….
    assert!(
        shell.read(|root| root
            .app
            .dndbeyond
            .remotes()
            .find(|remote| remote.name == "The Emberfall Marches")
            .is_some_and(|remote| root.app.campaigns.is_added(remote.id))),
        "The Emberfall Marches is listed and added"
    );
    assert!(
        shell
            .bounds("Add The Emberfall Marches from D&D Beyond")
            .is_none()
    );
    shell.capture("signed-in__02-remote-campaign-list__1200x800");
    shell.click("Add Salt Road from D&D Beyond");
    assert!(
        shell.read(|root| root
            .app
            .dndbeyond
            .remotes()
            .any(|remote| remote.name == "Salt Road" && root.app.campaigns.importing(remote.id))),
        "Add… opens the import form of that campaign"
    );
    shell.capture("signed-in__03-import-remote-campaign__1200x800");
    // The link picker: a campaign another local campaign links is blocked, with its reason.
    shell.click("Campaign");
    shell.click("Link and folder");
    let (choices, current) = shell.read(|root| {
        (
            root.app.dndbeyond.link_choices(&root.app.campaigns),
            root.app
                .campaigns
                .active_campaign()
                .and_then(|campaign| campaign.dndbeyond_campaign_id),
        )
    });
    let (_, name, _) = choices
        .iter()
        .find(|(_, _, lock)| lock.is_some())
        .expect("a campaign linked elsewhere")
        .clone();
    shell.click("D&D Beyond campaign link");
    assert!(
        shell.bounds(&name).is_some(),
        "the blocked choice is listed"
    );
    shell.capture("campaign-link__03-link-blocked-choice__1200x800");
    shell.click(&name);
    assert_eq!(
        shell.read(|root| root
            .app
            .campaigns
            .active_campaign()
            .and_then(|campaign| campaign.dndbeyond_campaign_id)),
        current,
        "a blocked choice changes nothing"
    );
}

/// Scenario `campaign-copy-registered`, step 01: a copy of the campaign's folder, added as a
/// campaign, is registered as a new campaign with a result that says so.
#[test]
fn campaign_copy_registered() {
    const NAME: &str = "Emberfall Again";
    let mut shell = settings("campaign-copy-registered", Seed::Campaign);
    let active = shell.read(|root| root.app.campaigns.active_campaign().expect("a campaign"));
    let copy = active.path.with_file_name("emberfall-copy");
    std::fs::create_dir_all(&copy).unwrap();
    std::fs::copy(active.path.join("Campaign.md"), copy.join("Campaign.md")).unwrap();
    if shell.bounds("Campaign name").is_none() {
        shell.click("Add campaign");
    }
    shell.click("Campaign name");
    shell.type_text(NAME);
    let path = copy.to_string_lossy().into_owned();
    shell.update(|root, _, _| root.app.campaigns.set_form_path(&path));
    shell.click("id:campaign-add");
    assert!(
        shell.bounds("Register as a new campaign").is_some(),
        "a copied folder offers to register as new: {:?}",
        crate::gui::kit::probe::names()
            .into_iter()
            .filter(|name| name.starts_with("Reg") || name.starts_with("id:campaign"))
            .collect::<Vec<_>>()
    );
    shell.click("Register as a new campaign");
    assert!(
        shell.read(|root| root.app.campaigns.all().iter().any(|c| c.name == NAME)),
        "the copy is registered"
    );
    shell.capture("campaign-copy-registered__01-copy-registered__1200x800");
}
