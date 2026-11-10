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

/// A page's name as a link, then what it is, in words that wrap.
fn page_row(
    ui: &mut egui::Ui,
    index: &ContentIndex,
    position: usize,
    before: Option<&str>,
) -> bool {
    let item = &index.items[position];
    let detail = match item.summary() {
        "" if item.entry.kind == "world" => type_label(world_type(&item.entry)),
        summary => summary.to_owned(),
    };
    ui.horizontal_wrapped(|ui| {
        if let Some(before) = before {
            ui.label(Type::Small.rich(ui, before).color(colors().muted));
        }
        let clicked = design::link(ui, &item.entry.name).clicked();
        if !detail.is_empty() {
            ui.label(Type::Small.rich(ui, detail).color(colors().muted));
        }
        clicked
    })
    .inner
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
    style::section_heading(ui, "From other pages");
    for ((_, title), pages) in &groups {
        let count: usize = pages.iter().map(|(_, facts)| facts.len()).sum();
        egui::CollapsingHeader::new(design::group_heading_text(ui, title, count))
            .id_salt(("lines_about", item.entry.id, title))
            .default_open(count <= OPEN_LINES)
            .icon(design::section_icon)
            .show(ui, |ui| {
                for (source, facts) in pages {
                    ui.push_id(source, |ui| {
                        if design::link(ui, &index.items[*source].entry.name).clicked() {
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
                if design::link(ui, &index.items[source].entry.name).clicked() {
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
            if page_row(ui, index, position, None) {
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
                if page_row(ui, index, position, None) {
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
        for &position in history {
            let date = date_words(&index.items[position].entry);
            if page_row(ui, index, position, date.as_deref()) {
                go = Some(Go::Item(position, None));
            }
        }
    }
    go.or(from_other_pages(ui, item, index, &listed))
}
