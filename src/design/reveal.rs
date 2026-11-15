//! One reveal part (mechanism M6, ruling R34): after an act or an open, the least scroll that
//! brings its target (an opened form, an act's result and the next action it makes possible)
//! whole into view, in the scroll area that owns the target.
//!
//! - **A ticket, asked at the act.** [`Reveal::ask`] consumes an [`Act`], which only a press of
//!   this frame gives ([`Act::clicked`]); an act of an older frame is refused. An `Act` is neither
//!   `Copy` nor `Clone`, so two reveals cannot take one act, and a draw path that has no press
//!   cannot ask. The request is bound at the ask to its owner (the [`Reveal`] value), its visit
//!   and its target, and at the target's first mark to the column that draws it.
//! - **Marks report geometry only.** [`Reveal::mark`] gives the rect of the bound target in this
//!   pass. A mark for another target or in another column does nothing; a mark outside a
//!   `ScrollColumn` (a raw `egui::ScrollArea`, `ScrollColumn::show_rows`) ends the request
//!   (`End::Unowned`) and moves nothing.
//! - **The column resolves once after its content** (`region`): progress is the column's real
//!   offset and the target's place in its content. The scroll is the offset the column applies at
//!   the start of its next pass, after it checked that the request is still live and that pass's
//!   input (`take_offset`). It is never animated: after the part writes an offset, and when it
//!   ends for the DM's input, it ends egui's stored animation of that column; the pane's focus
//!   reveal makes no request into a column where a reveal runs ([`holds`]).
//! - **It ends for good** ([`End`]): the target whole in view; a frame with no progress; a target
//!   taller than the view after one edge scroll (an opened form shows its top, an act's result its
//!   end, where its action is: Q2.1); any input of the DM's, read where the column starts (a
//!   wheel, a press or drag, a key, typed, pasted or composed text; in the act's own frame only
//!   the press itself is the act's); a focused field that an act's result would move; the owner's
//!   end (the `Reveal` dropped or [`Reveal::end`]; another visit; a pass of its column that does
//!   not draw the target; a frame in which its column was not drawn); a new ask (`Replaced`);
//!   [`PASSES`] scroll frames; and a deadline of `DEADLINE` frames from the act, which also ends
//!   a request whose target never shows. A resize never restarts it.
//!
//! Nothing repaints for a reveal once it has ended. A column's work per pass is one look at the
//! registry of live reveals (one per owner that asked), never at its content.
use std::{
    hash::Hash,
    sync::{Arc, Mutex, Weak},
};

use eframe::egui::{self, Context, Id, Rect, Response, Ui, Vec2};

/// A reveal's bound: frames in which it scrolls.
pub const PASSES: u8 = 4;

/// The frames after its act at which a reveal ends, whatever happened.
const DEADLINE: u64 = 2 * PASSES as u64;

/// The act that may ask for a reveal: a press of this frame. Neither `Copy` nor `Clone`:
/// [`Reveal::ask`] consumes it.
#[derive(Debug)]
pub struct Act {
    ctx: Context,
    frame: u64,
    id: Id,
    /// The pressed control, in screen points at the act.
    pressed: Rect,
}

impl Act {
    /// The act of a control the DM pressed in this pass.
    pub fn clicked(response: &Response) -> Option<Self> {
        response.clicked().then(|| Self {
            ctx: response.ctx.clone(),
            frame: response.ctx.cumulative_frame_nr(),
            id: response.id,
            pressed: response.rect,
        })
    }

    /// An act in this frame of `ctx`, for a test that opens a form without a press.
    #[cfg(test)]
    pub(crate) fn test(ctx: &Context, salt: impl Hash) -> Self {
        Self {
            ctx: ctx.clone(),
            frame: ctx.cumulative_frame_nr(),
            id: Id::new(("reveal_test_act", salt)),
            pressed: Rect::NOTHING,
        }
    }
}

/// What happens to the pressed control after an act (R34): an act's result may be brought into
/// view only when the control is replaced or stays whole in view.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pressed {
    Replaced,
    Stays,
}

/// What the target is, which decides the edge that shows when it is taller than the view.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Want {
    /// An opened form: its top shows. The field its opener focuses may move with it.
    Open,
    /// An act's result and the next action it makes possible: its end shows. It never moves a
    /// focused field, and a control that stays stays whole in view.
    #[allow(dead_code, reason = "step 2's callers (lane B6) ask for a result")]
    Result(Pressed),
}

