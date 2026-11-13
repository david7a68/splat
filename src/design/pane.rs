//! A pane: its body, then its one message footer, placed by the part for every caller.
//!
//! `Footer::pane(ui, body, add)` makes the body's `Ui` from the cursor to the pane's bottom and
//! draws the footer after it. The caller gives the messages and the body; it gets back only what
//! its body returns, never a y, so it cannot size the body by hand, draw the body before the
//! footer, or wrap it in a `Ui` the footer cannot see (round 28, `footer-escape.md`).
//!
//! - **`Body::Content` is the scroll column.** The part makes the vertical `ScrollColumn` (lane
//!   kept) and runs `add` as its content. Where the content ends is egui's own scroll output, the
//!   content's height and offset, not a guess from widget rects. The footer sits `S` below the
//!   content when the content is shorter than the room and not scrolled; it is pinned at the
//!   pane's bottom otherwise, and then draws a hairline where the content goes on below it. A body
//!   that takes all the room (a widget sized to the room, a layout that centres or justifies, a
//!   column as tall as the room) reads as tall: the footer is pinned below the room and never over
//!   a control. A header that must not scroll is drawn before the pane. A vertical `ScrollColumn`
//!   inside a `Content` pane is refused (a panic in debug builds and tests); a sideways scroll is
//!   allowed.
//! - **`Body::Fills(Fills::because(reason))`** is the one escape, for a body that manages its own
//!   room: an editor, the live roster, several columns that each scroll. Its footer is pinned
//!   below the room, with no hairline. Clippy refuses `Fills::because` outside `src/design/`, so
//!   each use carries a counted allow.
//!
//! The part keeps the body's top and never covers its content. Content that places itself by
//! the room's height (centred in it) moves when a message shrinks the room: that is the caller's
//! fault. Clippy refuses `Ui::centered_and_justified` and `Ui::vertical_centered_justified`
//! outside `src/design/`, and the gate's stillness check (G1a) catches the rest.
//!
//! Proof once, in the part (`components::tests::iter28_p_the_pane_places_its_footer_for_every_body`
//! and `iter27_a_the_footer_follows_a_short_body`): placement for each body shape, short and long,
//! checked against the accessibility tree's own rects, and the press proof (body press and footer
//! press while the footer follows, is pinned, grows and empties). A caller that uses `pane` with
//! no custom layer, clip, `Area` or raw interaction does not repeat it. Duties left with the
//! caller: none for placement.
//!
//! The caller gets no y:
//!
//! ```compile_fail,E0308
//! use splat::pane::{Body, Footer};
//! fn body_end(ui: &mut eframe::egui::Ui) {
//!     let footer = Footer::new("pane", 400.0);
//!     let bottom: f32 = footer.pane(ui, Body::Content, |ui| {
//!         ui.label("A row");
//!     });
//! }
//! ```
//!
//! What the body returns comes back, and nothing else:
//!
//! ```
//! use splat::pane::{Body, Footer};
//! fn rows(ui: &mut eframe::egui::Ui) -> usize {
//!     let footer = Footer::new("pane", 400.0);
//!     footer.pane(ui, Body::Content, |ui| {
//!         ui.label("A row");
//!         1
//!     })
//! }
//! ```
//!
//! A footer is placed once, with its body: it cannot be drawn again, before or after.
//!
//! ```compile_fail,E0382
//! use splat::pane::{Body, Footer};
//! fn twice(ui: &mut eframe::egui::Ui) {
//!     let footer = Footer::new("pane", 400.0);
//!     footer.pane(ui, Body::Content, |ui| ui.label("Body"));
//!     footer.pane(ui, Body::Content, |ui| ui.label("A second body"));
//! }
//! ```
//!
//! ```
//! use splat::pane::{Body, Footer};
//! fn once(ui: &mut eframe::egui::Ui) {
//!     Footer::new("pane", 400.0).pane(ui, Body::Content, |ui| ui.label("Body"));
//! }
//! ```
//!
//! A body that fills its room names why, from a closed list, through the one constructor that
//! Clippy can refuse; no free words, and no way around `Fills::because`:
//!
//! ```compile_fail,E0308
//! use splat::pane::{Body, Footer};
//! fn editor(ui: &mut eframe::egui::Ui) {
//!     Footer::new("pane", 400.0).pane(ui, Body::Fills("an editor"), |_| ());
//! }
//! ```
//!
//! ```compile_fail,E0423
//! use splat::pane::{Body, Fills, FillsReason, Footer};
//! fn editor(ui: &mut eframe::egui::Ui) {
//!     Footer::new("pane", 400.0).pane(ui, Body::Fills(Fills(FillsReason::MarkdownEditor)), |_| ());
//! }
//! ```
//!
//! ```
//! use splat::pane::{Body, Fills, FillsReason, Footer};
//! fn editor(ui: &mut eframe::egui::Ui) {
//!     let body = Body::Fills(Fills::because(FillsReason::MarkdownEditor));
//!     Footer::new("pane", 400.0).pane(ui, body, |_| ());
//! }
//! ```
//!
//! A caller cannot change what a pane measured: the record's fields are private, read only.
//!
//! ```compile_fail,E0616
//! fn move_end(record: &mut splat::pane::PaneRecord) {
//!     record.offset = 0.0;
//! }
//! ```
//!
//! ```
//! fn read_end(record: &mut splat::pane::PaneRecord) {
//!     let _ = record.offset();
//! }
//! ```
//!
//! Stable rustdoc does not check the error codes above, so each failing case differs from the
//! compiling case after it only by the rule it shows.

pub use super::components::Footer;
use super::{layout::ScrollColumn, tokens::S};
use eframe::egui::{self, Id, LayerId, Rect, Sense, Ui, pos2, vec2};

