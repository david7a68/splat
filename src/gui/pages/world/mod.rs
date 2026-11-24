//! The World page (egui: `src/world.rs`, `src/world/`, `src/world_page/`): one list of the
//! campaign's notes and entries, and the reader of the one that is open.
//!
//! The view keeps no World state of its own. It reads `root.app.world_page` (the egui page's own
//! state, `world::page::WorldPage`) through its view API, draws it, and calls the same acts the
//! egui page calls (`WorldPage::press_note`, `Reader::press_title`, …). What only GPUI needs (the
//! list's virtual state, the footer's presses of this frame) is in `State`.
use crate::gui::{SplatRoot, icons, kit, theme};
use crate::lookup::ContentIndex;
use crate::persistence::Campaign;
use gpui::{
    AnyElement, App, Context, InteractiveElement, IntoElement, KeyBinding, ParentElement, Styled,
    Window, actions, div, prelude::*, px,
};
use std::sync::Arc;

mod footer;
mod list;
mod panes;
mod reader;
#[cfg(test)]
mod tests;

actions!(world, [Escape]);

/// The width the list column starts at, and its range (egui's `list_column`).
const LIST_WIDTH: f32 = 260.0;
const LIST_RANGE: std::ops::Range<f32> = 190.0..480.0;

/// This area's GPUI-only state, kept in `root.pages.world`.
#[derive(Default)]
pub struct State {
    /// The notes list's rows and their virtual list, rebuilt only when what they show changes.
    list: list::ListCache,
    /// The presses of the footer lines drawn in this frame: a button takes its press by its
    /// place here, once (a kept line's ticket names its store's entry).
    pub(crate) presses: footer::Presses<crate::world::FooterAct>,
    /// The presses of the repair pane's footer, as `presses`.
    pub(crate) repair_presses: footer::Presses<crate::world::repair::RepairAct>,
}

/// Once, when the root is built.
pub(crate) fn init(_root: &mut SplatRoot, _window: &mut Window, cx: &mut Context<SplatRoot>) {
    cx.bind_keys([KeyBinding::new("escape", Escape, Some("World"))]);
}

/// Once per frame, before anything draws.
pub(crate) fn poll(_root: &mut SplatRoot, _window: &mut Window, _cx: &mut Context<SplatRoot>) {}

/// The page and what it acts on in a handler: the open campaign and the loaded notes.
pub(crate) fn world(root: &SplatRoot) -> Option<(Campaign, Arc<ContentIndex>)> {
    Some((root.app.campaigns.active_campaign()?, root.app.lookup.index()?))
}

/// A handler of `E` that acts on the page with the open campaign and the loaded notes, then
/// draws again. It does nothing while either is missing.
pub(crate) fn act<E: ?Sized + 'static>(
    cx: &mut Context<SplatRoot>,
    f: impl Fn(&mut SplatRoot, &Campaign, &ContentIndex, &mut Window, &mut Context<SplatRoot>)
    + 'static,
) -> Handler<E> {
    Box::new(cx.listener(move |root: &mut SplatRoot, _: &E, window, cx| {
        if let Some((campaign, index)) = world(root) {
            f(root, &campaign, &index, window, cx);
            cx.notify();
        }
    }))
}

/// A boxed handler of `E`, which a kit part's `on_click` (or `on_change`) takes.
pub(crate) type Handler<E> = Box<dyn Fn(&E, &mut Window, &mut App) + 'static>;

/// A handler of a field's words.
pub(crate) fn on_words(
    cx: &mut Context<SplatRoot>,
    f: impl Fn(&mut SplatRoot, &str) + 'static,
) -> Handler<str> {
    Box::new(cx.listener(move |root: &mut SplatRoot, words: &str, _, cx| {
        f(root, words);
        cx.notify();
    }))
}

/// A handler with no event (Enter or Escape in a field).
pub(crate) fn on_key(
    cx: &mut Context<SplatRoot>,
    f: impl Fn(&mut SplatRoot, &Campaign, &ContentIndex, &mut Window, &mut Context<SplatRoot>)
    + 'static,
) -> Box<dyn Fn(&mut Window, &mut App) + 'static> {
    let handler = act::<()>(cx, f);
    Box::new(move |window, cx| handler(&(), window, cx))
}

