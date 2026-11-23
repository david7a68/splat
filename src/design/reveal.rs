//! One reveal part (mechanism M6, rulings R34, R43, R44): after the DM opens a form, the least
//! scroll that brings the whole form into view, in the scroll column that holds the pressed
//! control and the form.
//!
//! - **A ticket, issued once per real activation.** [`Act::clicked`] gives an act only when this
//!   pass's own input activated that control (a primary press released on it, `Enter` or `Space`
//!   with its focus, or an accessibility click on it), and only once per control and frame. An
//!   `Act` is neither `Copy` nor `Clone`; [`Reveal::ask`] consumes it and refuses one of another
//!   frame. The act names the `ScrollColumn` that holds the pressed control; the ask binds the
//!   request to its owner (the [`Reveal`] value), its visit, its target and that column.
//! - **Marks report geometry only.** [`Reveal::mark`] gives the bound target's rect in this pass.
//!   A mark for another target, or in another column, does nothing.
//! - **The bound column resolves the request after its content, on every pass** (`region`):
//!   input first, also before the target has drawn; then progress, measured by the column's real
//!   offset and the target's place in its content. The scroll is the offset the column applies
//!   at the start of its next pass (`take_offset`), after it checked that pass's input. Nothing
//!   in the app animates a programmatic scroll (R43, [`super::theme::install`]); the part's own
//!   offset is not an egui request at all.
//! - **It ends for good** ([`End`]): the target whole in view; a frame with no progress; a form
//!   taller than the view after one scroll to its top (Q2.1); the DM's input to its column, read
//!   where the column starts by the one cancel rule ([`cancel`]: a wheel over it, a press or drag
//!   in it, a key, text or accessibility action that goes to it), of which only the input that
//!   made the act is exempt, in the act's frame; input to another column never ends it (Sol
//!   2-011); a focus change after the ask other than the opener's one focus inside the form,
//!   unless it went to a control outside its column or input to another column moved it; the
//!   owner's end (the `Reveal` dropped or [`Reveal::end`]; another visit; a pass of its column
//!   that does not draw a target it drew before; a frame in which its column was not drawn); a
//!   new ask (`Replaced`); [`PASSES`] scroll frames; and a deadline of `DEADLINE` frames from the
//!   act, which also ends a request whose target never shows. A resize never restarts it.
//! - **A press whose result comes later** ([`Reveal::ask_on_result`], issue #698): the request
//!   waits, moving nothing and with no deadline, until the owner says the result came
//!   ([`Reveal::result_came`]); its passes and its deadline start then. Every other end applies
//!   while it waits, judged by its own column (review of #838): the DM's input to that column
//!   (the column queue's rule) or leaving the page ends it, and input to another column, a focus
//!   in another column included, does not.
//!
//! A run that ended leaves the registry of live requests at once (its `End` stays with its
//! owner), so a column's work per pass is a look at the requests that run now.
use std::{
    hash::Hash,
    sync::{Arc, Mutex, Weak},
};

use egui::{self, Context, Id, Rect, Response, Ui, Vec2};

/// A reveal's bound: frames in which it scrolls.
pub const PASSES: u8 = 4;

/// The frames after its act at which a reveal ends, whatever happened.
const DEADLINE: u64 = 2 * PASSES as u64;

/// The act that may ask for a reveal: a press of this frame on a control in a `ScrollColumn`.
/// Neither `Copy` nor `Clone`: [`Reveal::ask`] consumes it.
#[derive(Debug)]
pub struct Act {
    ctx: Context,
    frame: u64,
    id: Id,
    /// The input events that made the act (the press's release and its press, the key, or the
    /// accessibility click): in the act's frame these, and nothing else, are the act's.
    exempt: Vec<egui::Event>,
    /// The `ScrollColumn` that holds the pressed control; `None` outside one.
    column: Option<Id>,
}

impl Act {
    /// The act of `response`'s control, drawn in `ui` in this pass and activated by this pass's
    /// input, as the context records it now: enabled and clicked in this pass. The saved
    /// `Response` gives only the control's id; its flags and rect are not trusted. `None` when
    /// the control was not activated in this pass, or when an act was already given for it in
    /// this frame.
    pub fn clicked(ui: &Ui, response: &Response) -> Option<Self> {
        let ctx = ui.ctx();
        let id = response.id;
        let drawn = ctx.viewport(|viewport| viewport.this_pass.widgets.get(id).is_some());
        let current = ctx
            .read_response(id)
            .filter(|current| drawn && current.enabled() && current.clicked())?;
        let rect = current.interact_rect;
        let focused = ctx.memory(|memory| memory.has_focus(id));
        let node = egui::accesskit::NodeId::from(id.value());
        let exempt = cancel::act_events(ui, node, focused, rect)?;
        let frame = ctx.cumulative_frame_nr();
        // One act per control and frame.
        let given = ctx.data_mut(|data| {
            let given = data.get_temp_mut_or_default::<Given>(Id::new("design_reveal_given"));
            if given.frame != frame {
                *given = Given {
                    frame,
                    ids: Vec::new(),
                };
            }
            let seen = given.ids.contains(&id);
            given.ids.push(id);
            seen
        });
        (!given).then(|| Self {
            ctx: ctx.clone(),
            frame,
            id,
            exempt,
            column: owning_region(ui)
                .filter(|(_, region)| region.hosts)
                .map(|(_, region)| region.key),
        })
    }

    /// An act in this frame, in the column `ui` is drawn in, for a test that opens a form with
    /// no press. No input of the frame is the act's.
    #[cfg(any(test, feature = "test-support"))]
    pub fn test(ui: &Ui, salt: impl Hash) -> Self {
        Self {
            column: owning_region(ui)
                .filter(|(_, region)| region.hosts)
                .map(|(_, region)| region.key),
            ..Self::outside(ui.ctx(), salt)
        }
    }

    /// An act in this frame of `ctx` and in no column, for a test with no frame drawn: its
    /// reveal ends `Unowned`.
    #[cfg(any(test, feature = "test-support"))]
    pub fn outside(ctx: &Context, salt: impl Hash) -> Self {
        Self {
            ctx: ctx.clone(),
            frame: ctx.cumulative_frame_nr(),
            id: Id::new(("reveal_test_act", salt)),
            exempt: Vec::new(),
            column: None,
        }
    }
}

/// The controls that gave an act in a frame.
#[derive(Clone, Default)]
struct Given {
    frame: u64,
    ids: Vec<Id>,
}

/// Why a reveal ended. Every end is for good.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum End {
    Whole,
    NoProgress,
    TooTall,
    ByTheDm,
    /// The keyboard focus moved after the ask (other than the opener's one focus in the form).
    Focus,
    OwnerGone,
    /// The pressed control is not in a `ScrollColumn` that hosts reveals (R46): nothing the
    /// part may scroll.
    Unowned,
    Bound,
    Replaced,
}

#[derive(Clone, Copy, Debug)]
struct Asked {
    frame: u64,
    /// The column's offset and the target's top in its content, when it asked.
    offset: f32,
    content_top: f32,
}

#[derive(Debug)]
struct Run {
    act_frame: u64,
    act_id: Id,
    exempt: Vec<egui::Event>,
    visit: Id,
    target: Id,
    phase: Result<(), End>,
    deadline: u64,
    left: u8,
    /// The column of the pressed control, bound at the ask.
    column: Id,
    /// The keyboard focus at the ask, then as last seen.
    focus: Option<Id>,
    /// The opener's field: the one control in the form that may take the focus.
    opener_field: Option<Id>,
    /// The target's rect, the column's content `Ui` and the pass, as marked.
    marked: Option<(Rect, Id, u64)>,
    /// The target has drawn in its column.
    drawn: bool,
    /// The last frame its column resolved it.
    frame: Option<u64>,
    asked: Option<Asked>,
    /// The form is taller than the view: one scroll to its top, then `TooTall`.
    edge: bool,
    /// The offset for the column's next pass, with the frame that wrote it.
    pending: Option<(f32, u64)>,
    /// It waits for the result of the work its act started (`Reveal::ask_on_result`): it moves
    /// nothing and has no deadline until `Reveal::result_came`.
    waiting: bool,
    /// Its column's view (screen points) in the last pass that resolved it: where its owner's
    /// input is judged at the column's start (`take_offset`).
    view: Option<Rect>,
    /// The last frame in which the DM's input went to another column (`cancel::Read::elsewhere`).
    elsewhere: Option<u64>,
}

impl Run {
    fn end(&mut self, end: End) {
        if self.phase.is_ok() {
            self.phase = Err(end);
            self.pending = None;
        }
    }
}

type Shared = Arc<Mutex<Run>>;

fn lock(run: &Mutex<Run>) -> std::sync::MutexGuard<'_, Run> {
    run.lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// The running requests, for the columns. The owner holds the only strong handle: dropping its
/// `Reveal` ends the request, and its queued offset with it.
#[derive(Clone, Default)]
struct Registry(Vec<Weak<Mutex<Run>>>);

fn registry_id() -> Id {
    Id::new("design_reveal_registry")
}

/// The running requests; a dropped or ended one leaves the registry here.
fn running(ctx: &Context) -> Vec<Shared> {
    ctx.data_mut(|data| {
        let registry = data.get_temp_mut_or_default::<Registry>(registry_id());
        let mut live = Vec::new();
        registry.0.retain(|run| match run.upgrade() {
            Some(run) if lock(&run).phase.is_ok() => {
                live.push(run);
                true
            }
            _ => false,
        });
        live
    })
}

/// How many requests the registry holds (tests: ended ones leave it).
#[cfg(test)]
fn registered(ctx: &Context) -> usize {
    ctx.data_mut(|data| {
        data.get_temp_mut_or_default::<Registry>(registry_id())
            .0
            .len()
    })
}

/// One owner's reveal. The owner holds it in the value whose life is the target's (the open
/// form), asks at its act and marks the target while drawing.
#[derive(Debug, Default)]
pub struct Reveal {
    run: Option<Shared>,
}