/// What a pane's body is, for the footer's place.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Body {
    /// Rows, forms, readers and lists: the pane's scroll column. The footer follows the content
    /// `S` below its end, and is pinned when the content is as tall as the room or scrolled.
    Content,
    /// A body that manages its own room: the footer is pinned. The one named escape.
    Fills(Fills),
}

/// Why a body manages its own room. Built only by `Fills::because`, which Clippy refuses outside
/// `src/design/`: each use carries a counted allow.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Fills(FillsReason);

impl Fills {
    pub fn because(reason: FillsReason) -> Self {
        Self(reason)
    }

    pub fn reason(self) -> FillsReason {
        self.0
    }

    /// For the design system's own tests, which prove the escape: Clippy refuses `because`.
    #[cfg(test)]
    pub(in crate::design) fn for_test(reason: FillsReason) -> Self {
        Self(reason)
    }
}

/// Why a body fills its room. Closed: a new reason is a change to this part.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FillsReason {
    /// A Markdown editor as tall as its room (Library's Edit Markdown).
    MarkdownEditor,
    /// A note's editor as tall as its room (a session note, a template, a gone session).
    NoteEditor,
    /// The live run's roster, whose room is the pane less the footer.
    LiveRoster,
    /// Several columns side by side, each with its own scroll (Party's list and sheet).
    Columns,
}

/// How a `Body::Content` pane's scroll column behaves beyond the default, for `Footer::pane_content`.
/// The content stays the pane's scroll column, and the footer follows short content as for
/// `Body::Content`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ContentScroll {
    from_top: bool,
    holds: bool,
}

impl ContentScroll {
    /// Open at the top in this frame (a view shown again); a hold ends.
    pub fn from_top(mut self, from_top: bool) -> Self {
        self.from_top = from_top;
        self
    }

    /// Hold a pressed control under the pointer while its press changes the content above it:
    /// the body calls [`hold_press`] at the press and [`held_seen`] where it draws that control.
    /// In the passes after the press the pane scrolls by as much as the control moved, adding
    /// blank room after the content only while that needs it. The DM's own wheel or drag, or
    /// `from_top`, ends the hold.
    pub fn holds_presses(mut self) -> Self {
        self.holds = true;
        self
    }
}

/// A held press, kept by the pane (one owner: the pane's id).
#[derive(Clone, Copy, Debug, Default)]
struct Held {
    keep: Option<Keep>,
    /// Blank room after the content, for the hold.
    room: f32,
}

/// The pressed control, its top at the press, the pass of the press, its top now, and the
/// content's height at the press.
#[derive(Clone, Copy, Debug)]
struct Keep {
    control: Id,
    top: f32,
    pass: u64,
    now: Option<f32>,
    content: Option<f32>,
    /// The control pressed: keyboard focus on any other control ends the hold, so the hold
    /// never takes back a scroll that brought the focused control into view.
    pressed: Id,
    /// The room's height at the press: a footer that comes or goes ends the hold.
    room: Option<f32>,
}

/// How many passes after a press the pane still moves its scroll to hold the control.
const KEEP_PASSES: u64 = 6;

fn holding_id() -> Id {
    Id::new("design_pane_holding")
}

/// The held press of the pane whose body is being drawn, if that pane holds presses.
fn with_held(ui: &Ui, change: impl FnOnce(&mut Held, u64)) {
    let pass = ui.ctx().cumulative_pass_nr();
    let Some(pane) = ui
        .ctx()
        .data(|data| data.get_temp::<Option<Id>>(holding_id()))
        .flatten()
    else {
        return;
    };
    ui.ctx()
        .data_mut(|data| change(data.get_temp_mut_or_default::<Held>(pane), pass));
}

/// The body pressed `pressed`, the control kept as `control` (any stable key of it, the same
/// that `held_seen` gives): hold it in place.
pub fn hold_press(ui: &Ui, control: impl std::hash::Hash, pressed: &egui::Response) {
    let control = Id::new(control);
    let (top, pressed) = (pressed.rect.top(), pressed.id);
    with_held(ui, |held, pass| {
        held.keep = Some(Keep {
            control,
            top,
            pass,
            now: None,
            content: None,
            pressed,
            room: None,
        });
    });
}

/// The body drew `control` with its top at `top` in this pass.
pub fn held_seen(ui: &Ui, control: impl std::hash::Hash, top: f32) {
    let control = Id::new(control);
    with_held(ui, |held, _| {
        if let Some(keep) = held.keep.as_mut().filter(|keep| keep.control == control) {
            keep.now = Some(top);
        }
    });
}

/// Where a pane's footer stands.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Placement {
    /// `S` below the content, which is shorter than the room and not scrolled.
    Follows,
    /// At the pane's bottom: the content is as tall as the room or scrolled, or the body is
    /// `Body::Fills`.
    Pinned,
}

/// What kind of pane is open now, innermost last, per layer.
#[derive(Clone, Default)]
struct Open {
    pass: u64,
    panes: Vec<(LayerId, bool)>,
}

fn open_id() -> Id {
    Id::new("design_open_panes")
}

/// Open (`Some(content)`) or close (`None`) the innermost pane on `ui`'s layer.
fn open(ui: &Ui, content: Option<bool>) {
    let (layer, pass) = (ui.layer_id(), ui.ctx().cumulative_pass_nr());
    ui.ctx().data_mut(|data| {
        let open = data.get_temp_mut_or_default::<Open>(open_id());
        if open.pass != pass {
            *open = Open {
                pass,
                panes: Vec::new(),
            };
        }
        match content {
            Some(content) => open.panes.push((layer, content)),
            None => {
                open.panes.pop();
            }
        }
    });
}

