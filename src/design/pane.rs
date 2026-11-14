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

    /// Hold a pressed control under the pointer through the one change its press makes to the
    /// content above it: the body calls [`hold_press`] at the press and [`held_seen`] where it
    /// draws that control. The first change of the content's height in the press's frame or the
    /// `CHANGE_FRAMES` (3) after it is corrected once: the pane scrolls by as much as the control
    /// moved (with blank room after the content while the scroll needs it), and the hold ends.
    /// It also ends with no change in those frames, at the DM's own wheel or drag, and at
    /// `from_top`. Nothing later is held: a later move is the pane's normal stillness. A second
    /// press while a hold waits replaces it.
    pub fn holds_presses(mut self) -> Self {
        self.holds = true;
        self
    }
}

/// The one editor of a `Footer::pane_editor` body that grows with its text, drawn once by
/// [`EditorRegion::editor`]. Only the pane makes one, for its body closure alone: the region
/// cannot be kept, copied, used twice or taken out of the closure, so a pane has at most one
/// region, and nothing but the editor is drawn in it.
///
/// While the content fits the pane's room, the region is as tall as the editor's text, and the
/// pane is a plain `Body::Content` pane. When it would not fit, the region is the room less the
/// rest of the content (as measured in the last pass; a change draws the pass again) and less
/// the footer, never less than `REGION_LEAST`; the editor scrolls inside it (a `ScrollColumn`,
/// its lane kept), and what comes after the region (a save row) and the footer stay in view.
/// When the region's height changes under a focused editor (the footer gains or loses a line),
/// the caret is brought back into view. A `ScrollColumn` inside the region is refused, as in a
/// `Content` body.
///
/// A second use does not compile:
///
/// ```compile_fail,E0382
/// use splat::pane::Footer;
/// fn two(ui: &mut eframe::egui::Ui, a: &mut String, b: &mut String) {
///     const NOTE: splat::layout::FieldName = splat::field_id!("doc_region_note");
///     Footer::new("pane", 400.0).pane_editor(ui, |ui, region| {
///         region.editor(ui, NOTE.once(splat::layout::Once::Example), a, |edit| edit);
///         region.editor(ui, NOTE.once(splat::layout::Once::Settings), b, |edit| edit);
///     });
/// }
/// ```
///
/// ```
/// use splat::pane::Footer;
/// fn one(ui: &mut eframe::egui::Ui, a: &mut String) {
///     const NOTE: splat::layout::FieldName = splat::field_id!("doc_region_note");
///     Footer::new("pane", 400.0).pane_editor(ui, |ui, region| {
///         region.editor(ui, NOTE.once(splat::layout::Once::Example), a, |edit| edit);
///         ui.label("A save row");
///     });
/// }
/// ```
///
/// It cannot leave its closure, to be used after the pane:
///
/// ```compile_fail
/// use splat::pane::Footer;
/// fn kept(ui: &mut eframe::egui::Ui, a: &mut String) {
///     const NOTE: splat::layout::FieldName = splat::field_id!("doc_region_note");
///     let region = Footer::new("pane", 400.0).pane_editor(ui, |_, region| region);
///     region.editor(ui, NOTE.once(splat::layout::Once::Example), a, |edit| edit);
/// }
/// ```
///
/// ```
/// use splat::pane::Footer;
/// fn used(ui: &mut eframe::egui::Ui, a: &mut String) -> bool {
///     const NOTE: splat::layout::FieldName = splat::field_id!("doc_region_note");
///     Footer::new("pane", 400.0)
///         .pane_editor(ui, |ui, region| region.editor(ui, NOTE.once(splat::layout::Once::Example), a, |edit| edit))
///         .response
///         .changed()
/// }
/// ```
///
/// Nor can a caller make one, or copy one:
///
/// ```compile_fail,E0451
/// fn made() -> splat::pane::EditorRegion<'static> {
///     splat::pane::EditorRegion { pane: eframe::egui::Id::NULL, max: 100.0, _scope: std::marker::PhantomData }
/// }
/// ```
///
/// ```compile_fail,E0382
/// use splat::pane::Footer;
/// fn copied(ui: &mut eframe::egui::Ui, a: &mut String) {
///     const NOTE: splat::layout::FieldName = splat::field_id!("doc_region_note");
///     Footer::new("pane", 400.0).pane_editor(ui, |ui, region| {
///         let copy = region;
///         region.editor(ui, NOTE.once(splat::layout::Once::Example), a, |edit| edit);
///         let _ = copy;
///     });
/// }
/// ```
pub struct EditorRegion<'r> {
    pane: Id,
    max: f32,
    /// Ties the region to the body closure (invariant in `'r`): it cannot be returned from it.
    _scope: std::marker::PhantomData<fn(&'r ()) -> &'r ()>,
}

/// The least height of an editor region: a pane too small for its content and this much of
/// the editor scrolls as a whole, as `Body::Content` does.
const REGION_LEAST: f32 = super::tokens::XXL * 2.0;

/// The region's height for a room of `room` with `rest` around it (unknown before the first
/// measure: the whole room, and the pass is drawn again).
fn region_height(room: f32, rest: Option<f32>) -> f32 {
    (room - rest.unwrap_or(0.0))
        .floor()
        .max(REGION_LEAST.min(room))
}

fn region_drawn_id(pane: Id) -> Id {
    pane.with("region_drawn")
}

/// The height the pane's editor region took in this pass, if it drew.
fn region_drawn(ui: &Ui, pane: Id) -> Option<f32> {
    let pass = ui.ctx().cumulative_pass_nr();
    ui.ctx()
        .data(|data| data.get_temp::<(u64, f32)>(region_drawn_id(pane)))
        .filter(|(drawn, _)| *drawn == pass)
        .map(|(_, height)| height)
}