/// Why a reveal ended. Every end is for good.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum End {
    Whole,
    NoProgress,
    TooTall,
    ByTheDm,
    /// An act's result would move a focused field.
    Focused,
    OwnerGone,
    /// Its target is drawn outside a `ScrollColumn`: nothing it may scroll.
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
    pressed: Rect,
    visit: Id,
    target: Id,
    want: Want,
    phase: Result<(), End>,
    deadline: u64,
    left: u8,
    /// The column that draws the target, bound at its first mark.
    column: Option<Id>,
    /// The target's rect, the column's content `Ui` and the pass, as marked.
    marked: Option<(Rect, Id, u64)>,
    /// The last frame its column resolved it.
    frame: Option<u64>,
    asked: Option<Asked>,
    /// The target is taller than the view: one edge scroll, then `TooTall`.
    edge: bool,
    /// The pressed control's place in the column's content, when it stays.
    pressed_content: Option<Rect>,
    /// The offset for the column's next pass, with the frame that wrote it.
    pending: Option<(f32, u64)>,
    /// End egui's stored animation of the column after its content.
    settle: bool,
}

impl Run {
    fn end(&mut self, end: End) {
        if self.phase.is_ok() {
            self.phase = Err(end);
            self.pending = None;
            self.settle |= end == End::ByTheDm;
        }
    }
}

type Shared = Arc<Mutex<Run>>;

fn lock(run: &Mutex<Run>) -> std::sync::MutexGuard<'_, Run> {
    run.lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// The live reveals, for the columns. The owner holds the only strong handle: dropping its
/// `Reveal` ends the request, and its queued offset with it.
#[derive(Clone, Default)]
struct Registry(Vec<Weak<Mutex<Run>>>);

fn registry_id() -> Id {
    Id::new("design_reveal_registry")
}

fn live(ctx: &Context) -> Vec<Shared> {
    ctx.data_mut(|data| {
        let registry = data.get_temp_mut_or_default::<Registry>(registry_id());
        registry.0.retain(|run| run.strong_count() > 0);
        registry.0.iter().filter_map(Weak::upgrade).collect()
    })
}

/// One owner's reveal. The owner holds it in the value whose life is the target's (the open
/// form), asks at its act and marks the target while drawing.
#[derive(Debug, Default)]
pub struct Reveal {
    run: Option<Shared>,
}

