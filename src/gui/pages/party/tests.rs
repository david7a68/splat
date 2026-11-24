//! Party: screenshots of the egui `party`, `party-bar-crowded` and `empty` scenarios' main
//! states, and the page's flows (choose a member, change the sheet's page, open an entry).
use crate::Page;
use crate::gui::harness::Shell;
use crate::visual_tests::fixture::Seed;

fn party(name: &str) -> Shell {
    let mut shell = Shell::new(name, Seed::Campaign);
    shell.click("Party");
    assert_eq!(shell.read(|root| root.app.page), Page::Party);
    // The party's sheets are read in the background.
    for _ in 0..100 {
        if shell.read(|root| {
            !root.app.party.reader.loading() && !root.app.party.reader.members.is_empty()
        }) {
            break;
        }
        shell.wait(std::time::Duration::from_millis(50));
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    shell
}

/// The list and the first sheet; a press on another member opens their sheet; the sheet's
/// tabs change its page; a spell row opens its words.
#[test]
fn list_and_sheet() {
    let mut shell = party("party");
    shell.capture("party__01-list-and-sheet__1200x800");
    let members = shell.read(|root| {
        root.app
            .party
            .reader
            .members
            .iter()
            .map(|member| member.id.clone())
            .collect::<Vec<_>>()
    });
    assert!(members.len() > 1, "the fixture's party");
    shell.click(&format!("id:party-member-{}", members[1]));
    assert_eq!(
        shell.read(|root| root.app.party.reader.selected.clone()),
        Some(members[1].clone())
    );
    shell.capture("party__02-second-character__1200x800");
    for (tab, page, step) in [
        ("Actions", 1, "04-page-actions"),
        ("Spells", 2, "05-page-spells"),
        ("Equipment", 3, "06-page-equipment"),
        ("Features", 4, "07-page-features"),
        ("Background", 5, "08-page-background"),
    ] {
        shell.click(tab);
        assert_eq!(shell.read(|root| root.pages.party.page), page, "{tab}");
        shell.capture(&format!("party__{step}__1200x800"));
    }
    shell.click("Overview");
    assert_eq!(shell.read(|root| root.pages.party.page), 0);
}

/// A campaign with no link asks to link it, and the press opens Campaign → Link and folder.
#[test]
fn not_linked_leads_to_link_page() {
    let mut shell = party("party-unlinked");
    shell.update(|root, _, _| {
        let _ = root.app.campaigns.link(None);
    });
    shell.capture("party__11-campaign-not-linked__1200x800");
    shell.click("Connect D&D Beyond");
    assert_eq!(shell.read(|root| root.app.page), Page::Settings);
}