/// Whether a vertical `ScrollColumn` about to be drawn on `ui` is inside a `Body::Content` pane,
/// which is a scroll column already. Debug builds and tests refuse it (a panic, so that its caller
/// is fixed); a release build draws its content in place (`ScrollColumn::inline`), and the pane
/// scrolls it.
pub(super) fn column_in_content(ui: &Ui) -> bool {
    let (layer, pass) = (ui.layer_id(), ui.ctx().cumulative_pass_nr());
    let inside = ui.ctx().data(|data| {
        data.get_temp::<Open>(open_id())
            .is_some_and(|open| open.pass == pass && open.panes.last() == Some(&(layer, true)))
    });
    assert!(
        !(inside && cfg!(debug_assertions)),
        "a ScrollColumn inside a `Body::Content` pane: the pane is the scroll column. Give its \
         content to the pane, draw a header that must not scroll before the pane, or use \
         `Body::Fills(Fills::because(FillsReason::Columns))` for columns that each scroll"
    );
    inside
}

/// Each pane identity drawn in this pass. Two live panes with one identity would share their
/// footer's height, scroll and focus; debug builds and tests refuse the second.
fn claim(ui: &Ui, id: Id) {
    let pass = ui.ctx().cumulative_pass_nr();
    let fresh = ui.ctx().data_mut(|data| {
        let claimed = data.get_temp_mut_or_default::<(u64, std::collections::HashSet<Id>)>(
            Id::new("design_pane_identities"),
        );
        if claimed.0 != pass {
            *claimed = (pass, Default::default());
        }
        claimed.1.insert(id)
    });
    assert!(
        fresh || !cfg!(debug_assertions),
        "two panes with one identity in one `Ui`: give each its own footer id or `push_id` scope"
    );
}

fn widget_count(ui: &Ui) -> usize {
    let layer = ui.layer_id();
    ui.ctx()
        .viewport(|viewport| viewport.this_pass.widgets.get_layer(layer).count())
}

/// A body control that gained keyboard focus in this pass comes whole into the pane's view,
/// above the footer: Tab never reaches a control hidden under it. So does the focused control
/// when the visible room changes under it (the footer came or grew, the session banner came, the
/// whole-row cut came with it; review of PR 306): a key press then never reaches a hidden control. Only
/// a control that the body added (`widgets`, on the pane's layer) is scrolled to; a change of
/// focus with a pointer press, hold or release is not, as `scroll_part` does for the footer.
fn reveal_focus(ui: &mut Ui, id: Id, widgets: std::ops::Range<usize>, room: Rect, visible: Rect) {
    let focused = ui.ctx().memory(|memory| memory.focused());
    let seen = id.with("seen_focus");
    // The room, not the clip: a wheel scroll moves the whole-row cut, and must not pull the
    // focused control back into view.
    let before = ui
        .ctx()
        .data(|data| data.get_temp::<(Option<Id>, Rect)>(seen));
    if before == Some((focused, room)) {
        return;
    }
    ui.ctx()
        .data_mut(|data| data.insert_temp(seen, (focused, room)));
    // The same control, a new room: scroll only when it is not whole in view.
    let moved_room = before.is_some_and(|(was, _)| was == focused);
    let pointer = ui.input(|input| {
        let pointer = &input.pointer;
        pointer.any_down() || pointer.any_pressed() || pointer.any_released()
    });
    let layer = ui.layer_id();
    let rect = focused.filter(|_| !pointer).and_then(|focused| {
        ui.ctx().viewport(|viewport| {
            let widgets_now = &viewport.this_pass.widgets;
            let (at_layer, at) = widgets_now.order(focused)?;
            (at_layer == layer && widgets.contains(&at))
                .then(|| widgets_now.get(focused).map(|widget| widget.rect))
                .flatten()
        })
    });
    if let Some(rect) = rect
        && !(moved_room && visible.contains_rect(rect))
    {
        ui.scroll_to_rect(rect, None);
    }
}

/// A press that changes the content's height above its control: in the passes after it the
/// column scrolls by as much as the control moved, so it stays under the pointer and a second
/// press of a still pointer presses it again. It acts only while the content's height differs
/// from the press's; the DM's own wheel or drag ends it. The blank room after the content is
/// only what the scroll needs to hold it there.
fn hold_place<R>(ui: &Ui, mut held: Held, output: &egui::scroll_area::ScrollAreaOutput<R>) -> Held {
    let mut state = output.state;
    let pass = ui.ctx().cumulative_pass_nr();
    let content = output.content_size.y - held.room;
    let scrolled = ui.input(|i| {
        i.raw_scroll_delta != egui::Vec2::ZERO
            || i.smooth_scroll_delta != egui::Vec2::ZERO
            || i.pointer.is_decidedly_dragging()
    });
    let focus = ui.ctx().memory(|memory| memory.focused());
    let room = output.inner_rect.height();
    let ended = held.keep.as_mut().and_then(|keep| {
        let room_changed = (*keep.room.get_or_insert(room) - room).abs() > 0.5;
        if scrolled {
            Some("scrolled")
        } else if pass > keep.pass + KEEP_PASSES {
            Some("passes")
        } else if focus.is_some_and(|focus| focus != keep.pressed) {
            Some("focus moved")
        } else if room_changed {
            Some("room changed")
        } else {
            None
        }
    });
    if let Some(why) = ended {
        held.keep = None;
        #[cfg(test)]
        ui.ctx().data_mut(|data| {
            data.insert_temp(Id::new("design_pane_hold_ended"), why);
            data.get_temp_mut_or_default::<Vec<(&'static str, u64, Option<Id>)>>(Id::new(
                "design_pane_hold_ends",
            ))
            .push((why, pass, focus));
        });
        #[cfg(not(test))]
        let _ = why;
    }
    if let Some(keep) = held.keep.as_mut()
        && (*keep.content.get_or_insert(content) - content).abs() > 0.5
        && let Some(now) = keep.now.take()
        && (now - keep.top).abs() > 0.5
    {
        let offset = (state.offset.y + now - keep.top).max(0.0);
        let end = output.content_size.y - output.inner_rect.height();
        held.room += (offset - end).max(0.0);
        state.offset.y = offset;
        state.store(ui.ctx(), output.id);
        ui.ctx()
            .request_discard("a press moved the content above its control");
        return held;
    }
    held.room = (state.offset.y + output.inner_rect.height() - content).max(0.0);
    held
}