impl Reveal {
    /// Ask at `act` for `target` of `visit` (the owner's object and what it shows now, such as
    /// the remote campaign and the campaign open) to come into view. An act of an older frame
    /// asks for nothing; a new ask replaces the request before it.
    pub fn ask(&mut self, act: Act, visit: impl Hash, target: impl Hash, want: Want) {
        if act.ctx.cumulative_frame_nr() != act.frame {
            return;
        }
        if let Some(old) = self.run.take() {
            lock(&old).end(End::Replaced);
        }
        let run = Arc::new(Mutex::new(Run {
            act_frame: act.frame,
            act_id: act.id,
            pressed: act.pressed,
            visit: Id::new(visit),
            target: Id::new(target),
            want,
            phase: Ok(()),
            deadline: act.frame + DEADLINE,
            left: PASSES,
            column: None,
            marked: None,
            frame: None,
            asked: None,
            edge: false,
            pressed_content: None,
            pending: None,
            settle: false,
        }));
        act.ctx.data_mut(|data| {
            data.get_temp_mut_or_default::<Registry>(registry_id())
                .0
                .push(Arc::downgrade(&run));
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
    /// pass. Geometry only: the column that owns it resolves the request after its content.
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
        let Some((content, region)) = owning_region(ui) else {
            run.end(End::Unowned);
            return;
        };
        if run.column.is_some_and(|column| column != region.key) {
            return;
        }
        run.column = Some(region.key);
        run.marked = Some((target_rect, content, ui.ctx().cumulative_pass_nr()));
    }
}

/// Whether a reveal runs in the column that `ui` is drawn in: the pane's focus reveal then makes
/// no request into it.
pub(super) fn holds(ui: &Ui) -> bool {
    let Some((_, region)) = owning_region(ui) else {
        return false;
    };
    live(ui.ctx()).iter().any(|run| {
        let run = lock(run);
        run.phase.is_ok() && run.column == Some(region.key)
    })
}

/// Any input of the DM's in this pass, for a reveal asked in `act_frame`.
fn dm_input(ui: &Ui, act_frame: u64) -> bool {
    input_in(ui, ui.ctx().cumulative_frame_nr() == act_frame)
}

/// Any input of the DM's in this pass. In the act's own frame (`act`) only the press and its
/// key (`Enter`, `Space`) are the act's.
fn input_in(ui: &Ui, act: bool) -> bool {
    ui.input(|input| {
        input.raw_scroll_delta != Vec2::ZERO
            || input.smooth_scroll_delta != Vec2::ZERO
            || (!act && (input.pointer.any_pressed() || input.pointer.is_decidedly_dragging()))
            || input.events.iter().any(|event| match event {
                egui::Event::Text(_)
                | egui::Event::Paste(_)
                | egui::Event::Ime(_)
                | egui::Event::MouseWheel { .. } => true,
                egui::Event::Key {
                    key, pressed: true, ..
                } => !(act && matches!(key, egui::Key::Enter | egui::Key::Space)),
                _ => false,
            })
    })
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
/// `Ui`'s id and its facts.
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

/// The offset a live reveal of the column `key` queued for this pass, taken once, after that
/// reveal and this pass's input are checked. `ScrollColumn` calls it before its area begins.
pub(super) fn take_offset(ui: &Ui, key: Id) -> Option<f32> {
    let frame = ui.ctx().cumulative_frame_nr();
    let mut offset = None;
    for run in live(ui.ctx()) {
        let mut run = lock(&run);
        if run.column != Some(key) {
            continue;
        }
        let Some((to, written)) = run.pending.take() else {
            continue;
        };
        if run.phase.is_err() || !(written == frame || written + 1 == frame) {
            continue;
        }
        if dm_input(ui, run.act_frame) {
            run.end(End::ByTheDm);
            continue;
        }
        run.settle = true;
        offset = Some(to);
    }
    offset
}

/// Run `add` as the content of the scroll column `key`, whose content `Ui` is `ui` and whose
/// viewport (egui's, in content points) is `viewport`; then resolve each reveal of this column.
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
    // The stack names a `Ui` by its unique id (`owning_region`).
    let content = ui.unique_id();
    ui.ctx()
        .data_mut(|data| data.insert_temp(region_id(content), region));
    // The DM's input, read before any child can consume a key: outside an act's frame, and in
    // it (a reveal asked by this content's act).
    let input = [input_in(ui, false), input_in(ui, true)];
    let inner = add(ui);
    let frame = ui.ctx().cumulative_frame_nr();
    let mut settle = false;
    for run in live(ui.ctx()) {
        let mut run = lock(&run);
        let input = input[usize::from(run.act_frame == frame)];
        resolve(ui, &mut run, &region, content, input);
        if run.column == Some(key) {
            settle |= std::mem::take(&mut run.settle);
        }
    }
    if settle {
        settle_animation(ui);
    }
    inner
}

/// One step of `run` after the content of the column `region` (content `Ui` `content`).
fn resolve(ui: &Ui, run: &mut Run, region: &Region, content: Id, input: bool) {
    let (frame, pass) = (
        ui.ctx().cumulative_frame_nr(),
        ui.ctx().cumulative_pass_nr(),
    );
    if run.phase.is_err() {
        return;
    }
    if frame > run.deadline {
        // Also a request whose target never marked ages out here.
        run.end(End::Bound);
        return;
    }
    if run.column != Some(region.key) {
        return;
    }
    if input {
        run.end(End::ByTheDm);
        return;
    }
    if run.frame.is_some_and(|last| frame > last + 1) {
        // A frame drew no pass of its column: the page left.
        run.end(End::OwnerGone);
        return;
    }
    run.frame = Some(frame);
    let Some(target) = run
        .marked
        .filter(|&(_, at, marked)| at == content && marked == pass)
        .map(|(rect, ..)| rect)
    else {
        // A pass of its column that does not draw the target.
        run.end(End::OwnerGone);
        return;
    };
    let view = region.view;
    if run.pressed_content.is_none() && run.want == Want::Result(Pressed::Stays) {
        // The act's pass or the next: the column has not moved since the press.
        run.pressed_content = Some(run.pressed.translate(egui::vec2(0.0, -region.origin)));
    }
    let span = match run.pressed_content {
        Some(pressed) => target.union(pressed.translate(egui::vec2(0.0, region.origin))),
        None => target,
    };
    if span.top() >= view.top() - 0.5 && span.bottom() <= view.bottom() + 0.5 {
        run.end(End::Whole);
        return;
    }
    if matches!(run.want, Want::Result(_))
        && ui
            .ctx()
            .memory(|memory| memory.focused())
            .is_some_and(|focused| focused != run.act_id)
    {
        run.end(End::Focused);
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
    let tall = span.height() > view.height();
    if tall && run.want == Want::Result(Pressed::Stays) {
        // The result and the control that stays do not fit together: the control stays.
        run.end(End::TooTall);
        return;
    }
    run.edge |= tall;
    let delta = if tall {
        match run.want {
            Want::Open => span.top() - view.top(),
            Want::Result(_) => span.bottom() - view.bottom(),
        }
    } else if span.bottom() > view.bottom() {
        span.bottom() - view.bottom()
    } else {
        span.top() - view.top()
    };
    run.asked = Some(Asked {
        frame,
        offset: region.offset,
        content_top,
    });
    run.pending = Some(((region.offset + delta).max(0.0), frame));
    ui.ctx().request_repaint();
}

/// End egui's stored scroll animation of the column whose content `ui` is: a request that
/// arrives at once (no duration) for less than a point, which egui ends at its next pass (an
/// animation less than a point from its target has arrived). Made after all of the column's
/// content, so that no nested area takes it.
#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
fn settle_animation(ui: &Ui) {
    ui.scroll_with_delta_animation(
        egui::vec2(0.0, -0.01),
        egui::style::ScrollAnimation {
            points_per_second: f32::INFINITY,
            duration: egui::Rangef::new(0.0, 0.0),
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui_kittest::kittest::Queryable;

    const WIDTH: f32 = 400.0;
    const HEIGHT: f32 = 300.0;

    /// A test page: a column with `above` px of room, the target (`Target top`, a block of
    /// `tall` px, `Target end`), then 900 px. `ask` asks in the next frame; `old` is an act of
    /// an older frame; `animate` starts egui's own animated scroll to the page's end.
    struct Page {
        reveal: Reveal,
        ask: bool,
        old: Option<Act>,
        want: Want,
        target: &'static str,
        above: f32,
        tall: f32,
        drawn: bool,
        nested: bool,
        raw: bool,
        animate: bool,
        /// Text the next frame's ask comes with (the act's frame).
        with: Option<egui::Event>,
    }

    impl Page {
        fn new() -> Self {
            Self {
                reveal: Reveal::default(),
                ask: true,
                old: None,
                want: Want::Open,
                target: "t",
                above: 900.0,
                tall: 40.0,
                drawn: true,
                nested: false,
                raw: false,
                animate: false,
                with: None,
            }
        }
    }

    fn room(ui: &mut Ui, height: f32) {
        ui.allocate_exact_size(egui::vec2(1.0, height), egui::Sense::hover());
    }

    fn target(ui: &mut Ui, page: &mut Page) {
        if page.ask {
            page.ask = false;
            page.reveal.ask(Act::test(ui.ctx(), 1), 0, "t", page.want);
        }
        if let Some(act) = page.old.take() {
            page.reveal.ask(act, 0, "t", page.want);
        }
        if page.drawn {
            let tall = page.tall;
            let rect = ui
                .vertical(|ui| {
                    ui.label("Target top");
                    room(ui, tall);
                    ui.label("Target end");
                })
                .response
                .rect;
            page.reveal.mark(ui, 0, page.target, rect);
        }
    }

    #[allow(
        clippy::disallowed_methods,
        reason = "a test's raw scroll area and animation"
    )]
    fn harness(page: Page) -> egui_kittest::Harness<'static, Page> {
        egui_kittest::Harness::builder()
            .with_size(egui::vec2(WIDTH, HEIGHT))
            .build_ui_state(
                |ui, page: &mut Page| {
                    if page.raw {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            room(ui, page.above);
                            target(ui, page);
                            room(ui, 900.0);
                        });
                        return;
                    }
                    super::super::ScrollColumn::new("outer").show(ui, |ui| {
                        ui.label("Outer top");
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
                        if std::mem::take(&mut page.animate) {
                            ui.scroll_to_rect(end, None);
                        }
                        room(ui, 900.0);
                    });
                },
                page,
            )
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

    /// It opens whole, ends, and stays ended: a resize does not bring it back.
    #[test]
    fn a_reveal_shows_its_target_once() {
        let mut h = harness(Page::new());
        h.run_steps(8);
        assert!(in_view(&h), "the target is not whole");
        assert_eq!(h.state().reveal.ended(), Some(End::Whole));
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

    /// An act of an older frame asks for nothing.
    #[test]
    fn an_old_act_asks_for_nothing() {
        let mut page = Page::new();
        page.ask = false;
        let mut h = harness(page);
        let act = Act::test(&h.ctx, 2);
        h.step();
        h.state_mut().old = Some(act);
        h.run_steps(8);
        assert!(
            h.state().reveal.ended().is_none() && !h.state().reveal.running(),
            "an old act made a request"
        );
        assert!(top(&h, "Target top") > HEIGHT, "an old act scrolled");
    }

    /// E1: A queues an offset and ends (its owner drops it, or ends it); the column's next pass
    /// draws an unrelated page and does not move.
    #[test]
    fn an_ended_request_never_moves_its_column() {
        for drop in [true, false] {
            let mut page = Page::new();
            page.ask = false;
            let mut h = harness(page);
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
            // Another page in the same column: the target is not drawn.
            h.state_mut().drawn = false;
            let outer = top(&h, "Outer top");
            h.run_steps(6);
            assert!(
                (top(&h, "Outer top") - outer).abs() < 0.5,
                "an ended request moved the column (dropped: {drop})"
            );
        }
    }

    /// A pass of its column that does not draw the target ends it; the deadline ends a request
    /// whose target never shows.
    #[test]
    fn a_missing_target_ends_the_reveal() {
        let mut page = Page::new();
        page.ask = false;
        let mut h = harness(page);
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
        let mut page = Page::new();
        page.drawn = false;
        let mut h = harness(page);
        h.run_steps(12);
        h.state_mut().drawn = true;
        h.run_steps(8);
        assert_eq!(h.state().reveal.ended(), Some(End::Bound));
        assert!(
            top(&h, "Target top") > HEIGHT,
            "a late target was scrolled to"
        );
    }

    /// A mark for another target does nothing; a mark outside a `ScrollColumn` ends it and moves
    /// nothing.
    #[test]
    fn a_mark_reports_only_its_bound_target() {
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
    }

    /// The target in a nested column scrolls that column only: the outer column stays.
    #[test]
    fn a_nested_column_owns_its_target() {
        let mut page = Page::new();
        page.nested = true;
        page.above = 400.0;
        let mut h = harness(page);
        let outer = top(&h, "Outer top");
        h.run_steps(8);
        assert_eq!(h.state().reveal.ended(), Some(End::Whole));
        let (start, end) = (top(&h, "Target top"), h.get_by_label("Target end").rect());
        assert!(
            start >= outer && end.bottom() <= outer + 140.0,
            "the target is not whole in its column: {start} to {end:?}"
        );
        assert!(
            (top(&h, "Outer top") - outer).abs() < 0.5,
            "the outer column moved: {outer} then {}",
            top(&h, "Outer top")
        );
    }

    /// Taller than the view: an opened form shows its top, an act's result its end, once.
    #[test]
    fn a_tall_target_shows_the_edge_its_want_names() {
        for (want, name) in [
            (Want::Open, "Target top"),
            (Want::Result(Pressed::Replaced), "Target end"),
        ] {
            let mut page = Page::new();
            page.want = want;
            page.tall = 600.0;
            let mut h = harness(page);
            h.run_steps(8);
            assert_eq!(h.state().reveal.ended(), Some(End::TooTall), "{want:?}");
            let at = h.get_by_label(name).rect();
            assert!(
                at.top() >= -0.5 && at.bottom() <= HEIGHT + 0.5,
                "{want:?}: {name} at {at:?}"
            );
        }
    }

    /// A wheel turn or typed text in the act's own frame ends the reveal: nothing moves.
    #[test]
    fn input_in_the_acts_frame_cancels() {
        for event in [
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
        ] {
            let mut h = harness(Page::new());
            h.state_mut().ask = false;
            h.step();
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

    /// egui's own animation, started before the act, does not outlive the reveal: after the DM
    /// types, nothing moves on the next frames.
    #[test]
    fn no_animation_outlives_the_reveal() {
        let mut page = Page::new();
        page.ask = false;
        page.animate = true;
        let mut h = harness(page);
        h.step();
        h.state_mut().ask = true;
        h.step();
        h.event(egui::Event::Text("s".into()));
        h.step();
        let mut tops = vec![top(&h, "Target top")];
        for _ in 0..20 {
            h.step();
            tops.push(top(&h, "Target top"));
        }
        assert!(
            tops.windows(2).all(|pair| (pair[0] - pair[1]).abs() < 0.5),
            "the column moved after the DM typed: {tops:?}"
        );
    }
}
