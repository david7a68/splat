//! A pane: its body, then its one message footer, placed by the part for every caller.
//!
//! `Footer::pane` makes the body's `Ui` itself, runs the caller's body in it, measures where
//! the body's content ends in this pass, and then draws the footer `S` below that end, or
//! pinned at the pane's bottom when the body fills its room or scrolls. A hairline marks the
//! footer's top only when it cuts content. The caller gives the messages and the body; it gets
//! back only what its body returns, never a y, so it cannot size the body by hand, draw the body
//! before the footer, or wrap it in a `Ui` the footer cannot see (round 28, `footer-escape.md`:
//! causes W, P and L).
//!
//! How the part measures the content's end, with no container rects:
//!
//! - Each `ScrollColumn` drawn in the body records, under the innermost open pane, its viewport
//!   and its content's end (the viewport's top plus the content's height, less the scroll
//!   offset). A column that scrolls ends at its viewport's bottom.
//! - A pane drawn in the body (a nested pane) records its room and its own end the same way.
//! - Every other widget the body added in this pass counts with its bottom, except one inside a
//!   recorded column or nested pane (that one's end stands for it) and one that holds the body's
//!   whole room (the pane's own press area). A widget that holds a recorded viewport or room is a
//!   wrapper (a scope, a `Frame`, a panel, a `Column`, a full-room allocation around a column): it
//!   ends at its bottom less the room its parts left empty, so a frame's margin counts and the
//!   empty room does not. The body's own `Ui` counts as one more such widget. A widget as tall as
//!   the body's room that holds no part (a splitter, an editor as tall as its room) counts only
//!   through the body's own `Ui`.
//!
//! Proof once, in the part (`components::tests::iter27_a_the_footer_follows_a_short_body` and
//! `iter28_p_the_pane_places_its_footer_for_every_body`): placement for every body shape of the
//! investigation, short and long, and the press proof (body press and footer press while the
//! footer follows, is pinned, grows and empties). A caller that uses `pane` with no custom
//! layer, clip, `Area` or raw interaction does not repeat it. Duties left with the caller: none
//! for placement; a scroll area in the body must be a `ScrollColumn` (Clippy refuses a raw one).
//!
//! The caller gets no y:
//!
//! ```compile_fail,E0308
//! use splat::pane::{Body, Footer};
//! fn body_end(ui: &mut egui::Ui) {
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
//! fn rows(ui: &mut egui::Ui) -> usize {
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
//! fn twice(ui: &mut egui::Ui) {
//!     let footer = Footer::new("pane", 400.0);
//!     footer.pane(ui, Body::Content, |ui| ui.label("Body"));
//!     footer.pane(ui, Body::Content, |ui| ui.label("A second body"));
//! }
//! ```
//!
//! ```
//! use splat::pane::{Body, Footer};
//! fn once(ui: &mut egui::Ui) {
//!     Footer::new("pane", 400.0).pane(ui, Body::Content, |ui| ui.label("Body"));
//! }
//! ```
//!
//! A body that fills its room names why, from a closed list; no free words, no `bool`:
//!
//! ```compile_fail,E0308
//! use splat::pane::{Body, Footer};
//! fn editor(ui: &mut egui::Ui) {
//!     Footer::new("pane", 400.0).pane(ui, Body::Fills("an editor"), |_| ());
//! }
//! ```
//!
//! ```
//! use splat::pane::{Body, FillsReason, Footer};
//! fn editor(ui: &mut egui::Ui) {
//!     Footer::new("pane", 400.0).pane(ui, Body::Fills(FillsReason::MarkdownEditor), |_| ());
//! }
//! ```
//!
//! A caller cannot make up a column's end or a pane's record: the registry has no public
//! constructor.
//!
//! ```compile_fail,E0451
//! let _ = splat::pane::PaneRecord { content_end: 0.0 };
//! ```
//!
//! ```
//! fn ends(records: &[splat::pane::PaneRecord]) -> Vec<f32> {
//!     records.iter().map(|record| record.content_end()).collect()
//! }
//! ```

pub use super::components::Footer;
use super::tokens::S;
use eframe::egui::{self, Id, LayerId, Rect, Sense, Ui, pos2, vec2};

