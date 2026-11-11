//! The loot list editor, for an encounter and for one creature's Changes. Each line is a row.
//! The add field takes the typed words with Enter, and offers the campaign's roll tables and
//! Library items while the DM types. A roll names its table from a list, so a key that no
//! table has shows on its row before play.
use crate::design::{self, Density, Status, Tone, Type, colors, icons};
use crate::roll_table::{Table, TableRoll};
use eframe::egui;
use splat::content::loot::{Loot, MAX_ROLLS, SEPARATOR};
use std::sync::Arc;

/// An item from the Library that a loot line can link to.
#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    pub name: String,
    pub lower: String,
    /// The line a loot list keeps: a link that Splat and Obsidian both follow.
    pub link: String,
    pub detail: String,
}

/// What the editor can offer. Built once for each load of the campaign's index.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Sources {
    pub tables: Vec<Table>,
    pub items: Vec<Item>,
}

/// The most suggestions of each kind under the add field.
const MOST_TABLES: usize = 4;
const MOST_ITEMS: usize = 6;
const NO_SEPARATOR: &str = "Use one row for each item. A ; separates items in the roster table.";

impl Sources {
    /// The marked tables in `(path, body)` notes, in that order. The first table with a key
    /// decides it, so a bad one says why and no later table takes its place by surprise.
    pub fn tables_from<'a>(notes: impl IntoIterator<Item = (&'a str, &'a str)>) -> Vec<Table> {
        let mut tables = crate::roll_table::tables_in(notes);
        tables.sort_by(|a, b| a.key.cmp(&b.key));
        tables
    }

    /// A roll on the table with this key, or why there can be none.
    pub fn roll(&self, key: &str) -> Result<TableRoll, String> {
        if let Some(problem) = self.problem(key) {
            return Err(problem);
        }
        let table = self.table(key).ok_or("No roll table has this key.")?;
        Ok(TableRoll {
            key: table.key.clone(),
            dice: table.dice.clone(),
            typed: None,
            rows: table.rows.clone(),
        })
    }

    pub fn table(&self, key: &str) -> Option<&Table> {
        self.tables.iter().find(|table| table.key == key)
    }

    /// Why a roll on this key cannot happen, or nothing.
    pub fn problem(&self, key: &str) -> Option<String> {
        match self.table(key) {
            None => Some(format!("No roll table has the key “{key}”.")),
            Some(table) => table.fault(),
        }
    }

    /// The keys among these lines that cannot roll.
    pub fn missing<'a>(&self, lines: impl IntoIterator<Item = &'a str>) -> Vec<String> {
        let mut keys = Vec::new();
        for line in lines {
            if let Loot::Roll { key, .. } = Loot::parse(line)
                && self.problem(&key).is_some()
                && !keys.contains(&key)
            {
                keys.push(key);
            }
        }
        keys
    }

    /// Tables, then items, that match what the DM typed. `Roll` before a word looks for tables
    /// only.
    fn matching(&self, typed: &str) -> (Vec<&Table>, Vec<&Item>) {
        let lower = typed.trim().to_lowercase();
        let (tables_only, query) = match lower.strip_prefix("roll") {
            Some(rest) if rest.is_empty() || rest.starts_with(' ') => {
                let rest = rest.trim();
                // `roll 2× gems` looks for `gems`.
                let rest = rest
                    .split_once(' ')
                    .filter(|(count, _)| count.trim_end_matches(['×', 'x']).parse::<u32>().is_ok())
                    .map_or(rest, |(_, key)| key.trim_start_matches(['×', 'x', ' ']));
                (true, rest.to_owned())
            }
            _ => (false, lower),
        };
        let tables = self
            .tables
            .iter()
            .filter(|table| table.key.to_lowercase().contains(&query))
            .take(MOST_TABLES)
            .collect();
        let items = if tables_only || query.is_empty() {
            Vec::new()
        } else {
            let mut found: Vec<(u8, &Item)> = self
                .items
                .iter()
                .filter_map(|item| {
                    crate::matching::name_rank(&item.lower, &query).map(|rank| (rank, item))
                })
                .collect();
            found.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.name.cmp(&b.1.name)));
            found
                .into_iter()
                .take(MOST_ITEMS)
                .map(|(_, item)| item)
                .collect()
        };
        (tables, items)
    }
}

