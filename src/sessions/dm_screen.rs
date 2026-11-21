//! The DM screen: the open session's plan and draft in tiles that the DM can dock and resize.
#[path = "dm_layout.rs"]
mod layout;

use super::{Book, Draft, Note, PLAN_WIDTH, Plan, Sessions, compact_plan_for_reading, page_column};
use crate::{
    design::{
        self, Action, Density, DockSide, Gap, Heading, SplitAxis, SplitMove, Type, colors, icons,
        layout::ScrollColumn,
        memo::{Memo, Revision},
        split_handle,
    },
    markdown_view,
    sessions::{WHAT_HAPPENED, section, without_empty_sections, without_own_heading},
};
use eframe::egui::{self, Key, Modifiers, Rect, pos2, vec2};
use layout::{Axis, EDGE_SHARE, Edge, Gaps, Layout, Place, Room};
pub use layout::{DmScreenLayout, SavedNode, SavedPart};
use std::{borrow::Cow, ops::Range, sync::Arc};

const THREADS: &str = "Open threads";
const REST: &str = "Rest of note";
const LIVE_NOTES: design::FieldName = crate::field_id!("dm_screen_what_happened");
const PLAY_PLAN_WIDTH: f32 = PLAN_WIDTH + 4.0 * design::L;
const LIVE_NOTES_WIDTH: f32 = 360.0;
const REFERENCE_WIDTH: f32 = 300.0;
/// A column that holds a full line of reading in its tile.
const NOTE_MEASURE: f32 = design::READER_MEASURE + super::NOTE_FRAME_EDGES;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Screen {
    Play,
    Notes,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
enum Tile {
    #[default]
    Plan,
    Happened,
    Threads,
    Rest,
}

impl Tile {
    fn title(self) -> &'static str {
        match self {
            Self::Plan => "Plan for tonight",
            Self::Happened => WHAT_HAPPENED,
            Self::Threads => THREADS,
            Self::Rest => REST,
        }
    }

    /// The tile's name in the saved layout.
    fn key(self) -> &'static str {
        match self {
            Self::Plan => "plan",
            Self::Happened => "what_happened",
            Self::Threads => "open_threads",
            Self::Rest => "rest_of_note",
        }
    }

    /// The tile's word on a narrow page's tab.
    fn tab(self) -> &'static str {
        match self {
            Self::Plan => "Plan",
            other => other.title(),
        }
    }
}

/// A tile being dragged by its grip.
#[derive(Clone, Copy)]
struct TileDrag(Tile);

/// What the DM asked of the layout in a frame: applied after the tiles drew.
enum LayoutAct {
    Resize(Edge, SplitMove),
    Place(Tile, Place),
    Swap(Tile, Tile),
    Hide(Tile),
    Show(Tile),
    Reset,
}

fn asked(by: SplitMove) -> Option<f32> {
    match by {
        SplitMove::By(by) => Some(by),
        SplitMove::Even => None,
    }
}

/// The layout until the DM changes it, for a page `width` wide with `gap` between columns, and
/// whether it shows as tiles. A 1440p screen, or 4K at 150 %, gives Open threads and the rest
/// of the note a column each, once both hold a full line of reading.
fn default_layout(width: f32, gap: f32) -> (Layout, bool) {
    let wide = width >= PLAY_PLAN_WIDTH + LIVE_NOTES_WIDTH + REFERENCE_WIDTH + 2.0 * gap;
    let four = width - 3.0 * gap - PLAY_PLAN_WIDTH - NOTE_MEASURE >= 2.0 * NOTE_MEASURE;
    let room = width - if four { 3.0 } else { 2.0 } * gap;
    let notes = (room - PLAY_PLAN_WIDTH - REFERENCE_WIDTH).clamp(LIVE_NOTES_WIDTH, NOTE_MEASURE);
    let reference = (room - PLAY_PLAN_WIDTH - notes).max(REFERENCE_WIDTH);
    let tile = Layout::tile;
    let layout = if four {
        Layout::split(
            Axis::Across,
            vec![
                (tile(Tile::Plan), PLAY_PLAN_WIDTH),
                (tile(Tile::Happened), notes),
                (tile(Tile::Threads), reference / 2.0),
                (tile(Tile::Rest), reference / 2.0),
            ],
        )
    } else {
        // Above the rest of the note, the threads take a third.
        let reference_tiles = Layout::split(
            Axis::Down,
            vec![(tile(Tile::Threads), 1.0), (tile(Tile::Rest), 2.0)],
        );
        Layout::split(
            Axis::Across,
            vec![
                (tile(Tile::Plan), PLAY_PLAN_WIDTH),
                (tile(Tile::Happened), notes),
                (reference_tiles, reference),
            ],
        )
    };
    (layout, wide)
}

