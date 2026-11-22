//! Composite columns of one pane (migration-32 layout step 7).
//!
//! Children stand side by side, each with a declared width: a fixed width, a resizable
//! [`Column`] whose edge the DM drags, or the rest of the room. [`PaneColumns::edges`] resolves every
//! child's edges ONCE, before anything of the pane paints, from the room and this pass's pending
//! splitter drag or key (`Column::width_within`). The result, [`PaneEdges`], is the one object
//! that both the header and the body use: [`PaneEdges::header`] bounds a header line by the
//! composite's outer edge (its children and their gutters, never the window), and
//! [`PaneEdges::show_children`] draws the children in those same edges. A child's own toolbar
//! uses [`PaneEdges::child_header`]. No API takes a child's edge where the composite's is meant.
//!
//! Edges belong to one pass and one room. Edges kept to a later pass, or used in another room (a
//! resize), are refused: the part resolves them again from the current room, so a stale width
//! never draws (`rejected_edges` counts each refusal for the tests). They cannot be copied, and
//! the children are drawn once:
//!
//! ```compile_fail,E0382
//! fn twice(ui: &mut egui::Ui) {
//!     let edges = splat_design::pane::PaneColumns::shared_task("doc_columns").rest(0.0).edges(ui);
//!     edges.show_children(ui, |_, _| ());
//!     edges.show_children(ui, |_, _| ());
//! }
//! ```
//!
//! ```
//! fn once(ui: &mut egui::Ui) {
//!     let edges = splat_design::pane::PaneColumns::shared_task("doc_columns").rest(0.0).edges(ui);
//!     edges.header(ui, |ui| ui.label("Title"));
//!     edges.show_children(ui, |_, _| ());
//! }
//! ```
//!
//! A caller cannot make edges of its own:
//!
//! ```compile_fail,E0451
//! fn made() -> splat_design::pane::PaneEdges {
//!     splat_design::pane::PaneEdges { pass: 0, room: egui::Rangef::new(0.0, 1.0), children: Vec::new(), spec: todo!() }
//! }
//! ```
//!
//! Two tasks:
//!
//! - **Shared** (`PaneColumns::shared_task`): the children are one task in one pane body, with the
//!   pane's one footer. In a `Body::Content` pane, the composite is as tall as its tallest child's
//!   whole content (each child's scroll columns add the height they hide), up to the room left in
//!   the pane, and never less than its least height while its content is taller. So the footer
//!   follows the taller child by `S` when both fit, and the gutter's rule ends there. Each child
//!   scrolls on its own when it is taller. In any other body, the composite fills the room left.
//! - **Independent** (`PaneColumns::independent`): unrelated tasks. Each child is a whole-height
//!   host of its own pane and footer, so one child's results never move the other's content.
//!
//! Stable rustdoc does not check the error codes above, so each failing case differs from the
//! compiling case after it only by the rule it shows.
use super::{OpenPane, innermost, open};
use crate::design::column::Column;
use egui::{self, Id, Rangef, Rect, Sense, Ui};

/// Whether the children are one task or unrelated ones.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Task {
    Shared,
    Independent,
}

#[derive(Clone)]
enum Width {
    Fixed(f32),
    Resizable(Box<Column>),
    Rest,
    /// A last child at the room's right, after the rest, whose left edge the DM drags
    /// (`Column::right`).
    Trailing(Box<Column>),
}

#[derive(Clone)]
struct Child {
    /// The gutter before the child.
    before: f32,
    width: Width,
}

/// The children of a composite, before their edges are resolved.
#[derive(Clone)]
pub struct PaneColumns {
    id: Id,
    task: Task,
    children: Vec<Child>,
    least: f32,
}

impl PaneColumns {
    /// Children that are one task: one footer follows the taller child.
    pub fn shared_task(id: impl std::hash::Hash) -> Self {
        Self::new(id, Task::Shared)
    }

    /// Children that are unrelated tasks: each hosts its own pane and footer.
    pub fn independent(id: impl std::hash::Hash) -> Self {
        Self::new(id, Task::Independent)
    }

