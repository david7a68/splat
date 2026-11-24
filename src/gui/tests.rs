//! The shell's screenshots and interaction tests (`docs/gpui-port.md`, Kit guide). Captures go to
//! `artifacts/screenshots/gpui/`, named `<scenario>__<step>__<width>x<height>`.
use super::gallery::Gallery;
use super::harness::Shell;
use crate::visual_tests::fixture::Seed;
use crate::{Page, SettingsTab};
use gpui::{px, size};

/// The shell over a campaign at the default size and at the smallest window.
#[test]
fn the_shell_at_its_sizes() {
    let mut shell = Shell::new("gpui-shell", Seed::Campaign);
    assert_eq!(shell.read(|root| root.app.page), Page::Encounters);
    assert_eq!(shell.size(), size(px(1200.0), px(800.0)));
    shell.capture("gpui-shell__01-encounters__1200x800");
    let mut shell = Shell::with_size("gpui-shell", Seed::Campaign, size(px(960.0), px(640.0)));
    assert_eq!(shell.size(), size(px(960.0), px(640.0)));
    shell.capture("gpui-shell__01-encounters__960x640");
}

/// With no campaign, a page that needs one says so and offers Add campaign, which opens
/// Settings on its account tab.
#[test]
fn no_campaign_leads_to_settings() {
    let mut shell = Shell::new("gpui-empty", Seed::Empty);
    shell.update(|root, _, cx| root.show_page(Page::Encounters, cx));
    shell.capture("gpui-empty__01-encounters-no-campaign__1200x800");
    shell.click("Add campaign");
    assert_eq!(shell.read(|root| root.app.page), Page::Settings);
    assert!(shell.read(|root| root.app.settings_tab == SettingsTab::Account));
}

/// The rail switches pages, and the Settings tabs switch tabs.
#[test]
fn the_rail_switches_pages() {
    let mut shell = Shell::new("gpui-rail", Seed::Campaign);
    for (word, page) in [
        ("Library", Page::Library),
        ("Session", Page::Sessions),
        ("Timeline", Page::Timeline),
        ("World", Page::World),
        ("Party", Page::Party),
        ("Campaign", Page::Campaign),
        ("Settings", Page::Settings),
        ("Encounters", Page::Encounters),
    ] {
        shell.click(word);
        assert_eq!(shell.read(|root| root.app.page), page, "after {word}");
    }
    shell.click("Settings");
    shell.click("Touchpad");
    assert!(shell.read(|root| root.app.settings_tab == SettingsTab::Touchpad));
    shell.click("8×");
    assert_eq!(
        shell.read(|root| root.app.touchpad_speed),
        crate::touchpad::TouchpadSpeed::Eight
    );
    shell.capture("gpui-rail__01-settings-touchpad__1200x800");
}

/// Ctrl+K opens Lookup beside the page, and Escape closes it; Ctrl+J opens the notes.
#[test]
fn the_global_keys_open_and_close_panels() {
    let mut shell = Shell::new("gpui-keys", Seed::Campaign);
    shell.press("ctrl-k");
    assert!(shell.read(|root| root.app.lookup.open));
    shell.press("escape");
    assert!(!shell.read(|root| root.app.lookup.open));
    shell.press("ctrl-j");
    assert!(shell.read(|root| root.app.sessions.notes_open));
    // Lookup takes the notes' place, and they come back when it closes.
    shell.press("ctrl-k");
    assert!(shell.read(|root| root.app.lookup.open && !root.app.sessions.notes_open));
    shell.press("escape");
    assert!(shell.read(|root| !root.app.lookup.open && root.app.sessions.notes_open));
}

/// The campaign switcher's menu opens from the rail and changes the campaign.
#[test]
fn the_switcher_changes_the_campaign() {
    let mut shell = Shell::new("gpui-switcher", Seed::Campaign);
    let before = shell.read(|root| root.app.campaigns.active_campaign().map(|c| c.name));
    assert_eq!(before.as_deref(), Some("The Emberfall Marches"));
    shell.click("id:active-campaign");
    shell.capture("gpui-switcher__01-menu__1200x800");
    shell.click("Quiet Harbor");
    let after = shell.read(|root| root.app.campaigns.active_campaign().map(|c| c.name));
    assert_eq!(after.as_deref(), Some("Quiet Harbor"));
}

/// Every kit part, and the parts that keep state working: a text field takes typed words into the
/// caller's `String`, a drop-down chooses, a menu item presses and closes its menu, a dialog opens
/// and Escape keeps editing.
#[test]
fn the_kit_parts_work() {
    let mut shell = Shell::new("gpui-kit", Seed::Campaign);
    shell.update(|root, _, cx| {
        root.gallery = Some(Gallery::default());
        cx.notify();
    });
    shell.capture("gpui-kit__01-gallery__1200x800");
    let gallery = |shell: &mut Shell, read: fn(&Gallery) -> String| {
        shell.read(|root| read(root.gallery.as_ref().expect("the gallery")))
    };
    shell.click("Campaign name");
    shell.type_text("Ashen");
    assert_eq!(gallery(&mut shell, |g| g.name.clone()), "Ashen");
    // The program's change replaces the field's words.
    shell.update(|root, _, cx| {
        root.gallery.as_mut().expect("the gallery").name = "Reset".into();
        cx.notify();
    });
    shell.click("Campaign name");
    shell.type_text("!");
    assert!(gallery(&mut shell, |g| g.name.clone()).contains("Reset"));
    shell.click("id:g-kind");
    shell.click("Spell");
    assert_eq!(gallery(&mut shell, |g| format!("{:?}", g.kind)), "Some(1)");
    shell.click("Packages");
    shell.capture("gpui-kit__02-menu__1200x800");
    shell.click("Rename");
    assert_eq!(gallery(&mut shell, |g| g.pressed.join(",")), "rename");
    assert!(shell.bounds("Rename").is_none(), "the menu closed");
    shell.click("Remember my sign-in");
    assert_eq!(gallery(&mut shell, |g| g.remember.to_string()), "true");
    shell.click("Rules and runs");
    assert_eq!(gallery(&mut shell, |g| g.tab.to_string()), "1");
    shell.click("Open dialog");
    shell.capture("gpui-kit__03-dialog__1200x800");
    // The dialog covers the page: a press on the page's primary does nothing.
    shell.click("Start fight");
    assert_eq!(gallery(&mut shell, |g| g.pressed.join(",")), "rename");
    shell.press("escape");
    assert_eq!(gallery(&mut shell, |g| g.dialog.to_string()), "false");
    shell.click("Start fight");
    assert_eq!(
        gallery(&mut shell, |g| g.pressed.join(",")),
        "rename,primary"
    );
    shell.wait(std::time::Duration::from_millis(200));
}
