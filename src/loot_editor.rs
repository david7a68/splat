//! The loot list editor, for an encounter and for one creature's Changes. Each line is a row.
//! The add field takes the typed words with Enter, and offers the campaign's roll tables and
//! Library items while the DM types. A roll names its table from a list, so a key that no
//! table has shows on its row before play.
#![allow(deprecated)] // Old message functions; Wave 2 moves this file to `design::outcome`.
use crate::design::{self, Density, Status, Tone, Type, colors, icons};
use crate::roll_table::Table;
use eframe::egui;
use splat::content::loot::{Loot, MAX_ROLLS, SEPARATOR};
use std::sync::Arc;

pub use splat_campaign::loot::{Item, Sources};

/// The most suggestions of each kind under the add field.
pub const MOST_TABLES: usize = 4;
const MOST_ITEMS: usize = 6;
pub const NO_SEPARATOR: &str = "Use one row for each item. A ; separates items in the roster table.";

/// Tables, then items, that match what the DM typed. `Roll` before a word looks for tables
/// only.
pub fn matching<'a>(sources: &'a Sources, typed: &str) -> (Vec<&'a Table>, Vec<&'a Item>) {
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
    let tables = sources
        .tables
        .iter()
        .filter(|table| table.key.to_lowercase().contains(&query))
        .take(MOST_TABLES)
        .collect();
    let items = if tables_only || query.is_empty() {
        Vec::new()
    } else {
        let mut found: Vec<(u8, &Item)> = sources
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
/// keep what the DM types, spaces and empty rows included, until it changes the loot. `owner` is
/// the Changes form's row (its typed key, issue #559).
pub fn show_cell(
    ui: &mut egui::Ui,
    owner: impl design::RowKey,
    name: &str,
    loot: &mut Option<String>,
) -> bool {
    let id = egui::Id::new(("loot_cell", &owner));
    let parts = splat::content::loot::parts(loot.as_deref().unwrap_or(""));
    let mut lines: Vec<String> = ui
        .data(|data| data.get_temp::<Vec<String>>(id))
        .filter(|kept| splat::content::loot::parts(&kept.join(";")) == parts)
        .unwrap_or_else(|| parts.clone());
    let changed = show(ui, owner, name, &mut lines, true);
    if changed {
        let joined = splat::content::loot::parts(&lines.join(";")).join("; ");
        *loot = (!joined.is_empty()).then_some(joined);
    }
    ui.data_mut(|data| data.insert_temp(id, lines));
    changed
}

/// The list and its add field. `owner` (an encounter draft's id, a creature's Changes row) keys
/// the list's fields, so one list's typing stays apart from another's (issue #559). `one_cell`
/// is for a creature, whose loot is one table cell, so a line cannot hold `;`. Returns whether
/// the lines changed.
pub fn show(
    ui: &mut egui::Ui,
    owner: impl design::RowKey,
    name: &str,
    lines: &mut Vec<String>,
    one_cell: bool,
) -> bool {
    let id = egui::Id::new(("loot_editor", &owner));
    let mut state: State = ui.data(|data| data.get_temp(id)).unwrap_or_default();
    let sources = sources(ui.ctx());
    let before = lines.clone();
    let mut remove = None;
    for at in 0..lines.len() {
        if let Some(gone) = line_row(
            ui,
            &owner,
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
        // A removal is a finished change: the success tone, as every finished change has.
        if design::undo_message(ui, Tone::Success, &words, "Undo remove loot") {
            lines.insert(at.min(lines.len()), line);
            state.removed = None;
        }
    }
    add_row(
        ui,
        &owner,
        name,
        lines,
        &mut state,
        sources.as_deref(),
        one_cell,
    );
    ui.data_mut(|data| data.insert_temp(id, state));
    *lines != before
}

const LOOT_LINE: design::FieldName = crate::field_id!("loot_line");
/// The list's own add field (keys only).
const LOOT_ADD: design::FieldName = crate::field_id!("loot_add");

/// The add field of `owner`'s list, keyed by the owner itself (issue #564: by its whole key, not
/// a 16-bit `Debug` text of an egui id, so two lists never share the field's text history).
fn add_field(owner: impl design::RowKey) -> design::FieldId {
    LOOT_ADD.of(owner)
}

/// One line. Returns its place when the DM removes it.
#[allow(clippy::too_many_arguments)]
fn line_row(
    ui: &mut egui::Ui,
    owner: &impl design::RowKey,
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
                // The field belongs to this list (its caller's id) and, by its label, to its line.
                let response = design::line_field(
                    ui,
                    LOOT_LINE.of(owner),
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
                if design::Action::quiet("Remove")
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                {
                    remove = true;
                    ui.close();
                }
            });
        });
    });
    // N12: a list row, so a footer never cuts through it.
    design::register_row(ui, row.response.rect);
    // Below the row, under the key it is about, so the row stays one line.
    if let Some(reason) = reason {
        // The reason's column starts where the key's does (the row's item spacing after it, as
        // the words started when a space of the key's offset came first).
        let mut under = ui.available_rect_before_wrap();
        under.min.x = (key_left + ui.spacing().item_spacing.x).max(under.min.x);
        ui.scope_builder(
            egui::UiBuilder::new()
                .max_rect(under)
                .layout(egui::Layout::top_down(egui::Align::Min)),
            |ui| {
                ui.add(egui::Label::new(Type::Small.rich(ui, reason).color(colors().muted)).wrap());
            },
        );
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
    crate::design::menu_popup(&*ui, &response).show(|ui| {
        let tables = sources.map_or(&[][..], |sources| &sources.tables[..]);
        if tables.is_empty() {
            // The menu's note, where its choices' words start (M-P0.26).
            design::choice_note(ui, false, Some(NO_TABLES));
        }
        for table in tables {
            if design::Action::quiet(&format!("{} · {} · {}", table.key, table.dice, table.path))
                .density(Density::Read)
                .show(ui)
                .clicked()
            {
                chosen = Some(table.key.clone());
                ui.close();
            }
        }
    });
    chosen
}