    fn new(id: impl std::hash::Hash, task: Task) -> Self {
        Self {
            id: Id::new(id),
            task,
            children: Vec::new(),
            least: 0.0,
        }
    }

    fn push(mut self, before: f32, width: Width) -> Self {
        debug_assert!(
            !self
                .children
                .last()
                .is_some_and(|child| matches!(child.width, Width::Trailing(_))),
            "a trailing column is the last child"
        );
        debug_assert!(
            !self
                .children
                .last()
                .is_some_and(|child| matches!(child.width, Width::Rest))
                || matches!(width, Width::Trailing(_)),
            "the rest of the room is the last child, or only a trailing column follows it"
        );
        debug_assert!(
            !matches!(width, Width::Trailing(_))
                || self
                    .children
                    .last()
                    .is_some_and(|child| matches!(child.width, Width::Rest)),
            "a trailing column follows the rest of the room"
        );
        debug_assert!(
            !self
                .children
                .last()
                .is_some_and(|child| matches!(child.width, Width::Resizable(_)))
                || matches!(width, Width::Rest),
            "only the rest of the room follows a resizable column: its splitter is its right edge"
        );
        self.children.push(Child { before, width });
        self
    }

    /// A child `width` wide (less if the room ends first), `before` after the previous one.
    pub fn fixed(self, before: f32, width: f32) -> Self {
        self.push(before, Width::Fixed(width))
    }

    /// A child whose right edge the DM drags (`Column`, on the left of what follows it).
    pub fn resizable(self, before: f32, column: Column) -> Self {
        self.push(before, Width::Resizable(Box::new(column)))
    }

    /// A child with the rest of the room, `before` after the previous one: the last child, or
    /// the one before a trailing column.
    pub fn rest(self, before: f32) -> Self {
        self.push(before, Width::Rest)
    }

    /// A last child at the room's right, after the rest of the room and `before` from it, whose
    /// left edge the DM drags (`Column::right`, a page's side panel such as Add participants).
    pub fn trailing(self, before: f32, column: Column) -> Self {
        self.push(before, Width::Trailing(Box::new(column)))
    }

    /// A shared composite in a `Content` pane keeps at least this height while its content is
    /// taller, before the pane scrolls instead (a few rows of each list).
    pub fn least_height(mut self, height: f32) -> Self {
        self.least = height;
        self
    }

    /// Each child's edges in `room`, from this pass's widths and pending drags.
    fn resolve(&self, ctx: &egui::Context, room: Rect) -> Vec<Rangef> {
        let mut x = room.left();
        // A trailing column's width, from the room its side panel has: from the rest's left to the
        // room's right.
        let mut trailing = None;
        self.children
            .iter()
            .enumerate()
            .map(|(at, child)| {
                let left = (x + child.before).min(room.right());
                let right = match &child.width {
                    Width::Rest => match self.children.get(at + 1) {
                        Some(Child {
                            before,
                            width: Width::Trailing(column),
                        }) => {
                            let width = column.width_within(
                                ctx,
                                Rect::from_x_y_ranges(left..=room.right(), room.y_range()),
                            );
                            trailing = Some(width);
                            (room.right() - width - before).max(left)
                        }
                        _ => room.right(),
                    },
                    Width::Trailing(_) => {
                        let width = trailing.unwrap_or(0.0);
                        x = room.right();
                        return Rangef::new(room.right() - width, room.right());
                    }
                    Width::Fixed(width) => left + width.min(room.right() - left).max(0.0),
                    Width::Resizable(column) => {
                        left + column.width_within(
                            ctx,
                            Rect::from_x_y_ranges(left..=room.right(), room.y_range()),
                        )
                    }
                };
                x = right;
                Rangef::new(left, right)
            })
            .collect()
    }

    /// Resolve every child's edges now, before the header paints: from `ui`'s room and this
    /// pass's splitter drag or key. Give the result to the header and to the body.
    pub fn edges(self, ui: &Ui) -> PaneEdges {
        let room = ui.available_rect_before_wrap();
        self.edges_at(ui, room)
    }

