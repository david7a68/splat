//! The one entry reader. The Library, Lookup, the run's side panel, and campaign
//! entries all call `show`, so an entry looks the same everywhere (T6.4).
#![allow(deprecated)] // Old message functions; Wave 2 moves this file to `design::outcome`.
use super::{class, creature, presentation, reader, spells};
use crate::design::{self, Type, colors, reader as style};
use eframe::egui::{self, Align, Layout, vec2};
use splat::content::{Entry, Reading, Reference};
use uuid::Uuid;

/// Keep the picture beside the title, so the stats can follow the meta line.
pub fn title_with_art(ui: &mut egui::Ui, title: &str, subtitle: &str, entry: Option<&Entry>) {
    let subtitle = if let Some(entry) = entry {
        let rarity = entry
            .fields
            .get("splat_item")
            .and_then(|item| item.get("rarity"))
            .and_then(|value| value.as_str());
        let mut parts = subtitle.split(" · ");
        if !entry.flags.is_empty() && subtitle.starts_with(crate::design::icons::WARN) {
            parts.next();
        }
        parts
            .filter(|part| rarity.is_none_or(|rarity| *part != presentation::book_case(rarity)))
            .collect::<Vec<_>>()
            .join(" · ")
    } else {
        subtitle.to_owned()
    };
    let art = entry.and_then(|entry| crate::art::entry_art(ui.ctx(), entry));
    ui.horizontal_top(|ui| {
        let width = (ui.available_width()
            - art.as_ref().map_or(0.0, |_| design::XXL * 2.0 + design::M))
        .max(design::XXL);
        ui.allocate_ui_with_layout(vec2(width, 0.0), Layout::top_down(Align::Min), |ui| {
            style::title(ui, title);
            style::subtitle(ui, &subtitle);
        });
        if let Some(path) = art {
            crate::art::thumbnail(ui, &path, title);
        }
    });
}

/// What the reader's links can open. An index belongs to the implementer.
pub trait Links {
    /// The one entry a reference names, or nothing if it is missing or ambiguous.
    fn find(&self, reference: &Reference) -> Option<(usize, String)>;
    /// Like `find`, for a reference that must name a spell.
    fn find_spell(&self, reference: &Reference) -> Option<(usize, String)>;
    /// Spells named in the words of a stat block paragraph.
    fn spells_named_in(&self, text: &str) -> Vec<(usize, String)>;
    /// The one spell with this name, in any case.
    fn spell_named(&self, name: &str) -> Option<(usize, String)>;
    /// The entry at a place that `find`, `find_spell` or `spell_named` gave.
    fn entry(&self, index: usize) -> Option<&Entry>;
    /// Where an entry's resolved Spellcasting is kept, with the content's other caches; `None`
    /// to resolve it on each call.
    fn spell_cache(&self) -> Option<&super::spells::SpellCache> {
        None
    }
}

/// For an entry shown with nothing to link to.
pub struct NoLinks;
impl Links for NoLinks {
    fn find(&self, _: &Reference) -> Option<(usize, String)> {
        None
    }
    fn find_spell(&self, _: &Reference) -> Option<(usize, String)> {
        None
    }
    fn spells_named_in(&self, _: &str) -> Vec<(usize, String)> {
        Vec::new()
    }
    fn spell_named(&self, _: &str) -> Option<(usize, String)> {
        None
    }
    fn entry(&self, _: usize) -> Option<&Entry> {
        None
    }
}

fn reading_view(ui: &mut egui::Ui, reading: &Reading, links: &dyn Links) -> Option<usize> {
    let mut selected = None;
    if reading.related.is_empty() {
        design::gap(ui, design::Gap::M);
    } else {
        design::heading(ui, design::Heading::Section(&reading.related_heading));
        for (target, label) in &reading.related {
            let reference = Uuid::parse_str(target)
                .map_or_else(|_| Reference::Key(target.clone()), Reference::Id);
            if let Some((index, _)) = links.find(&reference) {
                if design::link(ui, label).clicked() {
                    selected = Some(index);
                }
            } else {
                ui.label(
                    Type::Small
                        .rich(ui, format!("{label} (unavailable)"))
                        .color(colors().muted),
                );
            }
        }
    }
    selected
}

