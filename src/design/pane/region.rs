//! The scroll region of an owned list body (migration-32, layout PR 3b; design note approved
//! 2026-11-16): the native viewport over a whole-block plan (`plan`), the region's one offset
//! writer with its typed requests in one priority order (ruling L10), its Tab edges, and the
//! small-room policy (ruling L3). Its first host is the Library entry list.
//!
//! - **One offset writer.** Only the region writes its offset. A host asks with a typed request
//!   ([`Navigation`], or `RegionState::open`); the region resolves the requests once, at the start
//!   of its pass, after it reads that pass's input, in this order: a user scroll over the region
//!   (it cancels every pending request, R44); a held press in it (the others wait, never
//!   dropped); navigation (`Jump`, `Reset`, `Restore`, and the Tab edges); an opened form
//!   (`Want::Open`); focus clearance (the focused block whole again after the room changed).
//! - **Whole blocks (I1).** Only the blocks the mask says are whole are drawn: a block cut by
//!   either edge has no paint, no press area and no accessible node; a hairline marks the cut.
//!   A block taller than the view keeps its clipped paint as the counted legacy fallback (L6,
//!   the lead's answer on `TooTall`).
//! - **The native range (I2).** The scroll area's content is the plan's whole extent, never the
//!   mask: at its largest offset the last block is whole.
//! - **A list box (L10, the lead's ruling B).** The list is one Tab stop, an accessible list box:
//!   Tab enters at the selected row (else the first whole row) and the next Tab leaves the list.
//!   Up and Down (Home, End) move between rows, each a jump that shows the row whole, focus
//!   following it; so every row is reached, also one outside the view.
//! - **Small room (L3).** Below the tallest block, the list is laid out at that height in an
//!   outer scroll of the room it has; each entry into that state is counted.

#[cfg(test)]
use super::plan::Work;
use super::plan::{Block, BlockPlan, MeasureEnv, PlanState};
use crate::design::{layout::ScrollColumn, memo::Revised, tokens};
use eframe::egui::{self, Id, Key, Rect, Ui, UiBuilder, vec2};
use std::ops::Range;

/// A programmatic move of a region's offset (ruling M18): typed, never a raw offset.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Navigation {
    /// The least move that shows block `block` whole (a linked jump, the selected row).
    Jump { block: usize },
    /// Back to the origin (a new document or view).
    Reset,
    /// A kept offset, held to the native range (never snapped to a block).
    Restore(f32),
}

/// The requests a region holds until it resolves them (the held press and the user scroll are
/// read from the pass's input).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Pending {
    navigation: Option<Navigation>,
    open: Option<usize>,
    clearance: Option<usize>,
}

impl Pending {
    /// The highest request, as an offset from `current` (the others it outranks go).
    fn resolve(&mut self, plan: &BlockPlan, view: f32, current: f32) -> Option<f32> {
        let showing = |block: usize| {
            (block < plan.len())
                .then(|| plan.offset_showing(block, view, current))
                .flatten()
        };
        if let Some(navigation) = self.navigation.take() {
            *self = Self::default();
            return match navigation {
                Navigation::Jump { block } => showing(block),
                Navigation::Reset => Some(0.0),
                Navigation::Restore(offset) => Some(plan.clamp(offset, view)),
            };
        }
        if let Some(block) = self.open.take() {
            self.clearance = None;
            return showing(block);
        }
        self.clearance.take().and_then(showing)
    }
}

/// A region's own state, kept by its owner (one per list and view): its plan, its pending
/// requests, where it ended its last pass, and its small-room count.
pub struct RegionState {
    plan: PlanState,
    pending: Pending,
    /// The offset and the viewport at the end of its last pass.
    offset: f32,
    rect: Option<Rect>,
    view: f32,
    /// The block that held keyboard focus at the end of its last pass, and that control.
    focused: Option<usize>,
    focused_id: Option<Id>,
    /// The first and the last control in the region in its last pass (its scroll surface or a
    /// row), and whether focus was in the region.
    edges: (Option<Id>, Option<Id>),
    inside: bool,
    /// Where the focused control stands in its row: its first control, its last. Tab moves
    /// within a row of several controls (egui's own order) and leaves the list from its last.
    row_ends: (bool, bool),
    /// The frame of the last Tab press: egui moves focus back (Shift+Tab) a frame later.
    tab_frame: Option<u64>,
    /// The list box's stop: the selected row, which Tab enters at.
    selected: Option<usize>,
    /// A Tab out of an edge: the block whose first control takes focus once it is drawn.
    focus_next: Option<usize>,
    small: bool,
    small_rooms: usize,
}

