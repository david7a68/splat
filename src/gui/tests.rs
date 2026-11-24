//! The shell's screenshots and interaction tests (`docs/gpui-port.md`, Kit guide). Captures go to
//! `artifacts/screenshots/gpui/`, named `<scenario>__<step>__<width>x<height>`.
use super::gallery::Gallery;
use super::harness::Shell;
use crate::fixtures::Seed;
use crate::{Page, SettingsTab};
use gpui::{point, px, size};

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
    // The chosen speed moves every scroll area: a touchpad's scroll goes 8× as far; a mouse
    // wheel's lines and a scroll with Ctrl held (a zoom) keep their distance.
    let handle = gpui::ScrollHandle::new();
    let column = handle.clone();
    shell.update(move |root, _, cx| {
        root.pages.reader.preview = Some(std::rc::Rc::new(move |_, _, _| {
            use gpui::{IntoElement, ParentElement, Styled};
            super::kit::target(
                gpui::div()
                    .size_full()
                    .child(super::kit::scroll::scroll_column_with(
                        "touchpad-column",
                        &column,
                        gpui::div().h(px(4000.0)),
                    )),
                "touchpad-column",
            )
            .into_any_element()
        }));
        cx.notify();
    });
    let at = shell
        .bounds("touchpad-column")
        .expect("the column")
        .center();
    let moved = |shell: &mut Shell, delta: gpui::ScrollDelta, modifiers: gpui::Modifiers| {
        let before = handle.offset().y;
        shell.scroll(at, delta, modifiers);
        before - handle.offset().y
    };
    let none = gpui::Modifiers::default();
    let touchpad = gpui::ScrollDelta::Pixels(point(px(0.0), px(-10.0)));
    assert_eq!(
        moved(&mut shell, touchpad, none),
        px(80.0),
        "a touchpad scroll"
    );
    let ctrl = gpui::Modifiers::control();
    assert_eq!(moved(&mut shell, touchpad, ctrl), px(10.0), "a zoom");
    let wheel = gpui::ScrollDelta::Lines(point(0.0, -1.0));
    let fast_wheel = moved(&mut shell, wheel, none);
    shell.update(|root, _, _| root.app.touchpad_speed = crate::touchpad::TouchpadSpeed::One);
    assert_eq!(moved(&mut shell, wheel, none), fast_wheel, "a mouse wheel");
    assert_eq!(moved(&mut shell, touchpad, none), px(10.0), "1×");
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
    // The dialog took the keyboard, and Tab and Shift+Tab keep it inside: past its last action
    // to its first, never to the page under it.
    let dialog = shell
        .update(|_, window, cx| window.focused(cx))
        .expect("the dialog has the keyboard");
    for keys in [
        "tab",
        "tab",
        "tab",
        "tab",
        "tab",
        "shift-tab",
        "shift-tab",
        "shift-tab",
    ] {
        shell.press(keys);
        let (inside, moved) = shell.update(|_, window, cx| {
            (
                dialog.contains_focused(window, cx),
                !dialog.is_focused(window),
            )
        });
        assert!(
            inside && moved,
            "after {keys}, the keyboard is on a control of the dialog"
        );
    }
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

