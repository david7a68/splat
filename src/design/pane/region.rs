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
//! - **Fit height** (`BlockRegion::fit_height`, layout 11c, note C0-parts Part A). Outside a
//!   `Content` pane, a region is as tall as the room it is given; a fit region is as tall as its
//!   blocks, up to that room. In a child of a shared composite (`PaneColumns::shared_task`), its
//!   scroll column then adds the height it hides, so the composite is measured by the blocks, and
//!   a short list leaves the footer to follow it (`ScrollColumn::fit_height`'s rule, for blocks).

use super::laid::{Laid, LaidUi};
#[cfg(test)]
use super::plan::Work;
use super::plan::{Block, BlockPlan, MeasureEnv, PlanState};
use crate::design::{layout::ScrollColumn, memo::Revised, tokens};
use egui::{self, Id, Key, Rect, Ui, UiBuilder, vec2};
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
    /// The least move that shows the measured block with this stable key whole (layout 11c): the
    /// request follows its block when blocks move, and ends when the block leaves the plan. A
    /// block taller than the view shows its head at the view's top.
    Show(Id),
}

/// The frames a request may wait (a held press) before it ends.
const REQUEST_FRAMES: u64 = 8;

/// The block (by key) whose first control, or last, takes the focus once it draws, and the
/// frame a key asked it in.
type FocusTo = Option<(Id, bool, u64)>;

/// A Tab in a focused text field at the drawn edge: the field, the block it routes to if the
/// field gives Tab up, and whether to that block's last control.
type TabAfter = Option<(Id, usize, bool)>;

/// What the highest request wants shown.
enum Want {
    Block(usize),
    Origin,
    Kept(f32),
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
    /// The highest request (the others it outranks go): a block that is no longer in the plan
    /// ends its request.
    fn take(&mut self, plan: &BlockPlan) -> Option<Want> {
        let block = |at: usize| (at < plan.len()).then_some(Want::Block(at));
        if let Some(navigation) = self.navigation.take() {
            *self = Self::default();
            return match navigation {
                Navigation::Jump { block: at } => block(at),
                Navigation::Show(key) => plan.find(key).map(Want::Block),
                Navigation::Reset => Some(Want::Origin),
                Navigation::Restore(offset) => Some(Want::Kept(offset)),
            };
        }
        if let Some(at) = self.open.take() {
            self.clearance = None;
            return block(at);
        }
        self.clearance.take().and_then(block)
    }

    /// The highest request, as an offset of the region's own column from `current`.
    fn resolve(&mut self, plan: &BlockPlan, view: f32, current: f32) -> Option<f32> {
        match self.take(plan)? {
            Want::Block(at) => plan.offset_showing(at, view, current),
            Want::Origin => Some(0.0),
            Want::Kept(offset) => Some(plan.clamp(offset, view)),
        }
    }
}

/// A block an edge cuts, drawn in `child` (its own `Ui`): its paint and presses end at its parts'
/// span `y` (L6), so its cut part shows nothing and takes no press.
fn clip_to_part(child: &mut Ui, y: std::ops::RangeInclusive<f32>) {
    let part = Rect::from_x_y_ranges(child.max_rect().x_range(), y);
    child.set_clip_rect(child.clip_rect().intersect(part));
}

/// The blocks a mask draws, in order: each whole block, and each block an edge cuts, clipped to
/// its parts (an eligible unit of a split section, a `TooTall` unit; ruling L6). Every block
/// that meets the view takes one of these paths or, with no part to draw, none (ruling L7).
fn drawn_blocks(mask: &super::plan::Mask) -> Vec<(usize, Option<(f32, f32)>)> {
    let mut out: Vec<(usize, Option<(f32, f32)>)> =
        mask.whole.clone().map(|at| (at, None)).collect();
    for &(at, from, to) in &mask.parts {
        match out.iter_mut().find(|(block, _)| *block == at) {
            Some((_, Some(span))) => *span = (span.0.min(from), span.1.max(to)),
            Some((_, None)) => {}
            None => out.push((at, Some((from, to)))),
        }
    }
    out.sort_by_key(|(at, _)| *at);
    out
}

