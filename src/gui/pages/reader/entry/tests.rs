//! The entry views' screenshots and interaction tests, drawn in place of the page at the Library
//! reader's width, with the app's content index for links when the fixture has one.
use super::{EntryView, Source, index_links, title_with_art};
use crate::gui::harness::Shell;
use crate::gui::pages::reader::Preview;
use crate::gui::theme::{L, colors};
use crate::library::view::{Links, NoLinks};
use crate::visual_tests::fixture::Seed;
use gpui::{
    InteractiveElement, IntoElement, ParentElement, Pixels, ScrollHandle, Size,
    StatefulInteractiveElement, Styled, div, point, px, size,
};
use splat::content::Entry;
use splat::content::dice_text::{D20, RollLink};
use std::cell::Cell;
use std::rc::Rc;

/// A built-in entry from the 2024 definitions (`definitions/<kind>/<file>.md`).
fn builtin(path: &str) -> Entry {
    let name = path.rsplit('/').next().unwrap_or(path);
    Entry::parse(
        name,
        splat::content::definitions::source_file(path)
            .unwrap_or_else(|| panic!("{path} is a built-in definition")),
    )
    .unwrap_or_else(|error| panic!("{path}: {error}"))
}

/// The width of the Library's reader pane at 1200×800 (as `library-browse__03`).
const READER: f32 = 500.0;

/// Show `entry` as the Library's reader does, in a column `width` wide. Its links are the app's
/// content index when one has loaded; a followed link's place goes to `opened`.
fn show_entry(
    shell: &mut Shell,
    width: f32,
    entry: Entry,
    subtitle: &'static str,
    opened: Rc<Cell<Option<usize>>>,
) -> ScrollHandle {
    let scroll = ScrollHandle::new();
    let column = scroll.clone();
    let preview: Preview = Rc::new(move |root, _, cx| {
        let source = Source {
            path: entry.path.clone(),
            owner: "Built-in".into(),
            builtin: true,
        };
        let links: Rc<dyn Links> = root
            .app
            .lookup
            .index()
            .map_or_else(|| Rc::new(NoLinks) as Rc<dyn Links>, index_links);
        let opened = opened.clone();
        div()
            .size_full()
            .bg(colors().page)
            .p(L)
            .child(
                div()
                    .id("entry-column")
                    .w(px(width))
                    .h_full()
                    .overflow_y_scroll()
                    .track_scroll(&column)
                    .flex()
                    .flex_col()
                    .child(title_with_art(&entry.name, subtitle, Some(&entry), cx))
                    .child(
                        EntryView::new(
                            ("entry", entry.id.as_u128() as u64),
                            entry.name.clone(),
                            Some(&entry),
                            entry.body.clone(),
                            source,
                            links,
                        )
                        .on_open(move |index, _, _| opened.set(Some(index))),
                    ),
            )
            .into_any_element()
    });
    shell.update(|root, _, cx| {
        root.pages.reader.preview = Some(preview);
        cx.notify();
    });
    shell.settle();
    scroll
}

/// Scroll the column until the target `name` is wholly in the window, then settle.
fn scroll_to(shell: &mut Shell, scroll: &ScrollHandle, name: &str) {
    let height = shell.size().height;
    for _ in 0..60 {
        if shell
            .bounds(name)
            .is_some_and(|bounds| bounds.top() >= px(0.0) && bounds.bottom() <= height)
        {
            return;
        }
        let offset = scroll.offset();
        scroll.set_offset(point(offset.x, offset.y - px(200.0)));
        shell.settle();
    }
    panic!("{name} never came into view");
}

fn shell(name: &str, seed: Seed, window: Size<Pixels>) -> Shell {
    let mut shell = Shell::with_size(name, seed, window);
    // The content index loads in the background.
    for _ in 0..300 {
        if shell.read(|root| root.app.lookup.index().is_some()) {
            break;
        }
        shell.wait(std::time::Duration::from_millis(100));
    }
    shell
}

const DEFAULT: (f32, f32) = (1200.0, 800.0);
const SMALL: (f32, f32) = (960.0, 640.0);

fn at((w, h): (f32, f32)) -> Size<Pixels> {
    size(px(w), px(h))
}

