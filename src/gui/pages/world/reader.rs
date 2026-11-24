//! The note reader (egui: `world::Reader::read_in_pane` and `draw`): the title line and its
//! actions, the Log field, the file line, the note's faults with their repairs, the value rows
//! with Change and their pickers, the note's text, what other notes say about it, and the pane's
//! footer. Every act is the reader's own (`world::model`).
use super::{SplatRoot, act, footer, on_key, on_words};
use crate::gui::{icons, kit, theme};
use crate::lookup::{ContentIndex, Item};
use crate::persistence::Campaign;
use crate::world::Nav;
use crate::world::model::{
    BodyFacts, JoinAct, PickAct, TitleAct, ValueShown, file_link, missing_text, obsidian_link,
};
use gpui::{
    AnyElement, Context, Hsla, IntoElement, ParentElement, SharedString, Styled, Window, div,
    prelude::*, px,
};
use uuid::Uuid;

/// The label column of a value row (egui: `XXL * 3 + S`).
const LABEL_COLUMN: f32 = 104.0;

/// The widest value column (egui: `relationship_width`, `XXL * 7 + L`).
const VALUE_COLUMN: gpui::Pixels = px(240.0);

/// The width of `words` set in `ty`.
fn words_width(window: &Window, ty: theme::Type, words: &str) -> gpui::Pixels {
    let mut font = gpui::font(ty.family());
    font.weight = ty.weight();
    font.style = ty.style();
    let run = gpui::TextRun {
        len: words.len(),
        font,
        color: theme::colors().text,
        background_color: None,
        underline: None,
        strikethrough: None,
    };
    window
        .text_system()
        .shape_line(
            SharedString::from(words.to_owned()),
            ty.size(),
            &[run],
            None,
        )
        .width
}

/// What an act on a note reaches: the host's reader, and where the act goes next.
struct ReaderAct<'a> {
    reader: &'a mut crate::world::Reader,
    nav: Option<Nav>,
}

impl ReaderAct<'_> {
    /// Go to `nav` after the act (the host takes it: World's page, or Lookup).
    fn take_nav(&mut self, nav: Nav, _: &ContentIndex) {
        self.nav = Some(nav);
    }
}

/// A handler that acts on the note `id` with the host's reader, its item, the notes and the
/// campaign; the `Nav` an act returns goes to the host.
fn on_item<E: ?Sized + 'static>(
    cx: &mut Context<SplatRoot>,
    host: Host,
    id: Uuid,
    f: impl Fn(&mut ReaderAct<'_>, &Item, &ContentIndex, &Campaign) -> Option<Nav> + 'static,
) -> super::Handler<E> {
    act(cx, move |root, campaign, index, _, _| {
        let Some(at) = index.position(id) else {
            return;
        };
        let mut page = ReaderAct {
            reader: host.reader(root),
            nav: None,
        };
        let nav = f(&mut page, &index.items[at], index, campaign);
        if let Some(nav) = nav.or(page.nav) {
            host.take_nav(root, nav, index);
        }
    })
}

/// The open note's pane: the reader as the pane's body, its messages in the footer.
pub(super) fn render(
    root: &mut SplatRoot,
    campaign: &Campaign,
    index: &ContentIndex,
    at: usize,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let item = &index.items[at];
    let page = &mut root.app.world_page;
    page.reader.open_item(item);
    page.reader.check_disk(index, Some(campaign));
    let app = &mut root.app;
    let lines = app
        .world_page
        .footer_plain(Some(item), index, campaign, |frame, messages| {
            crate::world::TrackerUi::history_footer(
                &crate::trackers::Reader(&mut app.trackers),
                item.entry.id,
                frame,
                messages,
            );
        });
    let footer = footer::render(lines, cx);
    let body = render_reader(root, Host::World, item, index, campaign, window, cx);
    let mut pane = kit::Pane::new(("world-reader", item.entry.id.as_u128() as u64))
        .flush()
        .scroll(&root.pages.world.jump.scroll);
    for line in footer {
        pane = pane.footer(line);
    }
    pane.child(body).into_any_element()
}

/// Whose note reader a view draws (egui: one `world::Reader` each): the World page's, or
/// Lookup's compact one beside any page.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Host {
    World,
    Lookup,
}

impl Host {
    /// The host's reader.
    pub(crate) fn reader(self, root: &mut SplatRoot) -> &mut crate::world::Reader {
        match self {
            Host::World => &mut root.app.world_page.reader,
            Host::Lookup => &mut root.app.lookup.reader,
        }
    }

    /// Go where a reader's act or link leads: World's page opens the note (or asks Lookup for a
    /// missing one), Lookup shows it.
    pub(crate) fn take_nav(self, root: &mut SplatRoot, nav: Nav, index: &ContentIndex) {
        match self {
            Host::World => root.app.world_page.take_nav(nav, index),
            Host::Lookup => root.app.lookup.follow_nav(nav),
        }
    }

    /// The host's scroll and its heading jump.
    pub(crate) fn jump(self, root: &mut SplatRoot) -> &mut Jump {
        match self {
            Host::World => &mut root.pages.world.jump,
            Host::Lookup => &mut root.pages.lookup.jump,
        }
    }
}

/// A reader's scroll column and the heading a link asked it to show (egui: `want_heading` for
/// one draw).
#[derive(Default)]
pub(crate) struct Jump {
    pub scroll: gpui::ScrollHandle,
    /// The heading asked for in the last frame: its request ends in the next one, found or not,
    /// so it cannot reach a heading on another page.
    pending: Option<String>,
}

