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
//!   in the app animates a programmatic scroll (R43, [`super::theme::install`]).
//! - **It ends for good** ([`End`]): the target whole in view; a frame with no progress; a form
//!   taller than the view after one scroll to its top (Q2.1); any input of the DM's, read where
//!   the column starts (a wheel, a press or drag, a key, typed, pasted or composed text, an
//!   accessibility action), of which only the input that made the act is exempt, in the act's
//!   frame; a focus change after the ask other than the opener's one focus inside the form; the
//!   owner's end (the `Reveal` dropped or [`Reveal::end`]; another visit; a pass of its column
//!   that does not draw a target it drew before; a frame in which its column was not drawn); a
//!   new ask (`Replaced`); [`PASSES`] scroll frames; and a deadline of `DEADLINE` frames from the
//!   act, which also ends a request whose target never shows. A resize never restarts it.
//!
//! A run that ended leaves the registry of live requests at once (its `End` stays with its
//! owner), so a column's work per pass is a look at the requests that run now.
use std::{
    hash::Hash,
    sync::{Arc, Mutex, Weak},
};

use eframe::egui::{self, Context, Id, Rect, Response, Ui, Vec2};

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
        let exempt = ui.input(|input| {
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
        })?;
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
            column: owning_region(ui).map(|(_, region)| region.key),
        })
    }

    /// An act in this frame, in the column `ui` is drawn in, for a test that opens a form with
    /// no press. No input of the frame is the act's.
    #[cfg(test)]
    pub(crate) fn test(ui: &Ui, salt: impl Hash) -> Self {
        Self {
            column: owning_region(ui).map(|(_, region)| region.key),
            ..Self::outside(ui.ctx(), salt)
        }
    }

    /// An act in this frame of `ctx` and in no column, for a test with no frame drawn: its
    /// reveal ends `Unowned`.
    #[cfg(test)]
    pub(crate) fn outside(ctx: &Context, salt: impl Hash) -> Self {
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
    /// The pressed control is not in a `ScrollColumn`: nothing the part may scroll.
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
    /// `ScrollColumn` ends `Unowned`; a new ask replaces the request before it, also another
    /// owner's in the same column.
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

    /// The owner ended (its object went away, its campaign changed): the request ends for good.
    pub fn end(&mut self) {
        if let Some(run) = &self.run {
            lock(run).end(End::OwnerGone);
        }
    }

    /// Whether it runs.
    #[cfg(test)]
    pub(crate) fn running(&self) -> bool {
        self.run.as_ref().is_some_and(|run| lock(run).phase.is_ok())
    }

    /// Why it ended; `None` while it runs or before any act.
    #[cfg(test)]
    pub(crate) fn ended(&self) -> Option<End> {
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
            .and_then(|focused| ctx.read_response(focused))
            .is_some_and(|field| form.contains_rect(field.rect))
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

/// Whether a reveal runs in the column that `ui` is drawn in: the pane's focus reveal then makes
/// no request into it.
pub(super) fn holds(ui: &Ui) -> bool {
    let Some((_, region)) = owning_region(ui) else {
        return false;
    };
    running(ui.ctx())
        .iter()
        .any(|run| lock(run).column == region.key)
}

/// The DM's input of a pass, read before any child can consume a key.
struct Facts {
    frame: u64,
    events: Vec<egui::Event>,
    /// A wheel turn or a drag.
    moved: bool,
}

fn facts(ui: &Ui) -> Facts {
    let frame = ui.ctx().cumulative_frame_nr();
    ui.input(|input| Facts {
        frame,
        events: input.events.clone(),
        moved: input.raw_scroll_delta != Vec2::ZERO
            || input.smooth_scroll_delta != Vec2::ZERO
            || input.pointer.is_decidedly_dragging(),
    })
}

/// Whether `facts` hold input of the DM's for `run`. In the act's frame, each event that made
/// the act is exempt once; any other press, key, text, wheel or accessibility action counts.
fn dm_input(facts: &Facts, run: &Run) -> bool {
    if facts.moved {
        return true;
    }
    let mut exempt: Vec<&egui::Event> = if facts.frame == run.act_frame {
        run.exempt.iter().collect()
    } else {
        Vec::new()
    };
    for event in &facts.events {
        if let Some(at) = exempt.iter().position(|made| *made == event) {
            exempt.remove(at);
            continue;
        }
        if matches!(
            event,
            egui::Event::Text(_)
                | egui::Event::Paste(_)
                | egui::Event::Ime(_)
                | egui::Event::MouseWheel { .. }
                | egui::Event::PointerButton { .. }
                | egui::Event::Key { pressed: true, .. }
                | egui::Event::AccessKitActionRequest(_)
        ) {
            return true;
        }
    }
    false
}

/// A `ScrollColumn`'s facts in one pass, recorded where its content starts.
#[derive(Clone, Copy, Debug)]
struct Region {
    pass: u64,
    /// The column's persistent key, which [`take_offset`] reads.
    key: Id,
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

/// The offset the request of the column `key` queued for this pass, taken once, after this
/// pass's input is checked. `ScrollColumn` calls it before its area begins.
pub(super) fn take_offset(ui: &Ui, key: Id) -> Option<f32> {
    let frame = ui.ctx().cumulative_frame_nr();
    let run = running(ui.ctx())
        .into_iter()
        .find(|run| lock(run).column == key)?;
    let mut run = lock(&run);
    let (to, written) = run.pending.take()?;
    if !(written == frame || written + 1 == frame) {
        return None;
    }
    if dm_input(&facts(ui), &run) {
        run.end(End::ByTheDm);
        return None;
    }
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
pub(super) fn region<R>(ui: &mut Ui, key: Id, viewport: Rect, add: impl FnOnce(&mut Ui) -> R) -> R {
    let origin = ui.max_rect().min.to_vec2();
    let view = viewport.translate(origin).intersect(ui.clip_rect());
    let region = Region {
        pass: ui.ctx().cumulative_pass_nr(),
        key,
        view,
        origin: origin.y,
        offset: viewport.top(),
    };
    let content = ui.unique_id();
    ui.ctx()
        .data_mut(|data| data.insert_temp(region_id(content), region));
    // The DM's input, read before any child can consume a key.
    let facts = facts(ui);
    let inner = add(ui);
    // The request that runs in this column now (one asked by this content's act replaced any
    // older one), judged by the input of this pass.
    let run = running(ui.ctx())
        .into_iter()
        .find(|run| lock(run).column == key);
    if let Some(run) = run {
        let mut run = lock(&run);
        let input = dm_input(&facts, &run);
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
    // A control that took the focus since the ask, other than the opener's field.
    if !focus_allowed(ui.ctx(), run, target) {
        run.end(End::Focus);
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
            let rect = ui
                .vertical(|ui| {
                    ui.label("Target top");
                    field = Some(ui.button("Field"));
                    room(ui, tall);
                    ui.label("Target end");
                })
                .response
                .rect;
            if page.focus_inside == Some(true)
                && let Some(field) = field
            {
                page.focus_inside = None;
                field.request_focus();
            }
            page.reveal.mark(ui, 0, page.target, rect);
        }
    }

    #[allow(clippy::disallowed_methods, reason = "a test's scroll request")]
    fn content(ui: &mut Ui, page: &mut Page) {
        ui.label("Outer top");
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
            page.reveal.ask(Act::test(ui, 1), 0, "t");
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
        let end = ui.label("Page end").rect;
        if std::mem::take(&mut page.jump) {
            ui.scroll_to_rect(end, None);
        }
        room(ui, 900.0);
    }

    #[allow(clippy::disallowed_methods, reason = "a test's raw scroll area")]
    fn harness(mut page: Page) -> egui_kittest::Harness<'static, Page> {
        // The page asks after the theme (its fonts) is installed.
        let ask = std::mem::take(&mut page.ask);
        let mut h = egui_kittest::Harness::builder()
            .with_size(egui::vec2(WIDTH, HEIGHT))
            .build_ui_state(
                |ui, page: &mut Page| {
                    if page.raw {
                        egui::ScrollArea::vertical().show(ui, |ui| content(ui, page));
                    } else {
                        super::super::ScrollColumn::new("outer").show(ui, |ui| content(ui, page));
                    }
                },
                page,
            );
        super::super::theme::install(&h.ctx);
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
    /// pass: with a release inside its old rect, or with an accessibility click while the
    /// control is disabled.
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
        let away = top(&h, "Target top");
        let open = h.get_by_label("Open").rect();
        let id = h.state().saved.as_ref().expect("a saved Response").id;
        for disabled in [false, true] {
            h.state_mut().disabled = disabled;
            h.step();
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
                h.state().reveal.ended() == Some(End::ByTheDm)
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

    /// Input in the act's own frame ends the reveal, also another press; nothing moves.
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
            h.event(event.clone());
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

    /// R43: the installed style animates no scroll, and a programmatic scroll in a column
    /// arrives whole in one pass.
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
        h.step();
        h.step();
        let first = top(&h, "Page end");
        h.run_steps(10);
        assert!(
            (0.0..HEIGHT).contains(&first) && (top(&h, "Page end") - first).abs() < 0.5,
            "the scroll arrived over frames: {first} then {}",
            top(&h, "Page end")
        );
    }
}