/// A stat block's spells: each spell its lists name is a tile under the list's label, and a
/// line after the lists has the spells the entry gives or its words name that no list showed.
struct SpellLinks<'a> {
    entry: &'a Entry,
    /// The entry's spells, its lines' and cast paragraphs' tiles, and the spells that roll
    /// in printed order, which a tile's Ctrl number counts in.
    resolved: std::sync::Arc<spells::Resolved>,
    links: &'a dyn Links,
    next_list: usize,
    next_cast: usize,
    shown: Vec<usize>,
    opened: Option<usize>,
}

impl SpellLinks<'_> {
    /// A line's spells as a row of tiles: each that rolls with its number after the stat
    /// block's actions, and the spell it opens when the DM reads one.
    fn draw(&mut self, ui: &mut egui::Ui, line: &spells::SpellLine) {
        let tiles: Vec<crate::rolls::Tile> = line
            .tiles
            .iter()
            .map(|spell| match &spell.chip {
                Some(chip) => crate::rolls::Tile::Roll {
                    place: spell.place,
                    chip: chip.clone(),
                    read: Some(spell.name.clone()),
                },
                None => crate::rolls::Tile::Open(crate::rolls::OpenTile {
                    name: spell.name.clone(),
                    facts: spell.facts.clone(),
                    text: spell.text.clone(),
                }),
            })
            .collect();
        if let Some(at) = crate::rolls::tile_row(ui, &self.entry.name, &tiles) {
            self.opened = self.opened.or(Some(line.tiles[at].open));
        }
        self.shown.extend(line.tiles.iter().map(|spell| spell.open));
    }
}

impl creature::Spells for SpellLinks<'_> {
    fn list(&mut self, ui: &mut egui::Ui, spellcasting: &str, paragraph: &str) {
        let caster = spells::caster(self.entry, spellcasting);
        for text in paragraph.lines() {
            // Resolved with the entry, unless the reader's words differ from its body's.
            let resolved = std::sync::Arc::clone(&self.resolved);
            let fresh;
            let line = match resolved.line(text, caster, &mut self.next_list) {
                Some(line) => line,
                None => match spells::line(text, caster, &resolved.references, self.links) {
                    Some(line) => {
                        fresh = line;
                        &fresh
                    }
                    None => continue,
                },
            };
            design::help(ui, &line.label);
            self.draw(ui, line);
            if !line.missing.is_empty() {
                design::help(
                    ui,
                    &format!("Not found in this content: {}", line.missing.join(", ")),
                );
            }
        }
    }

    fn casts(&mut self, ui: &mut egui::Ui, paragraph: &str) {
        let resolved = std::sync::Arc::clone(&self.resolved);
        let fresh;
        // Resolved with the entry, unless the reader's words differ from its body's.
        let line = match resolved.cast(paragraph, &mut self.next_cast) {
            Some(line) => line,
            None => {
                fresh = spells::cast_line(self.entry, paragraph, &resolved.references, self.links);
                fresh.as_ref()
            }
        };
        if let Some(line) = line {
            self.draw(ui, line);
        }
    }

    fn rest(&mut self, ui: &mut egui::Ui, text: &str) {
        let mut targets = self.resolved.references.clone();
        // Resolved with the entry, unless the reader's words differ from its body's.
        let named = self.resolved.named_in(text).map_or_else(
            || self.links.spells_named_in(text),
            <[(usize, String)]>::to_vec,
        );
        for named in named {
            if !targets.contains(&Some(named.clone())) {
                targets.push(Some(named));
            }
        }
        targets.retain(|target| {
            target
                .as_ref()
                .is_none_or(|(place, _)| !self.shown.contains(place))
        });
        self.opened = self.opened.or(spell_links(ui, targets));
    }
}

fn spell_links(ui: &mut egui::Ui, targets: Vec<Option<(usize, String)>>) -> Option<usize> {
    if targets.is_empty() {
        return None;
    }
    let mut selected = None;
    ui.horizontal_wrapped(|ui| {
        ui.label(
            Type::Small
                .rich(ui, "Spell descriptions:")
                .color(colors().muted),
        );
        for target in targets {
            if let Some((index, name)) = target {
                if design::link(ui, &name).clicked() {
                    selected = Some(index);
                }
            } else {
                ui.label(
                    Type::Small
                        .rich(ui, "Spell unavailable")
                        .color(colors().muted),
                )
                .on_hover_text(
                    "This spell cannot be found, or two files claim to be the same entry in this package.",
                );
            }
        }
    });
    selected
}