impl RegionState {
    pub fn new(owner: &'static str) -> Self {
        Self {
            plan: PlanState::new(owner),
            pending: Pending::default(),
            offset: 0.0,
            rect: None,
            view: 0.0,
            focused: None,
            focused_id: None,
            edges: (None, None),
            inside: false,
            row_ends: (true, true),
            tab_frame: None,
            selected: None,
            focus_next: None,
            small: false,
            small_rooms: 0,
        }
    }

    /// Ask for a move (ruling M18): resolved at the region's next pass, in its priority order.
    pub fn navigate(&mut self, to: Navigation) {
        self.pending.navigation = Some(to);
    }

    /// An opened form's block, to show whole (`Want::Open`): outranked by navigation.
    pub fn open(&mut self, block: usize) {
        self.pending.open = Some(block);
    }

    /// How many times the region entered the small-room policy (ruling L3).
    pub fn small_rooms(&self) -> usize {
        self.small_rooms
    }

    /// The plan's work counters (I12) and diagnostics (L6, L7).
    #[cfg(test)]
    pub(in crate::design) fn work(&self) -> Work {
        self.plan.work()
    }
}

/// A list of blocks drawn in its region (see the module).
pub struct BlockRegion<'s> {
    id: Id,
    state: &'s mut RegionState,
    content: &'s Revised<Vec<Block>>,
    name: &'s str,
}

impl<'s> BlockRegion<'s> {
    pub fn new(
        id_salt: impl std::hash::Hash,
        state: &'s mut RegionState,
        content: &'s Revised<Vec<Block>>,
    ) -> Self {
        Self {
            id: Id::new(id_salt),
            state,
            content,
            name: "List",
        }
    }

    /// The list box's accessible name.
    pub fn name(mut self, name: &'s str) -> Self {
        self.name = name;
        self
    }

    /// The selected row: where Tab enters the list.
    pub fn selected(self, row: Option<usize>) -> Self {
        self.state.selected = row;
        self
    }

    /// Draw the list in the room left in `ui`: `block` draws block `at` in its own `Ui`, whole.
    /// Returns the blocks drawn whole.
    pub fn show(self, ui: &mut Ui, block: impl FnMut(&mut Ui, usize)) -> Range<usize> {
        let room = ui.available_rect_before_wrap();
        if super::in_content_pane(ui) {
            return self.in_place(ui, room.width(), block);
        }
        let env = MeasureEnv {
            width: room.width(),
            fonts: crate::design::theme::font_generation(ui.ctx()),
            scale: ui.ctx().pixels_per_point(),
        };
        let tallest = self.state.plan.plan(self.content, env).tallest();
        if room.height() + 0.5 >= tallest {
            self.state.small = false;
            return self.region(ui, env, room.height(), block);
        }
        // L3: below the tallest block the list is laid out at that height, in an outer scroll
        // of the room it has.
        if !std::mem::replace(&mut self.state.small, true) {
            self.state.small_rooms += 1;
        }
        let id = self.id;
        ScrollColumn::new(id.with("small_room"))
            .max_height(room.height())
            .show(ui, |ui| self.region(ui, env, tallest, block))
            .inner
    }

    /// In a `Body::Content` pane, which scrolls: the whole extent in place, the blocks in the
    /// visible clip drawn, each registered as a row, so the pane's whole-row cut (N12) applies.
    fn in_place(
        self,
        ui: &mut Ui,
        width: f32,
        mut block: impl FnMut(&mut Ui, usize),
    ) -> Range<usize> {
        let env = MeasureEnv {
            width,
            fonts: crate::design::theme::font_generation(ui.ctx()),
            scale: ui.ctx().pixels_per_point(),
        };
        let plan = self.state.plan.plan(self.content, env);
        let (rect, _) = ui.allocate_exact_size(vec2(width, plan.extent()), egui::Sense::hover());
        let mut drawn = 0..0;
        for at in 0..plan.len() {
            let (top, end) = plan.span(at);
            let block_rect = Rect::from_min_size(rect.min + vec2(0.0, top), vec2(width, end - top));
            if !ui.is_rect_visible(block_rect) {
                continue;
            }
            if drawn.is_empty() {
                drawn = at..at;
            }
            drawn.end = at + 1;
            let mut child =
                ui.new_child(UiBuilder::new().id_salt(("block", at)).max_rect(block_rect));
            block(&mut child, at);
            super::super::components::register_row(ui, block_rect);
        }
        drawn
    }

