//! The one entry reader: the Library, Lookup, the run's side panel and
//! campaign entries call [`EntryView`], so an entry looks the same everywhere.
//!
//! ```ignore
//! use crate::gui::pages::reader::entry::{self, EntryView};
//! EntryView::new(("library-entry", entry.id), &entry.name, Some(&entry), &entry.body, origin, links)
//!     .on_open(cx.listener(|root, index: &usize, window, cx| …))   // a related, feature or spell link
//!     .jump_in(&state.scroll)
//! ```
//!
//! It draws, in the reader's measure: line warnings, a reading's summary, a creature's stat block
//! (identity, vitals, the ability table whose numbers roll, detail rows, rules prose with its
//! actions as tiles and its spells), or the common layout (an item's summary, a spell's facts,
//! the source's facts and prose, each section under its heading), a class's traits and features,
//! a reading's related entries, and then the closed end sections: Source text, Technical details,
//! Source and identifiers.
//!
//! A pressed dice number asks for its roll in the app's state context (`rolls::take_ask`, taken
//! by the run or the preview stream); a pressed note link goes to Lookup through the frame logic,
//! as the Markdown view's do. A followed related, feature or spell link calls `on_open` with its
//! place in `links`.
#![allow(
    dead_code,
    reason = "the Library, Lookup and Run pages call the entry view when their ports merge; tests call it now"
)]
use crate::state_context::StateContext;
mod details;
pub mod parts;
mod stat_block;
pub mod tiles;

use crate::gui::kit;
use crate::gui::theme::{L, M, READER_MEASURE, Type, XXL, colors};
use crate::library::view::{Links, Origin, RunValues};
use crate::library::{class, creature, presentation, reader};
use gpui::{
    AnyElement, App, ElementId, IntoElement, ObjectFit, ParentElement, RenderOnce, ScrollHandle,
    SharedString, Styled, StyledImage, Window, div, img, prelude::*,
};
use splat::content::{Entry, Reference};
use std::rc::Rc;
use std::sync::Arc;

/// A followed link's place in the reader's links.
pub type OnOpen = Rc<dyn Fn(usize, &mut Window, &mut App)>;

/// A pressed note link in the entry's words, with the note it names.
pub type OnNote = Rc<dyn Fn(&str, &mut Window, &mut App)>;

/// Where an entry comes from, owned.
#[derive(Clone, Debug, Default)]
pub struct Source {
    pub path: String,
    pub owner: String,
    pub builtin: bool,
}

impl From<&Origin<'_>> for Source {
    fn from(origin: &Origin<'_>) -> Self {
        Self {
            path: origin.path.to_owned(),
            owner: origin.owner.to_owned(),
            builtin: origin.builtin,
        }
    }
}

/// The links of a content index, for an [`EntryView`].
pub fn index_links(index: Arc<crate::lookup::ContentIndex>) -> Rc<dyn Links> {
    Rc::new(IndexLinks(index))
}

/// A content index as the reader's links.
struct IndexLinks(Arc<crate::lookup::ContentIndex>);

impl Links for IndexLinks {
    fn find(&self, reference: &Reference) -> Option<(usize, String)> {
        self.0.find(reference)
    }
    fn find_spell(&self, reference: &Reference) -> Option<(usize, String)> {
        self.0.find_spell(reference)
    }
    fn spells_named_in(&self, text: &str) -> Vec<(usize, String)> {
        self.0.spells_named_in(text)
    }
    fn spell_named(&self, name: &str) -> Option<(usize, String)> {
        self.0.spell_named(name)
    }
    fn entry(&self, index: usize) -> Option<&Entry> {
        self.0.entry(index)
    }
    fn spell_cache(&self) -> Option<&crate::library::spells::SpellCache> {
        self.0.spell_cache()
    }
}

/// An entry as the reader shows it.
#[derive(IntoElement)]
pub struct EntryView {
    id: ElementId,
    name: SharedString,
    entry: Option<Entry>,
    body: SharedString,
    source: Source,
    links: Rc<dyn Links>,
    empty_value_block: bool,
    values: Option<RunValues>,
    related: Option<AnyElement>,
    on_open: Option<OnOpen>,
    on_note: Option<OnNote>,
    jump: Option<ScrollHandle>,
    wide: bool,
}

