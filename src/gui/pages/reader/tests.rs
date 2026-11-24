//! The Reader's screenshots and interaction tests: the Markdown view and the entry views, drawn
//! in place of the page (`State::preview`) at the widths the egui scenarios show them.
use super::Preview;
use crate::gui::harness::Shell;
use crate::gui::kit;
use crate::gui::theme::{L, READER_MEASURE, colors};
use crate::visual_tests::fixture::Seed;
use gpui::{IntoElement, ParentElement, Styled, div, px};
use std::rc::Rc;

/// The egui gallery's note (`design::gallery::NOTE`), as its Read column shows it.
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
