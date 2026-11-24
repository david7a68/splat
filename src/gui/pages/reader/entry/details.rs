//! The common reading layout of entries that are not styled stat blocks (egui
//! `library::reader::show`): an item's summary, a spell's or an encounter's facts, the source
//! text's labelled facts and prose, each section under its heading; and a class's core traits
//! and features (egui `library::class`).
use super::Out;
use super::parts::{detail, facts, identity, item};
use crate::gui::kit;
use crate::gui::theme::{L, M, S, Type, XS, colors};
use crate::library::presentation::{self, ReadingSection};
use crate::library::view::Links;
use crate::library::{class, creature, reader};
use gpui::{IntoElement, ParentElement, Styled, div, px};
use splat::content::Entry;

/// `after_stat_head`: a stat head is drawn just above. Its gap `S` comes before the first prose,
/// and a section heading takes none (the heading owns its space).
#[allow(clippy::too_many_arguments)]
pub(super) fn show(
    out: &mut Out<'_>,
    entry: Option<&Entry>,
    sections: &[ReadingSection<'_>],
    builtin: bool,
    has_summary: bool,
    lead_gap: bool,
    after_stat_head: bool,
) {
    let mut after_stat_head = after_stat_head;
    let mut settle = |out: &mut Out<'_>, heading: bool| {
        if std::mem::take(&mut after_stat_head) && !heading {
            out.push(kit::gap(S));
        }
    };
    let table_stats = entry.is_some_and(|e| ["spell", "item"].contains(&e.kind.as_str()));
    // This reader also receives already-split sections. Restore their headings before
    // filtering, because table ownership is defined by the named section.
    let clean_body = entry.filter(|_| table_stats).map(|entry| {
        let body = sections
            .iter()
            .map(|section| {
                if section.title == "Overview" {
                    section.body.to_owned()
                } else {
                    format!("## {}\n\n{}", section.title, section.body)
                }
            })
            .collect::<Vec<_>>()
            .join("\n\n");
        splat::content::stat_blocks::reading_body(entry, &body)
    });
    let clean_sections;
    let sections = if let Some(body) = &clean_body {
        clean_sections = presentation::reading_sections(body, "");
        clean_sections.as_slice()
    } else {
        sections
    };
    if !has_summary && lead_gap {
        out.push(kit::gap(L));
    }
    if let Some(entry) = entry {
        let printed = sections
            .iter()
            .find(|section| section.title == "Source text")
            .filter(|_| builtin && !table_stats)
            .and_then(|section| {
                reader::source_facts(section.body)
                    .0
                    .into_iter()
                    .find(|(label, _)| label == "Identity")
                    .map(|(_, identity)| identity)
            });
        if let Some(summary) = reader::item_summary_parts(entry, printed.as_deref()) {
            if !summary.identity.is_empty() {
                out.push(identity(summary.identity));
            }
            if let Some(rows) = facts(&summary.facts, out.window) {
                out.push(rows);
            }
            for (label, value) in &summary.details {
                let row = detail(label, value.clone(), out.window);
                out.push(row);
            }
        }
        let (groups, completion) = reader::preparation_parts(
            entry,
            !table_stats && sections.iter().any(|s| s.title == "Source text"),
        );
        for group in &groups {
            if let Some(rows) = facts(group, out.window) {
                out.push(rows);
            }
        }
        if let Some(mode) = completion {
            let row = detail("Completion", mode, out.window);
            out.push(row);
        }
    }
    for section in sections
        .iter()
        .filter(|section| presentation::visible_reading_section(section))
    {
        if section.title == "Source text" && has_summary {
            continue;
        }
        if section.body.contains("<!-- splat:") {
            continue;
        }
        settle(
            out,
            section.title != "Source text" && section.title != "Overview",
        );
        if section.title == "Source text"
            && let Some(entry) = entry.filter(|entry| class::supports(entry))
            && let Some(traits) = class::core_traits(section.body, &entry.name)
        {
            core_traits(out, &traits);
            continue;
        }
        if section.title == "Source text"
            && entry.is_some_and(|e| reader::represented_item_source(e, section.body))
        {
            continue;
        }
        if section.title == "Source text" && (builtin || table_stats) {
            let (mut facts, prose) = reader::source_facts(section.body);
            if table_stats {
                facts.clear();
            }
            if let Some(index) = facts.iter().position(|(label, _)| label == "Identity") {
                let identity_line = facts.remove(index).1;
                // An item's is already its identity line, above.
                if !entry.is_some_and(|e| e.fields.contains_key("splat_item")) {
                    out.push(identity(identity_line));
                }
            }
            if !facts.is_empty() {
                let mut rows = div().flex().flex_col();
                for (label, value) in facts {
                    rows = rows.child(detail(&label, value, out.window));
                }
                out.push(rows);
                out.push(kit::gap(M));
            }
            out.prose(&presentation::overview_markdown(&prose));
        } else {
            // An intro has no printed heading, so the reader adds none.
            if section.title != "Overview" {
                out.heading(section.title.clone());
            }
            // A hand-written stat block's actions read as the book's do.
            if let Some(entry) = entry.filter(|entry| creature::has_stats(entry)) {
                super::stat_block::rules_text(out, &entry.name, &entry.body, section.body);
            } else {
                let id = out.next_id("section");
                let markdown = out.markdown(id, section.body);
                out.push_prose(markdown);
            }
        }
    }
}

