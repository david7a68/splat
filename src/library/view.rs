//! The one entry reader. The Library, Lookup, the run's side panel, and campaign
//! entries all call `show`, so an entry looks the same everywhere (T6.4).
use super::{class, creature, presentation, reader};
use crate::design::{self, Type, colors, reader as style};
use eframe::egui::{self, Align, Layout, vec2};
use splat::content::{Entry, Reading, Reference};
use uuid::Uuid;

/// What the reader's links can open. An index belongs to the implementer.
pub(crate) trait Links {
    /// The one entry a reference names, or nothing if it is missing or ambiguous.
    fn find(&self, reference: &Reference) -> Option<(usize, String)>;
    /// Like `find`, for a reference that must name a spell.
    fn find_spell(&self, reference: &Reference) -> Option<(usize, String)>;
    /// Spells named in the words of a stat block paragraph.
    fn spells_named_in(&self, text: &str) -> Vec<(usize, String)>;
}

/// For an entry shown with nothing to link to.
pub(crate) struct NoLinks;
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
}

fn reading_view(ui: &mut egui::Ui, reading: &Reading, links: &dyn Links) -> Option<usize> {
    ui.add_space(design::M);
    style::markdown(ui, &reading.summary);
    let mut selected = None;
    if !reading.related.is_empty() {
        ui.add_space(design::S);
        style::section_heading(ui, &reading.related_heading);
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

fn spell_links(
    ui: &mut egui::Ui,
    references: &[Reference],
    paragraph: &str,
    links: &dyn Links,
) -> Option<usize> {
    let mut targets: Vec<Option<(usize, String)>> = references
        .iter()
        .map(|reference| links.find_spell(reference))
        .collect();
    for named in links.spells_named_in(paragraph) {
        if !targets.contains(&Some(named.clone())) {
            targets.push(Some(named));
        }
    }
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
                    "This spell is missing or has a duplicate identity in this package.",
                );
            }
        }
    });
    selected
}

/// Where an entry comes from.
pub(crate) struct Origin<'a> {
    pub path: &'a str,
    pub owner: &'a str,
    pub builtin: bool,
}

/// Values from an open run. The saved entry stays unchanged.
pub(crate) struct RunValues {
    pub armor_class: Option<i64>,
    pub hp: Option<i64>,
    pub initiative: Option<i64>,
    pub by_hand: Vec<String>,
}

/// Draw an entry's content. Returns the index of a link the DM followed.
pub(crate) fn show(
    ui: &mut egui::Ui,
    name: &str,
    entry: Option<&Entry>,
    body: &str,
    origin: &Origin<'_>,
    links: &dyn Links,
) -> Option<usize> {
    show_inner(ui, name, entry, body, origin, links, None)
}

pub(crate) fn show_run(
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
) -> Option<usize> {
    let reading = entry.and_then(|entry| entry.reading().ok().flatten());
    let sections = presentation::reading_sections(body, name);
    let styled = entry.filter(|entry| reading.is_none() && creature::supports(entry, &sections));
    let reading = reading.as_ref();
    let builtin = origin.builtin;
    let room = ui.available_width();
    let art = entry.and_then(|entry| crate::art::entry_art(ui.ctx(), entry));
    // A wide window's margin holds what is rarely wanted, so the prose keeps its measure and
    // the page has no dead space.
    if room >= design::READER_MEASURE + design::XXL + REFERENCE_MIN {
        let mut open_link = None;
        ui.horizontal_top(|ui| {
            let layout = Layout::top_down(Align::Min);
            let measure = design::READER_MEASURE;
            ui.allocate_ui_with_layout(vec2(measure, 0.0), layout, |ui| {
                ui.set_max_width(measure);
                open_link = content(
                    ui, entry, reading, &sections, styled, builtin, links, values,
                );
            });
            ui.add_space(design::XL);
            let width = ui.available_width().min(REFERENCE_MAX);
            ui.allocate_ui_with_layout(vec2(width, 0.0), layout, |ui| {
                ui.set_max_width(width);
                if let Some(path) = &art {
                    crate::art::hero(ui, path, name);
                }
                reference(ui, entry, &sections, styled.is_some(), origin);
            });
        });
        return open_link;
    }
    // Rules text keeps its measure in a wide side panel too.
    style::measure(ui, |ui| {
        if let Some(path) = &art {
            crate::art::small_hero(ui, path, name);
        }
        let open_link = content(
            ui, entry, reading, &sections, styled, builtin, links, values,
        );
        ui.add_space(design::L);
        design::rule(ui);
        reference(ui, entry, &sections, styled.is_some(), origin);
        open_link
    })
}

const REFERENCE_MIN: f32 = 280.0;
const REFERENCE_MAX: f32 = 360.0;
/// The reader's measure, its gap and the margin at their widest.
pub(crate) const WIDE_READER: f32 = design::READER_MEASURE + design::XL + REFERENCE_MAX;

/// What the entry says. `styled` is the entry when it reads as a styled stat block.
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
) -> Option<usize> {
    let mut open_link = None;
    if let Some(reading) = reading {
        open_link = reading_view(ui, reading, links);
    }
    if let Some(entry) = styled {
        let references = creature::spell_references(entry);
        creature::show(ui, entry, sections, values, |ui, paragraph| {
            open_link = open_link.or(spell_links(ui, &references, paragraph, links));
        });
    } else {
        // A creature the DM wrote has no source text to style. Its numbers still lead.
        if let Some(entry) = entry.filter(|entry| reading.is_none() && creature::has_stats(entry)) {
            creature::stat_head(ui, entry, values);
        }
        reader::show(ui, entry, sections, builtin, reading.is_some());
    }
    if let Some(entry) = entry.filter(|entry| class::lists_features(entry)) {
        open_link = open_link.or(class::show_features(ui, entry, links));
    }
    open_link
}

/// What is rarely wanted: where the entry comes from, and how it is built.
fn reference(
    ui: &mut egui::Ui,
    entry: Option<&Entry>,
    sections: &[presentation::ReadingSection<'_>],
    styled: bool,
    origin: &Origin<'_>,
) {
    let technical: Vec<_> = sections
        .iter()
        .filter(|section| splat::content::has_data_marker(section.body))
        .collect();
    if !styled && !technical.is_empty() {
        ui.add_space(design::M);
        design::quiet_section(ui, "Technical details", |ui| {
            for section in technical {
                design::quiet_section(ui, &section.title, |ui| {
                    style::compact_markdown(ui, section.body);
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
    // A verdict means nothing at a table, so it waits inside. Someone in the world has none.
    if entry.is_none_or(|entry| entry.kind != "world") {
        design::quiet_section(ui, "Engine support", |ui| {
            ui.label(support);
            if conversion.is_some() {
                for gap in gap_list("conversion_gaps").chain(open_gaps.iter().copied()) {
                    ui.monospace(gap);
                }
            } else {
                ui.label("Draft structure does not establish engine support.");
            }
        });
    }
    design::quiet_section(ui, "Source and identifiers", |ui| {
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