/// The scroll facts of a `Body::Content` pane: its viewport, its content's height, its offset.
#[derive(Clone, Copy, Debug)]
struct Scroll {
    viewport: Rect,
    height: f32,
    offset: f32,
}

impl Scroll {
    /// The content is shorter than the room and not scrolled.
    fn fits(self) -> bool {
        self.height < self.viewport.height() - 0.5 && self.offset <= 0.5
    }

    /// Content goes on below the room.
    fn cut(self) -> bool {
        self.viewport.top() + self.height - self.offset > self.viewport.bottom() + 0.5
    }
}

/// What a pane did in one pass, for the gate (package G): its room, its body kind, the scroll
/// output of a `Content` body (viewport, content height, offset), its footer's rect when it held
/// a message, where the footer stands, and whether it cut content (then it draws its hairline).
#[derive(Clone, Copy, Debug)]
pub struct PaneRecord {
    id: Id,
    local: Id,
    room: Rect,
    body: Body,
    viewport: Option<Rect>,
    height: Option<f32>,
    offset: f32,
    footer: Option<Rect>,
    placement: Placement,
    cut: bool,
}

impl PaneRecord {
    /// The footer's id, scoped to the pane's parent `Ui` (its focus group is `id().with("focus")`).
    pub fn id(&self) -> Id {
        self.id
    }

    /// The id the caller gave `Footer::new`, before the pane scoped it.
    pub fn local(&self) -> Id {
        self.local
    }

    /// The pane's room: from the body's top to the pane's bottom.
    pub fn room(&self) -> Rect {
        self.room
    }

    pub fn body(&self) -> Body {
        self.body
    }

    /// The scroll column's viewport, for a `Content` body.
    pub fn viewport(&self) -> Option<Rect> {
        self.viewport
    }

    /// The content's height as egui laid it out, for a `Content` body.
    pub fn content_height(&self) -> Option<f32> {
        self.height
    }

    /// How far the content is scrolled.
    pub fn offset(&self) -> f32 {
        self.offset
    }

    /// Whether the scroll output says the content fits: a `Content` body shorter than its room
    /// and not scrolled.
    pub fn fits(&self) -> bool {
        match (self.viewport, self.height) {
            (Some(viewport), Some(height)) => Scroll {
                viewport,
                height,
                offset: self.offset,
            }
            .fits(),
            _ => false,
        }
    }

    /// The footer's rect, when it held a message.
    pub fn footer(&self) -> Option<Rect> {
        self.footer
    }

    /// Whether the footer follows the content or is pinned. Set also while it is empty.
    pub fn placement(&self) -> Placement {
        self.placement
    }

    /// Whether the footer cut content: it then draws its hairline.
    pub fn cut(&self) -> bool {
        self.cut
    }
}

/// Where test builds keep each pane's record of a pass.
#[cfg(test)]
const PANE_RECORDS: &str = "design_pane_records";

#[cfg(test)]
fn keep(ui: &Ui, pane: PaneRecord) {
    let pass = ui.ctx().cumulative_pass_nr();
    ui.ctx().data_mut(|data| {
        let kept = data.get_temp_mut_or_default::<(u64, Vec<PaneRecord>)>(Id::new(PANE_RECORDS));
        if kept.0 != pass {
            *kept = (pass, Vec::new());
        }
        kept.1.push(pane);
    });
}

/// Each pane of the last completed pass, in the order they finished (an inner pane before the
/// pane that holds it), keyed by the footer's id; empty when that pass drew none (a record
/// never outlives its pane). Only test builds keep them, beside `footer_bands`. Package G's
/// check "a footer below a short body" (G6) reads them through `Driver::pane_records`, and
/// checks them against its own accessibility-tree rects, not against the record's placement:
/// for a pane that is not `Body::Fills`, whose footer shows and whose scroll output `fits()`, a
/// gap of more than `2 × L` from the last body widget to the footer is a finding.
#[cfg(test)]
pub fn pane_records(ctx: &egui::Context) -> Vec<PaneRecord> {
    let last = ctx.cumulative_pass_nr().saturating_sub(1);
    ctx.data(|data| data.get_temp::<(u64, Vec<PaneRecord>)>(Id::new(PANE_RECORDS)))
        .filter(|(pass, _)| *pass == last)
        .map(|(_, panes)| panes)
        .unwrap_or_default()
}

/// The record of the pane whose footer the caller made with `Footer::new(local, …)`, in the last
/// completed pass (the first, when several scopes draw one).
#[cfg(test)]
pub fn pane_record(ctx: &egui::Context, local: Id) -> Option<PaneRecord> {
    pane_records(ctx)
        .into_iter()
        .find(|record| record.local == local)
}