/// A class's imported core traits: their facts, the starting equipment's options, then the
/// source after them under its own heading.
fn core_traits(out: &mut Out<'_>, traits: &class::CoreTraits) {
    out.heading("Core traits");
    if let Some(rows) = facts(&traits.facts, out.window) {
        out.push(rows);
    }
    if let Some((words, options)) = &traits.equipment {
        out.heading("Starting equipment");
        for part in words {
            let id = out.next_id("equipment");
            let markdown = out.markdown(id, part);
            out.push_prose(markdown);
        }
        if let Some(rows) = facts(options, out.window) {
            out.push(rows);
        }
    }
    if let Some((title, body)) = &traits.rest {
        out.heading(title);
        let id = out.next_id("class-table");
        let markdown = out.markdown(id, body);
        out.push_prose(markdown);
    }
}

/// A class's or subclass's related tables and its features by level, each a link to its entry.
pub(super) fn class_features(out: &mut Out<'_>, entry: &Entry, links: &dyn Links) {
    let features = class::features(entry, links);
    let related = class::related(entry, links);
    let c = colors();
    if !related.is_empty() {
        out.heading("Related");
        for (at, (target, name)) in related.into_iter().enumerate() {
            let link = out.link(format!("related-{at}"), name, target);
            out.push(item(link));
        }
    }
    if features.is_empty() {
        return;
    }
    out.heading(if class::supports(entry) {
        "Class features"
    } else {
        "Subclass features"
    });
    for (at, feature) in features.into_iter().enumerate() {
        let level = feature
            .level
            .map_or("—".into(), |level| format!("Level {level}"));
        let words = match feature.target {
            Some(target) => out
                .link(format!("feature-{at}"), feature.name.clone(), target)
                .into_any_element(),
            None => kit::text(
                Type::Stat,
                c.muted,
                format!("{} (unavailable)", feature.name),
            )
            .into_any_element(),
        };
        out.push(item(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(XS)
                .child(
                    div()
                        .w(px(60.0))
                        .flex_none()
                        .child(kit::text(Type::Stat, c.muted, level)),
                )
                .child(words),
        ));
    }
}

/// An item's structured details, each under its label: nested values under section headings,
/// plain values as detail rows.
pub(super) fn item_details(out: &mut Out<'_>, entry: &Entry) -> Vec<gpui::AnyElement> {
    let mut sections = Vec::new();
    if let Some(item) = entry.fields.get("splat_item").and_then(|v| v.as_mapping()) {
        for (at, (key, value)) in item.iter().enumerate() {
            let value = reader::remaining_item_detail(entry, key, value);
            if !reader::blank(&value) && (value.is_mapping() || value.is_sequence()) {
                let mut body = Vec::new();
                structured_value(out, &value, &mut body);
                sections.push(
                    kit::QuietSection::new(
                        super::parts::child(&out.id, format!("item-detail-{at}")),
                        crate::design::reader::human_label(&reader::display_value(key)),
                    )
                    .children(body)
                    .into_any_element(),
                );
            }
        }
    }
    sections
}

fn structured_value(out: &Out<'_>, value: &serde_yaml_ng::Value, body: &mut Vec<gpui::AnyElement>) {
    if let Some(fields) = value.as_mapping() {
        for (key, value) in fields {
            let key = reader::display_value(key);
            if value.is_mapping() || value.is_sequence() {
                body.push(
                    super::parts::section_heading(crate::design::reader::human_label(&key), false)
                        .into_any_element(),
                );
                structured_value(out, value, body);
            } else {
                body.push(
                    detail(&key, reader::display_value(value), out.window).into_any_element(),
                );
            }
        }
    } else if let Some(values) = value.as_sequence() {
        for value in values {
            structured_value(out, value, body);
        }
    } else {
        body.push(
            item(kit::text(
                Type::Body,
                colors().text,
                reader::display_value(value),
            ))
            .into_any_element(),
        );
    }
}