/// The reader's body for the note `item`, in `host`'s reader.
pub(crate) fn render_reader(
    root: &mut SplatRoot,
    host: Host,
    item: &Item,
    index: &ContentIndex,
    campaign: &Campaign,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = theme::colors();
    let id = item.entry.id;
    // A followed link's heading: the text scrolls to it in this frame (`read_in_pane`).
    let heading = host.reader(root).take_heading_for(id);
    let ctx = root.ctx.clone();
    let jump = host.jump(root);
    if let Some(old) = jump.pending.take() {
        let _ = crate::markdown_view::wanted_heading(&ctx, &old);
    }
    if let Some(heading) = heading {
        crate::markdown_view::want_heading(&ctx, Some(heading.clone()));
        jump.pending = Some(heading);
    }
    let scroll = jump.scroll.clone();
    let compact = host.reader(root).compact;
    let reader = &mut host.reader(root);
    let mut column = div().flex().flex_col().w_full().pb(theme::L);
    // A link to several notes waits for the DM's choice, above the text.
    if !reader.link_choices().is_empty() {
        column = column.child(kit::text(
            theme::Type::Small,
            c.muted,
            "This link matches more than one note. Choose one.",
        ));
        for &position in reader.link_choices() {
            column = column.child(
                kit::Link::new(
                    ("world-many", position),
                    index.items[position].entry.path.clone(),
                )
                .wrap()
                .on_click(act(cx, move |root, _, index, _, _| {
                    if let Some(nav) = host.reader(root).choose_many(index, position) {
                        host.take_nav(root, nav, index);
                    }
                })),
            );
        }
    }
    column = column.child(title_line(root, host, item, index, campaign, cx));
    if let Some(log) = log_field(root, host, item, index, campaign, cx) {
        column = column.child(log);
    }
    if host.reader(root).editing(id, campaign) {
        return column
            .child(super::panes::edit_form(
                root, host, item, index, campaign, cx,
            ))
            .into_any_element();
    }
    let facts = host.reader(root).body_facts(item, index, Some(campaign));
    column = column.child(faults(&facts, host, item, campaign, cx));
    // A value block that draws no row adds no gap: in World, the prose then starts where a
    // plain note's starts. Lookup's compact reader keeps the reader's gap.
    let mut empty_value_block = false;
    if facts.values {
        empty_value_block = !compact && host.reader(root).value_rows(item, index).is_empty();
        column = column.child(values(root, host, item, index, campaign, window, cx));
    }
    if facts.duplicate_values {
        column = column.child(duplicate_values(host, item, index, cx));
    }
    if (item.entry.kind == "world" || item.is_world_note()) && !facts.has_prose {
        column = column.child(div().pt(theme::L).child(kit::text(
            theme::Type::Small,
            c.muted,
            "Nothing written yet.",
        )));
        empty_value_block = false;
    }
    if facts.plain_note {
        column = column.child(note_text(host, item, index, &facts, compact, &scroll, cx));
        column = column.child(related(host, item, index, cx));
        if facts.make_type {
            column = column.child(make_type(root, host, item, cx));
        }
    } else {
        let history = super::super::trackers::history::render(root, &item.entry, window, cx);
        let related = div()
            .flex()
            .flex_col()
            .children(history)
            .child(related(host, item, index, cx));
        let entry = host.reader(root).read_entry(item).clone();
        let links = match root.app.lookup.index() {
            Some(index) => crate::gui::pages::reader::entry::index_links(index),
            None => std::rc::Rc::new(crate::library::view::NoLinks),
        };
        column = column.child(entry_text(
            host,
            item,
            &entry,
            links,
            empty_value_block,
            related,
            &scroll,
            cx,
        ));
    }
    column.into_any_element()
}

#[allow(clippy::too_many_arguments)]
/// An entry's text as the reader shows every entry (egui `view::show_with_related`): its stat
/// block or facts, its sections, then the tracker history of a house rule note in play and what
/// other notes say about it, then the closed end sections. Its related, feature and spell links
/// open their notes here; a note link in its words follows from this note; a dice number rolls.
fn entry_text(
    host: Host,
    item: &Item,
    entry: &splat::content::Entry,
    links: std::rc::Rc<dyn crate::library::view::Links>,
    empty_value_block: bool,
    related: impl IntoElement,
    scroll: &gpui::ScrollHandle,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let id = item.entry.id;
    let go = cx.listener(move |root: &mut SplatRoot, at: &usize, _, cx| {
        if let Some((_, index)) = super::world(root) {
            let reader = host.reader(root);
            if let Some(nav) = reader.go_to(*at) {
                host.take_nav(root, nav, &index);
            }
        }
        cx.notify();
    });
    let follow = cx.listener(move |root: &mut SplatRoot, note: &str, _, cx| {
        if let Some((_, index)) = super::world(root)
            && let Some(position) = index.position(id)
        {
            let reader = host.reader(root);
            if let Some(nav) = reader.follow(&index.items[position], &index, note) {
                host.take_nav(root, nav, &index);
            }
        }
        cx.notify();
    });
    crate::gui::pages::reader::entry::EntryView::new(
        ("world-entry", id.as_u128() as u64),
        item.entry.name.clone(),
        Some(entry),
        item.entry.body.clone(),
        crate::gui::pages::reader::entry::Source {
            path: item.entry.path.clone(),
            owner: item.owner.clone(),
            builtin: item.is_builtin(),
        },
        links,
    )
    .empty_value_block(empty_value_block)
    .related(related)
    .on_open(move |at, window, cx| go(&at, window, cx))
    .on_note(follow)
    .jump_in(scroll)
    .into_any_element()
}