/// G6 from rects the gate reads itself (the accessibility tree), not from the record's placement.
/// For a pane that is not `Body::Fills`, whose scroll output says its content fits, the gap from
/// the last body widget above `footer` (its rect, scaled like `body`) to the footer's top; `Some`
/// when it is more than `2 × L` (row padding and a frame under the last item end there). `scale`
/// turns the record's points into the rects' units.
#[cfg(test)]
pub fn short_body_gap(
    record: &PaneRecord,
    scale: f32,
    footer: Rect,
    body: impl IntoIterator<Item = Rect>,
) -> Option<f32> {
    if matches!(record.body, Body::Fills(_)) || !record.fits() {
        return None;
    }
    let room = record.room * scale;
    let end = body
        .into_iter()
        .filter(|rect| {
            rect.is_positive()
                && rect.top() >= room.top() - 0.5
                && rect.bottom() <= footer.top() + 0.5
                && rect.right() > room.left()
                && rect.left() < room.right()
        })
        .map(|rect| rect.bottom())
        .fold(room.top(), f32::max);
    let gap = footer.top() - end;
    (gap > 2.0 * super::tokens::L * scale).then_some(gap)
}

/// The rect of the footer that the caller made with `Footer::new(local, …)`, as egui laid out its
/// focus group in the last pass (the first, when several scopes draw one).
#[cfg(test)]
pub fn footer_rect(ctx: &egui::Context, local: Id) -> Option<Rect> {
    let id = pane_record(ctx, local)?.id;
    ctx.read_response(id.with("focus"))
        .map(|response| response.rect)
}

/// The body's clip: what is left of `ui`'s clip above the body's room, and above `row_cut`, the
/// top of the registered row that the footer or the session banner would cut (N12, ruling R4).
/// The one place that decides where the body is cut. egui clips paint and presses alike, so a
/// row under the cut neither shows nor takes a press.
fn body_clip(ui: &Ui, body_room: Rect, row_cut: Option<f32>) -> Rect {
    let mut clip = ui.clip_rect();
    clip.max.y = clip.max.y.min(body_room.bottom());
    if let Some(cut) = row_cut {
        clip.max.y = clip.max.y.min(cut);
    }
    clip
}

/// The top of the lowest registered row that the viewport's bottom edge cuts (its top above the
/// edge, its bottom below), unless that row starts at the viewport's top (one row taller than the
/// room keeps the plain cut). Prose and notes are not rows: they keep a line cut.
fn whole_row_cut(rows: &[Rect], viewport: Rect) -> Option<f32> {
    let edge = viewport.bottom();
    rows.iter()
        .filter(|row| {
            row.top() < edge - 0.5 && row.bottom() > edge + 0.5 && row.top() > viewport.top() + 0.5
        })
        .map(|row| row.top())
        .reduce(f32::min)
}

fn banner_id() -> Id {
    Id::new("splat_bottom_banner")
}

/// A banner at the window's bottom edge shows in this pass (the session banner): a pane above it
/// cuts its body at whole rows, as a footer does. Call it where the banner is drawn, before the
/// pages.
pub fn note_bottom_banner(ctx: &egui::Context) {
    let pass = ctx.cumulative_pass_nr();
    ctx.data_mut(|data| data.insert_temp(banner_id(), pass));
}

fn banner_below(ctx: &egui::Context) -> bool {
    let pass = ctx.cumulative_pass_nr();
    ctx.data(|data| data.get_temp::<u64>(banner_id())) == Some(pass)
}

