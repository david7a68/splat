//! What other pages say about the one on show. A fact has one home, and these lists bring it
//! here: what is in this place, what is part of this group, the events that concern it, and
//! each line elsewhere that names it. See `docs/design/world-data.md`.
use super::{Go, Role, date_words, lines::Line, type_group, type_label, world_type};
use crate::{
    design::{self, Type, colors, reader as style},
    lookup::{ContentIndex, Item, Mention},
};
use eframe::egui;
use std::collections::BTreeMap;

/// A group with more lines than this starts closed, so a page with many links stays short.
const OPEN_LINES: usize = 8;
/// More lines than this from one page are counted, not shown.
const LINES_PER_PAGE: usize = 3;
/// About two rows of a reader's width.
const LINE_WIDTH: usize = 220;

/// The World page's order: people, places, the DM's other types, plain notes, then play.
pub(crate) fn page_group(item: &Item) -> (u8, String) {
    match item.note_role() {
        Some(Role::World) => (3, "Notes".into()),
        Some(Role::Session) => (4, "Sessions".into()),
        Some(Role::Transcript) => (5, "Transcripts".into()),
        None => match world_type(&item.entry) {
            "npc" => (0, type_group("npc")),
            "location" => (1, type_group("location")),
            other => (2, type_group(other)),
        },
    }
}

/// A page's name as a link, then what it is, in words that wrap. The name and the words are one
/// paragraph with one line pitch, `Small`'s (FG2): the words' first line follows the name, and
/// each wrapped line starts at the name's edge. The name keeps its `XL` target, centred on the
/// first line, so the paragraph has that room above and below it and no target overlaps.
fn page_row(ui: &mut egui::Ui, index: &ContentIndex, position: usize) -> bool {
    let item = &index.items[position];
    let detail = match item.summary() {
        "" if item.entry.kind == "world" => type_label(world_type(&item.entry)),
        summary => summary.to_owned(),
    };
    let c = colors();
    let width = ui.available_width().max(1.0);
    let name = &item.entry.name;
    let name_galley = design::one_line(ui, name, Type::Body.font(ui), c.accent, width);
    let name_width = name_galley.size().x;
    let target_width = name_width.max(design::XL);
    let line = Type::Small.line();
    let detail_galley = (!detail.is_empty()).then(|| {
        let mut job = egui::text::LayoutJob::default();
        job.append(
            &detail,
            target_width + ui.spacing().item_spacing.x,
            egui::TextFormat {
                font_id: Type::Small.font(ui),
                line_height: Some(line),
                color: c.muted,
                ..Default::default()
            },
        );
        job.wrap.max_width = width;
        ui.fonts_mut(|fonts| fonts.layout_job(job))
    });
    let lines = detail_galley
        .as_ref()
        .map_or(line, |galley| galley.size().y.max(line));
    // The target's room above and below the first line.
    let pad = (design::XL - line) / 2.0;
    let (rect, row) =
        ui.allocate_exact_size(egui::vec2(width, lines + pad * 2.0), egui::Sense::hover());
    let first = rect.top() + pad;
    let target = egui::Rect::from_min_size(rect.min, egui::vec2(target_width, design::XL));
    let response = ui.interact(target, row.id.with("link"), egui::Sense::click());
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Link, ui.is_enabled(), name));
    if ui.is_rect_visible(rect) {
        let color = if response.hovered() {
            c.focus
        } else {
            c.accent
        };
        let top = first + line / 2.0 - name_galley.size().y / 2.0;
        ui.painter().hline(
            rect.left()..=rect.left() + name_width,
            top + name_galley.size().y - 1.0,
            egui::Stroke::new(1.0, color),
        );
        ui.painter()
            .galley(egui::pos2(rect.left(), top), name_galley, color);
        if let Some(galley) = detail_galley {
            ui.painter()
                .galley(egui::pos2(rect.left(), first), galley, c.muted);
        }
        if response.has_focus() {
            design::focus_ring(ui, target);
        }
    }
    response
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .clicked()
}

/// History rows put each date in one column, as wide as the widest date plus an `S` gap, so
/// names and summaries share their left edges. The column takes at most a third of the width,
/// and a longer date wraps in it. A row without a date keeps the column empty.
fn history_rows(ui: &mut egui::Ui, index: &ContentIndex, history: &[usize]) -> Option<Go> {
    let dates: Vec<Option<String>> = history
        .iter()
        .map(|&position| date_words(&index.items[position].entry))
        .collect();
    let font = Type::Small.font(ui);
    let column = dates
        .iter()
        .flatten()
        .map(|date| {
            ui.painter()
                .layout_no_wrap(date.clone(), font.clone(), colors().muted)
                .size()
                .x
        })
        .fold(0.0_f32, f32::max)
        .min(ui.available_width() / 3.0);
    let mut go = None;
    for (&position, date) in history.iter().zip(&dates) {
        let clicked = ui
            .horizontal_top(|ui| {
                let gap = ui.spacing().item_spacing.x;
                ui.spacing_mut().item_spacing.x = design::S;
                let size = egui::vec2(column, design::XL);
                ui.allocate_ui_with_layout(size, egui::Layout::top_down(egui::Align::Min), |ui| {
                    ui.set_min_size(size);
                    ui.set_max_width(column);
                    // The first line sits on the name's line; a wrapped date grows down.
                    ui.add_space(((size.y - Type::Small.line()) / 2.0).max(0.0));
                    if let Some(date) = date {
                        ui.add(
                            egui::Label::new(Type::Small.rich(ui, date).color(colors().muted))
                                .wrap(),
                        );
                    }
                });
                ui.spacing_mut().item_spacing.x = gap;
                page_row(ui, index, position)
            })
            .inner;
        if clicked {
            go = Some(Go::Item(position, None));
        }
    }
    go
}