/// A region's own state, kept by its owner (one per list and view): its plan, its pending
/// requests, where it ended its last pass, and its small-room count.
pub struct RegionState {
    plan: PlanState,
    pending: Pending,
    /// The offset and the viewport at the end of its last pass.
    offset: f32,
    rect: Option<Rect>,
    /// Its column's key and view, as its content saw them in the last pass: the owner the DM's
    /// input is judged by at the start of the next (`reveal::cancel`; review of #852).
    column: Option<(Id, Rect)>,
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
    /// Blocks refused because they were drawn from a stale layout (release builds; layout 11c).
    refused: usize,
    /// The frame a pending request was first seen in, for its end after `REQUEST_FRAMES`.
    asked: Option<u64>,
    /// Where the keyboard stood at the end of the last pass (a measured body).
    stand: Option<KeyStand>,
    /// An arrow pressed at the drawn edge in the last pass, from this control: down or up.
    arrow: Option<(Id, bool)>,
    /// A Tab in a focused text field at the drawn edge in this pass: the field, the block it
    /// routes to if the field gives Tab up, and whether to its last control.
    tab_after: TabAfter,
    /// The block (by key) whose first control, or last, takes the focus once it draws; the frame
    /// it was asked in.
    focus_to: FocusTo,
}

impl RegionState {
    pub fn new(owner: &'static str) -> Self {
        Self {
            plan: PlanState::new(owner),
            pending: Pending::default(),
            offset: 0.0,
            rect: None,
            column: None,
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
            refused: 0,
            asked: None,
            stand: None,
            arrow: None,
            tab_after: None,
            focus_to: None,
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

    /// The blocks refused because they were drawn from a stale layout.
    pub fn refused(&self) -> usize {
        self.refused
    }

    /// The plan's work counters (I12) and diagnostics (L6, L7).
    #[cfg(test)]
    pub(in crate::design) fn work(&self) -> Work {
        self.plan.work()
    }
}

#[cfg(test)]
thread_local! {
    /// The blocks the in-place path of a body that is not measured looked at (tests, #565).
    pub(in crate::design) static IN_PLACE_EXAMINED: std::cell::Cell<usize> =
        const { std::cell::Cell::new(0) };
}

/// A list of blocks drawn in its region (see the module).
pub struct BlockRegion<'s> {
    id: Id,
    state: &'s mut RegionState,
    content: &'s Revised<Vec<Block>>,
    name: &'s str,
    /// As tall as its blocks, up to the room (`fit_height`).
    fit: bool,
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
            fit: false,
        }
    }

    /// As tall as its blocks, up to the room it is given, not the whole room (see the module):
    /// for a list in a child of a shared composite, so a short list does not make the composite
    /// as tall as the room and pin the pane's footer. In a `Content` pane a region draws its
    /// whole extent in place, so this changes nothing there.
    pub fn fit_height(mut self) -> Self {
        self.fit = true;
        self
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
        let plan = self.state.plan.plan(self.content, env);
        let (tallest, extent) = (plan.tallest(), plan.extent());
        if room.height() + 0.5 >= tallest {
            self.state.small = false;
            let view = if self.fit {
                room.height().min(extent)
            } else {
                room.height()
            };
            return self.region(ui, env, view, block);
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

    /// Draw the measured blocks (layout 11c) in the room left in `ui`: `laid` gives block `at`'s
    /// layout, and `draw` its controls in that layout's slots. A block drawn from a layout the
    /// plan did not measure (a stale one) is refused and counted. Returns the blocks drawn whole.
    pub fn show_laid<'l>(
        self,
        ui: &mut Ui,
        laid: impl Fn(usize) -> &'l Laid,
        mut draw: impl FnMut(&mut LaidUi<'_>, usize),
    ) -> Range<usize> {
        let Self {
            id,
            state,
            content,
            name,
            fit,
        } = self;
        let mut refused = 0;
        let whole = BlockRegion {
            id,
            state: &mut *state,
            content,
            name,
            fit,
        }
        .show(ui, |ui, at| {
            let Block::Measured(measured) = content[at] else {
                debug_assert!(false, "`show_laid` draws measured blocks only");
                return;
            };
            if !super::laid::render(ui, laid(at), measured.layout(), |ui| draw(ui, at)) {
                refused += 1;
            }
        });
        state.refused += refused;
        whole
    }

    /// In a `Body::Content` pane, which scrolls: the whole extent in place. A measured body
    /// (layout 11c) is drawn from its mask, decided before any block draws: only the blocks whole
    /// in the pane's visible room draw (a block cut at either edge has no paint, no press area
    /// and no accessible node; a hairline marks the cut), and a block taller than the room that
    /// meets it keeps its clipped paint, counted (L6), so its controls stay reachable. Another
    /// body draws the blocks in the visible clip, each registered as a row, so the pane's
    /// whole-row cut (N12) applies after paint.
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
        let Self {
            id, state, content, ..
        } = self;
        let plan = state.plan.plan(content, env);
        let (rect, _) = ui.allocate_exact_size(vec2(width, plan.extent()), egui::Sense::hover());
        let place = |plan: &BlockPlan, at: usize| {
            let (top, end) = plan.span(at);
            Rect::from_min_size(rect.min + vec2(0.0, top), vec2(width, end - top))
        };
        // Every block drawn whole is registered as a row (the pane's whole-row cut, and the
        // gate's row checks), in both bodies: a measured body's too (layout 11c A2c-3).
        let mut registered = Vec::new();
        let drawn = 'body: {
            if !plan.measured() {
                // Review of PR 555 (#565): the blocks in the visible clip, found from the plan's
                // spans (they only grow): the first block that ends below the clip's top, by
                // halving, then each block until one starts below its bottom. A frame examines
                // the blocks it draws and about `log n` more, not every block.
                let clip = ui.clip_rect();
                let (above, below) = (clip.top() - rect.top(), clip.bottom() - rect.top());
                let (mut first, mut end) = (0, plan.len());
                while first < end {
                    let mid = first + (end - first) / 2;
                    #[cfg(test)]
                    IN_PLACE_EXAMINED.with(|count| count.set(count.get() + 1));
                    if plan.span(mid).1 <= above {
                        first = mid + 1;
                    } else {
                        end = mid;
                    }
                }
                let mut drawn = 0..0;
                for at in first..plan.len() {
                    #[cfg(test)]
                    IN_PLACE_EXAMINED.with(|count| count.set(count.get() + 1));
                    if plan.span(at).0 >= below {
                        break;
                    }
                    let block_rect = place(plan, at);
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
                    registered.push(block_rect);
                }
                break 'body drawn;
            }
            // The room the pane shows this pass (its column's view inside the body's clip, after this
            // pass's input), in the content's terms: the mask is decided before any block draws.
            let clip = ui.clip_rect();
            let (offset, view) = (clip.top() - rect.top(), clip.height().max(0.0));
            let held = cancel_in_place(
                ui,
                (&mut state.pending, &mut state.asked, &mut state.focus_to),
            );
            let plan = state.plan.plan(content, env);
            route_keys(
                ui,
                Keys {
                    stand: &mut state.stand,
                    arrow: &mut state.arrow,
                    tab_after: &mut state.tab_after,
                    focus_to: &mut state.focus_to,
                    pending: &mut state.pending,
                },
                plan,
            );
            resolve_in_place(
                ui,
                (&mut state.pending, &mut state.asked),
                plan,
                held,
                (offset, view),
            );
            let mask = state.plan.mask(content, env, offset, view);
            let plan = state.plan.plan(content, env);
            let (ctx, layer) = (ui.ctx().clone(), ui.layer_id());
            // Each block drawn as parts: its parts' span and its widgets in this pass; and every
            // drawn block's widgets.
            let mut parts = Vec::new();
            let mut ranges = Vec::new();
            for (at, part) in drawn_blocks(&mask) {
                let block_rect = place(plan, at);
                // A block's `Ui` (and so its controls' ids and their focus) is its own, by its key:
                // the same whatever other blocks draw before it in this pass (review item 1).
                // A keyed block's `Ui` is its key's own (`laid::block_scope`), the same as where its
                // host draws it in place (`draw_laid`), so a switch between the two keeps its
                // controls' ids and their focus (review of PR 869).
                let scope = plan
                    .key(at)
                    .map_or_else(|| ui.id().with(("block", id, at)), super::laid::block_scope);
                let mut child = ui.new_child(
                    UiBuilder::new()
                        .id_salt(scope)
                        .global_scope(true)
                        .max_rect(block_rect),
                );
                if let Some((top, end)) = part {
                    clip_to_part(&mut child, rect.top() + top..=rect.top() + end);
                }
                let from = widget_count(&ctx, layer);
                block(&mut child, at);
                let range = from..widget_count(&ctx, layer);
                if let Some(span) = part {
                    parts.push((at, span, range.clone()));
                }
                ranges.push((at, range));
            }
            keyboard_after_draw(
                &ctx,
                layer,
                (
                    &mut state.focus_to,
                    &mut state.stand,
                    &mut state.tab_after,
                    &mut state.pending,
                ),
                plan,
                &ranges,
                &mask.whole,
            );
            // Review of PR 757 (P1): a control of a block drawn as parts that its part's clip hides
            // can take the focus by Tab. When it gains it, the unit that holds it (L6: the head, or
            // the rest) comes whole into view (a queued request), so the control shows and keeps it.
            let focused = ctx.memory(|memory| memory.focused());
            let gained = focused.filter(|id| Some(*id) != state.focused_id);
            state.focused_id = focused;
            if let Some(id) = gained
                && let Some((at, (from, to), control)) = ctx.viewport(|viewport| {
                    let widgets = &viewport.this_pass.widgets;
                    let (at_layer, index) = widgets.order(id)?;
                    let rect = widgets.get(id)?.rect;
                    parts
                        .iter()
                        .find(|(_, _, range)| at_layer == layer && range.contains(&index))
                        .map(|(at, span, _)| (*at, *span, rect))
                })
            {
                let (top, bottom) = (control.top() - rect.top(), control.bottom() - rect.top());
                if top < from - 0.5 || bottom > to + 0.5 {
                    let (start, end) = plan.span(at);
                    let unit = match plan.head(at) {
                        Some(head) if top >= start + head - 0.5 => (start + head, end),
                        Some(head) => (start, start + head),
                        None => (start, end),
                    };
                    let target = least_move(unit, offset, view);
                    if (target - offset).abs() > 0.5 {
                        crate::design::reveal::queue(ui, target - offset);
                    }
                }
            }
            // A hairline marks each cut edge: content goes on beyond it.
            let hairline = |y: f32| {
                ui.painter().hline(
                    rect.left()..=rect.right(),
                    y,
                    egui::Stroke::new(tokens::HAIRLINE_WIDTH, tokens::colors().hairline),
                );
            };
            let extent = plan.extent();
            if let Some((from, to)) = mask.eligible {
                if from > offset.max(0.0) + 0.5 {
                    hairline(clip.top() + tokens::HAIRLINE_WIDTH / 2.0);
                }
                if to < (offset + view).min(extent) - 0.5 {
                    hairline(clip.bottom() - tokens::HAIRLINE_WIDTH / 2.0);
                }
            }
            registered.extend(mask.whole.clone().map(|at| place(plan, at)));
            mask.whole
        };
        for rect in registered {
            super::super::components::register_row(ui, rect);
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
            ..
        } = self;
        let ctx = ui.ctx().clone();
        // The pass's input first, by the one cancel rule for its column (`reveal::cancel`; review
        // of #852): the DM's movement over the column or another input to it (a press, a key
        // that moves a view or a caret) cancels every pending request (R44); a press held still
        // in it makes them wait. Input to another column never cancels.
        let input = state
            .column
            .map(|(key, view)| {
                use crate::design::reveal::cancel;
                cancel::input(
                    ui,
                    cancel::Owner {
                        key,
                        view,
                        area: None,
                    },
                )
            })
            .unwrap_or_default();
        // A request this pass's own key makes (an arrow, End) waits too while the press is held.
        let (scrolled, held) = (input.moved || input.other, input.held);
        if scrolled {
            state.pending = Pending::default();
            // A queued arrow Jump's focus goes with it (issue #539): a row drawn later takes no
            // focus the DM's wheel already cancelled.
            state.focus_next = None;
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
            state.column = crate::design::reveal::column_view(ui);
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
            for (at, part) in drawn_blocks(&mask) {
                let (top, end) = plan.span(at);
                let rect = Rect::from_min_size(origin + vec2(0.0, top), vec2(width, end - top));
                let from = widget_count(&ctx, layer);
                let mut child =
                    list.new_child(UiBuilder::new().id_salt(("block", at)).max_rect(rect));
                if let Some((top, end)) = part {
                    clip_to_part(&mut child, origin.y + top..=origin.y + end);
                }
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

/// The first step of the resolver of a measured body in a `Body::Content` pane (layout 11c),
/// whose column is the pane's: run once per pass after that pass's input, before any block
/// draws. The column's one rule (`reveal::cancel`): the DM's movement over the column, or
/// another input to it (a press, a key that moves a view or a caret), cancels every OLDER
/// request; a press held still in it makes them wait, up to `REQUEST_FRAMES`. Whether one waits.
fn cancel_in_place(
    ui: &Ui,
    (pending, asked, focus_to): (&mut Pending, &mut Option<u64>, &mut FocusTo),
) -> bool {
    let frame = ui.ctx().cumulative_frame_nr();
    let (cancel, held) = crate::design::reveal::column_view(ui)
        .map(|(key, view)| {
            use crate::design::reveal::cancel;
            cancel::input(
                ui,
                cancel::Owner {
                    key,
                    view,
                    area: None,
                },
            )
        })
        .map_or((false, false), |input| {
            (
                input.moved || input.other,
                input.held && !input.moved && !input.other,
            )
        });
    if cancel || asked.is_some_and(|asked| frame > asked + REQUEST_FRAMES) {
        *pending = Pending::default();
        // Review of PR 787 (P2): a routed key's focus goes with its request.
        if cancel {
            *focus_to = None;
        }
    }
    held
}

/// The last step: the highest request (`Pending::take`, also one a key made in this pass)
/// becomes the least move of the column that shows its block whole (a block taller than the
/// view: its head at the view's top), queued for the column's next pass (`reveal::queue`),
/// which another pass of this frame draws. A request whose block left the plan ends.
fn resolve_in_place(
    ui: &Ui,
    (pending, asked): (&mut Pending, &mut Option<u64>),
    plan: &BlockPlan,
    held: bool,
    (offset, view): (f32, f32),
) {
    let frame = ui.ctx().cumulative_frame_nr();
    if *pending == Pending::default() {
        *asked = None;
        return;
    }
    asked.get_or_insert(frame);
    if held {
        return;
    }
    *asked = None;
    let target = match pending.take(plan) {
        Some(Want::Block(at)) => least_move(plan.span(at), offset, view),
        Some(Want::Origin) => 0.0,
        Some(Want::Kept(kept)) => kept,
        None => return,
    };
    if (target - offset).abs() > 0.5 {
        crate::design::reveal::queue(ui, target - offset);
    }
}

/// Where the keyboard stood in a measured body at the end of its last pass: the focused control,
/// its block, whether it is that block's first and last control, and the blocks drawn whole.
#[derive(Clone, Debug)]
struct KeyStand {
    focused: Id,
    block: usize,
    first: bool,
    last: bool,
    whole: Range<usize>,
}

/// A measured body's keyboard state, apart from its plan.
struct Keys<'s> {
    stand: &'s mut Option<KeyStand>,
    arrow: &'s mut Option<(Id, bool)>,
    tab_after: &'s mut TabAfter,
    focus_to: &'s mut FocusTo,
    pending: &'s mut Pending,
}

/// Keyboard routing across masked blocks (layout 11c, A1c; v2 review item 3). egui moves the
/// focus among the controls drawn; at the body's drawn edge this moves it to a block that is not
/// drawn: Tab from the last control of the last whole block, Shift+Tab from the first control of
/// the first, an arrow that egui could not move (nothing drawn beyond), Home and End. Only keys
/// that go to this column count (`reveal::keys_in`, by the focused control). The key's own
/// request (`Navigation::Show` of the target's key) is made AFTER the cancel step, so its own
/// movement cancels only older requests; the target's first control (its last for Shift+Tab)
/// takes the focus once it draws (`keyboard_after_draw`).
fn route_keys(ui: &Ui, keys: Keys<'_>, plan: &BlockPlan) {
    let frame = ui.ctx().cumulative_frame_nr();
    let focused = ui.ctx().memory(|memory| memory.focused());
    let Some(stand) = keys
        .stand
        .clone()
        .filter(|stand| Some(stand.focused) == focused)
    else {
        *keys.arrow = None;
        return;
    };
    let owns = crate::design::reveal::column_view(ui)
        .is_some_and(|(key, view)| crate::design::reveal::keys_in(ui, key, view));
    let n = plan.len();
    let mut target = None;
    // Review of PR 787 (P2): a focused text field keeps its keys. Home, End and the arrows move
    // its caret, so they never route (nor an arrow's fallback: the focus does not move); Tab
    // routes only when the field gives it up (egui moves the focus off it in this pass: a field
    // that indents keeps it), which `keyboard_after_draw` sees.
    if egui::text_edit::TextEditState::load(ui.ctx(), stand.focused).is_some() {
        *keys.arrow = None;
        if owns {
            let tab = |modifiers: egui::Modifiers| {
                ui.input(|input| {
                    input.events.iter().any(|event| {
                        matches!(event, egui::Event::Key { key: Key::Tab, pressed: true, modifiers: m, .. }
                            if *m == modifiers)
                    })
                })
            };
            if tab(egui::Modifiers::NONE)
                && stand.last
                && stand.block + 1 < n
                && stand.block + 1 >= stand.whole.end
            {
                *keys.tab_after = Some((stand.focused, stand.block + 1, false));
            } else if tab(egui::Modifiers::SHIFT)
                && stand.first
                && stand.block > 0
                && stand.block - 1 < stand.whole.start
            {
                *keys.tab_after = Some((stand.focused, stand.block - 1, true));
            }
        }
        return;
    }
    // An arrow of the last pass that egui could not move (the focus stayed on its control).
    if let Some((id, down)) = keys.arrow.take()
        && id == stand.focused
    {
        target = if down {
            (stand.block + 1 < n).then_some((stand.block + 1, false))
        } else {
            stand.block.checked_sub(1).map(|at| (at, false))
        };
    }
    if owns && target.is_none() {
        let pressed = |key: Key, modifiers: egui::Modifiers| {
            ui.input(|input| {
                input.events.iter().any(|event| {
                    matches!(event, egui::Event::Key { key: k, pressed: true, modifiers: m, .. }
                        if *k == key && *m == modifiers)
                })
            })
        };
        let none = egui::Modifiers::NONE;
        if pressed(Key::Tab, none)
            && stand.last
            && stand.block + 1 < n
            && stand.block + 1 >= stand.whole.end
        {
            target = Some((stand.block + 1, false));
        } else if pressed(Key::Tab, egui::Modifiers::SHIFT)
            && stand.first
            && stand.block > 0
            && stand.block - 1 < stand.whole.start
        {
            target = Some((stand.block - 1, true));
        } else if pressed(Key::Home, none) && n > 0 {
            target = Some((0, false));
        } else if pressed(Key::End, none) && n > 0 {
            target = Some((n - 1, false));
        } else if pressed(Key::ArrowDown, none) && stand.block + 1 >= stand.whole.end {
            *keys.arrow = Some((stand.focused, true));
        } else if pressed(Key::ArrowUp, none) && stand.block <= stand.whole.start {
            *keys.arrow = Some((stand.focused, false));
        }
    }
    if let Some((at, last)) = target
        && let Some(key) = plan.key(at)
    {
        *keys.focus_to = Some((key, last, frame));
        if !stand.whole.contains(&at) {
            keys.pending.navigation = Some(Navigation::Show(key));
        }
    }
    if keys.arrow.is_some() {
        // egui moves the focus by an arrow at the end of this pass: look again in the next one.
        ui.ctx()
            .request_discard("a measured body looks where an arrow left the focus");
    }
}

/// After a measured body's blocks draw: a block that a key routed to takes the focus on its first
/// control (its last for Shift+Tab) once it is drawn whole (`focus_to`, which ends after
/// `REQUEST_FRAMES`); and where the keyboard stands now, for the next pass's routing.
fn keyboard_after_draw(
    ctx: &egui::Context,
    layer: egui::LayerId,
    (focus_to, stand, tab_after, pending): (
        &mut FocusTo,
        &mut Option<KeyStand>,
        &mut TabAfter,
        &mut Pending,
    ),
    plan: &BlockPlan,
    ranges: &[(usize, Range<usize>)],
    whole: &Range<usize>,
) {
    let controls = |range: &Range<usize>| -> Vec<Id> {
        ctx.viewport(|viewport| {
            viewport
                .this_pass
                .widgets
                .get_layer(layer)
                .skip(range.start)
                .take(range.len())
                .filter(|widget| widget.enabled && widget.sense.is_focusable())
                .map(|widget| widget.id)
                .collect()
        })
    };
    // A Tab that a text field gave up (egui moved the focus off it in this pass) routes now.
    if let Some((field, at, last)) = tab_after.take()
        && ctx.memory(|memory| memory.focused()) != Some(field)
        && let Some(key) = plan.key(at)
    {
        *focus_to = Some((key, last, ctx.cumulative_frame_nr()));
        if !whole.contains(&at) {
            pending.navigation = Some(Navigation::Show(key));
            ctx.request_discard("a text field gave Tab up at a measured body's drawn edge");
        }
    }
    if let Some((key, last, asked)) = *focus_to {
        if ctx.cumulative_frame_nr() > asked + REQUEST_FRAMES {
            *focus_to = None;
        } else if let Some(at) = plan.find(key)
            && whole.contains(&at)
            && let Some((_, range)) = ranges.iter().find(|(block, _)| *block == at)
        {
            let ids = controls(range);
            if let Some(id) = if last { ids.last() } else { ids.first() } {
                ctx.memory_mut(|memory| memory.request_focus(*id));
            }
            *focus_to = None;
        }
    }
    let focused = ctx.memory(|memory| memory.focused());
    *stand = focused.and_then(|id| {
        ranges.iter().find_map(|(at, range)| {
            let ids = controls(range);
            let place = ids.iter().position(|control| *control == id)?;
            Some(KeyStand {
                focused: id,
                block: *at,
                first: place == 0,
                last: place + 1 == ids.len(),
                whole: whole.clone(),
            })
        })
    });
}

/// The least move of a view `view` tall at `offset` that shows `top..end` whole (taller than the
/// view: its top at the view's top), as the new offset.
fn least_move((top, end): (f32, f32), offset: f32, view: f32) -> f32 {
    if end - top > view || top < offset {
        top
    } else if end > offset + view {
        end - view
    } else {
        offset
    }
}

/// The widgets on `layer` so far in this pass.
fn widget_count(ctx: &egui::Context, layer: egui::LayerId) -> usize {
    ctx.viewport(|viewport| viewport.this_pass.widgets.get_layer(layer).count())
}