/// The page.
pub(crate) fn render(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let Some(campaign) = root.app.campaigns.active_campaign() else {
        return div().into_any_element();
    };
    root.pages.world.presses.clear();
    let index = root.app.lookup.index();
    root.app.world_page.follow_campaign(campaign.id);
    let header = header(root, index.as_deref(), cx);
    let page = div()
        .key_context("World")
        .on_action(cx.listener(escape))
        .flex()
        .flex_col()
        .size_full()
        .child(header);
    let Some(index) = index else {
        // The notes are still loading.
        return page
            .child(kit::text(
                theme::Type::Small,
                theme::colors().muted,
                "Wait for the notes to load before creating a note.",
            ))
            .into_any_element();
    };
    root.app.world_page.refresh(&index, &campaign);
    let groups = index.world_groups(&root.app.world_page.search().trim().to_lowercase());
    if root.app.world_page.is_empty(&index, &groups) {
        return page
            .child(kit::empty_state(
                icons::WORLD,
                "No world notes yet",
                "World notes are the Markdown files in this campaign’s World folder.",
                Some(new_menu("world-empty-new", false, cx)),
            ))
            .into_any_element();
    }
    drop(groups);
    let list = list::render(root, &index, window, cx);
    let pane = panes::render(root, &campaign, &index, window, cx);
    page.child(
        div().flex_1().min_h_0().child(
            kit::Columns::new("world-columns")
                .column(
                    px(LIST_WIDTH),
                    px(LIST_RANGE.start)..px(LIST_RANGE.end),
                    div().size_full().pr(theme::S).child(list),
                )
                .rest(div().size_full().pl(theme::L).child(pane)),
        ),
    )
    .into_any_element()
}

/// Escape on the page: an open picker closes, else the root's Escape goes on (Lookup, the notes
/// panel).
fn escape(root: &mut SplatRoot, _: &Escape, _: &mut Window, cx: &mut Context<SplatRoot>) {
    if root.app.world_page.reader.escape() {
        cx.notify();
    } else {
        cx.propagate();
    }
}

/// The header (M0603): the title and its count, `New ▾`, the count of notes that need fixing
/// with its menu of them, and the count that cannot be read, which opens the first one's
/// repair. The actions follow the title, as egui's title line places them.
fn header(
    _root: &mut SplatRoot,
    index: Option<&ContentIndex>,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = theme::colors();
    let (count, fixing, unreadable) = index
        .map(crate::world::page::WorldPage::header_counts)
        .unwrap_or_default();
    let mut line = div()
        .flex()
        .flex_row()
        .flex_wrap()
        .items_center()
        .gap(theme::S)
        .min_h(theme::Density::Scan.control())
        .child(
            div()
                .flex()
                .flex_row()
                .items_baseline()
                .gap(theme::S + theme::XS)
                .child(kit::text(theme::Type::Title, c.text, "World").whitespace_nowrap())
                .when_some(count, |title, count| {
                    title.child(kit::text(theme::Type::Small, c.muted, count))
                }),
        )
        .child(new_menu("world-new", index.is_none(), cx));
    if let (Some(fixing), Some(index)) = (fixing, index) {
        let mut menu = kit::StatusMenu::new(
            "world-fixing",
            kit::Status::new(icons::WARN, fixing, c.result.warning),
        );
        for &at in index.note_faults().listed() {
            let id = index.items[at].entry.id;
            menu = menu.item(kit::MenuItem::new(index.items[at].entry.name.clone()).on_click(
                act(cx, move |root, _, _, _, _| root.app.world_page.press_note(id)),
            ));
        }
        line = line.child(menu);
    }
    if let Some(unreadable) = unreadable {
        line = line.child(
            kit::status_link(
                "world-unreadable",
                kit::Status::new(icons::ERROR, unreadable, c.result.error),
            )
            .on_click(act(cx, |root, campaign, index, _, _| {
                root.app.world_page.press_unreadable(campaign, index);
            })),
        );
    }
    div()
        .flex()
        .flex_col()
        .w_full()
        .flex_none()
        .child(line)
        .child(kit::rule().mt(theme::LINE_GAP))
        .child(kit::gap(theme::L))
        .into_any_element()
}

/// `New ▾` and its choices (`Note…`, `NPC…`, `Location…`, `Generate NPC`). `…` marks a choice
/// that asks for a name first. Blocked while the notes load.
fn new_menu(id: &'static str, loading: bool, cx: &mut Context<SplatRoot>) -> AnyElement {
    let mut menu = kit::MenuButton::new(
        id,
        kit::Button::secondary((id, 0u32), format!("New {}", icons::DOWN))
            .icon(icons::ADD)
            .name("New ▾")
            .blocked(loading.then_some("Wait for the notes to load before creating a note.")),
    );
    for (word, kind) in [
        ("Note…", "note"),
        ("NPC…", "npc"),
        ("Location…", "location"),
        ("Generate NPC", "generate"),
    ] {
        menu = menu.item(
            kit::MenuItem::new(word).on_click(cx.listener(move |root: &mut SplatRoot, _, _, cx| {
                let index = root.app.lookup.index();
                root.app.world_page.press_new(kind, index.as_deref());
                cx.notify();
            })),
        );
    }
    menu.into_any_element()
}