    /// `edges` for a composite `width` wide at the start of `ui`'s room (panes that keep to the
    /// left of a wider page), resolved before the header paints. The body draws the children in a
    /// `Ui` of that width (`show_children` checks it), and the header reads a child's edges with
    /// [`PaneEdges::child_within`] and the same width.
    pub fn edges_in(self, ui: &Ui, width: f32) -> PaneEdges {
        let room = narrowed(ui, width);
        self.edges_at(ui, room)
    }

    fn edges_at(self, ui: &Ui, room: Rect) -> PaneEdges {
        PaneEdges {
            children: self.resolve(ui.ctx(), room),
            pass: ui.ctx().cumulative_pass_nr(),
            room: room.x_range(),
            spec: self,
        }
    }
}

/// A composite's resolved edges, for one pass and one room (`PaneColumns::edges`).
pub struct PaneEdges {
    spec: PaneColumns,
    pass: u64,
    room: Rangef,
    children: Vec<Rangef>,
}

fn rejected_id() -> Id {
    Id::new("design_rejected_edges")
}

/// How many times edges were refused (kept to a later pass, or used in another room) and
/// resolved again.
#[cfg(any(test, feature = "test-support"))]
pub fn rejected_edges(ctx: &egui::Context) -> u64 {
    ctx.data(|data| data.get_temp::<u64>(rejected_id()))
        .unwrap_or(0)
}

/// For `ScrollColumn`: `hidden` more height of the child measured in `slot`.
pub(super) fn add_hidden(ctx: &egui::Context, slot: Id, hidden: f32) {
    let pass = ctx.cumulative_pass_nr();
    ctx.data_mut(|data| {
        let kept = data.get_temp_mut_or_insert_with(slot, || (pass, 0.0_f32));
        if kept.0 != pass {
            *kept = (pass, 0.0);
        }
        kept.1 += hidden;
    });
}

fn outer(children: &[Rangef], room: Rangef) -> Rangef {
    match (children.first(), children.last()) {
        (Some(first), Some(last)) => Rangef::new(first.min, last.max),
        _ => room,
    }
}

/// `ui`'s room, `width` wide from its left.
fn narrowed(ui: &Ui, width: f32) -> Rect {
    let room = ui.available_rect_before_wrap();
    room.with_max_x((room.left() + width).min(room.right()))
}

/// Draw `add` with its right edge at `right`.
fn bounded<R>(ui: &mut Ui, right: f32, add: impl FnOnce(&mut Ui) -> R) -> R {
    ui.scope(|ui| {
        let left = ui.max_rect().left();
        ui.set_max_width((right - left).max(0.0));
        add(ui)
    })
    .inner
}