impl Reveal {
    /// Ask at `act` for the form `target` of `visit` (the owner's object and what it shows now,
    /// such as the remote campaign and the campaign open) to come whole into view, in the column
    /// of the pressed control. An act of an older frame asks for nothing; an act outside a
    /// `ScrollColumn` that hosts reveals (`ScrollColumn::hosts_reveals`, R46) ends `Unowned`; a
    /// new ask replaces the request before it, also another owner's in the same column.
    pub fn ask(&mut self, act: Act, visit: impl Hash, target: impl Hash) {
        if act.ctx.cumulative_frame_nr() != act.frame {
            return;
        }
        if let Some(old) = self.run.take() {
            lock(&old).end(End::Replaced);
        }
        let mut run = Run {
            act_frame: act.frame,
            act_id: act.id,
            exempt: act.exempt,
            visit: Id::new(visit),
            target: Id::new(target),
            phase: Ok(()),
            deadline: act.frame + DEADLINE,
            left: PASSES,
            column: act.column.unwrap_or(Id::NULL),
            focus: focus(&act.ctx, act.id),
            opener_field: None,
            marked: None,
            drawn: false,
            frame: None,
            asked: None,
            edge: false,
            pending: None,
            waiting: false,
            view: None,
            elsewhere: None,
        };
        let Some(column) = act.column else {
            run.end(End::Unowned);
            self.run = Some(Arc::new(Mutex::new(run)));
            return;
        };
        // One request per column: a newer ask in it replaces an older one of another owner.
        for other in running(&act.ctx) {
            let mut other = lock(&other);
            if other.column == column {
                other.end(End::Replaced);
            }
        }
        let run = Arc::new(Mutex::new(run));
        act.ctx.data_mut(|data| {
            let registry = data.get_temp_mut_or_default::<Registry>(registry_id());
            registry.0.retain(|run| run.strong_count() > 0);
            registry.0.push(Arc::downgrade(&run));
        });
        act.ctx.request_repaint();
        self.run = Some(run);
    }

    /// At the press that starts work whose result comes later, such as a Connect that a worker
    /// answers (issue #698, M0279): the request waits for [`Reveal::result_came`], bound to the
    /// press's column, and moves nothing. While it waits it ends as an asked reveal ends (the
    /// DM's input to its column after the act's frame, by the column queue's rule; a focus that
    /// moves into its column; the owner's end or another visit; a frame without its column; a
    /// newer ask), but it has no deadline: the work sets the time, and the owner ends it when
    /// the work ends with no result to show. Input to another column never ends it.
    pub fn ask_on_result(&mut self, act: Act, visit: impl Hash, target: impl Hash) {
        self.ask(act, visit, target);
        if let Some(run) = &self.run {
            let mut run = lock(run);
            if run.phase.is_ok() {
                run.waiting = true;
                run.deadline = u64::MAX;
            }
        }
    }

    /// The awaited result came, and the target shows it from this frame: a waiting request runs
    /// from now as an asked one does (its [`PASSES`] and its deadline). It changes nothing in a
    /// request that ended or that does not wait.
    pub fn result_came(&mut self, ctx: &Context) {
        let Some(run) = &self.run else {
            return;
        };
        let mut run = lock(run);
        if run.phase.is_ok() && run.waiting {
            run.waiting = false;
            run.deadline = ctx.cumulative_frame_nr() + DEADLINE;
            run.left = PASSES;
            ctx.request_repaint();
        }
    }

    /// The owner ended (its object went away, its campaign changed): the request ends for good.
    pub fn end(&mut self) {
        if let Some(run) = &self.run {
            lock(run).end(End::OwnerGone);
        }
    }

    /// Whether it runs.
    #[cfg(any(test, feature = "test-support"))]
    pub fn running(&self) -> bool {
        self.run.as_ref().is_some_and(|run| lock(run).phase.is_ok())
    }

    /// Whether it waits for its result (`ask_on_result`).
    #[cfg(any(test, feature = "test-support"))]
    pub fn waits(&self) -> bool {
        self.run.as_ref().is_some_and(|run| {
            let run = lock(run);
            run.phase.is_ok() && run.waiting
        })
    }

    /// Why it ended; `None` while it runs or before any act.
    #[cfg(any(test, feature = "test-support"))]
    pub fn ended(&self) -> Option<End> {
        self.run.as_ref().and_then(|run| lock(run).phase.err())
    }

    /// The bound target of `visit` is drawn at `target_rect` (screen points) in `ui`, in this
    /// pass. Geometry only: its bound column resolves the request after its content.
    pub fn mark(&mut self, ui: &Ui, visit: impl Hash, target: impl Hash, target_rect: Rect) {
        let Some(run) = &self.run else {
            return;
        };
        let mut run = lock(run);
        if run.phase.is_err() || Id::new(target) != run.target {
            return;
        }
        if Id::new(visit) != run.visit {
            run.end(End::OwnerGone);
            return;
        }
        if let Some((content, region)) = owning_region(ui)
            && region.key == run.column
        {
            run.marked = Some((target_rect, content, ui.ctx().cumulative_pass_nr()));
        }
    }
}

/// Sol 2-011 (reviews of #838): after its act's frame, a focus that went to a control outside
/// the request's column, or that the DM's input to another column moved (in this pass or the
/// last: egui moves the focus at the end of the pass of the key), is that column's, so the
/// request takes it as its own and keeps running. A focus that moves into its own column with
/// no such input still ends it (`focus_allowed`).
fn take_foreign_focus(ui: &Ui, run: &mut Run, key: Id, view: Rect) {
    let frame = ui.ctx().cumulative_frame_nr();
    if frame == run.act_frame {
        return;
    }
    let focused = focus(ui.ctx(), run.act_id);
    if focused.is_none() || focused == run.focus {
        return;
    }
    let moved_elsewhere = run.elsewhere.is_some_and(|at| frame <= at + 1);
    if moved_elsewhere || !keys_in(ui, key, view) {
        run.focus = focused;
    }
}

/// The keyboard focus, other than the pressed control's own (which the press may give it, and
/// which ends when the opened form replaces the control).
fn focus(ctx: &Context, act: Id) -> Option<Id> {
    ctx.memory(|memory| memory.focused())
        .filter(|focused| *focused != act)
}

/// Whether the focus now is allowed for `run`: unchanged; lost (no field to keep still); or
/// taken by the opener's field, the first control inside `form` that took it. Records it.
fn focus_allowed(ctx: &Context, run: &mut Run, form: Option<Rect>) -> bool {
    let focused = focus(ctx, run.act_id);
    if focused == run.focus || focused.is_none() {
        run.focus = focused;
        return true;
    }
    let inside = form.is_some_and(|form| {
        focused
            // The field's rect as drawn, read without `read_response`, whose interaction
            // would surrender the field's focus on a press elsewhere (the act's own press).
            .and_then(|focused| {
                ctx.viewport(|viewport| {
                    viewport
                        .this_pass
                        .widgets
                        .get(focused)
                        .or_else(|| viewport.prev_pass.widgets.get(focused))
                        .map(|widget| widget.rect)
                })
            })
            .is_some_and(|field| form.contains_rect(field))
    });
    let opener = run.opener_field.is_none_or(|field| Some(field) == focused);
    if inside && opener {
        run.opener_field = focused;
        run.focus = focused;
        true
    } else {
        false
    }
}

/// Whether a reveal runs in the column that `ui` is drawn in, other than one that waits for a
/// result: the pane's focus reveal then makes no request into it.
pub(super) fn holds(ui: &Ui) -> bool {
    let Some((_, region)) = owning_region(ui).filter(|(_, region)| region.hosts) else {
        return false;
    };
    // A request that waits for a result owns no scroll yet.
    running(ui.ctx()).iter().any(|run| {
        let run = lock(run);
        run.column == region.key && !run.waiting
    })
}

/// A `ScrollColumn`: its persistent key, whether it may host a reveal (R46,
/// `ScrollColumn::hosts_reveals`), and the offset a region's request queued for it that this
/// pass applies (`take_queued`), which `settle_queued` keeps from egui's own scroll targets.
#[derive(Clone, Copy, Debug)]
pub(super) struct Column {
    pub key: Id,
    pub hosts: bool,
    /// The egui scroll area's own id: its state says whether it still moves from the DM's drag.
    pub area: Id,
    pub applied: Option<f32>,
}

/// The one request queued for a column (layout 11c, A1b; review of PR 770): a measured region's
/// request, or a typed jump ([`jump`]). A newer request for the column replaces it, whatever
/// asked it; one rule applies and cancels them all (`take_queued`).
#[derive(Clone, Copy, Debug)]
struct Queued {
    offset: f32,
    /// What asked it: a region's request (`None`) or a jump's reason.
    why: Option<JumpTo>,
    /// The column's view (screen points) when it was queued: only input over it is the DM's
    /// input for this request.
    view: Rect,
    /// The frame it was queued in, and the last frame it waited in.
    made: u64,
    frame: u64,
}

fn queued_id(column: Id) -> Id {
    column.with("design_region_queued")
}

/// Queue a move of `by` points for the column `ui` is drawn in (a region's resolved request,
/// layout 11c): the column applies it at the start of its next pass (`take_queued`), drawn in
/// another pass of this frame. A newer request replaces it. Outside a `ScrollColumn`, nothing.
pub(in crate::design) fn queue(ui: &Ui, by: f32) {
    if let Some((_, region)) = owning_region(ui) {
        enqueue(ui, &region, by, None);
    }
}

/// Queue the column's offset `by` points from where it is now, for its next pass: its one
/// request, replacing an older one.
fn enqueue(ui: &Ui, region: &Region, by: f32, why: Option<JumpTo>) {
    let frame = ui.ctx().cumulative_frame_nr();
    let queued = Queued {
        offset: (region.offset + by).max(0.0),
        why,
        view: region.view,
        made: frame,
        frame,
    };
    ui.ctx()
        .data_mut(|data| data.insert_temp(queued_id(region.key), queued));
    ui.ctx().request_discard("a request moves its column");
}

/// The key and the view of the `ScrollColumn` that `ui` is drawn in, the view in screen points (its area's viewport
/// inside its clip): a sideways clip egui keeps wider than the column is cut to the column.
pub(in crate::design) fn column_view(ui: &Ui) -> Option<(Id, Rect)> {
    owning_region(ui).map(|(_, region)| (region.key, region.view))
}