/// The title, its kind line, and its actions on its line (Edit, Start fight or Add to fight,
/// Add to Log). When they do not fit beside the title, they move under it together.
fn title_line(
    root: &mut SplatRoot,
    host: Host,
    item: &Item,
    index: &ContentIndex,
    campaign: &Campaign,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let reader = &host.reader(root);
    let facts = reader.title_facts(item, index, Some(campaign));
    let id = item.entry.id;
    let mut actions = Vec::new();
    let action = |cx: &mut Context<SplatRoot>, act: TitleAct| {
        on_item::<gpui::ClickEvent>(cx, host, id, move |page, item, index, campaign| {
            page.reader.press_title(act, item, index, Some(campaign));
            None
        })
    };
    if facts.edit {
        actions.push(
            kit::Button::quiet("world-edit", "Edit")
                .icon(icons::EDIT)
                .density(theme::Density::Read)
                .name(format!("Edit {}", item.entry.name))
                .on_click(action(cx, TitleAct::Edit))
                .into_any_element(),
        );
    }
    if let Some(start) = &facts.start {
        actions.push(
            kit::Button::quiet("world-start-fight", "Start fight")
                .icon(icons::RUN)
                .density(theme::Density::Read)
                .name(format!("Start a fight with {}", start.name))
                .blocked(reader.start_blocked)
                .on_click(action(cx, TitleAct::StartFight))
                .into_any_element(),
        );
    }
    if let Some(add) = &facts.add {
        actions.push(
            kit::Button::quiet("world-add-to-fight", "Add to fight")
                .icon(icons::ADD)
                .density(theme::Density::Read)
                .name(format!("Add {} to the fight", add.name))
                .on_click(action(cx, TitleAct::AddToFight))
                .into_any_element(),
        );
    }
    if facts.edit {
        actions.push(
            kit::Button::quiet("world-add-to-log", "Add to Log")
                .icon(icons::ADD_TO_LOG)
                .density(theme::Density::Read)
                .on_click(action(cx, TitleAct::AddToLog))
                .into_any_element(),
        );
    }
    div()
        .flex()
        .flex_row()
        .flex_wrap()
        .items_start()
        .gap_x(theme::L)
        .child(
            // The title and kind line, with the note's picture beside them when it has one.
            div().max_w(theme::LIST_MEASURE).child(
                crate::gui::pages::reader::entry::title_with_art(
                    &facts.title,
                    &facts.kind_line,
                    Some(&item.entry),
                    cx,
                ),
            ),
        )
        .child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(theme::XS)
                .children(actions),
        )
        .into_any_element()
}

/// `Add to Log`'s field under the title, while it is open: the stamp, then the field. Enter adds
/// the line, Escape closes it.
fn log_field(
    root: &mut SplatRoot,
    host: Host,
    item: &Item,
    index: &ContentIndex,
    campaign: &Campaign,
    cx: &mut Context<SplatRoot>,
) -> Option<AnyElement> {
    let c = theme::colors();
    let reader = &mut host.reader(root);
    let view = reader.log_view(item, index, campaign)?;
    let (campaign_id, id) = (campaign.id, item.entry.id);
    if view.select {
        reader.log_selected(campaign_id, id);
    }
    let enter = on_key(cx, move |root, campaign, index, _, _| {
        if let Some(at) = index.position(id)
            && let Some(nav) = host.reader(root).log_enter(&index.items[at], campaign)
        {
            host.take_nav(root, nav, index);
        }
    });
    let escape = on_key(cx, move |root, _, _, _, _| {
        host.reader(root).log_cancel(campaign_id, id);
    });
    let field = kit::TextField::new(("world-log", id.as_u128() as u64), view.words)
        .placeholder("What happened?")
        .name(view.name)
        .focus_selected(view.select)
        .on_change(on_words(cx, move |root, words| {
            host.reader(root).set_log_words(campaign_id, id, words);
        }))
        .on_enter(enter)
        .on_escape(escape);
    let add = kit::Button::secondary("world-log-add", "Add")
        .density(theme::Density::Read)
        .on_click(on_item(cx, host, id, |page, item, _, campaign| {
            page.reader.log_enter(item, campaign)
        }));
    let cancel = kit::Button::quiet("world-log-cancel", "Cancel")
        .density(theme::Density::Read)
        .on_click(cx.listener(move |root: &mut SplatRoot, _, _, cx| {
            host.reader(root).log_cancel(campaign_id, id);
            cx.notify();
        }));
    Some(
        div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(theme::S)
            .pt(theme::S)
            .max_w(theme::READER_MEASURE)
            .when_some(view.stamp, |line, stamp| {
                line.child(kit::text(theme::Type::Small, c.muted, stamp))
            })
            .child(div().flex_1().min_w(px(160.0)).child(field))
            .child(add)
            .child(cancel)
            .into_any_element(),
    )
}