impl EditorRegion<'_> {
    /// The pane's one editor, drawn through `field` (its id and text guard, `FieldId::editor`)
    /// inside the region, as tall as its text up to the region's height, then scrolling.
    pub fn editor(
        self,
        ui: &mut Ui,
        field: super::layout::FieldId,
        text: &mut dyn egui::TextBuffer,
        style: impl for<'t> FnOnce(egui::TextEdit<'t>) -> egui::TextEdit<'t>,
    ) -> super::layout::Editor {
        let pane = innermost(ui);
        assert!(
            pane.is_some_and(|open| open.pane == self.pane && open.content)
                || !cfg!(debug_assertions),
            "an editor region is drawn in its own pane's body, not in a pane inside it"
        );
        let key = self.pane.with("editor_region");
        let max = self.max;
        let before = ui.ctx().data(|data| data.get_temp::<f32>(key.with("max")));
        ui.ctx()
            .data_mut(|data| data.insert_temp(key.with("max"), max));
        let output = ScrollColumn::new(key)
            .max_height(max)
            .min_scrolled_height(max)
            .fit_height()
            .show_in_region(ui, |ui| {
                // A scroll inside the region is refused, as in a `Content` body.
                open(
                    ui,
                    Some(OpenPane {
                        layer: ui.layer_id(),
                        content: true,
                        usable: pane.map_or(ui.clip_rect(), |open| open.usable),
                        edge: false,
                        pane: self.pane,
                    }),
                );
                let editor = field.editor(ui, text, style);
                open(ui, None);
                // The region's height changed under the focused editor (the footer gained or
                // lost a line, the window changed): the caret stays in view. Only then, so the
                // DM's own wheel is never pulled back.
                if before.is_some_and(|before| (before - max).abs() > 0.5)
                    && editor.response.has_focus()
                    && let Some(range) = editor.cursor_range
                {
                    let caret = editor
                        .galley
                        .pos_from_cursor(range.primary)
                        .translate(editor.galley_pos.to_vec2());
                    if !ui.clip_rect().contains_rect(caret) {
                        ui.scroll_to_rect(caret, None);
                        ui.ctx()
                            .request_discard("the editor region keeps its caret in view");
                    }
                }
                editor
            });
        let pass = ui.ctx().cumulative_pass_nr();
        let height = output.inner_rect.height();
        ui.ctx().data_mut(|data| {
            data.insert_temp(region_drawn_id(self.pane), (pass, height));
        });
        output.inner
    }
}

/// A held press, kept by the pane (one owner: the pane's id).
#[derive(Clone, Copy, Debug, Default)]
struct Held {
    keep: Option<Keep>,
    /// Blank room after the content, for the hold.
    room: f32,
    /// The correction measured in a pass, as the scroll offset to apply before the next body.
    pending: Option<f32>,
    /// The content's height (without the blank room) in the last pass.
    content: f32,
    /// The frame the pane last drew in.
    drawn: u64,
}

/// The pressed control, its top at the press and in this pass (cleared before each body pass),
/// the content's height at the press, and the frame of the press.
#[derive(Clone, Copy, Debug)]
struct Keep {
    control: Id,
    top: f32,
    now: Option<f32>,
    content: Option<f32>,
    frame: u64,
}

/// Before a body pass: an expired hold ends (more than `CHANGE_FRAMES` since the press, or the
/// pane did not draw in the frame before), before any change can count; the sighting of the
/// control is cleared, so only this pass's counts. The pending correction's offset, if any.
fn hold_before_body(ui: &Ui, held: &mut Held) -> Option<f32> {
    let frame = ui.ctx().cumulative_frame_nr();
    let away = held.drawn + 1 < frame;
    if away {
        held.pending = None;
    }
    if let Some(keep) = held.keep.as_mut() {
        if away || frame > keep.frame + CHANGE_FRAMES {
            held.keep = None;
            hold_ended(ui, "expired");
        } else {
            keep.now = None;
        }
    }
    held.drawn = frame;
    held.pending.take()
}

/// How many frames after its press a change may come and still be the press's own (the shell
/// applies an act after the view drew).
const CHANGE_FRAMES: u64 = 3;

fn holding_id() -> Id {
    Id::new("design_pane_holding")
}

/// The held press of the pane whose body is being drawn, if that pane holds presses.
fn with_held(ui: &Ui, change: impl FnOnce(&mut Held)) {
    let Some(pane) = ui
        .ctx()
        .data(|data| data.get_temp::<Option<Id>>(holding_id()))
        .flatten()
    else {
        return;
    };
    ui.ctx()
        .data_mut(|data| change(data.get_temp_mut_or_default::<Held>(pane)));
}

/// The body pressed `pressed`, the control kept as `control` (any stable key of it, the same
/// that `held_seen` gives): hold it in place.
pub fn hold_press(ui: &Ui, control: impl std::hash::Hash, pressed: &egui::Response) {
    let control = Id::new(control);
    let top = pressed.rect.top();
    let frame = ui.ctx().cumulative_frame_nr();
    with_held(ui, |held| {
        held.keep = Some(Keep {
            control,
            top,
            now: None,
            content: None,
            frame,
        });
    });
}