/// The campaign's sources, from the index every page shares. The index keeps them, so they
/// are read once for each load of the campaign.
pub fn sources(ctx: &egui::Context) -> Option<Arc<Sources>> {
    ctx.data(|data| {
        data.get_temp::<Arc<crate::lookup::ContentIndex>>(egui::Id::new("reader_link_index"))
    })
    .map(|index| index.loot_sources())
}

/// The table keys in one creature's loot that cannot roll, when the sources are known.
pub fn missing_in(ctx: &egui::Context, loot: Option<&str>) -> Vec<String> {
    match (sources(ctx), loot) {
        (Some(sources), Some(loot)) => {
            let parts = splat::content::loot::parts(loot);
            sources.missing(parts.iter().map(String::as_str))
        }
        _ => Vec::new(),
    }
}

/// The words a link line shows: `[[Items/Potion|Potion of Healing]]` reads `Potion of Healing`.
pub fn shown(line: &str) -> Option<&str> {
    let line = line.trim();
    if let Some(inner) = line.strip_prefix("[[").and_then(|l| l.strip_suffix("]]")) {
        return Some(inner.rsplit('|').next().unwrap_or(inner));
    }
    let inner = line.strip_prefix('[')?;
    let (name, rest) = inner.split_once("](")?;
    rest.ends_with(')').then_some(name)
}

/// Where a wiki link line goes: `[[Items/Potion|Potion of Healing]]` goes to `Items/Potion`.
pub fn link_target(line: &str) -> Option<&str> {
    let inner = line.trim().strip_prefix("[[")?.strip_suffix("]]")?;
    Some(inner.split('|').next().unwrap_or(inner))
}

/// The line in plain words, for a summary.
pub fn plain(line: &str) -> String {
    match Loot::parse(line) {
        Loot::Roll { times: 1, key } => format!("a roll on {key}"),
        Loot::Roll { times, key } => format!("{times} rolls on {key}"),
        Loot::Text(text) => shown(&text).unwrap_or(&text).to_owned(),
    }
}

/// What the editor keeps between frames.
#[derive(Clone, Default)]
struct State {
    add: design::AddField,
    /// The line being typed in, which keeps its field until it loses focus.
    editing: Option<usize>,
    /// A count being typed, by line.
    count: Option<(usize, String)>,
    removed: Option<(usize, String)>,
    reason: Option<String>,
}

/// One pick from the suggestions or the table menu.
enum Pick {
    Table(String),
    Item(String),
}

/// One creature's loot, as its Changes keep it: one line, parts separated by `;`. The rows
/// keep what the DM types, spaces and empty rows included, until it changes the loot.
pub fn show_cell(
    ui: &mut egui::Ui,
    id: impl std::hash::Hash,
    name: &str,
    loot: &mut Option<String>,
) -> bool {
    let id = egui::Id::new(("loot_cell", id));
    let parts = splat::content::loot::parts(loot.as_deref().unwrap_or(""));
    let mut lines: Vec<String> = ui
        .data(|data| data.get_temp::<Vec<String>>(id))
        .filter(|kept| splat::content::loot::parts(&kept.join(";")) == parts)
        .unwrap_or_else(|| parts.clone());
    let changed = show(ui, id, name, &mut lines, true);
    if changed {
        let joined = splat::content::loot::parts(&lines.join(";")).join("; ");
        *loot = (!joined.is_empty()).then_some(joined);
    }
    ui.data_mut(|data| data.insert_temp(id, lines));
    changed
}

