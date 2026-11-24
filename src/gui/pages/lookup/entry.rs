//! The entry Lookup shows: its name, its kind line and its words in the reader's prose, with its
//! links opening in Lookup. The full reader (stat blocks, NPC cards, edit forms, `Add to fight`)
//! is `world::Reader`'s, which the World and Reader ports draw in GPUI; this is Lookup's own view
//! of an entry until that reader is wired in here.
use crate::gui::kit;
use crate::gui::theme::{S, Type, XS, colors};
use crate::gui::SplatRoot;
use gpui::{AnyElement, Context, IntoElement, ParentElement, SharedString, Styled, Window, div};

pub(super) fn render(
    root: &mut SplatRoot,
    position: usize,
    _: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let Some(index) = root.app.lookup.index() else {
        return div().into_any_element();
    };
    let Some(item) = index.items.get(position) else {
        return div().into_any_element();
    };
    let entry = &item.entry;
    let detail = index.result_detail(position);
    let markdown = index.reader_markdown(&entry.body, Some(entry.id));
    let from = entry.id;
    let mut column = div()
        .flex()
        .flex_col()
        .gap(XS)
        .pr(S)
        .child(kit::text(Type::Display, c.text, entry.name.clone()))
        .child(kit::text(Type::Small, c.muted, detail))
        .child(kit::gap(S));
    if !markdown.trim().is_empty() {
        column = column.child(
            kit::markdown::Markdown::new(("lookup-entry", position), SharedString::from(markdown))
                .compact()
                .on_link(cx.listener(move |root, target: &str, _, cx| {
                    let SplatRoot { app, bridge, .. } = root;
                    app.lookup.open_note_link(target, Some(from), bridge.ctx());
                    cx.notify();
                })),
        );
    }
    kit::scroll_column(("lookup-entry-scroll", position), column).into_any_element()
}