#[derive(Default)]
pub(super) struct Preview {
    screen: Option<Screen>,
    tile: Tile,
    projection: Memo<Revision, Arc<Sections>>,
    /// True only while Play draws its tiles, so that the plan's rows take the tile's layout
    /// there and nowhere else (Play gives way to the full view for a conflict or a group).
    in_tiles: bool,
    /// The DM's layout, once they moved, resized, hid or showed a tile; `None` is the default
    /// layout (`default_layout`). It is the DM's choice for the app, saved with the window's
    /// state, and stays when the DM screen closes.
    layout: Option<Layout>,
}

impl Preview {
    pub(super) fn is_open(&self) -> bool {
        self.screen.is_some()
    }

    pub(super) fn is_play(&self) -> bool {
        self.screen == Some(Screen::Play)
    }

    pub(super) fn in_tiles(&self) -> bool {
        self.in_tiles
    }

    pub(super) fn open_play(&mut self) {
        self.screen = Some(Screen::Play);
    }

    pub(super) fn show_notes(&mut self) {
        if self.is_open() {
            self.screen = Some(Screen::Notes);
        }
    }

    pub(super) fn toggle_button(&mut self, ui: &mut egui::Ui) {
        let word = if self.is_open() {
            "Close DM screen"
        } else {
            "DM screen"
        };
        if Action::quiet(word).show(ui).clicked() {
            if self.is_open() {
                self.screen = None;
                self.tile = Tile::default();
            } else {
                self.open_play();
            }
        }
    }

    pub(super) fn page_keys(&mut self, ui: &egui::Ui) {
        if ui.input_mut(|input| input.consume_key(Modifiers::CTRL, Key::PageUp)) {
            self.screen = Some(Screen::Play);
        }
        if ui.input_mut(|input| input.consume_key(Modifiers::CTRL, Key::PageDown)) {
            self.screen = Some(Screen::Notes);
        }
    }

    pub(super) fn pager(&mut self, ui: &mut egui::Ui) {
        let pages = [(Some(Screen::Play), "Play"), (Some(Screen::Notes), "Notes")];
        let width = pages
            .iter()
            .map(|&(_, word)| Action::quiet(word).natural_width(ui))
            .sum();
        ui.vertical_centered(|ui| {
            ui.allocate_ui(egui::vec2(width, Density::Read.control()), |ui| {
                design::segmented(ui, &mut self.screen, &pages);
            });
        });
    }
}

impl Preview {
    /// Apply what the DM asked of `drawn`, the layout this frame drew. The first change makes
    /// the default layout the DM's own; Reset layout gives the default back.
    fn apply(&mut self, mut drawn: Layout, acts: Vec<LayoutAct>, room: Room) {
        if acts.is_empty() {
            return;
        }
        for act in acts {
            match act {
                LayoutAct::Resize(edge, by) => drawn.resize(&edge, asked(by), room),
                LayoutAct::Place(tile, place) => drawn.place_tile(tile, place),
                LayoutAct::Swap(a, b) => drawn.swap(a, b),
                LayoutAct::Hide(tile) => drawn.hide(tile),
                LayoutAct::Show(tile) => drawn.show(tile),
                LayoutAct::Reset => {
                    self.layout = None;
                    return;
                }
            }
        }
        self.layout = Some(drawn);
    }

    /// The DM's layout, for the app's window state; `None` while it is the default.
    pub(super) fn saved_layout(&self) -> Option<DmScreenLayout> {
        self.layout.as_ref().map(Layout::to_saved)
    }

    pub(super) fn restore_layout(&mut self, saved: Option<&DmScreenLayout>) {
        self.layout = saved.and_then(Layout::from_saved);
    }
}