/// Where an entry comes from.
pub struct Origin<'a> {
    pub path: &'a str,
    pub owner: &'a str,
    pub builtin: bool,
    /// The host's revision of what it shows (layout PR 9b-R1c): a new value whenever the entry,
    /// its body or its name can differ. The view keeps its reading of the entry and its prepared
    /// text while this and the entry's id stay the same, so a quiet frame parses nothing.
    pub revision: u64,
}

/// Where the shared view's Markdown is kept (layout PR 9b-R1c): one document for each part of
/// the entry, at the host's revision.
#[derive(Clone, Copy)]
pub(super) struct Docs {
    id: Uuid,
    revision: u64,
}

impl Docs {
    /// For a test that draws the section reader alone, in a new context each time.
    #[cfg(test)]
    pub(super) fn for_test() -> Self {
        Self {
            id: Uuid::nil(),
            revision: 0,
        }
    }

    /// The part's Markdown, kept prepared (`markdown_view::show_document`).
    pub(super) fn show(self, ui: &mut egui::Ui, part: crate::markdown_view::EntryPart, text: &str) {
        crate::markdown_view::show_document(
            ui,
            crate::markdown_view::ReaderSource {
                doc: crate::markdown_view::DocumentId::Part(self.id, part),
                revision: self.revision,
                text,
            },
        );
    }
}

/// Everything the shared view's reading of an entry is made from (layout PR 9b-R1c; the
/// invariants note, class 2): the entry, its body and its name, and the host's revision of them.
/// The reading is made from this alone, with no `Ui`, and kept by a key made from the same
/// struct: each one names every field, so a new input cannot miss the key.
struct EntryInputs<'a> {
    entry: Option<&'a Entry>,
    body: &'a str,
    name: &'a str,
    /// The host's revision of the entry, its body and its name (`Origin::revision`).
    revision: u64,
}

impl EntryInputs<'_> {
    /// The entry's id (nil for a file that could not be read), and the revision that stands for
    /// its words: the entry's fields, its body and its name.
    fn key(&self) -> (Uuid, u64) {
        let Self {
            entry,
            body: _,
            name: _,
            revision,
        } = self;
        (entry.map_or(Uuid::nil(), |entry| entry.id), *revision)
    }
}