/// One line from another page. Pressing it opens that page at the line's heading.
fn fact(ui: &mut egui::Ui, line: &Line, names: &[String]) -> bool {
    let words = line.snippet(names, LINE_WIDTH);
    let (label, text) = match &line.label {
        Some(label) if words.starts_with(&format!("{label}:")) => words.split_at(label.len() + 1),
        _ => ("", words.as_str()),
    };
    design::line_link(ui, label, text).clicked()
}

/// A source note's name. A copy of this note, or any file with its name, is named by its path,
/// as search rows name duplicates.
fn source_name(index: &ContentIndex, item: &Item, source: usize) -> String {
    let entry = &index.items[source].entry;
    if entry.name.eq_ignore_ascii_case(&item.entry.name)
        || index.duplicate_world.contains_key(&entry.path)
    {
        entry.path.clone()
    } else {
        entry.name.clone()
    }
}

/// The lines other pages hold about this one, by the kind of page and then by page.
/// `listed` holds the pages that a list above already shows with their summaries.
fn from_other_pages(
    ui: &mut egui::Ui,
    item: &Item,
    index: &ContentIndex,
    listed: &[usize],
) -> Option<Go> {
    let mentions = index.lines_about(item.entry.id);
    if mentions.is_empty() {
        return None;
    }
    let mut names = vec![item.entry.name.to_lowercase()];
    names.extend(super::aliases(&item.entry).iter().map(|a| a.to_lowercase()));
    type Pages<'a> = Vec<(usize, Vec<&'a Line>)>;
    let mut groups: BTreeMap<(u8, String), Pages> = BTreeMap::new();
    let mut only_linked = Vec::new();
    for page in mentions.chunk_by(|a, b| a.source == b.source) {
        let source = page[0].source;
        let facts: Vec<&Line> = page
            .iter()
            .filter_map(|&Mention { line, .. }| Some(&index.items[source].lines[line?]))
            .filter(|line| !line.bare)
            .collect();
        // A summary that a list above shows is not said twice.
        let said = listed.contains(&source);
        let summary = index.items[source].summary();
        let facts: Vec<&Line> = facts
            .into_iter()
            .filter(|line| !(said && line.label.is_none() && line.text == summary))
            .collect();
        if facts.is_empty() {
            if !said {
                only_linked.push(source);
            }
        } else {
            groups
                .entry(page_group(&index.items[source]))
                .or_default()
                .push((source, facts));
        }
    }
    if groups.is_empty() && only_linked.is_empty() {
        return None;
    }
    let mut go = None;
    ui.add_space(design::S);
    style::section_heading(ui, "From other notes");
    for ((_, title), pages) in &groups {
        let count: usize = pages.iter().map(|(_, facts)| facts.len()).sum();
        egui::CollapsingHeader::new(design::group_heading_text(ui, title, count))
            .id_salt(("lines_about", item.entry.id, title))
            .default_open(count <= OPEN_LINES)
            .icon(design::section_icon)
            .show(ui, |ui| {
                for (source, facts) in pages {
                    ui.push_id(source, |ui| {
                        if design::link(ui, &source_name(index, item, *source)).clicked() {
                            go = Some(Go::Item(*source, None));
                        }
                        for line in facts.iter().take(LINES_PER_PAGE) {
                            if fact(ui, line, &names) {
                                go = Some(Go::Item(*source, line.heading.clone()));
                            }
                        }
                        if facts.len() > LINES_PER_PAGE {
                            ui.label(
                                Type::Small
                                    .rich(
                                        ui,
                                        design::count(
                                            facts.len() - LINES_PER_PAGE,
                                            "more line",
                                            "more lines",
                                        ),
                                    )
                                    .color(colors().muted),
                            );
                        }
                    });
                }
            });
    }
    if !only_linked.is_empty() {
        ui.horizontal_wrapped(|ui| {
            ui.label(
                Type::Small
                    .rich(ui, "Also linked from")
                    .color(colors().muted),
            );
            for source in only_linked {
                if design::link(ui, &source_name(index, item, source)).clicked() {
                    go = Some(Go::Item(source, None));
                }
            }
        });
    }
    go
}

/// What is here, who is part of this, its history, and what other pages say about it.
pub(super) fn related(ui: &mut egui::Ui, item: &Item, index: &ContentIndex) -> Option<Go> {
    let mut go = None;
    let here = index.inside(&item.entry);
    let mut listed = here.clone();
    if !here.is_empty() {
        ui.add_space(design::S);
        style::section_heading(ui, "Here");
        for position in here {
            if page_row(ui, index, position) {
                go = Some(Go::Item(position, None));
            }
        }
    }
    let members = index.members(item.entry.id);
    listed.extend(members);
    if !members.is_empty() {
        ui.add_space(design::S);
        style::section_heading(ui, "Members");
        let mut groups: BTreeMap<(u8, String), Vec<usize>> = BTreeMap::new();
        for &position in members {
            groups
                .entry(page_group(&index.items[position]))
                .or_default()
                .push(position);
        }
        let several = groups.len() > 1;
        for ((_, title), positions) in groups {
            if several {
                design::group_heading(ui, &title, positions.len());
            }
            for position in positions {
                if page_row(ui, index, position) {
                    go = Some(Go::Item(position, None));
                }
            }
        }
    }
    let history = index.history_of(item.entry.id);
    listed.extend(history);
    if !history.is_empty() {
        ui.add_space(design::S);
        style::section_heading(ui, "History");
        if let Some(to) = history_rows(ui, index, history) {
            go = Some(to);
        }
    }
    go.or(from_other_pages(ui, item, index, &listed))
}