/// The offset queued for the column `key`, taken at the start of its pass. Only the DM's input
/// over that column (or to a control in it) counts (review of PR 757): a wheel turn over it or a
/// drag begun in it ends the request, also while the press is held; a press held still in it
/// makes the request wait, up to `QUEUED_FRAMES`; any other press in it, or a key, text or input
/// method event while the focus is in it, ends it; else it applies in its own frame or the next
/// one. A release ends a held press and is not input. A key counts only when it moves a view or
/// a caret (`moves_view`) and goes to this column (`keys_in`).
pub(super) fn take_queued(ui: &Ui, key: Id, area: Id) -> Option<f32> {
    let id = queued_id(key);
    let mut queued = ui.ctx().data(|data| data.get_temp::<Queued>(id))?;
    let _ = queued.why;
    let frame = ui.ctx().cumulative_frame_nr();
    let input = cancel::input(
        ui,
        cancel::Owner {
            key,
            view: queued.view,
            area: Some(area),
        },
    );
    // Review of PR 757 (P2): a key or another input during a held press ends the request as at
    // any other time; only a press held still makes it wait.
    if !input.moved && !input.other && input.held && frame <= queued.made + QUEUED_FRAMES {
        // A press held still: the request waits, up to its bound.
        queued.frame = frame;
        ui.ctx().data_mut(|data| data.insert_temp(id, queued));
        return None;
    }
    ui.ctx().data_mut(|data| data.remove::<Queued>(id));
    (frame <= queued.frame + 1 && !input.moved && !input.held && !input.other)
        .then_some(queued.offset)
}

/// The one place the DM's input ends a pending request (Sol 2-011; reviews of PRs 757, 770,
/// 780 and 838). Every request that moves a column by itself answers to it, by its owner
/// column: the column's queued request (a region's request, a [`jump`], [`bring_into_view`]),
/// a retry ([`dm_scrolls`]), a measured body's request in place, an asked reveal and one that
/// waits for a result. The raw reader is private here, so no other code can judge cancel input:
/// input to another column never ends a request, and every kind passes the same two-column case
/// table (`pane::laid` tests).
pub(in crate::design) mod cancel {
    use super::*;

    /// The column a request belongs to: its persistent key, its view (screen points), and its
    /// scroll area's id when its own fling after a drag counts.
    #[derive(Clone, Copy, Debug)]
    pub(in crate::design) struct Owner {
        pub key: Id,
        pub view: Rect,
        pub area: Option<Id>,
    }

    /// The DM's input to one column in this pass.
    #[derive(Clone, Copy, Debug, Default)]
    pub(in crate::design) struct ColumnInput {
        /// The DM scrolls it: a wheel with the pointer over its view or its bar, a drag begun
        /// there, or its own fling after a drag.
        pub moved: bool,
        /// A press that began in it is held.
        pub held: bool,
        /// A press is released over it in this pass.
        pub released: bool,
        /// Where the pointer is while it presses, holds or releases in it.
        pub at: Option<egui::Pos2>,
        /// Another input to it: a press in it, a key that moves a view or a caret
        /// (`moves_view`) or activates a control (Enter, Space) while the keys go to it
        /// (`keys_in`), or an accessibility action on a control in it.
        pub other: bool,
    }

    impl ColumnInput {
        /// Any input to it at all.
        pub(in crate::design) fn any(self) -> bool {
            self.moved || self.held || self.other
        }

        /// The DM's pointer presses, holds or releases in it: a focus reveal there waits
        /// (review of #875), so nothing moves under a click.
        pub(in crate::design) fn pressing(self) -> bool {
            self.held || self.released
        }
    }

    /// A pass's input as it goes to one column, read where the column starts, before a child
    /// can consume a key. Its fields are private: it is judged only by [`Read::input`].
    pub(in crate::design) struct Read {
        frame: u64,
        moved: bool,
        held: bool,
        released: bool,
        at: Option<egui::Pos2>,
        /// Each event of the pass, and whether it is input to this column.
        events: Vec<(egui::Event, bool)>,
        /// A press, key, text or accessibility action went to another column: a focus it moves
        /// is that column's (`Read::elsewhere`).
        elsewhere: bool,
    }

    /// The one reader: this pass's input to `owner`'s column.
    pub(in crate::design) fn read(ui: &Ui, owner: Owner) -> Read {
        let Owner { key, view, area } = owner;
        let fling = area
            .and_then(|area| egui::scroll_area::State::load(ui.ctx(), area))
            .is_some_and(|state| state.velocity() != Vec2::ZERO);
        let keys = keys_in(ui, key, view);
        let pointer_here = pointer_in(ui, key, view);
        // The content's view and the scroll bar's lane right of it: a press on the bar is the
        // DM's.
        let reach = Rect::from_min_max(
            view.min,
            egui::pos2(
                view.right() + super::super::layout::lane_width(),
                view.bottom(),
            ),
        );
        let frame = ui.ctx().cumulative_frame_nr();
        let (moved, held, released, at, events) = ui.input(|input| {
            let pointer = &input.pointer;
            let over = |pos: Option<egui::Pos2>| pos.is_some_and(|pos| reach.contains(pos));
            let wheel = input.raw_scroll_delta != Vec2::ZERO
                || input.smooth_scroll_delta != Vec2::ZERO
                || input
                    .events
                    .iter()
                    .any(|event| matches!(event, egui::Event::MouseWheel { .. }));
            let held = pointer.any_down() && over(pointer.press_origin());
            let dragged = held && pointer.is_decidedly_dragging();
            let moved = (wheel && pointer_here && over(pointer.hover_pos())) || dragged || fling;
            // egui forgets the press's origin in the pass of its release; the release's own
            // place says whose it is.
            let released = input.events.iter().any(|event| {
                matches!(event, egui::Event::PointerButton { pos, pressed: false, .. }
                    if reach.contains(*pos))
            });
            let at = pointer.latest_pos();
            (moved, held, released, at, input.events.clone())
        });
        let mut elsewhere = false;
        let events = events
            .into_iter()
            .map(|event| {
                let (input, here) = match &event {
                    egui::Event::PointerButton {
                        pos, pressed: true, ..
                    } => (true, reach.contains(*pos) && !held),
                    egui::Event::AccessKitActionRequest(request) => {
                        (true, action_in(ui, request.target, reach))
                    }
                    event if moves_view(event) || activates(event) => (true, keys),
                    _ => (false, false),
                };
                elsewhere |= input && !here;
                (event, here)
            })
            .collect();
        Read {
            frame,
            moved,
            held,
            released,
            at,
            events,
            elsewhere,
        }
    }

    /// The events of this pass that made an act of the control `node`: an accessibility click
    /// on it; Enter or Space while it has the focus (`focused`); else the primary release in
    /// its `rect`, with its press when it came in the same pass. Each is exempt from the one
    /// rule once, in the act's frame (`Read::input`). `None`: no event of this pass made it.
    pub(in crate::design) fn act_events(
        ui: &Ui,
        node: egui::accesskit::NodeId,
        focused: bool,
        rect: Rect,
    ) -> Option<Vec<egui::Event>> {
        ui.input(|input| {
            let events = &input.events;
            let access = events.iter().find(|event| {
                matches!(
                    event,
                    egui::Event::AccessKitActionRequest(request)
                        if request.target == node
                            && request.action == egui::accesskit::Action::Click
                )
            });
            if let Some(event) = access {
                return Some(vec![event.clone()]);
            }
            let key = events.iter().find(|event| {
                matches!(
                    event,
                    egui::Event::Key {
                        key: egui::Key::Enter | egui::Key::Space,
                        pressed: true,
                        ..
                    }
                )
            });
            if focused && let Some(event) = key {
                return Some(vec![event.clone()]);
            }
            let release = events.iter().position(|event| {
                matches!(
                    event,
                    egui::Event::PointerButton {
                        pos,
                        button: egui::PointerButton::Primary,
                        pressed: false,
                        ..
                    } if rect.contains(*pos)
                )
            })?;
            let mut exempt = vec![events[release].clone()];
            // Its press, when it came in the same pass.
            if let Some(press) = events[..release].iter().rev().find(|event| {
                matches!(
                    event,
                    egui::Event::PointerButton {
                        button: egui::PointerButton::Primary,
                        pressed: true,
                        ..
                    }
                )
            }) {
                exempt.push(press.clone());
            }
            Some(exempt)
        })
    }

    /// This pass's input to `owner`'s column (`read`, then `Read::input` with nothing exempt).
    pub(in crate::design) fn input(ui: &Ui, owner: Owner) -> ColumnInput {
        read(ui, owner).input(None)
    }

    impl Read {
        /// Whether this pass's input went to another column (a press outside this one, a key
        /// or text while the keys go elsewhere): a focus it moves is not this column's.
        pub(in crate::design) fn elsewhere(&self) -> bool {
            self.elsewhere
        }

        /// The input to the column. In the frame of an act (`act`: its frame and the events
        /// that made it), each of those events is exempt once.
        pub(in crate::design) fn input(&self, act: Option<(u64, &[egui::Event])>) -> ColumnInput {
            let mut exempt: Vec<&egui::Event> = match act {
                Some((frame, events)) if frame == self.frame => events.iter().collect(),
                _ => Vec::new(),
            };
            let mut other = false;
            for (event, here) in &self.events {
                if let Some(at) = exempt.iter().position(|made| *made == event) {
                    exempt.remove(at);
                    continue;
                }
                other |= *here;
            }
            ColumnInput {
                moved: self.moved,
                held: self.held,
                released: self.released,
                at: (self.held || self.released).then_some(self.at).flatten(),
                other,
            }
        }
    }

    /// Whether an accessibility action's target is a control drawn in `reach` in the last pass.
    fn action_in(ui: &Ui, target: egui::accesskit::NodeId, reach: Rect) -> bool {
        ui.ctx().viewport(|viewport| {
            viewport.prev_pass.widgets.layers().any(|(_, widgets)| {
                widgets.iter().any(|widget| {
                    egui::accesskit::NodeId::from(widget.id.value()) == target
                        && reach.intersects(widget.rect)
                })
            })
        })
    }

    /// A key that activates the focused control: a press of it.
    fn activates(event: &egui::Event) -> bool {
        matches!(
            event,
            egui::Event::Key {
                key: egui::Key::Enter | egui::Key::Space,
                pressed: true,
                ..
            }
        )
    }

