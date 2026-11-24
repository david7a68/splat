//! The Reader's screenshots and interaction tests: the Markdown view and the entry views, drawn
//! in place of the page (`State::preview`) at the widths the scenarios show them.
use super::Preview;
use crate::fixtures::Seed;
use crate::gui::harness::Shell;
use crate::gui::kit;
use crate::gui::theme::{L, READER_MEASURE, colors};
use gpui::{IntoElement, ParentElement, Styled, div, px};
use std::rc::Rc;

/// The gallery's note, as its Read column shows it.
const NOTE: &str = "# Session 4: The Old Mill

2026-11-02, The Emberfall Marches

## Plan for tonight

*How does the session begin? Start close to the action.*

## What happened

We reached *the old mill* at dusk. **Oren** lit the lamps, and ***nobody*** spoke. The road behind us was covered in ash, and it stayed that way until the wolves came down from the ridge.

- Roll `1d20 + 5` for the bridge
- Bridge Ambush left two wolves
- ~~Kestrel has the key~~ The key is lost
-

> The miller said the road was covered in ash.

### Later that night

See [the map](https://example.com) for the road north.

```
Reward: 40 gp
```
";

/// Show `part` in place of the page, in a column `width` wide.
fn show(shell: &mut Shell, width: f32, part: impl Fn() -> gpui::AnyElement + 'static) {
    let preview: Preview = Rc::new(move |_, _, _| {
        div()
            .size_full()
            .bg(colors().page)
            .p(L)
            .child(div().w(px(width)).child(part()))
            .into_any_element()
    });
    shell.update(|root, _, cx| {
        root.pages.reader.preview = Some(preview);
        cx.notify();
    });
    shell.settle();
}

#[test]
fn gallery_markdown() {
    let mut shell = Shell::new("gallery", Seed::Empty);
    show(&mut shell, f32::from(READER_MEASURE) + 40.0, || {
        kit::markdown("note", NOTE).into_any_element()
    });
    shell.capture("gallery__07-markdown__1200x800");
}

/// Show `part` in place of the page in a scroll column `width` wide; the column's handle.
fn show_scrolled(
    shell: &mut Shell,
    width: f32,
    part: impl Fn(&gpui::ScrollHandle) -> gpui::AnyElement + 'static,
) -> gpui::ScrollHandle {
    use gpui::{InteractiveElement, StatefulInteractiveElement};
    let scroll = gpui::ScrollHandle::new();
    let column = scroll.clone();
    let preview: Preview = Rc::new(move |_, _, _| {
        div()
            .size_full()
            .bg(colors().page)
            .p(L)
            .child(
                div()
                    .id("reader-column")
                    .w(px(width))
                    .h_full()
                    .overflow_y_scroll()
                    .track_scroll(&column)
                    .child(part(&column)),
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

/// Wait for the campaign's content index, which loads in the background.
fn with_index(shell: &mut Shell) {
    for _ in 0..300 {
        if shell.read(|root| root.app.lookup.index().is_some()) {
            return;
        }
        shell.wait(std::time::Duration::from_millis(100));
        // The index loads on a real thread: give it real time too, as a loaded machine needs.
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    panic!("the content index never loaded");
}

/// Press the start of the linked paragraph whose words start with `start`.
fn press_words(shell: &mut Shell, start: &str) {
    let name = crate::gui::kit::probe::names()
        .into_iter()
        .find(|name| name.starts_with(start))
        .unwrap_or_else(|| panic!("no linked paragraph starts with {start:?}"));
    let bounds = shell.bounds(&name).expect("the paragraph's place");
    shell.click_at(gpui::point(
        bounds.left() + px(8.0),
        bounds.top() + px(10.0),
    ));
}

#[test]
fn a_note_link_opens_its_note_in_lookup() {
    let mut shell = Shell::new("markdown-links", Seed::World);
    with_index(&mut shell);
    show_scrolled(&mut shell, f32::from(READER_MEASURE), |_| {
        kit::markdown("setting", "[[Verori|The home country]]").into_any_element()
    });
    assert!(
        !shell.read(|root| root.app.lookup.open),
        "Lookup starts closed"
    );
    press_words(&mut shell, "The home country");
    assert!(
        shell.read(|root| root.app.lookup.open),
        "the frame logic routed the note link to Lookup"
    );
}

#[test]
fn a_dice_link_and_a_roll_table_roll() {
    let mut shell = Shell::new("markdown-links", Seed::World);
    with_index(&mut shell);
    let link = crate::content::dice_text::RollLink::d20(
        crate::content::dice_text::D20::Check,
        6,
        "Strength",
    );
    let source = format!(
        "{} to lift the gate.\n\n<!-- splat:rolls key=forced_trip dice=1d20 -->\n\n| d20 | Result |\n| --- | --- |\n| 1-5 | No connection. |\n| 6-10 | Rough arrival. |\n| 11-20 | Clean arrival. |\n",
        link.markdown("+6")
    );
    show_scrolled(&mut shell, f32::from(READER_MEASURE), move |_| {
        kit::markdown("gate", source.clone()).into_any_element()
    });
    shell.update(|root, _, _| crate::rolls::set_entry(&root.ctx, "Gate Mechanics"));
    press_words(&mut shell, "+6");
    let rolled = shell
        .read(|root| crate::rolls::plain::preview_words(&root.ctx, "Gate Mechanics", &[&link]));
    assert!(rolled.is_some(), "the dice link asked for its roll");
    assert!(
        shell
            .bounds(crate::gui::kit::markdown::ROLL_RESULT)
            .is_none()
    );
    shell.click("Roll 1d20");
    assert!(
        shell
            .bounds(crate::gui::kit::markdown::ROLL_RESULT)
            .is_some(),
        "the table's roll came back to this reader"
    );
    shell.capture("markdown-links__08-roll-table-rolled__1200x800");
}

#[test]
fn reader_tables() {
    const TABLE: &str = "| Level | Proficiency Bonus | Features | Cantrips | Prepared Spells | 1st | 2nd | 3rd | 4th | 5th | 6th | 7th | 8th | 9th |\n\
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |\n\
| 5 | +3 | Memorize Spell | 4 | 9 | 4 | 3 | 2 | 0 | 0 | 0 | 0 | 0 | Ninth-slot |\n\n\
The table leaves this note readable.\n\n\
Read the row for your class level. A slot is spent when you cast a spell of that level or higher, and every slot comes back when you finish a Long Rest, so plan the hardest fights of the day around the slots you still have.\n";
    for (window, width, name, scrolls) in [
        (
            (1200.0, 800.0),
            500.0,
            "reader-tables__01-wide-table-in-library__1200x800",
            true,
        ),
        (
            (960.0, 640.0),
            380.0,
            "reader-tables__01-wide-table-in-library__960x640",
            true,
        ),
        (
            (1920.0, 1080.0),
            1100.0,
            "reader-tables__01-wide-table-in-library__1920x1080",
            false,
        ),
    ] {
        let mut shell = Shell::with_size(
            "reader-tables",
            Seed::Empty,
            gpui::size(px(window.0), px(window.1)),
        );
        show_scrolled(&mut shell, width, |_| {
            kit::Markdown::new("table", TABLE)
                .wide_tables()
                .into_any_element()
        });
        assert_eq!(
            shell
                .bounds("More columns: scroll sideways or use Shift+wheel.")
                .is_some(),
            scrolls,
            "{name}: a table scrolls sideways only where it does not fit"
        );
        shell.capture(name);
    }
}

#[test]
fn a_typed_jump_scrolls_to_its_heading() {
    let mut shell = Shell::new("markdown-links", Seed::Empty);
    let long = format!(
        "{}\n\n## Later that night\n\nThe end.\n",
        "Words on a line.\n\n".repeat(60)
    );
    let scroll = show_scrolled(&mut shell, f32::from(READER_MEASURE), move |handle| {
        kit::markdown("long", long.clone())
            .jump_in(handle)
            .into_any_element()
    });
    assert_eq!(scroll.offset().y, px(0.0));
    let prepares = crate::gui::kit::markdown::PREPARES.with(std::cell::Cell::get);
    shell.update(|root, _, cx| {
        crate::markdown_view::want_heading(&root.ctx, Some("Later that night".into()));
        cx.notify();
    });
    shell.settle();
    assert!(
        scroll.offset().y < px(-200.0),
        "the column scrolled down to the heading: {:?}",
        scroll.offset()
    );
    // The jump and the scroll draw the note again; its links are not resolved again.
    assert_eq!(
        crate::gui::kit::markdown::PREPARES.with(std::cell::Cell::get),
        prepares,
        "a repaint of an unchanged note prepares it again"
    );
}
