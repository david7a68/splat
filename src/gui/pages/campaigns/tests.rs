//! Campaign: screenshots of its tabs (egui `campaign-content`, `signed-in` step 15), and the
//! Link and folder tab's flows (Edit details from the folder block; the session folder's draft).
use crate::gui::harness::Shell;
use crate::visual_tests::fixture::Seed;
use crate::{CampaignTab, Page};

fn campaign(name: &str) -> Shell {
    let mut shell = Shell::new(name, Seed::Campaign);
    shell.click("Campaign");
    assert_eq!(shell.read(|root| root.app.page), Page::Campaign);
    shell
}

/// Link and folder: its blocks; Edit details opens the details form with the campaign's words;
/// an edit of the session folder makes a draft that Discard ends.
#[test]
fn link_and_folder() {
    let mut shell = campaign("campaign-link");
    shell.click("Link and folder");
    assert!(shell.read(|root| root.app.campaign_tab == CampaignTab::Link));
    shell.capture("campaign-link__01-link-and-folder__1200x800");
    let active = shell.read(|root| root.app.campaigns.active_campaign().expect("a campaign"));
    shell.click("id:link-edit-details");
    assert_eq!(shell.read(|root| root.app.campaigns.editing()), Some(active.id));
    assert_eq!(
        shell.read(|root| root.app.campaigns.form_words().0.to_owned()),
        active.name
    );
    shell.capture("campaign-link__02-edit-details__1200x800");
    let saved = shell.read(|root| root.app.sessions.folder_text().to_owned());
    shell.click("id:session_folder");
    shell.type_text(" more");
    assert_eq!(
        shell.read(|root| root.app.sessions.folder_text().to_owned()),
        format!("{saved} more")
    );
    shell.click("Discard");
    assert_eq!(shell.read(|root| root.app.sessions.folder_text().to_owned()), saved);
}

/// Packages: the list and the chosen package; a box changes the choices (Unsaved), and Discard
/// brings Campaign.md's choices back with an Undo in the footer.
#[test]
fn packages_choose_and_discard() {
    let mut shell = campaign("campaign-content");
    assert!(shell.read(|root| root.app.campaign_tab == CampaignTab::Packages));
    for _ in 0..200 {
        if shell.read(|root| !root.app.campaign_content.loading()) {
            break;
        }
        shell.wait(std::time::Duration::from_millis(50));
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    shell.capture("campaign-content__01-packages-tab__1200x800");
    let rows = shell.read(|root| root.app.campaign_content.package_rows(&root.app.campaigns));
    let first = rows.first().expect("a package");
    let (id, checked) = (first.id, first.checked);
    shell.click(&format!("id:content-use-{id}"));
    let now = shell.read(|root| root.app.campaign_content.package_rows(&root.app.campaigns));
    assert_eq!(now[0].checked, !checked, "the box changes the choice");
    assert!(shell.read(|root| root.app.campaign_content.selections_changed()));
    shell.capture("campaign-content__04-selection-changed__1200x800");
    shell.click("id:content-discard");
    assert!(!shell.read(|root| root.app.campaign_content.selections_changed()));
    let after = shell.read(|root| root.app.campaign_content.package_rows(&root.app.campaigns));
    assert_eq!(after[0].checked, checked, "Discard brings the saved choice back");
}