/// The list and its add field. `id` keeps one list's typing apart from another's. `one_cell`
/// is for a creature, whose loot is one table cell, so a line cannot hold `;`. Returns whether
/// the lines changed.
pub fn show(
    ui: &mut egui::Ui,
    id: impl std::hash::Hash,
    name: &str,
    lines: &mut Vec<String>,
    one_cell: bool,
) -> bool {
    let id = egui::Id::new(("loot_editor", id));
    let mut state: State = ui.data(|data| data.get_temp(id)).unwrap_or_default();
    let sources = sources(ui.ctx());
    let before = lines.clone();
    let mut remove = None;
    for at in 0..lines.len() {
        if let Some(gone) = line_row(
            ui,
            name,
            lines,
            at,
            &mut state,
            sources.as_deref(),
            one_cell,
        ) {
            remove = Some(gone);
        }
    }
    if let Some(at) = remove {
        let line = lines.remove(at);
        // A row emptied by hand is gone with nothing to bring back.
        state.removed = (!line.trim().is_empty()).then_some((at, line));
        state.editing = None;
        state.count = None;
    }
    if let Some((at, line)) = state.removed.clone() {
        let words = format!("Removed {}.", plain(&line));
        if design::undo_message(ui, Tone::Info, &words, "Undo remove loot") {
            lines.insert(at.min(lines.len()), line);
            state.removed = None;
        }
    }
    add_row(ui, name, lines, &mut state, sources.as_deref(), one_cell);
    ui.data_mut(|data| data.insert_temp(id, state));
    *lines != before
}

/// One line. Returns its place when the DM removes it.
fn line_row(
    ui: &mut egui::Ui,
    name: &str,
    lines: &mut [String],
    at: usize,
    state: &mut State,
    sources: Option<&Sources>,
    one_cell: bool,
) -> Option<usize> {
    let c = colors();
    let mut remove = false;
    let mut reason = None;
    let mut key_left = 0.0;
    let menu = design::XL + design::S;
    let row = ui.horizontal(|ui| {
        let line = lines[at].clone();
        let editing = state.editing == Some(at);
        match Loot::parse(&line) {
            Loot::Roll { times, key } if !editing => {
                ui.label(Type::Body.rich(ui, icons::DICE).color(c.muted));
                ui.label(Type::Body.rich(ui, "Roll").color(c.text));
                let mut count = match &state.count {
                    Some((place, typed)) if *place == at => typed.clone(),
                    _ => times.to_string(),
                };
                let response = design::number_field(
                    ui,
                    &format!("Rolls on {key}, {name} loot line {}", at + 1),
                    &mut count,
                    "1",
                    design::XXL + design::S,
                    Density::Read.control(),
                );
                if response.changed() {
                    if let Ok(times) = count.trim().parse::<u32>()
                        && (1..=MAX_ROLLS).contains(&times)
                    {
                        lines[at] = Loot::Roll {
                            times,
                            key: key.clone(),
                        }
                        .line();
                    }
                    state.count = Some((at, count));
                } else if !response.has_focus()
                    && state.count.as_ref().is_some_and(|(p, _)| *p == at)
                {
                    state.count = None;
                }
                ui.label(Type::Body.rich(ui, "×").color(c.muted));
                key_left = ui.cursor().left();
                let picked = table_menu(
                    ui,
                    &key,
                    &format!("Table for {name} loot line {}", at + 1),
                    sources,
                );
                if let Some(chosen) = picked {
                    lines[at] = Loot::Roll { times, key: chosen }.line();
                }
                match sources.map(|sources| (sources.problem(&key), sources.table(&key))) {
                    Some((Some(problem), _)) => {
                        design::status(
                            ui,
                            Status {
                                icon: icons::WARN,
                                word: "Cannot roll",
                                color: c.result.warning,
                            },
                        );
                        reason = Some(problem);
                    }
                    Some((None, Some(table))) => {
                        ui.label(
                            Type::Small
                                .rich(ui, format!("{} · {}", table.dice, table.path))
                                .color(c.muted),
                        );
                    }
                    _ => {}
                }
            }
            Loot::Text(text) if !editing && shown(&text).is_some() => {
                ui.label(Type::Body.rich(ui, icons::LIBRARY).color(c.muted));
                let name = shown(&text).unwrap_or(&text);
                match link_target(&text) {
                    // As a creature's name does, the item's name opens it.
                    Some(target) => {
                        if design::link(ui, name).clicked() {
                            crate::markdown_view::send_note_link(ui.ctx(), target.to_owned(), None);
                        }
                    }
                    None => {
                        ui.label(Type::Body.rich(ui, name).color(c.text));
                    }
                }
                ui.label(Type::Small.rich(ui, "Library item").color(c.muted));
            }
            _ => {
                let mut words = line.clone();
                let width = (ui.available_width() - menu).max(120.0);
                let response = design::text_input(
                    ui,
                    &format!("{name} loot line {}", at + 1),
                    &mut words,
                    "What the party finds",
                    width,
                );
                if response.has_focus() {
                    state.editing = Some(at);
                } else if editing {
                    state.editing = None;
                }
                if response.changed() {
                    if one_cell && words.contains(SEPARATOR) {
                        state.reason = Some(NO_SEPARATOR.into());
                    } else {
                        lines[at] = words;
                        state.reason = None;
                    }
                }
                if response.lost_focus() && lines[at].trim().is_empty() {
                    remove = true;
                }
            }
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            design::row_menu(ui, &format!("{name} loot line {}", at + 1), |ui| {
                if ui.button("Remove").clicked() {
                    remove = true;
                    ui.close();
                }
            });
        });
    });
    // Below the row, under the key it is about, so the row stays one line.
    if let Some(reason) = reason {
        ui.horizontal(|ui| {
            ui.add_space((key_left - row.response.rect.left()).max(0.0));
            ui.add(egui::Label::new(Type::Small.rich(ui, reason).color(colors().muted)).wrap());
        });
    }
    remove.then_some(at)
}