/// The shared view's reading of one entry, kept between frames (layout PR 9b-R1c): its summary,
/// its sections (from the body with a stat block's lines taken out) and whether it reads as a
/// styled stat block. Kept for its inputs' key.
struct Prepared {
    key: (Uuid, u64),
    reading: Option<Reading>,
    sections: Vec<presentation::ReadingSection<'static>>,
    styled: bool,
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// Each time the shared view read an entry again (its summary, body and sections), on this
    /// thread: none on a quiet frame (layout PR 9b-R1c's proof).
    pub static ENTRY_PREPARES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

impl Prepared {
    fn of(inputs: &EntryInputs<'_>) -> Self {
        #[cfg(any(test, feature = "test-support"))]
        ENTRY_PREPARES.with(|count| count.set(count.get() + 1));
        let key = inputs.key();
        let EntryInputs {
            entry,
            body,
            name,
            revision: _,
        } = *inputs;
        let reading = entry.and_then(|entry| entry.reading().ok().flatten());
        let clean_body;
        let body = if let Some(entry) = entry.filter(|entry| creature::has_stats(entry)) {
            clean_body = creature::without_vitals(entry, body);
            clean_body.as_str()
        } else if let Some(entry) =
            entry.filter(|entry| ["spell", "item"].contains(&entry.kind.as_str()))
        {
            clean_body = splat::content::stat_blocks::reading_body(entry, body);
            clean_body.as_str()
        } else {
            body
        };
        let sections: Vec<_> = presentation::reading_sections(body, name)
            .into_iter()
            .filter(|section| {
                if entry.is_some_and(|e| ["creature", "spell", "item"].contains(&e.kind.as_str()))
                    && presentation::overview_markdown(&section.body)
                        .trim()
                        .is_empty()
                {
                    return false;
                }
                section.title != "Personal notes"
                    || section.body.trim() != "This section is kept when the sheet refreshes."
            })
            .map(presentation::ReadingSection::into_owned)
            .collect();
        let styled =
            reading.is_none() && entry.is_some_and(|entry| creature::supports(entry, &sections));
        Self {
            key,
            reading,
            sections,
            styled,
        }
    }
}

/// Each view's kept reading, by the view's place, with the frame it last drew in. A view not
/// drawn in the last frame loses it, so an entry no view shows keeps nothing.
#[derive(Clone, Default)]
struct KeptViews(std::sync::Arc<std::sync::Mutex<KeptMap>>);

type KeptMap = std::collections::HashMap<egui::Id, (u64, std::sync::Arc<Prepared>)>;

/// The view's reading of the entry, kept for its inputs' key. The `Ui` gives only the view's
/// place and the frame.
fn prepared(ui: &egui::Ui, inputs: &EntryInputs<'_>) -> std::sync::Arc<Prepared> {
    let key = inputs.key();
    let frame = ui.ctx().cumulative_frame_nr();
    let views = ui.ctx().data_mut(|data| {
        data.get_temp_mut_or_default::<KeptViews>(egui::Id::new("entry_view_kept"))
            .clone()
    });
    let mut views = views
        .0
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    views.retain(|_, (drawn, _)| *drawn + 1 >= frame);
    match views.get_mut(&ui.id()) {
        Some((drawn, kept)) if kept.key == key => {
            *drawn = frame;
            kept.clone()
        }
        _ => {
            let made = std::sync::Arc::new(Prepared::of(inputs));
            views.insert(ui.id(), (frame, made.clone()));
            made
        }
    }
}

/// Values from an open run. The saved entry stays unchanged.
pub struct RunValues {
    pub armor_class: Option<i64>,
    pub hp: Option<i64>,
    pub current_hp: Option<i64>,
    pub initiative: Option<i64>,
    pub speed: Option<String>,
}

/// Draw an entry's content. Returns the index of a link the DM followed.
#[cfg(test)]
pub(crate) fn show(
    ui: &mut egui::Ui,
    name: &str,
    entry: Option<&Entry>,
    body: &str,
    origin: &Origin<'_>,
    links: &dyn Links,
) -> Option<usize> {
    show_inner(ui, name, entry, body, origin, links, None, false, |_| None)
}

/// `empty_value_block`: the caller drew a value block above the entry, and that block drew
/// no row. The prose then starts without the reader's own gap, where a plain note's prose
/// starts. Line warnings keep the gap after them.
#[allow(clippy::too_many_arguments)]
pub fn show_with_related(
    ui: &mut egui::Ui,
    name: &str,
    entry: Option<&Entry>,
    body: &str,
    origin: &Origin<'_>,
    links: &dyn Links,
    empty_value_block: bool,
    related: impl FnOnce(&mut egui::Ui) -> Option<usize>,
) -> Option<usize> {
    show_inner(
        ui,
        name,
        entry,
        body,
        origin,
        links,
        None,
        empty_value_block,
        related,
    )
}

pub fn show_run(
    ui: &mut egui::Ui,
    entry: &Entry,
    origin: &Origin<'_>,
    links: &dyn Links,
    values: &RunValues,
) -> Option<usize> {
    show_inner(
        ui,
        &entry.name,
        Some(entry),
        &entry.body,
        origin,
        links,
        Some(values),
        false,
        |_| None,
    )
}

#[allow(clippy::too_many_arguments)]
fn show_inner(
    ui: &mut egui::Ui,
    name: &str,
    entry: Option<&Entry>,
    body: &str,
    origin: &Origin<'_>,
    links: &dyn Links,
    values: Option<&RunValues>,
    empty_value_block: bool,
    related: impl FnOnce(&mut egui::Ui) -> Option<usize>,
) -> Option<usize> {
    crate::rolls::set_entry(ui.ctx(), name);
    // The gap before the prose separates it from what is above. Line warnings need it; an
    // empty value block does not.
    let lead_gap = !empty_value_block || entry.is_some_and(|entry| !entry.flags.is_empty());
    if let Some(entry) = entry {
        ui.ctx()
            .data_mut(|data| data.insert_temp(egui::Id::new("reader_document"), entry.id));
        // Line warnings wrap at the reader measure, where the stat block and the prose end, as
        // World's do (round 26 FG3).
        if !entry.flags.is_empty() {
            // One `L` after the kind line, also before a warning (row 94).
            design::gap(ui, design::Gap::L);
            style::measure(ui, |ui| {
                for flag in &entry.flags {
                    design::message(ui, crate::design::Tone::Warning, &flag.message);
                }
            });
        }
    }
    // Layout PR 9b-R1c: the reading, the body's sections and the prepared text are kept for the
    // entry and the host's revision, so a quiet frame parses and lays out nothing.
    let prepared = prepared(
        ui,
        &EntryInputs {
            entry,
            body,
            name,
            revision: origin.revision,
        },
    );
    let docs = Docs {
        id: prepared.key.0,
        revision: origin.revision,
    };
    let reading = prepared.reading.as_ref();
    let sections = prepared.sections.as_slice();
    let styled = entry.filter(|_| prepared.styled);
    let builtin = origin.builtin;
    // Rules text keeps its measure in a wide side panel too.
    style::measure(ui, |ui| {
        let open_link = content(
            ui, entry, reading, sections, styled, builtin, links, values, lead_gap, docs,
        );
        let open_link = open_link.or(related(ui));
        if entry.is_some() {
            design::gap(ui, design::Gap::L);
            design::rule(ui);
            reference(
                ui,
                entry,
                sections,
                styled.is_some(),
                reading.is_some(),
                origin,
                docs,
            );
        }
        open_link
    })
}

const REFERENCE_MAX: f32 = 360.0;
/// The reader's measure, its gap and the margin at their widest.
pub const WIDE_READER: f32 = design::READER_MEASURE + design::XL + REFERENCE_MAX;

/// What the entry says. `styled` is the entry when it reads as a styled stat block.
/// `lead_gap` is false when nothing above needs a gap before the first prose.
#[allow(clippy::too_many_arguments)]
fn content(
    ui: &mut egui::Ui,
    entry: Option<&Entry>,
    reading: Option<&Reading>,
    sections: &[presentation::ReadingSection<'_>],
    styled: Option<&Entry>,
    builtin: bool,
    links: &dyn Links,
    values: Option<&RunValues>,
    lead_gap: bool,
    docs: Docs,
) -> Option<usize> {
    let mut open_link = None;
    if let Some(reading) = reading {
        if lead_gap {
            design::gap(ui, design::Gap::L);
        }
        docs.show(
            ui,
            crate::markdown_view::EntryPart::Summary,
            &reading.summary,
        );
    }
    if let Some(entry) = styled {
        let mut spells = SpellLinks {
            entry,
            resolved: spells::resolve(entry, links),
            links,
            next_list: 0,
            next_cast: 0,
            shown: Vec::new(),
            opened: None,
        };
        creature::show(ui, entry, sections, values, &mut spells);
        open_link = open_link.or(spells.opened);
    } else {
        // A creature the DM wrote has no source text to style. Its numbers still lead.
        // The reader adds no lead gap after the stat head: prose gets the stat head's `S`, and
        // the first heading owns its space, so it sits where a styled one does.
        let mut lead_gap = lead_gap;
        let mut after_stat_head = false;
        if let Some(entry) = entry.filter(|entry| reading.is_none() && creature::has_stats(entry)) {
            creature::stat_head(ui, entry, values);
            lead_gap = false;
            after_stat_head = true;
        }
        reader::show(
            ui,
            entry,
            sections,
            builtin,
            reading.is_some(),
            lead_gap,
            after_stat_head,
            docs,
        );
    }
    if let Some(entry) = entry.filter(|entry| class::lists_features(entry)) {
        open_link = open_link.or(class::show_features(ui, entry, links));
    }
    if let Some(reading) = reading {
        open_link = open_link.or(reading_view(ui, reading, links));
    }
    open_link
}

/// What is rarely wanted: where the entry comes from, and how it is built.
fn reference(
    ui: &mut egui::Ui,
    entry: Option<&Entry>,
    sections: &[presentation::ReadingSection<'_>],
    styled: bool,
    has_reading: bool,
    origin: &Origin<'_>,
    docs: Docs,
) {
    design::gap(ui, design::Gap::M);
    // M0387 (round 31 reader-end-section-pitch): the end sections are one group at the Read
    // row pitch. A header keeps egui's height (its `Small` words and button padding, at least
    // `interact_size`), and the gap after it is the rest of one `Read` row, so a reader that
    // fits its pane still does. An open section's body gets the page's spacing back.
    let page = ui.spacing().clone();
    let header =
        (design::Type::Small.line() + 2.0 * page.button_padding.y).max(page.interact_size.y);
    ui.scope(|ui| {
        ui.spacing_mut().item_spacing.y = (design::Density::Read.control() - header).max(0.0);
        end_sections(
            ui,
            &page,
            entry,
            sections,
            styled,
            has_reading,
            origin,
            docs,
        );
    });
}

/// The reader's end sections (`reference`), header after header; `page` is the page's
/// spacing, which each open body gets back. `has_reading`: the entry has a reading (a summary).
#[allow(clippy::too_many_arguments)]
fn end_sections(
    ui: &mut egui::Ui,
    page: &egui::style::Spacing,
    entry: Option<&Entry>,
    sections: &[presentation::ReadingSection<'_>],
    styled: bool,
    has_reading: bool,
    origin: &Origin<'_>,
    docs: Docs,
) {
    let body = |ui: &mut egui::Ui| *ui.spacing_mut() = page.clone();
    if has_reading
        && let Some(source) = sections
            .iter()
            .find(|section| section.title == "Source text")
    {
        design::quiet_section(ui, "Source text", |ui| {
            body(ui);
            docs.show(ui, crate::markdown_view::EntryPart::Source, &source.body);
        });
    }
    let technical: Vec<_> = sections
        .iter()
        .filter(|section| {
            !section.title.starts_with("Splat data")
                && splat::content::has_data_marker(&section.body)
        })
        .collect();
    let item_details = entry.is_some_and(reader::has_item_details);
    if (!styled && !technical.is_empty()) || item_details {
        design::quiet_section(ui, "Technical details", |ui| {
            body(ui);
            if let Some(entry) = entry.filter(|_| item_details) {
                reader::item_details(ui, entry);
            }
            for section in technical {
                design::quiet_section(ui, &section.title, |ui| {
                    style::compact_markdown(ui, &section.body);
                });
            }
        });
    }
    let conversion = entry.and_then(|e| e.fields.get("splat_conversion"));
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
        .filter(|gap| !origin.builtin || !splat::content::adapters::is_implemented(gap))
        .collect();
    let support = match status {
        Some("specified") if open_gaps.is_empty() => "Implemented",
        Some("specified") => "Incomplete",
        Some("informational") => "Reference only",
        Some("partial") => "Conversion incomplete",
        _ => "Not established",
    };
    // An unreadable file has no source or support details to report.
    if entry.is_none() {
        return;
    }
    design::quiet_section(ui, "Source and identifiers", |ui| {
        body(ui);
        if entry.is_some_and(|entry| !["world", "package"].contains(&entry.kind.as_str())) {
            design::quiet_section(ui, "What Splat applies", |ui| {
                ui.label(support);
                if conversion.is_some() {
                    for gap in gap_list("conversion_gaps").chain(open_gaps.iter().copied()) {
                        ui.monospace(gap);
                    }
                } else {
                    ui.label("Splat has not established what it applies from this entry.");
                }
            });
        }
        ui.label(origin.owner);
        ui.label(origin.path);
        if let Some(entry) = entry {
            ui.label(format!("{} · {}", entry.kind, entry.id));
            if let Some(key) = &entry.key {
                ui.monospace(key);
            }
            if let Some(sources) = entry
                .fields
                .get("splat_sources")
                .and_then(|v| v.as_sequence())
            {
                for source in sources {
                    if let Some(text) = source.as_str() {
                        ui.label(text);
                        continue;
                    }
                    let title = source
                        .get("title")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Source");
                    let locator = source.get("locator").and_then(|v| v.as_str()).unwrap_or("");
                    ui.label(format!("{title} · {locator}"));
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The top of the painted prose line, read through `show_with_related`.
    fn prose_top(entry: &Entry, empty_value_block: bool) -> f32 {
        text_top(entry, empty_value_block, "Keep this prose.")
    }

    /// The top of the first painted text that holds `words`.
    fn text_top(entry: &Entry, empty_value_block: bool, words: &str) -> f32 {
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let output = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                show_with_related(
                    ui,
                    &entry.name,
                    Some(entry),
                    &entry.body,
                    &Origin {
                        path: &entry.path,
                        owner: "Campaign",
                        builtin: false,
                        revision: 0,
                    },
                    &NoLinks,
                    empty_value_block,
                    |_| None,
                );
            });
        });
        output
            .shapes
            .iter()
            .find_map(|shape| match &shape.shape {
                egui::Shape::Text(text) if text.galley.job.text.contains(words) => Some(text.pos.y),
                _ => None,
            })
            .expect("the words are painted")
    }

    /// The tops of a creature's opening stat block rule and of its first section heading, and
    /// the bottom of the closing rule above that heading, through `show_with_related`.
    fn stat_block_rules(entry: &Entry, heading: &str) -> (f32, f32, f32) {
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let output = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                show_with_related(
                    ui,
                    &entry.name,
                    Some(entry),
                    &entry.body,
                    &Origin {
                        path: &entry.path,
                        owner: "Campaign",
                        builtin: false,
                        revision: 0,
                    },
                    &NoLinks,
                    false,
                    |_| None,
                );
            });
        });
        let heading_top = output
            .shapes
            .iter()
            .find_map(|shape| match &shape.shape {
                egui::Shape::Text(text) if text.galley.job.text == heading => Some(text.pos.y),
                _ => None,
            })
            .unwrap_or_else(|| panic!("{heading} is painted"));
        // A stat block opens and closes with a heavy rule: a 2 px bar in the rule colour.
        let heavy: Vec<egui::Rect> = output
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Rect(rect)
                    if (rect.rect.height() - 2.0).abs() < 0.1
                        && rect.fill == design::colors().rule =>
                {
                    Some(rect.rect)
                }
                _ => None,
            })
            .collect();
        let opening = heavy
            .first()
            .expect("the stat block opens with a heavy rule");
        let closing = heavy
            .iter()
            .rfind(|rule| rule.bottom() <= heading_top)
            .expect("a heavy rule closes the stat block above the heading");
        assert!(closing.top() > opening.top(), "two heavy rules");
        (opening.top(), closing.bottom(), heading_top)
    }

    /// FD5 (round 21): a schema-2 creature with no source text to style (Slate Wolf). Its
    /// first heading starts as far below the stat block's closing rule as a styled creature's
    /// does, also with a line warning. Must stay true: the stat block opens at the same height.
    #[test]
    fn iter21_e_fd5_converted_creature_heading_gap_matches_a_styled_creature() {
        let styled = Entry::parse(
            "Aboleth.md",
            splat::content::definitions::source_file("definitions/creatures/aboleth.md").unwrap(),
        )
        .unwrap();
        assert!(
            creature::supports(&styled, &presentation::reading_sections(&styled.body, "")),
            "fixture: the Aboleth reads as a styled stat block"
        );
        // Slate Wolf as the visual fixture writes it: a schema-2 note from the note writer,
        // with its stat tables and no source text.
        let written = splat::content::formats::fixture(
            "Creatures/Slate Wolf.md",
            "splat_id: 00000000-0000-0000-0000-000000000009\nsplat_kind: creature\nsplat_name: Slate Wolf\nsplat_creature: {size: medium, type: beast, alignment: Unaligned, hit_points_text: '22 (4d8 + 4)', cr_text: '1 (XP 200; PB +2)'}\nsplat_stats: {max_hp: {value: 22}, armor_class: {value: 13}, initiative: {value: 2}, speed: {value: 40, unit: ft}, strength: {value: 12}, dexterity: {value: 15}}\n",
            "\n# Slate Wolf\n\n## Actions\n\nBite. Melee Attack Roll: +4, reach 5 ft. Hit: 7 Piercing damage.\n",
        )
        .unwrap();
        assert!(
            written.contains("splat_schema: 2")
                && written.contains("| Hit Points | 22 (4d8 + 4) |"),
            "fixture: the writer gives a schema-2 note with its stat table"
        );
        let slate = Entry::parse("Creatures/Slate Wolf.md", &written).unwrap();
        assert!(
            creature::has_stats(&slate)
                && !creature::supports(&slate, &presentation::reading_sections(&slate.body, "")),
            "fixture: Slate Wolf has a stat head and no source text to style"
        );
        let mut flagged = slate.clone();
        flagged.flags = vec![splat::content::layout::Flag::new(
            19,
            "Hit Points not read".into(),
        )];
        let (styled_open, styled_close, styled_heading) = stat_block_rules(&styled, "Traits");
        let styled_gap = styled_heading - styled_close;
        for (name, entry) in [("Slate Wolf", &slate), ("flagged Slate Wolf", &flagged)] {
            let (open, close, heading) = stat_block_rules(entry, "Actions");
            assert!(
                (heading - close - styled_gap).abs() <= 0.5,
                "iter21-E-FD5-gap: {name}'s Actions starts {} below the closing rule; a styled creature's Traits starts {styled_gap} below",
                heading - close
            );
            if entry.flags.is_empty() {
                assert_eq!(
                    open, styled_open,
                    "iter21-E-FD5-open: {name}'s stat block opens where a styled one does"
                );
            }
        }
    }

    fn note(flags: Vec<splat::content::layout::Flag>) -> Entry {
        let written = splat::content::formats::fixture(
            "World.md",
            "splat_id: 00000000-0000-0000-0000-000000000002\nsplat_kind: world\nsplat_name: Watch House\nsplat_world:\n  type: location\n",
            "\n# Watch House\n\nKeep this prose.\n",
        )
        .unwrap();
        let mut entry =
            Entry::parse("Places/Watch House.md", &written).expect("fixture: a world note");
        entry.flags = flags;
        entry
    }

    /// FT2 (iter 18): a value block that drew no row takes away the reader's gap before the
    /// prose, and only that gap. Line warnings keep the gap after them.
    #[test]
    fn iter18_e_ft2_empty_value_block_skips_only_the_lead_gap() {
        let plain = note(Vec::new());
        let with_gap = prose_top(&plain, false);
        let without_gap = prose_top(&plain, true);
        assert_eq!(
            with_gap - without_gap,
            design::L,
            "iter-18-E-FT2-flag: the flag takes back the reader's L before the prose ({with_gap} with the gap, {without_gap} without)"
        );
        // Must stay true: after a line warning, the prose keeps its gap with the flag set.
        let warned = note(vec![splat::content::layout::Flag::new(
            3,
            "Unknown field".into(),
        )]);
        assert_eq!(
            prose_top(&warned, true),
            prose_top(&warned, false),
            "iter-18-E-FT2-warning: a line warning keeps the gap before the prose"
        );
        // Row 94 (FH2): a warning starts `L` below what is above it, as the prose does.
        let warning_top = text_top(&warned, false, "Unknown field");
        assert!(
            warning_top >= with_gap - 0.5,
            "iter-28-L-94: the warning starts at {warning_top}, the prose of a plain note at {with_gap}"
        );
        // The reader shows a flag's words as they were made or saved, and adds no line: an
        // older saved flag and a new one each name line 19 once, in their whole sentence.
        let old_words = "Hit Points “about twenty” not read (line 19). Start with a number, such as 22 (4d8 + 4).";
        let new_words = "Line 19: Hit Points “about twenty” was not read. Start with a number, such as 22 (4d8 + 4).";
        let saved = note(vec![
            serde_json::from_value(serde_json::json!({"line": 19, "message": old_words})).unwrap(),
            splat::content::layout::Flag::unread(
                splat::content::layout::Field::HitPoints,
                19,
                "Hit Points",
                "about twenty",
                "Start with a number, such as 22 (4d8 + 4).",
            ),
        ]);
        let painted = painted_texts(&saved);
        for words in [old_words, new_words] {
            assert!(
                painted.iter().any(|text| text == words),
                "{words} in {painted:?}"
            );
        }
        let lines: usize = painted
            .iter()
            .map(|text| text.to_lowercase().matches("line 19").count())
            .sum();
        assert_eq!(lines, 2, "one line reference for each flag: {painted:?}");
    }

    /// Every text the reader paints for this entry.
    fn painted_texts(entry: &Entry) -> Vec<String> {
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let output = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                show_with_related(
                    ui,
                    &entry.name,
                    Some(entry),
                    &entry.body,
                    &Origin {
                        path: &entry.path,
                        owner: "Campaign",
                        builtin: false,
                        revision: 0,
                    },
                    &NoLinks,
                    false,
                    |_| None,
                );
            });
        });
        output
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Text(text) => Some(text.galley.job.text.clone()),
                _ => None,
            })
            .collect()
    }
}