/// What a pane's body is, for the footer's place.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Body {
    /// Rows, forms, readers, lists and panels: the footer follows the content `S` below its end,
    /// and is pinned when the content reaches the pane's bottom or scrolls. The default.
    Content,
    /// A body that fills its room on purpose: the footer is pinned. The one named escape;
    /// the source lint counts each use (`fills-escape`), shrink only.
    Fills(FillsReason),
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
}

/// A column or a nested pane recorded under the innermost open pane: the range of widgets it
/// added on the pane's layer, its viewport or room, where its content ends, and whether content
/// goes on past that end (a column that scrolls, a nested pane whose body is cut with no footer
/// of its own).
#[derive(Clone, Debug)]
struct Part {
    widgets: std::ops::Range<usize>,
    room: Rect,
    end: f32,
    cuts: bool,
}

/// The panes open in this pass, innermost last, each with what it holds.
#[derive(Clone, Default)]
struct Open {
    pass: u64,
    panes: Vec<(LayerId, Vec<Part>)>,
}

fn open_id() -> Id {
    Id::new("design_open_panes")
}

fn widget_count(ui: &Ui) -> usize {
    let layer = ui.layer_id();
    ui.ctx()
        .viewport(|viewport| viewport.this_pass.widgets.get_layer(layer).count())
}

/// Record a part under the innermost open pane on `ui`'s layer, if there is one.
fn record(ui: &Ui, part: Part) {
    let (layer, pass) = (ui.layer_id(), ui.ctx().cumulative_pass_nr());
    ui.ctx().data_mut(|data| {
        let open = data.get_temp_mut_or_default::<Open>(open_id());
        if open.pass == pass
            && let Some((pane_layer, parts)) = open.panes.last_mut()
            && *pane_layer == layer
        {
            parts.push(part);
        }
    });
}

/// Where a `ScrollColumn` drawn now begins its widgets.
pub(super) fn column_start(ui: &Ui) -> usize {
    widget_count(ui)
}

/// A `ScrollColumn` that began its widgets at `start` has drawn: record its viewport and its
/// content's end under the innermost open pane. A column that scrolls ends at its viewport's
/// bottom.
pub(super) fn column_drawn<R>(
    ui: &Ui,
    start: usize,
    output: &egui::scroll_area::ScrollAreaOutput<R>,
) {
    let room = output.inner_rect;
    let offset = output.state.offset.y;
    let scrolls = offset > 0.5 || output.content_size.y > room.height() + 0.5;
    let end = if scrolls {
        room.bottom()
    } else {
        room.top() + output.content_size.y - offset
    };
    record(
        ui,
        Part {
            widgets: start..widget_count(ui),
            room,
            end,
            cuts: scrolls,
        },
    );
}

/// What a pane did in one pass, for the gate (package G): its room, where its body's content
/// ended, its footer's rect when it held a message, its body kind, and whether the footer cut
/// the content (then it is pinned with its hairline).
#[derive(Clone, Copy, Debug)]
pub struct PaneRecord {
    id: Id,
    room: Rect,
    content_end: f32,
    footer: Option<Rect>,
    placement: Placement,
    body: Body,
    cut: bool,
}

/// Where a pane's footer stands.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Placement {
    /// `S` below the body's content, which ends above the pane's bottom.
    Follows,
    /// At the pane's bottom: the body is `Body::Fills`, or its content reaches the body's room
    /// (it scrolls or runs past it).
    Pinned,
}

impl PaneRecord {
    /// The footer's id.
    pub fn id(&self) -> Id {
        self.id
    }

    /// The pane's room: from the body's top to the pane's bottom.
    pub fn room(&self) -> Rect {
        self.room
    }

    /// Where the body's content ended in this pass (the body's top when it showed nothing).
    pub fn content_end(&self) -> f32 {
        self.content_end
    }

    /// The footer's rect, when it held a message.
    pub fn footer(&self) -> Option<Rect> {
        self.footer
    }

    /// The footer's top, when it held a message.
    pub fn footer_top(&self) -> Option<f32> {
        self.footer.map(|footer| footer.top())
    }

    /// Whether the footer follows a short body or is pinned. Set also while it is empty.
    pub fn placement(&self) -> Placement {
        self.placement
    }