pub const NO_TABLES: &str =
    "No roll tables yet. Put <!-- splat:rolls key=gems dice=1d6 --> above a table in a World note.";

/// The field that adds a line, the table menu beside it, and the suggestions under it.
#[allow(
    clippy::disallowed_methods,
    reason = "staged: the loot list’s own field stays open and adds with Enter, as the design system describes it (design::results::STAGED_ALLOWS)"
)]
fn add_row(
    ui: &mut egui::Ui,
    owner: &impl design::RowKey,
    name: &str,
    lines: &mut Vec<String>,
    state: &mut State,
    sources: Option<&Sources>,
    one_cell: bool,
) {
    // The gap is the row's item spacing, so the menu ends at the column's right edge.
    let menu = design::menu_button_width(ui, ROLL_MENU) + ui.spacing().item_spacing.x;
    // The list's own field (M0404), keyed by the list's owner, as its lines are (`line_row`).
    let field = add_field(owner);
    let label = format!("Add loot to {name}");
    let menu_name = format!("{ROLL_MENU} for {name}");
    let added = design::add_field(field, &label, "Type loot, or find an item or table")
        .suggest(|typed| {
            let Some(sources) = sources else {
                return Vec::new();
            };
            let (tables, items) = matching(sources, typed);
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
        })
        .keys_only(menu, |ui| {
            table_menu(ui, ROLL_MENU, &menu_name, sources).map(Pick::Table)
        })
        .show(ui, &mut state.add);
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

pub const ROLL_MENU: &str = "Roll on a table";

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
pub fn typed_times(typed: &str) -> u32 {
    typed_count(typed).map_or(1, |times| times.clamp(1, MAX_ROLLS))
}

/// A roll with a count above what one line can hold.
pub fn too_many(typed: &str) -> bool {
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
                    per: Default::default(),
                    problem: None,
                    rows: Vec::new(),
                    markup: Vec::new(),
                },
                Table {
                    key: "broken".into(),
                    dice: "1d6".into(),
                    path: "Treasure.md".into(),
                    heading: None,
                    per: Default::default(),
                    problem: Some("1d6 needs a row for 3.".into()),
                    rows: Vec::new(),
                    markup: Vec::new(),
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
        let (tables, items) = matching(&sources, "po");
        assert!(tables.is_empty());
        assert_eq!(items[0].name, "Potion of Healing");
        let (tables, items) = matching(&sources, "Roll 2× ge");
        assert_eq!(tables[0].key, "gems");
        assert!(items.is_empty());
        let (tables, _) = matching(&sources, "roll");
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
        use crate::design::test_harness::Themed as _;
        use egui_kittest::kittest::Queryable;
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(640.0, 400.0))
            .themed_state(
                |ctx, (key, name, loot): &mut (uuid::Uuid, &str, Option<String>)| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        show_cell(ui, *key, name, loot);
                    });
                },
                (
                    uuid::Uuid::from_u128(1),
                    "Goblin",
                    Some("Iron; Roll gems".to_owned()),
                ),
            );
        harness.run_steps(2);
        // M-U0.1 (M0401): the row menu's Remove is a design action, one `Read` control high as
        // every row menu's actions are (a raw egui button is 26 px), and its Undo brings the
        // line back.
        harness
            .get_by_label("More actions for Goblin loot line 1")
            .click();
        harness.run_steps(2);
        let remove = harness.get_by_label("Remove").rect();
        assert!(
            (remove.height() - Density::Read.control()).abs() < 0.5,
            "m-u0-1-loot-remove: the row menu's Remove is {} px high",
            remove.height()
        );
        harness.get_by_label("Remove").click();
        harness.run_steps(2);
        assert_eq!(harness.state().2.as_deref(), Some("Roll gems"));
        harness.get_by_label("Undo remove loot").click();
        harness.run_steps(2);
        assert_eq!(harness.state().2.as_deref(), Some("Iron; Roll gems"));
        harness.get_by_label("Goblin loot line 1").focus();
        harness.run_steps(1);
        harness.key_press(egui::Key::End);
        for typed in [" ", "key"] {
            harness.get_by_label("Goblin loot line 1").type_text(typed);
            harness.run_steps(2);
        }
        assert_eq!(harness.state().2.as_deref(), Some("Iron key; Roll gems"));
        // The cell never holds an empty part, and the row stays while it has focus.
        harness.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::A);
        harness.key_press(egui::Key::Backspace);
        harness.run_steps(2);
        assert_eq!(harness.state().2.as_deref(), Some("Roll gems"));
        assert!(harness.query_by_label("Goblin loot line 1").is_some());
        // M-R0.7 (m-r0-7-loot-owner): the add field is its list's (`LOOT_ADD` keyed by the
        // list's owner), not its place's. Words typed and erased in the goblin's field do not
        // come back with Ctrl+Z in the orc's field, drawn at the same place.
        let add = "Add loot to Goblin";
        harness.get_by_label(add).click();
        harness.run_steps(1);
        harness.get_by_label(add).type_text("Gem");
        harness.run_steps(16);
        harness.get_by_label(add).type_text("s");
        harness.run_steps(1);
        for _ in 0.."Gems".len() {
            harness.key_press(egui::Key::Backspace);
        }
        harness.run_steps(1);
        *harness.state_mut() = (uuid::Uuid::from_u128(2), "Orc", None);
        harness.run_steps(2);
        let add = "Add loot to Orc";
        harness.get_by_label(add).click();
        harness.run_steps(1);
        assert!(harness.get_by_label(add).is_focused());
        harness.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::Z);
        harness.run_steps(2);
        assert!(
            harness
                .get_by_label(add)
                .value()
                .is_none_or(|words| words.is_empty()),
            "m-r0-7-loot-owner: Ctrl+Z in the orc's field gave it the goblin's words"
        );
        // Issue #564: two owners whose egui ids print the same 16-bit `Debug` text (the old
        // key) still have two add fields: the field is keyed by the whole owner.
        let mut printed = std::collections::HashMap::new();
        let (a, b) = (1_u128..)
            .map(uuid::Uuid::from_u128)
            .find_map(|owner| {
                let text = format!("{:?}", egui::Id::new(("loot_editor", owner)));
                printed.insert(text, owner).map(|before| (before, owner))
            })
            .expect("two owners print the same 16 bits");
        assert_ne!(
            add_field(a),
            add_field(b),
            "#564: {a} and {b} share a printed id, never a field"
        );
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