/// The rail, the side panel and a page's columns keep the widths the DM drags them to
/// (`design::Column`): the side panel's natural width is the window's share (`side_panel_width`,
/// 432 px at 1200), a drag stops at the column's range, the widths are saved for the next start
/// under egui's column ids, and Home on a splitter gives the natural width back.
#[test]
fn columns_keep_the_widths_the_dm_drags() {
    let mut shell = Shell::new("gpui-columns", Seed::Campaign);
    let saved = |shell: &mut Shell, key: &str| {
        shell.read(|root| {
            root.remembered
                .as_ref()
                .and_then(|state| state.column_widths.get(key).copied())
        })
    };
    // The rail: dragged wider, then only as far as its range.
    assert_eq!(shell.read(|root| root.rail_width()), px(136.0));
    let edge = shell
        .bounds("Resize navigation column")
        .expect("the rail's splitter");
    let y = edge.center().y;
    shell.drag(edge.center(), point(px(200.0), y));
    assert_eq!(shell.read(|root| root.rail_width()), px(200.0));
    assert_eq!(saved(&mut shell, "navigation"), Some(200.0));
    shell.capture("gpui-columns__01-rail-dragged__1200x800");
    let edge = shell
        .bounds("Resize navigation column")
        .expect("the rail's splitter");
    shell.drag(edge.center(), point(px(600.0), y));
    assert_eq!(shell.read(|root| root.rail_width()), px(320.0));
    // Home on the splitter: the natural width, and nothing saved.
    let edge = shell
        .bounds("Resize navigation column")
        .expect("the rail's splitter");
    shell.click_at(edge.center());
    shell.press("home");
    assert_eq!(shell.read(|root| root.rail_width()), px(136.0));
    assert_eq!(saved(&mut shell, "navigation"), None);
    // Lookup beside the page at the window's share; dragged wider; Left on its splitter.
    shell.press("ctrl-k");
    assert_eq!(shell.read(|root| root.side_width()), Some(px(432.0)));
    shell.capture("gpui-columns__02-lookup__1200x800");
    let edge = shell
        .bounds("Resize Lookup column")
        .expect("Lookup's splitter");
    shell.drag(edge.center(), point(edge.center().x - px(60.0), y));
    assert_eq!(shell.read(|root| root.side_width()), Some(px(492.0)));
    assert_eq!(saved(&mut shell, "lookup"), Some(492.0));
    let edge = shell
        .bounds("Resize Lookup column")
        .expect("Lookup's splitter");
    shell.click_at(edge.center());
    shell.press("left");
    assert_eq!(shell.read(|root| root.side_width()), Some(px(508.0)));
    shell.press("escape");
    assert_eq!(shell.read(|root| root.side_width()), None);
    // A page's splitter (the Library's entry list) keeps its width under `library_entries`.
    shell.click("Library");
    for _ in 0..400 {
        if shell.bounds("column library_entries").is_some() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
        shell.wait(std::time::Duration::from_millis(50));
    }
    let list = shell.bounds("column library_entries").unwrap_or_else(|| {
        panic!(
            "the Library's entry column: {:?}",
            super::kit::probe::names()
        )
    });
    let splitter = point(list.right(), list.center().y);
    shell.drag(splitter, point(splitter.x + px(50.0), splitter.y));
    let width = saved(&mut shell, "library_entries").expect("the list's width is kept");
    let wanted = f32::from(list.size.width) + 50.0;
    assert!(
        (width - wanted).abs() <= 2.0,
        "the list is {width} px wide, not {wanted}"
    );
}

/// A menu keeps inside the pane that holds its trigger (egui `popups`): near the pane's foot it
/// opens above its trigger, and in a narrow pane it is no wider than the pane's body, clear of
/// the scroll bar's lane, with its words wrapped.
#[test]
fn a_menu_keeps_inside_its_pane() {
    use super::kit::{Button, MenuButton, MenuItem, Pane};
    use gpui::{IntoElement, ParentElement, Styled};
    let mut shell = Shell::new("gpui-menu-room", Seed::Campaign);
    shell.update(|root, _, cx| {
        root.pages.reader.preview = Some(std::rc::Rc::new(|_, _, _| {
            let menu = MenuButton::new("room-menu", Button::secondary("room-more", "More"))
                .item(MenuItem::new("Rename"))
                .item(MenuItem::new("Move this creature to another encounter"))
                .item(MenuItem::new("Remove"));
            gpui::div()
                .flex()
                .flex_row()
                .size_full()
                .child(gpui::div().flex_1())
                .child(
                    super::kit::target(gpui::div().w(px(220.0)).h_full(), "narrow-pane").child(
                        Pane::new("room-pane")
                            .child(gpui::div().h(px(690.0)))
                            .child(menu),
                    ),
                )
                .into_any_element()
        }));
        cx.notify();
    });
    shell.click("More");
    let trigger = shell.bounds("More").expect("the trigger");
    let pane = shell.bounds("narrow-pane").expect("the pane");
    shell.capture("gpui-menu-room__01-open__1200x800");
    let right = pane.right() - super::theme::M - super::theme::SCROLL_BAR_WIDTH;
    for item in [
        "Rename",
        "Move this creature to another encounter",
        "Remove",
    ] {
        let bounds = shell
            .bounds(item)
            .unwrap_or_else(|| panic!("{item} is shown"));
        assert!(
            bounds.bottom() <= trigger.top(),
            "{item} opens above its trigger"
        );
        assert!(bounds.right() <= right, "{item} keeps inside the pane");
    }
}