pub(super) fn pager_height(ui: &egui::Ui) -> f32 {
    Density::Read.control() + ui.spacing().item_spacing.y
}

/// What the What happened tile edits: the section's words, without the line break that ends
/// the note. `None` when the note has no `What happened` section. One reading
/// (`sessions::section`) with the app's other writers of the section.
fn happened_words(body: &str) -> Option<Range<usize>> {
    let found = section(body, WHAT_HAPPENED)?;
    let mut end = found.end;
    if end == body.len() && end > found.words {
        let words = &body[found.words..end];
        end -= words
            .strip_suffix("\r\n")
            .or_else(|| words.strip_suffix('\n'))
            .map_or(0, |kept| words.len() - kept.len());
    }
    Some(found.words..end)
}

/// Positions in the draft, and the projected words for the read-only tiles.
struct Sections {
    /// The draft's revision that these were read from.
    revision: u64,
    happened: Option<Range<usize>>,
    threads: String,
    remainder: String,
}

impl Sections {
    fn read(
        body: &str,
        revision: u64,
        note: &Note,
        model: &super::plan::PlanModel,
        plan: &Plan<'_>,
    ) -> Self {
        let happened = section(body, WHAT_HAPPENED);
        let threads = section(body, THREADS);
        // The sections with a tile of their own leave the rest of the note, from the end so
        // that each cut keeps the earlier offsets.
        let mut cuts: Vec<Range<usize>> = [happened, threads]
            .into_iter()
            .flatten()
            .map(|found| found.heading..found.next)
            .collect();
        cuts.sort_by_key(|cut| cut.start);
        let mut remainder = body.to_owned();
        for cut in cuts.into_iter().rev() {
            remainder.replace_range(cut, "");
        }
        let remainder = compact_plan_for_reading(
            &without_own_heading(&remainder, note).replace("\r\n", "\n"),
            plan.fights,
            plan.groups,
            &model.scene_lines(),
            true,
        );
        Self {
            revision,
            happened: happened_words(body),
            threads: threads.map_or_else(String::new, |found| {
                body[found.words..found.end].trim().to_owned()
            }),
            remainder: without_empty_sections(&remainder),
        }
    }
}