    /// The DM's keyboard input that moves a view or a caret (reviews of PR 757): an arrow, Home or
    /// End with any modifiers (Ctrl+Home moves the caret too); Tab, Page Up or Page Down with Shift
    /// or none; typed text, a paste, an input method event. A shortcut (Ctrl+S, Ctrl+Page Down: the
    /// next tab) moves nothing.
    fn moves_view(event: &egui::Event) -> bool {
        use egui::Key;
        match event {
            egui::Event::Key {
                key,
                pressed: true,
                modifiers,
                ..
            } => {
                let shortcut =
                    modifiers.ctrl || modifiers.command || modifiers.mac_cmd || modifiers.alt;
                // A caret key moves the caret with any modifier (Ctrl+Home, Ctrl+Left: review of PR
                // 757); Tab and the page keys with Ctrl, Cmd or Alt are shortcuts (the next tab).
                matches!(
                    key,
                    Key::ArrowUp
                        | Key::ArrowDown
                        | Key::ArrowLeft
                        | Key::ArrowRight
                        | Key::Home
                        | Key::End
                ) || (!shortcut && matches!(key, Key::Tab | Key::PageUp | Key::PageDown))
            }
            egui::Event::Text(_) | egui::Event::Paste(_) | egui::Event::Ime(_) => true,
            _ => false,
        }
    }
}

/// Which column this pass's keys go to (review of PR 757): the innermost `ScrollColumn` whose
/// content drew the focused control, or with nothing focused, the innermost one under the
/// pointer. Each column records it after its content (`record_keys`); a nested column finishes
/// first, so it is the innermost. Kept for the pass it was recorded in and the one before it.
#[derive(Clone, Copy, Debug, Default)]
struct KeyOwner {
    pass: u64,
    focused: Option<Id>,
    focus: Option<Id>,
    pointer: Option<Id>,
    /// Where the pointer was when `pointer` was recorded.
    at: Option<egui::Pos2>,
}

fn key_owner_id() -> Id {
    Id::new("design_reveal_key_owner")
}

/// After a column's content (its widgets on `layer` are `widgets`): whether it is the innermost
/// column of the focused control, or under the pointer, in this pass.
fn record_keys(
    ui: &Ui,
    key: Id,
    layer: egui::LayerId,
    widgets: std::ops::Range<usize>,
    view: Rect,
) {
    let ctx = ui.ctx();
    let pass = ctx.cumulative_pass_nr();
    let focused = ctx.memory(|memory| memory.focused());
    let holds = focused.is_some_and(|id| {
        ctx.viewport(|viewport| {
            viewport
                .this_pass
                .widgets
                .order(id)
                .is_some_and(|(at_layer, at)| at_layer == layer && widgets.contains(&at))
        })
    });
    let reach = Rect::from_min_max(
        view.min,
        egui::pos2(view.right() + super::layout::lane_width(), view.bottom()),
    );
    let under = ui.input(|input| {
        input
            .pointer
            .hover_pos()
            .is_some_and(|pos| reach.contains(pos))
    });
    let at = ui.input(|input| input.pointer.hover_pos());
    ctx.data_mut(|data| {
        let owners = data.get_temp_mut_or_default::<[KeyOwner; 2]>(key_owner_id());
        if owners[1].pass != pass {
            owners[0] = owners[1];
            owners[1] = KeyOwner {
                pass,
                focused,
                ..KeyOwner::default()
            };
        }
        let now = &mut owners[1];
        // Review of PR 787 (P2): the focus can move in a later column of the pass; the owner is
        // the innermost column of the focused control as it is now.
        if now.focused != focused {
            now.focused = focused;
            now.focus = None;
        }
        if holds && now.focus.is_none() {
            now.focus = Some(key);
        }
        if now.at != at {
            now.at = at;
            now.pointer = None;
        }
        if under && now.pointer.is_none() {
            now.pointer = Some(key);
        }
    });
}

/// Whether this pass's keys go to the column `key` (`KeyOwner`). A focus not yet recorded (it
/// moved in this frame) goes to no column. A key never belongs to two columns.
pub(in crate::design) fn keys_in(ui: &Ui, key: Id, view: Rect) -> bool {
    let focused = ui.ctx().memory(|memory| memory.focused());
    match focused {
        Some(_) => {
            newest_owner(ui, |owner| {
                (owner.focused == focused).then_some(owner.focus?)
            }) == Some(key)
        }
        None => pointer_in(ui, key, view),
    }
}

/// Whether the column `key` is the innermost one under the pointer (`KeyOwner`): a wheel there
/// scrolls it, not the column around it.
fn pointer_in(ui: &Ui, key: Id, view: Rect) -> bool {
    let hover = ui.input(|input| input.pointer.hover_pos());
    let owners = ui
        .ctx()
        .data(|data| data.get_temp::<[KeyOwner; 2]>(key_owner_id()))
        .unwrap_or_default();
    // Review of PR 787 (P2): a record made with the pointer elsewhere says nothing of where it is
    // now (the DM moved from an inner column to its outer one in this frame): then the column's
    // own view decides.
    match owners
        .iter()
        .rev()
        .filter(|owner| owner.pass != 0 && owner.at == hover)
        .find_map(|owner| owner.pointer)
    {
        Some(owner) => owner == key,
        None => hover.is_some_and(|at| view.contains(at)),
    }
}

/// The newest recorded owner that `pick` finds.
fn newest_owner(ui: &Ui, pick: impl Fn(&KeyOwner) -> Option<Id>) -> Option<Id> {
    ui.ctx()
        .data(|data| data.get_temp::<[KeyOwner; 2]>(key_owner_id()))
        .unwrap_or_default()
        .iter()
        .rev()
        .filter(|owner| owner.pass != 0)
        .find_map(pick)
}

/// The frames a queued request may wait for a held press to end.
const QUEUED_FRAMES: u64 = 8;

/// After a column's content, in a pass that queued a region's request or applied one: a raw
/// `scroll_to_rect` of that content (egui keeps it as a target that lands at the start of the
/// next pass, after the queued offset) is replaced by a target already in view, so no egui target
/// writes the offset after the request. The DM's own wheel is not a target.
fn settle_queued(ui: &mut Ui, column: Column, view: Rect) {
    let queued = ui
        .ctx()
        .data(|data| data.get_temp::<Queued>(queued_id(column.key)))
        .is_some_and(|queued| queued.frame == ui.ctx().cumulative_frame_nr());
    if (queued || column.applied.is_some()) && view.height() > 2.0 {
        ui.scroll_to_rect(view.shrink(1.0), None);
    }
}

/// A `ScrollColumn`'s facts in one pass, recorded where its content starts.
#[derive(Clone, Copy, Debug)]
struct Region {
    pass: u64,
    /// The column's persistent key, which [`take_offset`] reads.
    key: Id,
    /// The column may host a reveal: only then does an act bind it.
    hosts: bool,
    /// The visible view, in screen points: the area's native viewport inside its clip.
    view: Rect,
    /// The content's top, in screen points.
    origin: f32,
    /// The vertical offset this pass draws with.
    offset: f32,
}

fn region_id(content: Id) -> Id {
    content.with("design_reveal_region")
}

/// The innermost scroll area of `ui`, if a `ScrollColumn` drew it in this pass: its content
/// `Ui`'s unique id (the id egui's `UiStack` names it by) and its facts.
fn owning_region(ui: &Ui) -> Option<(Id, Region)> {
    let pass = ui.ctx().cumulative_pass_nr();
    let content = ui
        .stack()
        .iter()
        .find(|frame| frame.kind() == Some(egui::UiKind::ScrollArea))?
        .id;
    ui.ctx()
        .data(|data| data.get_temp::<Region>(region_id(content)))
        .filter(|region| region.pass == pass)
        .map(|region| (content, region))
}

/// Why a column moves by itself without a press in it (ruling M18): a closed list. Each is an
/// offset the column writes at its next pass, as it writes a reveal's; none is an `Act`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JumpTo {
    /// A reader opened at a linked section: the section's heading at the view's top; a
    /// recording's text opened at a timeline marker: the marker's line at the view's top.
    LinkedHeading,
    /// An open menu's control after its pane changed size under it (a list row's `⋯`, M0259):
    /// the least scroll that shows it whole ([`bring_into_view`]), so the menu does not hang
    /// from a control the DM cannot see.
    OpenMenuControl,
    /// A list's row brought into view because the page opened at it, a link revealed it, or
    /// it moved there. As a [`jump`], the row before it at the view's top (the Timeline's
    /// events, M0556); as [`bring_into_view`], the least scroll that shows the row whole with
    /// what it opened (History's opened run, M0194).
    ListedRow,
    /// A journey's check whose result an act just wrote (a roll, an entered number; M0546): the
    /// least scroll that shows its line whole ([`bring_into_view`]), so the DM sees what the act
    /// did.
    CheckResult,
    /// The card that took what the DM added from another column (an encounter's group that a
    /// creature from the Add pane went to, M0226): the least scroll that shows the card whole
    /// ([`bring_into_view`]), fired once by the add (ruling M27).
    AddedTarget,
}

/// The `ScrollColumn` that holds `ui` moves, at its next pass, so that the screen y `top` of
/// this pass is at its view's top (layout PR 10c, ruling M18). It is the column's one queued
/// request (`enqueue`): a newer request for the column, a jump or a region's, replaces it, and
/// the DM's own input over that column (or to a control in it) drops it (`take_queued`). Outside
/// a `ScrollColumn` it does nothing.
pub fn jump(ui: &Ui, top: f32, why: JumpTo) {
    let Some((_, region)) = owning_region(ui) else {
        return;
    };
    enqueue(ui, &region, top - region.view.top(), Some(why));
}

/// The least scroll, with no press, that shows the whole `rect` (a rect of this pass) in the
/// view of the `ScrollColumn` that holds `ui` (M-P0.25): none when it is whole in view; else
/// its nearer edge at the view's edge, and its top at the view's top when it is taller than
/// the view. It is the column's one queued request, as [`jump`]'s is: a newer request for the
/// column replaces it, and the DM's own input over that column (or to a control in it) drops it
/// (`take_queued`). Outside a `ScrollColumn` it does nothing. Whether it asked for a scroll.
pub fn bring_into_view(ui: &Ui, rect: Rect, why: JumpTo) -> bool {
    let Some((_, region)) = owning_region(ui) else {
        return false;
    };
    let view = region.view;
    let by = if rect.top() < view.top() || rect.height() > view.height() {
        rect.top() - view.top()
    } else if rect.bottom() > view.bottom() {
        rect.bottom() - view.bottom()
    } else {
        0.0
    };
    if by.abs() < 0.5 {
        return false;
    }
    enqueue(ui, &region, by, Some(why));
    true
}