#[test]
fn creature_stat_block_and_its_dice() {
    for (window, name) in [
        (DEFAULT, "library-browse__03-creature-stat-block__1200x800"),
        (SMALL, "library-browse__03-creature-stat-block__960x640"),
    ] {
        let mut shell = shell("library-browse", Seed::Empty, at(window));
        let opened = Rc::new(Cell::new(None));
        show_entry(
            &mut shell,
            READER.min(window.0 - 700.0),
            builtin("definitions/creatures/adult_black_dragon.md"),
            "Creature · CR 14 · D&D 2024 · SRD 5.2.1",
            opened,
        );
        shell.capture(name);
    }
    // A press on an ability's modifier asks for its check; outside a run the preview stream
    // rolls it, and its result is kept for the entry.
    let mut shell = shell("library-browse", Seed::Empty, at(DEFAULT));
    let scroll = show_entry(
        &mut shell,
        READER,
        builtin("definitions/creatures/adult_black_dragon.md"),
        "Creature · CR 14 · D&D 2024 · SRD 5.2.1",
        Rc::new(Cell::new(None)),
    );
    let check = RollLink::d20(D20::Check, 6, "Strength");
    shell.click(&format!("{} +6", crate::rolls::link_label(&check)));
    let rolled = shell.read(|root| {
        crate::rolls::plain::preview_words(root.bridge.ctx(), "Adult Black Dragon", &[&check])
    });
    assert!(
        rolled.is_some_and(|parts| !parts.is_empty()),
        "the Strength check was rolled for the Adult Black Dragon"
    );
    // In a run, the armed action's tile row says what to do now and offers what can follow; a
    // press on a follow-up goes to the run (egui `rolls::tile_row` under `show_run`).
    let dragon = builtin("definitions/creatures/adult_black_dragon.md");
    let armed = crate::rolls::plain::actions_of(&dragon.body)[0]
        .1
        .link
        .clone();
    shell.update(|root, _, cx| {
        let ctx = root.bridge.ctx();
        crate::rolls::set_roller(ctx, Some(0));
        crate::rolls::set_armed(
            ctx,
            Some(crate::rolls::ArmedView {
                link: armed,
                occurrence: None,
                status: "Click a target. Escape stops.".into(),
                reserve: String::new(),
                actions: vec![("Stop", crate::rolls::ArmedAction::Stop)],
            }),
        );
        cx.notify();
    });
    shell.settle();
    scroll_to(&mut shell, &scroll, "Stop");
    assert!(
        shell.bounds("Stop").is_some(),
        "the armed action's follow-up shows under its tiles: {:?}",
        crate::gui::kit::probe::names()
    );
    shell.capture("reader-armed-tile__01-armed-action-in-a-run__1200x800");
    shell.click("Stop");
    assert_eq!(
        shell.read(|root| crate::rolls::take_armed_action(root.bridge.ctx())),
        Some(crate::rolls::ArmedAction::Stop),
        "Stop goes to the run"
    );
}

#[test]
fn spell_facts() {
    let mut shell = shell("library-browse", Seed::Empty, at(DEFAULT));
    show_entry(
        &mut shell,
        READER,
        builtin("definitions/spells/fireball.md"),
        "Spell · Level 3 · D&D 2024 · SRD 5.2.1",
        Rc::new(Cell::new(None)),
    );
    shell.capture("library-browse__04-spell__1200x800");
}

#[test]
fn class_traits_and_features_open_their_entries() {
    let mut shell = shell("library-browse", Seed::Campaign, at(DEFAULT));
    let opened = Rc::new(Cell::new(None));
    let scroll = show_entry(
        &mut shell,
        READER,
        builtin("definitions/classes/wizard.md"),
        "Class · D&D 2024 · SRD 5.2.1",
        opened.clone(),
    );
    shell.capture("library-browse__05-class-traits-and-features__1200x800");
    let index = shell
        .read(|root| root.app.lookup.index())
        .expect("the campaign's content index loads");
    let wizard = builtin("definitions/classes/wizard.md");
    let first = crate::library::class::features(&wizard, &*index)
        .into_iter()
        .find_map(|feature| Some((feature.target?, feature.name)))
        .expect("the Wizard has a feature in the index");
    // Scroll the feature into view, then press it.
    let name = first.1.clone();
    scroll_to(&mut shell, &scroll, &name);
    shell.click(&name);
    assert_eq!(opened.get(), Some(first.0), "the feature's entry opens");
}

#[test]
fn the_end_sections_open_on_a_press() {
    let mut shell = shell("library-browse", Seed::Empty, at(DEFAULT));
    let scroll = show_entry(
        &mut shell,
        READER,
        builtin("definitions/spells/fireball.md"),
        "Spell · Level 3 · D&D 2024 · SRD 5.2.1",
        Rc::new(Cell::new(None)),
    );
    assert!(
        shell.bounds("What Splat applies").is_none(),
        "the section starts closed"
    );
    scroll_to(&mut shell, &scroll, "Source and identifiers");
    shell.click("Source and identifiers");
    assert!(
        shell.bounds("What Splat applies").is_some(),
        "the open section shows what is inside it"
    );
}