/// The file line, then every warning and repair of the note, at the reader measure.
fn faults(
    facts: &BodyFacts,
    host: Host,
    item: &Item,
    campaign: &Campaign,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = theme::colors();
    let id = item.entry.id;
    let mut column = div()
        .flex()
        .flex_col()
        .gap(theme::S)
        .max_w(theme::READER_MEASURE);
    if let Some(line) = &facts.file_line {
        let path = campaign.path.join(&line.path);
        let whole = path.display().to_string();
        let edit = obsidian_link(&path);
        let show = file_link(&path);
        column = column.child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap_x(theme::S)
                .pt(theme::XS)
                .child(
                    kit::text(theme::Type::Small, c.muted, line.path.clone())
                        .when(!line.shared_name, |words| words.truncate()),
                )
                .when(line.editor, |links| {
                    links.child(
                        kit::Link::new("world-file-edit", "Edit in Obsidian")
                            .hint(whole.clone())
                            .on_click(move |_, _, cx| cx.open_url(&edit)),
                    )
                })
                .child(
                    kit::Link::new("world-file-show", "Show file")
                        .hint(whole)
                        .on_click(move |_, _, cx| cx.open_url(&show)),
                ),
        );
    }
    if let Some((reason, whole)) = &facts.unread {
        let mut message = kit::Message::new(kit::Tone::Warning, reason.clone());
        if facts.file_line.is_none() {
            let url = obsidian_link(&campaign.path.join(&item.entry.path));
            message = message.action(
                kit::Button::secondary("world-unread-edit", "Edit in Obsidian")
                    .density(theme::Density::Read)
                    .on_click(move |_, _, cx| cx.open_url(&url)),
            );
        }
        column = column.child(with_hint("world-unread", message, whole));
    }
    for (at, (notice, whole)) in facts.notices.iter().enumerate() {
        column = column.child(with_hint(
            ("world-notice", at),
            kit::Message::new(kit::Tone::Warning, notice.clone()),
            whole,
        ));
    }
    if let Some((stem, file)) = &facts.rename {
        let mut message = kit::Message::new(
            kit::Tone::Warning,
            format!(
                "The file is {stem}.md, but the name is {}.",
                item.entry.name
            ),
        );
        let to_note = stem.clone();
        message = message.action(
            kit::Button::secondary("world-rename-note", format!("Rename the note to {stem}"))
                .density(theme::Density::Read)
                .on_click(on_item(cx, host, id, move |page, item, _, campaign| {
                    page.reader.rename_note(campaign, item, &to_note)
                })),
        );
        if let Some(to) = file {
            let name = to.rsplit('/').next().unwrap_or(to).to_owned();
            let to = to.clone();
            message = message.action(
                kit::Button::secondary("world-rename-file", format!("Rename the file to {name}"))
                    .density(theme::Density::Read)
                    .on_click(on_item(cx, host, id, move |page, item, _, campaign| {
                        page.reader.rename_file(campaign, item, &to)
                    })),
            );
        }
        column = column.child(message);
    }
    if !facts.duplicates.is_empty() {
        let mut block = div()
            .flex()
            .flex_col()
            .gap(theme::XS)
            .child(kit::Message::new(
                kit::Tone::Warning,
                crate::world::model::duplicate_words(),
            ));
        for (at, (path, words, help)) in facts.duplicates.iter().enumerate() {
            let path = path.clone();
            block = block.child(
                div()
                    .pl(theme::S + theme::ICON_SLOT)
                    .flex()
                    .flex_col()
                    .items_start()
                    .gap(theme::XS)
                    .child(
                        kit::Button::secondary(("world-separate", at), words.clone())
                            .density(theme::Density::Read)
                            .on_click(act(cx, move |root, campaign, index, _, _| {
                                if let Some(nav) = host.reader(root).separate(campaign, &path) {
                                    host.take_nav(root, nav, index);
                                }
                            })),
                    )
                    .when_some(help.clone(), |choice, help| choice.child(kit::help(help))),
            );
        }
        column = column.child(block.pb(theme::M));
    }
    for warning in &facts.warnings {
        column = column.child(kit::Message::new(kit::Tone::Warning, warning.clone()));
    }
    column.into_any_element()
}

/// A message with its whole words on hover.
fn with_hint(
    id: impl Into<gpui::ElementId>,
    message: kit::Message,
    whole: &str,
) -> impl IntoElement {
    use gpui::{InteractiveElement, StatefulInteractiveElement};
    div()
        .id(id.into())
        .tooltip(kit::tooltip::tip(SharedString::from(whole.to_owned())))
        .child(message)
}

/// The value rows (Stat block, Place, Part of, Status) with their Change, the open picker, the
/// place chooser, the Status field, the run's status and the join form.
fn values(
    root: &mut SplatRoot,
    host: Host,
    item: &Item,
    index: &ContentIndex,
    campaign: &Campaign,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = theme::colors();
    let id = item.entry.id;
    let reader = &mut host.reader(root);
    let rows = reader.value_rows(item, index);
    // One Change column (AI1): `L` after the widest value, which wraps in the column when it is
    // wider than the reader's value room (egui: `relationship_value_width`).
    let body = |words: &str| words_width(window, theme::Type::Body, words);
    let small = |words: &str| words_width(window, theme::Type::Small, words);
    let column_width = rows
        .iter()
        .map(|row| match &row.value {
            ValueShown::Link(_, name) | ValueShown::Words(name) => body(name),
            ValueShown::PlaceChoices(words) => body(&format!("{} {words}", icons::WARN)),
            ValueShown::Missing(name) => {
                body(&missing_text(name)) + small(crate::world::model::NOT_FOUND_WORDS)
            }
            ValueShown::NotSet => small("not set"),
            ValueShown::NotRead => small("not read"),
            ValueShown::Groups(groups) => groups
                .iter()
                .map(|(name, target)| match target {
                    Some(_) => body(name),
                    None => body(&missing_text(name)) + small(crate::world::model::NOT_FOUND_WORDS),
                })
                .fold(px(0.0), |a, b| a.max(b)),
        })
        .fold(px(0.0), |a, b| a.max(b))
        .min(VALUE_COLUMN)
        + px(1.0);
    let mut column = div().flex().flex_col().pt(theme::L);
    for row in rows {
        let key = row.key;
        let value: AnyElement = match row.value {
            ValueShown::Link(at, name) => kit::Link::new(("world-value", at), name)
                .wrap()
                .on_click(on_item(cx, host, id, move |page, _, _, _| {
                    page.reader.go_to(at)
                }))
                .into_any_element(),
            ValueShown::PlaceChoices(words) => div()
                .flex()
                .flex_row()
                .items_center()
                .gap(theme::XS)
                .child(kit::icon(icons::WARN, theme::Type::Body, c.result.warning))
                .child(
                    kit::Link::new("world-place-choices", words)
                        .wrap()
                        .on_click(cx.listener(move |root: &mut SplatRoot, _, _, cx| {
                            host.reader(root).toggle_place_choices();
                            cx.notify();
                        })),
                )
                .into_any_element(),
            ValueShown::Missing(name) => missing(&name).into_any_element(),
            ValueShown::NotSet => {
                kit::text(theme::Type::Small, c.muted, "not set").into_any_element()
            }
            ValueShown::NotRead => {
                kit::text(theme::Type::Small, c.result.warning, "not read").into_any_element()
            }
            ValueShown::Words(words) => {
                kit::text(theme::Type::Body, c.text, words).into_any_element()
            }
            ValueShown::Groups(groups) => div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap_x(theme::S)
                .children(groups.into_iter().enumerate().map(|(n, (name, target))| {
                    match target {
                        Some(at) => kit::Link::new(("world-group", n), name)
                            .wrap()
                            .on_click(on_item(cx, host, id, move |page, _, _, _| {
                                page.reader.go_to(at)
                            }))
                            .into_any_element(),
                        None => missing(&name).into_any_element(),
                    }
                }))
                .into_any_element(),
        };
        let change = kit::Button::quiet(
            gpui::ElementId::Name(format!("world-change-{key}").into()),
            "Change",
        )
        .name(format!("Change {}", row.label))
        .blocked(row.blocked)
        .on_click(on_item(cx, host, id, move |page, item, _, _| {
            page.reader.press_change(key, item);
            None
        }));
        column = column.child(value_row(
            row.label,
            value,
            column_width,
            Some(change.into_any_element()),
        ));
        if key == "in" && host.reader(root).place_choices_open() {
            column = column.child(place_chooser(host, item, index, cx));
        }
    }
    let reader = &mut host.reader(root);
    if reader.picker_view().is_some() {
        column = column.child(picker(root, host, item, index, false, cx));
    }
    column = column.child(status_field(root, host, item, cx));
    if crate::world::fights(&item.entry) {
        let reader = &host.reader(root);
        if let Some((arrival, initiative)) = reader.in_run_of(id) {
            let words = format!(
                "In the run · initiative {}",
                initiative.map_or_else(|| "unknown".to_owned(), |n| n.to_string())
            );
            column = column.child(
                kit::status_link(
                    "world-in-run",
                    kit::Status::new(icons::RUN, words, c.accent),
                )
                .on_click(cx.listener(move |root: &mut SplatRoot, _, _, cx| {
                    host.reader(root).press_in_run(arrival);
                    cx.notify();
                })),
            );
        } else if let Some(join) = join_form(root, host, item, index, campaign, cx) {
            column = column.child(join);
        }
    }
    column.into_any_element()
}