/// Whether the DM's own input scrolls, holds or keys the `ScrollColumn` that holds `ui` in this
/// pass (reviews of PRs 770 and 780): its column's one rule ([`cancel`]), the rule its queued
/// request answers to. A retry that would move the column by itself (a row menu's bring-back
/// after a resize, M0259) ends on it, so the column stays where the DM leaves it, and input to
/// another column never ends it. Outside a `ScrollColumn` it is false.
pub fn dm_scrolls(ui: &Ui) -> bool {
    let Some((_, region)) = owning_region(ui) else {
        return false;
    };
    cancel::input(
        ui,
        cancel::Owner {
            key: region.key,
            view: region.view,
            area: None,
        },
    )
    .any()
}

/// The offset the reveal of the column `key` queued for this pass, taken once, after this pass's
/// input is checked (a jump's and a region's are the column's queued request: `take_queued`). `ScrollColumn` calls it before
/// its area begins.
pub(super) fn take_offset(ui: &Ui, column: Column) -> Option<f32> {
    // A column that hosts no reveal holds no request: no registry read.
    if !column.hosts {
        return None;
    }
    let key = column.key;
    let frame = ui.ctx().cumulative_frame_nr();
    let run = running(ui.ctx())
        .into_iter()
        .find(|run| lock(run).column == key)?;
    let mut run = lock(&run);
    let (to, written) = run.pending.take()?;
    if !(written == frame || written + 1 == frame) {
        return None;
    }
    // The column's input to its own request ([`cancel`]), in the view it had when it asked.
    let owner = cancel::Owner {
        key,
        view: run.view.unwrap_or(Rect::EVERYTHING),
        area: Some(column.area),
    };
    let read = cancel::read(ui, owner);
    if read.input(Some((run.act_frame, &run.exempt))).any() {
        run.end(End::ByTheDm);
        return None;
    }
    if read.elsewhere() {
        run.elsewhere = Some(frame);
    }
    take_foreign_focus(ui, &mut run, owner.key, owner.view);
    // A control that took the focus since the last pass, other than the opener's field.
    let form = run.marked.map(|(form, ..)| form);
    if !focus_allowed(ui.ctx(), &mut run, form) {
        run.end(End::Focus);
        return None;
    }
    Some(to)
}

/// Run `add` as the content of the scroll column `key`, whose content `Ui` is `ui` and whose
/// viewport (egui's, in content points) is `viewport`; then resolve the request of this column.
/// `ScrollColumn` calls it; only it may.
pub(super) fn region<R>(
    ui: &mut Ui,
    column: Column,
    viewport: Rect,
    add: impl FnOnce(&mut Ui) -> R,
) -> R {
    let key = column.key;
    let origin = ui.max_rect().min.to_vec2();
    let view = viewport.translate(origin).intersect(ui.clip_rect());
    let region = Region {
        pass: ui.ctx().cumulative_pass_nr(),
        key,
        hosts: column.hosts,
        view,
        origin: origin.y,
        offset: viewport.top(),
    };
    let content = ui.unique_id();
    ui.ctx()
        .data_mut(|data| data.insert_temp(region_id(content), region));
    let layer = ui.layer_id();
    let count = |ui: &Ui| {
        ui.ctx()
            .viewport(|viewport| viewport.this_pass.widgets.get_layer(layer).count())
    };
    let from = count(ui);
    if !column.hosts {
        // No request can bind here: no input copy and no registry read.
        let inner = add(ui);
        record_keys(ui, key, layer, from..count(ui), view);
        settle_queued(ui, column, view);
        return inner;
    }
    // The DM's input to this column ([`cancel`]), read before any child can consume a key.
    let read = cancel::read(
        ui,
        cancel::Owner {
            key,
            view,
            area: Some(column.area),
        },
    );
    let inner = add(ui);
    record_keys(ui, key, layer, from..count(ui), view);
    settle_queued(ui, column, view);
    // The request that runs in this column now (one asked by this content's act replaced any
    // older one), judged by the input of this pass.
    let run = running(ui.ctx())
        .into_iter()
        .find(|run| lock(run).column == key);
    if let Some(run) = run {
        let mut run = lock(&run);
        let input = read.input(Some((run.act_frame, &run.exempt))).any();
        if read.elsewhere() {
            run.elsewhere = Some(ui.ctx().cumulative_frame_nr());
        }
        resolve(ui, &mut run, &region, content, input);
    }
    inner
}