/// Ctrl+S has one owner (`SplatRoot::save`): with drafts open on two pages, it saves only the
/// draft of the page shown, wherever the keyboard is (a rail item the DM just pressed, the
/// Lookup field), and nothing while a menu is open over the page.
#[test]
fn ctrl_s_saves_only_the_page_shown() {
    use std::time::Duration;
    let mut shell = Shell::new("gpui-ctrl-s", Seed::Campaign);
    let wait = |shell: &mut Shell, done: fn(&super::SplatRoot) -> bool| {
        for _ in 0..400 {
            if shell.read(done) {
                break;
            }
            std::thread::sleep(Duration::from_millis(5));
            shell.wait(Duration::from_millis(50));
        }
        assert!(shell.read(done), "the page is read");
    };
    let library_dirty = |shell: &mut Shell| shell.read(|root| root.app.library.dirty());
    let encounter_dirty =
        |shell: &mut Shell| shell.read(|root| root.app.encounters.encounter_dirty());
    // A Library draft: a local entry's Markdown, typed into.
    shell.click("Library");
    wait(&mut shell, |root| {
        root.app.library.loaded() && !root.app.library.loading()
    });
    shell.click("id:library-scope");
    shell.click("Ashen Marches");
    shell.click("Ash Exposure · Rule");
    shell.click("Edit Markdown");
    shell.click("Markdown editor");
    shell.press("ctrl-end");
    shell.type_text(" More ash.");
    assert!(library_dirty(&mut shell));
    // An Encounters draft: a saved encounter's roster, reordered.
    shell.click("Encounters");
    wait(&mut shell, |root| {
        !root.app.encounters.busy() && root.app.encounters.loaded()
    });
    let first = |prefix: &str| {
        crate::gui::kit::probe::names()
            .into_iter()
            .find(|name| name.starts_with(prefix))
            .unwrap_or_else(|| panic!("no target starts with {prefix:?}"))
    };
    shell.click(&first("id:edit:"));
    shell.click(&first("id:roster-menu:"));
    shell.click("Move down");
    assert!(encounter_dirty(&mut shell));
    // A menu over the page: Ctrl+S waits.
    shell.click(&first("id:roster-menu:"));
    shell.press("ctrl-s");
    assert!(encounter_dirty(&mut shell), "no save under an open menu");
    shell.press("escape");
    // The keyboard in Lookup's field: Ctrl+S is still the page's.
    shell.press("ctrl-k");
    shell.press("ctrl-s");
    assert!(!encounter_dirty(&mut shell), "Ctrl+S saved the encounter");
    assert!(library_dirty(&mut shell), "and not the Library's draft");
    shell.press("escape");
    // The keyboard on the rail item just pressed: Ctrl+S saves the Library's draft.
    shell.click("Library");
    shell.press("ctrl-s");
    wait(&mut shell, |root| !root.app.library.loading());
    assert!(
        !library_dirty(&mut shell),
        "Ctrl+S saved the Library's draft"
    );
}

/// Report only (run by hand with `--ignored --nocapture`): what a quiet frame costs on the
/// heaviest pages, and whether it grows with the content. World with a few notes and with 2,000
/// more; the Library browsing the SRD.
#[test]
#[ignore = "a timing report, not a check: run by hand"]
fn frame_cost_report() {
    use std::time::Duration;
    let mut shell = Shell::new("gpui-frame-cost", Seed::Campaign);
    let quiet = shell.frame_cost(20);
    shell.click("World");
    let world_few = shell.frame_cost(20);
    let notes = shell.read(|root| {
        root.app.lookup.index().map_or(0, |i| {
            i.world_groups("").values().map(Vec::len).sum::<usize>()
        })
    });
    let folder = shell.read(|root| root.app.campaigns.active_campaign().unwrap().path);
    for at in 0..2000 {
        let path = format!("World/Many/Note {at}.md");
        let id = crate::character::stable_id(&path);
        let file = folder.join(&path);
        std::fs::create_dir_all(file.parent().unwrap()).unwrap();
        std::fs::write(
            file,
            crate::content::formats::fixture(
                "World.md",
                &format!(
                    "splat_id: {id}\nsplat_kind: world\nsplat_name: Note {at}\nsplat_world:\n  type: npc\n"
                ),
                &format!("\n# Note {at}\n\nA note for tonight, with words to search through.\n"),
            )
            .unwrap(),
        )
        .unwrap();
    }
    let before = shell.read(|root| root.app.lookup.index().map(|index| index.generation()));
    shell.update(|root, _, _| root.app.lookup.refresh());
    for _ in 0..3000 {
        std::thread::sleep(Duration::from_millis(20));
        shell.wait(Duration::from_millis(50));
        let ready = shell.read(|root| {
            root.app
                .lookup
                .index()
                .is_some_and(|index| Some(index.generation()) != before)
        });
        if ready {
            break;
        }
    }
    let many = shell.read(|root| {
        root.app.lookup.index().map_or(0, |i| {
            i.world_groups("").values().map(Vec::len).sum::<usize>()
        })
    });
    let world_many = shell.frame_cost(20);
    shell.click("Library");
    for _ in 0..400 {
        if shell.read(|root| root.app.library.loaded() && !root.app.library.loading()) {
            break;
        }
        std::thread::sleep(Duration::from_millis(20));
        shell.wait(Duration::from_millis(50));
    }
    let library = shell.frame_cost(20);
    println!(
        "frame cost: Encounters list {quiet:?}; World with {notes} notes {world_few:?}, with {many} notes {world_many:?}; Library browse {library:?}"
    );
}