/// A missing value: its name with the warning mark, then ` · Not found`, in the warning colour.
fn missing(name: &str) -> impl IntoElement {
    let warning = theme::colors().result.warning;
    div()
        .flex()
        .flex_row()
        .flex_wrap()
        .items_baseline()
        .child(kit::text(theme::Type::Body, warning, missing_text(name)))
        .child(kit::text(
            theme::Type::Small,
            warning,
            crate::world::model::NOT_FOUND_WORDS,
        ))
}

/// A labelled value: the muted label column, the value, then its Change `L` after it.
fn value_row(
    label: &str,
    value: AnyElement,
    width: gpui::Pixels,
    change: Option<AnyElement>,
) -> impl IntoElement {
    let c = theme::colors();
    div()
        .flex()
        .flex_row()
        .items_center()
        .min_h(theme::Density::Scan.row())
        .child(div().w(px(LABEL_COLUMN)).flex_none().child(kit::text(
            theme::Type::Small,
            c.muted,
            label.to_owned(),
        )))
        .child(div().flex_none().w(width).child(value))
        .when_some(change, |row, change| {
            row.child(div().pl(theme::L).child(change))
        })
}

/// The place link's chooser: each file it matches, with `Always use this one`.
fn place_chooser(
    host: Host,
    item: &Item,
    index: &ContentIndex,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = theme::colors();
    let id = item.entry.id;
    let mut column = div()
        .flex()
        .flex_col()
        .gap(theme::XS)
        .pl(px(LABEL_COLUMN))
        .child(kit::text(
            theme::Type::Small,
            c.muted,
            "This link matches more than one note. Choose one.",
        ));
    for at in crate::world::model::place_choices(index, id) {
        let path = index.items[at].entry.path.clone();
        column = column.child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(theme::L)
                .child(
                    kit::Link::new(("world-place-file", at), path.clone())
                        .wrap()
                        .on_click(on_item(cx, host, id, move |page, _, _, _| {
                            page.reader.go_to(at)
                        })),
                )
                .child(
                    kit::Button::secondary(("world-place-use", at), "Always use this one")
                        .density(theme::Density::Read)
                        .name(format!("Always use {path}"))
                        .on_click(on_item(cx, host, id, move |page, item, index, campaign| {
                            page.reader.choose_place(campaign, item, index, at)
                        })),
                ),
        );
    }
    column.into_any_element()
}