impl EntryView {
    /// The entry `name`, its parsed `entry` (none for an unreadable file), the `body` to read
    /// (usually `entry.body`; a draft's words while one is open), where it comes from, and what
    /// its links can open (`index_links`, or `Rc::new(NoLinks)`).
    pub fn new(
        id: impl Into<ElementId>,
        name: impl Into<SharedString>,
        entry: Option<&Entry>,
        body: impl Into<SharedString>,
        source: impl Into<Source>,
        links: Rc<dyn Links>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            entry: entry.cloned(),
            body: body.into(),
            source: source.into(),
            links,
            empty_value_block: false,
            values: None,
            related: None,
            on_open: None,
            on_note: None,
            jump: None,
            wide: false,
        }
    }

    /// Let the entry's tables take the parent's whole width while its prose and stat block keep
    /// the reader's measure.
    pub fn wide_tables(mut self) -> Self {
        self.wide = true;
        self
    }

    /// The caller drew a value block above the entry, and that block drew no row: the prose
    /// starts without the reader's own gap. Line warnings keep the gap after them.
    pub fn empty_value_block(mut self, empty: bool) -> Self {
        self.empty_value_block = empty;
        self
    }

    /// Values from an open run: the vitals show them, and the printed values
    /// under them where they differ. The saved entry stays unchanged.
    pub fn run(mut self, values: RunValues) -> Self {
        self.values = Some(values);
        self
    }

    /// The caller's related entries, after the entry's own content.
    pub fn related(mut self, related: impl IntoElement) -> Self {
        self.related = Some(related.into_any_element());
        self
    }

    /// A followed related, feature or spell link, with its place in the links.
    pub fn on_open(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_open = Some(Rc::new(handler));
        self
    }

    /// A pressed note link in the entry's words goes to `handler`, not to Lookup. Web links, bold names and dice keep
    /// the reader's routing.
    pub fn on_note(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_note = Some(Rc::new(handler));
        self
    }

    /// The scroll column the entry is in: a typed jump to a section heading scrolls it there.
    pub fn jump_in(mut self, handle: &ScrollHandle) -> Self {
        self.jump = Some(handle.clone());
        self
    }

    /// The entry drawn now with links the caller borrows (the Library's own notes, which it does
    /// not share), in place of the links given to `new`.
    pub fn render_with(self, links: &dyn Links, window: &mut Window, cx: &mut App) -> AnyElement {
        self.build(links, window, cx).into_any_element()
    }
}

/// The app's state context, where rolls and note links are left for the frame logic.
fn host(cx: &App) -> Option<StateContext> {
    cx.try_global::<kit::markdown::Host>()
        .map(|host| host.0.clone())
}

/// The column an entry's parts go into, in order, and what drawing them needs.
pub(super) struct Out<'w> {
    pub id: ElementId,
    pub window: &'w Window,
    pub ctx: Option<StateContext>,
    pub document: Option<uuid::Uuid>,
    /// The entry's name: a roll pressed in its parts is kept under it (`rolls::set_entry` at the
    /// press, as the reader drawn last is not the one pressed).
    pub name: SharedString,
    pub on_open: Option<OnOpen>,
    pub on_note: Option<OnNote>,
    pub jump: Option<ScrollHandle>,
    /// Tables may pass the measure: every other part keeps it itself.
    pub wide: bool,
    items: Vec<AnyElement>,
    count: usize,
}