impl Footer<'_> {
    /// The pane: its body, drawn by `add` from the cursor down (for `Body::Content`, as the
    /// pane's own scroll column), then the footer when it holds a message: `S` below the
    /// content when it fits, or pinned at the bottom of what is left of `ui`. The body's top is
    /// the same with or without messages; only its room's bottom comes up by the footer's
    /// height (and `S`) while the footer holds a message. The pane takes the rest of `ui`. `ui`
    /// lays out top down. Returns what `add` returns.
    pub fn pane<R>(self, ui: &mut Ui, body: Body, add: impl FnOnce(&mut Ui) -> R) -> R {
        self.place(ui, body, ContentScroll::default(), add)
    }

    /// `pane` with `Body::Content`, whose scroll column behaves as `scroll` says.
    pub fn pane_content<R>(
        self,
        ui: &mut Ui,
        scroll: ContentScroll,
        add: impl FnOnce(&mut Ui) -> R,
    ) -> R {
        self.place(ui, Body::Content, scroll, add)
    }

    fn place<R>(
        mut self,
        ui: &mut Ui,
        body: Body,
        options: ContentScroll,
        add: impl FnOnce(&mut Ui) -> R,
    ) -> R {
        debug_assert!(
            ui.layout().main_dir() == egui::Direction::TopDown,
            "a pane's message footer ends a top-down Ui"
        );
        let room = ui.available_rect_before_wrap();
        // The pane's identity is scoped to its parent `Ui`, so one pane function drawn in two
        // `push_id` scopes keeps two footers apart; two in one scope are refused.
        let local = self.rescope(ui.id());
        let (id, width) = self.id_and_width(room.width());
        claim(ui, id);
        let key = id.with("height");
        let stored = ui
            .ctx()
            .data(|data| data.get_temp::<f32>(key))
            .unwrap_or(0.0);
        let shown = !self.is_empty();
        if !shown && stored != 0.0 {
            ui.ctx().data_mut(|data| data.remove::<f32>(key));
            ui.ctx().request_discard("the message footer emptied");
        }
        let most = (room.height() / 3.0).floor().max(0.0);
        // Last pass's height; a change is measured below, and the pass is drawn again.
        let height = if shown { stored.min(most) } else { 0.0 };
        let pinned = room.bottom() - height;
        // The body has the same room whether the footer follows it or is pinned, so it never
        // changes its own size when the footer moves.
        let limit = if height > 0.0 {
            pinned - S
        } else {
            room.bottom()
        };
        let body_room = Rect::from_min_max(room.min, pos2(room.right(), limit.max(room.top())));

        // The body, in the part's own `Ui`; one id in every state, so its scroll and focus stay.
        let mut body_ui = ui.new_child(
            egui::UiBuilder::new()
                .id_salt(id.with("body"))
                .max_rect(body_room)
                .layout(*ui.layout()),
        );
        // Last pass's whole-row cut, while the room is the same; a new cut draws the pass again.
        let cut_key = id.with("row_cut");
        let row_cut = ui
            .ctx()
            .data(|data| data.get_temp::<(Rect, f32)>(cut_key))
            .filter(|(kept, _)| *kept == body_room)
            .map(|(_, cut)| cut);
        let visible = body_clip(ui, body_room, row_cut);
        body_ui.set_clip_rect(visible);
        let rows_before = super::components::rows_this_pass(ui.ctx()).len();
        let (inner, scroll) = match body {
            Body::Content => {
                // The viewport is the real room, however small: egui's own 64 px minimum would
                // leave the last rows of a small room out of reach under the clip.
                let room_height = body_room.height();
                let held_key = id.with("held");
                let mut held = ui
                    .ctx()
                    .data(|data| data.get_temp::<Held>(held_key))
                    .unwrap_or_default();
                if !options.holds || options.from_top {
                    held = Held::default();
                }
                let mut column = ScrollColumn::new(id.with("column"))
                    .max_height(room_height)
                    .min_scrolled_height(room_height);
                if options.from_top {
                    column = column.vertical_scroll_offset(0.0);
                }
                if options.holds {
                    ui.ctx().data_mut(|data| {
                        data.insert_temp(held_key, held);
                        data.insert_temp(holding_id(), Some(held_key));
                    });
                }
                let output = column.show(&mut body_ui, |ui| {
                    open(ui, Some(true));
                    let first = widget_count(ui);
                    let inner = add(ui);
                    let last = widget_count(ui);
                    open(ui, None);
                    reveal_focus(ui, id, first..last, body_room, visible);
                    if options.holds {
                        super::extras::space(ui, held.room);
                    }
                    inner
                });
                let room = if options.holds {
                    ui.ctx()
                        .data_mut(|data| data.insert_temp(holding_id(), None::<Id>));
                    let held = ui
                        .ctx()
                        .data(|data| data.get_temp::<Held>(held_key))
                        .unwrap_or_default();
                    let room = held.room;
                    let held = hold_place(ui, held, &output);
                    ui.ctx().data_mut(|data| data.insert_temp(held_key, held));
                    room
                } else {
                    0.0
                };
                let scroll = Scroll {
                    viewport: output.inner_rect,
                    height: output.content_size.y - room,
                    offset: output.state.offset.y,
                };
                (output.inner, Some(scroll))
            }
            Body::Fills(_) => {
                open(&body_ui, Some(false));
                let inner = add(&mut body_ui);
                open(&body_ui, None);
                (inner, None)
            }
        };
        drop(body_ui);
        // N12: an edge (the footer, the session banner) never cuts through a registered row.
        let edge = shown || banner_below(ui.ctx());
        let new_cut = match scroll {
            Some(scroll) if edge && scroll.cut() => {
                let rows = super::components::rows_this_pass(ui.ctx());
                whole_row_cut(rows.get(rows_before..).unwrap_or_default(), scroll.viewport)
            }
            _ => None,
        };
        if new_cut != row_cut {
            ui.ctx().data_mut(|data| match new_cut {
                Some(cut) => data.insert_temp(cut_key, (body_room, cut)),
                None => data.remove::<(Rect, f32)>(cut_key),
            });
            ui.ctx().request_discard("the body's whole-row cut moved");
        }
        if let Some(cut) = new_cut {
            // The hairline marks where the rows stop.
            ui.painter().hline(
                room.left()..=room.left() + width,
                cut - super::tokens::HAIRLINE_WIDTH / 2.0,
                egui::Stroke::new(
                    super::tokens::HAIRLINE_WIDTH,
                    super::tokens::colors().hairline,
                ),
            );
        }

        let placement = match scroll {
            Some(scroll) if scroll.fits() => Placement::Follows,
            _ => Placement::Pinned,
        };
        // Content goes on below the room: the footer cuts it, and draws its hairline.
        let cut = shown && scroll.is_some_and(Scroll::cut);
        let mut footer = None;
        if shown {
            let top = match (placement, scroll) {
                (Placement::Follows, Some(scroll)) => {
                    (scroll.viewport.top() + scroll.height + S).min(pinned)
                }
                _ => pinned,
            };
            let rect = Rect::from_min_size(pos2(room.left(), top), vec2(width, height));
            // Nothing in the body under the footer takes a press there, nor in the `S` above it
            // (egui gives a press to a widget a few points away): the footer comes after the body,
            // so this blank target is above the body's, and the footer's own controls above it.
            let _ = ui.interact(
                Rect::from_min_max(pos2(rect.left(), rect.top() - S), rect.max),
                id.with("blocks_body"),
                Sense::CLICK,
            );
            // A whole-row cut has its own hairline; the footer draws one only for a line cut.
            // The kept lines take their control ids from this pane's footer and their ticket.
            ui.ctx()
                .data_mut(|data| data.insert_temp(Id::new("design_footer_drawing"), id));
            self.draw_messages(ui, rect, cut && new_cut.is_none(), most, stored);
            if height > 0.0 {
                footer = Some(rect);
                #[cfg(test)]
                super::components::record_band(ui, room, rect);
            } else {
                // Not measured yet: this pass is drawn again with the footer's height.
                ui.ctx()
                    .request_discard("the message footer measures its height");
            }
        }
        ui.advance_cursor_after_rect(room);
        #[cfg(test)]
        keep(
            ui,
            PaneRecord {
                id,
                local,
                room,
                body,
                viewport: scroll.map(|scroll| scroll.viewport),
                height: scroll.map(|scroll| scroll.height),
                offset: scroll.map_or(0.0, |scroll| scroll.offset),
                footer,
                placement,
                cut,
            },
        );
        #[cfg(not(test))]
        let _ = (footer, placement, local);
        inner
    }
}