/// The open picker of a value: its search field, its matches, Part of's removes, a new
/// location's form, `Clear value` and `Cancel`. `in_form`: the edit form's picker, whose choice
/// goes to the form (its groups), not to the note.
pub(super) fn picker(
    root: &mut SplatRoot,
    host: Host,
    item: &Item,
    index: &ContentIndex,
    in_form: bool,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let reader = &mut host.reader(root);
    let groups = in_form
        .then(|| reader.draft_groups(item.entry.id))
        .flatten();
    let draft = groups.as_deref();
    let Some(view) = reader.picker_view() else {
        return div().into_any_element();
    };
    if view.focus {
        reader.picker_focused();
    }
    let (matches, removes) = reader.picker_matches(item, index, draft);
    let id = item.entry.id;
    let pick = move |cx: &mut Context<SplatRoot>, pick: PickAct| {
        on_item::<gpui::ClickEvent>(cx, host, id, move |page, item, index, campaign| {
            if in_form {
                page.reader
                    .pick_into_draft(pick.clone(), item, index, campaign);
                None
            } else {
                page.reader
                    .pick_and_change(pick.clone(), item, index, campaign)
            }
        })
    };
    let mut column = div()
        .flex()
        .flex_col()
        .gap(theme::XS)
        .py(theme::S)
        .max_w(theme::FORM_WIDTH)
        .child(
            kit::TextField::new(
                gpui::ElementId::Name(format!("world-picker-{}", view.key).into()),
                view.search,
            )
            .placeholder(view.label)
            .name(view.label)
            .focus_selected(view.focus)
            .on_change(on_words(cx, move |root, words| {
                host.reader(root).set_picker_search(words);
            }))
            .on_escape(on_key(cx, move |root, campaign, index, _, _| {
                if let Some(at) = index.position(id) {
                    let _ = host.reader(root).pick_value(
                        PickAct::Cancel,
                        &index.items[at],
                        index,
                        campaign,
                        None,
                    );
                }
            })),
        );
    for (at, name) in matches {
        column = column.child(
            kit::Link::new(("world-pick", at), name)
                .wrap()
                .on_click(pick(cx, PickAct::Match(at))),
        );
    }
    for (at, name) in removes.into_iter().enumerate() {
        column = column.child(
            kit::Button::quiet(("world-pick-remove", at), name)
                .line_start()
                .on_click(pick(cx, PickAct::Remove(at))),
        );
    }
    if view.key == "in" {
        let reason = view
            .new_location
            .trim()
            .is_empty()
            .then_some("Type a name first.");
        column = column.child(
            kit::QuietSection::new(("world-new-location", id.as_u128() as u64), "New location…")
                .child(kit::field(
                    "Location name",
                    None,
                    None,
                    kit::TextField::new(
                        ("world-new-location-name", id.as_u128() as u64),
                        view.new_location,
                    )
                    .name("Location name")
                    .on_change(on_words(cx, move |root, words| {
                        host.reader(root).set_new_location(words);
                    })),
                ))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(theme::S)
                        .child(
                            kit::Button::secondary("world-create-location", "Create location here")
                                .blocked(reason)
                                .on_click(pick(cx, PickAct::CreateLocation)),
                        )
                        .when_some(reason, |line, reason| {
                            line.child(kit::text(theme::Type::Small, theme::colors().muted, reason))
                        }),
                ),
        );
    }
    let mut actions = div()
        .flex()
        .flex_row()
        .flex_wrap()
        .items_center()
        .gap(theme::S);
    if view.key != "part_of" {
        actions = actions.child(
            kit::Button::quiet("world-pick-clear", "Clear value")
                .line_start()
                .on_click(pick(cx, PickAct::Clear)),
        );
    }
    let cancel =
        kit::Button::quiet("world-pick-cancel", "Cancel").on_click(pick(cx, PickAct::Cancel));
    actions = actions.child(if view.key == "part_of" {
        cancel.line_start()
    } else {
        cancel
    });
    column.child(actions).into_any_element()
}

/// The Status field while it is open: its words, and while they differ from the note's, the
/// draft's `Unsaved` and `Save status`. Enter saves.
fn status_field(
    root: &mut SplatRoot,
    host: Host,
    item: &Item,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let reader = &mut host.reader(root);
    let Some(words) = reader.status_field().map(str::to_owned) else {
        return div().into_any_element();
    };
    let select = reader.take_status_select();
    let unsaved = reader.status_unsaved(item);
    let id = item.entry.id;
    let save = move |cx: &mut Context<SplatRoot>| {
        on_item::<gpui::ClickEvent>(cx, host, id, |page, item, index, campaign| {
            page.reader.save_status(item, index, campaign)
        })
    };
    let enter = save(cx);
    let field = kit::TextField::new(("world-status", id.as_u128() as u64), words)
        .name("Status")
        .focus_selected(select)
        .on_change(on_words(cx, move |root, words| {
            host.reader(root).set_status_text(words);
        }))
        .on_enter(move |window, cx| enter(&gpui::ClickEvent::default(), window, cx));
    let mut block = div()
        .flex()
        .flex_col()
        .max_w(theme::FORM_WIDTH)
        .child(kit::field("Status", None, None, field));
    if unsaved {
        let c = theme::colors();
        block = block.child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(theme::S)
                .child(kit::status(&kit::Status::new(
                    icons::DRAFT,
                    "Unsaved",
                    c.document.unsaved,
                )))
                .child(
                    kit::Button::secondary("world-save-status", "Save status")
                        .density(theme::Density::Read)
                        .on_click(save(cx)),
                ),
        );
    }
    block.into_any_element()
}