/// One step of `run` after the content of its column `region` (content `Ui` `content`).
fn resolve(ui: &Ui, run: &mut Run, region: &Region, content: Id, input: bool) {
    let (frame, pass) = (
        ui.ctx().cumulative_frame_nr(),
        ui.ctx().cumulative_pass_nr(),
    );
    if run.phase.is_err() {
        return;
    }
    run.view = Some(region.view);
    if input {
        run.end(End::ByTheDm);
        return;
    }
    if frame > run.deadline {
        // Also a request whose target never showed ends here.
        run.end(End::Bound);
        return;
    }
    if run.frame.is_some_and(|last| frame > last + 1) {
        // A frame drew no pass of its column: the page left.
        run.end(End::OwnerGone);
        return;
    }
    run.frame = Some(frame);
    let target = run
        .marked
        .filter(|&(_, at, marked)| at == content && marked == pass)
        .map(|(rect, ..)| rect);
    take_foreign_focus(ui, run, region.key, region.view);
    // A control that took the focus since the ask, other than the opener's field.
    if !focus_allowed(ui.ctx(), run, target) {
        run.end(End::Focus);
        return;
    }
    if run.waiting {
        // Its result has not come: nothing moves (`Reveal::ask_on_result`).
        return;
    }
    let Some(target) = target else {
        if run.drawn {
            // A pass of its column that does not draw the target it drew before.
            run.end(End::OwnerGone);
        }
        return;
    };
    run.drawn = true;
    let view = region.view;
    if target.top() >= view.top() - 0.5 && target.bottom() <= view.bottom() + 0.5 {
        run.end(End::Whole);
        return;
    }
    let content_top = target.top() - region.origin;
    if let Some(asked) = run.asked
        && asked.frame != frame
    {
        if run.edge {
            run.end(End::TooTall);
            return;
        }
        if (asked.offset - region.offset).abs() < 0.5
            && (asked.content_top - content_top).abs() < 0.5
        {
            run.end(End::NoProgress);
            return;
        }
    }
    if run.asked.is_none_or(|asked| asked.frame != frame) {
        if run.left == 0 {
            run.end(End::Bound);
            return;
        }
        run.left -= 1;
    }
    // Taller than the view: its top shows.
    run.edge |= target.height() > view.height();
    let delta = if run.edge || target.top() < view.top() {
        target.top() - view.top()
    } else {
        target.bottom() - view.bottom()
    };
    run.asked = Some(Asked {
        frame,
        offset: region.offset,
        content_top,
    });
    run.pending = Some(((region.offset + delta).max(0.0), frame));
    ui.ctx().request_repaint();
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui_kittest::kittest::Queryable;

    const WIDTH: f32 = 400.0;
    const HEIGHT: f32 = 300.0;

    /// A test page: a column with `Open` and `Other` at its top, then `above` px of room, the
    /// form (`Target top`, `Field`, a block of `tall` px, `Target end`), then 900 px. Pressing
    /// `Open` asks; `ask` asks with a test act in the next frame.
    struct Page {
        reveal: Reveal,
        ask: bool,
        /// A saved `Response` of `Open`, for a later pass.
        saved: Option<Response>,
        replay: bool,
        target: &'static str,
        above: f32,
        tall: f32,
        drawn: bool,
        nested: bool,
        raw: bool,
        /// A second reveal that asks with `Open`'s press too.
        second: Reveal,
        /// The control that takes the focus in the next frame: `Field` in the form, or `Other`.
        focus_inside: Option<bool>,
        /// Asks egui's own scroll to the page's end.
        jump: bool,
        /// `Open` is disabled; `Open` keeps the keyboard focus.
        disabled: bool,
        focus_open: bool,
        /// Another owner's request in the column, which waits for a target never drawn.
        older: Reveal,
        waiting: bool,
        /// `Open`'s id, as last drawn.
        open_id: Option<Id>,
        /// Room above `Open`, which moves it down.
        open_down: f32,
        /// Room between the form and `Page end`.
        after: f32,
        /// The outer column does not host reveals.
        plain: bool,
        /// Ask the least scroll that shows the target (`bring_into_view`) in this pass.
        least: bool,
        /// What `dm_scrolls` said in the target's column in the last pass.
        scrolled: Option<bool>,
        /// A typed jump to the target's top (`jump`), asked once.
        heading: bool,
        /// `ask` waits for a result (`ask_on_result`).
        awaited: bool,
        /// The awaited result comes in the next pass (`result_came`).
        came: bool,
        /// A second column beside the page's, in the right half of the window.
        beside: bool,
        /// `Beside`, in that column, takes the focus in the next frame.
        focus_beside: bool,
        /// The outer column follows the keyboard focus (`ScrollColumn::follows_focus`).
        follows: bool,
    }

    impl Page {
        fn saved_id(&self) -> Id {
            self.open_id.expect("Open is drawn")
        }
    }

    impl Page {
        fn new() -> Self {
            Self {
                reveal: Reveal::default(),
                ask: true,
                saved: None,
                replay: false,
                target: "t",
                above: 900.0,
                tall: 40.0,
                drawn: true,
                nested: false,
                raw: false,
                second: Reveal::default(),
                focus_inside: None,
                jump: false,
                disabled: false,
                focus_open: false,
                older: Reveal::default(),
                waiting: false,
                open_id: None,
                open_down: 0.0,
                after: 0.0,
                plain: false,
                least: false,
                scrolled: None,
                heading: false,
                awaited: false,
                came: false,
                beside: false,
                focus_beside: false,
                follows: false,
            }
        }
    }

    fn room(ui: &mut Ui, height: f32) {
        ui.allocate_exact_size(egui::vec2(1.0, height), egui::Sense::hover());
    }

    fn target(ui: &mut Ui, page: &mut Page) {
        if page.drawn {
            let tall = page.tall;
            let mut field = None;
            let heading = std::mem::take(&mut page.heading);
            let rect = ui
                .vertical(|ui| {
                    if heading {
                        jump(ui, ui.cursor().top(), JumpTo::LinkedHeading);
                    }
                    ui.label("Target top");
                    field = Some(ui.button("Field"));
                    room(ui, tall);
                    ui.label("Target end");
                })
                .response
                .rect;
            if std::mem::take(&mut page.least) {
                bring_into_view(ui, rect, JumpTo::OpenMenuControl);
            }
            page.scrolled = Some(dm_scrolls(ui));
            if page.focus_inside == Some(true)
                && let Some(field) = field
            {
                page.focus_inside = None;
                field.request_focus();
            }
            page.reveal.mark(ui, 0, page.target, rect);
        }
    }

    fn content(ui: &mut Ui, page: &mut Page) {
        ui.label("Outer top");
        room(ui, page.open_down);
        let open = ui.add_enabled(!page.disabled, egui::Button::new("Open"));
        page.open_id = Some(open.id);
        let other = ui.button("Other");
        if page.focus_open {
            open.request_focus();
        }
        if std::mem::take(&mut page.waiting) {
            page.older.ask(Act::test(ui, 3), 0, "older");
        }
        if let Some(act) = Act::clicked(ui, &open) {
            page.reveal.ask(act, 0, "t");
            if let Some(again) = Act::clicked(ui, &open) {
                page.second.ask(again, 0, "t");
            }
            page.saved = Some(open.clone());
        }
        if std::mem::take(&mut page.replay)
            && let Some(saved) = &page.saved
            && let Some(act) = Act::clicked(ui, saved)
        {
            page.reveal.ask(act, 0, "t");
        }
        if std::mem::take(&mut page.ask) {
            if page.awaited {
                page.reveal.ask_on_result(Act::test(ui, 1), 0, "t");
            } else {
                page.reveal.ask(Act::test(ui, 1), 0, "t");
            }
        }
        if std::mem::take(&mut page.came) {
            page.reveal.result_came(ui.ctx());
        }
        if page.focus_inside == Some(false) {
            page.focus_inside = None;
            other.request_focus();
        }
        if page.nested {
            super::super::ScrollColumn::new("inner")
                .max_height(120.0)
                .min_scrolled_height(120.0)
                .show(ui, |ui| {
                    room(ui, page.above);
                    target(ui, page);
                    room(ui, 900.0);
                });
        } else {
            room(ui, page.above);
            target(ui, page);
        }
        room(ui, page.after);
        let end = ui.label("Page end").rect;
        if std::mem::take(&mut page.jump) {
            ui.scroll_to_rect(end, None);
        }
        room(ui, 900.0);
    }

    fn harness(mut page: Page) -> egui_kittest::Harness<'static, Page> {
        use crate::design::test_harness::Themed as _;
        // The page asks after the theme (its fonts) is installed.
        let ask = std::mem::take(&mut page.ask);
        let mut h = egui_kittest::Harness::builder()
            .with_size(egui::vec2(WIDTH, HEIGHT))
            .themed_ui_state(
                |ui, page: &mut Page| {
                    if page.raw {
                        // A scroll area that is not a `ScrollColumn`.
                        super::super::sideways_scroll(
                            ui,
                            "raw",
                            super::super::Sideways::default(),
                            |ui| content(ui, page),
                        );
                    } else if page.beside {
                        // The page's column in the left half, another column in the right half.
                        ui.horizontal_top(|ui| {
                            let half = egui::vec2(WIDTH / 2.0, HEIGHT);
                            let down = egui::Layout::top_down(egui::Align::Min);
                            ui.allocate_ui_with_layout(half, down, |ui| {
                                super::super::ScrollColumn::new("outer")
                                    .hosts_reveals()
                                    .show(ui, |ui| content(ui, page));
                            });
                            ui.allocate_ui_with_layout(half, down, |ui| {
                                super::super::ScrollColumn::new("beside")
                                    .hosts_reveals()
                                    .show(ui, |ui| {
                                        let beside = ui.button("Beside");
                                        if std::mem::take(&mut page.focus_beside) {
                                            beside.request_focus();
                                        }
                                        room(ui, 2000.0);
                                    });
                            });
                        });
                    } else {
                        let mut column = super::super::ScrollColumn::new("outer");
                        if !page.plain {
                            column = column.hosts_reveals();
                        }
                        if page.follows {
                            column = column.follows_focus();
                        }
                        column.show(ui, |ui| content(ui, page));
                    }
                },
                page,
            );
        h.step();
        h.state_mut().ask = ask;
        h
    }

    fn top(h: &egui_kittest::Harness<'_, Page>, name: &str) -> f32 {
        h.get_by_label(name).rect().top()
    }

    fn in_view(h: &egui_kittest::Harness<'_, Page>) -> bool {
        ["Target top", "Target end"].iter().all(|name| {
            let rect = h.get_by_label(name).rect();
            rect.top() >= -0.5 && rect.bottom() <= HEIGHT + 0.5
        })
    }

    fn wheel(h: &mut egui_kittest::Harness<'_, Page>, by: f32) {
        h.event(egui::Event::PointerMoved(egui::pos2(WIDTH / 2.0, 100.0)));
        h.event(egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0.0, by),
            modifiers: egui::Modifiers::NONE,
        });
    }

    fn quiet() -> Page {
        let mut page = Page::new();
        page.ask = false;
        page
    }

    /// It opens whole, ends, and stays ended: a resize does not bring it back. An ended request
    /// leaves the registry.
    #[test]
    fn a_reveal_shows_its_target_once() {
        let mut h = harness(Page::new());
        h.run_steps(8);
        assert!(in_view(&h), "the target is not whole");
        assert_eq!(h.state().reveal.ended(), Some(End::Whole));
        assert_eq!(registered(&h.ctx), 0, "an ended request stays registered");
        let before = top(&h, "Target top");
        h.set_size(egui::vec2(WIDTH, 120.0));
        h.run_steps(8);
        assert!(
            (top(&h, "Target top") - before).abs() < 0.5
                && h.state().reveal.ended() == Some(End::Whole),
            "a resize moved an ended reveal's target: {before} then {}",
            top(&h, "Target top")
        );
    }

    /// The events of one raw input pass, then that pass.
    fn pass(h: &mut egui_kittest::Harness<'_, Page>, events: Vec<egui::Event>) {
        h.input_mut().events.extend(events);
        h.step();
    }

    fn button(pos: egui::Pos2, pressed: bool) -> egui::Event {
        egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        }
    }

    fn key(key: egui::Key) -> egui::Event {
        egui::Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }
    }

    fn access_click(id: Id) -> egui::Event {
        egui::Event::AccessKitActionRequest(egui::accesskit::ActionRequest {
            action: egui::accesskit::Action::Click,
            target: egui::accesskit::NodeId::from(id.value()),
            data: None,
        })
    }

    /// A real press asks once: a second `Act::clicked` in the pass gives nothing. A saved
    /// `Response` of a key activation (a copy keeps its click flag) gives nothing on a later
    /// pass: with a release inside its old rect after the control moved away, or with an
    /// accessibility click while the control is disabled.
    #[test]
    fn one_press_gives_one_act() {
        let mut page = quiet();
        page.focus_open = true;
        let mut h = harness(page);
        pass(&mut h, vec![key(egui::Key::Enter)]);
        h.run_steps(8);
        assert!(in_view(&h), "the key did not reveal the form");
        assert!(
            h.state().second.ended().is_none() && !h.state().second.running(),
            "one press gave two acts"
        );
        h.state_mut().focus_open = false;
        wheel(&mut h, 2000.0);
        h.run_steps(6);
        let open = h.get_by_label("Open").rect();
        let id = h.state().saved.as_ref().expect("a saved Response").id;
        for disabled in [false, true] {
            h.state_mut().disabled = disabled;
            h.state_mut().open_down = if disabled { 0.0 } else { 120.0 };
            h.step();
            let away = top(&h, "Target top");
            h.state_mut().replay = true;
            let event = if disabled {
                access_click(id)
            } else {
                button(open.center(), false)
            };
            pass(
                &mut h,
                vec![egui::Event::PointerMoved(open.center()), event],
            );
            h.run_steps(8);
            assert!(
                h.state().reveal.ended() == Some(End::Whole)
                    && away > HEIGHT
                    && (top(&h, "Target top") - away).abs() < 0.5,
                "a saved Response made an act (disabled: {disabled}): {:?}, {away} then {}",
                h.state().reveal.ended(),
                top(&h, "Target top")
            );
        }
    }

    /// In the act's pass only the input that made it is exempt: a second accessibility click,
    /// a second press on the control, or `Space` with the `Enter` that pressed, cancel. Alone,
    /// each act reveals.
    #[test]
    fn only_the_acts_own_input_is_exempt() {
        for extra in [false, true] {
            let expected = if extra { End::ByTheDm } else { End::Whole };
            // Accessibility clicks.
            let mut h = harness(quiet());
            let id = h.state().saved_id();
            let mut events = vec![access_click(id)];
            if extra {
                events.push(access_click(id));
            }
            pass(&mut h, events);
            h.run_steps(8);
            assert_eq!(
                h.state().reveal.ended(),
                Some(expected),
                "access, extra {extra}"
            );
            // A press and release on `Open`, then a press again.
            let mut h = harness(quiet());
            let at = h.get_by_label("Open").rect().center();
            pass(&mut h, vec![egui::Event::PointerMoved(at)]);
            let mut events = vec![button(at, true), button(at, false)];
            if extra {
                events.push(button(at, true));
            }
            pass(&mut h, events);
            h.run_steps(8);
            assert_eq!(
                h.state().reveal.ended(),
                Some(expected),
                "pointer, extra {extra}"
            );
            // `Enter` with `Open` focused, and `Space`.
            let mut page = quiet();
            page.focus_open = true;
            let mut h = harness(page);
            let mut events = vec![key(egui::Key::Enter)];
            if extra {
                events.push(key(egui::Key::Space));
            }
            pass(&mut h, events);
            h.run_steps(8);
            assert_eq!(
                h.state().reveal.ended(),
                Some(expected),
                "keys, extra {extra}"
            );
        }
    }

    /// N5: an older owner's request waits in the column; a real press replaces it, with typed
    /// text in the same pass: the newer request ends for the text, and nothing moves.
    #[test]
    fn a_replacing_act_sees_its_own_pass_input() {
        let mut h = harness(quiet());
        h.state_mut().waiting = true;
        h.step();
        assert!(h.state().older.running(), "the older request does not wait");
        let before = top(&h, "Target top");
        let at = h.get_by_label("Open").rect().center();
        pass(&mut h, vec![egui::Event::PointerMoved(at)]);
        pass(
            &mut h,
            vec![
                button(at, true),
                button(at, false),
                egui::Event::Text("s".into()),
            ],
        );
        h.run_steps(8);
        assert_eq!(h.state().older.ended(), Some(End::Replaced));
        assert_eq!(h.state().reveal.ended(), Some(End::ByTheDm));
        assert!(
            (top(&h, "Target top") - before).abs() < 0.5,
            "the column moved after the DM typed in the act's pass"
        );
    }

    /// E1: A queues an offset and ends (its owner drops it, or ends it); the column's next pass
    /// draws an unrelated page and does not move.
    #[test]
    fn an_ended_request_never_moves_its_column() {
        for drop in [true, false] {
            let mut h = harness(quiet());
            h.state_mut().ask = true;
            h.step();
            assert!(
                h.state().reveal.running() && !in_view(&h),
                "no queued request"
            );
            if drop {
                h.state_mut().reveal = Reveal::default();
            } else {
                h.state_mut().reveal.end();
            }
            h.state_mut().drawn = false;
            let outer = top(&h, "Outer top");
            h.run_steps(6);
            assert!(
                (top(&h, "Outer top") - outer).abs() < 0.5,
                "an ended request moved the column (dropped: {drop})"
            );
        }
    }

    /// A pass of its column that does not draw a target it drew ends it; the deadline ends a
    /// request whose target never shows; a wheel turn before the first mark ends it too.
    #[test]
    fn a_missing_target_ends_the_reveal() {
        let mut h = harness(quiet());
        h.state_mut().ask = true;
        h.step();
        h.state_mut().drawn = false;
        h.step();
        h.state_mut().drawn = true;
        h.state_mut().above += 1000.0;
        h.step();
        let back = top(&h, "Target top");
        h.run_steps(8);
        assert_eq!(h.state().reveal.ended(), Some(End::OwnerGone));
        assert!(
            back > HEIGHT && (top(&h, "Target top") - back).abs() < 0.5,
            "a target that left a pass was scrolled to: {back} then {}",
            top(&h, "Target top")
        );
        for wheel_first in [false, true] {
            let mut page = quiet();
            page.drawn = false;
            let mut h = harness(page);
            h.state_mut().ask = true;
            h.step();
            if wheel_first {
                wheel(&mut h, 0.0);
                h.step();
            } else {
                h.run_steps(12);
            }
            let before = top(&h, "Outer top");
            h.state_mut().drawn = true;
            h.run_steps(8);
            assert_eq!(
                h.state().reveal.ended(),
                Some(if wheel_first {
                    End::ByTheDm
                } else {
                    End::Bound
                })
            );
            assert!(
                (top(&h, "Outer top") - before).abs() < 0.5,
                "a target that showed late was scrolled to (wheel first: {wheel_first})"
            );
        }
        // Issue #698 (M0279, `m-p0-awaited`): a press whose result comes later. The request waits
        // past the deadline and moves nothing, also once its target is drawn; when the result
        // comes, the target comes whole into view. The DM's wheel during the wait ends it, and
        // the result then moves nothing.
        for wheel_first in [false, true] {
            let mut page = quiet();
            page.drawn = false;
            page.awaited = true;
            let mut h = harness(page);
            h.state_mut().ask = true;
            h.step();
            h.run_steps(12);
            if wheel_first {
                wheel(&mut h, 0.0);
                h.step();
            }
            h.state_mut().drawn = true;
            h.run_steps(4);
            let before = top(&h, "Outer top");
            assert_eq!(
                h.state().reveal.running(),
                !wheel_first,
                "m-p0-awaited: it waits past the deadline (wheel first: {wheel_first})"
            );
            assert!(!in_view(&h), "the target starts out of view");
            h.state_mut().came = true;
            h.run_steps(8);
            if wheel_first {
                assert_eq!(h.state().reveal.ended(), Some(End::ByTheDm));
                assert!(
                    (top(&h, "Outer top") - before).abs() < 0.5,
                    "m-p0-awaited: a result after the DM's wheel moves nothing"
                );
            } else {
                assert_eq!(h.state().reveal.ended(), Some(End::Whole));
                assert!(
                    in_view(&h),
                    "m-p0-awaited: the result brings its target whole into view"
                );
            }
        }
        // Review of #838 (P2, `m-p0-awaited-beside`): the DM's wheel over another column while
        // the request waits is input to that column, not to the request's: it keeps waiting,
        // and its result still brings the target into view. A wheel over its own column ends it.
        // So does a control of the other column that takes the focus: it keeps waiting.
        {
            let mut page = quiet();
            page.drawn = false;
            page.awaited = true;
            page.beside = true;
            let mut h = harness(page);
            h.state_mut().ask = true;
            h.step();
            h.run_steps(4);
            h.state_mut().focus_beside = true;
            h.run_steps(4);
            assert!(
                h.state().reveal.waits(),
                "m-p0-awaited-beside: a focus in another column, {:?}",
                h.state().reveal.ended()
            );
        }
        for own in [false, true] {
            let mut page = quiet();
            page.drawn = false;
            page.awaited = true;
            page.beside = true;
            let mut h = harness(page);
            h.state_mut().ask = true;
            h.step();
            h.run_steps(4);
            let x = if own { WIDTH / 4.0 } else { 3.0 * WIDTH / 4.0 };
            h.event(egui::Event::PointerMoved(egui::pos2(x, 100.0)));
            h.event(egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                delta: egui::vec2(0.0, -40.0),
                modifiers: egui::Modifiers::NONE,
            });
            h.run_steps(4);
            assert_eq!(
                h.state().reveal.waits(),
                !own,
                "m-p0-awaited-beside: a wheel over {} column",
                if own { "its own" } else { "another" }
            );
            h.state_mut().drawn = true;
            h.state_mut().came = true;
            h.run_steps(8);
            if !own {
                assert_eq!(h.state().reveal.ended(), Some(End::Whole));
                assert!(
                    in_view(&h),
                    "m-p0-awaited-beside: the result still shows its target"
                );
            }
        }
    }

    /// A mark for another target does nothing; an act outside a `ScrollColumn` asks for
    /// nothing; a form in another column than the pressed control is not revealed.
    #[test]
    fn only_the_bound_target_in_its_column_counts() {
        let mut page = Page::new();
        page.target = "another";
        let mut h = harness(page);
        h.run_steps(12);
        assert!(
            top(&h, "Target top") > HEIGHT,
            "another target was revealed"
        );
        assert_eq!(h.state().reveal.ended(), Some(End::Bound));
        let mut page = Page::new();
        page.raw = true;
        let mut h = harness(page);
        h.run_steps(8);
        assert_eq!(h.state().reveal.ended(), Some(End::Unowned));
        assert!(top(&h, "Target top") > HEIGHT, "a raw area was scrolled");
        let mut page = Page::new();
        page.nested = true;
        page.above = 400.0;
        let mut h = harness(page);
        let outer = top(&h, "Outer top");
        h.run_steps(12);
        assert_eq!(h.state().reveal.ended(), Some(End::Bound));
        assert!(
            (top(&h, "Outer top") - outer).abs() < 0.5,
            "a form in another column moved a column"
        );
    }

    /// Taller than the view: an opened form shows its top, once.
    #[test]
    fn a_tall_form_shows_its_top() {
        let mut page = Page::new();
        page.tall = 600.0;
        let mut h = harness(page);
        h.run_steps(8);
        assert_eq!(h.state().reveal.ended(), Some(End::TooTall));
        let at = h.get_by_label("Target top").rect();
        assert!(at.top() >= -0.5 && at.bottom() <= HEIGHT + 0.5, "{at:?}");
    }

    /// Input in the act's own frame ends the reveal, also another press; nothing moves. Sol
    /// 2-011: the input goes to the reveal's column (the pointer over it, nothing focused), as
    /// the one cancel rule asks (`cancel`); the two-column case table checks input elsewhere.
    #[test]
    fn input_in_the_acts_frame_cancels() {
        let events = [
            egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                delta: egui::vec2(0.0, 0.0),
                modifiers: egui::Modifiers::NONE,
            },
            egui::Event::Text("s".into()),
            egui::Event::Key {
                key: egui::Key::ArrowDown,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            },
            egui::Event::PointerButton {
                pos: egui::pos2(WIDTH - 20.0, HEIGHT - 20.0),
                button: egui::PointerButton::Secondary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ];
        for event in events {
            let mut h = harness(quiet());
            let before = top(&h, "Target top");
            h.state_mut().ask = true;
            let events = &mut h.input_mut().events;
            events.push(egui::Event::PointerMoved(egui::pos2(WIDTH / 2.0, 100.0)));
            events.push(event.clone());
            h.run_steps(8);
            assert!(
                (top(&h, "Target top") - before).abs() < 0.5
                    && h.state().reveal.ended() == Some(End::ByTheDm),
                "{event:?} in the act's frame did not cancel: {before} then {}; {:?}",
                top(&h, "Target top"),
                h.state().reveal.ended()
            );
        }
    }

    /// The opener's one focus inside the form is allowed; a focus outside it ends the reveal
    /// before the column moves.
    #[test]
    fn a_focus_change_after_the_ask_cancels() {
        let mut page = Page::new();
        page.focus_inside = Some(true);
        let mut h = harness(page);
        h.run_steps(8);
        assert_eq!(h.state().reveal.ended(), Some(End::Whole));
        let mut h = harness(quiet());
        h.state_mut().focus_inside = Some(false);
        h.state_mut().ask = true;
        let before = top(&h, "Outer top");
        h.run_steps(8);
        assert_eq!(h.state().reveal.ended(), Some(End::Focus));
        assert!(
            (top(&h, "Outer top") - before).abs() < 0.5,
            "the column moved after the focus change"
        );
    }

    /// R46: a column that does not host reveals binds no request: the act ends `Unowned` and
    /// nothing moves.
    #[test]
    fn only_a_column_that_hosts_reveals_binds() {
        let mut page = Page::new();
        page.plain = true;
        let mut h = harness(page);
        let before = top(&h, "Target top");
        let view_top = top(&h, "Outer top");
        h.run_steps(8);
        assert_eq!(h.state().reveal.ended(), Some(End::Unowned));
        assert!(
            (top(&h, "Target top") - before).abs() < 0.5,
            "a column that hosts no reveal moved"
        );
        // Layout PR 10c (ruling M18): a typed jump needs no act and no hosting column. The
        // column writes it as its own offset: the target's top at the view's top, once.
        h.state_mut().heading = true;
        h.run_steps(3);
        let at = top(&h, "Target top");
        assert!(
            (at - view_top).abs() < 1.0,
            "jump: the target's top {at} is at the view's top {view_top}"
        );
        h.run_steps(4);
        assert!(
            (top(&h, "Target top") - at).abs() < 0.5,
            "jump-once: the column stays where the jump put it"
        );
        // M-P0.25: the least scroll that shows the target, with no press. Above the view, its
        // top comes to the view's top; whole in view, nothing moves; below the view, its bottom
        // comes to the view's bottom (not its top to the view's top).
        h.state_mut().after = 600.0;
        h.state_mut().jump = true;
        h.run_steps(3);
        assert!(
            !in_view(&h) && top(&h, "Target top") < view_top,
            "the case starts with the target above the view"
        );
        h.state_mut().least = true;
        h.run_steps(3);
        assert!(
            in_view(&h) && (top(&h, "Target top") - view_top).abs() < 2.0,
            "m-p0-least-scroll: above the view, its top comes to the view's top: {} vs {view_top}",
            top(&h, "Target top")
        );
        let still = picture(&h);
        h.state_mut().least = true;
        h.run_steps(3);
        assert_eq!(
            picture(&h),
            still,
            "m-p0-least-scroll: whole in view, nothing moves"
        );
        h.state_mut().above += 600.0;
        h.run_steps(2);
        assert!(!in_view(&h), "the target is now below the view");
        h.state_mut().least = true;
        h.run_steps(3);
        let end = h.get_by_label("Target end").rect();
        assert!(
            in_view(&h) && end.bottom() > HEIGHT - 40.0 && top(&h, "Target top") > view_top + 100.0,
            "m-p0-least-scroll: below the view, the least scroll leaves the target whole at the \
             view's bottom: top {}, end {end:?}",
            top(&h, "Target top")
        );
        h.state_mut().after = 0.0;
        // Review of PR 770: the DM's own scroll of the column, here a press on its bar and a
        // drag of it, is seen (`dm_scrolls`) while it lasts and not after it, so a retry that
        // would move the column (the row menu's bring-back) ends on it.
        let bar = egui::pos2(WIDTH - 8.0 - super::super::lane_width() / 2.0, 40.0);
        let press = |pressed: bool, pos: egui::Pos2| egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        };
        h.input_mut().events.push(egui::Event::PointerMoved(bar));
        h.step();
        assert_eq!(
            h.state().scrolled,
            Some(false),
            "m-p0-dm-scrolls: no input yet"
        );
        h.input_mut().events.push(press(true, bar));
        h.step();
        assert_eq!(
            h.state().scrolled,
            Some(true),
            "m-p0-dm-scrolls: a press on the column's bar"
        );
        h.input_mut()
            .events
            .push(egui::Event::PointerMoved(bar + egui::vec2(0.0, 20.0)));
        h.step();
        assert_eq!(
            h.state().scrolled,
            Some(true),
            "m-p0-dm-scrolls: the bar's drag"
        );
        h.input_mut()
            .events
            .push(press(false, bar + egui::vec2(0.0, 20.0)));
        h.step();
        h.step();
        assert_eq!(
            h.state().scrolled,
            Some(false),
            "m-p0-dm-scrolls: after the release"
        );
        // Layout PR 10d: a jump asked while the DM drags the column's content (the pointer still
        // down, moving it) is dropped: the DM's scroll wins, and the column moves only with the
        // drag.
        let grab = egui::pos2(WIDTH / 2.0, 150.0);
        h.event(egui::Event::PointerMoved(grab));
        h.event(egui::Event::PointerButton {
            pos: grab,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        });
        h.step();
        for step in 1..=4 {
            h.event(egui::Event::PointerMoved(
                grab - egui::vec2(0.0, 20.0 * step as f32),
            ));
            h.step();
        }
        let dragging = h.ctx.dragged_id().is_some();
        let before = top(&h, "Target top");
        h.state_mut().heading = true;
        for step in 5..=7 {
            h.event(egui::Event::PointerMoved(
                grab - egui::vec2(0.0, 20.0 * step as f32),
            ));
            h.step();
        }
        let after = top(&h, "Target top");
        assert!(
            dragging && (after - (before - 60.0)).abs() <= 2.0,
            "jump-while-dragging: asked during the DM's drag ({dragging}), the jump moved the \
             target's top from {before} to {after} (the view's top is {view_top})"
        );
        // M0450: a column outside a pane that follows the focus (the session banner) brings a
        // control that gains keyboard focus whole into its view, by the pane's rules; the DM's
        // wheel after it is not pulled back.
        h.event(egui::Event::PointerButton {
            pos: grab - egui::vec2(0.0, 140.0),
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        });
        h.run_steps(2);
        h.state_mut().follows = true;
        let whole = |h: &egui_kittest::Harness<'_, Page>, name: &str| {
            let rect = h.get_by_label(name).rect();
            (
                rect.top() >= view_top - 0.5 && rect.bottom() <= HEIGHT + 0.5,
                rect,
            )
        };
        assert!(
            !whole(&h, "Other").0,
            "the case starts with Other out of view"
        );
        h.state_mut().focus_inside = Some(false);
        h.run_steps(4);
        let (shown, other) = whole(&h, "Other");
        assert!(
            h.get_by_label("Other").is_focused() && shown,
            "m0450-focus: Other took the focus and is whole in the view: {other:?}"
        );
        h.input_mut().events.push(key(egui::Key::Tab));
        h.run_steps(4);
        let focused = h.get_by_label("Field").is_focused();
        let (shown, field) = whole(&h, "Field");
        assert!(
            focused && shown,
            "m0450-tab: Tab gave the hidden Field the focus ({focused}) and it is whole in the \
             view: {field:?}"
        );
        wheel(&mut h, 120.0);
        h.run_steps(4);
        let moved = h.get_by_label("Field").rect();
        assert!(
            moved.top() > field.top() + 60.0,
            "m0450-wheel: the wheel moved Field from {field:?} to {moved:?}, and it stays"
        );
    }

    /// The tops of the page's markers, for a picture-by-picture comparison.
    fn picture(h: &egui_kittest::Harness<'_, Page>) -> [f32; 3] {
        [
            top(h, "Outer top"),
            top(h, "Target top"),
            top(h, "Page end"),
        ]
    }

    /// From the pass in which the reveal ended, every following pass shows the same picture.
    fn still_after(h: &mut egui_kittest::Harness<'_, Page>, case: &str) {
        let last = picture(h);
        for step in 0..10 {
            h.step();
            assert_eq!(
                picture(h),
                last,
                "{case}: the column moved {step} passes after the reveal ended ({:?})",
                h.state().reveal.ended()
            );
        }
    }

    /// F1 (R46): an older egui request in the reveal's column never lands after the reveal
    /// ended. (a) A raw request in pass A, then an open whose form is whole, in pass B; (b) the
    /// same, with text typed in B; (c) a raw request in the same pass as the part's own move.
    #[test]
    fn no_older_request_lands_after_a_reveal_ends() {
        // (a) The page end sits 40 points below the view, so the raw request moves the column a
        // little and the form stays whole.
        let mut page = quiet();
        page.above = 0.0;
        let mut h = harness(page);
        let end = h.get_by_label("Page end").rect().bottom();
        h.state_mut().after = HEIGHT + 40.0 - end;
        h.run_steps(2);
        h.state_mut().jump = true;
        h.step();
        h.state_mut().ask = true;
        h.step();
        assert_eq!(h.state().reveal.ended(), Some(End::Whole), "(a)");
        still_after(&mut h, "(a)");
        // (b) A running reveal, cancelled by text in its own pass, after a raw request: the text
        // goes to its column (the pointer over it, nothing focused; Sol 2-011).
        let mut h = harness(quiet());
        h.state_mut().after = 400.0;
        h.state_mut().jump = true;
        h.step();
        h.state_mut().ask = true;
        let events = &mut h.input_mut().events;
        events.push(egui::Event::PointerMoved(egui::pos2(WIDTH / 2.0, 100.0)));
        events.push(egui::Event::Text("s".into()));
        h.step();
        assert_eq!(h.state().reveal.ended(), Some(End::ByTheDm), "(b)");
        still_after(&mut h, "(b)");
        // A new request after the DM's input (a caret follow, a focus scroll) still lands.
        wheel(&mut h, 3000.0);
        h.run_steps(40);
        assert!(
            top(&h, "Page end") > HEIGHT,
            "(b): the wheel did not go back up"
        );
        h.state_mut().jump = true;
        h.run_steps(2);
        let end = h.get_by_label("Page end").rect();
        assert!(
            end.top() >= -0.5 && end.bottom() <= HEIGHT + 0.5,
            "(b): a new request after the reveal ended did not land: {end:?}"
        );
        // (c) The open's own move and a raw request to the page's end in one pass: the reveal's
        // offset wins and the form shows whole; nothing moves after.
        let mut h = harness(quiet());
        h.state_mut().after = 400.0;
        h.state_mut().ask = true;
        h.state_mut().jump = true;
        h.step();
        for _ in 0..4 {
            if !h.state().reveal.running() {
                break;
            }
            h.step();
        }
        assert_eq!(h.state().reveal.ended(), Some(End::Whole), "(c)");
        assert!(in_view(&h), "(c): the form is not whole");
        still_after(&mut h, "(c)");
    }

    /// R43: the installed style animates no scroll: a programmatic scroll in a column lands
    /// whole in the second pass after its request (egui applies its zero-length target when the
    /// next pass begins), and nothing moves after.
    #[test]
    fn no_programmatic_scroll_animates() {
        let ctx = Context::default();
        super::super::theme::install(&ctx);
        let animation = ctx.style().scroll_animation;
        assert!(
            animation.duration.max == 0.0 && animation.points_per_second.is_infinite(),
            "the style animates scrolls: {animation:?}"
        );
        let mut h = harness(quiet());
        h.state_mut().jump = true;
        h.run_steps(3);
        let first = top(&h, "Page end");
        h.run_steps(10);
        assert!(
            (0.0..HEIGHT).contains(&first) && (top(&h, "Page end") - first).abs() < 0.5,
            "the scroll arrived over frames: {first} then {}",
            top(&h, "Page end")
        );
    }
}