#[cfg(test)]
mod hold_tests {
    use super::*;
    use egui_kittest::{Harness, kittest::Queryable};

    #[derive(Default)]
    struct State {
        holds: bool,
        rows_above: usize,
        message: bool,
    }

    /// A list with rows above a button and many below, then a control far below; the button's
    /// press adds rows above it. With `message`, the footer holds a reason.
    fn list(ui: &mut Ui, state: &mut State) {
        use crate::design::outcome::{Messages, NoPress, Outcome, PaneFrame};
        let mut scroll = ContentScroll::default();
        if state.holds {
            scroll = scroll.holds_presses();
        }
        let frame = PaneFrame::new(ui);
        let mut messages = Messages::<NoPress>::new(&frame, "held", ui.available_width());
        if state.message {
            messages.outcome(Outcome::Refused(crate::sentence!("Save the draft first.")));
        }
        let _ = messages.pane_content(ui, scroll, |ui| {
            for row in 0..state.rows_above {
                ui.label(format!("Row above {row}"));
            }
            let response = ui.button("Add a row above");
            held_seen(ui, "add", response.rect.top());
            if response.clicked() {
                hold_press(ui, "add", &response);
                state.rows_above += 3;
            }
            for row in 0..60 {
                ui.label(format!("Row below {row}"));
            }
            let _ = ui.button("Far control");
        });
    }

