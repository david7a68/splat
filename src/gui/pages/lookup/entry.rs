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
    let markdown = prose(&index.reader_markdown(&entry.body, Some(entry.id)), &entry.name);
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

/// The entry's words as the reader shows them: its own title is the heading above, so a first
/// `# Name` line goes, and Obsidian's `%% … %%` comments are not words to read.
fn prose(markdown: &str, name: &str) -> String {
    let mut text = markdown.trim_start();
    if let Some(rest) = text.strip_prefix("# ")
        && let Some((title, after)) = rest.split_once('\n')
        && title.trim().eq_ignore_ascii_case(name.trim())
    {
        text = after;
    }
    let mut out = String::with_capacity(text.len());
    let mut parts = text.split("%%");
    if let Some(first) = parts.next() {
        out.push_str(first);
    }
    // Every other part sits between two marks: a comment.
    for (at, part) in parts.enumerate() {
        if at % 2 == 1 {
            out.push_str(part);
        }
    }
    out
}