/// The quiet menu of the campaign's tables, with the caret after the word. Returns a choice.
fn table_menu(
    ui: &mut egui::Ui,
    word: &str,
    name: &str,
    sources: Option<&Sources>,
) -> Option<String> {
    let response = design::menu_button(ui, word, false);
    design::named(ui, &response, name);
    let mut chosen = None;
    egui::Popup::menu(&response).show(|ui| {
        let tables = sources.map_or(&[][..], |sources| &sources.tables[..]);
        if tables.is_empty() {
            ui.add(egui::Label::new(Type::Small.rich(ui, NO_TABLES).color(colors().muted)).wrap());
        }
        for table in tables {
            if ui
                .button(format!("{} · {} · {}", table.key, table.dice, table.path))
                .clicked()
            {
                chosen = Some(table.key.clone());
                ui.close();
            }
        }
    });
    chosen
}

const NO_TABLES: &str =
    "No roll tables yet. Put <!-- splat:rolls key=gems dice=1d6 --> above a table in a World note.";

/// The field that adds a line, the table menu beside it, and the suggestions under it.
fn add_row(
    ui: &mut egui::Ui,
    name: &str,
    lines: &mut Vec<String>,
    state: &mut State,
    sources: Option<&Sources>,
    one_cell: bool,
) {
    // The gap is the row's item spacing, so the menu ends at the column's right edge.
    let menu = design::menu_button_width(ui, ROLL_MENU) + ui.spacing().item_spacing.x;
    let added = design::add_field(
        ui,
        &mut state.add,
        &format!("Add loot to {name}"),
        "Type loot, or find an item or table",
        menu,
        |ui| {
            table_menu(ui, ROLL_MENU, &format!("{ROLL_MENU} for {name}"), sources).map(Pick::Table)
        },
        |typed| {
            let Some(sources) = sources else {
                return Vec::new();
            };
            let (tables, items) = sources.matching(typed);
            let times = typed_times(typed);
            vec![
                design::Suggestions {
                    label: "Roll tables",
                    rows: tables
                        .into_iter()
                        .map(|table| {
                            let title = if times == 1 {
                                table.key.clone()
                            } else {
                                format!("{times} × {}", table.key)
                            };
                            let detail = table
                                .fault()
                                .unwrap_or_else(|| format!("{} · {}", table.dice, table.path));
                            (title, detail, Pick::Table(table.key.clone()))
                        })
                        .collect(),
                },
                design::Suggestions {
                    label: "Items",
                    rows: items
                        .into_iter()
                        .map(|item| {
                            (
                                item.name.clone(),
                                item.detail.clone(),
                                Pick::Item(item.link.clone()),
                            )
                        })
                        .collect(),
                },
            ]
        },
    );
    if added.field.changed() {
        state.reason = None;
    }
    let typed = state.add.words.trim().to_owned();
    let line = match added.chosen {
        Some(Pick::Table(key)) => Some(
            Loot::Roll {
                times: typed_times(&typed),
                key,
            }
            .line(),
        ),
        Some(Pick::Item(link)) => Some(link),
        None if added.enter && too_many(&typed) => {
            state.reason = Some(format!("One line rolls at most {MAX_ROLLS} times."));
            None
        }
        None if added.enter && !typed.is_empty() => Some(typed),
        None => None,
    };
    if let Some(line) = line {
        if one_cell && line.contains(SEPARATOR) {
            state.reason = Some(NO_SEPARATOR.into());
        } else {
            lines.push(line);
            state.add.clear();
            state.reason = None;
            state.removed = None;
        }
        // The field stays open after Enter, because several lines often follow each other.
        state.add.focus = added.enter;
    }
    if let Some(reason) = &state.reason {
        design::blocked_reason(ui, Some(reason));
    }
}