impl Sessions {
    pub(super) fn dm_play(
        &mut self,
        ui: &mut egui::Ui,
        book: &mut Book,
        plan: &Plan<'_>,
        note: &Note,
    ) {
        let model = self.plan_model(super::reader_index(ui).as_deref(), plan);
        let Some(draft) = &self.draft else { return };
        let revision = draft.revision();
        let sections = Arc::clone(
            self.dm_screen
                .projection
                .get_or(self.plan_model.revision(), || {
                    Arc::new(Sections::read(&draft.body, revision, note, &model, plan))
                }),
        );
        let save = ui.input_mut(|input| input.consume_key(Modifiers::COMMAND, Key::S));
        let width = ui.available_width();
        // Columns are `L` apart, and tiles in a column `S` and the line spacing.
        let gap_x = ui.spacing().item_spacing.x + Gap::S.size();
        let gap_y = ui.spacing().item_spacing.y + Gap::S.size();
        let room = Room {
            plan_min: PLAY_PLAN_WIDTH,
            gaps: Gaps {
                across: gap_x,
                down: gap_y,
            },
        };
        let custom = self.dm_screen.layout.is_some();
        let (layout, wide) = match &self.dm_screen.layout {
            Some(layout) => (layout.clone(), layout.min_size(room).x <= width + 0.5),
            None => default_layout(width, gap_x),
        };
        let mut acts = Vec::new();
        let mut shown = self.dm_screen.tile;
        self.dm_screen.in_tiles = true;
        let mut draw = |ui: &mut egui::Ui, tile, chrome: Option<&mut Vec<LayoutAct>>| {
            let chrome = chrome.map(|acts| TileChrome {
                layout: &layout,
                custom,
                acts,
            });
            self::tile(ui, tile, chrome, |ui| {
                self.dm_tile(ui, tile, book, plan, note, &sections);
            });
        };
        if wide {
            let height = ui.available_height();
            let area = Rect::from_min_size(ui.cursor().min, vec2(width, height));
            let placed = layout.place(area, room).unwrap_or_default();
            page_column(ui, width, height, |ui| {
                // The handles first: a column edge's band keeps the hangs of the tiles after
                // it clear of it.
                for edge in &placed.edges {
                    let names = edge
                        .before
                        .iter()
                        .map(|tile| tile.title())
                        .collect::<Vec<_>>()
                        .join(" and ");
                    let (axis, name) = match edge.axis {
                        Axis::Across => (SplitAxis::Columns, format!("Resize {names} column")),
                        Axis::Down if edge.before.len() > 1 => {
                            (SplitAxis::Rows, format!("Resize {names} tiles"))
                        }
                        Axis::Down => (SplitAxis::Rows, format!("Resize {names} tile")),
                    };
                    let id = egui::Id::new(("session_dm_split", &edge.split, edge.after));
                    if let Some(by) = split_handle(ui, id, axis, edge.gap, &name) {
                        acts.push(LayoutAct::Resize(edge.clone(), by));
                    }
                }
                for &(tile, rect) in &placed.tiles {
                    let mut child = ui.new_child(
                        egui::UiBuilder::new()
                            .max_rect(rect)
                            .layout(egui::Layout::top_down(egui::Align::Min)),
                    );
                    draw(&mut child, tile, Some(&mut acts));
                }
                if let Some(act) = dock_drop(ui, area, &placed.tiles, &layout) {
                    acts.push(act);
                }
            });
        } else {
            // Tabs list every tile, hidden ones last: the tiles' menus, with Show and Reset,
            // are not here.
            let tabs: Vec<(Tile, &str)> = layout
                .tiles()
                .into_iter()
                .chain(layout.hidden())
                .map(|tile| (tile, tile.tab()))
                .collect();
            if !tabs.iter().any(|&(tile, _)| tile == shown) {
                shown = tabs[0].0;
            }
            design::segmented(ui, &mut shown, &tabs);
            draw(ui, shown, None);
        }
        self.dm_screen.tile = shown;
        self.dm_screen.apply(layout, acts, room);
        self.dm_screen.in_tiles = false;
        if save && let Some(draft) = &mut self.draft {
            draft.save();
        }
        if let Some(target) = markdown_view::take_note_link(ui.ctx()) {
            self.open_note_link(target);
        }
    }

    fn dm_tile(
        &mut self,
        ui: &mut egui::Ui,
        tile: Tile,
        book: &mut Book,
        plan: &Plan<'_>,
        note: &Note,
        sections: &Sections,
    ) {
        match tile {
            Tile::Plan => {
                // A wider tile keeps the plan's own width, so each row's actions stay by its
                // words, as the reader keeps its measure.
                let width = ui.available_width().min(PLAY_PLAN_WIDTH - 2.0 * design::S);
                page_column(ui, width, ui.available_height(), |ui| {
                    self.plan_pane(ui, book, plan, None);
                });
            }
            Tile::Happened => {
                let draft = self.draft.as_mut().expect("the open draft");
                // The Plan tile draws first, and its acts (Remove, Add, Undo) change the note in
                // that frame. The words' place is then read again from the changed note.
                let range = if draft.revision() == sections.revision {
                    sections.happened.clone()
                } else {
                    happened_words(&draft.body)
                };
                live_notes(ui, note, draft, range);
            }
            Tile::Threads | Tile::Rest => {
                let text = if tile == Tile::Threads {
                    &sections.threads
                } else {
                    &sections.remainder
                };
                if !text.trim().is_empty() {
                    ScrollColumn::new((note.id, tile)).show(ui, |ui| {
                        ui.set_max_width(ui.available_width().min(design::READER_MEASURE));
                        markdown_view::show_reader(ui, text);
                    });
                }
            }
        }
    }
}

