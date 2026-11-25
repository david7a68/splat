//! The shell's screenshots and interaction tests (`docs/development.md`, GUI tests). Captures go to
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
    super::pages::encounters::tests::ready(&mut shell);
    shell.capture("gpui-shell__01-encounters__1200x800");
    let mut shell = Shell::with_size("gpui-shell", Seed::Campaign, size(px(960.0), px(640.0)));
    assert_eq!(shell.size(), size(px(960.0), px(640.0)));
    super::pages::encounters::tests::ready(&mut shell);
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
    // The keyboard in the notes panel's editor, then the panel closed by its own key: the
    // editor's state lives on, but it left the window, so the keyboard comes back to the root
    // and the global keys still work. (A session first, so the panel has a note.)
    shell.press("ctrl-j");
    shell.click("Session");
    shell.click("id:session-empty-new");
    shell.click("Session title");
    shell.type_text("Ash night");
    shell.press("enter");
    wait_for(&mut shell, "the session is made", |root| {
        !root.app.sessions.gv_form_open() && root.app.sessions.gv_selected().is_some()
    });
    shell.click("Encounters");
    shell.press("ctrl-j");
    assert!(shell.read(|root| root.app.sessions.notes_open));
    shell.click("Note text");
    shell.type_text("Ash");
    // The note's `/` menu takes its own Escape: the menu closes and the panel stays.
    shell.type_text(" /");
    assert!(shell.bounds("Search").is_some(), "the / menu is open");
    shell.press("escape");
    assert!(shell.bounds("Search").is_none(), "Escape closed the / menu");
    assert!(
        shell.read(|root| root.app.sessions.notes_open),
        "and not the notes panel"
    );
    shell.press("ctrl-j");
    assert!(!shell.read(|root| root.app.sessions.notes_open));
    shell.press("ctrl-k");
    assert!(
        shell.read(|root| root.app.lookup.open),
        "Ctrl+K works after the focused panel closed"
    );
    shell.press("escape");
    assert!(!shell.read(|root| root.app.lookup.open));
}

