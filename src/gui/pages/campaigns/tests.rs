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
    assert_eq!(
        shell.read(|root| root.app.campaigns.editing()),
        Some(active.id)
    );
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
    assert_eq!(
        shell.read(|root| root.app.sessions.folder_text().to_owned()),
        saved
    );
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
    assert_eq!(
        after[0].checked, checked,
        "Discard brings the saved choice back"
    );
}

/// Rules and runs: an edit makes the form Unsaved; Discard brings the saved settings back with
/// its Undo in the footer, and the Undo puts the edit back.
#[test]
fn rules_and_runs_discard_and_undo() {
    let mut shell = campaign("campaign-rules");
    shell.click("Rules and runs");
    assert!(shell.read(|root| root.app.campaign_tab == CampaignTab::RunSettings));
    for _ in 0..200 {
        if shell.read(|root| root.app.encounters.run_form().is_some()) {
            break;
        }
        shell.wait(std::time::Duration::from_millis(50));
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    shell.capture("campaign-rules__01-rules-and-runs__1200x800");
    let saved = shell.read(|root| root.app.encounters.run_form().expect("the form").folder);
    let id = shell.read(|root| root.app.campaigns.active_campaign().expect("a campaign").id);
    shell.click(&format!("id:run_folder-{id}"));
    shell.type_text(" 2");
    assert_eq!(
        shell.read(|root| root.app.encounters.run_form().expect("the form").folder),
        format!("{saved} 2")
    );
    shell.click("Discard");
    assert_eq!(
        shell.read(|root| root.app.encounters.run_form().expect("the form").folder),
        saved
    );
    shell.capture("campaign-rules__02-discarded__1200x800");
    shell.click("Undo discard of Rules and runs");
    assert_eq!(
        shell.read(|root| root.app.encounters.run_form().expect("the form").folder),
        format!("{saved} 2")
    );
}

/// Wait for the Packages tab's load (it reads on a thread).
fn content_loaded(shell: &mut Shell) {
    for _ in 0..200 {
        if shell.read(|root| !root.app.campaign_content.loading()) {
            break;
        }
        shell.wait(std::time::Duration::from_millis(50));
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    shell.settle();
}

/// An invalid note in the campaign (egui `campaign-content` 11-14): its message offers Repair,
/// which opens the note's editor and save row; Discard brings the file's text back with an
/// Undo, and a draft Save accepts saves with Ctrl+S, after which the page reads the campaign
/// again and the repair ends.
#[test]
fn packages_repair_an_invalid_note() {
    const BROKEN: &str = "Notes/Broken.md";
    let mut shell = campaign("campaign-content-repair");
    content_loaded(&mut shell);
    let active = shell.read(|root| root.app.campaigns.active_campaign().expect("a campaign"));
    std::fs::create_dir_all(active.path.join("Notes")).unwrap();
    std::fs::write(
        active.path.join(BROKEN),
        "---\nsplat_schema: 2\nsplat_id: [\n---\n",
    )
    .unwrap();
    shell.click("Reload from disk");
    content_loaded(&mut shell);
    let open = |shell: &mut Shell| {
        shell.read(|root| {
            let open = root.pages.campaigns.repairs_open.clone();
            root.app
                .campaign_content
                .repair_rows(&|path| open.contains(path))
        })
    };
    let rows = open(&mut shell);
    assert_eq!(rows.len(), 1, "the invalid note is in repair");
    assert!(rows[0].words.starts_with("Notes/Broken.md cannot be read."));
    assert!(
        shell.bounds("Save repaired note").is_none(),
        "closed, no editor"
    );
    shell.capture("campaign-content__11-repair-needed__1200x800");
    shell.click("Repair");
    assert!(
        shell.bounds("Save repaired note").is_some(),
        "Repair opens the editor"
    );
    assert!(shell.bounds("Close repair").is_some());
    shell.capture("campaign-content__12-repair-open__1200x800");
    // A draft Save accepts: Unsaved, Save is the primary.
    let valid = crate::library::note_text("Broken", "rule", None);
    let edit = |shell: &mut Shell, words: String| {
        shell.update(move |root, _, _| {
            let active = root.app.campaigns.active_campaign().expect("a campaign");
            root.app
                .campaign_content
                .edit_repair(&active, BROKEN, &words);
        });
    };
    edit(&mut shell, valid.clone());
    assert_eq!(open(&mut shell)[0].draft, valid);
    shell.capture("campaign-content__13-repair-dirty__1200x800");
    // Discard: the file's text comes back, with its Undo in the footer.
    shell.click("Discard");
    assert!(open(&mut shell)[0].draft.contains("splat_id: ["));
    assert!(
        shell.bounds("Undo discard of Broken.md").is_some(),
        "{:?}",
        crate::gui::kit::probe::names()
            .into_iter()
            .filter(|name| name.contains("Undo"))
            .collect::<Vec<_>>()
    );
    shell.click("Undo discard of Broken.md");
    assert_eq!(
        open(&mut shell)[0].draft,
        valid,
        "Undo gives the draft back"
    );
    // Ctrl+S saves the repair; the page reads the campaign again and the repair ends.
    shell.press("ctrl-s");
    content_loaded(&mut shell);
    assert_eq!(
        std::fs::read_to_string(active.path.join(BROKEN)).unwrap(),
        valid
    );
    assert!(open(&mut shell).is_empty(), "the repaired note reads again");
}