/// Where a dragged tile goes if it is let go now: along an edge of the whole `area` when the
/// pointer is within `XL` of it, else beside the tile under the pointer, on its nearest side.
/// The place shows while the tile is held; the act comes on the frame it is let go.
fn dock_drop(
    ui: &egui::Ui,
    area: Rect,
    tiles: &[(Tile, Rect)],
    layout: &Layout,
) -> Option<LayoutAct> {
    let held = egui::DragAndDrop::payload::<TileDrag>(ui.ctx())?.0;
    let pointer = ui
        .input(|input| input.pointer.latest_pos())
        .filter(|pointer| area.contains(*pointer))?;
    // A grip near the screen's edge starts inside that edge's band: a short slip of the held
    // tile must not move it. It goes somewhere only once the pointer leaves it.
    if tiles
        .iter()
        .any(|&(tile, rect)| tile == held && rect.contains(pointer))
    {
        return None;
    }
    let (place, preview) = match DockSide::ALL
        .into_iter()
        .find(|side| side.band(area, design::XL).contains(pointer))
    {
        Some(side) => (Place::Edge(side), side.part(area, EDGE_SHARE)),
        None => {
            let &(target, rect) = tiles.iter().find(|(_, rect)| rect.contains(pointer))?;
            let side = DockSide::nearest(rect, pointer);
            (Place::Beside(target, side), side.part(rect, 0.5))
        }
    };
    if !layout.can_place(held, place) {
        return None;
    }
    design::dock_preview(ui, preview);
    ui.input(|input| input.pointer.any_released()).then(|| {
        egui::DragAndDrop::clear_payload(ui.ctx());
        LayoutAct::Place(held, place)
    })
}

/// A tile's grip and menu, with what they need.
struct TileChrome<'a> {
    layout: &'a Layout,
    custom: bool,
    acts: &'a mut Vec<LayoutAct>,
}

/// A tile's frame, as tall as its room. With `chrome`, its title line: the grip that drags the
/// tile, its title over a hairline, and its menu at the line's end, the grip and the menu
/// centred on the title's line.
fn tile(
    ui: &mut egui::Ui,
    tile: Tile,
    chrome: Option<TileChrome<'_>>,
    body: impl FnOnce(&mut egui::Ui),
) {
    ui.scope_builder(egui::UiBuilder::new().id(("session_dm_tile", tile)), |ui| {
        egui::Frame::new()
            .stroke(egui::Stroke::new(
                design::HAIRLINE_WIDTH,
                colors().field_border,
            ))
            .corner_radius(design::RADIUS_CONTROL)
            .inner_margin(design::S)
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                ui.set_min_height(ui.available_height());
                if let Some(chrome) = chrome {
                    let inside = ui.max_rect();
                    let menu = Density::Read.control();
                    let grip = design::XL;
                    let gap = ui.spacing().item_spacing.x;
                    let words = (ui.available_width() - grip - menu - 2.0 * gap).max(0.0);
                    let title = ui
                        .horizontal_top(|ui| {
                            // The grip's room, drawn below once the title's line is known.
                            ui.allocate_exact_size(vec2(grip, 0.0), egui::Sense::hover());
                            ui.allocate_ui_with_layout(
                                vec2(words, 0.0),
                                egui::Layout::top_down(egui::Align::Min),
                                |ui| {
                                    ui.set_width(words);
                                    design::heading(ui, Heading::Section(tile.title()))
                                },
                            )
                            .inner
                        })
                        .inner;
                    let line = title.rect.center().y;
                    let at = |x: f32, size: f32| {
                        egui::UiBuilder::new().max_rect(Rect::from_center_size(
                            pos2(x, line),
                            egui::Vec2::splat(size),
                        ))
                    };
                    let mut grip_ui = ui.new_child(at(inside.left() + grip / 2.0, grip));
                    let id = egui::Id::new(("session_dm_grip", tile));
                    let handle =
                        design::drag_grip(&mut grip_ui, id, &format!("Move {}", tile.title()));
                    design::drag_source(
                        &grip_ui,
                        &handle,
                        Some(TileDrag(tile)),
                        design::Ghost::new(icons::DRAG, tile.title()),
                    );
                    tile_menu(
                        &mut ui.new_child(at(inside.right() - menu / 2.0, menu)),
                        tile,
                        inside,
                        chrome,
                    );
                }
                body(ui);
            });
    });
}