/// The body drew `control` with its top at `top` in this pass.
pub fn held_seen(ui: &Ui, control: impl std::hash::Hash, top: f32) {
    let control = Id::new(control);
    with_held(ui, |held| {
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

/// One open pane body (or the inside of an editor region, which counts as a `Content` body:
/// it refuses a scroll inside it).
#[derive(Clone, Copy)]
struct OpenPane {
    layer: LayerId,
    /// The body is a scroll column already (`Body::Content`, or an editor region's inside).
    content: bool,
    /// What a layer opened from the body may use.
    usable: Rect,
    /// An edge (the footer, the session banner) ends the body: a `Fills` body's scroll columns
    /// then cut at their last whole registered row.
    edge: bool,
    /// The pane's footer id.
    pane: Id,
}

/// What kind of pane is open now, innermost last, per layer, with its body's usable rect.
#[derive(Clone, Default)]
struct Open {
    pass: u64,
    panes: Vec<OpenPane>,
}

fn open_id() -> Id {
    Id::new("design_open_panes")
}

/// Open (`Some(pane)`) or close (`None`) the innermost pane on `ui`'s layer.
fn open(ui: &Ui, content: Option<OpenPane>) {
    let pass = ui.ctx().cumulative_pass_nr();
    ui.ctx().data_mut(|data| {
        let open = data.get_temp_mut_or_default::<Open>(open_id());
        if open.pass != pass {
            *open = Open {
                pass,
                panes: Vec::new(),
            };
        }
        match content {
            Some(pane) => open.panes.push(pane),
            None => {
                open.panes.pop();
            }
        }
    });
}

/// The innermost open pane body on `ui`'s layer in this pass.
fn innermost(ui: &Ui) -> Option<OpenPane> {
    let (layer, pass) = (ui.layer_id(), ui.ctx().cumulative_pass_nr());
    ui.ctx().data(|data| {
        data.get_temp::<Open>(open_id())
            .filter(|open| open.pass == pass)
            .and_then(|open| {
                open.panes
                    .iter()
                    .rev()
                    .find(|pane| pane.layer == layer)
                    .copied()
            })
    })
}

/// Whether a vertical `ScrollColumn` about to be drawn on `ui` is inside a `Body::Content` pane,
/// which is a scroll column already (or inside an editor region, which scrolls its editor).
/// Debug builds and tests refuse it (a panic, so that its caller is fixed); a release build draws
/// its content in place (`ScrollColumn::inline`), and the pane scrolls it.
pub(super) fn column_in_content(ui: &Ui) -> bool {
    let (layer, pass) = (ui.layer_id(), ui.ctx().cumulative_pass_nr());
    let inside = ui.ctx().data(|data| {
        data.get_temp::<Open>(open_id()).is_some_and(|open| {
            open.pass == pass
                && open
                    .panes
                    .last()
                    .is_some_and(|pane| pane.layer == layer && pane.content)
        })
    });
    assert!(
        !(inside && cfg!(debug_assertions)),
        "a ScrollColumn inside a `Body::Content` pane: the pane is the scroll column. Give its \
         content to the pane, draw a header that must not scroll before the pane, or use \
         `Body::Fills(Fills::because(FillsReason::Columns))` for columns that each scroll"
    );
    inside
}

/// The usable rect of the innermost pane body that `ui` is drawn in, on `ui`'s layer: its room
/// above the footer, inside the clip. A part that opens a layer from a body (a drop-down's list)
/// keeps inside it, so it never covers the pane's footer. `None` outside a pane.
pub fn body_room(ui: &Ui) -> Option<Rect> {
    innermost(ui).map(|pane| pane.usable)
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

/// What the hold needs of a pass's scroll column, kept until the footer has measured.
#[derive(Clone, Copy)]
struct ColumnFacts {
    state: egui::scroll_area::State,
    content_size: egui::Vec2,
    inner_rect: Rect,
}

/// After a body pass, once the footer has measured (see `ContentScroll::holds_presses`): when
/// the content's height first differs from the press's, the control's move in this pass is kept
/// as the pending correction, and the hold ends. The correction is applied before the next
/// body pass (`hold_before_body`), so that pass draws with it and the scroll area clamps it
/// with that pass's room; another pass is asked for. A control not drawn in this pass ends the
/// hold with no correction. The blank room after the content is only what the scroll needs.
fn hold_place(ui: &Ui, mut held: Held, output: ColumnFacts) -> Held {
    let state = output.state;
    let content = output.content_size.y - held.room;
    held.content = content;
    let scrolled = ui.input(|i| {
        i.raw_scroll_delta != egui::Vec2::ZERO
            || i.smooth_scroll_delta != egui::Vec2::ZERO
            || i.pointer.is_decidedly_dragging()
    });
    if let Some(keep) = held.keep.as_mut() {
        let changed = (*keep.content.get_or_insert(content) - content).abs() > 0.5;
        let end = if scrolled {
            Some("scrolled")
        } else if changed && keep.now.is_none() {
            Some("gone")
        } else if changed {
            Some("corrected")
        } else {
            None
        };
        if let Some(why) = end {
            let keep = held.keep.take().expect("a hold");
            hold_ended(ui, why);
            if why == "corrected"
                && let Some(now) = keep.now
                && (now - keep.top).abs() > 0.5
            {
                held.pending = Some((state.offset.y + now - keep.top).max(0.0));
                ui.ctx()
                    .request_discard("a press moved the content above its control");
                return held;
            }
        }
    }
    held.room = (state.offset.y + output.inner_rect.height() - content).max(0.0);
    held
}

/// Test builds record why the last hold ended.
fn hold_ended(ui: &Ui, why: &'static str) {
    #[cfg(test)]
    ui.ctx()
        .data_mut(|data| data.insert_temp(Id::new("design_pane_hold_ended"), why));
    #[cfg(not(test))]
    let _ = (ui, why);
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

/// The registered rows that a `Fills` body's scroll column already cut, in this pass: a column
/// that holds another leaves the inner column's rows to it.
#[derive(Clone, Default)]
struct Claimed {
    pass: u64,
    rows: Vec<std::ops::Range<usize>>,
}

fn claimed_id() -> Id {
    Id::new("design_pane_claimed_rows")
}

/// The content of a `ScrollColumn` (`ui` is its content `Ui`, its clip the column's view).
/// Directly in a `Body::Fills` pane that an edge ends (the footer, the session banner), the
/// column never cuts through a registered row (N12, ruling R4): its clip ends at the top of the
/// lowest row that its view's bottom would cut, as measured in the last pass with the same view
/// (a new cut draws the pass again), and a hairline marks it. Each column cuts at its own last
/// whole row; a row that starts at the view's top keeps the plain cut, as in `Body::Content`.
/// Elsewhere it only runs `add`.
pub(super) fn whole_rows<R>(ui: &mut Ui, column: Id, add: impl FnOnce(&mut Ui) -> R) -> R {
    let Some(pane) = innermost(ui).filter(|pane| !pane.content && pane.edge) else {
        return add(ui);
    };
    let view = ui.clip_rect();
    let key = pane.pane.with(("column_cut", column));
    let kept = ui
        .ctx()
        .data(|data| data.get_temp::<(Rect, f32)>(key))
        .filter(|(at, _)| *at == view)
        .map(|(_, cut)| cut);
    if let Some(cut) = kept {
        let mut clip = view;
        clip.max.y = clip.max.y.min(cut);
        ui.set_clip_rect(clip);
    }
    let before = super::components::rows_this_pass(ui.ctx()).len();
    let inner = add(ui);
    let rows = super::components::rows_this_pass(ui.ctx());
    let after = rows.len();
    let pass = ui.ctx().cumulative_pass_nr();
    let own: Vec<Rect> = ui.ctx().data_mut(|data| {
        let claimed = data.get_temp_mut_or_default::<Claimed>(claimed_id());
        if claimed.pass != pass {
            *claimed = Claimed {
                pass,
                rows: Vec::new(),
            };
        }
        let own = (before..after)
            .filter(|at| !claimed.rows.iter().any(|range| range.contains(at)))
            .filter_map(|at| rows.get(at).copied())
            .collect();
        claimed.rows.push(before..after);
        own
    });
    let cut = whole_row_cut(&own, view);
    if cut != kept {
        ui.ctx().data_mut(|data| match cut {
            Some(cut) => data.insert_temp(key, (view, cut)),
            None => data.remove::<(Rect, f32)>(key),
        });
        ui.ctx().request_discard("a column's whole-row cut moved");
    }
    if let Some(cut) = cut {
        ui.painter().with_clip_rect(view).hline(
            view.x_range(),
            cut - super::tokens::HAIRLINE_WIDTH / 2.0,
            egui::Stroke::new(
                super::tokens::HAIRLINE_WIDTH,
                super::tokens::colors().hairline,
            ),
        );
    }
    inner
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
        self.place(ui, body, ContentScroll::default(), |ui, _| add(ui))
    }

    /// `pane` with `Body::Content`, whose scroll column behaves as `scroll` says.
    pub fn pane_content<R>(
        self,
        ui: &mut Ui,
        scroll: ContentScroll,
        add: impl FnOnce(&mut Ui) -> R,
    ) -> R {
        self.place(ui, Body::Content, scroll, |ui, _| add(ui))
    }

    /// A `Body::Content` pane whose body holds one editor that grows with its text: the body
    /// gets an [`EditorRegion`], which draws the editor once. While the content fits, the pane
    /// is exactly `Body::Content` (the footer `S` below the content). When it does not, the
    /// region gives way: its height is the room less the rest of the content (and less the
    /// footer), the editor scrolls inside it, and what comes after it (a save row) and the
    /// footer stay in view. A body that does not use the region is `Body::Content`.
    pub fn pane_editor<R>(
        self,
        ui: &mut Ui,
        add: impl for<'r> FnOnce(&mut Ui, EditorRegion<'r>) -> R,
    ) -> R {
        self.place(ui, Body::Content, ContentScroll::default(), add)
    }

    fn place<R>(
        mut self,
        ui: &mut Ui,
        body: Body,
        options: ContentScroll,
        add: impl for<'r> FnOnce(&mut Ui, EditorRegion<'r>) -> R,
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
        // An edge (the footer, the session banner) ends the body: no registered row is cut.
        let edge = shown || banner_below(ui.ctx());
        // What a layer opened from the body may use: the body's room, inside the clip.
        let usable = visible.intersect(body_room);
        let mut hold_facts = None;
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
                    if held.keep.is_some() {
                        hold_ended(ui, "from top");
                    }
                    held = Held::default();
                }
                let mut column = ScrollColumn::new(id.with("column"))
                    .max_height(room_height)
                    .min_scrolled_height(room_height);
                if options.from_top {
                    column = column.vertical_scroll_offset(0.0);
                }
                if options.holds
                    && let Some(offset) = hold_before_body(ui, &mut held)
                {
                    // The correction measured in the last pass, with the blank room it needs
                    // in this pass's room.
                    held.room = (offset + room_height - held.content).max(0.0);
                    column = column.vertical_scroll_offset(offset);
                }
                if options.holds {
                    ui.ctx().data_mut(|data| {
                        data.insert_temp(held_key, held);
                        data.insert_temp(holding_id(), Some(held_key));
                    });
                }
                // The editor region's height: the room less the rest of the content, as last
                // measured; a change of the rest draws the pass again (below).
                let rest_key = id.with("region_rest");
                let rest = ui.ctx().data(|data| data.get_temp::<f32>(rest_key));
                let region_max = region_height(room_height, rest);
                let output = column.show(&mut body_ui, |ui| {
                    open(
                        ui,
                        Some(OpenPane {
                            layer: ui.layer_id(),
                            content: true,
                            usable,
                            edge,
                            pane: id,
                        }),
                    );
                    let first = widget_count(ui);
                    let inner = add(
                        ui,
                        EditorRegion {
                            pane: id,
                            max: region_max,
                            _scope: std::marker::PhantomData,
                        },
                    );
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
                    hold_facts = Some((
                        held_key,
                        ColumnFacts {
                            state: output.state,
                            content_size: output.content_size,
                            inner_rect: output.inner_rect,
                        },
                    ));
                    ui.ctx()
                        .data(|data| data.get_temp::<Held>(held_key))
                        .unwrap_or_default()
                        .room
                } else {
                    0.0
                };
                let scroll = Scroll {
                    viewport: output.inner_rect,
                    height: output.content_size.y - room,
                    offset: output.state.offset.y,
                };
                // The rest of the content around the editor region, for the next pass.
                let drawn = region_drawn(ui, id);
                let now = drawn.map(|drawn| (scroll.height - drawn).max(0.0));
                if let Some(now) = now
                    && rest.is_none_or(|rest| (rest - now).abs() > 0.5)
                {
                    ui.ctx().data_mut(|data| data.insert_temp(rest_key, now));
                    ui.ctx()
                        .request_discard("the editor region measured the content around it");
                } else if now.is_none() && rest.is_some() {
                    ui.ctx().data_mut(|data| data.remove::<f32>(rest_key));
                }
                (output.inner, Some(scroll))
            }
            Body::Fills(_) => {
                open(
                    &body_ui,
                    Some(OpenPane {
                        layer: body_ui.layer_id(),
                        content: false,
                        usable,
                        edge,
                        pane: id,
                    }),
                );
                let inner = add(
                    &mut body_ui,
                    EditorRegion {
                        pane: id,
                        max: 0.0,
                        _scope: std::marker::PhantomData,
                    },
                );
                open(&body_ui, None);
                (inner, None)
            }
        };
        drop(body_ui);
        // N12: an edge (the footer, the session banner) never cuts through a registered row.
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
        // The held press, once the footer has measured this pass.
        if let Some((held_key, facts)) = hold_facts {
            let held = ui
                .ctx()
                .data(|data| data.get_temp::<Held>(held_key))
                .unwrap_or_default();
            let held = hold_place(ui, held, facts);
            ui.ctx().data_mut(|data| data.insert_temp(held_key, held));
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

    struct State {
        holds: bool,
        rows_above: usize,
        /// The press's rows, until the test applies them (an act applied in a later frame).
        later: Option<usize>,
        from_top: bool,
        /// Rows between the pressed control and the far control.
        below: usize,
        /// The footer holds a reason.
        message: bool,
        /// The pane is not drawn (closed).
        hidden: bool,
        /// The pressed control is gone.
        gone: bool,
    }

    impl State {
        fn new(holds: bool, later: bool) -> Self {
            Self {
                holds,
                rows_above: 2,
                later: later.then_some(0),
                from_top: false,
                below: 60,
                message: false,
                hidden: false,
                gone: false,
            }
        }
    }

    /// A list with rows above a button and many below, then a control far below; the button's
    /// press adds rows above it (or keeps them in `later`).
    fn list(ui: &mut Ui, state: &mut State) {
        use crate::design::outcome::{Messages, NoPress, Outcome, PaneFrame};
        if state.hidden {
            return;
        }
        let mut scroll = ContentScroll::default().from_top(std::mem::take(&mut state.from_top));
        if state.holds {
            scroll = scroll.holds_presses();
        }
        let frame = PaneFrame::new(ui);
        let mut messages = Messages::<NoPress>::new(&frame, "held", ui.available_width());
        if state.message {
            messages.outcome(Outcome::Refused(crate::sentence!(
                "Save the draft first. The other changes wait until it is saved."
            )));
        }
        let _ = messages.pane_content(ui, scroll, |ui| {
            for row in 0..state.rows_above {
                ui.label(format!("Row above {row}"));
            }
            if !state.gone {
                let response = ui.button("Add a row above");
                held_seen(ui, "add", response.rect.top());
                if response.clicked() {
                    hold_press(ui, "add", &response);
                    match state.later.as_mut() {
                        Some(rows) => *rows += 3,
                        None => state.rows_above += 3,
                    }
                }
            }
            let _ = ui.button("Next control");
            for row in 0..state.below {
                ui.label(format!("Row below {row}"));
            }
            let _ = ui.button("Far control");
        });
    }

    fn harness(state: State) -> Harness<'static, State> {
        let mut h = Harness::builder()
            .with_size(egui::vec2(400.0, 300.0))
            .build_ui_state(list, state);
        crate::design::theme::install(&h.ctx);
        h.run_steps(4);
        h
    }

    fn ended(h: &Harness<'_, State>) -> Option<&'static str> {
        h.ctx
            .data(|data| data.get_temp::<&'static str>(Id::new("design_pane_hold_ended")))
    }

    fn top(h: &Harness<'_, State>, label: &str) -> f32 {
        h.get_by_label(label).rect().top()
    }

    /// The press's rows, applied now.
    fn apply_later(h: &mut Harness<'_, State>) {
        if let Some(rows) = h.state_mut().later.take() {
            h.state_mut().rows_above += rows;
        }
    }

    /// The pane's rule, not the hold's: focus moves to the control at the list's end, and it
    /// comes whole into view; then a footer comes with no change of focus, and the focused
    /// control comes whole above it.
    #[test]
    fn a_focused_control_comes_whole_above_the_footer() {
        let mut h = harness(State::new(true, false));
        h.get_by_label("Far control").focus();
        h.run_steps(4);
        let focused = h.ctx.memory(|memory| memory.focused());
        assert!(focused.is_some(), "Tab gave a control focus");
        h.state_mut().message = true;
        h.run_steps(4);
        assert_eq!(h.ctx.memory(|memory| memory.focused()), focused);
        let far = h
            .ctx
            .viewport(|viewport| {
                viewport
                    .prev_pass
                    .widgets
                    .get(focused.unwrap())
                    .map(|w| w.rect)
            })
            .expect("the focused control is drawn");
        let record = pane_records(&h.ctx).into_iter().next().unwrap();
        let (viewport, footer) = (record.viewport().unwrap(), record.footer().unwrap());
        assert!(
            far.top() >= viewport.top() - 0.5 && far.bottom() <= footer.top() + 0.5,
            "the focused control is whole above the footer: {far:?}, footer {footer:?}"
        );
    }

    /// The DM's own wheel ends a waiting hold, and so does `from_top`: the change that comes
    /// after either is not corrected (the control moves by the rows added).
    #[test]
    fn the_wheel_or_from_top_ends_a_waiting_hold() {
        for by in ["scrolled", "from top"] {
            let mut h = harness(State::new(true, true));
            h.get_by_label("Add a row above").click();
            h.step();
            if by == "scrolled" {
                h.event(egui::Event::MouseWheel {
                    unit: egui::MouseWheelUnit::Point,
                    delta: egui::vec2(0.0, -1.0),
                    modifiers: egui::Modifiers::NONE,
                });
            } else {
                h.state_mut().from_top = true;
            }
            h.step();
            assert_eq!(ended(&h), Some(by));
            let before = top(&h, "Add a row above");
            apply_later(&mut h);
            h.run_steps(4);
            let after = top(&h, "Add a row above");
            assert!(
                after - before > 20.0,
                "{by}: not held: {before} then {after}"
            );
        }
    }

    /// Review 4, D1: a hold that expired does not scroll. A change in the fourth frame after
    /// the press, or after the pane was closed for a frame, is not the press's: the control
    /// moves by the rows added.
    #[test]
    fn an_expired_hold_does_not_scroll() {
        for closed in [false, true] {
            let mut h = harness(State::new(true, true));
            let before = top(&h, "Add a row above");
            h.get_by_label("Add a row above").click();
            // The press's frame F.
            h.step();
            if closed {
                h.state_mut().hidden = true;
                h.step();
                h.state_mut().hidden = false;
            } else {
                // F+1 to F+3 with no change.
                h.run_steps(3);
            }
            apply_later(&mut h);
            h.step();
            h.run_steps(4);
            let after = top(&h, "Add a row above");
            assert_eq!(ended(&h), Some("expired"), "closed {closed}");
            assert!(
                after - before > 20.0,
                "closed {closed}: not held: {before} then {after}"
            );
        }
    }

    /// Review 4, D2: rows reorder above the pressed control (the same height), then the control
    /// goes and the height changes: no sighting in that pass, so the hold ends with no scroll.
    #[test]
    fn a_control_that_is_gone_is_not_held() {
        let mut h = harness(State::new(true, true));
        h.get_by_label("Add a row above").click();
        h.step();
        // Reorder: three rows move from below the control to above it.
        h.state_mut().rows_above += 3;
        h.state_mut().below -= 3;
        h.step();
        let row = top(&h, "Row above 0");
        h.state_mut().gone = true;
        h.step();
        h.run_steps(4);
        assert_eq!(ended(&h), Some("gone"));
        assert!(
            (top(&h, "Row above 0") - row).abs() < 0.5,
            "nothing scrolled: {row} then {}",
            top(&h, "Row above 0")
        );
    }

    /// `ContentScroll::holds_presses`: a press that adds rows above its control keeps the
    /// control under the pointer, so a second press of a still pointer presses it again, also
    /// when the act comes a frame after the press, and (review 4, D3) when the footer changes
    /// height in the same frame: the first frame shown has the control in place. The one
    /// correction ends the hold. Without the option, the control moves down by the rows added.
    #[test]
    fn a_held_press_keeps_its_control_under_the_pointer() {
        for (holds, later, footer) in [
            (true, false, false),
            (true, true, false),
            (true, true, true),
            (false, false, false),
        ] {
            let mut h = harness(State::new(holds, later));
            let before = top(&h, "Add a row above");
            h.get_by_label("Add a row above").click();
            h.step();
            if later {
                apply_later(&mut h);
                h.state_mut().message = footer;
                h.step();
                let first = top(&h, "Add a row above");
                assert!(
                    (first - before).abs() < 0.5,
                    "footer {footer}: the first frame shown: {before} then {first}"
                );
            }
            h.run_steps(8);
            let after = top(&h, "Add a row above");
            assert_eq!(h.state().rows_above, 5);
            if holds {
                assert!((after - before).abs() < 0.5, "held: {before} then {after}");
                assert_eq!(
                    ended(&h),
                    Some("corrected"),
                    "the one correction ends the hold"
                );
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
    /// caller (the clip is the pane's; no caller sets one), this test for the part. Round 28, P2:
    /// the same in a `Body::Fills` pane for the rows of its scroll columns, two side by side with
    /// rows of different heights: each column cuts at its own last whole row.
    #[test]
    #[allow(
        clippy::disallowed_methods,
        reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
    )]
    fn a_footer_never_cuts_through_a_registered_row() {
        use crate::design::layout::ScrollColumn;
        let mut hidden_by_the_cut = 0;
        let (mut columns_apart, mut plain_fills_cut) = (0, 0);
        for fills in [false, true] {
            for message in [true, false] {
                for height in (300..=360).step_by(7) {
                    let mut harness = egui_kittest::Harness::builder()
                        .with_size(vec2(600.0, height as f32))
                        .build_ui(move |ui| {
                            let frame = PaneFrame::new(ui);
                            let mut messages = Messages::<NoPress>::new(
                                &frame,
                                "row_cut_test",
                                ui.available_width(),
                            );
                            if message {
                                messages.outcome(Outcome::Refused(crate::sentence!(
                                    "Save the draft first."
                                )));
                            }
                            let rows = |ui: &mut Ui, name: &str, height: f32| {
                                for at in 0..30 {
                                    let _ = two_line_row(
                                        ui,
                                        height,
                                        false,
                                        &format!("{name} {at}"),
                                        "Its second line",
                                    );
                                }
                            };
                            if fills {
                                let body = Body::Fills(Fills::for_test(FillsReason::Columns));
                                let _ = messages.pane(ui, body, |ui| {
                                    ui.columns(2, |columns| {
                                        ScrollColumn::new("rows_left")
                                            .show(&mut columns[0], |ui| rows(ui, "Row", 44.0));
                                        ScrollColumn::new("rows_right")
                                            .show(&mut columns[1], |ui| rows(ui, "Tall", 60.0));
                                    });
                                });
                            } else {
                                let _ =
                                    messages.pane(ui, Body::Content, |ui| rows(ui, "Row", 44.0));
                            }
                        });
                    crate::design::theme::install(&harness.ctx);
                    harness.run_steps(4);
                    let output = harness.output();
                    let rows = harness
                        .ctx
                        .data(|data| data.get_temp::<(u64, Vec<Rect>)>(Id::new(ROW_RECTS)))
                        .map(|(_, rows)| rows)
                        .unwrap_or_default();
                    let count = if fills { 60 } else { 30 };
                    assert!(
                        rows.len() == count,
                        "{fills} {message} {height}: rows {}",
                        rows.len()
                    );
                    // Each column: its rows (by side) and the clip its first row is painted in.
                    let clip_of = |first: &str| {
                        output
                            .shapes
                            .iter()
                            .find(|clipped| {
                                matches!(&clipped.shape, egui::Shape::Text(text) if text.galley.job.text == first)
                            })
                            .map(|clipped| clipped.clip_rect)
                            .unwrap_or_else(|| panic!("{first} is painted"))
                    };
                    let sides: Vec<(Rect, Vec<Rect>)> = if fills {
                        let middle = 300.0;
                        vec![
                            (
                                clip_of("Row 0"),
                                rows.iter()
                                    .copied()
                                    .filter(|row| row.center().x < middle)
                                    .collect(),
                            ),
                            (
                                clip_of("Tall 0"),
                                rows.iter()
                                    .copied()
                                    .filter(|row| row.center().x >= middle)
                                    .collect(),
                            ),
                        ]
                    } else {
                        vec![(clip_of("Row 0"), rows.clone())]
                    };
                    let record = pane_records(&harness.ctx)
                        .into_iter()
                        .next()
                        .expect("the pane keeps its record");
                    let room = record.room();
                    for (view, side) in &sides {
                        let clip = view.bottom();
                        // The column's own view: a `Content` pane's viewport, or the clip of
                        // a `Fills` column with no cut (the room's bottom bounds it).
                        let body_bottom = footer_rect(&harness.ctx, Id::new("row_cut_test"))
                            .map_or(room.bottom(), |footer| footer.top() - S);
                        let viewport = record.viewport().unwrap_or(Rect::from_min_max(
                            view.min,
                            pos2(view.right(), body_bottom),
                        ));
                        // With no footer the column keeps its plain cut (a row may show in part).
                        for row in side.iter().filter(|_| message) {
                            assert!(
                                row.bottom() <= clip + 0.5 || row.top() >= clip - 0.5,
                                "n12-cut: fills {fills}, message {message}, height {height}: row {row:?} cut at {clip}"
                            );
                        }
                        if message {
                            hidden_by_the_cut += side
                                .iter()
                                .filter(|row| {
                                    row.top() >= clip - 0.5 && row.top() < viewport.bottom()
                                })
                                .count();
                            let hairline = output.shapes.iter().any(|clipped| {
                                matches!(&clipped.shape, egui::Shape::LineSegment { points, .. }
                                    if (points[0].y - clip).abs() <= 1.0 && points[0].y == points[1].y
                                        && points[0].x <= view.center().x && points[1].x >= view.center().x)
                            });
                            assert!(
                                hairline || clip >= viewport.bottom() - 0.5,
                                "n12-hairline: fills {fills}, height {height}, cut {clip}"
                            );
                        } else if fills {
                            plain_fills_cut += side
                                .iter()
                                .filter(|row| row.top() < clip - 0.5 && row.bottom() > clip + 0.5)
                                .count();
                        } else {
                            assert!(
                                (clip - viewport.bottom()).abs() <= 0.5,
                                "n12-plain: with no footer the column keeps its own cut: {clip} {viewport:?}"
                            );
                        }
                    }
                    if fills && message && (sides[0].0.bottom() - sides[1].0.bottom()).abs() > 0.5 {
                        columns_apart += 1;
                    }
                }
            }
        }
        assert!(hidden_by_the_cut > 0, "n12-reached: some height cut a row");
        assert!(
            columns_apart > 0,
            "n12-columns: each column cuts at its own last whole row"
        );
        assert!(
            plain_fills_cut > 0,
            "n12-plain-fills: with no edge a `Fills` column keeps its plain cut"
        );
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

#[cfg(test)]
mod editor_region_tests {
    use super::*;
    use crate::design::{
        layout::Once,
        outcome::{Messages, NoPress, Outcome, PaneFrame},
    };

    const LOCAL: &str = "editor_region_test";

    #[derive(Default)]
    struct State {
        lines: usize,
        /// Messages in the footer: 0, 1 or 2.
        messages: usize,
        /// Draw the region (`pane_editor`); `false` with `plain` draws `Body::Content`.
        region: bool,
        plain: bool,
        text: String,
        save: Option<Rect>,
        caret: Option<Rect>,
        focus: bool,
    }

    fn harness(height: f32, state: State) -> egui_kittest::Harness<'static, State> {
        let mut harness = egui_kittest::Harness::builder()
            .with_size(vec2(600.0, height))
            .build_ui_state(
                |ui, state: &mut State| {
                    let want = state.lines;
                    if state.text.lines().count() != want {
                        state.text = (0..want)
                            .map(|line| format!("Line {line}"))
                            .collect::<Vec<_>>()
                            .join("\n");
                    }
                    let frame = PaneFrame::new(ui);
                    let mut messages =
                        Messages::<NoPress>::new(&frame, LOCAL, ui.available_width());
                    if state.messages > 0 {
                        messages
                            .outcome(Outcome::Refused(crate::sentence!("Save the draft first.")));
                    }
                    if state.messages > 1 {
                        messages.outcome(Outcome::Refused(crate::sentence!(
                            "Line 3 opens a list that never closes."
                        )));
                    }
                    const FIELD: crate::design::layout::FieldName =
                        crate::field_id!("editor_region_test_text");
                    let focus = std::mem::take(&mut state.focus);
                    let text = &mut state.text;
                    let save_row = |ui: &mut Ui| ui.button("Save").rect;
                    let (save, caret) = if state.plain {
                        let paned = messages.pane(ui, Body::Content, |ui| {
                            ui.label("Title");
                            for line in 0..want {
                                ui.label(format!("Line {line}"));
                            }
                            (save_row(ui), None)
                        });
                        paned.body
                    } else {
                        let region = state.region;
                        messages
                            .pane_editor(ui, |ui, editor| {
                                ui.label("Title");
                                let mut caret = None;
                                if region {
                                    let output = editor.editor(
                                        ui,
                                        FIELD.once(Once::Example),
                                        text,
                                        |edit| edit.desired_width(f32::INFINITY),
                                    );
                                    if focus {
                                        output.response.request_focus();
                                    }
                                    caret = output.cursor_range.map(|range| {
                                        output
                                            .galley
                                            .pos_from_cursor(range.primary)
                                            .translate(output.galley_pos.to_vec2())
                                    });
                                } else {
                                    for line in 0..want {
                                        ui.label(format!("Line {line}"));
                                    }
                                }
                                (save_row(ui), caret)
                            })
                            .body
                    };
                    state.save = Some(save);
                    state.caret = caret;
                },
                state,
            );
        crate::design::theme::install(&harness.ctx);
        harness.run_steps(4);
        harness
    }

    fn footer(harness: &egui_kittest::Harness<'_, State>) -> Option<Rect> {
        let at = footer_rect(&harness.ctx, Id::new(LOCAL));
        if at.is_none() {
            eprintln!(
                "no footer: record {:?}, records {}",
                pane_record(&harness.ctx, Id::new(LOCAL)),
                pane_records(&harness.ctx).len()
            );
        }
        at
    }

    /// The clip of the editor's painted text: the region's view.
    fn region_view(harness: &egui_kittest::Harness<'_, State>) -> Rect {
        harness
            .output()
            .shapes
            .iter()
            .find(|clipped| {
                matches!(&clipped.shape, egui::Shape::Text(text)
                    if text.galley.job.text.starts_with("Line 0\nLine 1"))
            })
            .map(|clipped| clipped.clip_rect)
            .expect("the editor's text is painted")
    }

    /// Round 28, P2: a pane's editor region. A short draft: the footer is `S` under the save
    /// row, as for `Body::Content`. A 600-line draft at 300 px: the save row is whole in the room
    /// above the footer, the region scrolls, the caret at the end is in view; the footer gains a
    /// line: the save row stays whole in view, and so does the caret. Growing past the room, the
    /// save row never moves by more than the growth. A body that does not use the region is
    /// `Body::Content`.
    #[test]
    fn an_editor_region_keeps_its_save_row_and_caret_in_view() {
        // Short: the footer follows the save row.
        let short = harness(
            640.0,
            State {
                lines: 4,
                messages: 1,
                region: true,
                ..State::default()
            },
        );
        let save = short.state().save.expect("the save row draws");
        let at = footer(&short).expect("the footer shows");
        assert!(
            (at.top() - (save.bottom() + S)).abs() <= 1.0,
            "p2-short: the footer {at:?} is `S` under the save row {save:?}"
        );
        // Long, 300 px room: the save row whole above the footer, the region scrolls.
        let mut long = harness(
            300.0,
            State {
                lines: 600,
                messages: 1,
                region: true,
                focus: true,
                ..State::default()
            },
        );
        long.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::End);
        long.run_steps(2);
        long.event(egui::Event::Text("!".into()));
        long.run_steps(4);
        let whole = |harness: &egui_kittest::Harness<'_, State>, case: &str| {
            let save = harness.state().save.expect("the save row draws");
            let at = footer(harness).expect("the footer shows");
            let room = pane_record(&harness.ctx, Id::new(LOCAL))
                .expect("a record")
                .room();
            assert!(
                save.top() >= room.top() - 0.5 && save.bottom() <= at.top() - S + 0.5,
                "p2-save-{case}: the save row {save:?} is whole in the room {room:?} above the footer {at:?}"
            );
            let view = region_view(harness);
            let caret = harness.state().caret.expect("the editor has its caret");
            assert!(
                view.expand(0.5).contains_rect(caret),
                "p2-caret-{case}: the caret {caret:?} is in the region's view {view:?}"
            );
            assert!(
                view.bottom() <= save.top() + 0.5 && view.height() < 600.0,
                "p2-region-{case}: the region {view:?} scrolls above the save row {save:?}"
            );
        };
        assert!(
            long.state().text.ends_with("Line 599!"),
            "p2-typed: the caret is at the end"
        );
        whole(&long, "long");
        // The footer gains a line with the caret at the end.
        long.state_mut().messages = 2;
        long.run_steps(4);
        whole(&long, "footer-grew");
        long.state_mut().messages = 0;
        long.run_steps(4);
        whole(&long, "footer-emptied");

        // Growing past the room: the save row never moves by more than the growth, and it is
        // whole above the footer at every count.
        let mut grow = harness(
            300.0,
            State {
                lines: 2,
                messages: 1,
                region: true,
                ..State::default()
            },
        );
        let line = grow.ctx.style().text_styles[&egui::TextStyle::Body].size * 1.5;
        let mut last = grow.state().save.expect("the save row draws").top();
        let mut moved = 0;
        let mut held = 0;
        for lines in 3..30 {
            grow.state_mut().lines = lines;
            grow.run_steps(4);
            let save = grow.state().save.expect("the save row draws");
            let at = footer(&grow).expect("the footer shows");
            assert!(
                save.top() - last >= -0.5 && save.top() - last <= line,
                "p2-growth: {lines} lines moved the save row from {last} to {}",
                save.top()
            );
            assert!(
                save.bottom() <= at.top() - S + 0.5,
                "p2-growth-whole: {lines} lines, save row {save:?}, footer {at:?}"
            );
            if save.top() - last > 0.5 {
                moved += 1;
            } else {
                held += 1;
            }
            last = save.top();
        }
        assert!(
            moved > 0 && held > 0,
            "p2-growth-reached: moved {moved}, held {held}"
        );

        // No region: exactly `Body::Content`.
        for lines in [3, 60] {
            let with = harness(
                400.0,
                State {
                    lines,
                    messages: 1,
                    ..State::default()
                },
            );
            let plain = harness(
                400.0,
                State {
                    lines,
                    messages: 1,
                    plain: true,
                    ..State::default()
                },
            );
            let facts = |harness: &egui_kittest::Harness<'_, State>| {
                let record = pane_record(&harness.ctx, Id::new(LOCAL)).expect("a record");
                (
                    harness.state().save.expect("the save row draws"),
                    record.placement(),
                    record.viewport(),
                    record.content_height(),
                    footer(harness),
                    record.cut(),
                )
            };
            assert_eq!(
                facts(&with),
                facts(&plain),
                "p2-plain: {lines} lines, a body with no region is `Body::Content`"
            );
        }
    }
}
