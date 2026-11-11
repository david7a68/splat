//! The one entry reader. The Library, Lookup, the run's side panel, and campaign
//! entries all call `show`, so an entry looks the same everywhere (T6.4).
use super::{class, creature, presentation, reader};
use crate::design::{self, Type, colors, reader as style};
use eframe::egui::{self, Align, Layout, vec2};
use splat::content::{Entry, Reading, Reference};
use uuid::Uuid;

/// Keep the picture beside the title, so the stats can follow the meta line.
pub(crate) fn title_with_art(
    ui: &mut egui::Ui,
    title: &str,
    subtitle: &str,
    entry: Option<&Entry>,
) {
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
                    "This spell cannot be found, or two files claim to be the same entry in this package.",
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
    pub current_hp: Option<i64>,
    pub initiative: Option<i64>,
    pub by_hand: Vec<String>,
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
pub(crate) fn show_with_related(
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
        for flag in &entry.flags {
            design::message(
                ui,
                crate::design::Tone::Warning,
                &if flag.message.contains(" not read (line ") {
                    flag.message.clone()
                } else {
                    format!("Line {}: {}", flag.line, flag.message)
                },
            );
        }
    }
    let reading = entry.and_then(|entry| entry.reading().ok().flatten());
    let clean_body;
    let body = if let Some(entry) = entry.filter(|entry| creature::has_stats(entry)) {
        clean_body = creature::without_vitals(entry, body);
        clean_body.as_str()
    } else if let Some(entry) = entry.filter(|entry| {
        entry.fields.get("splat_schema").and_then(|v| v.as_u64()) == Some(2)
            && ["spell", "item"].contains(&entry.kind.as_str())
    }) {
        clean_body = splat::content::stat_blocks::reading_body(entry, body);
        clean_body.as_str()
    } else {
        body
    };
    let sections: Vec<_> = presentation::reading_sections(body, name)
        .into_iter()
        .filter(|section| {
            if entry.is_some_and(|e| {
                e.fields.get("splat_schema").and_then(|v| v.as_u64()) == Some(2)
                    && ["creature", "spell", "item"].contains(&e.kind.as_str())
            }) && presentation::overview_markdown(section.body)
                .trim()
                .is_empty()
            {
                return false;
            }
            section.title != "Personal notes"
                || section.body.trim() != "This section is kept when the sheet refreshes."
        })
        .collect();
    let styled = entry.filter(|entry| reading.is_none() && creature::supports(entry, &sections));
    let reading = reading.as_ref();
    let builtin = origin.builtin;
    // Rules text keeps its measure in a wide side panel too.
    style::measure(ui, |ui| {
        let open_link = content(
            ui, entry, reading, &sections, styled, builtin, links, values, lead_gap,
        );
        let open_link = open_link.or(related(ui));
        if entry.is_some() {
            ui.add_space(design::L);
            design::rule(ui);
            reference(ui, entry, &sections, styled.is_some(), origin);
        }
        open_link
    })
}

const REFERENCE_MAX: f32 = 360.0;
/// The reader's measure, its gap and the margin at their widest.
pub(crate) const WIDE_READER: f32 = design::READER_MEASURE + design::XL + REFERENCE_MAX;

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
) -> Option<usize> {
    let mut open_link = None;
    if let Some(reading) = reading {
        if lead_gap {
            ui.add_space(design::L);
        }
        style::markdown(ui, &reading.summary);
    }
    if let Some(entry) = styled {
        let references = creature::spell_references(entry);
        creature::show(ui, entry, sections, values, |ui, paragraph| {
            open_link = open_link.or(spell_links(ui, &references, paragraph, links));
        });
    } else {
        // A creature the DM wrote has no source text to style. Its numbers still lead.
        // The stat head is above the prose, so the gap after it stays.
        let mut lead_gap = lead_gap;
        if let Some(entry) = entry.filter(|entry| reading.is_none() && creature::has_stats(entry)) {
            creature::stat_head(ui, entry, values);
            lead_gap = true;
        }
        reader::show(ui, entry, sections, builtin, reading.is_some(), lead_gap);
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
    origin: &Origin<'_>,
) {
    ui.add_space(design::M);
    if entry.is_some_and(|entry| entry.reading().ok().flatten().is_some())
        && let Some(source) = sections
            .iter()
            .find(|section| section.title == "Source text")
    {
        design::quiet_section(ui, "Source text", |ui| style::markdown(ui, source.body));
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
        design::quiet_section(ui, "Technical details", |ui| {
            if let Some(entry) = entry.filter(|_| item_details) {
                reader::item_details(ui, entry);
            }
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
    // An unreadable file has no source or support details to report.
    if entry.is_none() {
        return;
    }
    design::quiet_section(ui, "Source and identifiers", |ui| {
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
                egui::Shape::Text(text) if text.galley.job.text.contains("Keep this prose.") => {
                    Some(text.pos.y)
                }
                _ => None,
            })
            .expect("the prose is painted")
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
        let warned = note(vec![splat::content::layout::Flag {
            line: 3,
            message: "Unknown field".into(),
        }]);
        assert_eq!(
            prose_top(&warned, true),
            prose_top(&warned, false),
            "iter-18-E-FT2-warning: a line warning keeps the gap before the prose"
        );
    }
}