/// The form that adds an NPC with no stat block to the run.
fn join_form(
    root: &mut SplatRoot,
    host: Host,
    item: &Item,
    index: &ContentIndex,
    campaign: &Campaign,
    cx: &mut Context<SplatRoot>,
) -> Option<AnyElement> {
    let c = theme::colors();
    let view = host.reader(root).join_view(item, index, Some(campaign))?;
    let id = item.entry.id;
    let join = move |cx: &mut Context<SplatRoot>, act: JoinAct| {
        on_item::<gpui::ClickEvent>(cx, host, id, move |page, item, index, campaign| {
            page.reader
                .join_act(act.clone(), item, index, Some(campaign))
        })
    };
    let words = move |cx: &mut Context<SplatRoot>, make: fn(String) -> JoinAct| {
        on_words(cx, move |root, words| {
            if let Some((campaign, index)) = super::world(root)
                && let Some(at) = index.position(id)
            {
                let _ = host.reader(root).join_act(
                    make(words.to_owned()),
                    &index.items[at],
                    &index,
                    Some(&campaign),
                );
            }
        })
    };
    let mut form = div()
        .flex()
        .flex_col()
        .gap(theme::S)
        .pt(theme::M)
        .max_w(theme::FORM_WIDTH)
        .child(kit::text(theme::Type::Strong, c.text, view.title));
    let chosen_none = view.stat_block.is_none();
    match view.stat_block {
        Some((_, name)) => {
            form = form.child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(theme::S)
                    .child(kit::text(theme::Type::Small, c.muted, "Stat block"))
                    .child(kit::text(theme::Type::Body, c.text, name))
                    .child(
                        kit::Button::quiet("world-join-change", "Change")
                            .density(theme::Density::Read)
                            .on_click(join(cx, JoinAct::ChangeStatBlock)),
                    ),
            );
        }
        None => {
            form = form.child(
                kit::TextField::new(("world-join-search", id.as_u128() as u64), view.search)
                    .placeholder("Stat block: Commoner, Guard…")
                    .name("Stat block")
                    .on_change(words(cx, JoinAct::Search)),
            );
            for (at, name) in view.matches {
                form = form.child(
                    kit::Link::new(("world-join-match", at), name)
                        .wrap()
                        .on_click(join(cx, JoinAct::Choose(at))),
                );
            }
        }
    }
    if let Some(save) = view.save {
        form = form.child(
            kit::Checkbox::new("world-join-save", "Save this as their stat block", save)
                .on_click(join(cx, JoinAct::Save(!save))),
        );
    }
    if view.sides.len() > 1 {
        let chosen = view.sides.iter().position(|side| *side == view.side);
        let sides = view.sides.clone();
        form = form.child(kit::field(
            "Side",
            None,
            None,
            kit::Dropdown::new(
                "world-join-side",
                "Side",
                view.sides
                    .iter()
                    .map(|side| SharedString::from(crate::design::reader::human_label(side)))
                    .collect::<Vec<SharedString>>(),
                chosen,
            )
            .on_select(cx.listener(move |root: &mut SplatRoot, at: &usize, _, cx| {
                if let (Some(side), Some((campaign, index))) = (sides.get(*at), super::world(root))
                    && let Some(position) = index.position(id)
                {
                    let _ = host.reader(root).join_act(
                        JoinAct::Side(side.clone()),
                        &index.items[position],
                        &index,
                        Some(&campaign),
                    );
                }
                cx.notify();
            })),
        ));
    }
    form = form.child(kit::field(
        "Initiative",
        Some("Leave empty to roll it.".into()),
        None,
        kit::TextField::new(
            ("world-join-initiative", id.as_u128() as u64),
            view.initiative,
        )
        .name("Initiative")
        .width(theme::L * 5.0)
        .on_change(words(cx, JoinAct::Initiative)),
    ));
    let blocked = chosen_none.then_some("Choose a stat block first.");
    form = form.child(
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(theme::S)
            .child(
                kit::Button::secondary("world-join-add", "Add to fight")
                    .density(theme::Density::Read)
                    .blocked(blocked)
                    .on_click(join(cx, JoinAct::Add)),
            )
            .child(
                kit::Button::quiet("world-join-cancel", "Cancel")
                    .density(theme::Density::Read)
                    .on_click(join(cx, JoinAct::Cancel)),
            ),
    );
    Some(form.into_any_element())
}

/// A duplicate note's values, read only.
fn duplicate_values(
    host: Host,
    item: &Item,
    index: &ContentIndex,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = theme::colors();
    let id = item.entry.id;
    let mut column = div().flex().flex_col();
    for (label, name, target) in crate::world::Reader::duplicate_rows(item, index) {
        let value = match target {
            Some(at) => kit::Link::new(("world-dup-value", at), name)
                .wrap()
                .on_click(on_item(cx, host, id, move |page, _, _, _| {
                    page.reader.go_to(at)
                }))
                .into_any_element(),
            None if label == "Status" => {
                kit::text(theme::Type::Body, c.text, name).into_any_element()
            }
            None => missing(&name).into_any_element(),
        };
        column = column.child(value_row(label, value, VALUE_COLUMN, None));
    }
    column.into_any_element()
}

/// A plain note's text, by its sections (the first heading is the title), at the reader
/// measure (egui `show_document`). A note link follows from this note (a link to several notes
/// waits for the DM's choice); a web link opens, a bold name goes to Lookup, a dice number rolls;
/// a followed link's heading scrolls into view.
fn note_text(
    host: Host,
    item: &Item,
    index: &ContentIndex,
    facts: &BodyFacts,
    compact: bool,
    scroll: &gpui::ScrollHandle,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let id = item.entry.id;
    let ctx = cx
        .try_global::<kit::markdown::Host>()
        .map(|host| host.0.clone());
    let body = crate::world::with_named_links(
        &item.entry.body,
        &std::collections::HashMap::new(),
        crate::world::model::session_names(index),
    );
    let sections = crate::library::presentation::reading_sections(&body, &item.entry.name);
    // In Lookup, a plain note's prose has the reader's gap below the kind line, as every
    // entry's has. World's follows its file line.
    let mut column = div()
        .flex()
        .flex_col()
        .gap(theme::S)
        .pt(if compact && facts.has_prose {
            theme::L
        } else {
            theme::S
        });
    for (at, section) in sections.iter().enumerate() {
        if section.title != "Overview" {
            // A followed link's heading: the column scrolls it to its top.
            if let Some(ctx) = &ctx
                && crate::markdown_view::wanted_heading(ctx, &section.title)
            {
                column = column.child(kit::markdown::jump_here(scroll.clone()));
            }
            column = column.child(kit::heading(kit::Heading::Section(&section.title), false));
        }
        column = column.child(
            kit::markdown::Markdown::new(("world-text", at), section.body.to_owned())
                .document(id)
                .jump_in(scroll)
                .on_note(cx.listener(move |root: &mut SplatRoot, note: &str, _, cx| {
                    if let Some((_, index)) = super::world(root)
                        && let Some(position) = index.position(id)
                        && let Some(nav) =
                            host.reader(root)
                                .follow(&index.items[position], &index, note)
                    {
                        host.take_nav(root, nav, &index);
                    }
                    cx.notify();
                })),
        );
    }
    column.into_any_element()
}