impl PaneEdges {
    /// The edges for `ui`'s pass and room: these, or resolved again when they are stale.
    fn current(&self, ui: &Ui) -> std::borrow::Cow<'_, [Rangef]> {
        self.current_in(ui, ui.available_rect_before_wrap())
    }

    fn current_in(&self, ui: &Ui, room: Rect) -> std::borrow::Cow<'_, [Rangef]> {
        let same = |a: f32, b: f32| (a - b).abs() <= 0.5;
        if self.pass == ui.ctx().cumulative_pass_nr()
            && same(self.room.min, room.left())
            && same(self.room.max, room.right())
        {
            return std::borrow::Cow::Borrowed(&self.children);
        }
        ui.ctx().data_mut(|data| {
            *data.get_temp_mut_or_default::<u64>(rejected_id()) += 1;
        });
        std::borrow::Cow::Owned(self.spec.resolve(ui.ctx(), room))
    }

    /// Child `index`'s edges for a composite `width` wide at the start of `ui`'s room
    /// (`PaneColumns::edges_in`); kept edges are resolved again.
    pub fn child_within(&self, ui: &Ui, width: f32, index: usize) -> Rangef {
        self.current_in(ui, narrowed(ui, width))[index]
    }

    /// The composite's outer edges in `ui`'s pass and room: from its first child's left to its
    /// last child's right. Kept edges are refused and resolved again, as for `header`.
    pub fn outer(&self, ui: &Ui) -> Rangef {
        outer(&self.current(ui), self.room)
    }

    /// Child `index`'s edges in `ui`'s pass and room (kept edges are resolved again).
    pub fn child(&self, ui: &Ui, index: usize) -> Rangef {
        self.current(ui)[index]
    }

    /// A header line of the whole composite (a title with its actions, tabs), drawn by `add` with
    /// its right edge at the composite's outer edge.
    pub fn header<R>(&self, ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
        let right = outer(&self.current(ui), self.room).max;
        bounded(ui, right, add)
    }

    /// A header line of child `index` alone (its toolbar), drawn by `add` with its right edge at
    /// that child's right edge.
    pub fn child_header<R>(&self, index: usize, ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
        let right = self.current(ui)[index].max;
        bounded(ui, right, add)
    }

    /// Draw the children from the cursor down, each by `add` with its index, in these edges.
    pub fn show_children(self, ui: &mut Ui, mut add: impl FnMut(&mut Ui, usize)) {
        let edges = self.current(ui).into_owned();
        let spec = &self.spec;
        let ctx = ui.ctx().clone();
        let pass = ctx.cumulative_pass_nr();
        let room = ui.available_rect_before_wrap();
        let top = room.top();
        let parent = innermost(ui);
        let whole_key = spec.id.with("whole_height");
        let stored = ctx.data(|data| data.get_temp::<f32>(whole_key));
        // In a `Content` pane, a shared composite is as tall as its tallest child, up to the room
        // left below it (in the scrolled layout, so the offset changes nothing), and at least its
        // least height while its content is taller. Anywhere else it fills the room left.
        let extent = parent
            .filter(|pane| pane.content)
            .and_then(|pane| pane.extent);
        let height = match (spec.task, extent) {
            (Task::Shared, Some((origin, room_height))) => {
                let cap = (room_height - (top - origin)).max(spec.least);
                stored.map_or(cap, |whole| whole.min(cap))
            }
            _ => room.height(),
        }
        .max(0.0);
        let rect = Rect::from_x_y_ranges(outer(&edges, self.room), top..=top + height);
        let clip = ui.clip_rect();
        let mut whole = 0.0_f32;
        // Only a shared composite in a `Content` pane is as tall as its content.
        let measured = spec.task == Task::Shared && extent.is_some();
        // A child draws in its own `Ui`, clipped to its edges and the parent's clip. A shared
        // child's scroll columns add what they hide to its slot: its whole height.
        let mut child = |ui: &mut Ui, index: usize| {
            ui.set_clip_rect(ui.clip_rect().intersect(clip));
            if !measured {
                add(ui, index);
                return;
            }
            let slot = spec.id.with(("hidden", index));
            ctx.data_mut(|data| data.insert_temp(slot, (pass, 0.0_f32)));
            open(
                ui,
                Some(OpenPane {
                    layer: ui.layer_id(),
                    content: false,
                    usable: parent
                        .map_or(clip, |pane| pane.usable)
                        .intersect(ui.max_rect()),
                    edge: parent.is_some_and(|pane| pane.edge),
                    pane: parent.map_or(spec.id, |pane| pane.pane),
                    measure: Some(slot),
                    extent: None,
                }),
            );
            // Its own scope: egui's side panel makes the `Ui` it gives as tall as the panel, so
            // only what the child drew in the scope is its content.
            let drawn = ui.scope(|ui| add(ui, index)).response.rect;
            open(ui, None);
            let hidden = ctx
                .data(|data| data.get_temp::<(u64, f32)>(slot))
                .filter(|(at, _)| *at == pass)
                .map_or(0.0, |(_, hidden)| hidden);
            whole = whole.max(drawn.bottom() - top + hidden);
        };
        let layout = egui::Layout::top_down(egui::Align::Min);
        let mut index = 0;
        while index < spec.children.len() {
            let column = rect
                .with_min_x(edges[index].min)
                .with_max_x(edges[index].max);
            let trailing = match spec.children.get(index + 1) {
                Some(Child {
                    before,
                    width: Width::Trailing(trailing),
                }) if matches!(spec.children[index].width, Width::Rest) => {
                    Some((*before, trailing))
                }
                _ => None,
            };
            match (&spec.children[index].width, trailing) {
                (Width::Rest, Some((before, trailing))) => {
                    // The trailing column's splitter is its own (a side panel at the right of a
                    // room from the rest's left), drawn first; the rest takes what it leaves.
                    let mut beside = ui.new_child(
                        egui::UiBuilder::new()
                            .id_salt(spec.id.with(("column", index)))
                            .max_rect(rect.with_min_x(edges[index].min))
                            .layout(layout),
                    );
                    (**trailing)
                        .clone()
                        .show_inside(&mut beside, |ui| child(ui, index + 1));
                    let left = beside.available_rect_before_wrap();
                    let mut ui = beside.new_child(
                        egui::UiBuilder::new()
                            .id_salt(spec.id.with(("column", index + 1)))
                            .max_rect(left.with_max_x((left.right() - before).max(left.left())))
                            .layout(layout),
                    );
                    child(&mut ui, index);
                    index += 1;
                }
                (Width::Trailing(_), _) => {}
                (Width::Fixed(_) | Width::Rest, _) => {
                    let mut ui = ui.new_child(
                        egui::UiBuilder::new()
                            .id_salt(spec.id.with(("column", index)))
                            .max_rect(column)
                            .layout(layout),
                    );
                    child(&mut ui, index);
                }
                (Width::Resizable(resizable), _) => {
                    // The splitter is the column's own (a side panel), in a room from its left to
                    // the room's right; the rest of the room follows it there.
                    let mut beside = ui.new_child(
                        egui::UiBuilder::new()
                            .id_salt(spec.id.with(("column", index)))
                            .max_rect(rect.with_min_x(edges[index].min))
                            .layout(layout),
                    );
                    (**resizable)
                        .clone()
                        .show_inside(&mut beside, |ui| child(ui, index));
                    if let Some(next) = spec.children.get(index + 1) {
                        index += 1;
                        // A trailing column after the rest (Astra's P2 on PR 668): its side panel
                        // at the room's right, drawn before the rest takes what is left.
                        let trailing = match spec.children.get(index + 1) {
                            Some(Child {
                                before,
                                width: Width::Trailing(trailing),
                            }) => {
                                (**trailing)
                                    .clone()
                                    .show_inside(&mut beside, |ui| child(ui, index + 1));
                                Some(*before)
                            }
                            _ => None,
                        };
                        let rest = beside.available_rect_before_wrap();
                        let rest = rest
                            .with_min_x((rest.left() + next.before).min(rest.right()))
                            .with_max_x((rest.right() - trailing.unwrap_or(0.0)).max(rest.left()));
                        let mut ui = beside.new_child(
                            egui::UiBuilder::new()
                                .id_salt(spec.id.with(("column", index)))
                                .max_rect(rest)
                                .layout(layout),
                        );
                        child(&mut ui, index);
                        if trailing.is_some() {
                            index += 1;
                        }
                    }
                }
            }
            index += 1;
        }
        ui.allocate_rect(rect, Sense::hover());
        if measured && stored.is_none_or(|stored| (stored - whole).abs() > 0.5) {
            ctx.data_mut(|data| data.insert_temp(whole_key, whole));
            ctx.request_discard("composite columns measured their children");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::design::tokens::L;
    use egui::{Event, Modifiers, PointerButton, vec2};
    use egui_kittest::{
        Harness,
        kittest::{NodeT, Queryable},
    };
    use std::{cell::Cell, rc::Rc};

    fn right_of(harness: &Harness<'_>, label: &str) -> f32 {
        edges_of(harness, label).1
    }

    fn edges_of(harness: &Harness<'_>, label: &str) -> (f32, f32) {
        let node = harness.get_by_label(label);
        node.accesskit_node()
            .bounding_box()
            .map_or((f32::NAN, f32::NAN), |b| (b.x0 as f32, b.x1 as f32))
    }

    /// Layout step 7: a composite of a fixed child, a gutter and a resizable child (Session's
    /// plan and note). A header's right-aligned action ends at the composite's outer edge, a
    /// child's toolbar at that child's edge, and both at the edge the child really draws (a
    /// control as wide as the child), not at the window's: at 960 and 1920, in every pass of a
    /// splitter drag, and after a resize. Edges kept to the next pass are refused and resolved
    /// again: across a resize they would draw the header at the old width.
    #[test]
    fn a_composite_header_and_its_children_share_their_edges() {
        use crate::design::test_harness::ThemedPage as _;
        let stale = Rc::new(Cell::new(false));
        let stale_in = stale.clone();
        // What `outer` and `child` say in the pass, from the edges the pass uses.
        let said = Rc::new(Cell::new((0.0_f32, 0.0_f32)));
        let said_in = said.clone();
        let mut kept: Option<PaneEdges> = None;
        let mut harness = Harness::builder()
            .with_size(vec2(1920.0, 640.0))
            .themed_ui(move |ui| {
                let spec = PaneColumns::shared_task("edges_test")
                    .fixed(0.0, 200.0)
                    .resizable(
                        L,
                        Column::left("edges_note", 300.0, 150.0..=f32::INFINITY).page_keeps(0.0),
                    );
                let fresh = spec.clone().edges(ui);
                let edges = if stale_in.get() {
                    kept.take().unwrap_or(fresh)
                } else {
                    fresh
                };
                kept = Some(spec.edges(ui));
                said_in.set((edges.outer(ui).max, edges.child(ui, 1).max));
                edges.header(ui, |ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                        let _ = ui.button("Record");
                    });
                });
                edges.child_header(1, ui, |ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                        let _ = ui.button("Read");
                    });
                });
                edges.show_children(ui, |ui, index| {
                    let width = ui.available_width();
                    let _ = ui.add_sized([width, 20.0], egui::Button::new(format!("Fill {index}")));
                });
            });
        harness.run_steps(3);
        let check = |harness: &Harness<'_>, case: &str| {
            let (record, read, fill) = (
                right_of(harness, "Record"),
                right_of(harness, "Read"),
                right_of(harness, "Fill 1"),
            );
            let plan = edges_of(harness, "Fill 0");
            assert!(
                (record - fill).abs() <= 1.0 && (read - fill).abs() <= 1.0,
                "{case}: Record ends at {record}, Read at {read}, the note child at {fill}"
            );
            assert!(
                (plan.1 - plan.0 - 200.0).abs() <= 1.0,
                "{case}: the fixed child is 200 px wide: {plan:?}"
            );
            fill
        };
        let wide = check(&harness, "1920");
        assert!(
            wide < 1000.0,
            "the edge is the composite's, not the window's: {wide}"
        );
        // Drag the splitter 80 px right: every pass of the drag agrees.
        let handle = harness
            .ctx
            .read_response(Id::new("edges_note").with("__resize"))
            .expect("the splitter shows")
            .rect
            .center();
        harness.hover_at(handle);
        harness.step();
        for (pos, pressed) in [
            (handle, true),
            (handle + vec2(40.0, 0.0), true),
            (handle + vec2(80.0, 0.0), true),
            (handle + vec2(80.0, 0.0), false),
        ] {
            harness.hover_at(pos);
            harness.event(Event::PointerButton {
                pos,
                button: PointerButton::Primary,
                pressed,
                modifiers: Modifiers::NONE,
            });
            harness.step();
            check(&harness, &format!("dragging at {pos:?}"));
        }
        harness.remove_cursor();
        harness.run_steps(2);
        let dragged = check(&harness, "after the drag");
        assert!(
            (dragged - (wide + 80.0)).abs() <= 1.0,
            "the drag moved the edge by 80 px: {wide} to {dragged}"
        );
        harness.set_size(vec2(960.0, 640.0));
        harness.run_steps(3);
        check(&harness, "960");
        // Edges kept from the last pass, then a resize to a room narrower than the note: refused
        // and resolved again, so the header still ends where the child does.
        let rejected = rejected_edges(&harness.ctx);
        stale.set(true);
        harness.set_size(vec2(420.0, 640.0));
        harness.step();
        let narrow = check(&harness, "kept edges after a resize to 420");
        assert!(narrow <= 420.0 + 0.5, "the note fits the room: {narrow}");
        assert!(
            (said.get().0 - narrow).abs() <= 1.0 && (said.get().1 - narrow).abs() <= 1.0,
            "kept edges: outer and child say {:?}, the note child ends at {narrow}",
            said.get()
        );
        assert!(
            rejected_edges(&harness.ctx) > rejected,
            "the kept edges were refused"
        );
        // Layout step 6b: the rest of the room and a trailing column at its right (the encounter
        // editor's roster and Add participants), in panes narrower than the page
        // (`edges_in`). The rest's edge read before the body (`child_within`) is where the rest
        // really ends, and the trailing column starts there, at 960 and 1920 and through a drag
        // of its splitter.
        let mut trailing = Harness::builder()
            .with_size(vec2(1920.0, 640.0))
            .themed_ui(move |ui| {
                let width = (ui.available_width() - 100.0).min(1200.0);
                let spec = || {
                    PaneColumns::shared_task("trailing_test")
                        .rest(0.0)
                        .trailing(0.0, Column::right("trailing_add", 400.0, 300.0..=600.0))
                };
                let edges = spec().edges_in(ui, width);
                let rest = edges.child_within(ui, width, 0);
                ui.horizontal(|ui| {
                    let _ = ui.add_sized([rest.span(), 20.0], egui::Button::new("Head"));
                });
                ui.allocate_ui_with_layout(
                    vec2(width, ui.available_height()),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        ui.set_width(width);
                        edges.show_children(ui, |ui, index| {
                            let width = ui.available_width();
                            let _ = ui.add_sized(
                                [width, 20.0],
                                egui::Button::new(format!("Part {index}")),
                            );
                        });
                    },
                );
            });
        let check = |harness: &Harness<'_>, case: &str| {
            let head = right_of(harness, "Head");
            let rest = right_of(harness, "Part 0");
            let add = edges_of(harness, "Part 1");
            assert!(
                (head - rest).abs() <= 1.0 && add.0 >= rest - 0.5 && add.1 > add.0,
                "{case}: the head says the rest ends at {head}, it ends at {rest}, Add is {add:?}"
            );
        };
        trailing.run_steps(3);
        check(&trailing, "1920");
        let handle = trailing
            .ctx
            .read_response(Id::new("trailing_add").with("__resize"))
            .expect("the trailing splitter shows")
            .rect
            .center();
        trailing.hover_at(handle);
        trailing.step();
        for (pos, pressed) in [
            (handle, true),
            (handle - vec2(60.0, 0.0), true),
            (handle - vec2(60.0, 0.0), false),
        ] {
            trailing.hover_at(pos);
            trailing.event(Event::PointerButton {
                pos,
                button: PointerButton::Primary,
                pressed,
                modifiers: Modifiers::NONE,
            });
            trailing.step();
            check(&trailing, &format!("dragging at {pos:?}"));
        }
        trailing.set_size(vec2(960.0, 640.0));
        trailing.run_steps(3);
        check(&trailing, "960");
        // A resizable column, the rest and a trailing column (Astra's P2 on PR 668): all three
        // draw, each in its resolved edges, the rest between the two splitters.
        let three = Rc::new(std::cell::RefCell::new(Vec::new()));
        let three_in = three.clone();
        let mut both = Harness::builder()
            .with_size(vec2(1600.0, 640.0))
            .themed_ui(move |ui| {
                let edges = PaneColumns::shared_task("three_test")
                    .resizable(0.0, Column::left("three_left", 300.0, 200.0..=400.0))
                    .rest(0.0)
                    .trailing(0.0, Column::right("three_right", 300.0, 200.0..=400.0))
                    .edges(ui);
                *three_in.borrow_mut() = (0..3).map(|at| edges.child(ui, at)).collect();
                edges.show_children(ui, |ui, index| {
                    let width = ui.available_width();
                    let _ =
                        ui.add_sized([width, 20.0], egui::Button::new(format!("Third {index}")));
                });
            });
        both.run_steps(3);
        let planned = three.borrow().clone();
        for (index, planned) in planned.iter().enumerate() {
            let drawn = edges_of(&both, &format!("Third {index}"));
            assert!(
                (drawn.0 - planned.min).abs() <= 1.0 && (drawn.1 - planned.max).abs() <= 1.0,
                "child {index} is drawn at {drawn:?}, planned at {planned:?}"
            );
        }
    }
}