/// Escape: the root's order (an open menu, then Lookup) runs before a page's own Escape binding.
/// The session form binds Escape to Cancel; a drop-down open in it, or Lookup open beside it,
/// takes the Escape first and the DM's typed words stay.
#[test]
fn escape_takes_the_menu_and_lookup_before_the_page() {
    let mut shell = Shell::new("gpui-escape", Seed::Campaign);
    shell.click("Session");
    shell.click("id:session-empty-new");
    let form_open = |shell: &mut Shell| shell.read(|root| root.app.sessions.gv_form_open());
    assert!(form_open(&mut shell), "New session opens the form");
    shell.click("Session title");
    shell.type_text("Bridge night");
    shell.click("id:session-template");
    let menu_open = |shell: &mut Shell| shell.update(|_, _, cx| super::kit::menu::any_open(cx));
    assert!(menu_open(&mut shell), "the template list is open");
    shell.press("escape");
    assert!(!menu_open(&mut shell), "Escape closed the list");
    assert!(form_open(&mut shell), "and not the form");
    shell.press("ctrl-k");
    assert!(shell.read(|root| root.app.lookup.open));
    shell.click("Session title");
    shell.press("escape");
    assert!(
        !shell.read(|root| root.app.lookup.open),
        "Escape closed Lookup"
    );
    assert!(form_open(&mut shell), "and not the form");
    // Nothing of the root's is open: the form's own Escape cancels it.
    shell.click("Session title");
    shell.press("escape");
    assert!(!form_open(&mut shell), "Escape cancels the form");
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
    // A message's action stands `L` after its words when it fits on their line (egui's FK3).
    let words = shell
        .bounds("Saved “Bridge Ambush”.")
        .expect("the message's words");
    let undo = shell.bounds("id:g-undo").expect("its Undo");
    assert!(
        undo.top() < words.bottom()
            && (undo.left() - words.right() - super::theme::L).abs() <= px(1.0),
        "the Undo {undo:?} is L after the words {words:?}"
    );
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
    // A press on another menu's trigger while one is open closes the first and opens the second
    // with the same press.
    shell.click("Packages");
    assert!(shell.bounds("Rename").is_some(), "the menu is open");
    shell.click("id:g-kind");
    assert!(shell.bounds("Rename").is_none(), "the menu closed");
    assert!(
        shell.bounds("Spell").is_some(),
        "the drop-down opened with the same press"
    );
    shell.press("escape");
    // A pane that overflows shows its scroll bar all the time, as egui's did.
    assert_eq!(
        shell.update(|_, _, cx| gpui_component::Theme::global(cx).scrollbar_mode),
        gpui_component::scroll::ScrollbarMode::Always
    );
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
    // The safe action has the keyboard when the dialog opens: Enter keeps editing.
    shell.press("enter");
    assert_eq!(
        gallery(&mut shell, |g| g.dialog.to_string()),
        "false",
        "Enter on the opened dialog pressed Keep editing"
    );
    shell.click("Open dialog");
    // Tab and Shift+Tab keep the keyboard inside: past its last action to its first, never to
    // the page under it. Its three actions come round again after three of either.
    let focused = |shell: &mut Shell| {
        shell
            .update(|_, window, cx| window.focused(cx))
            .expect("the dialog has the keyboard")
    };
    let start = focused(&mut shell);
    for keys in ["tab", "shift-tab"] {
        let mut seen = vec![start.clone()];
        for _ in 0..3 {
            shell.press(keys);
            seen.push(focused(&mut shell));
        }
        assert!(
            seen[1] != seen[0] && seen[2] != seen[1] && seen[3] == seen[0],
            "{keys} goes round the dialog's three actions"
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
/// under their column ids, and Home on a splitter gives the natural width back.
#[test]
fn columns_keep_the_widths_the_dm_drags() {
    let mut shell = Shell::new("gpui-columns", Seed::Campaign);
    super::pages::encounters::tests::ready(&mut shell);
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
    // While the drag goes on the rail follows it, and nothing is written: the width is saved
    // once, when the drag ends.
    shell.hold_and_move(edge.center(), point(px(200.0), y));
    assert_eq!(shell.read(|root| root.rail_width()), px(200.0));
    assert_eq!(
        saved(&mut shell, "navigation"),
        None,
        "no write during a drag"
    );
    shell.release(point(px(200.0), y));
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

/// A menu keeps inside the pane that holds its trigger: near the pane's foot it
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
    // The campaign cannot change with a draft open: the switcher shows its lock.
    assert!(
        shell
            .bounds("Active campaign: The Emberfall Marches (locked)")
            .is_some(),
        "the switcher is locked: {:?}",
        crate::gui::kit::probe::names()
    );
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

/// A menu whose trigger is no longer drawn (its row went in a reload, its form closed) closes:
/// no invisible menu keeps the global keys refused. A drop-down asked to take the keyboard has
/// it: Enter opens its list.
#[test]
fn a_menu_closes_when_its_trigger_goes() {
    use super::kit::{Button, Dropdown, MenuButton, MenuItem};
    use gpui::{IntoElement, ParentElement};
    use std::cell::Cell;
    use std::rc::Rc;
    let mut shell = Shell::new("gpui-menu-gone", Seed::Campaign);
    let shown = Rc::new(Cell::new(true));
    let focus = Rc::new(Cell::new(false));
    let (draws, focuses) = (shown.clone(), focus.clone());
    shell.update(move |root, _, cx| {
        let (draws, focuses) = (draws.clone(), focuses.clone());
        root.pages.reader.preview = Some(Rc::new(move |_, _, _| {
            let mut page = gpui::div().child(
                Dropdown::new("focus-kind", "Kind", ["Creature", "Spell"], None)
                    .focus(focuses.take()),
            );
            if draws.get() {
                page = page.child(
                    MenuButton::new("gone-menu", Button::secondary("gone-more", "More"))
                        .item(MenuItem::new("Rename")),
                );
            }
            page.into_any_element()
        }));
        cx.notify();
    });
    shell.click("More");
    assert!(shell.bounds("Rename").is_some(), "the menu is open");
    shown.set(false);
    shell.update(|_, _, cx| cx.notify());
    assert!(
        !shell.update(|_, _, cx| super::kit::menu::any_open(cx)),
        "the menu closed with its trigger"
    );
    shell.press("ctrl-k");
    assert!(shell.read(|root| root.app.lookup.open), "Ctrl+K works");
    shell.press("escape");
    focus.set(true);
    shell.update(|_, _, cx| cx.notify());
    shell.press("enter");
    assert!(
        shell.bounds("Spell").is_some(),
        "the focused drop-down opened on Enter"
    );
}

/// A text editor's own edit (a command's result, words put in at the caret) keeps the DM's Undo
/// history under it; words from elsewhere end it.
#[test]
fn a_program_write_keeps_the_text_undo_when_it_is_the_fields_own() {
    use super::kit::{EditorHandle, TextEditor};
    use gpui::IntoElement;
    use std::cell::RefCell;
    use std::rc::Rc;
    let mut shell = Shell::new("gpui-text-undo", Seed::Campaign);
    let kept: Rc<RefCell<Option<EditorHandle>>> = Rc::default();
    let words: Rc<RefCell<String>> = Rc::default();
    let (draw_kept, draw_words) = (kept.clone(), words.clone());
    shell.update(move |root, _, cx| {
        let (kept, words) = (draw_kept.clone(), draw_words.clone());
        root.pages.reader.preview = Some(Rc::new(move |_, window, cx| {
            let handle = kept
                .borrow_mut()
                .get_or_insert_with(|| EditorHandle::new("", window, cx))
                .clone();
            let typed = words.clone();
            TextEditor::new("undo-editor", words.borrow().clone())
                .handle(&handle)
                .name("Undo editor")
                .on_change(move |now: &str, _, _| *typed.borrow_mut() = now.to_owned())
                .into_any_element()
        }));
        cx.notify();
    });
    let handle = || kept.borrow().clone().expect("the editor's handle");
    let shown = |shell: &mut Shell, handle: &EditorHandle| {
        let handle = handle.clone();
        shell.update(move |_, _, cx| handle.words(cx).to_string())
    };
    shell.click("Undo editor");
    shell.type_text("Ash");
    assert_eq!(*words.borrow(), "Ash");
    // An editor with no type of its own draws in `Body`, as egui's did, not in gpui-component's
    // smaller size.
    let line = {
        let handle = handle();
        shell.update(move |_, _, cx| handle.caret_bounds(cx).map(|caret| caret.size.height))
    };
    assert_eq!(line, Some(super::theme::Type::Body.line()), "a Body line");
    // The field's own edit (the caller's words change with it): the words change, and Ctrl+Z
    // takes the change back.
    let editor = handle();
    let (own, model) = (editor.clone(), words.clone());
    shell.update(move |_, window, cx| {
        *model.borrow_mut() = "Ash falls".to_owned();
        own.show_words("Ash falls", window, cx);
    });
    assert_eq!(shown(&mut shell, &editor), "Ash falls");
    shell.press("ctrl-z");
    assert_eq!(
        shown(&mut shell, &editor),
        "Ash",
        "Ctrl+Z took back the field's own edit"
    );
    // Words from elsewhere end the history: Ctrl+Z keeps them.
    let (other, model) = (editor.clone(), words.clone());
    shell.update(move |_, window, cx| {
        *model.borrow_mut() = "From disk".to_owned();
        other.show_other_words("From disk", window, cx);
    });
    shell.press("ctrl-z");
    assert_eq!(shown(&mut shell, &editor), "From disk");
}

/// A wrapped link in a row gives way to the row's other parts: its words wrap and never run
/// under the button beside it (a long fight title before `Add rewards`).
#[test]
fn a_wrapped_link_gives_way_in_a_row() {
    use super::kit::{Button, Link};
    use gpui::{IntoElement, ParentElement, Styled};
    let mut shell = Shell::new("gpui-link-wrap", Seed::Campaign);
    shell.update(|root, _, cx| {
        root.pages.reader.preview = Some(std::rc::Rc::new(|_, _, _| {
            gpui::div()
                .w(px(320.0))
                .flex()
                .flex_row()
                .items_start()
                .gap(super::theme::S)
                .child(
                    gpui::div().flex_1().min_w_0().child(
                        Link::new(
                            "long-title",
                            "Wolves at the Ford Below the Burning Shrine Road",
                        )
                        .wrap(),
                    ),
                )
                .child(Button::quiet("beside", "Add rewards"))
                .into_any_element()
        }));
        cx.notify();
    });
    let link = shell
        .bounds("Wolves at the Ford Below the Burning Shrine Road")
        .expect("the link");
    let button = shell.bounds("Add rewards").expect("the button");
    assert!(
        link.right() <= button.left(),
        "the link {link:?} ends before the button {button:?}"
    );
    assert!(
        link.size.height > super::theme::Type::Body.line() * 1.5,
        "its words wrap: {link:?}"
    );
}

/// The rail's Run slot shortens a long state to the word that matters (egui's
/// `run_nav_detail`), and keeps a state that fits whole.
#[test]
fn the_run_slot_detail_fits_the_rail() {
    let mut shell = Shell::new("gpui-run-detail", Seed::Campaign);
    let fit = |shell: &mut Shell, detail: &'static str, width: f32| {
        shell.update(move |_, window, _| super::kit::nav::fit_run_detail(window, detail, px(width)))
    };
    assert_eq!(fit(&mut shell, "Ended · Round 3", 120.0), "Ended");
    assert_eq!(fit(&mut shell, "Round 2 · Paused", 120.0), "Paused");
    assert_eq!(fit(&mut shell, "Ended · Round 3", 400.0), "Ended · Round 3");
}

/// Wait until `done`, letting the app's background work land (a load on a thread).
fn wait_for(shell: &mut Shell, what: &str, done: impl Fn(&super::SplatRoot) -> bool) {
    for _ in 0..400 {
        if shell.read(&done) {
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
        shell.wait(std::time::Duration::from_millis(50));
    }
    assert!(shell.read(&done), "{what}");
}

/// The first target whose name starts with `prefix`.
fn first_named(prefix: &str) -> String {
    super::kit::probe::names()
        .into_iter()
        .find(|name| name.starts_with(prefix))
        .unwrap_or_else(|| {
            panic!(
                "no target starts with {prefix:?}: {:?}",
                super::kit::probe::names()
            )
        })
}

/// The app's dialogs (the egui `dialogs` scenario): the copy dialog and its destination menu,
/// a renamed copy, its result and the copy's editor; then the close prompt over an unsaved
/// Library draft, where Escape and Enter both keep editing.
#[test]
fn dialogs() {
    let mut shell = Shell::new("dialogs", Seed::Campaign);
    shell.click("Library");
    wait_for(&mut shell, "the Library loads", |root| {
        root.app.library.loaded() && !root.app.library.loading()
    });
    shell.click("Creatures");
    shell.click(&first_named("Aboleth ·"));
    shell.click("Copy to…");
    wait_for(&mut shell, "the copy plan", |root| {
        root.app
            .copy_dialog
            .as_ref()
            .is_some_and(|dialog| dialog.plan().is_some())
    });
    shell.capture("dialogs__01-copy-dialog__1200x800");
    let destination = |shell: &mut Shell| {
        shell.read(|root| {
            root.app
                .copy_dialog
                .as_ref()
                .and_then(|dialog| dialog.destination())
        })
    };
    let first = destination(&mut shell);
    // The drop-down's menu opens over the dialog (drawn above it), and its rows take the press.
    shell.click("Destination");
    shell.capture("dialogs__02-copy-destination-menu__1200x800");
    shell.click("id:copy-destination-choice-2");
    assert_ne!(
        destination(&mut shell),
        first,
        "a row of the menu took the press"
    );
    shell.click("Destination");
    shell.click("id:copy-destination-choice-1");
    assert_eq!(destination(&mut shell), first);
    shell.click("Copy name");
    shell.press("ctrl-a");
    shell.type_text("Watch Aboleth");
    wait_for(&mut shell, "the plan names the new file", |root| {
        root.app
            .copy_dialog
            .as_ref()
            .is_some_and(|dialog| dialog.name() == "Watch Aboleth" && dialog.plan().is_some())
    });
    shell.capture("dialogs__03-copy-renamed__1200x800");
    shell.click("Create copy");
    assert!(
        shell.read(|root| root
            .app
            .copy_dialog
            .as_ref()
            .is_some_and(|dialog| dialog.completed().is_some())),
        "Create copy writes the copy"
    );
    shell.capture("dialogs__04-copy-complete__1200x800");
    shell.click("Edit the copy");
    wait_for(&mut shell, "the copy opens", |root| {
        root.app.copy_dialog.is_none() && !root.app.library.loading()
    });
    shell.capture("dialogs__05-edit-the-copy__1200x800");
    // A Library draft, and the window's close button.
    shell.click("id:library-scope");
    shell.click("Ashen Marches");
    shell.click("Ash Exposure · Rule");
    shell.click("Edit Markdown");
    shell.click("Markdown editor");
    shell.press("ctrl-end");
    shell.type_text(" Draft ruling.");
    assert!(shell.read(|root| root.app.library.dirty()));
    // The last input is the mouse's, as when the DM presses the window's close button.
    shell.click("Markdown editor");
    let close = |shell: &mut Shell| shell.update(|root, window, cx| root.should_close(window, cx));
    assert!(!close(&mut shell), "the draft keeps the window open");
    assert!(shell.read(|root| root.app.confirm_close));
    let path = shell.capture("dialogs__06-unsaved-changes-on-close__1200x800");
    // The safe default that has the keyboard shows the focus ring, though a press opened the
    // prompt (egui's ring on Keep editing), so the DM sees what Enter does.
    let keep = shell.bounds("Keep editing").expect("Keep editing");
    let picture = image::open(&path).expect("the capture").to_rgba8();
    let scale = picture.width() as f32 / f32::from(shell.size().width);
    // A target's box lies inside its border: the ring is the pixel left of it.
    let edge = picture.get_pixel(
        ((f32::from(keep.left()) - 1.0) * scale) as u32,
        (f32::from(keep.center().y) * scale) as u32,
    );
    let ring = gpui::Rgba::from(super::theme::colors().focus);
    let ring = [ring.r, ring.g, ring.b].map(|channel| (channel * 255.0).round() as i32);
    assert!(
        (0..3).all(|at| (i32::from(edge[at]) - ring[at]).abs() <= 8),
        "Keep editing's edge {edge:?} at {keep:?} is the focus ring {ring:?}"
    );
    shell.press("escape");
    assert!(
        !shell.read(|root| root.app.confirm_close),
        "Escape keeps editing"
    );
    assert!(shell.read(|root| root.app.library.dirty()));
    shell.capture("dialogs__07-kept-editing__1200x800");
    // Keep editing has the keyboard when the prompt opens: Enter keeps editing too.
    assert!(!close(&mut shell));
    shell.press("enter");
    assert!(
        !shell.read(|root| root.app.confirm_close),
        "Enter keeps editing"
    );
    assert!(shell.read(|root| root.app.library.dirty()));
}

/// Each page keeps its state while the DM visits the others (the egui `nav-state` scenario):
/// the Library's search and reader, Party's member and sheet page, the open encounter, and the
/// Campaign page's last tab.
#[test]
fn nav_state() {
    use crate::CampaignTab;
    let mut shell = Shell::new("nav-state", Seed::Campaign);
    shell.click("Library");
    wait_for(&mut shell, "the Library loads", |root| {
        root.app.library.loaded() && !root.app.library.loading()
    });
    shell.click("id:library-search");
    shell.type_text("dragon");
    shell.click(&first_named("Adult Black Dragon ·"));
    let path = shell.capture("nav-state__01-library-set-up__1200x800");
    // The search field shows its magnifier at its left, before the words (egui's search box):
    // the strip from its edge to where the words start holds the icon's strokes.
    let field = shell
        .bounds("id:library-search")
        .expect("the Library's search");
    let picture = image::open(&path).expect("the capture").to_rgba8();
    let scale = picture.width() as f32 / f32::from(shell.size().width);
    let well = *picture.get_pixel(
        ((f32::from(field.left()) + 3.0) * scale) as u32,
        (f32::from(field.center().y) * scale) as u32,
    );
    let marked = (4..14)
        .flat_map(|x| (-6..=6).map(move |y| (x, y)))
        .filter(|(x, y)| {
            let pixel = picture.get_pixel(
                ((f32::from(field.left()) + *x as f32) * scale) as u32,
                ((f32::from(field.center().y) + *y as f32) * scale) as u32,
            );
            (0..3).any(|at| (i32::from(pixel[at]) - i32::from(well[at])).abs() > 40)
        })
        .count();
    assert!(marked > 4, "the magnifier is drawn: {marked} marked pixels");
    shell.click("Party");
    wait_for(&mut shell, "the party is read", |root| {
        !root.app.party.reader.members.is_empty() && !root.app.party.reader.loading()
    });
    shell.click(&first_named("Iselle Marrowind"));
    shell.click("Spells");
    let party = |root: &super::SplatRoot| {
        (
            root.app.party.reader.selected.clone(),
            root.pages.party.page,
        )
    };
    let party_set_up = shell.read(party);
    assert_eq!(party_set_up.1, 2, "the Spells page");
    // Kestrel's detail line ("Rogue · Level 5 · Bloodied · Poisoned") is whole, as egui's was:
    // it runs under the compare columns, up to HP.
    let room = shell
        .bounds("Kestrel Dunmoor detail")
        .expect("Kestrel's detail line");
    let words = shell
        .bounds("Kestrel Dunmoor detail words")
        .expect("Kestrel's detail words");
    assert!(
        words.right() <= room.right(),
        "Kestrel's detail {words:?} fits its line {room:?}"
    );
    shell.capture("nav-state__02-party-set-up__1200x800");
    shell.click("Encounters");
    wait_for(&mut shell, "the encounters are read", |root| {
        !root.app.encounters.busy() && root.app.encounters.loaded()
    });
    // The paused run's encounter, as egui's scenario opened it (Edit beside Bridge Ambush, its
    // In progress row): its editor offers Resume.
    let paused = shell
        .read(|root| root.app.encounters.paused_run().map(|run| run.path.clone()))
        .expect("the fixture's paused run");
    shell.click(&format!("id:edit:{}", paused.display()));
    assert!(shell.read(|root| root.app.encounters.editor_open()));
    let head = shell.update(|root, _, _| {
        let active = root.app.campaigns.active_campaign().expect("a campaign");
        root.app
            .encounters
            .editor_frame(&active)
            .map(|view| (view.name, view.start_word, view.run_state))
    });
    assert_eq!(
        head,
        Some((
            "Bridge Ambush".to_owned(),
            "Resume",
            Some("Round 2 · Paused".to_owned())
        )),
        "the editor shows the paused run's encounter, with Resume and its state"
    );
    assert!(
        shell.bounds("id:encounter-start").is_some(),
        "its Resume is drawn"
    );
    shell.capture("nav-state__03-encounters-set-up__1200x800");
    shell.click("Campaign");
    shell.click("Rules and runs");
    shell.click("Settings");
    shell.capture("nav-state__04-settings__1200x800");
    shell.click("Library");
    assert_eq!(
        shell.read(|root| root.app.library.search_text().to_owned()),
        "dragon",
        "the Library keeps its search"
    );
    assert!(shell.bounds(&first_named("Adult Black Dragon ·")).is_some());
    shell.click("Campaign");
    assert!(
        shell.read(|root| root.app.campaign_tab == CampaignTab::RunSettings),
        "the Campaign page reopens its last tab"
    );
    // A drop-down's value is in `Body`, as a text field's is (egui's `value_font`).
    let value = shell
        .bounds("Roll NPC initiative value")
        .expect("the NPC initiative value");
    assert_eq!(value.size.height, super::theme::Type::Body.line());
    shell.capture("nav-state__06-campaign-returned__1200x800");
    shell.click("Party");
    assert_eq!(
        shell.read(party),
        party_set_up,
        "Party keeps its member and page"
    );
    shell.click("Encounters");
    assert!(
        shell.read(|root| root.app.encounters.editor_open()),
        "Encounters keeps its open editor"
    );
}