    pub fn body(&self) -> Body {
        self.body
    }

    /// Whether the footer cut content: it is then pinned, with its hairline.
    pub fn cut(&self) -> bool {
        self.cut
    }
}

/// Where test builds keep each pane's record of the last pass.
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

/// Each pane of the last pass that drew one, in the order they finished (an inner pane before
/// the pane that holds it), keyed by the footer's id (`id`). Only test builds keep them, beside
/// `footer_bands`. Package G's check "a footer below a short body" (G6) reads them after each
/// state: for each record with `footer_top()` and `placement() == Placement::Follows`, the gap
/// `footer_top − content_end` is at most `S + 1` px (the part makes it `S`); a `Pinned` footer
/// (the body scrolls, runs past its room or is `Body::Fills`) is not checked. A pane drawn with
/// `Footer::show_by_hand` has no record, so G6 counts its states as by hand until Wave 2 moves it.
#[cfg(test)]
pub fn pane_records(ctx: &egui::Context) -> Vec<PaneRecord> {
    ctx.data(|data| data.get_temp::<(u64, Vec<PaneRecord>)>(Id::new(PANE_RECORDS)))
        .map(|(_, panes)| panes)
        .unwrap_or_default()
}

impl Footer<'_> {
    /// The pane: its body, drawn by `add` in the part's own `Ui` from the cursor down, then the
    /// footer, `S` below the body's content when the footer holds a message, or pinned at the
    /// bottom of what is left of `ui` when the body fills it (`Body::Fills`), reaches it or
    /// scrolls. The body's top is the same with or without messages, and the body never moves:
    /// only its room's bottom comes up by the footer's height (and `S`) while the footer holds
    /// a message. The pane takes the rest of `ui`. `ui` lays out top down. Returns what `add`
    /// returns.
    pub fn pane<R>(mut self, ui: &mut Ui, body: Body, add: impl FnOnce(&mut Ui) -> R) -> R {
        debug_assert!(
            ui.layout().main_dir() == egui::Direction::TopDown,
            "a pane's message footer ends a top-down Ui"
        );
        let room = ui.available_rect_before_wrap();
        let (id, width) = self.id_and_width(room.width());
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
        let start = widget_count(ui);
        let mut body_ui = ui.new_child(
            egui::UiBuilder::new()
                .id_salt(id.with("body"))
                .max_rect(body_room)
                .layout(*ui.layout()),
        );
        body_ui.set_clip_rect(body_clip(ui, body_room));
        let first = widget_count(ui);
        let pass = ui.ctx().cumulative_pass_nr();
        let layer = ui.layer_id();
        ui.ctx().data_mut(|data| {
            let open = data.get_temp_mut_or_default::<Open>(open_id());
            if open.pass != pass {
                *open = Open {
                    pass,
                    panes: Vec::new(),
                };
            }
            open.panes.push((layer, Vec::new()));
        });
        let inner = add(&mut body_ui);
        let taken = body_ui.min_rect();
        drop(body_ui);
        let parts = ui.ctx().data_mut(|data| {
            data.get_temp_mut_or_default::<Open>(open_id())
                .panes
                .pop()
                .map(|(_, parts)| parts)
                .unwrap_or_default()
        });
        let last = widget_count(ui);
        let (end, reaches) = content_end(ui, first..last, &parts, taken, body_room, limit);
        let end = end.unwrap_or(room.top());

        let mut footer = None;
        // Content goes on past the body's clip: the footer cuts it, and draws its hairline.
        let cut = shown && reaches;
        let placement = match body {
            Body::Content if end + S < pinned - 0.5 => Placement::Follows,
            _ => Placement::Pinned,
        };
        if shown {
            let top = match placement {
                Placement::Follows => end + S,
                Placement::Pinned => pinned,
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
            self.draw_messages(ui, rect, cut, most, stored);
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
        // An outer pane takes this one's end: its footer's bottom, or its body's end. Nothing of
        // it goes on below its own footer.
        record(
            ui,
            Part {
                widgets: start..widget_count(ui),
                room,
                end: footer.map_or(end, |footer| footer.bottom()),
                cuts: footer.is_none() && reaches,
            },
        );
        #[cfg(test)]
        keep(
            ui,
            PaneRecord {
                id,
                room,
                content_end: end,
                footer,
                placement,
                body,
                cut,
            },
        );
        inner
    }
}

/// The body's clip: what is left of `ui`'s clip above the body's room. The one place that
/// decides where the body is cut. Seam for round 28's N12 (package X, ruling R4: a footer never
/// cuts through a registered row): X narrows this clip to the last whole row that `ScrollColumn`
/// registers (it records in `column_drawn`), and the pane's hairline then marks that cut; no
/// caller changes.
fn body_clip(ui: &Ui, body_room: Rect) -> Rect {
    let mut clip = ui.clip_rect();
    clip.max.y = clip.max.y.min(body_room.bottom());
    clip
}

/// Where the body's content ends: the largest of each recorded part's end and the end of each
/// other widget the body added (`widgets`, on the pane's layer), and of `body`, the rect the
/// body's own `Ui` took (so a frame drawn straight in it counts with its margin); `None` when
/// the body showed nothing. A wrapper (a widget that holds a part's room) ends at its bottom less
/// the room its parts left empty. The pane's own press area (a widget that holds the body's
/// whole room), a widget as tall as the room that holds no part (a splitter), and the widgets
/// inside a part do not count; the body's own rect still holds the last two. Also whether content
/// goes on past `limit`, the body's clip: a widget runs past it, or a part that reaches it is
/// cut.
fn content_end(
    ui: &Ui,
    widgets: std::ops::Range<usize>,
    parts: &[Part],
    body: Rect,
    body_room: Rect,
    limit: f32,
) -> (Option<f32>, bool) {
    // A wrapper ends where the content of the parts it holds ends, plus what it adds below their
    // room (a frame's margin, a row under a column): its bottom, less the room they left empty.
    // A widget that holds no part ends at its bottom.
    let wrapper_end = |rect: Rect| {
        let held = || {
            parts
                .iter()
                .filter(move |part| rect.expand(0.5).contains_rect(part.room))
        };
        let room = held().map(|part| part.room.bottom()).reduce(f32::max);
        let end = held().map(|part| part.end).reduce(f32::max);
        match (room, end) {
            (Some(room), Some(end)) => rect.bottom() - (room - end).max(0.0),
            _ => rect.bottom(),
        }
    };
    let layer = ui.layer_id();
    let leaves = ui.ctx().viewport(|viewport| {
        viewport
            .this_pass
            .widgets
            .get_layer(layer)
            .enumerate()
            .skip(widgets.start)
            .take(widgets.len())
            .filter(|(at, _)| !parts.iter().any(|part| part.widgets.contains(at)))
            .map(|(_, widget)| widget.rect)
            .filter(|rect| {
                // As tall as the body's room, from its top, and holding no part: a splitter, a
                // panel's frame, an editor as tall as its room. The body's own rect below has it.
                let spans = rect.top() <= body_room.top() + 0.5
                    && (rect.bottom() - body_room.bottom()).abs() <= 0.5
                    && !parts
                        .iter()
                        .any(|part| rect.expand(0.5).contains_rect(part.room));
                rect.is_finite()
                    && rect.is_positive()
                    && !rect.expand(0.5).contains_rect(body_room)
                    && !spans
            })
            .chain(body.is_positive().then_some(body))
            .fold((None::<f32>, false), |(end, overflow), rect| {
                let this = wrapper_end(rect);
                // Content goes on under the footer where a widget runs past the clip.
                (
                    Some(end.map_or(this, |end| end.max(this))),
                    overflow || rect.bottom() > limit + 0.5,
                )
            })
    });
    let (leaves, overflow) = leaves;
    // A part's content goes on only where it is cut at the clip: a column that scrolls down to
    // it, a nested pane whose body is cut there with no footer of its own. A frame's margin under
    // a column, a row under it, or a nested pane's own footer is not cut content.
    let cut = overflow || parts.iter().any(|part| part.cuts && part.end > limit - 0.5);
    let end = parts
        .iter()
        .map(|part| part.end)
        .chain(leaves)
        .reduce(f32::max);
    (end, cut)
}