/// A tile's menu, the keyboard's way to do what the grip does: swap it with another tile, put
/// it beside another or along an edge of the screen, hide it, show a hidden tile, or go back to
/// the default layout. Only what can be done is listed.
fn tile_menu(ui: &mut egui::Ui, tile: Tile, inside: Rect, chrome: TileChrome<'_>) {
    let TileChrome {
        layout,
        custom,
        acts,
    } = chrome;
    let others: Vec<Tile> = layout
        .tiles()
        .into_iter()
        .filter(|other| *other != tile)
        .collect();
    design::row_menu(ui, tile.title(), |ui| {
        if !others.is_empty() {
            design::menu_submenu(ui, "Swap with", inside, |ui| {
                for &other in &others {
                    if Action::quiet(other.title())
                        .name(&format!("Swap with {}", other.title()))
                        .show(ui)
                        .clicked()
                    {
                        acts.push(LayoutAct::Swap(tile, other));
                        ui.close();
                    }
                }
            });
            for (side, word) in [
                (DockSide::Left, "Move to the left of"),
                (DockSide::Right, "Move to the right of"),
                (DockSide::Top, "Move above"),
                (DockSide::Bottom, "Move below"),
            ] {
                design::menu_submenu(ui, word, inside, |ui| {
                    for &other in &others {
                        if Action::quiet(other.title())
                            .name(&format!("{word} {}", other.title()))
                            .show(ui)
                            .clicked()
                        {
                            acts.push(LayoutAct::Place(tile, Place::Beside(other, side)));
                            ui.close();
                        }
                    }
                    if Action::quiet("All tiles")
                        .name(&format!("{word} all tiles"))
                        .show(ui)
                        .clicked()
                    {
                        acts.push(LayoutAct::Place(tile, Place::Edge(side)));
                        ui.close();
                    }
                });
            }
        }
        if layout.can_hide(tile) && Action::quiet("Hide tile").show(ui).clicked() {
            acts.push(LayoutAct::Hide(tile));
            ui.close();
        }
        for hidden in layout.hidden() {
            if Action::quiet(&format!("Show {}", hidden.title()))
                .show(ui)
                .clicked()
            {
                acts.push(LayoutAct::Show(hidden));
                ui.close();
            }
        }
        if custom && Action::quiet("Reset layout").show(ui).clicked() {
            acts.push(LayoutAct::Reset);
            ui.close();
        }
    });
}

/// The What happened editor, over `range` of the draft. With no such section (`None`), the
/// first words typed start one at the end of the note.
fn live_notes(ui: &mut egui::Ui, note: &Note, draft: &mut Draft, range: Option<Range<usize>>) {
    let shown = range
        .as_ref()
        .map_or("", |range| &draft.body[range.clone()]);
    let mut words = Cow::Borrowed(shown);
    // The editor fills its tile: it is where the DM writes during play.
    let height = ui.available_height();
    ScrollColumn::new((note.id, Tile::Happened)).show(ui, |ui| {
        let font = Type::Body.font(ui);
        let width = ui.available_width().min(design::READER_MEASURE);
        let hint = if range.is_some() {
            "Capture what happens at the table…"
        } else {
            "Type what happens at the table. Splat adds a What happened section to the end of the note."
        };
        let output = LIVE_NOTES.of(note.id).editor(ui, &mut words, |edit| {
            edit.hint_text(hint)
                .font(font)
                .desired_width(width)
                .min_size(egui::vec2(0.0, height))
        });
        design::named(ui, &output.response, WHAT_HAPPENED);
    });
    if words.as_ref() == shown {
        return;
    }
    let replacement = words.into_owned();
    let body = &mut draft.body;
    let newline = if body.contains("\r\n") { "\r\n" } else { "\n" };
    match range {
        Some(range) => {
            // For an empty heading, keep what is typed on its own line, also when the next
            // heading immediately follows it or the file ends at the heading.
            let before = &body[..range.start];
            let after = &body[range.end..];
            let leading = if before.ends_with('\n') { "" } else { newline };
            let trailing = if after.is_empty() || after.starts_with(['\r', '\n']) {
                ""
            } else {
                newline
            };
            body.replace_range(range, &format!("{leading}{replacement}{trailing}"));
        }
        None => {
            // As `insert_in_note` adds words to a note without the section: at its end. The
            // editor's words stay exactly as typed, so the caret keeps its place.
            body.truncate(body.trim_end().len());
            if !body.is_empty() {
                body.push_str(newline);
                body.push_str(newline);
            }
            body.push_str(&format!(
                "## {WHAT_HAPPENED}{newline}{replacement}{newline}"
            ));
        }
    }
    draft.edited();
}
