//! Party: screenshots of the egui `party`, `party-bar-crowded` and `empty` scenarios' main
//! states, and the page's flows (choose a member, change the sheet's page, open an entry).
use crate::Page;
use crate::fixtures::Seed;
use crate::gui::harness::Shell;

fn party(name: &str) -> Shell {
    let mut shell = Shell::new(name, Seed::Campaign);
    shell.click("Party");
    assert_eq!(shell.read(|root| root.app.page), Page::Party);
    // The party's sheets are read in the background.
    for _ in 0..400 {
        if shell.read(|root| !root.app.party.reader.members.is_empty() && sheet_ready(root)) {
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

/// The selected member's sheet is read, and a signed-in Party page's sync has started.
fn sheet_ready(root: &crate::gui::SplatRoot) -> bool {
    let party = &root.app.party;
    !party.reader.loading()
        && party
            .reader
            .selected
            .as_ref()
            .is_some_and(|id| party.reader.sheets.contains_key(id))
        && (root.app.settings.campaign_session().1.is_none() || party.combat.enabled)
}

/// The Party page of a signed-in DM, with the first member's sheet and its combat sync open.
fn signed_in(name: &str, window: gpui::Size<gpui::Pixels>) -> Shell {
    let mut shell = Shell::with_size(name, Seed::Campaign, window);
    shell.update(|root, _, _| {
        root.app
            .settings
            .simulate(crate::settings::Account::Connected);
    });
    shell.click("Party");
    for _ in 0..400 {
        if shell.read(sheet_ready) {
            break;
        }
        shell.wait(std::time::Duration::from_millis(50));
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    shell
}

/// A read of the selected character's sheet, with every group the sync panel can edit (the
/// egui `signed-in` scenario's sheet).
fn read_sheet(shell: &mut Shell) {
    shell.update(|root, _, _| {
        let id: u64 = root
            .app
            .party
            .reader
            .selected
            .as_deref()
            .and_then(|id| id.parse().ok())
            .expect("a selected character");
        let sheet = serde_json::json!({
            "id": id, "canEdit": true, "removedHitPoints": 4, "temporaryHitPoints": 5,
            "inspiration": true, "bonusHitPoints": 0,
            "deathSaves": {"successCount": 0, "failCount": 0}, "conditions": [],
            "currencies": {"cp": 12, "sp": 30, "ep": 0, "gp": 145, "pp": 2},
            "actions": {"class": [{"id": 1, "entityTypeId": 2, "name": "Second Wind", "limitedUse": {"numberUsed": 1}}]},
            "inventory": [
                {"id": 3, "quantity": 18, "definition": {"name": "Crossbow Bolts"}},
                {"id": 4, "quantity": 1, "limitedUse": {"maxUses": 3, "numberUsed": 1}, "definition": {"name": "Wand of Web"}}
            ],
            "creatures": [{"id": 5, "name": "Grit", "removedHitPoints": 2, "temporaryHitPoints": 0, "definition": {"name": "Mastiff", "averageHitPoints": 5}}],
            "classes": [{"id": 6, "level": 6, "hitDiceUsed": 2, "definition": {"name": "Fighter", "hitDice": 10}}]
        });
        root.app.party.simulate_sheet_read(62, &sheet);
    });
}

fn values(shell: &mut Shell, key: &str, field: &str) -> serde_json::Value {
    shell.read(|root| root.app.party.combat.values()[key][field].clone())
}

/// The sync editor on the sheet: before the first read it says what it waits for; after one,
/// every value has − and +, and each press changes that value in the sync (egui `signed-in`).
#[test]
fn party_sync_editor() {
    for (w, h) in [(1200.0, 800.0), (960.0, 640.0)] {
        let size = format!("{w}x{h}");
        let mut shell = signed_in("signed-in", gpui::size(gpui::px(w), gpui::px(h)));
        assert!(shell.read(|root| root.app.party.combat.enabled));
        shell.click("Combat sync · Waiting for the first sheet read");
        assert!(shell.bounds("Sync this sheet with D&D Beyond").is_some());
        assert!(shell.bounds("id:party-sync-settings").is_some());
        shell.capture(&format!("signed-in__05-party-sync-on__{size}"));
        read_sheet(&mut shell);
        assert!(shell.bounds("Combat sync · On").is_some());
        shell.capture(&format!("signed-in__07-party-sync-editable__{size}"));
        shell.click("Short rest…");
        if w > 1000.0 {
            assert!(shell.bounds("Take short rest").is_some());
        }
        shell.capture(&format!("signed-in__06-party-short-rest__{size}"));
    }
}

/// Presses through the editor change the sync's values: HP, inspiration, a condition, a coin,
/// an item's charges, and a short rest that is set up and cancelled.
#[test]
fn party_sync_editor_edits() {
    let mut shell = signed_in(
        "signed-in-edits",
        gpui::size(gpui::px(1200.0), gpui::px(2400.0)),
    );
    shell.click("Combat sync · Waiting for the first sheet read");
    read_sheet(&mut shell);
    // A short rest is set up in the panel, then cancelled: nothing goes.
    shell.click("Short rest…");
    shell.click("Increase Fighter (d10)");
    assert!(matches!(
        shell.read(|root| root.app.party.combat.rest()),
        splat_party::combat_sync::view::Rest::Short { ref dice, .. } if dice[0].1.value == 1
    ));
    shell.click("Cancel");
    assert!(matches!(
        shell.read(|root| root.app.party.combat.rest()),
        splat_party::combat_sync::view::Rest::Choose { blocker: None, .. }
    ));
    assert_eq!(values(&mut shell, "hp", "removedHitPoints"), 4);
    shell.click("Decrease Current HP");
    assert_eq!(values(&mut shell, "hp", "removedHitPoints"), 5);
    assert!(shell.read(|root| root.app.party.combat.dirty()));
    shell.click("Increase Temporary HP");
    assert_eq!(values(&mut shell, "hp", "temporaryHitPoints"), 6);
    shell.click("Has inspiration");
    assert_eq!(values(&mut shell, "inspiration", "inspiration"), false);
    shell.click("Sheet conditions");
    shell.click("Poisoned");
    assert!(shell.read(|root| {
        root.app.party.combat.values()["conditions"]
            .get("11")
            .is_some()
    }));
    shell.click("Currency");
    shell.click("Decrease Gold");
    assert_eq!(values(&mut shell, "currency", "gp"), 144);
    shell.click("Inventory · 2 items");
    shell.click("Increase Wand of Web charges spent");
    assert_eq!(
        shell.read(|root| root.app.party.combat.values()["item/4"]["charges"].clone()),
        2
    );
    // Every − and + of a block share an x, whatever the number (Gold 144 beside 2).
    let lefts: Vec<_> = ["Platinum", "Gold", "Copper"]
        .iter()
        .map(|coin| shell.bounds(&format!("Increase {coin}")).map(|b| b.left()))
        .collect();
    assert!(
        lefts.iter().all(|x| x.is_some() && *x == lefts[0]),
        "{lefts:?}"
    );
    // A rest waits until the changes reach D&D Beyond.
    shell.click("Short rest…");
    assert!(matches!(
        shell.read(|root| root.app.party.combat.rest()),
        splat_party::combat_sync::view::Rest::Choose {
            blocker: Some("Wait until your changes reach D&D Beyond."),
            ..
        }
    ));
    // Turning sync off stops it, and it stays off.
    shell.click("Sync this sheet with D&D Beyond");
    assert!(!shell.read(|root| root.app.party.combat.enabled));
    shell.wait(std::time::Duration::from_millis(100));
    assert!(!shell.read(|root| root.app.party.combat.enabled));
}

/// The sheet's numbers roll: an ability's modifier, a save and a skill ask for their roll,
/// which the frame logic makes a preview away from the run.
#[test]
fn sheet_numbers_roll() {
    let mut shell = party("party-rolls");
    let name = shell.read(|root| {
        let id = root
            .app
            .party
            .reader
            .selected
            .clone()
            .expect("a selected member");
        root.app.party.reader.sheets[&id]
            .definition
            .splat_name
            .clone()
    });
    let check = shell.read(|root| {
        let id = root.app.party.reader.selected.clone().unwrap();
        root.app.party.reader.sheets[&id].stat("strength_modifier")
    });
    let check = check.expect("a strength modifier");
    shell.click(&format!("Strength {}", crate::design::signed(check)));
    let rolled = shell.read(|root| {
        crate::rolls::plain::preview_words(
            root.bridge.ctx(),
            &name,
            &[&splat::content::dice_text::RollLink::d20(
                splat::content::dice_text::D20::Check,
                check,
                "Strength",
            )],
        )
    });
    assert!(rolled.is_some(), "the press made a preview roll for {name}");
}

/// Technical details: the character check, the optional rules and the campaign content counts.
#[test]
fn technical_details() {
    // A tall window, so the section at the sheet's end is in view without a scroll.
    let mut shell = Shell::with_size(
        "party-technical",
        Seed::Campaign,
        gpui::size(gpui::px(1200.0), gpui::px(2000.0)),
    );
    shell.click("Party");
    for _ in 0..400 {
        if shell.read(sheet_ready) {
            break;
        }
        shell.wait(std::time::Duration::from_millis(50));
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    shell.click("Technical details");
    assert!(shell.read(|root| root.pages.party.checked.is_some()));
    assert!(
        shell.bounds("Character check").is_some(),
        "the character check"
    );
    shell.click("Character check");
    shell.capture("party__09-source-and-parity__1200x2000");
}

/// The party bar on the Session page during the fixture's fight: every member whole, fitted by
/// measured widths; while the fight is open, an item keeps its width when its HP gets shorter,
/// so nothing after it moves back (egui `party-bar-crowded`, `party_bar` tests).
#[test]
fn party_bar_holds_through_a_fight() {
    let mut shell = Shell::new("party-bar-crowded", Seed::Campaign);
    shell.update(|root, _, cx| {
        let campaign = root.app.campaigns.active_campaign().expect("a campaign");
        let modules = root.app.campaigns.modules().to_vec();
        root.app.encounters.refresh_now(&campaign, &modules);
        let path = root
            .app
            .encounters
            .saved_runs()
            .iter()
            .find(|run| run.encounter == crate::fixtures::RUN)
            .expect("the fixture's saved run")
            .path
            .clone();
        root.app.encounters.open_saved_run(&path, &campaign);
        root.app.sessions.party_bar = Some(true);
        // Kestrel is already Bloodied. Two more make three of five; two keep temporary HP.
        let run = root.app.encounters.run_mut().expect("the run");
        for (name, damage) in [("Iselle Marrowind", 8), ("Oren Thistlewick", 23)] {
            let at = run
                .state
                .participants
                .iter()
                .position(|p| p.name == name)
                .expect("a party member");
            let p = &run.state.participants[at];
            let (hp, max) = (p.hp.map(|hp| hp - damage), p.max_hp);
            run.run_command(crate::encounter_run::Command::Health(at, hp, max));
        }
        root.show_page(Page::Run, cx);
    });
    shell.click("Session");
    assert_eq!(shell.read(|root| root.app.page), Page::Sessions);
    for _ in 0..400 {
        if shell.read(|root| root.pages.party.bar_hold.is_some()) {
            break;
        }
        shell.wait(std::time::Duration::from_millis(50));
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    shell.capture("party-bar-crowded__01-three-bloodied__1200x800");
    let widths = |shell: &mut Shell| {
        shell.read(|root| {
            let hold = root.pages.party.bar_hold.as_ref().expect("a fight's hold");
            format!("{hold:?}")
        })
    };
    let before = widths(&mut shell);
    // Brannoch Vale's temporary HP goes: his item has less to show.
    shell.update(|root, _, _| {
        let run = root.app.encounters.run_mut().expect("the run");
        let at = run
            .state
            .participants
            .iter()
            .position(|p| p.name == "Brannoch Vale")
            .expect("Brannoch Vale");
        run.run_command(crate::encounter_run::Command::TempHp(at, 0));
    });
    assert_eq!(
        shell.read(|root| {
            let run = root.app.encounters.run().unwrap();
            run.state
                .participants
                .iter()
                .find(|p| p.name == "Brannoch Vale")
                .map(|p| p.temporary_hp)
        }),
        Some(0)
    );
    let after = widths(&mut shell);
    assert_eq!(
        before, after,
        "the bar holds its level and widths in the fight"
    );
}