impl Out<'_> {
    /// A part at the reader's measure.
    pub fn push(&mut self, element: impl IntoElement) {
        let element = if self.wide {
            div()
                .w_full()
                .max_w(READER_MEASURE)
                .child(element)
                .into_any_element()
        } else {
            element.into_any_element()
        };
        self.items.push(element);
    }

    /// Markdown made by [`Out::markdown`], which keeps its own measure.
    pub fn push_prose(&mut self, element: AnyElement) {
        self.items.push(element);
    }

    /// Imported rules text with its run-in titles, as a part.
    pub fn prose(&mut self, text: &str) {
        let text = text
            .split("\n\n")
            .map(crate::design::reader::run_in_markdown)
            .collect::<Vec<_>>()
            .join("\n\n");
        let id = self.next_id("prose");
        let markdown = self.markdown(id, &text);
        self.push_prose(markdown);
    }

    /// A new child id for the next part named `name`.
    pub fn next_id(&mut self, name: &str) -> ElementId {
        self.count += 1;
        parts::child(&self.id, format!("{name}-{}", self.count))
    }

    /// A section heading (`XL` above it unless it is first), where a typed jump lands.
    pub fn heading(&mut self, title: impl Into<SharedString>) {
        let title = title.into();
        if let (Some(handle), Some(ctx)) = (&self.jump, &self.ctx)
            && crate::markdown_view::wanted_heading(ctx, &title)
        {
            self.items
                .push(kit::markdown::jump_here(handle.clone()).into_any_element());
        }
        let first = self.items.is_empty();
        self.push(parts::section_heading(title, first));
    }

    /// The Reader's Markdown of this entry.
    pub fn markdown(&self, id: ElementId, text: &str) -> AnyElement {
        let mut markdown = kit::Markdown::new(id, text.to_owned())
            .document(self.document)
            .entry(self.name.clone());
        if self.wide {
            markdown = markdown.wide_tables();
        }
        if let Some(handle) = &self.jump {
            markdown = markdown.jump_in(handle);
        }
        self.notes_to_caller(markdown).into_any_element()
    }

    /// The Reader's Markdown as compact field rows.
    pub fn compact_markdown(&self, id: ElementId, text: &str) -> AnyElement {
        let markdown = kit::Markdown::new(id, text.to_owned())
            .fields()
            .document(self.document)
            .entry(self.name.clone());
        self.notes_to_caller(markdown).into_any_element()
    }

    /// The caller's note links, when it takes them.
    fn notes_to_caller(&self, markdown: kit::Markdown) -> kit::Markdown {
        match self.on_note.clone() {
            Some(on_note) => markdown.on_note(move |note, window, cx| on_note(note, window, cx)),
            None => markdown,
        }
    }

    /// A link that opens the entry at `target` in the links.
    pub fn link(&self, name: String, words: String, target: usize) -> kit::Link {
        let on_open = self.on_open.clone();
        kit::Link::new(parts::child(&self.id, name), words).on_click(move |_, window, cx| {
            if let Some(on_open) = &on_open {
                on_open(target, window, cx);
            }
        })
    }
}

/// A stat block's spells: each spell its lists name is a tile under the list's label, and a line
/// after the lists has the spells the entry gives or its words name that no list showed.
pub(super) struct SpellLinks<'a> {
    entry: &'a Entry,
    resolved: Arc<crate::library::spells::Resolved>,
    links: &'a dyn Links,
    next_list: usize,
    next_cast: usize,
    shown: Vec<usize>,
}

impl RenderOnce for EntryView {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let links = self.links.clone();
        self.build(&*links, window, cx)
    }
}

impl EntryView {
    fn build(self, links: &dyn Links, window: &mut Window, cx: &mut App) -> gpui::Div {
        let ctx = host(cx);
        if let Some(ctx) = &ctx {
            crate::rolls::set_entry(ctx, &self.name);
        }
        let entry = self.entry.as_ref();
        let mut out = Out {
            id: self.id.clone(),
            window,
            ctx,
            document: entry.map(|entry| entry.id),
            name: self.name.clone(),
            on_open: self.on_open.clone(),
            on_note: self.on_note.clone(),
            jump: self.jump.clone(),
            wide: self.wide,
            items: Vec::new(),
            count: 0,
        };
        // The gap before the prose separates it from what is above. Line warnings need it; an
        // empty value block does not.
        let lead_gap =
            !self.empty_value_block || entry.is_some_and(|entry| !entry.flags.is_empty());
        if let Some(entry) = entry
            && !entry.flags.is_empty()
        {
            // One `L` after the kind line, also before a warning.
            out.push(kit::gap(L));
            for flag in &entry.flags {
                out.push(parts::item(kit::Message::new(
                    kit::Tone::Warning,
                    flag.message.clone(),
                )));
            }
        }
        let reading = entry.and_then(|entry| entry.reading().ok().flatten());
        let body: String = if let Some(entry) = entry.filter(|entry| creature::has_stats(entry)) {
            creature::without_vitals(entry, &self.body)
        } else if let Some(entry) =
            entry.filter(|entry| ["spell", "item"].contains(&entry.kind.as_str()))
        {
            splat::content::stat_blocks::reading_body(entry, &self.body)
        } else {
            self.body.to_string()
        };
        let sections: Vec<_> = presentation::reading_sections(&body, &self.name)
            .into_iter()
            .filter(|section| {
                if entry.is_some_and(|e| ["creature", "spell", "item"].contains(&e.kind.as_str()))
                    && presentation::overview_markdown(section.body)
                        .trim()
                        .is_empty()
                {
                    return false;
                }
                section.title != "Personal notes"
                    || section.body.trim() != "This section is kept when the sheet refreshes."
            })
            .collect();
        let styled =
            entry.filter(|entry| reading.is_none() && creature::supports(entry, &sections));
        content(
            &mut out,
            entry,
            reading.as_ref(),
            &sections,
            styled,
            self.source.builtin,
            links,
            self.values.as_ref(),
            lead_gap,
        );
        if let Some(related) = self.related {
            out.push(related);
        }
        if entry.is_some() {
            out.push(kit::gap(L));
            out.push(parts::rule());
            reference(&mut out, entry, &sections, styled.is_some(), &self.source);
        }
        // Rules text keeps its measure in a wide side panel too.
        div()
            .flex()
            .flex_col()
            .w_full()
            .when(!self.wide, |column| column.max_w(READER_MEASURE))
            .children(out.items)
    }
}