    fn region(
        self,
        ui: &mut Ui,
        env: MeasureEnv,
        view: f32,
        mut block: impl FnMut(&mut Ui, usize),
    ) -> Range<usize> {
        let Self {
            id,
            state,
            content,
            name,
        } = self;
        let ctx = ui.ctx().clone();
        // The pass's input first: a user scroll over the region cancels every pending request
        // (R44); a press held in it makes them wait.
        let over = |rect: Option<Rect>, pos: Option<egui::Pos2>| {
            rect.zip(pos).is_some_and(|(rect, pos)| rect.contains(pos))
        };
        let (scrolled, held) = ctx.input(|input| {
            let pointer = &input.pointer;
            (
                over(state.rect, pointer.hover_pos())
                    && (input.raw_scroll_delta.y != 0.0 || input.smooth_scroll_delta.y != 0.0),
                pointer.any_down() && over(state.rect, pointer.press_origin()),
            )
        });
        if scrolled {
            state.pending = Pending::default();
        }
        // The list box: Tab leaves from the region's last control (Shift+Tab from its first), so
        // egui's own Tab goes on past the list; while a row holds focus, Up and Down (Home, End)
        // are jumps that focus follows.
        if ctx.input(|input| input.key_pressed(Key::Tab)) {
            state.tab_frame = Some(ctx.cumulative_frame_nr());
        }
        if state.inside {
            let (tab, back) =
                ctx.input(|input| (input.key_pressed(Key::Tab), input.modifiers.shift));
            let leaves = if back {
                state.row_ends.0
            } else {
                state.row_ends.1
            };
            if tab
                && leaves
                && let Some(edge) = if back { state.edges.0 } else { state.edges.1 }
            {
                ctx.memory_mut(|memory| memory.request_focus(edge));
            }
        }
        if let Some(at) = state.focused
            && ctx.memory(|memory| memory.focused()) == state.focused_id
        {
            let rows = content.len();
            let key = |key| ctx.input_mut(|input| input.consume_key(egui::Modifiers::NONE, key));
            let to = if key(Key::ArrowDown) {
                (at + 1 < rows).then_some(at + 1)
            } else if key(Key::ArrowUp) {
                at.checked_sub(1)
            } else if key(Key::Home) {
                (rows > 0).then_some(0)
            } else if key(Key::End) {
                rows.checked_sub(1)
            } else {
                None
            };
            if let Some(to) = to {
                state.pending.navigation = Some(Navigation::Jump { block: to });
                state.focus_next = Some(to);
            }
        }
        let plan = state.plan.plan(content, env);
        let current = plan.clamp(state.offset, view);
        // Focus clearance: the room changed under the focused block.
        if (view - state.view).abs() > 0.5
            && let Some(focused) = state.focused
            && !plan.mask("region", current, view).whole.contains(&focused)
        {
            state.pending.clearance = Some(focused);
        }
        state.view = view;
        let target = (!held)
            .then(|| state.pending.resolve(plan, view, current))
            .flatten();
        let extent = plan.extent();
        let mut column = ScrollColumn::new(id)
            .max_height(view)
            .min_scrolled_height(view);
        if let Some(offset) = target {
            column = column.vertical_scroll_offset(offset);
        }
        let layer = ui.layer_id();
        let mut drawn: Vec<(usize, Range<usize>)> = Vec::new();
        let region_from = widget_count(&ctx, layer);
        let output = column.show_viewport(ui, |ui, viewport| {
            let mask = state
                .plan
                .mask(content, env, viewport.top(), viewport.height());
            let plan = state.plan.plan(content, env);
            // The native range is the whole extent (I2).
            ui.set_min_height(extent);
            let origin = ui.max_rect().min;
            let width = ui.available_width();
            // The rows are the options of one accessible list box.
            let mut list =
                ui.new_child(UiBuilder::new().id_salt("list_box").max_rect(ui.max_rect()));
            ctx.accesskit_node_builder(list.unique_id(), |node| {
                node.set_role(egui::accesskit::Role::ListBox);
                node.set_label(name);
            });
            let tall = mask.too_tall.iter().map(|tall| tall.block);
            for at in mask.whole.clone().chain(tall) {
                let (top, end) = plan.span(at);
                let rect = Rect::from_min_size(origin + vec2(0.0, top), vec2(width, end - top));
                let from = widget_count(&ctx, layer);
                let mut child =
                    list.new_child(UiBuilder::new().id_salt(("block", at)).max_rect(rect));
                block(&mut child, at);
                drawn.push((at, from..widget_count(&ctx, layer)));
            }
            // A hairline marks each cut edge: content goes on beyond it.
            let hairline = |y: f32| {
                ui.painter().hline(
                    origin.x..=origin.x + width,
                    y,
                    egui::Stroke::new(tokens::HAIRLINE_WIDTH, tokens::colors().hairline),
                );
            };
            if let Some((from, to)) = mask.eligible {
                if from > viewport.top() + 0.5 {
                    hairline(origin.y + viewport.top() + tokens::HAIRLINE_WIDTH / 2.0);
                }
                if to < (viewport.bottom()).min(extent) - 0.5 {
                    hairline(origin.y + viewport.bottom() - tokens::HAIRLINE_WIDTH / 2.0);
                }
            }
            mask.whole
        });
        let region_widgets = region_from..widget_count(&ctx, layer);
        state.offset = output.state.offset.y;
        state.rect = Some(output.inner_rect);
        let whole = output.inner;
        // Focus: which block holds it, the edge rows' controls, and the list box's stop.
        let focused = ctx.memory(|memory| memory.focused());
        let widgets = |range: &Range<usize>| -> Vec<egui::WidgetRect> {
            ctx.viewport(|viewport| {
                viewport
                    .this_pass
                    .widgets
                    .get_layer(layer)
                    .skip(range.start)
                    .take(range.len())
                    .copied()
                    .collect()
            })
        };
        let first_control = |range: &Range<usize>| {
            widgets(range)
                .into_iter()
                .find(|widget| widget.enabled && widget.sense.is_focusable())
                .map(|widget| widget.id)
        };
        let was_in = state.inside;
        let controls: Vec<Id> = widgets(&region_widgets)
            .into_iter()
            .filter(|widget| widget.enabled && widget.sense.is_focusable())
            .map(|widget| widget.id)
            .collect();
        state.inside = focused.is_some_and(|id| controls.contains(&id));
        state.edges = (controls.first().copied(), controls.last().copied());
        state.focused = focused.and_then(|id| {
            drawn
                .iter()
                .find(|(_, range)| widgets(range).iter().any(|widget| widget.id == id))
                .map(|(at, _)| *at)
        });
        state.focused_id = focused.filter(|_| state.focused.is_some());
        state.row_ends = match (focused, state.focused) {
            (Some(id), Some(at)) => drawn
                .iter()
                .find(|(row, _)| *row == at)
                .map(|(_, range)| {
                    let row: Vec<Id> = widgets(range)
                        .into_iter()
                        .filter(|widget| widget.enabled && widget.sense.is_focusable())
                        .map(|widget| widget.id)
                        .collect();
                    (row.first() == Some(&id), row.last() == Some(&id))
                })
                .unwrap_or((true, true)),
            // On the region's own scroll surface.
            _ => (true, true),
        };
        if let (Some(id), Some(_)) = (focused, state.focused) {
            // The focused row keeps the vertical arrows: they move between rows (above).
            ctx.memory_mut(|memory| {
                memory.set_focus_lock_filter(
                    id,
                    egui::EventFilter {
                        vertical_arrows: true,
                        ..Default::default()
                    },
                );
            });
        }
        // A Tab into the region (its scroll surface or a row) lands on the list box's stop: the
        // selected row, else the first whole row.
        let by_tab = state
            .tab_frame
            .is_some_and(|frame| ctx.cumulative_frame_nr() <= frame + 1);
        if state.inside && !was_in && by_tab {
            let stop = state
                .selected
                .filter(|row| *row < content.len())
                .unwrap_or(whole.start);
            if state.focused != Some(stop) {
                match drawn.iter().find(|(row, _)| *row == stop) {
                    Some((_, range)) => {
                        if let Some(first) = first_control(range) {
                            ctx.memory_mut(|memory| memory.request_focus(first));
                        }
                    }
                    None => {
                        state.pending.navigation = Some(Navigation::Jump { block: stop });
                        state.focus_next = Some(stop);
                    }
                }
            }
        }
        // A row an arrow (or the stop) reached takes focus on its first control once drawn.
        if let Some(next) = state.focus_next
            && let Some((_, range)) = drawn.iter().find(|(at, _)| *at == next)
        {
            let first = ctx.viewport(|viewport| {
                viewport
                    .this_pass
                    .widgets
                    .get_layer(layer)
                    .skip(range.start)
                    .take(range.len())
                    .find(|widget| widget.enabled && widget.sense.is_focusable())
                    .map(|widget| widget.id)
            });
            if let Some(first) = first {
                ctx.memory_mut(|memory| memory.request_focus(first));
            }
            state.focus_next = None;
        }
        whole
    }
}

/// The widgets on `layer` so far in this pass.
fn widget_count(ctx: &egui::Context, layer: egui::LayerId) -> usize {
    ctx.viewport(|viewport| viewport.this_pass.widgets.get_layer(layer).count())
}