pub(crate) const ROLL_MENU: &str = "Roll on a table";

/// The count typed after `Roll`, such as 2 in `Roll 2× ge`.
fn typed_count(typed: &str) -> Option<u32> {
    let mut words = typed.split_whitespace();
    let roll = words
        .next()
        .is_some_and(|word| word.eq_ignore_ascii_case("roll"));
    words
        .next()
        .filter(|_| roll)
        .and_then(|word| word.trim_end_matches(['×', 'x', 'X']).parse().ok())
}

/// The count to keep when the DM picks a table, at most `MAX_ROLLS`.
fn typed_times(typed: &str) -> u32 {
    typed_count(typed).map_or(1, |times| times.clamp(1, MAX_ROLLS))
}

/// A roll with a count above what one line can hold.
fn too_many(typed: &str) -> bool {
    typed_count(typed).is_some_and(|times| times > MAX_ROLLS)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sources() -> Sources {
        Sources {
            tables: vec![
                Table {
                    key: "gems".into(),
                    dice: "1d4".into(),
                    path: "Treasure.md".into(),
                    heading: None,
                    problem: None,
                    rows: Vec::new(),
                },
                Table {
                    key: "broken".into(),
                    dice: "1d6".into(),
                    path: "Treasure.md".into(),
                    heading: None,
                    problem: Some("1d6 needs a row for 3.".into()),
                    rows: Vec::new(),
                },
            ],
            items: vec![Item {
                name: "Potion of Healing".into(),
                lower: "potion of healing".into(),
                link: "[[Potion of Healing]]".into(),
                detail: "Potion".into(),
            }],
        }
    }

    #[test]
    fn typed_words_find_tables_and_items_and_roll_finds_tables_only() {
        let sources = sources();
        let (tables, items) = sources.matching("po");
        assert!(tables.is_empty());
        assert_eq!(items[0].name, "Potion of Healing");
        let (tables, items) = sources.matching("Roll 2× ge");
        assert_eq!(tables[0].key, "gems");
        assert!(items.is_empty());
        let (tables, _) = sources.matching("roll");
        assert_eq!(tables.len(), 2);
    }

    #[test]
    fn a_key_with_no_table_or_a_bad_table_cannot_roll() {
        let sources = sources();
        assert_eq!(
            sources.missing([
                "Roll gems",
                "Roll 2× coins",
                "Roll broken",
                "30 gp",
                "Roll coins"
            ]),
            ["coins", "broken"]
        );
        assert!(sources.problem("coins").unwrap().contains("No roll table"));
    }

    #[test]
    fn the_first_table_with_a_key_decides_it_and_a_bad_one_says_why() {
        let good = "<!-- splat:rolls key=gems dice=1d2 -->\n\n| d2 | Gem |\n|---|---|\n| 1 | Ruby |\n| 2 | Opal |\n";
        let bad = good.replace("| 2 | Opal |\n", "");
        let sources = Sources {
            tables: Sources::tables_from([("A.md", good), ("B.md", bad.as_str())]),
            items: Vec::new(),
        };
        let roll = sources.roll("gems").unwrap();
        assert_eq!((roll.dice.as_str(), roll.rows.len()), ("1d2", 2));
        let sources = Sources {
            tables: Sources::tables_from([("B.md", bad.as_str()), ("A.md", good)]),
            items: Vec::new(),
        };
        assert!(sources.roll("gems").unwrap_err().starts_with("B.md: "));
        assert!(sources.roll("coins").unwrap_err().contains("key “coins”."));
    }

    #[test]
    fn a_creatures_line_keeps_a_typed_space_and_an_emptied_row_leaves_the_rest() {
        use egui_kittest::kittest::Queryable;
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(640.0, 400.0))
            .build_state(
                |ctx, loot: &mut Option<String>| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        show_cell(ui, "goblin", "Goblin", loot);
                    });
                },
                Some("Iron; Roll gems".to_owned()),
            );
        crate::design::theme::install(&harness.ctx);
        harness.run_steps(2);
        harness.get_by_label("Goblin loot line 1").focus();
        harness.run_steps(1);
        harness.key_press(egui::Key::End);
        for typed in [" ", "key"] {
            harness.get_by_label("Goblin loot line 1").type_text(typed);
            harness.run_steps(2);
        }
        assert_eq!(harness.state().as_deref(), Some("Iron key; Roll gems"));
        // The cell never holds an empty part, and the row stays while it has focus.
        harness.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::A);
        harness.key_press(egui::Key::Backspace);
        harness.run_steps(2);
        assert_eq!(harness.state().as_deref(), Some("Roll gems"));
        assert!(harness.query_by_label("Goblin loot line 1").is_some());
    }

    #[test]
    fn link_lines_show_their_names() {
        assert_eq!(
            shown("[[Items/Potion|Potion of Healing]]"),
            Some("Potion of Healing")
        );
        assert_eq!(shown("[[Potion]]"), Some("Potion"));
        assert_eq!(
            shown("[Potion](splat:dnd2024:items/potion)"),
            Some("Potion")
        );
        assert_eq!(shown("30 gp"), None);
        assert_eq!(
            link_target("[[Items/Potion|Potion of Healing]]"),
            Some("Items/Potion")
        );
        assert_eq!(link_target("[Potion](splat:dnd2024:items/potion)"), None);
        assert_eq!(plain("Roll 2× gems"), "2 rolls on gems");
        assert_eq!(plain("[[Potion]]"), "Potion");
        assert_eq!(typed_times("Roll 2× ge"), 2);
        assert_eq!(typed_times("roll 3x"), 3);
        assert_eq!(typed_times("gems"), 1);
        assert_eq!(typed_times("Roll 99 gems"), MAX_ROLLS);
        assert!(too_many("Roll 99 gems") && !too_many("Roll 2 gems") && !too_many("gems"));
    }
}