/// What is here, its members, its history, and the lines other notes hold about it.
fn related(
    host: Host,
    item: &Item,
    index: &ContentIndex,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = theme::colors();
    let id = item.entry.id;
    let facts = crate::world::related::related_facts(item, index);
    let mut column = div().flex().flex_col().max_w(theme::READER_MEASURE);
    let page_row = |cx: &mut Context<SplatRoot>, page: crate::world::related::PageFacts| {
        let at = page.position;
        div()
            .flex()
            .flex_col()
            .child(
                kit::Link::new(("world-related", at), page.name)
                    .wrap()
                    .on_click(on_item(cx, host, id, move |page, _, _, _| {
                        page.reader.go_to(at)
                    })),
            )
            .when(!page.detail.is_empty(), |row| {
                row.child(kit::text(theme::Type::Small, c.muted, page.detail))
            })
    };
    if !facts.here.is_empty() {
        column = column.child(kit::heading(kit::Heading::Section("Here"), false));
        for page in facts.here {
            column = column.child(page_row(cx, page));
        }
    }
    if !facts.members.is_empty() {
        column = column.child(kit::heading(kit::Heading::Section("Members"), false));
        for (title, pages) in facts.members {
            if let Some(title) = title {
                column = column.child(kit::heading(
                    kit::Heading::Group {
                        title: &title,
                        count: pages.len(),
                    },
                    false,
                ));
            }
            for page in pages {
                column = column.child(page_row(cx, page));
            }
        }
    }
    if !facts.history.is_empty() {
        column = column.child(kit::heading(kit::Heading::Section("History"), false));
        for (date, page) in facts.history {
            column = column.child(
                div()
                    .flex()
                    .flex_row()
                    .items_start()
                    .gap(theme::S)
                    .child(
                        div()
                            .w(px(120.0))
                            .flex_none()
                            .pt(theme::XS)
                            .child(kit::text(
                                theme::Type::Small,
                                c.muted,
                                date.unwrap_or_default(),
                            )),
                    )
                    .child(page_row(cx, page)),
            );
        }
    }
    if !facts.from.is_empty() || !facts.also.is_empty() {
        column = column.child(kit::heading(
            kit::Heading::Section("From other notes"),
            false,
        ));
        column = column.child(kit::rule());
    }
    for (group_at, group) in facts.from.into_iter().enumerate() {
        let mut section = kit::QuietSection::new(
            ("world-from", group_at),
            format!("{} {}", group.title.to_uppercase(), group.count),
        );
        for (source, name, lines, more) in group.pages {
            section = section.child(
                kit::Link::new(("world-from-page", source), name)
                    .wrap()
                    .on_click(on_item(cx, host, id, move |page, _, _, _| {
                        page.reader.go_to(source)
                    })),
            );
            for (line_at, line) in lines.into_iter().enumerate() {
                let heading = line.heading.clone();
                section = section.child(
                    div()
                        .id(("world-from-line", source * 8 + line_at))
                        .cursor_pointer()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .gap(theme::XS)
                        .when(!line.label.is_empty(), |words| {
                            words.child(kit::text(theme::Type::Small, c.text, line.label.clone()))
                        })
                        .child(kit::text(theme::Type::Small, c.text, line.text))
                        .on_click(on_item(cx, host, id, move |page, _, index, _| {
                            page.reader.show_heading(heading.clone());
                            page.take_nav(Nav::Item(source), index);
                            None
                        })),
                );
            }
            if more > 0 {
                section = section.child(kit::text(
                    theme::Type::Small,
                    c.muted,
                    crate::design::count(more, "more line", "more lines"),
                ));
            }
        }
        column = column.child(section.default_open(group.open));
    }
    if !facts.also.is_empty() {
        let mut line = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap_x(theme::S)
            .child(kit::text(theme::Type::Small, c.muted, "Also linked from"));
        for (source, name) in facts.also {
            line = line.child(
                kit::Link::new(("world-also", source), name)
                    .wrap()
                    .on_click(on_item(cx, host, id, move |page, _, _, _| {
                        page.reader.go_to(source)
                    })),
            );
        }
        column = column.child(line);
    }
    column.into_any_element()
}

/// `Give this note a type`: Make NPC, Make location, or a type of the DM's own.
fn make_type(
    root: &mut SplatRoot,
    host: Host,
    item: &Item,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let id = item.entry.id;
    let other = host.reader(root).other_type().trim().to_lowercase();
    let make = move |cx: &mut Context<SplatRoot>, kind: String| {
        on_item::<gpui::ClickEvent>(cx, host, id, move |page, item, _, campaign| {
            page.reader.make_type(campaign, item, &kind)
        })
    };
    let mut section = kit::QuietSection::new(
        ("world-make-type", id.as_u128() as u64),
        "Give this note a type",
    )
    .child(kit::help(crate::world::TYPE_HELP))
    .child(
        div()
            .flex()
            .flex_row()
            .flex_wrap()
            .gap(theme::S)
            .child(
                kit::Button::quiet("world-make-npc", "Make NPC")
                    .density(theme::Density::Read)
                    .line_start()
                    .on_click(make(cx, "npc".to_owned())),
            )
            .child(
                kit::Button::quiet("world-make-location", "Make location")
                    .density(theme::Density::Read)
                    .on_click(make(cx, "location".to_owned())),
            ),
    )
    .child(
        div().max_w(theme::FORM_WIDTH).child(kit::field(
            "Another type",
            Some("A word of your own, such as faction or quest.".into()),
            None,
            kit::TextField::new(
                ("world-other-type", id.as_u128() as u64),
                host.reader(root).other_type().to_owned(),
            )
            .name("Another type")
            .on_change(on_words(cx, move |root, words| {
                host.reader(root).set_other_type(words);
            })),
        )),
    );
    if !other.is_empty() {
        section = section.child(
            kit::Button::quiet("world-make-other", format!("Make {other}"))
                .density(theme::Density::Read)
                .line_start()
                .on_click(make(cx, other.clone())),
        );
    }
    section = section.child(
        kit::QuietSection::new(
            ("world-type-storage", id.as_u128() as u64),
            "Technical details",
        )
        .child(kit::help(crate::world::TYPE_STORAGE)),
    );
    div().pt(theme::M).child(section).into_any_element()
}

#[allow(dead_code, reason = "the colour a value takes, for a check")]
fn value_color(value: &ValueShown) -> Hsla {
    let c = theme::colors();
    match value {
        ValueShown::NotRead | ValueShown::Missing(_) | ValueShown::PlaceChoices(_) => {
            c.result.warning
        }
        ValueShown::NotSet => c.muted,
        _ => c.text,
    }
}