    fn ended(h: &Harness<'_, State>) -> Option<&'static str> {
        h.ctx
            .data(|data| data.get_temp::<&'static str>(Id::new("design_pane_hold_ended")))
    }

    /// A hold ends when keyboard focus moves (the focused control then comes whole into view
    /// above the footer, and the hold does not take that scroll back) and when a footer comes
    /// (the room changed).
    #[test]
    fn a_focus_move_or_a_new_footer_ends_the_hold() {
        // Focus moves to a control far below, with a footer present.
        let mut h = Harness::builder()
            .with_size(egui::vec2(400.0, 300.0))
            .build_ui_state(
                list,
                State {
                    holds: true,
                    rows_above: 2,
                    message: true,
                },
            );
        crate::design::theme::install(&h.ctx);
        h.run_steps(4);
        h.get_by_label("Add a row above").click();
        h.get_by_label("Far control").focus();
        h.run_steps(8);
        let ends = h.ctx.data(|data| {
            data.get_temp::<Vec<(&'static str, u64, Option<Id>)>>(Id::new("design_pane_hold_ends"))
        });
        let focused = h.ctx.memory(|memory| memory.focused());
        assert_eq!(
            ended(&h),
            Some("focus moved"),
            "ends {ends:?}, focused now {focused:?}, rows {}",
            h.state().rows_above
        );
        let far = h.get_by_label("Far control").rect();
        let record = pane_records(&h.ctx).into_iter().next().unwrap();
        let (viewport, footer) = (record.viewport().unwrap(), record.footer().unwrap());
        assert!(
            far.top() >= viewport.top() - 0.5 && far.bottom() <= footer.top() + 0.5,
            "the focused control is whole above the footer: {far:?}, footer {footer:?}"
        );
        // A footer that comes after the press.
        let mut h = Harness::builder()
            .with_size(egui::vec2(400.0, 300.0))
            .build_ui_state(
                list,
                State {
                    holds: true,
                    rows_above: 2,
                    message: false,
                },
            );
        crate::design::theme::install(&h.ctx);
        h.run_steps(4);
        h.get_by_label("Add a row above").click();
        h.run_steps(1);
        h.state_mut().message = true;
        h.run_steps(8);
        assert_eq!(ended(&h), Some("room changed"));
    }

    /// `ContentScroll::holds_presses`: a press that adds rows above its control keeps the
    /// control under the pointer, so a second press of a still pointer presses it again. Without
    /// the option, the control moves down by the rows added.
    #[test]
    fn a_held_press_keeps_its_control_under_the_pointer() {
        for holds in [true, false] {
            let mut h = Harness::builder()
                .with_size(egui::vec2(400.0, 300.0))
                .build_ui_state(
                    list,
                    State {
                        holds,
                        rows_above: 2,
                        message: false,
                    },
                );
            crate::design::theme::install(&h.ctx);
            h.run_steps(4);
            let before = h.get_by_label("Add a row above").rect().top();
            h.get_by_label("Add a row above").click();
            h.run_steps(8);
            let after = h.get_by_label("Add a row above").rect().top();
            assert_eq!(h.state().rows_above, 5);
            if holds {
                assert!((after - before).abs() < 0.5, "held: {before} then {after}");
            } else {
                assert!(after - before > 20.0, "not held: {before} then {after}");
            }
        }
    }
}

#[cfg(test)]
mod row_cut_tests {
    use super::*;
    use crate::design::{
        components::{ROW_RECTS, two_line_row},
        outcome::{Messages, NoPress, Outcome, PaneFrame},
    };

    /// N12 (ruling R4): a footer never cuts through a registered row. At every height, each
    /// registered row is whole above the body's clip or wholly under it; the hairline marks the
    /// cut; and with no footer, the rows keep the plain cut of the scroll column. Rung 1 for a
    /// caller (the clip is the pane's; no caller sets one), this test for the part.
    #[test]
    fn a_footer_never_cuts_through_a_registered_row() {
        let mut hidden_by_the_cut = 0;
        for message in [true, false] {
            for height in (300..=360).step_by(7) {
                let mut harness = egui_kittest::Harness::builder()
                    .with_size(vec2(600.0, height as f32))
                    .build_ui(move |ui| {
                        let frame = PaneFrame::new(ui);
                        let mut messages =
                            Messages::<NoPress>::new(&frame, "row_cut_test", ui.available_width());
                        if message {
                            messages.outcome(Outcome::Refused(crate::sentence!(
                                "Save the draft first."
                            )));
                        }
                        let _ = messages.pane(ui, Body::Content, |ui| {
                            for at in 0..30 {
                                let _ = two_line_row(
                                    ui,
                                    44.0,
                                    false,
                                    &format!("Row {at}"),
                                    "Its second line",
                                );
                            }
                        });
                    });
                crate::design::theme::install(&harness.ctx);
                harness.run_steps(4);
                let output = harness.output();
                let rows = harness
                    .ctx
                    .data(|data| data.get_temp::<(u64, Vec<Rect>)>(Id::new(ROW_RECTS)))
                    .map(|(_, rows)| rows)
                    .unwrap_or_default();
                assert!(rows.len() == 30, "{message} {height}: rows {}", rows.len());
                let clip = output
                    .shapes
                    .iter()
                    .find(|clipped| {
                        matches!(&clipped.shape, egui::Shape::Text(text) if text.galley.job.text == "Row 0")
                    })
                    .map(|clipped| clipped.clip_rect.bottom())
                    .expect("Row 0 is painted");
                let record = pane_records(&harness.ctx)
                    .into_iter()
                    .next()
                    .expect("the pane keeps its record");
                let viewport = record.viewport().expect("a Content body");
                // With no footer the column keeps its plain cut (a row may show in part).
                for row in rows.iter().filter(|_| message) {
                    assert!(
                        row.bottom() <= clip + 0.5 || row.top() >= clip - 0.5,
                        "n12-cut: message {message}, height {height}: row {row:?} cut at {clip}"
                    );
                }
                if message {
                    hidden_by_the_cut += rows
                        .iter()
                        .filter(|row| row.top() >= clip - 0.5 && row.top() < viewport.bottom())
                        .count();
                    let hairline = output.shapes.iter().any(|clipped| {
                        matches!(&clipped.shape, egui::Shape::LineSegment { points, .. }
                            if (points[0].y - clip).abs() <= 1.0 && points[0].y == points[1].y)
                    });
                    assert!(
                        hairline || clip >= viewport.bottom() - 0.5,
                        "n12-hairline: height {height}, cut {clip}"
                    );
                } else {
                    assert!(
                        (clip - viewport.bottom()).abs() <= 0.5,
                        "n12-plain: with no footer the column keeps its own cut: {clip} {viewport:?}"
                    );
                }
            }
        }
        assert!(hidden_by_the_cut > 0, "n12-reached: some height cut a row");
    }

    /// Review of PR 306: with keyboard focus on a registered row near the bottom, a footer (or
    /// the session banner) that comes without a change of focus brings that row whole into view
    /// before a key reaches it, and Enter fires exactly its one action. Both edges.
    #[test]
    fn a_new_edge_brings_the_focused_row_whole_into_view() {
        #[derive(Default)]
        struct State {
            message: bool,
            banner: bool,
            focus: Option<usize>,
            pressed: Vec<usize>,
            rows: Vec<Rect>,
            clip: f32,
        }
        for banner in [false, true] {
            let mut harness = egui_kittest::Harness::builder()
                .with_size(vec2(600.0, 420.0))
                .build_state(
                    |ctx, state: &mut State| {
                        if state.banner {
                            note_bottom_banner(ctx);
                            egui::TopBottomPanel::bottom("row_cut_banner").show(ctx, |ui| {
                                for line in 0..3 {
                                    ui.label(format!("Banner line {line}"));
                                }
                            });
                        }
                        egui::CentralPanel::default().show(ctx, |ui| {
                            let frame = PaneFrame::new(ui);
                            let mut messages = Messages::<NoPress>::new(
                                &frame,
                                "row_focus_test",
                                ui.available_width(),
                            );
                            if state.message {
                                messages.outcome(Outcome::Refused(crate::sentence!(
                                    "Save the draft first. The other changes wait until it is saved."
                                )));
                            }
                            let focus = state.focus.take();
                            let (rows, presses, clip) = messages
                                .pane(ui, Body::Content, |ui| {
                                let mut rows = Vec::new();
                                let mut presses = Vec::new();
                                for at in 0..30 {
                                    let row = two_line_row(
                                        ui,
                                        44.0,
                                        false,
                                        &format!("Row {at}"),
                                        "Its second line",
                                    );
                                    if focus == Some(at) {
                                        row.request_focus();
                                    }
                                    if row.clicked() {
                                        presses.push(at);
                                    }
                                    rows.push(row.rect);
                                }
                                (rows, presses, ui.clip_rect().bottom())
                            })
                                .body;
                            state.rows = rows;
                            state.pressed.extend(presses);
                            state.clip = clip;
                        });
                    },
                    State::default(),
                );
            crate::design::theme::install(&harness.ctx);
            harness.run_steps(3);
            // The last row whole in view with no edge: focus it.
            let clip = harness.state().clip;
            let target = harness
                .state()
                .rows
                .iter()
                .rposition(|row| row.bottom() <= clip)
                .expect("a row is whole in view");
            harness.state_mut().focus = Some(target);
            harness.run_steps(2);
            // The edge comes; focus does not change.
            if banner {
                harness.state_mut().banner = true;
            } else {
                harness.state_mut().message = true;
            }
            harness.run_steps(4);
            let (row, clip) = (harness.state().rows[target], harness.state().clip);
            assert!(
                row.bottom() <= clip + 0.5 && row.top() >= 0.0,
                "n12-focus banner {banner}: row {target} {row:?} not whole above {clip}"
            );
            harness.key_press(egui::Key::Enter);
            harness.run_steps(2);
            assert_eq!(
                harness.state().pressed,
                vec![target],
                "n12-enter banner {banner}: exactly the focused row's action"
            );
        }
    }
}
