//! What other pages say about the one on show. A fact has one home, and these lists bring it
//! here: what is in this place, what is part of this group, the events that concern it, and
//! each line elsewhere that names it. See `docs/design/world-data.md`.
#![warn(deprecated)] // Its parent allows old message functions; this file does not.
use super::{Role, date_words, lines::Line, type_group, type_label, world_type};
use crate::lookup::{ContentIndex, Item, Mention};
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

/// A source note's name: a session's as the Sessions page gives it (`ContentIndex::shown_name`).
/// A copy of this note, or any file with its name, is named by its path, as search rows name
/// duplicates.
fn source_name(index: &ContentIndex, item: &Item, source: usize) -> String {
    let entry = &index.items[source].entry;
    if entry.name.eq_ignore_ascii_case(&item.entry.name)
        || index.duplicate_world.contains_key(&entry.path)
    {
        entry.path.clone()
    } else {
        index.name_of(&index.items[source]).to_owned()
    }
}

/// One page in a related list, as data: its place, name and what it is.
pub struct PageFacts {
    pub position: usize,
    pub name: String,
    pub detail: String,
}

/// A line from another page: its label, its words and the heading it opens at.
pub struct LineFacts {
    pub label: String,
    pub text: String,
    pub heading: Option<String>,
}

/// One kind of page under From other notes: its title, its count of lines, whether it starts
/// open, and each page with its lines and the count left out.
pub struct FromGroup {
    pub title: String,
    pub count: usize,
    pub open: bool,
    pub pages: Vec<(usize, String, Vec<LineFacts>, usize)>,
}

/// What the reader shows of related notes, as data.
#[derive(Default)]
pub struct RelatedFacts {
    pub here: Vec<PageFacts>,
    /// Members by kind; the kind's title only when there are several kinds.
    pub members: Vec<(Option<String>, Vec<PageFacts>)>,
    pub history: Vec<(Option<String>, PageFacts)>,
    pub from: Vec<FromGroup>,
    pub also: Vec<(usize, String)>,
}

fn page_facts(index: &ContentIndex, position: usize) -> PageFacts {
    let item = &index.items[position];
    let detail = match item.summary() {
        "" if item.entry.kind == "world" => type_label(world_type(&item.entry)),
        summary => summary.to_owned(),
    };
    PageFacts {
        position,
        name: index.name_of(item).to_owned(),
        detail,
    }
}

/// What is here, who is part of this, its history, and what other pages say about it, as data.
pub fn related_facts(item: &Item, index: &ContentIndex) -> RelatedFacts {
    let mut facts = RelatedFacts::default();
    let here = index.inside(&item.entry);
    let mut listed = here.clone();
    facts.here = here.iter().map(|&at| page_facts(index, at)).collect();
    let members = index.members(item.entry.id);
    listed.extend(members);
    let mut groups: BTreeMap<(u8, String), Vec<usize>> = BTreeMap::new();
    for &position in members {
        groups
            .entry(page_group(&index.items[position]))
            .or_default()
            .push(position);
    }
    let several = groups.len() > 1;
    facts.members = groups
        .into_iter()
        .map(|((_, title), positions)| {
            (
                several.then_some(title),
                positions.iter().map(|&at| page_facts(index, at)).collect(),
            )
        })
        .collect();
    let history = index.history_of(item.entry.id);
    listed.extend(history);
    facts.history = history
        .iter()
        .map(|&at| (date_words(&index.items[at].entry), page_facts(index, at)))
        .collect();
    let mentions = index.lines_about(item.entry.id);
    if mentions.is_empty() {
        return facts;
    }
    let mut names = vec![item.entry.name.to_lowercase()];
    names.extend(super::aliases(&item.entry).iter().map(|a| a.to_lowercase()));
    type Pages<'a> = Vec<(usize, Vec<&'a Line>)>;
    let mut groups: BTreeMap<(u8, String), Pages> = BTreeMap::new();
    for page in mentions.chunk_by(|a, b| a.source == b.source) {
        let source = page[0].source;
        let said = listed.contains(&source);
        let summary = index.items[source].summary();
        let lines: Vec<&Line> = page
            .iter()
            .filter_map(|&Mention { line, .. }| Some(&index.items[source].lines[line?]))
            .filter(|line| !line.bare)
            .filter(|line| !(said && line.label.is_none() && line.text == summary))
            .collect();
        if lines.is_empty() {
            if !said {
                facts.also.push((source, source_name(index, item, source)));
            }
        } else {
            groups
                .entry(page_group(&index.items[source]))
                .or_default()
                .push((source, lines));
        }
    }
    facts.from = groups
        .into_iter()
        .map(|((_, title), pages)| {
            let count: usize = pages.iter().map(|(_, lines)| lines.len()).sum();
            FromGroup {
                title,
                count,
                open: count <= OPEN_LINES,
                pages: pages
                    .into_iter()
                    .map(|(source, lines)| {
                        let more = lines.len().saturating_sub(LINES_PER_PAGE);
                        let shown = lines
                            .iter()
                            .take(LINES_PER_PAGE)
                            .map(|line| {
                                let words = line.snippet(&names, LINE_WIDTH);
                                let (label, text) = match &line.label {
                                    Some(label) if words.starts_with(&format!("{label}:")) => {
                                        let (label, text) = words.split_at(label.len() + 1);
                                        (label.to_owned(), text.to_owned())
                                    }
                                    _ => (String::new(), words.clone()),
                                };
                                LineFacts {
                                    label,
                                    text,
                                    heading: line.heading.clone(),
                                }
                            })
                            .collect();
                        (source, source_name(index, item, source), shown, more)
                    })
                    .collect(),
            }
        })
        .collect();
    facts
}