/// What the entry says. `styled` is the entry when it reads as a styled stat block. `lead_gap`
/// is false when nothing above needs a gap before the first prose.
#[allow(clippy::too_many_arguments)]
fn content(
    out: &mut Out<'_>,
    entry: Option<&Entry>,
    reading: Option<&splat::content::Reading>,
    sections: &[presentation::ReadingSection<'_>],
    styled: Option<&Entry>,
    builtin: bool,
    links: &dyn Links,
    values: Option<&RunValues>,
    lead_gap: bool,
) {
    if let Some(reading) = reading {
        if lead_gap {
            out.push(kit::gap(L));
        }
        let id = out.next_id("summary");
        let summary = out.markdown(id, &reading.summary);
        out.push_prose(summary);
    }
    if let Some(entry) = styled {
        let mut spells = SpellLinks {
            entry,
            resolved: crate::library::spells::resolve(entry, links),
            links,
            next_list: 0,
            next_cast: 0,
            shown: Vec::new(),
        };
        stat_block::show(out, entry, sections, values, &mut spells);
    } else {
        // A creature the DM wrote has no source text to style. Its numbers still lead; the
        // reader adds no lead gap after the stat head.
        let mut lead_gap = lead_gap;
        let mut after_stat_head = false;
        if let Some(entry) = entry.filter(|entry| reading.is_none() && creature::has_stats(entry)) {
            stat_block::stat_head(out, entry, values);
            lead_gap = false;
            after_stat_head = true;
        }
        details::show(
            out,
            entry,
            sections,
            builtin,
            reading.is_some(),
            lead_gap,
            after_stat_head,
        );
    }
    if let Some(entry) = entry.filter(|entry| class::lists_features(entry)) {
        details::class_features(out, entry, links);
    }
    if let Some(reading) = reading {
        reading_view(out, reading, links);
    }
}

/// A reading's related entries, each a link (or `unavailable`).
fn reading_view(out: &mut Out<'_>, reading: &splat::content::Reading, links: &dyn Links) {
    if reading.related.is_empty() {
        out.push(kit::gap(M));
        return;
    }
    out.heading(reading.related_heading.clone());
    for (at, (target, label)) in reading.related.iter().enumerate() {
        let reference = uuid::Uuid::parse_str(target)
            .map_or_else(|_| Reference::Key(target.clone()), Reference::Id);
        if let Some((index, _)) = links.find(&reference) {
            let link = out.link(format!("reading-{at}"), label.clone(), index);
            out.push(parts::item(link));
        } else {
            out.push(parts::item(kit::text(
                Type::Small,
                colors().muted,
                format!("{label} (unavailable)"),
            )));
        }
    }
}

/// What is rarely wanted, closed: where the entry comes from, and how it is built.
fn reference(
    out: &mut Out<'_>,
    entry: Option<&Entry>,
    sections: &[presentation::ReadingSection<'_>],
    styled: bool,
    source: &Source,
) {
    out.push(kit::gap(M));
    if entry.is_some_and(|entry| entry.reading().ok().flatten().is_some())
        && let Some(text) = sections
            .iter()
            .find(|section| section.title == "Source text")
    {
        let id = out.next_id("source-text");
        let markdown = out.markdown(id, text.body);
        let section = kit::QuietSection::new(out.next_id("source-text-section"), "Source text")
            .child(markdown);
        out.push(section);
    }
    let technical: Vec<_> = sections
        .iter()
        .filter(|section| {
            !section.title.starts_with("Splat data")
                && splat::content::has_data_marker(section.body)
        })
        .collect();
    let item_details = entry.is_some_and(reader::has_item_details);
    if (!styled && !technical.is_empty()) || item_details {
        let mut body = Vec::new();
        if let Some(entry) = entry.filter(|_| item_details) {
            body.extend(details::item_details(out, entry));
        }
        for section in technical {
            let id = out.next_id("technical");
            let markdown = out.compact_markdown(id, section.body);
            body.push(
                kit::QuietSection::new(out.next_id("technical-section"), section.title.clone())
                    .child(markdown)
                    .into_any_element(),
            );
        }
        let section = kit::QuietSection::new(out.next_id("technical-details"), "Technical details")
            .children(body);
        out.push(section);
    }
    // An unreadable file has no source or support details to report.
    let Some(entry) = entry else {
        return;
    };
    let conversion = entry.fields.get("splat_conversion");
    let status = conversion
        .and_then(|c| c.get("status"))
        .and_then(|s| s.as_str());
    let gap_list = |key: &'static str| {
        conversion
            .and_then(move |c| c.get(key))
            .and_then(|v| v.as_sequence())
            .into_iter()
            .flatten()
            .filter_map(|gap| gap.as_str())
    };
    let open_gaps: Vec<&str> = gap_list("engine_gaps")
        // Only built-in rules can be on the implemented list. A note cannot claim it by name.
        .filter(|gap| !source.builtin || !splat::content::adapters::is_implemented(gap))
        .collect();
    let support = match status {
        Some("specified") if open_gaps.is_empty() => "Implemented",
        Some("specified") => "Incomplete",
        Some("informational") => "Reference only",
        Some("partial") => "Conversion incomplete",
        _ => "Not established",
    };
    let c = colors();
    let line = |words: String| parts::item(kit::text(Type::Body, c.text, words));
    let mono = |words: String| parts::item(kit::text(Type::Mono, c.text, words));
    let mut body: Vec<AnyElement> = Vec::new();
    if !["world", "package"].contains(&entry.kind.as_str()) {
        let mut applies: Vec<AnyElement> = vec![line(support.to_owned()).into_any_element()];
        if conversion.is_some() {
            for gap in gap_list("conversion_gaps").chain(open_gaps.iter().copied()) {
                applies.push(mono(gap.to_owned()).into_any_element());
            }
        } else {
            applies.push(
                line("Splat has not established what it applies from this entry.".to_owned())
                    .into_any_element(),
            );
        }
        body.push(
            kit::QuietSection::new(out.next_id("applies"), "What Splat applies")
                .children(applies)
                .into_any_element(),
        );
    }
    body.push(line(source.owner.clone()).into_any_element());
    body.push(line(source.path.clone()).into_any_element());
    body.push(line(format!("{} · {}", entry.kind, entry.id)).into_any_element());
    if let Some(key) = &entry.key {
        body.push(mono(key.clone()).into_any_element());
    }
    if let Some(sources) = entry
        .fields
        .get("splat_sources")
        .and_then(|v| v.as_sequence())
    {
        for source in sources {
            if let Some(text) = source.as_str() {
                body.push(line(text.to_owned()).into_any_element());
                continue;
            }
            let title = source
                .get("title")
                .and_then(|v| v.as_str())
                .unwrap_or("Source");
            let locator = source.get("locator").and_then(|v| v.as_str()).unwrap_or("");
            body.push(line(format!("{title} · {locator}")).into_any_element());
        }
    }
    let section =
        kit::QuietSection::new(out.next_id("identifiers"), "Source and identifiers").children(body);
    out.push(section);
}

/// An entry's title and subtitle, with its picture beside them when it has one: the rarity is left out of an item's subtitle (its identity line says
/// it), and so is a flagged entry's warning mark.
pub fn title_with_art(title: &str, subtitle: &str, entry: Option<&Entry>, cx: &App) -> AnyElement {
    let subtitle = if let Some(entry) = entry {
        let rarity = entry
            .fields
            .get("splat_item")
            .and_then(|item| item.get("rarity"))
            .and_then(|value| value.as_str());
        let mut parts_of = subtitle.split(" · ");
        if !entry.flags.is_empty() && subtitle.starts_with(crate::design::icons::WARN) {
            parts_of.next();
        }
        parts_of
            .filter(|part| rarity.is_none_or(|rarity| *part != presentation::book_case(rarity)))
            .collect::<Vec<_>>()
            .join(" · ")
    } else {
        subtitle.to_owned()
    };
    let art = entry.and_then(|entry| host(cx).and_then(|ctx| crate::art::entry_art(&ctx, entry)));
    div()
        .flex()
        .flex_row()
        .items_start()
        .gap(M)
        .child(
            div()
                .flex_1()
                .min_w(XXL)
                .flex()
                .flex_col()
                .child(parts::title(title.to_owned()))
                .child(parts::subtitle(subtitle)),
        )
        .children(art.map(|path| {
            kit::target(
                div()
                    .size(XXL * 2.0)
                    .flex_none()
                    .child(img(path).size(XXL * 2.0).object_fit(ObjectFit::Contain)),
                title.to_owned(),
            )
        }))
        .into_any_element()
}

#[cfg(test)]
mod tests;
