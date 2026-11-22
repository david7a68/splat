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
//!   content when the content is shorter than the room and not scrolled, and may then take the
//!   room down to the pane's end; it is pinned at the pane's bottom otherwise, at most a third of
//!   the pane tall (ruling Q-c), and then draws a hairline where the content goes on below it. A
//!   footer scrolls inside only beyond that room. A body
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
//! use splat_design::pane::{Body, Footer};
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
//! use splat_design::pane::{Body, Footer};
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
//! use splat_design::pane::{Body, Footer};
//! fn twice(ui: &mut egui::Ui) {
//!     let footer = Footer::new("pane", 400.0);
//!     footer.pane(ui, Body::Content, |ui| ui.label("Body"));
//!     footer.pane(ui, Body::Content, |ui| ui.label("A second body"));
//! }
//! ```
//!
//! ```
//! use splat_design::pane::{Body, Footer};
//! fn once(ui: &mut egui::Ui) {
//!     Footer::new("pane", 400.0).pane(ui, Body::Content, |ui| ui.label("Body"));
//! }
//! ```
//!
//! A body that fills its room names why, from a closed list, through the one constructor that
//! Clippy can refuse; no free words, and no way around `Fills::because`:
//!
//! ```compile_fail,E0308
//! use splat_design::pane::{Body, Footer};
//! fn editor(ui: &mut egui::Ui) {
//!     Footer::new("pane", 400.0).pane(ui, Body::Fills("an editor"), |_| ());
//! }
//! ```
//!
//! ```compile_fail,E0423
//! use splat_design::pane::{Body, Fills, FillsReason, Footer};
//! fn editor(ui: &mut egui::Ui) {
//!     Footer::new("pane", 400.0).pane(ui, Body::Fills(Fills(FillsReason::MarkdownEditor)), |_| ());
//! }
//! ```
//!
//! ```
//! use splat_design::pane::{Body, Fills, FillsReason, Footer};
//! fn editor(ui: &mut egui::Ui) {
//!     let body = Body::Fills(Fills::because(FillsReason::MarkdownEditor));
//!     Footer::new("pane", 400.0).pane(ui, body, |_| ());
//! }
//! ```
//!
//! A caller cannot change what a pane measured: the record's fields are private, read only.
//!
//! ```compile_fail,E0616
//! fn move_end(record: &mut splat_design::pane::PaneRecord) {
//!     record.offset = 0.0;
//! }
//! ```
//!
//! ```
//! fn read_end(record: &mut splat_design::pane::PaneRecord) {
//!     let _ = record.offset();
//! }
//! ```
//!
//! Stable rustdoc does not check the error codes above, so each failing case differs from the
//! compiling case after it only by the rule it shows.

pub use super::components::Footer;
use super::{layout::ScrollColumn, tokens::S};

/// Composite columns and their edges, resolved before the pane paints (layout step 7).
/// A line of a list whose rows vary in height, as a recipe (layout 11c, issue #683).
mod action_line;
mod columns;
/// A block laid out before paint by a design recipe (layout 11c).
mod laid;
/// A notice's row in a list (layout 11c scope C): the Trackers panel's Due rows.
mod notice_row;
/// The whole-block plan of a body and its index (layout step 3a; step 3b draws from it).
mod plan;
/// The Session plan's closed row as a recipe (layout 11c A2b).
mod plan_row;
/// The scroll region of an owned list body over its plan (layout step 3b).
mod region;
pub use action_line::{ActionLine, LineColumns, slot as line_slot};
#[cfg(any(test, feature = "test-support"))]
pub use columns::rejected_edges;
pub use columns::{PaneColumns, PaneEdges};
use egui::{self, Id, LayerId, Rect, Sense, Ui, pos2, vec2};
pub use laid::{Laid, LaidUi, draw_laid};
pub use notice_row::{NoticeRoll, NoticeRow, check_height, slot as notice_slot};
pub use plan::{ACTION_ROW, Block, ENTRY_ROW_HEIGHT, Measured, Row};
pub use plan_row::{PlanColumns, PlanRow, SummaryPart, rule as plan_rule, slot as plan_slot};
pub use region::{BlockRegion, Navigation, RegionState};

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
    /// The run's Arrivals list, with its own scroll under the panel's footer. Its group blocks
    /// hold their members and faults: the pane's column would hide a whole block that the
    /// footer's edge cuts (N12), so the list keeps a plain cut until its blocks are rows.
    ArrivalsList,
}

/// How a `Body::Content` pane's scroll column behaves beyond the default, for `Footer::pane_content`.
/// The content stays the pane's scroll column, and the footer follows short content as for
/// `Body::Content`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ContentScroll {
    from_top: bool,
    holds: bool,
    reveals: bool,
}

impl ContentScroll {
    /// R46: the pane's column may host a reveal (`ScrollColumn::hosts_reveals`).
    pub fn hosts_reveals(mut self) -> Self {
        self.reveals = true;
        self
    }

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
/// use splat_design::pane::Footer;
/// fn two(ui: &mut egui::Ui, a: &mut splat_design::text_draft::TextDraft, b: &mut splat_design::text_draft::TextDraft) {
///     const NOTE: splat_design::layout::FieldName = splat_design::field_id!("doc_region_note");
///     Footer::new("pane", 400.0).pane_editor(ui, |ui, region| {
///         region.editor(ui, NOTE.once(splat_design::layout::Once::Example), a, splat_design::pane::Face::Font(egui::FontId::default()), |edit| edit);
///         region.editor(ui, NOTE.once(splat_design::layout::Once::Settings), b, splat_design::pane::Face::Font(egui::FontId::default()), |edit| edit);
///     });
/// }
/// ```
///
/// ```
/// use splat_design::pane::Footer;
/// fn one(ui: &mut egui::Ui, a: &mut splat_design::text_draft::TextDraft) {
///     const NOTE: splat_design::layout::FieldName = splat_design::field_id!("doc_region_note");
///     Footer::new("pane", 400.0).pane_editor(ui, |ui, region| {
///         region.editor(ui, NOTE.once(splat_design::layout::Once::Example), a, splat_design::pane::Face::Font(egui::FontId::default()), |edit| edit);
///         ui.label("A save row");
///     });
/// }
/// ```
///
/// It cannot leave its closure, to be used after the pane:
///
/// ```compile_fail
/// use splat_design::pane::Footer;
/// fn kept(ui: &mut egui::Ui, a: &mut splat_design::text_draft::TextDraft) {
///     const NOTE: splat_design::layout::FieldName = splat_design::field_id!("doc_region_note");
///     let region = Footer::new("pane", 400.0).pane_editor(ui, |_, region| region);
///     region.editor(ui, NOTE.once(splat_design::layout::Once::Example), a, splat_design::pane::Face::Font(egui::FontId::default()), |edit| edit);
/// }
/// ```
///
/// ```
/// use splat_design::pane::Footer;
/// fn used(ui: &mut egui::Ui, a: &mut splat_design::text_draft::TextDraft) -> bool {
///     const NOTE: splat_design::layout::FieldName = splat_design::field_id!("doc_region_note");
///     Footer::new("pane", 400.0)
///         .pane_editor(ui, |ui, region| region.editor(ui, NOTE.once(splat_design::layout::Once::Example), a, splat_design::pane::Face::Font(egui::FontId::default()), |edit| edit))
///         .response
///         .changed()
/// }
/// ```
///
/// Nor can a caller make one, or copy one:
///
/// ```compile_fail,E0451
/// fn made() -> splat_design::pane::EditorRegion<'static> {
///     splat_design::pane::EditorRegion { pane: egui::Id::NULL, max: 100.0, _scope: std::marker::PhantomData }
/// }
/// ```
///
/// ```compile_fail,E0382
/// use splat_design::pane::Footer;
/// fn copied(ui: &mut egui::Ui, a: &mut splat_design::text_draft::TextDraft) {
///     const NOTE: splat_design::layout::FieldName = splat_design::field_id!("doc_region_note");
///     Footer::new("pane", 400.0).pane_editor(ui, |ui, region| {
///         let copy = region;
///         region.editor(ui, NOTE.once(splat_design::layout::Once::Example), a, splat_design::pane::Face::Font(egui::FontId::default()), |edit| edit);
///         let _ = copy;
///     });
/// }
/// ```
pub struct EditorRegion<'r> {
    pane: Id,
    max: f32,
    /// A pane's region, or a viewport of its own (`editor_view`).
    host: Host,
    /// Ties the region to the body closure (invariant in `'r`): it cannot be returned from it.
    _scope: std::marker::PhantomData<fn(&'r ()) -> &'r ()>,
}

/// Where an editor viewport is drawn.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Host {
    /// A `pane_editor` body's region: as tall as the text up to the region's height.
    Pane,
    /// A viewport of its own (`editor_view`): `fills`, as tall as `max` with any text; else as
    /// tall as the text up to `max`.
    Alone { fills: bool },
}

/// How a document editor lays its text out ([`EditorRegion::editor`], [`editor_view`]).
#[derive(Clone, Debug)]
pub enum Face {
    /// Plain text in one font (a note's source in monospace, prose).
    Font(egui::FontId),
    /// Text laid out by a styling layouter (Markdown's source). `folds`: the layout depends on
    /// where the caret and the selection are (a fold opens on their lines), so the kept layout
    /// is keyed by them too.
    Laid {
        layout: fn(&Ui, &dyn egui::TextBuffer, f32) -> std::sync::Arc<egui::Galley>,
        /// The font the layout measures its text in (its prose type): a change lays out again.
        font: egui::FontId,
        folds: bool,
    },
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// How many times a document editor laid out its text (layout PR 9b's quiet-frame proof).
    pub static KEPT_LAYOUTS_MADE: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// The text work of the document editors and their hosts (layout PR 9b and 9b-C), counted from
/// the host's entry through its paint: a host's quiet frames do none of it. A short `String`
/// field keeps its own hash (ruling Q3).
#[cfg(any(test, feature = "test-support"))]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TextWork {
    /// Document editors drawn (`FieldId::editor`): the proof that the host's editor was there.
    pub drawn: usize,
    /// Layouts the kept layouter made.
    pub layouts: usize,
    /// Bytes of documents the field guard hashed.
    pub hashed: usize,
    /// Whole-text compares the drafts made.
    pub compared: usize,
    /// Bytes of words copied (a draft's clone, a host's counted copy).
    pub copied: usize,
}

#[cfg(any(test, feature = "test-support"))]
impl TextWork {
    /// A document editor drew in each of `frames` frames, and nothing else was done.
    pub fn is_quiet(&self, frames: usize) -> bool {
        self.drawn >= frames
            && (self.layouts, self.hashed, self.compared, self.copied) == (0, 0, 0, 0)
    }
}

/// The [`TextWork`] done while `frames` runs.
#[cfg(any(test, feature = "test-support"))]
pub fn text_work(frames: impl FnOnce()) -> TextWork {
    let read = || TextWork {
        drawn: super::layout::DOCUMENTS_DRAWN.with(std::cell::Cell::get),
        layouts: KEPT_LAYOUTS_MADE.with(std::cell::Cell::get),
        hashed: super::layout::DRAFT_TEXT_BYTES.with(std::cell::Cell::get),
        compared: super::text_draft::WORDS_COMPARED.with(std::cell::Cell::get),
        copied: super::text_draft::WORDS_COPIED.with(std::cell::Cell::get),
    };
    let before = read();
    frames();
    let after = read();
    TextWork {
        drawn: after.drawn - before.drawn,
        layouts: after.layouts - before.layouts,
        hashed: after.hashed - before.hashed,
        compared: after.compared - before.compared,
        copied: after.copied - before.copied,
    }
}

/// Everything a kept layout of a document was made from (layout PR 9b, note v2 §3): the
/// draft's identity and revision, the measure (width, font, fonts' generation, which the theme's
/// installation also raises, scale), the font atlas's epoch, the light or dark visuals, the face,
/// and for a face that folds, the caret and the selection of a focused field.
#[derive(Clone, PartialEq)]
struct LaidKey {
    draft: super::text_draft::DraftKey,
    measure: super::memo::Measure,
    atlas: u64,
    dark: bool,
    /// The face's layout (the font is in `measure`): 0 for plain text, else the function.
    face: usize,
    caret: Option<(bool, Option<(usize, usize)>)>,
}

/// The kept layouts of the document editors, one per field. egui runs [`update`] at the end of
/// each pass (`CacheStorage::update`), also a pass that draws no document: a field not drawn in
/// a pass loses its kept galley then (note v2 §3, eviction), so a closed editor keeps no text.
///
/// [`update`]: egui::cache::CacheTrait::update
#[derive(Default)]
struct KeptLayouts {
    kept: egui::IdMap<KeptLayout>,
}

struct KeptLayout {
    key: LaidKey,
    galley: std::sync::Arc<egui::Galley>,
    /// Which layout this is: new for each galley the kept layouter makes ([`kept_layout_made`]).
    made: u64,
    /// Drawn in this pass.
    used: bool,
}

/// Which kept galley the document editor `field` (its text edit's id) showed last: a number new
/// for each galley the part lays out, so a host keeps its own work per laid-out text (a
/// Markdown source's rules and fold marks) by it, exactly. None while the field keeps none.
pub fn kept_layout_made(ctx: &egui::Context, field: Id) -> Option<u64> {
    ctx.memory_mut(|memory| {
        memory
            .caches
            .cache::<KeptLayouts>()
            .kept
            .get(&field)
            .map(|kept| kept.made)
    })
}

impl egui::cache::CacheTrait for KeptLayouts {
    fn update(&mut self) {
        self.kept.retain(|_, kept| std::mem::take(&mut kept.used));
    }

    fn len(&self) -> usize {
        self.kept.len()
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// The layouter of a document editor: the laid-out text kept for its [`LaidKey`]. egui calls
/// it with the text it was given and again after each edit in its own pass (several in one
/// frame too); each call reads the draft's live key (`text_draft::Lent`), so an edited text is
/// never answered with the galley of another revision.
pub(in crate::design) fn kept_layouter<'a>(
    field: Id,
    face: Face,
    live: &'a std::cell::Cell<super::text_draft::DraftKey>,
) -> impl FnMut(&Ui, &dyn egui::TextBuffer, f32) -> std::sync::Arc<egui::Galley> + 'a {
    move |ui: &Ui, text: &dyn egui::TextBuffer, wrap: f32| {
        let font = match &face {
            Face::Font(font) => font.clone(),
            Face::Laid { font, .. } => font.clone(),
        };
        let caret = match &face {
            Face::Laid { folds: true, .. } => {
                let focused = ui.memory(|memory| memory.has_focus(field));
                let range = egui::TextEdit::load_state(ui.ctx(), field)
                    .and_then(|state| state.cursor.char_range())
                    .map(|range| (range.primary.index, range.secondary.index));
                Some((focused, range))
            }
            _ => None,
        };
        let key = LaidKey {
            draft: live.get(),
            measure: super::memo::Measure::of(ui, wrap, &font),
            atlas: super::theme::atlas_epoch(ui.ctx()),
            dark: ui.visuals().dark_mode,
            face: match &face {
                Face::Font(_) => 0,
                Face::Laid { layout, .. } => *layout as usize,
            },
            caret,
        };
        let hit = ui.memory_mut(|memory| {
            let kept = memory.caches.cache::<KeptLayouts>().kept.get_mut(&field)?;
            kept.used = true;
            (kept.key == key).then(|| kept.galley.clone())
        });
        if let Some(galley) = hit {
            return galley;
        }
        #[cfg(any(test, feature = "test-support"))]
        KEPT_LAYOUTS_MADE.with(|made| made.set(made.get() + 1));
        let galley = match &face {
            Face::Font(_) => {
                let color = ui
                    .visuals()
                    .override_text_color
                    .unwrap_or_else(|| ui.visuals().widgets.inactive.text_color());
                let job =
                    egui::text::LayoutJob::simple(text.as_str().to_owned(), font, color, wrap);
                ui.fonts_mut(|fonts| fonts.layout_job(job))
            }
            Face::Laid { layout, .. } => layout(ui, text, wrap),
        };
        ui.memory_mut(|memory| {
            static MADE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
            let kept = KeptLayout {
                key,
                galley: galley.clone(),
                made: MADE.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
                used: true,
            };
            memory
                .caches
                .cache::<KeptLayouts>()
                .kept
                .insert(field, kept);
        });
        galley
    }
}

/// How tall an editor viewport of its own is ([`editor_view`]).
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ViewHeight {
    /// As tall as its text, up to this outside height; then its text scrolls inside the frame.
    AtMost(f32),
    /// The rest of the `Ui`'s height, with any text (an editor that fills its pane's body).
    Fills,
}

/// An editor viewport of its own, outside a `pane_editor` body: the same closed part as
/// [`EditorRegion::editor`], with the same fixed frame, inner text scroll and lane, caret
/// follow and reveal. For an editor that a page's own scroll holds (Campaign's repair) or that
/// fills its pane's body (a Session template). Keyed by `field`'s id.
pub fn editor_view(
    ui: &mut Ui,
    height: ViewHeight,
    field: super::layout::FieldId,
    text: &mut super::text_draft::TextDraft,
    face: Face,
    style: impl for<'t> FnOnce(egui::TextEdit<'t>) -> egui::TextEdit<'t>,
) -> super::layout::Editor {
    let (max, fills) = match height {
        ViewHeight::AtMost(most) => (most, false),
        ViewHeight::Fills => (ui.available_height(), true),
    };
    EditorRegion {
        pane: field.id().with("editor_view"),
        max: max.floor().max(0.0),
        host: Host::Alone { fills },
        _scope: std::marker::PhantomData,
    }
    .editor(ui, field, text, face, style)
}

/// An editor region's focused caret in the last pass, in its content (rows), with that pass's
/// view (its height and width) and offset.
#[derive(Clone, Copy, Debug)]
struct KeptCaret {
    top: f32,
    bottom: f32,
    view: f32,
    width: f32,
    offset: f32,
}

/// A named request to an editor viewport's one offset writer ([`jump_editor_view`]).
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum EditorJump {
    /// Open at the text's top: a newly selected document (a Session note).
    Top,
    /// Bring this part of the text into view, centred: `top` and `bottom` in the text's own
    /// coordinates (its galley's, as [`super::layout::Editor::galley`] lays it out). For a place
    /// the program wrote to while the field may have no focus (the plan's rewards in a note).
    Show { top: f32, bottom: f32 },
}

/// Ask the editor viewport of `field` (an [`editor_view`]) for one offset at the start of its
/// next pass: its one offset writer applies it before the caret follow, and a reveal that was
/// pending gives way. The owner asks once per its own event (a selection change, a write), never
/// every frame; a request that no pass drew yet waits for the next one that does.
pub fn jump_editor_view(ctx: &egui::Context, field: super::layout::FieldId, jump: EditorJump) {
    ctx.data_mut(|data| data.insert_temp(editor_jump_id(field.id().with("editor_view")), jump));
}

/// Where an editor region keeps its pending jump: by the region's own key.
fn editor_jump_id(pane: Id) -> Id {
    pane.with("editor_region").with("jump")
}

/// The fixed frame of every editor viewport: the field's well, its hairline and an `S` inner
/// margin. The text and the scroll lane are inside it.
fn editor_frame() -> egui::Frame {
    egui::Frame::new()
        .fill(super::tokens::colors().well)
        .stroke(egui::Stroke::new(
            super::tokens::HAIRLINE_WIDTH,
            super::tokens::colors().field_border,
        ))
        .corner_radius(super::tokens::RADIUS_CONTROL)
        .inner_margin(S)
}

/// What an editor viewport drew in one pass, for checks: its frame, the clip it was drawn in,
/// its inner text view, the right edge of its scroll lane, and whether the region was held at
/// its least height (`REGION_LEAST`; the pane then scrolls as a whole, a counted state).
#[derive(Clone, Copy, Debug)]
pub struct EditorFrameRecord {
    field: Id,
    frame: Rect,
    clip: Rect,
    view: Rect,
    least: bool,
    offset: f32,
}

impl EditorFrameRecord {
    /// The editor's field id.
    pub fn field(&self) -> Id {
        self.field
    }

    /// The fixed outside frame.
    pub fn frame(&self) -> Rect {
        self.frame
    }

    /// The clip the frame was drawn in: the frame is whole when this holds it.
    pub fn clip(&self) -> Rect {
        self.clip
    }

    /// The inner scroll view (the text and its lane), inside the frame.
    pub fn view(&self) -> Rect {
        self.view
    }

    /// The region was held at its least height: the pane scrolls as a whole, and the frame
    /// may be cut by the pane's own scroll (the 64 px fallback, ruling L3; counted).
    pub fn least(&self) -> bool {
        self.least
    }

    /// The text's scroll offset in that pass.
    pub fn offset(&self) -> f32 {
        self.offset
    }

    /// All four outside edges are in view.
    pub fn whole(&self) -> bool {
        self.clip.expand(0.5).contains_rect(self.frame)
    }
}

#[cfg(any(test, feature = "test-support"))]
const EDITOR_FRAMES: &str = "design_editor_frames";

/// Each editor viewport of the last completed pass, in the order drawn. Test builds only.
#[cfg(any(test, feature = "test-support"))]
pub fn editor_frames(ctx: &egui::Context) -> Vec<EditorFrameRecord> {
    let last = ctx.cumulative_pass_nr().saturating_sub(1);
    ctx.data(|data| data.get_temp::<(u64, Vec<EditorFrameRecord>)>(Id::new(EDITOR_FRAMES)))
        .filter(|(pass, _)| *pass == last)
        .map(|(_, frames)| frames)
        .unwrap_or_default()
}

/// The least height of an editor region's text view: a pane too small for its content and this
/// much of the editor scrolls as a whole, as `Body::Content` does.
const REGION_LEAST: f32 = super::tokens::XXL * 2.0;

/// The fixed frame's height around the text view: its inner margin and its stroke, top and
/// bottom (`editor_frame`).
const FRAME_HEIGHT: f32 = 2.0 * (S + super::tokens::HAIRLINE_WIDTH);

/// The region's height (its frame's) for a room of `room` with `rest` around it (unknown before
/// the first measure: the whole room, and the pass is drawn again). Its text view keeps at
/// least `REGION_LEAST`.
fn region_height(room: f32, rest: Option<f32>) -> f32 {
    (room - rest.unwrap_or(0.0))
        .floor()
        .max(REGION_LEAST.min(room) + FRAME_HEIGHT)
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
    ///
    /// The part draws the editor's fixed frame (I10): all four outside edges stay put, and the
    /// text and the scroll lane scroll inside it. The frame takes the whole width it is given,
    /// so a save row under it ends at the same outside right edge. `style` sets the text's face;
    /// the part sets the frame, the margin and the width, so a caller cannot frame the text
    /// inside the scroll or put the lane outside the frame.
    ///
    /// A focused caret that moved since the last pass (set from code, for example to a broken
    /// line), or the first caret after the editor was not drawn, and lies outside the view is
    /// revealed, centred and with no animation, in the next pass; a caret that did not move is
    /// never scrolled to, so the DM's own wheel stays.
    pub fn editor(
        self,
        ui: &mut Ui,
        field: super::layout::FieldId,
        text: &mut super::text_draft::TextDraft,
        face: Face,
        style: impl for<'t> FnOnce(egui::TextEdit<'t>) -> egui::TextEdit<'t>,
    ) -> super::layout::Editor {
        let pane = innermost(ui);
        assert!(
            self.host != Host::Pane
                || pane.is_some_and(|open| open.pane == self.pane && open.content)
                || !cfg!(debug_assertions),
            "an editor region is drawn in its own pane's body, not in a pane inside it"
        );
        let key = self.pane.with("editor_region");
        let frame = editor_frame();
        let inset = frame.total_margin();
        // The frame is the region; the text's view is the frame less its margins and stroke.
        let max = (self.max - inset.sum().y).max(0.0);
        let least = self.host == Host::Pane && self.max <= REGION_LEAST + FRAME_HEIGHT + 0.5;
        let top = ui.cursor().top();
        // The focused caret of the last pass (`KeptCaret`). The region is the one writer of its
        // offset, and it writes at most once per pass, before the pass lays out, from the view's
        // height NOW (no animation: the first frame shown has the caret in view):
        // - a named jump (`jump_editor_view`);
        // - a caret that was in view, did not move in the text, and lies outside the view after
        //   the last pass laid out again at another width or height (the footer gained or lost
        //   a line, the window or the column got narrower or shorter), in a pass that applied no
        //   jump: the least scroll that keeps it in view, measured in that new layout; a caret
        //   the DM scrolled away from stays away (their wheel is never pulled back);
        // - a reveal is pending (the caret moved outside the view in the last pass): the caret
        //   centred in the view as tall as it is now.
        // Otherwise nothing, so the DM's own wheel is never pulled back.
        /// The editor region's one offset for a pass: `caret` (content rows) was seen at `offset` in
        /// the last pass; `view` is the region's height NOW. A pending `reveal` centres the caret;
        /// else `keep` gives the least scroll that keeps the caret in view. `None`: leave the
        /// offset as it is.
        fn follow_caret(
            caret: std::ops::Range<f32>,
            offset: f32,
            view: f32,
            reveal: bool,
            keep: bool,
        ) -> Option<f32> {
            if !(reveal || keep) {
                return None;
            }
            let at = if reveal {
                (caret.start + caret.end - view) / 2.0
            } else if caret.end > offset + view {
                caret.end - view
            } else if caret.start < offset {
                caret.start
            } else {
                offset
            }
            .max(0.0);
            ((at - offset).abs() > 0.5).then_some(at)
        }
        let caret_key = key.with("caret");
        let reveal_key = key.with("reveal");
        let kept = ui.ctx().data(|data| data.get_temp::<KeptCaret>(caret_key));
        let reveal = ui
            .ctx()
            .data_mut(|data| data.remove_temp::<bool>(reveal_key))
            .unwrap_or(false);
        let keep_key = key.with("keep");
        let keep = ui
            .ctx()
            .data_mut(|data| data.remove_temp::<bool>(keep_key))
            .unwrap_or(false);
        // The caret was in view in the pass before (by that pass's offset and view).
        let was_in_view = kept.is_some_and(|kept| {
            kept.top >= kept.offset - 0.5 && kept.bottom <= kept.offset + kept.view + 0.5
        });
        // A named jump (`jump_editor_view`) is this pass's one offset write; a pending reveal
        // gives way to it.
        let jump = ui.ctx().data_mut(|data| {
            let id = editor_jump_id(self.pane);
            let jump = data.get_temp::<EditorJump>(id);
            data.remove::<EditorJump>(id);
            jump
        });
        let fills = self.host == Host::Alone { fills: true };
        let mut column = ScrollColumn::new(key)
            .max_height(max)
            .min_scrolled_height(max);
        if !fills {
            column = column.fit_height();
        }
        let at = match jump {
            Some(EditorJump::Top) => Some(0.0),
            Some(EditorJump::Show { top, bottom }) => Some(((top + bottom - max) / 2.0).max(0.0)),
            None => kept.and_then(|kept| {
                follow_caret(kept.top..kept.bottom, kept.offset, max, reveal, keep)
            }),
        };
        if let Some(at) = at {
            column = column.vertical_scroll_offset(at);
        }
        let usable = pane.map_or(ui.clip_rect(), |open| open.usable);
        let pane_id = self.pane;
        let shown = frame.show(ui, |ui| {
            // The frame takes the whole width: its outside right edge is the column's.
            ui.set_min_width(ui.available_width());
            column.show_in_region(ui, |ui| {
                // A scroll inside the region is refused, as in a `Content` body.
                open(
                    ui,
                    Some(OpenPane {
                        layer: ui.layer_id(),
                        content: true,
                        usable,
                        edge: false,
                        pane: pane_id,
                        measure: None,
                        extent: None,
                    }),
                );
                // Where the content starts in this layout (the view's top less the offset used).
                let origin = ui.max_rect().top();
                let least_text = if fills { max } else { 0.0 };
                let editor = field.editor(ui, text, face, |edit| {
                    style(edit)
                        .frame(false)
                        .margin(egui::Vec2::ZERO)
                        .desired_width(f32::INFINITY)
                        .min_size(vec2(0.0, least_text))
                });
                // The editor asks its scroll area to follow a caret that moved (egui's
                // `scroll_to_rect`, animated, to the nearest edge). The region follows the caret
                // itself, so that ask is replaced by one that moves nothing: no animated target
                // competes with the region's offset in a later pass.
                ui.scroll_to_rect_animation(
                    Rect::from_center_size(ui.clip_rect().center(), egui::Vec2::ZERO),
                    None,
                    egui::style::ScrollAnimation::none(),
                );
                open(ui, None);
                (editor, origin)
            })
        });
        let output = shown.inner;
        let frame_rect = shown.response.rect;
        let pass = ui.ctx().cumulative_pass_nr();
        // What the region took in the body, as the body laid it out (the scroll output's view
        // is not always what the body allocated).
        let height = (ui.cursor().top() - top - ui.spacing().item_spacing.y).max(0.0);
        let (mut editor, origin) = output.inner;
        // The view egui allocated: the area's inner rect is its most; a fitted view is as tall as
        // its content.
        let view = Rect::from_min_size(
            output.inner_rect.min,
            vec2(
                output.inner_rect.width(),
                output
                    .inner_rect
                    .height()
                    .min(if fills { max } else { output.content_size.y }),
            ),
        );
        editor.frame = frame_rect;
        editor.view = view;
        // The pane's focus arbiter reveals the fixed frame, not the text edit (as tall as all its
        // text): a click into a whole frame moves nothing.
        super::components::reveal_as(ui, editor.response.id, frame_rect);
        // The text is clipped to the part of it in view: a caller that paints over the text
        // from outside the scroll (a rule, a mark) stays inside the frame's view.
        editor.text_clip_rect = editor.text_clip_rect.intersect(view);
        if editor.response.has_focus() {
            super::components::focus_ring(ui, frame_rect);
        }
        // Keep the focused caret for the next pass, in content coordinates.
        let caret = editor
            .response
            .has_focus()
            .then_some(())
            .and(editor.cursor_range.map(|range| {
                editor
                    .galley
                    .pos_from_cursor(range.primary)
                    .translate(editor.galley_pos.to_vec2())
            }));
        // A caret that moved since the last pass (set from code, a key, typed text, or the first
        // pass with focus) and lies outside the view: a reveal for the next pass, drawn again at
        // once. One that did not move is never scrolled to, so the DM's own wheel is not pulled
        // back. The editor did not draw in the pass before (the pane showed something else, as
        // Read does): its kept caret is old, and a caret now is a new one.
        let drawn_key = key.with("drawn");
        let drawn_before = ui
            .ctx()
            .data_mut(|data| {
                let before = data.get_temp::<u64>(drawn_key);
                data.insert_temp(drawn_key, pass);
                before
            })
            .is_some_and(|before| before + 1 >= pass);
        // The caret's place in the text, kept beside its position: the text laid out again at
        // another width (a window or column change) moves the caret's pixels, not the caret.
        let index_key = key.with("caret_index");
        let index = editor
            .response
            .has_focus()
            .then_some(())
            .and(editor.cursor_range.map(|range| range.primary.index));
        let index_before = ui.ctx().data(|data| data.get_temp::<usize>(index_key));
        // The region was laid out at another width or height than in the pass before (the
        // footer gained or lost a line, the window or the column changed size).
        let relaid = kept.is_some_and(|kept| {
            (kept.view - max).abs() > 0.5 || (kept.width - view.width()).abs() > 0.5
        });
        if let Some(caret) = caret {
            let (caret_top, caret_bottom) = (caret.top() - origin, caret.bottom() - origin);
            let moved = !drawn_before
                || editor.response.changed()
                || kept.is_none()
                || index_before != index;
            let offset = output.state.offset.y;
            let outside = caret_top < offset - 0.5 || caret_bottom > offset + max + 0.5;
            if moved && outside {
                ui.ctx().data_mut(|data| data.insert_temp(reveal_key, true));
                ui.ctx()
                    .request_discard("an editor region reveals a caret outside its view");
                ui.ctx().request_repaint();
            } else if was_in_view && outside && relaid && jump.is_none() {
                // Laid out again (narrower, shorter), the caret that was in view fell out of it:
                // keep it, by the least scroll, in the next pass, drawn again at once. Only for a
                // new layout: the DM's wheel that takes the caret out of view, and a named jump,
                // are this pass's place, and stay (review of PR 825).
                ui.ctx().data_mut(|data| data.insert_temp(keep_key, true));
                ui.ctx()
                    .request_discard("an editor region keeps its caret in view");
                ui.ctx().request_repaint();
            }
        }
        ui.ctx().data_mut(|data| match caret {
            Some(caret) => {
                data.insert_temp(
                    caret_key,
                    KeptCaret {
                        top: caret.top() - origin,
                        bottom: caret.bottom() - origin,
                        view: max,
                        width: view.width(),
                        offset: output.state.offset.y,
                    },
                );
            }
            None => data.remove::<KeptCaret>(caret_key),
        });
        ui.ctx().data_mut(|data| match index {
            Some(index) => data.insert_temp(index_key, index),
            None => data.remove::<usize>(index_key),
        });
        if self.host == Host::Pane {
            ui.ctx().data_mut(|data| {
                data.insert_temp(region_drawn_id(self.pane), (pass, height));
            });
        }
        #[cfg(any(test, feature = "test-support"))]
        {
            let record = EditorFrameRecord {
                field: editor.response.id,
                frame: frame_rect,
                clip: ui.clip_rect(),
                view,
                least,
                offset: output.state.offset.y,
            };
            ui.ctx().data_mut(|data| {
                let kept = data.get_temp_mut_or_default::<(u64, Vec<EditorFrameRecord>)>(Id::new(
                    EDITOR_FRAMES,
                ));
                if kept.0 != pass {
                    *kept = (pass, Vec::new());
                }
                kept.1.push(record);
            });
        }
        #[cfg(not(any(test, feature = "test-support")))]
        let _ = least;
        editor
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

/// Whether the footer `id` followed its content in the last pass (`Footer::pane`): its top
/// stays and it grows down, so an answer to a press of one of its lines stands under that line.
pub(super) fn footer_follows(ctx: &egui::Context, id: Id) -> bool {
    ctx.data(|data| data.get_temp::<bool>(id.with("follows")))
        .unwrap_or(false)
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
    /// Inside a child of composite columns (`Columns`): where its scroll columns add the height
    /// they hide, so the composite knows each child's whole height (`columns::Measure`).
    measure: Option<Id>,
    /// A `Content` body: its content's top in this pass (in the scrolled layout) and the height
    /// of its room, so a part in it knows the room left below a point, whatever the offset.
    extent: Option<(f32, f32)>,
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
    let inside = in_content_pane(ui);
    assert!(
        !(inside && cfg!(debug_assertions)),
        "a ScrollColumn inside a `Body::Content` pane: the pane is the scroll column. Give its \
         content to the pane, draw a header that must not scroll before the pane, or use \
         `Body::Fills(Fills::because(FillsReason::Columns))` for columns that each scroll"
    );
    inside
}

/// Whether `ui` draws in a `Body::Content` pane's body, which scrolls (a region there draws its
/// blocks in place: `BlockRegion`).
pub(super) fn in_content_pane(ui: &Ui) -> bool {
    let (layer, pass) = (ui.layer_id(), ui.ctx().cumulative_pass_nr());
    ui.ctx().data(|data| {
        data.get_temp::<Open>(open_id()).is_some_and(|open| {
            open.pass == pass
                && open
                    .panes
                    .last()
                    .is_some_and(|pane| pane.layer == layer && pane.content)
        })
    })
}

/// For `ScrollColumn`: a column drawn in a child of composite columns adds the height its view
/// hides to that child's whole height (`Columns`), so the composite is measured by its content,
/// never by the room its columns were given.
pub(super) fn hidden_height<R>(ui: &Ui, output: &egui::scroll_area::ScrollAreaOutput<R>) {
    if let Some(slot) = innermost(ui).and_then(|pane| pane.measure) {
        let hidden = (output.content_size.y - output.inner_rect.height()).max(0.0);
        columns::add_hidden(ui.ctx(), slot, hidden);
    }
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

/// A body control that gained keyboard focus in this pass comes whole into the pane's view, with
/// the words that belong to it (`components::reveal_with`: its field's caption, help and error,
/// and a reason its act brought, [`reveal_with_focus`]), above the footer: Tab never reaches a
/// control hidden under it. The pane is the one arbiter of this focus clearance (I9). It checks
/// again when the visible room changes under the focused control (the footer came or grew, the
/// session banner came, the whole-row cut came with it; review of PR 306), and when the focused
/// field itself changes with the same focus (layout PR 10: a provider change adds or removes its
/// help, a reason comes, the content above it moves it): it then scrolls only when the field is
/// not whole in view. The DM's own wheel moves neither the room nor the field's place in the
/// content, so it is never pulled back. Only a control that the body added (`widgets`, on the
/// pane's layer) is scrolled to. Nothing moves while the pointer presses, holds or releases; a
/// focus that came with a press and whose block is cut waits for the release, and the block then
/// comes whole into view (a click on a cut field, a drop-down's chosen row). An editor viewport's
/// block is its fixed frame (`components::reveal_as`), not its text.
fn reveal_focus(ui: &mut Ui, id: Id, widgets: std::ops::Range<usize>, room: Rect, visible: Rect) {
    let seen = id.with("seen_focus");
    let layer = ui.layer_id();
    // A control of this body and its rect in this pass, if it is one.
    let in_body = |control: Id| {
        ui.ctx().viewport(|viewport| {
            let widgets_now = &viewport.this_pass.widgets;
            let (at_layer, at) = widgets_now.order(control)?;
            (at_layer == layer && widgets.contains(&at))
                .then(|| widgets_now.get(control).map(|widget| widget.rect))
                .flatten()
        })
    };
    // The subject: the focused control of this body, else the body's control whose list is
    // open (the keyboard is on the list's rows; layout PR 10b).
    let focused = ui.ctx().memory(|memory| memory.focused());
    let (focused, own) =
        match focused.and_then(|focused| in_body(focused).map(|own| (focused, own))) {
            Some((focused, own)) => (Some(focused), Some(own)),
            None => match super::components::open_owner(ui.ctx())
                .and_then(|owner| in_body(owner).map(|own| (owner, own)))
            {
                Some((owner, own)) => (Some(owner), Some(own)),
                None => (focused, None),
            },
        };
    let block = focused
        .zip(own)
        .map(|(focused, rect)| super::components::reveal_rect(ui.ctx(), focused, rect));
    // Where the block is in the content (the content's origin is the column's top less its
    // offset, so a wheel does not move it) and how tall it is, to whole points.
    let origin = ui.max_rect().top();
    let place = block.map(|block| {
        [
            (block.top() - origin).round() as i32,
            block.height().round() as i32,
        ]
    });
    // The room, not the clip: a wheel scroll moves the whole-row cut, and must not pull the
    // focused control back into view.
    let before = ui
        .ctx()
        .data(|data| data.get_temp::<(Option<Id>, Rect, Option<[i32; 2]>)>(seen));
    let pointer = ui.input(|input| {
        let pointer = &input.pointer;
        pointer.any_down() || pointer.any_pressed() || pointer.any_released()
    });
    // Nothing moves while the pointer presses, holds or releases. A focused block that is not
    // whole and fits the view waits, unseen, for the pointer's release, and then comes whole
    // above the footer (I9, layout PR 10b): a click on a field whose caption or control is cut,
    // and a drop-down's chosen row that gives the focus back to its control. A block taller
    // than the view (an editor's text) is never pulled to its top by a click.
    let cut = block
        .is_some_and(|block| !visible.contains_rect(block) && block.height() <= visible.height());
    let key = (focused, room, place);
    // The pointer on the focused control itself: a click on it, also again on a field that
    // already had the focus (layout PR 10c), whose caption the DM's wheel took out of view.
    // The waiting reveal forgets the seen state, so the release reveals the block even with the
    // same focus, room and place; a drag of the scroll bar elsewhere is not a click on it.
    let on_it = own.is_some_and(|own| {
        ui.input(|input| input.pointer.latest_pos())
            .is_some_and(|at| own.contains(at))
    });
    if pointer && cut && (on_it || before != Some(key)) {
        ui.ctx()
            .data_mut(|data| data.remove::<(Option<Id>, Rect, Option<[i32; 2]>)>(seen));
        return;
    }
    if before == Some(key) {
        return;
    }
    ui.ctx().data_mut(|data| data.insert_temp(seen, key));
    // The same control, a new room or a changed field: scroll only when it is not whole in view.
    // A field taller than the view that changed in the same room (an editor that grows as the DM
    // types) keeps its own caret follow: its top is not pulled into view.
    let same = before.is_some_and(|(was, _, _)| was == focused);
    let same_room = before.is_some_and(|(_, was, _)| was == room);
    // A reveal that runs in this column owns its scroll (`design::reveal`).
    if let Some(rect) = block.filter(|_| !pointer)
        && !(same
            && (visible.contains_rect(rect) || (same_room && rect.height() > visible.height())))
        && !super::reveal::holds(ui)
    {
        ui.scroll_to_rect(rect, None);
    }
}

/// Words that belong to the focused control now (a reason that its act brought, at another
/// field): the pane's one focus arbiter ([`Footer::pane`]'s body) brings them into view with the
/// focused field, above the footer, in the pass they come. It asks no scroll of its own, so it
/// never competes with the arbiter or with the DM's wheel. With no focus it does nothing.
pub fn reveal_with_focus(ui: &Ui, words: Rect) {
    if let Some(focused) = ui.ctx().memory(|memory| memory.focused()) {
        super::components::reveal_with(ui, focused, words);
    }
}

/// Room a body keeps for content that an act will add (layout step 11b; ruling R30, D-2). A
/// closed set: each room's height comes from tokens and line heights, never from the content.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Room {
    /// A Log heading and one new entry of at most two prose lines, under a note's body.
    LogEntry,
}

impl Room {
    /// The room's height in `ui`'s fonts.
    pub fn height(self, ui: &Ui) -> f32 {
        match self {
            Self::LogEntry => {
                let prose =
                    ui.fonts_mut(|fonts| fonts.row_height(&super::reader::prose_type(ui).font(ui)));
                super::L + super::Type::Heading.line() + S + 2.0 * prose
            }
        }
    }
}

/// The body's room for content that an act will add, where the body's content ends: blank
/// space of the room's height, drawn here and nowhere else, so the content lands in room
/// already in view and nothing moves. Its rect, for [`reveal_with_focus_and_growth`].
pub fn keep_room(ui: &mut Ui, room: Room) -> Rect {
    let top = ui.cursor().top();
    let height = room.height(ui);
    super::extras::space(ui, height);
    Rect::from_min_max(
        pos2(ui.max_rect().left(), top),
        pos2(ui.max_rect().right(), top + height),
    )
}

/// How much a pane's footer grows when an act ends (layout step 11b). A closed set.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Growth {
    /// The act's result line and its Undo.
    ResultWithUndo,
}

impl Growth {
    /// The footer's growth.
    pub fn height(self) -> f32 {
        match self {
            Self::ResultWithUndo => S + 2.0 * super::Density::Read.control(),
        }
    }
}

/// An act's room, named for its one field (`field`, drawn in this pass): `room` (a
/// [`keep_room`]) and, below it, as much again as the footer will grow when the act ends belong
/// to the field while it has the keyboard. The pane's focus arbiter keeps the field and all of
/// that whole above the footer as it is now, so when the act's content and its result come,
/// nothing moves and nothing is covered. Only the field: another control that takes the
/// keyboard (the field's own Save, a link in the note) is revealed alone. The field too is
/// revealed alone when its block, the room and the growth together are taller than the pane's
/// body room: a rect that spans both edges of the view makes no scroll, and the keyboard's
/// target would stay hidden (Astra's review of PR 736).
pub fn reveal_with_focus_and_growth(ui: &Ui, field: &egui::Response, room: Rect, growth: Growth) {
    if !field.has_focus() {
        return;
    }
    let named = room.with_max_y(room.max.y + growth.height());
    let block = super::components::reveal_rect(ui.ctx(), field.id, field.rect).union(named);
    if body_room(ui).is_none_or(|body| block.height() <= body.height()) {
        super::components::reveal_with(ui, field.id, named);
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
    #[cfg(any(test, feature = "test-support"))]
    ui.ctx()
        .data_mut(|data| data.insert_temp(Id::new("design_pane_hold_ended"), why));
    #[cfg(not(any(test, feature = "test-support")))]
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
#[cfg(any(test, feature = "test-support"))]
const PANE_RECORDS: &str = "design_pane_records";

#[cfg(any(test, feature = "test-support"))]
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
#[cfg(any(test, feature = "test-support"))]
pub fn pane_records(ctx: &egui::Context) -> Vec<PaneRecord> {
    let last = ctx.cumulative_pass_nr().saturating_sub(1);
    ctx.data(|data| data.get_temp::<(u64, Vec<PaneRecord>)>(Id::new(PANE_RECORDS)))
        .filter(|(pass, _)| *pass == last)
        .map(|(_, panes)| panes)
        .unwrap_or_default()
}

/// The record of the pane whose footer the caller made with `Footer::new(local, …)`, in the last
/// completed pass (the first, when several scopes draw one).
#[cfg(any(test, feature = "test-support"))]
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
#[cfg(any(test, feature = "test-support"))]
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
#[cfg(any(test, feature = "test-support"))]
pub fn footer_rect(ctx: &egui::Context, local: Id) -> Option<Rect> {
    let id = pane_record(ctx, local)?.id;
    ctx.read_response(id.with("focus"))
        .map(|response| response.rect)
}

/// The body's clip: what is left of `ui`'s clip above the body's room, and above `row_cut`, the
/// top of the registered row that the footer or the session banner would cut (N12, ruling R4),
/// widened at the left by the most a `line_start` action hangs, inside the parent's clip.
/// The one place that decides where the body is cut. egui clips paint and presses alike, so a
/// row under the cut neither shows nor takes a press.
fn body_clip(ui: &Ui, body_room: Rect, row_cut: Option<f32>) -> Rect {
    let mut clip = ui.clip_rect();
    clip.min.x = hang_left(ui, body_room);
    clip.max.y = clip.max.y.min(body_room.bottom());
    if let Some(cut) = row_cut {
        clip.max.y = clip.max.y.min(cut);
    }
    clip
}

/// The leftmost x that a quiet `line_start` action at `room`'s left (its text edge) may use for
/// its hang (its padding and icon): `LINE_START_REACH` into the margin left of it, never left of
/// what encloses it (the parent's clip: always the intersection, so a pane cut by its parent
/// never widens that clip), and never over a band that a layout part reserved in this pass
/// before this body drew (`reserve`: a `Column`'s splitter band, a `ScrollColumn`'s lane). The
/// one place that decides the hang's room: a pane's body clip and a pane's scroll columns (their
/// surface, their clip and so their presses) all start here. It reads only this pass's
/// reservations, made by the parts that own them, never last pass's widgets; nothing is stored
/// and no content moves: the text edge is the room's left in every pass. Where the room left is
/// shorter than an action's hang, the action stays whole (`Action::paint`).
fn hang_left(ui: &Ui, room: Rect) -> f32 {
    let edge = room.left();
    let parent = ui.clip_rect().left();
    let mut left = parent.max(edge - super::components::LINE_START_REACH);
    // Every band across the hang's span, also one that crosses the text edge (the body's clip
    // stops at the text edge; the action moves clear of the rest, `hang_limit`).
    for band in reserved(ui) {
        if band.left() < edge - 0.5
            && band.right() > left
            && band.top() < room.bottom()
            && band.bottom() > room.top()
        {
            left = band.right();
        }
    }
    left.min(edge).max(parent)
}

/// The bands a layout part keeps clear of its neighbours' `line_start` hangs, in this pass, per
/// layer: a `Column`'s splitter grab band and a `ScrollColumn`'s scroll lane. The part reserves
/// its band before it draws its body, and so before any body beside it (a column comes before
/// the page beside it; a scroll column before the one right of it).
#[derive(Clone, Default)]
struct Reserved {
    pass: u64,
    bands: Vec<(LayerId, Rect)>,
}

fn reserved_id() -> Id {
    Id::new("design_pane_reserved_bands")
}

/// Keep `band` clear of the `line_start` hangs beside it, for this pass (see `hang_left`).
pub(crate) fn reserve(ui: &Ui, band: Rect) {
    let (layer, pass) = (ui.layer_id(), ui.ctx().cumulative_pass_nr());
    ui.ctx().data_mut(|data| {
        let reserved = data.get_temp_mut_or_default::<Reserved>(reserved_id());
        if reserved.pass != pass {
            *reserved = Reserved {
                pass,
                bands: Vec::new(),
            };
        }
        reserved.bands.push((layer, band));
    });
    // A band that comes after a column beside it drew cannot move that column's hangs any more
    // in this pass: the layout owner reserves sibling lanes first (`reserve_lanes`). This is not
    // refused and gives no protection: test builds only record it (`late_bands`).
    let late = ui.ctx().data(|data| {
        data.get_temp::<Limits>(limits_id())
            .filter(|limits| limits.pass == pass)
            .and_then(|limits| {
                limits
                    .columns
                    .iter()
                    .find(|(_, limit, span)| {
                        span.intersects(band)
                            && band.right() > *limit + 0.5
                            && band.left() < span.right()
                    })
                    .map(|(_, limit, span)| (*limit, *span))
            })
    });
    // Test builds keep each one (`late_bands`); the part test asserts it. Today: Lookup
    // (`src/lib.rs:2945`) draws before the Library reader reserves its lane
    // (`src/library.rs:8308`); their common layout owner must reserve before Lookup draws.
    #[cfg(any(test, feature = "test-support"))]
    if let Some((limit, span)) = late {
        ui.ctx().data_mut(|data| {
            data.get_temp_mut_or_default::<Vec<(Rect, f32, Rect)>>(late_id())
                .push((band, limit, span));
        });
    }
    #[cfg(not(any(test, feature = "test-support")))]
    let _ = late;
}

#[cfg(any(test, feature = "test-support"))]
fn late_id() -> Id {
    Id::new("design_pane_late_bands")
}

/// Bands reserved after a column whose hang they cross had drawn (test builds): each band, that
/// column's hang limit and its hang's span.
#[cfg(any(test, feature = "test-support"))]
pub fn late_bands(ctx: &egui::Context) -> Vec<(Rect, f32, Rect)> {
    ctx.data(|data| data.get_temp::<Vec<(Rect, f32, Rect)>>(late_id()))
        .unwrap_or_default()
}

/// For the part that lays out columns side by side in a pane body (a `Fills` body): reserve
/// each column's scroll lane (`columns`: each column's whole rect) before ANY of them draws, so
/// no draw order lets a column's `line_start` hang reach the lane of the column left of it.
pub fn reserve_lanes(ui: &Ui, columns: &[Rect]) {
    let lane = super::layout::lane_width();
    for column in columns {
        reserve(
            ui,
            Rect::from_x_y_ranges((column.right() - lane)..=column.right(), column.y_range()),
        );
    }
}

fn reserved(ui: &Ui) -> Vec<Rect> {
    let (layer, pass) = (ui.layer_id(), ui.ctx().cumulative_pass_nr());
    ui.ctx().data(|data| {
        data.get_temp::<Reserved>(reserved_id())
            .filter(|reserved| reserved.pass == pass)
            .map(|reserved| {
                reserved
                    .bands
                    .iter()
                    .filter(|(at, _)| *at == layer)
                    .map(|(_, band)| *band)
                    .collect()
            })
            .unwrap_or_default()
    })
}

fn body_column_id() -> Id {
    Id::new("design_pane_body_column")
}

/// For `ScrollColumn::show`: inside a pane's body (its own `Content` column, or a column in a
/// `Fills` body), reserve the column's lane and give how far its scroll area starts left of the
/// text edge (`hang_left`); `None` outside a pane, where nothing changes.
pub(super) fn column_reach(ui: &Ui, lane: f32) -> Option<f32> {
    let pass = ui.ctx().cumulative_pass_nr();
    let body = ui
        .ctx()
        .data(|data| data.get_temp::<(u64, Id)>(body_column_id()))
        == Some((pass, ui.id()));
    if !body && innermost(ui).is_none() {
        return None;
    }
    let room = ui.available_rect_before_wrap();
    reserve(
        ui,
        Rect::from_x_y_ranges((room.right() - lane)..=room.right(), room.y_range()),
    );
    Some((room.left() - hang_left(ui, room)).max(0.0))
}

/// For `ScrollColumn::show`: run `area` (the column's scroll area) in a child that starts `reach`
/// left of the room, so the area's surface covers the hang, and take the room it used.
pub(super) fn reaching<R>(ui: &mut Ui, reach: f32, area: impl FnOnce(&mut Ui) -> R) -> R {
    let room = ui.available_rect_before_wrap();
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .id_salt("hang_reach")
            .max_rect(room.with_min_x(room.left() - reach))
            .layout(*ui.layout()),
    );
    let inner = area(&mut child);
    let used = child.min_rect().intersect(room);
    ui.allocate_rect(used, Sense::hover());
    inner
}

/// For `ScrollColumn::show`, inside its scroll area: the content starts `reach` in from the
/// area's own left (its surface), at the text edge, and a `line_start` action in it hangs no
/// further left than that surface (`hang_limit`), so its paint, its presses and the surface
/// agree. Other content keeps the column's clip (a row's divider still meets its splitter).
pub(super) fn inset<R>(ui: &mut Ui, reach: f32, add: impl FnOnce(&mut Ui) -> R) -> R {
    let limit = ui.max_rect().left();
    let inside = ui.max_rect().with_min_x(limit + reach);
    ui.scope_builder(
        egui::UiBuilder::new()
            .id_salt("hang_inset")
            .max_rect(inside)
            .layout(*ui.layout()),
        |ui| {
            // egui's stack names each `Ui` by its unique id.
            let (id, pass) = (ui.stack().id, ui.ctx().cumulative_pass_nr());
            ui.ctx().data_mut(|data| {
                let limits = data.get_temp_mut_or_default::<Limits>(limits_id());
                if limits.pass != pass {
                    *limits = Limits {
                        pass,
                        columns: Vec::new(),
                    };
                }
                let edge = limit + reach;
                let view = ui.clip_rect();
                let span = Rect::from_x_y_ranges(
                    (edge - super::components::LINE_START_REACH)..=edge,
                    view.y_range(),
                );
                limits.columns.push((id, limit, span));
            });
            add(ui)
        },
    )
    .inner
}

/// The hang limit of each pane column drawn in this pass, by its content `Ui`'s id.
#[derive(Clone, Default)]
struct Limits {
    pass: u64,
    /// Each column's content `Ui`, its hang limit and its hang's span (left of its text edge).
    columns: Vec<(Id, f32, Rect)>,
}

fn limits_id() -> Id {
    Id::new("design_pane_hang_limits")
}

/// The leftmost x a `line_start` action drawn in `ui`, whose hang covers `span` (from its
/// padding's left to its text edge), may paint or take a press at: `ui`'s clip; inside a pane's
/// column no further left than the column's surface (`inset`, the nearest such column up `ui`'s
/// stack); and right of every band a layout part reserved across `span`, also one that crosses
/// the text edge (the action then moves past it).
pub(crate) fn hang_limit(ui: &Ui, span: Rect) -> f32 {
    let clip = ui.clip_rect().left();
    let pass = ui.ctx().cumulative_pass_nr();
    let column = ui.ctx().data(|data| {
        let limits = data
            .get_temp::<Limits>(limits_id())
            .filter(|limits| limits.pass == pass)?;
        let mut at = Some(ui.stack().clone());
        while let Some(frame) = at {
            if let Some((_, limit, _)) = limits.columns.iter().find(|(id, ..)| *id == frame.id) {
                return Some(*limit);
            }
            at = frame.parent.clone();
        }
        None
    });
    let mut limit = column.map_or(clip, |limit| clip.max(limit));
    for band in reserved(ui) {
        if band.intersects(span) && band.right() > limit {
            limit = band.right();
        }
    }
    limit
}

/// The top of the lowest registered row that the viewport's bottom edge cuts (its top above the
/// edge, its bottom below), unless that row starts at the viewport's top (one row taller than the
/// room keeps the plain cut). Prose and notes are not rows: they keep a line cut. Rows side by
/// side (a sheet's two columns) overlap in height, so a cut moved up for one row can cross a row
/// of the other column: the cut moves again, until no row crosses it (layout step 11a, Astra's
/// P1 on PR 700). Each move goes up by more than half a pixel, so it ends.
fn whole_row_cut(rows: &[Rect], viewport: Rect) -> Option<f32> {
    let crossing = |edge: f32| {
        rows.iter()
            .filter(|row| {
                row.top() < edge - 0.5
                    && row.bottom() > edge + 0.5
                    && row.top() > viewport.top() + 0.5
            })
            .map(|row| row.top())
            .reduce(f32::min)
    };
    let mut cut = None;
    while let Some(top) = crossing(cut.unwrap_or(viewport.bottom())) {
        cut = Some(top);
    }
    cut
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

/// The content of a `ScrollColumn::follows_focus` column outside a pane (the session banner): a
/// control of it that gains keyboard focus comes whole into the column's view, by the rules of
/// the pane's one focus arbiter ([`reveal_focus`]: once per focus, room or place; never while the
/// pointer presses; the DM's wheel is not pulled back). In a pane's body the pane is that
/// arbiter, so it only runs `add`; so it does without `follows`.
pub(super) fn follow_focus<R>(
    ui: &mut Ui,
    follows: bool,
    column: Id,
    add: impl FnOnce(&mut Ui) -> R,
) -> R {
    if !follows || innermost(ui).is_some() {
        return add(ui);
    }
    let first = widget_count(ui);
    let inner = add(ui);
    let last = widget_count(ui);
    let view = ui.clip_rect();
    reveal_focus(
        ui,
        column.with("design_column_focus"),
        first..last,
        view,
        view,
    );
    inner
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
    let first = widget_count(ui);
    let inner = add(ui);
    let last = widget_count(ui);
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
    // The column's visible room: a focused control of this column that the new room hides (a
    // footer line came and its cut crossed the focused row) comes whole into view, as in a
    // `Content` body; the column scrolls by the least distance.
    let mut visible = view;
    if let Some(cut) = cut {
        visible.max.y = visible.max.y.min(cut);
    }
    reveal_focus(ui, key, first..last, view, visible);
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

/// A footer that empties forgets its measured height and the words it was measured with.
fn forget(ui: &Ui, id: Id) {
    ui.ctx().data_mut(|data| {
        data.remove::<f32>(id.with("height"));
        data.remove::<u64>(id.with("words"));
    });
}

/// Where the next pass, measured, places a footer of `demand` (`place`).
struct Next {
    placement: Placement,
    rect: Rect,
    /// The most it may take where it stands (Q-c): the room to the pane's end, or a third.
    most: f32,
    /// The body's room, and a `Content` body's scroll output in it.
    body_room: Rect,
    view: Option<Scroll>,
}

/// The body's room ends `S` above the footer's height, a third of the pane at most; the footer
/// follows the content when the content fits that room, else it is pinned at the pane's bottom.
/// The content is laid out the same in that room: `scroll` is this pass's output.
fn next_place(room: Rect, width: f32, demand: f32, third: f32, scroll: Option<Scroll>) -> Next {
    let height = demand.min(third);
    let limit = if height > 0.0 {
        room.bottom() - height - S
    } else {
        room.bottom()
    };
    let body_room = Rect::from_min_max(room.min, pos2(room.right(), limit.max(room.top())));
    let view = scroll.map(|scroll| Scroll {
        viewport: Rect::from_min_max(
            scroll.viewport.min,
            pos2(
                scroll.viewport.right(),
                body_room.bottom().max(scroll.viewport.top()),
            ),
        ),
        ..scroll
    });
    let (placement, top, most) = match view {
        Some(view) if view.fits() => {
            let top = (view.viewport.top() + view.height + S).min(room.bottom() - height);
            (Placement::Follows, top, room.bottom() - top)
        }
        _ => (Placement::Pinned, room.bottom() - height, third),
    };
    Next {
        placement,
        rect: Rect::from_min_size(pos2(room.left(), top), vec2(width, demand.min(most))),
        most,
        body_room,
        view,
    }
}

/// A typed footer whose lines came after the body (`pane_then`), drawn where the next pass places
/// it (`next_place`) after the body was laid out for another demand.
struct Stop {
    id: Id,
    layer: LayerId,
    room: Rect,
    width: f32,
    next: Next,
    /// The body's paint, and its widgets on the pane's layer (their places in egui's list).
    body: (usize, usize),
    widgets: (usize, usize),
    /// The row cut's hairline that the pass drew for the body's room, if any.
    cut_line: Option<usize>,
    rows_before: usize,
}

/// The body stops where the next pass's body room ends (`Stop`): at that room's end, or at its
/// last whole registered row there (N12) with that row cut's hairline, which the next pass keeps,
/// so it is drawn the same; a line cut's hairline at the footer's top. Nothing moves: the footer
/// was drawn once, in its place. The edge holds for presses and for a screen reader as for the
/// paint (review of PR 456, P2): each body widget that reaches below it takes presses only above
/// it from now on (re-registered with its rect cut at the edge, for hover only: egui keeps the
/// last rect, so its press area ends there, and its focus order, accessible node and id stay), and one wholly below it is hidden from a
/// screen reader. The focused control keeps both: the next pass, in the new room, brings it whole
/// into view (`reveal_focus`: the room changed under it). Returns whether content goes on below
/// the body's room. The tooltip of a control it hides goes in that pass too (PR 456's P3).
#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
fn stop_body(ui: &mut Ui, stop: Stop) -> bool {
    use egui::layers::ShapeIdx;
    let Stop {
        id,
        layer,
        room,
        width,
        next,
        body,
        widgets,
        cut_line,
        rows_before,
    } = stop;
    let cut = next.view.is_some_and(Scroll::cut);
    let row_cut = next.view.filter(|_| cut).and_then(|view| {
        let rows = super::components::rows_this_pass(ui.ctx());
        whole_row_cut(rows.get(rows_before..).unwrap_or_default(), view.viewport)
    });
    let limit = row_cut.unwrap_or(next.body_room.bottom());
    let focused = ui.ctx().memory(|memory| memory.focused());
    let below: Vec<egui::WidgetRect> = ui.ctx().viewport(|viewport| {
        viewport
            .this_pass
            .widgets
            .get_layer(layer)
            .skip(widgets.0)
            .take(widgets.1.saturating_sub(widgets.0))
            .filter(|widget| widget.interact_rect.bottom() > limit && Some(widget.id) != focused)
            .copied()
            .collect()
    });
    if !below.is_empty() {
        // Its rect cut at the edge: egui keeps the last rect given for an id, and its press area
        // is that rect inside the clip (empty for a widget wholly below the edge).
        let edge = ui.new_child(
            egui::UiBuilder::new()
                .id_salt(id.with("stop_body"))
                .disabled(),
        );
        let above = Rect::everything_above(limit);
        for widget in &below {
            let _ = edge.interact(widget.rect.intersect(above), widget.id, Sense::hover());
            if widget.rect.top() >= limit {
                ui.ctx()
                    .accesskit_node_builder(widget.id, |node| node.set_hidden());
                // Its tooltip, drawn in this pass from the last pass's hover, goes too.
                ui.ctx().graphics_mut(|graphics| {
                    for count in 0..2 {
                        let tip = LayerId::new(
                            egui::Order::Tooltip,
                            egui::Tooltip::tooltip_id(widget.id, count),
                        );
                        if graphics.get(tip).is_some() {
                            let list = graphics.entry(tip);
                            for at in 0..list.next_idx().0 {
                                list.reset_shape(egui::layers::ShapeIdx(at));
                            }
                        }
                    }
                });
            }
        }
    }
    ui.ctx().graphics_mut(|graphics| {
        let list = graphics.entry(layer);
        for at in body.0..body.1 {
            list.mutate_shape(ShapeIdx(at), |shape| {
                shape.clip_rect.max.y = shape.clip_rect.max.y.min(limit);
            });
        }
        // The row cut's hairline of the body's room as this pass had it goes.
        if let Some(at) = cut_line {
            list.reset_shape(ShapeIdx(at));
        }
    });
    let hairline = |y: f32| {
        ui.painter().hline(
            room.left()..=room.left() + width,
            y,
            egui::Stroke::new(
                super::tokens::HAIRLINE_WIDTH,
                super::tokens::colors().hairline,
            ),
        );
    };
    if let Some(cut) = row_cut {
        hairline(cut - super::tokens::HAIRLINE_WIDTH / 2.0);
    } else if cut {
        hairline(next.rect.top() + super::tokens::HAIRLINE_WIDTH / 2.0);
    }
    // Keep this cut for the next pass's body room.
    ui.ctx().data_mut(|data| match row_cut {
        Some(cut) => data.insert_temp(id.with("row_cut"), (next.body_room, cut)),
        None => data.remove::<(Rect, f32)>(id.with("row_cut")),
    });
    cut
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
        self.place(
            ui,
            body,
            ContentScroll::default(),
            false,
            |ui, _| add(ui),
            |inner, _| inner,
        )
    }

    /// `pane` for a body whose acts start or end its footer's messages (a form whose fields,
    /// Save and Discard change what the footer says). `add` draws the body; then `then` gets what
    /// `add` returned and this footer, after the body and before the footer paints, and adds the
    /// messages from the state the body's acts left. So the frame of an act shows the message
    /// the act brought and never one it ended, also when egui draws that frame in one pass (no
    /// discarded pass hides it). Messages added before stay. Returns what `then` returns.
    pub fn pane_then<R, T>(
        self,
        ui: &mut Ui,
        body: Body,
        add: impl FnOnce(&mut Ui) -> R,
        then: impl FnOnce(R, &mut Self) -> T,
    ) -> T {
        self.place(
            ui,
            body,
            ContentScroll::default(),
            true,
            |ui, _| add(ui),
            then,
        )
    }

    /// `pane_then` with `Body::Content`, whose scroll column behaves as `scroll` says.
    pub fn pane_content_then<R, T>(
        self,
        ui: &mut Ui,
        scroll: ContentScroll,
        add: impl FnOnce(&mut Ui) -> R,
        then: impl FnOnce(R, &mut Self) -> T,
    ) -> T {
        self.place(ui, Body::Content, scroll, true, |ui, _| add(ui), then)
    }

    /// `pane` with `Body::Content`, whose scroll column behaves as `scroll` says.
    pub fn pane_content<R>(
        self,
        ui: &mut Ui,
        scroll: ContentScroll,
        add: impl FnOnce(&mut Ui) -> R,
    ) -> R {
        self.place(
            ui,
            Body::Content,
            scroll,
            false,
            |ui, _| add(ui),
            |inner, _| inner,
        )
    }

    /// A `Body::Content` pane whose body holds one editor that grows with its text: the body
    /// gets an [`EditorRegion`], which draws the editor once. While the content fits, the pane
    /// is exactly `Body::Content` (the footer `S` below the content). When it does not, the
    /// region gives way: its height is the room less the rest of the content (and less the
    /// footer), the editor scrolls inside it, and what comes after it (a save row) and the
    /// footer stay in view. A body that does not use the region is `Body::Content`.
    ///
    /// The contract: the region (the editor's fixed frame) is `max(floor(room − rest), min(64,
    /// room) + frame)` tall, where `frame` is the frame's margin and stroke: the text view keeps
    /// 64 px (below 64 px of room, the room). Save stays in view while the rest of the content
    /// fits the room less that; else the region keeps it and the pane scrolls as a whole, as
    /// `Body::Content` does.
    pub fn pane_editor<R>(
        self,
        ui: &mut Ui,
        add: impl for<'r> FnOnce(&mut Ui, EditorRegion<'r>) -> R,
    ) -> R {
        self.place(
            ui,
            Body::Content,
            ContentScroll::default(),
            false,
            add,
            |inner, _| inner,
        )
    }

    /// The pane. `late`: messages also come after the body, from `then` (`pane_then`).
    fn place<R, T>(
        mut self,
        ui: &mut Ui,
        body: Body,
        options: ContentScroll,
        late: bool,
        add: impl for<'r> FnOnce(&mut Ui, EditorRegion<'r>) -> R,
        then: impl FnOnce(R, &mut Self) -> T,
    ) -> T {
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
        // A typed footer whose retained plan holds knows its demand now (layout PR 2b); else the
        // last pass's demand, which this pass measures again. A footer whose messages come after
        // the body (`pane_then`) is planned after it.
        let planned = if late { None } else { self.planned(ui, width) };
        let stored = match planned {
            Some(demand) => {
                ui.ctx().data_mut(|data| data.insert_temp(key, demand));
                demand
            }
            None => ui
                .ctx()
                .data(|data| data.get_temp::<f32>(key))
                .unwrap_or(0.0),
        };
        // Whether the footer holds a message while the body draws. Messages that come after the
        // body (`pane_then`) are known only then: until then the footer keeps the room it had in
        // the last pass, as it does when its messages stay (it showed when it has a height).
        let held = !self.is_empty() || (late && stored != 0.0);
        if !held && stored != 0.0 {
            forget(ui, id);
            ui.ctx().request_discard("the message footer emptied");
        }
        // Ruling Q-c: a pinned footer takes at most a third of the pane; a footer that follows a
        // short body may take the room down to the pane's end (below).
        let most = (room.height() / 3.0).floor().max(0.0);
        // Last pass's demand, whole (what the messages want, uncapped); a change is measured
        // below, and the pass is drawn again.
        let demand = if held { stored } else { 0.0 };
        // The pinned footer's height.
        let height = demand.min(most);
        let pinned = room.bottom() - height;
        // The body has the same room whether the footer follows it or is pinned (it ends `S`
        // above the pinned footer), so it never changes its own size when the footer moves.
        let limit = if height > 0.0 {
            pinned - S
        } else {
            room.bottom()
        };
        let body_room = Rect::from_min_max(room.min, pos2(room.right(), limit.max(room.top())));

        // Last pass's whole-row cut, while the room is the same; a new cut draws the pass again.
        let cut_key = id.with("row_cut");
        let row_cut = ui
            .ctx()
            .data(|data| data.get_temp::<(Rect, f32)>(cut_key))
            .filter(|(kept, _)| *kept == body_room)
            .map(|(_, cut)| cut);
        let visible = body_clip(ui, body_room, row_cut);
        // The body, in the part's own `Ui`; one id in every state, so its scroll and focus stay.
        let mut body_ui = ui.new_child(
            egui::UiBuilder::new()
                .id_salt(id.with("body"))
                .max_rect(body_room)
                .layout(*ui.layout()),
        );
        body_ui.set_clip_rect(visible);
        let rows_before = super::components::rows_this_pass(ui.ctx()).len();
        let layer = ui.layer_id();
        let paint_at = |ui: &Ui| {
            ui.ctx()
                .graphics(|graphics| graphics.get(layer).map_or(0, |list| list.next_idx().0))
        };
        let body_from = paint_at(ui);
        let widgets_from = widget_count(ui);
        // An edge (the footer, the session banner, or the body's own bottom where the pane asks
        // for it, `Messages::rows_end_whole`) ends the body: no registered row is cut.
        let edge = held || banner_below(ui.ctx()) || self.whole_rows;
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
                if options.reveals {
                    column = column.hosts_reveals();
                }
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
                let content = |ui: &mut Ui| {
                    open(
                        ui,
                        Some(OpenPane {
                            layer: ui.layer_id(),
                            content: true,
                            usable,
                            edge,
                            pane: id,
                            measure: None,
                            extent: Some((ui.cursor().top(), room_height)),
                        }),
                    );
                    let first = widget_count(ui);
                    let inner = add(
                        ui,
                        EditorRegion {
                            pane: id,
                            max: region_max,
                            host: Host::Pane,
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
                };
                // The column is this pane's: `ScrollColumn::show` gives it the hang's room.
                let pass = ui.ctx().cumulative_pass_nr();
                ui.ctx().data_mut(|data| {
                    data.insert_temp(body_column_id(), (pass, body_ui.id()));
                });
                let output = column.show(&mut body_ui, content);
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
                        measure: None,
                        extent: None,
                    }),
                );
                let inner = add(
                    &mut body_ui,
                    EditorRegion {
                        pane: id,
                        max: 0.0,
                        host: Host::Pane,
                        _scope: std::marker::PhantomData,
                    },
                );
                open(&body_ui, None);
                (inner, None)
            }
        };
        drop(body_ui);
        let body_to = paint_at(ui);
        let widgets_to = widget_count(ui);
        // `pane_then`: the messages from the state the body's acts left, before the footer
        // paints.
        let out = then(inner, &mut self);
        let shown = !self.is_empty();
        if !shown && held && stored != 0.0 {
            forget(ui, id);
            ui.ctx().request_discard("the message footer emptied");
        }
        // Typed lines that came after the body: their retained plan, now (it holds when they are
        // the last pass's lines at this width).
        let planned = if late && shown {
            self.planned(ui, width)
        } else {
            planned
        };
        if late && let Some(demand) = planned {
            ui.ctx().data_mut(|data| data.insert_temp(key, demand));
        }
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
        let cut_line = paint_at(ui);
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

        let mut placement = match scroll {
            Some(scroll) if scroll.fits() => Placement::Follows,
            _ => Placement::Pinned,
        };
        // Content goes on below the room: the footer cuts it, and draws its hairline.
        let mut cut = shown && scroll.is_some_and(Scroll::cut);
        let mut footer = None;
        if shown {
            let third = most;
            // Where the footer starts, and the most it may take there: below a short body, `S`
            // after the content and the room down to the pane's end (Q-c); pinned, a third of
            // the pane. It is as tall as its demand up to that, and scrolls inside beyond it.
            let (top, most) = match (placement, scroll) {
                (Placement::Follows, Some(scroll)) => {
                    let top = (scroll.viewport.top() + scroll.height + S).min(pinned);
                    (top, room.bottom() - top)
                }
                _ => (pinned, most),
            };
            // A typed footer is always measured: its plan is made before it is placed (`planned`,
            // L1c). A legacy footer is measured when it was measured with these words. One that
            // was not (an act brought its messages in this pass, or it comes) below a short body
            // is laid out in its whole room there and fitted to what it shows, in this pass, so
            // its messages show whole at once, even in a frame of one pass; the pass is drawn
            // again, measured. Pinned, it keeps its measured room until that pass (the legacy
            // footer's limit: its words are closures, measured only by drawing them).
            let measured = if self.is_typed() {
                planned.is_some()
            } else {
                let words = self.words_key();
                let words_id = id.with("words");
                let measured = demand > 0.0
                    && ui.ctx().data(|data| data.get_temp::<u64>(words_id)) == Some(words);
                if !measured {
                    ui.ctx().data_mut(|data| data.insert_temp(words_id, words));
                }
                measured
            };
            let fit = placement == Placement::Follows && !measured;
            // Lines that come after the body (`pane_then`) are planned after it. When their demand
            // is not the one the body's room was made for, the footer is placed by it as the next
            // pass places it and drawn there once; the body's paint stops at its edge
            // (`stop_body`), and the pass is drawn again with the body in its new room.
            let after = planned
                .filter(|_| late)
                .filter(|now| (now - demand).abs() >= super::tokens::HAIRLINE_WIDTH)
                .map(|now| next_place(room, width, now, third, scroll));
            let (top, most) = match &after {
                Some(next) => (next.rect.top(), next.most),
                None => (top, most),
            };
            let tall = if let Some(next) = &after {
                ui.ctx()
                    .request_discard("the body's room changed with its footer");
                next.rect.height()
            } else if fit {
                ui.ctx()
                    .request_discard("the message footer measures its new messages");
                most
            } else {
                demand.min(most)
            };
            let rect = Rect::from_min_size(pos2(room.left(), top), vec2(width, tall));
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
            // Laid out in its room; when fitted, as tall as what it shows, in this same pass. It
            // draws the hairline of a line cut at its top; placed after the body was laid out for
            // another demand, `stop_body` draws the hairlines.
            let rect = self
                .draw_messages(
                    ui,
                    rect,
                    after.is_none() && cut && new_cut.is_none(),
                    most,
                    stored,
                    fit,
                )
                .0;
            if let Some(next) = after {
                placement = next.placement;
                cut = stop_body(
                    ui,
                    Stop {
                        id,
                        layer,
                        room,
                        width,
                        next,
                        body: (body_from, body_to),
                        widgets: (widgets_from, widgets_to),
                        cut_line: new_cut.map(|_| cut_line),
                        rows_before,
                    },
                );
            }
            if rect.height() > 0.0 {
                footer = Some(rect);
                #[cfg(any(test, feature = "test-support"))]
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
        // Where the footer stood, for the next pass's order of its lines: an answer to a pressed
        // line stands under it while the footer follows (`Messages::answer`).
        ui.ctx().data_mut(|data| {
            data.insert_temp(id.with("follows"), placement == Placement::Follows);
        });
        ui.advance_cursor_after_rect(room);
        #[cfg(any(test, feature = "test-support"))]
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
        #[cfg(not(any(test, feature = "test-support")))]
        let _ = (footer, placement, cut, local);
        out
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
        /// The far control's field shows its help (the same control, a changed field).
        help: bool,
        /// A field near the top, under `Next control`.
        near: bool,
        /// A text field under it, with its words.
        clip: Option<String>,
        /// A field whose act adds content (layout step 11b), with its words, its own Save
        /// under it, and its room at the list's end with the footer's growth named for it.
        grown: Option<String>,
        /// Rows between that Save and the room.
        grown_gap: usize,
        /// That room, as the last pass drew it.
        room: Option<Rect>,
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
                help: false,
                near: false,
                clip: None,
                grown: None,
                grown_gap: 0,
                room: None,
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
            if state.near {
                crate::design::small_field(ui, "Near caption", None, None, |ui| {
                    let _ = ui.button("Near control");
                });
            }
            if let Some(text) = state.clip.as_mut() {
                const CLIP: crate::design::layout::FieldName =
                    crate::field_id!("hold_test_clip_text");
                let _ = crate::design::labelled_field(
                    ui,
                    CLIP.once(crate::design::layout::Once::Example),
                    "Clip caption",
                    text,
                    None,
                    None,
                );
            }
            for row in 0..state.below {
                ui.label(format!("Row below {row}"));
            }
            // A field block: its caption, the control and (when shown) its help.
            let help = state
                .help
                .then_some("The far control's help, which comes with no change of focus.");
            crate::design::small_field(ui, "Far caption", help, None, |ui| {
                let _ = ui.button("Far control");
            });
            let grown = state.grown.as_mut().map(|text| {
                const GROWN: crate::design::layout::FieldName =
                    crate::field_id!("hold_test_grown_text");
                let field = crate::design::labelled_field(
                    ui,
                    GROWN.once(crate::design::layout::Once::Example),
                    "Grown caption",
                    text,
                    None,
                    None,
                );
                let _ = ui.button("Save grown");
                field
            });
            if grown.is_some() {
                for row in 0..state.grown_gap {
                    ui.label(format!("Row before the room {row}"));
                }
            }
            state.room = grown.map(|field| {
                let room = keep_room(ui, Room::LogEntry);
                reveal_with_focus_and_growth(ui, &field, room, Growth::ResultWithUndo);
                // The note's later sections (a reader's `From other notes`): the scroll area
                // cannot show past the content's end.
                for row in 0..6 {
                    ui.label(format!("Row after the room {row}"));
                }
                room
            });
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
        // Layout PR 10 (I9): its whole field block, caption first, not the control alone.
        let caption = h.get_by_label("Far caption").rect();
        assert!(
            caption.top() >= viewport.top() - 0.5 && caption.bottom() <= footer.top() + 0.5,
            "i9-caption: the focused field's caption {caption:?} is whole above the footer {footer:?}"
        );
        // The same control, a changed field (its help comes, as a provider change brings one):
        // the arbiter checks again, and the whole block comes above the footer.
        h.state_mut().help = true;
        h.run_steps(4);
        assert_eq!(h.ctx.memory(|memory| memory.focused()), focused);
        let help = h
            .get_by_label("The far control's help, which comes with no change of focus.")
            .rect();
        let record = pane_records(&h.ctx).into_iter().next().unwrap();
        let (viewport, footer) = (record.viewport().unwrap(), record.footer().unwrap());
        let caption = h.get_by_label("Far caption").rect();
        assert!(
            caption.top() >= viewport.top() - 0.5 && help.bottom() <= footer.top() + 0.5,
            "i9-same-id: with the same focus, the changed field from {caption:?} to {help:?} is \
             whole above the footer {footer:?}"
        );
        // The DM's own wheel is not pulled back: the field's place in the content and the room
        // did not change.
        let before = h.get_by_label("Far caption").rect().top();
        h.hover_at(viewport.center());
        h.event(egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0.0, 120.0),
            modifiers: egui::Modifiers::NONE,
        });
        h.run_steps(4);
        let after = h.get_by_label("Far caption").rect().top();
        assert!(
            after > before + 60.0,
            "i9-wheel: the wheel moved the field down from {before} to {after}, and it stays"
        );
        // Focus goes up to a field above the view: its caption comes into view with it, not
        // only the control at the view's top edge.
        h.state_mut().near = true;
        h.run_steps(2);
        h.get_by_label("Near control").focus();
        h.run_steps(4);
        let record = pane_records(&h.ctx).into_iter().next().unwrap();
        let viewport = record.viewport().unwrap();
        let (caption, control) = (
            h.get_by_label("Near caption").rect(),
            h.get_by_label("Near control").rect(),
        );
        assert!(
            caption.top() >= viewport.top() - 0.5 && control.bottom() <= viewport.bottom() + 0.5,
            "i9-caption-up: the field above the view comes whole, caption {caption:?} and control \
             {control:?}, into the view {viewport:?}"
        );
        // Layout PR 10b: a direct click on a text field whose caption the view's edge cuts. At
        // the release, its whole field comes into view, caption included.
        h.state_mut().clip = Some("Words".to_owned());
        h.run_steps(2);
        let caption = |h: &Harness<'_, State>| {
            h.get_by_role_and_label(egui::accesskit::Role::Label, "Clip caption")
                .rect()
        };
        let viewport = pane_records(&h.ctx)
            .into_iter()
            .next()
            .unwrap()
            .viewport()
            .unwrap();
        let top = caption(&h).bottom() - viewport.top() + 2.0;
        h.hover_at(viewport.center());
        h.event(egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0.0, -top),
            modifiers: egui::Modifiers::NONE,
        });
        h.run_steps(4);
        let (cut, field) = (
            caption(&h),
            h.get_by_role_and_label(egui::accesskit::Role::TextInput, "Clip caption")
                .rect(),
        );
        assert!(
            cut.top() < viewport.top() - 0.5 && field.bottom() <= viewport.bottom(),
            "i9-click-setup: the caption {cut:?} is cut at the view's top {viewport:?}, the field \
             {field:?} is in view"
        );
        h.get_by_role_and_label(egui::accesskit::Role::TextInput, "Clip caption")
            .click();
        h.run_steps(4);
        let caption = caption(&h);
        assert!(
            caption.top() >= viewport.top() - 0.5,
            "i9-click: after a click on the field, its caption {caption:?} is whole in the view \
             {viewport:?}"
        );
        // Layout PR 10c: the same field, focused all along. The DM's wheel takes its caption out
        // of view again (no reveal: the wheel is never pulled back); a second click on the field
        // brings its whole field into view, with the same focus, room and place as before.
        let focused = h.ctx.memory(|memory| memory.focused());
        let up = caption.bottom() - viewport.top() + 2.0;
        h.hover_at(viewport.center());
        h.event(egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0.0, -up),
            modifiers: egui::Modifiers::NONE,
        });
        h.run_steps(4);
        let cut = h
            .get_by_role_and_label(egui::accesskit::Role::Label, "Clip caption")
            .rect();
        assert!(
            cut.top() < viewport.top() - 0.5 && h.ctx.memory(|memory| memory.focused()) == focused,
            "i9-again-setup: the wheel cut the caption {cut:?} at the view's top {viewport:?}, \
             the field kept its focus"
        );
        h.get_by_role_and_label(egui::accesskit::Role::TextInput, "Clip caption")
            .click();
        h.run_steps(4);
        let caption = h
            .get_by_role_and_label(egui::accesskit::Role::Label, "Clip caption")
            .rect();
        assert!(
            caption.top() >= viewport.top() - 0.5,
            "i9-again: a second click on the focused field brings its caption {caption:?} into \
             the view {viewport:?}"
        );

        // Layout step 11b (Astra's review of PR 736, P1): an act's room and the footer's growth
        // belong to its one field while it has the keyboard. The growth is counted here apart
        // from `Growth`, so a growth of 0 fails too.
        let growth = S + 2.0 * crate::design::Density::Read.control();
        h.state_mut().message = false;
        h.state_mut().help = false;
        h.state_mut().near = false;
        h.state_mut().clip = None;
        h.state_mut().grown = Some("away".to_owned());
        h.state_mut().grown_gap = 60;
        h.run_steps(4);
        let view = |h: &Harness<'_, State>| {
            pane_records(&h.ctx)
                .into_iter()
                .next()
                .unwrap()
                .viewport()
                .unwrap()
        };
        let field = |h: &Harness<'_, State>| {
            h.get_by_role_and_label(egui::accesskit::Role::TextInput, "Grown caption")
                .rect()
        };
        let save = |h: &Harness<'_, State>| h.get_by_label("Save grown").rect();
        let whole = |rect: Rect, view: Rect| {
            rect.top() >= view.top() - 0.5 && rect.bottom() <= view.bottom() + 0.5
        };
        let wheel = |h: &mut Harness<'_, State>, delta: f32| {
            h.hover_at(view(h).center());
            h.event(egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                delta: egui::vec2(0.0, delta),
                modifiers: egui::Modifiers::NONE,
            });
            // The wheel's smooth scroll ends before the next act.
            h.run_steps(10);
        };
        // A long note: the field, its room and the growth are taller than the view. The DM's
        // wheel takes the field above the view, with the room below it.
        let middle = |h: &mut Harness<'_, State>| {
            let by = field(h).bottom() - view(h).top() + 40.0;
            wheel(h, -by);
            let (field, room, view) = (field(h), h.state().room.unwrap(), view(h));
            assert!(
                field.bottom() < view.top() && room.bottom() > view.bottom(),
                "i11b-setup: the field {field:?} is above the view {view:?}, its room {room:?} \
                 below it"
            );
        };
        middle(&mut h);
        h.get_by_role_and_label(egui::accesskit::Role::TextInput, "Grown caption")
            .focus();
        h.run_steps(4);
        let (shown, at) = (field(&h), view(&h));
        assert!(
            whole(shown, at),
            "i11b-field-alone: with its room too far below, the focused field {shown:?} comes \
             whole into the view {at:?} alone"
        );
        middle(&mut h);
        h.get_by_label("Save grown").focus();
        h.run_steps(4);
        let (shown, at) = (save(&h), view(&h));
        assert!(
            whole(shown, at),
            "i11b-save-alone: Tab to the field's Save {shown:?} brings it whole into the view \
             {at:?}"
        );
        // A short note: the field, its room and the growth fit the view. Its Save, whole at the
        // view's end with the room below the view, takes the keyboard and nothing scrolls; the
        // field takes it and comes into view with its room and the growth.
        h.state_mut().grown_gap = 0;
        h.run_steps(4);
        h.get_by_label("Next control").focus();
        h.run_steps(4);
        let by = save(&h).bottom() - (view(&h).bottom() - 10.0);
        wheel(&mut h, -by);
        let (before, room, at) = (save(&h), h.state().room.unwrap(), view(&h));
        assert!(
            whole(before, at)
                && room.bottom() + growth > at.bottom()
                && room.bottom() + growth - field(&h).top() <= at.height(),
            "i11b-setup-short: Save {before:?} whole in the view {at:?}, the room {room:?} and \
             the growth {growth} past its end, all of it no taller than the view"
        );
        h.get_by_label("Save grown").focus();
        h.run_steps(4);
        let after = save(&h);
        assert!(
            (after.top() - before.top()).abs() <= 0.5,
            "i11b-save-no-growth: the room and the growth are the field's, not Save's: Save \
             stays at {before:?}, not {after:?}"
        );
        h.get_by_role_and_label(egui::accesskit::Role::TextInput, "Grown caption")
            .focus();
        h.run_steps(4);
        let (shown, room, at) = (field(&h), h.state().room.unwrap(), view(&h));
        assert!(
            whole(shown, at) && room.bottom() + growth <= at.bottom() + 0.5,
            "i11b-field-with-room: the focused field {shown:?}, its room {room:?} and the growth \
             {growth} come whole into the view {at:?}"
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
    /// rows of different heights: each column cuts at its own last whole row. Layout step 11a
    /// (SM5, AM16): with no footer line, a pane whose messages ask for it (`rows_end_whole`) cuts
    /// at its own bottom as at a footer.
    #[test]
    fn a_footer_never_cuts_through_a_registered_row() {
        use crate::design::layout::ScrollColumn;
        let (mut hidden_by_the_cut, mut hidden_when_asked) = (0, 0);
        let (mut columns_apart, mut plain_fills_cut) = (0, 0);
        for fills in [false, true] {
            for (message, whole) in [(true, false), (false, false), (false, true)] {
                // A whole-row cut: at the footer, or at the body's own bottom when asked.
                let cuts = message || whole;
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
                            if whole {
                                messages.rows_end_whole();
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
                                    // Two columns side by side, each half of the body.
                                    let all = ui.available_rect_before_wrap();
                                    let (left, right) = all.split_left_right_at_fraction(0.5);
                                    let column =
                                        |ui: &mut Ui, at: Rect, id: &str, name: &str, tall: f32| {
                                            ui.scope_builder(
                                                egui::UiBuilder::new().max_rect(at),
                                                |ui| {
                                                    ScrollColumn::new(id)
                                                        .show(ui, |ui| rows(ui, name, tall));
                                                },
                                            );
                                        };
                                    column(ui, left, "rows_left", "Row", 44.0);
                                    column(ui, right, "rows_right", "Tall", 60.0);
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
                        for row in side.iter().filter(|_| cuts) {
                            assert!(
                                row.bottom() <= clip + 0.5 || row.top() >= clip - 0.5,
                                "n12-cut: fills {fills}, message {message}, whole {whole}, height {height}: row {row:?} cut at {clip}"
                            );
                        }
                        if cuts {
                            let hidden = side
                                .iter()
                                .filter(|row| {
                                    row.top() >= clip - 0.5 && row.top() < viewport.bottom()
                                })
                                .count();
                            hidden_by_the_cut += hidden;
                            if whole {
                                hidden_when_asked += hidden;
                            }
                            let hairline = output.shapes.iter().any(|clipped| {
                                matches!(&clipped.shape, egui::Shape::LineSegment { points, .. }
                                    if (points[0].y - clip).abs() <= 1.0 && points[0].y == points[1].y
                                        && points[0].x <= view.center().x && points[1].x >= view.center().x)
                            });
                            assert!(
                                hairline || clip >= viewport.bottom() - 0.5,
                                "n12-hairline: fills {fills}, whole {whole}, height {height}, cut {clip}"
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
        // Astra's P1 on PR 700, at 1920x1080: a wrapped Armor fact (1032..1068) in one column and
        // Darkvision (1056..1074) in the other, the edge at 1072. Darkvision moves the cut to 1056,
        // which crosses Armor: the cut moves again, to Armor's top. Both rows stay whole.
        let view = Rect::from_min_max(pos2(0.0, 40.0), pos2(1200.0, 1072.0));
        let (armor, darkvision) = (
            Rect::from_min_max(pos2(0.0, 1032.0), pos2(600.0, 1068.0)),
            Rect::from_min_max(pos2(600.0, 1056.0), pos2(1200.0, 1074.0)),
        );
        let cut = whole_row_cut(&[armor, darkvision], view);
        assert_eq!(
            cut,
            Some(1032.0),
            "n12-columns-fixed-point: a cut that moves for one column leaves the other whole"
        );
        assert!(
            hidden_when_asked > 0,
            "n12-rows-end-whole: with no footer line, `rows_end_whole` cut a row at the body's bottom"
        );
        assert!(
            columns_apart > 0,
            "n12-columns: each column cuts at its own last whole row"
        );
        assert!(
            plain_fills_cut > 0,
            "n12-plain-fills: with no edge a `Fills` column keeps its plain cut"
        );

        // Migration-32 layout step 3a: the same rule from the whole-block plan, before anything
        // paints (`plan`). Checked against the blocks' own spans, never against the mask itself.
        use super::plan::{Block, BlockPlan, MeasureEnv, Part, PlanState, Route, Row, index_reads};
        // On the plan's physical pixel grid (design §3: at most one physical pixel after rounding).
        let grid = |plan: &BlockPlan, y: f32| (y * plan.scale()).round();
        // Whether block `at` lies whole in the view.
        let inside = |plan: &BlockPlan, at: usize, offset: f32, view: f32| {
            let (top, end) = plan.span(at);
            grid(plan, top) >= grid(plan, offset) && grid(plan, end) <= grid(plan, offset + view)
        };
        // I1 at one offset and view: a block that fits is eligible exactly when it is whole in
        // the view; nothing eligible lies outside the view; a view that holds a whole block that
        // fits has eligible blocks. I12: the mask counts every index entry it reads.
        let i1 = |plan: &BlockPlan, offset: f32, view: f32, case: &str| {
            let before = index_reads();
            let mask = plan.mask("plan_test", offset, view);
            assert_eq!(
                index_reads() - before,
                mask.read,
                "plan-i12 {case}: the mask counts each index entry it reads (offset {offset}, view {view})"
            );
            let mut any_whole = false;
            for at in 0..plan.len() {
                let (top, end) = plan.span(at);
                let fits = grid(plan, end - top) <= grid(plan, view);
                let whole = inside(plan, at, offset, view);
                any_whole |= whole && fits;
                let eligible = mask.eligible.is_some_and(|(from, to)| {
                    grid(plan, top) >= grid(plan, from) && grid(plan, end) <= grid(plan, to)
                });
                if fits {
                    assert_eq!(
                        (eligible, mask.whole.contains(&at)),
                        (whole, whole),
                        "plan-i1 {case}: block {at} {top}..{end} at offset {offset} in a view of {view} at scale {}: {mask:?}",
                        plan.scale()
                    );
                }
            }
            if let Some((from, to)) = mask.eligible {
                assert!(
                    grid(plan, from) >= grid(plan, offset)
                        && grid(plan, to) <= grid(plan, offset + view),
                    "plan-i1 {case}: eligible {from}..{to} lies in the view {offset}+{view} at scale {}",
                    plan.scale()
                );
            }
            assert!(
                !any_whole || !mask.whole.is_empty(),
                "plan-i1 {case}: a whole block at offset {offset} in {view} is eligible: {mask:?}"
            );
            // Review of PR 461 (re-check, 1): each route the mask returns shows its unit whole
            // there, by the mask's own rule; so does `offset_showing` for each block in the view
            // that is not whole.
            let routes = mask
                .no_whole
                .iter()
                .flat_map(|gap| [Some(gap.above), gap.below])
                .flatten();
            for route in routes {
                if let Some(at) = route.offset {
                    let shown = plan.mask("plan_test", at, view);
                    assert!(
                        shown.eligible.is_some_and(|(from, to)| {
                            grid(plan, route.top) >= grid(plan, from)
                                && grid(plan, route.top + route.height) <= grid(plan, to)
                        }),
                        "plan-l7 {case}: at offset {offset} in {view} the route {at} shows {route:?} whole: {shown:?}"
                    );
                }
            }
            for at in 0..plan.len() {
                let (top, end) = plan.span(at);
                let meets = grid(plan, end) > grid(plan, offset)
                    && grid(plan, top) < grid(plan, offset + view);
                if meets
                    && !inside(plan, at, offset, view)
                    && let Some(route) = plan.offset_showing(at, view, offset)
                {
                    let shown = plan.mask("plan_test", route, view);
                    assert!(
                        shown.whole.contains(&at),
                        "plan-route {case}: offset_showing({at}) from {offset} in {view} is {route}, where block {at} is whole: {shown:?}"
                    );
                }
            }
            mask
        };
        let blocks = || {
            std::iter::once(Block::Section(Row::TwoLine))
                .chain(std::iter::repeat_n(Block::Row(Row::TwoLine), 29))
                .chain([Block::Section(Row::Empty)])
                .chain(std::iter::repeat_n(Block::Row(Row::OneLine), 10))
                .collect::<Vec<_>>()
        };
        let mut content = crate::design::memo::Revised::new(blocks());
        let env = MeasureEnv {
            width: 600.0,
            fonts: 1,
            scale: 1.0,
        };
        // Top and bottom edges at every offset, a quarter point apart in turn (fractional
        // offsets), at scales 1 to 4, with a footer that comes, grows and goes (the view shrinks
        // and comes back; the plan does not change).
        for scale in [1.0, 1.5, 2.0, 4.0] {
            let mut scaled = PlanState::new("plan_test");
            let plan = scaled.plan(&content, MeasureEnv { scale, ..env });
            let n = plan.len();
            let extent = plan.extent();
            assert_eq!((n, plan.scale()), (41, scale), "plan-setup: 41 blocks");
            for view in [100.0, 300.0, 333.0] {
                let mut step = 0;
                loop {
                    let offset = 7.0 * step as f32 + [0.0, 0.25, 0.5, 0.75][step % 4];
                    if offset > plan.max_offset(view) {
                        break;
                    }
                    i1(plan, offset, view, "sweep");
                    step += 1;
                }
                // I2: the native range is the whole extent, whatever the mask; at the largest
                // offset the last block is whole and eligible.
                let last = plan.max_offset(view);
                let mask = i1(plan, last, view, "max offset");
                assert!(
                    (last - (extent - view)).abs() <= 0.5
                        && mask.whole.end == n
                        && plan.extent() == extent,
                    "plan-i2: at the largest offset {last} of {extent} in {view} the last block is whole: {mask:?}"
                );
            }
        }
        // Design §3 (review of PR 461, 3): at scale 4, a 40 pt row with half a point of it above
        // the view (two physical pixels) is not whole; with no unit whole, the offset stays and
        // the routes show each row whole.
        let rows = BlockPlan::of_heights(&[(40.0, None), (40.0, None)], 4.0);
        let pixels = i1(&rows, 0.5, 40.0, "scale 4");
        assert_eq!(
            (
                pixels.whole.is_empty(),
                pixels.eligible,
                pixels
                    .no_whole
                    .map(|gap| (gap.above.offset, gap.below.and_then(|below| below.offset)))
            ),
            (true, None, Some((Some(0.0), Some(40.0)))),
            "plan-grid: two pixels above the view at scale 4 leave the first row not whole: {pixels:?}"
        );

        let mut state = PlanState::new("plan_test");
        let plan = state.plan(&content, env);
        let n = plan.len();
        // A section taller than the view splits at its heading (ruling L6); a row taller than
        // the view is a counted `TooTall`, never eligible.
        let section = plan.span(0).1;
        let split = plan.mask("plan_test", 0.0, section - 10.0);
        let row_end = plan.span(1).1;
        assert!(
            split.whole.is_empty()
                && split
                    .eligible
                    .is_some_and(|(from, to)| from == 0.0 && to < section)
                && split.too_tall.is_empty(),
            "plan-l6: a section of {section} in a view of {} shows its heading whole: {split:?}",
            section - 10.0
        );
        let tall = plan.mask("plan_test", section, 30.0);
        assert!(
            tall.eligible.is_none()
                && tall
                    .too_tall
                    .iter()
                    .map(|tall| (tall.block, tall.part))
                    .eq([(1, Part::Block)]),
            "plan-l6: a row of {} in a view of 30 is too tall: {tall:?} (ends {row_end})",
            row_end - section
        );
        // Review of PR 461, 2: each part of a split section is checked alone. A section of a
        // 34 pt heading over a 40 pt row in a 35 pt view: the heading is whole, and the row,
        // taller than the view, is the section's `TooTall`.
        let two_line = BlockPlan::of_heights(&[(74.0, Some(34.0))], 1.0);
        let heading_whole = i1(&two_line, 0.0, 35.0, "heading over a tall row");
        assert!(
            heading_whole.eligible == Some((0.0, 34.0))
                && heading_whole
                    .too_tall
                    .iter()
                    .map(|tall| (tall.block, tall.part, tall.height))
                    .eq([(0, Part::Row, 40.0)]),
            "plan-l6: the heading is whole and the row is too tall: {heading_whole:?}"
        );

        // Ruling L7: two 90 px blocks in a 100 px view at offset 40 leave neither whole. The
        // offset stays, the diagnostic is there, and its routes show each block whole.
        let pair = BlockPlan::of_heights(&[(90.0, None), (90.0, None)], 1.0);
        let mask = i1(&pair, 40.0, 100.0, "90/100/40");
        let gap = mask
            .no_whole
            .expect("plan-l7: the 90/100/40 case reports no whole block");
        let route = |block, part, top, height, offset| Route {
            block,
            part,
            top,
            height,
            offset: Some(offset),
        };
        assert_eq!(
            (gap.offset, gap.view, gap.above, gap.below),
            (
                40.0,
                100.0,
                route(0, Part::Block, 0.0, 90.0, 0.0),
                Some(route(1, Part::Block, 90.0, 90.0, 80.0))
            ),
            "plan-l7: the diagnostic names the offset, the view and both blocks with their routes"
        );
        for (at, route) in [(0, 0.0), (1, 80.0)] {
            assert_eq!(
                (
                    pair.offset_showing(at, 100.0, 40.0),
                    i1(&pair, route, 100.0, "route").whole
                ),
                (Some(route), at..at + 1),
                "plan-l7: the route {route} shows block {at} whole"
            );
        }
        // Review of PR 461, 1: a split section's routes come from its parts. A 54 pt section
        // (heading 34, empty row 20) in a 35 pt view at offset 10: neither part is whole, no part
        // is too tall, and the routes show the heading (offset 0) and the row (offset 19).
        let empty = BlockPlan::of_heights(&[(54.0, Some(34.0))], 1.0);
        let parts = i1(&empty, 10.0, 35.0, "split section");
        assert_eq!(
            (
                parts.eligible,
                parts.too_tall.clone(),
                parts.no_whole.map(|gap| (gap.above, gap.below))
            ),
            (
                None,
                Vec::new(),
                Some((
                    route(0, Part::Heading, 0.0, 34.0, 0.0),
                    Some(route(0, Part::Row, 34.0, 20.0, 19.0))
                ))
            ),
            "plan-l7: a split section's routes are its parts: {parts:?}"
        );
        for (route, part) in [(0.0, (0.0, 34.0)), (19.0, (34.0, 54.0))] {
            assert_eq!(
                i1(&empty, route, 35.0, "part route").eligible,
                Some(part),
                "plan-l7: the route {route} shows the part {part:?} whole"
            );
        }

        // Review of PR 461 (re-check, 1): at scale 1.25 a 32 pt row after a 74 pt section in a
        // 31.9 pt view: at offset 74 the row is not whole (its end's pixel is below the view's),
        // and the route that `offset_showing` gives shows it whole, by the mask's rule.
        let after_section = BlockPlan::of_heights(&[(74.0, Some(34.0)), (32.0, None)], 1.25);
        let hidden = i1(&after_section, 74.0, 31.9, "scale 1.25");
        let shown = after_section
            .offset_showing(1, 31.9, 74.0)
            .map(|at| i1(&after_section, at, 31.9, "scale 1.25 route").whole);
        assert!(
            !hidden.whole.contains(&1) && shown == Some(1..2),
            "plan-route: the row's route at scale 1.25 shows it whole: {hidden:?}, {shown:?}"
        );
        // Review of PR 461 (re-check, 2): every unit too tall for the view that meets it is
        // reported and counted. Rows of 40, 20 and 40 pt in a 35 pt view at offset 30: the middle
        // row is whole, and both outer rows are too tall.
        let three = crate::design::memo::Revised::new(vec![
            Block::Row(Row::TwoLine),
            Block::Row(Row::Empty),
            Block::Row(Row::TwoLine),
        ]);
        let mut counted = PlanState::new("plan_test");
        let both = counted.mask(&three, env, 30.0, 35.0);
        assert_eq!(
            (
                both.eligible,
                both.too_tall
                    .iter()
                    .map(|tall| (tall.block, tall.part, tall.height))
                    .collect::<Vec<_>>(),
                counted.work().too_tall
            ),
            (
                Some((40.0, 60.0)),
                vec![(0, Part::Block, 40.0), (2, Part::Block, 40.0)],
                2
            ),
            "plan-l6: both outer rows are too tall, each counted: {both:?}"
        );

        // I12 and M6: a quiet frame (same content, same environment) measures nothing, and a
        // mask reads `O(log n)` index entries, every one counted: three searches of
        // ⌈log2(n + 1)⌉ reads at most, and 19 more (the eligible span, the two cut blocks' spans
        // and boundaries, and the two routes), here and in 100,000 blocks.
        let quiet = state.work();
        assert_eq!(
            (quiet.builds, quiet.blocks_measured),
            (1, n),
            "plan-i12: one plan"
        );
        let bound = |n: usize| 3 * (usize::BITS - n.leading_zeros()) as usize + 19;
        for frame in 0..10 {
            let mask = state.mask(&content, env, 7.0 * frame as f32, 300.0);
            assert!(
                mask.read <= bound(n),
                "plan-i12: a mask reads at most {} entries in {n} blocks: {}",
                bound(n),
                mask.read
            );
        }
        let many: Vec<(f32, Option<f32>)> = (0..100_000)
            .map(|at| {
                if at % 50 == 0 {
                    (74.0, Some(34.0))
                } else {
                    (40.0, None)
                }
            })
            .collect();
        let large = BlockPlan::of_heights(&many, 1.0);
        let most = (0..200)
            .flat_map(|at| [(at as f32 * 20_011.5, 300.0), (at as f32 * 20_011.5, 35.0)])
            .map(|(offset, view)| {
                i1(&large, offset.min(large.max_offset(view)), view, "large").read
            })
            .max()
            .unwrap_or(0);
        assert!(
            most <= bound(large.len()),
            "plan-i12: a mask reads at most {} entries in {} blocks: {most}",
            bound(large.len()),
            large.len()
        );
        let after = state.work();
        assert_eq!(
            (after.builds, after.blocks_measured, after.bytes_indexed),
            (quiet.builds, quiet.blocks_measured, quiet.bytes_indexed),
            "plan-i12: ten quiet frames measure and index nothing"
        );
        let wider = MeasureEnv {
            width: 640.0,
            ..env
        };
        let _ = state.plan(&content, wider);
        assert_eq!(
            state.work().builds,
            2,
            "plan-m6: a new width measures the plan again"
        );
        // Content that shrinks while scrolled: the new revision measures again, the offset is
        // kept in the new range (never snapped), and there the last block is whole.
        let far = state.plan(&content, wider).max_offset(300.0);
        content.revise(|blocks| {
            blocks.truncate(12);
            crate::design::memo::Changed::Yes
        });
        let shrunk = state.plan(&content, wider);
        let kept = shrunk.clamp(far, 300.0);
        assert!(
            shrunk.len() == 12
                && kept == shrunk.max_offset(300.0)
                && i1(shrunk, kept, 300.0, "shrunk").whole.end == 12,
            "plan-shrink: offset {far} kept at {kept} in the shrunk content, its last block whole"
        );
        assert_eq!(
            state.work().builds,
            3,
            "plan-m6: a new content revision measures again"
        );

        // Layout step 3b: the region of an owned list draws only whole blocks, keeps the plan's
        // whole extent as its native range, is its offset's one writer (its requests in their
        // order) and owns Tab at its edges. Every frame is drawn in one pass (reruns denied).
        use super::region::{BlockRegion, Navigation, RegionState};
        struct List {
            state: RegionState,
            rows: crate::design::memo::Revised<Vec<Block>>,
            room: f32,
            selected: Option<usize>,
        }
        let mut list = egui_kittest::Harness::builder()
            .with_size(egui::vec2(400.0, 420.0))
            .build_ui_state(
                |ui, list: &mut List| {
                    let (room, selected) = (list.room, list.selected);
                    let List { state, rows, .. } = list;
                    let _ = ui.button("Before");
                    ui.allocate_ui(egui::vec2(ui.available_width(), room), |ui| {
                        ui.set_max_height(room);
                        BlockRegion::new("region_test", state, rows)
                            .name("Rows")
                            .selected(selected)
                            .show(ui, |ui, at| {
                                let _ = ui.add_sized(
                                    ui.available_size(),
                                    egui::Button::new(format!("Row {at}")),
                                );
                            });
                    });
                    let _ = ui.button("After");
                },
                List {
                    state: RegionState::new("region_test"),
                    rows: crate::design::memo::Revised::new(vec![Block::Row(Row::Entry); 60]),
                    room: 300.0,
                    selected: None,
                },
            );
        crate::design::theme::install(&list.ctx);
        list.ctx
            .options_mut(|options| options.max_passes = std::num::NonZeroUsize::MIN);
        list.run_steps(3);
        // The rows with an accessible node, and each one's rect; and the rows painted.
        let shown = |h: &egui_kittest::Harness<'_, List>| -> Vec<(usize, Rect)> {
            use egui_kittest::kittest::Queryable;
            (0..60)
                .filter_map(|at| {
                    h.query_by_label(&format!("Row {at}"))
                        .map(|node| (at, node.rect()))
                })
                .collect()
        };
        let painted = |h: &egui_kittest::Harness<'_, List>| -> Vec<String> {
            fn walk(shape: &egui::Shape, out: &mut Vec<String>) {
                match shape {
                    egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, out)),
                    egui::Shape::Text(text) => out.push(text.galley.text().to_owned()),
                    _ => {}
                }
            }
            let mut out = Vec::new();
            for shape in &h.output().shapes {
                walk(&shape.shape, &mut out);
            }
            out
        };
        // I1: each row drawn lies whole in the room; a row the room's edge cuts has no node and no
        // paint.
        let whole = |h: &egui_kittest::Harness<'_, List>, case: &str| -> Vec<usize> {
            let rows = shown(h);
            let top = rows
                .iter()
                .map(|(_, rect)| rect.top())
                .fold(f32::MAX, f32::min);
            let room = h.state().room.max(ENTRY_ROW_HEIGHT);
            assert!(
                !rows.is_empty()
                    && rows
                        .iter()
                        .all(|(_, rect)| rect.bottom() <= top + room + 0.5)
                    && rows.windows(2).all(|pair| pair[1].0 == pair[0].0 + 1),
                "region {case}: the rows drawn are whole and in order: {rows:?}"
            );
            let first = rows[0].0;
            let last = rows[rows.len() - 1].0;
            let texts = painted(h);
            for cut in [first.wrapping_sub(1), last + 1] {
                assert!(
                    !texts.contains(&format!("Row {cut}")),
                    "region {case}: the cut row {cut} is not painted"
                );
            }
            rows.iter().map(|(at, _)| *at).collect()
        };
        assert_eq!(whole(&list, "first frame"), (0..6).collect::<Vec<_>>());
        // A jump shows its row whole, in the frame that asks (M18).
        list.state_mut()
            .state
            .navigate(Navigation::Jump { block: 40 });
        list.step();
        assert!(
            whole(&list, "jump").contains(&40),
            "region: the jump shows row 40"
        );
        // I2: the native range is the whole extent: at the largest offset the last row is whole.
        list.state_mut()
            .state
            .navigate(Navigation::Restore(f32::MAX));
        list.step();
        assert!(
            whole(&list, "restore").contains(&59),
            "region: the last row is reached"
        );
        // A new revision of the rows plans again.
        let builds = list.state().state.work().builds;
        list.state_mut().rows.revise(|rows| {
            rows.truncate(10);
            crate::design::memo::Changed::Yes
        });
        list.step();
        list.step();
        assert!(
            list.state().state.work().builds > builds && whole(&list, "shrunk").ends_with(&[9]),
            "region: ten rows, planned again, the last one whole"
        );
        list.state_mut().rows.revise(|rows| {
            *rows = vec![Block::Row(Row::Entry); 60];
            crate::design::memo::Changed::Yes
        });
        list.state_mut().state.navigate(Navigation::Reset);
        list.run_steps(2);
        assert_eq!(whole(&list, "reset").first(), Some(&0));
        // L10: a user scroll over the region cancels a jump asked in the same frame.
        let centre = shown(&list)[2].1.center();
        list.state_mut()
            .state
            .navigate(Navigation::Jump { block: 50 });
        list.input_mut()
            .events
            .push(egui::Event::PointerMoved(centre));
        list.input_mut().events.push(egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0.0, -10.0),
            modifiers: egui::Modifiers::NONE,
        });
        list.step();
        list.step();
        assert!(
            !whole(&list, "wheel").contains(&50),
            "region: the wheel cancels the jump"
        );
        list.state_mut().state.navigate(Navigation::Reset);
        list.run_steps(2);
        // Issue #539: the wheel cancels a key's jump and its focus. A press held in the region
        // makes End's jump wait; the wheel cancels it; a jump made later to that row by other
        // means draws it, and the row takes no focus the wheel cancelled.
        {
            use egui_kittest::kittest::{NodeT, Queryable};
            let first = shown(&list)[0].0;
            list.get_by_label(&format!("Row {first}")).focus();
            list.step();
            let (held, away) = (shown(&list)[1].1.center(), egui::pos2(395.0, 5.0));
            list.input_mut()
                .events
                .push(egui::Event::PointerMoved(held));
            list.input_mut().events.push(egui::Event::PointerButton {
                pos: held,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            });
            list.step();
            list.key_press(egui::Key::End);
            list.step();
            list.input_mut().events.push(egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                delta: egui::vec2(0.0, -10.0),
                modifiers: egui::Modifiers::NONE,
            });
            list.step();
            list.input_mut()
                .events
                .push(egui::Event::PointerMoved(away));
            list.input_mut().events.push(egui::Event::PointerButton {
                pos: away,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            });
            list.step();
            assert!(
                !whole(&list, "key-wheel").contains(&59),
                "region: the wheel cancels End's jump"
            );
            list.state_mut()
                .state
                .navigate(Navigation::Jump { block: 59 });
            list.run_steps(3);
            assert!(
                whole(&list, "key-wheel-later").contains(&59)
                    && !list.get_by_label("Row 59").accesskit_node().is_focused(),
                "#539: the row End asked for takes no focus after the wheel cancelled it"
            );
        }
        list.state_mut().state.navigate(Navigation::Reset);
        list.run_steps(2);
        // A press held in the region makes a jump wait; it lands after the release.
        let centre = shown(&list)[1].1.center();
        list.input_mut()
            .events
            .push(egui::Event::PointerMoved(centre));
        list.input_mut().events.push(egui::Event::PointerButton {
            pos: centre,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        });
        list.step();
        list.state_mut()
            .state
            .navigate(Navigation::Jump { block: 30 });
        list.step();
        assert!(
            !whole(&list, "held").contains(&30),
            "region: the jump waits while the press is held"
        );
        list.input_mut().events.push(egui::Event::PointerButton {
            pos: centre,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        });
        list.step();
        list.step();
        assert!(
            whole(&list, "released").contains(&30),
            "region: the jump lands after the release"
        );
        // L10, a list box (the lead's ruling B): Down from the last whole row shows the next row
        // whole and focuses it; End and Home reach the last and the first row; Tab enters at the
        // selected row and the next Tab leaves the list.
        let focus_on = |h: &egui_kittest::Harness<'_, List>, label: &str| {
            use egui_kittest::kittest::{NodeT, Queryable};
            let node = h
                .query_by_label(label)
                .map(|node| node.accesskit_node().id());
            node.is_some()
                && h.ctx
                    .memory(|memory| memory.focused())
                    .map(|id| egui::accesskit::NodeId(id.value()))
                    == node
        };
        let key = |h: &mut egui_kittest::Harness<'_, List>, key: egui::Key, shift: bool| {
            h.input_mut().events.push(egui::Event::Key {
                key,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: if shift {
                    egui::Modifiers::SHIFT
                } else {
                    egui::Modifiers::NONE
                },
            });
            h.run_steps(3);
        };
        list.state_mut().state.navigate(Navigation::Reset);
        list.run_steps(2);
        let last = *whole(&list, "arrows").last().expect("rows");
        {
            use egui_kittest::kittest::Queryable;
            list.get_by_label(&format!("Row {last}")).focus();
        }
        list.run_steps(2);
        key(&mut list, egui::Key::ArrowDown, false);
        assert!(
            whole(&list, "down").contains(&(last + 1))
                && focus_on(&list, &format!("Row {}", last + 1)),
            "region: Down from row {last} shows row {} whole and focuses it",
            last + 1
        );
        key(&mut list, egui::Key::End, false);
        assert!(
            whole(&list, "end").contains(&59) && focus_on(&list, "Row 59"),
            "region: End reaches the last row"
        );
        key(&mut list, egui::Key::Home, false);
        assert!(
            whole(&list, "home").contains(&0) && focus_on(&list, "Row 0"),
            "region: Home reaches the first row"
        );
        list.state_mut().selected = Some(3);
        {
            use egui_kittest::kittest::Queryable;
            list.get_by_label("Before").focus();
        }
        list.run_steps(2);
        key(&mut list, egui::Key::Tab, false);
        assert!(
            focus_on(&list, "Row 3"),
            "region: Tab enters the list at the selected row"
        );
        key(&mut list, egui::Key::Tab, false);
        assert!(
            focus_on(&list, "After"),
            "region: the next Tab leaves the list"
        );
        key(&mut list, egui::Key::Tab, true);
        assert!(
            focus_on(&list, "Row 3"),
            "region: Shift+Tab enters at the selected row"
        );
        list.state_mut().selected = None;
        // L3: below the tallest row, the list is laid out at that height in an outer scroll,
        // counted.
        list.state_mut().room = 30.0;
        list.run_steps(2);
        assert!(
            list.state().state.small_rooms() == 1 && !shown(&list).is_empty(),
            "region: the small room is counted and the rows still draw"
        );
    }

    /// Review of PR 306: with keyboard focus on a registered row near the bottom, a footer (or
    /// the session banner) that comes without a change of focus brings that row whole into view
    /// before a key reaches it, and Enter fires exactly its one action. Both edges. Review of PR
    /// 337 (D): the same in a scroll column of a `Body::Fills` pane, beside a second column with
    /// taller rows.
    #[test]
    fn a_new_edge_brings_the_focused_row_whole_into_view() {
        use crate::design::layout::ScrollColumn;
        #[derive(Default)]
        struct State {
            message: bool,
            banner: bool,
            focus: Option<usize>,
            pressed: Vec<usize>,
            rows: Vec<Rect>,
            clip: f32,
        }
        for (fills, banner) in [(false, false), (false, true), (true, false), (true, true)] {
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
                            let draw = move |ui: &mut Ui, name: &str, tall: f32, focus: Option<usize>| {
                                let mut rows = Vec::new();
                                let mut presses = Vec::new();
                                for at in 0..30 {
                                    let row = two_line_row(
                                        ui,
                                        tall,
                                        false,
                                        &format!("{name} {at}"),
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
                            };
                            let (rows, presses, clip) = if fills {
                                let body = Body::Fills(Fills::for_test(FillsReason::Columns));
                                messages
                                    .pane(ui, body, |ui| {
                                        let all = ui.available_rect_before_wrap();
                                        let (left, right) = all.split_left_right_at_fraction(0.5);
                                        let shown = ui
                                            .scope_builder(egui::UiBuilder::new().max_rect(left), |ui| {
                                                ScrollColumn::new("focus_left")
                                                    .show(ui, |ui| draw(ui, "Row", 44.0, focus))
                                                    .inner
                                            })
                                            .inner;
                                        ui.scope_builder(egui::UiBuilder::new().max_rect(right), |ui| {
                                            ScrollColumn::new("focus_right")
                                                .show(ui, |ui| draw(ui, "Tall", 60.0, None));
                                        });
                                        shown
                                    })
                                    .body
                            } else {
                                messages
                                    .pane(ui, Body::Content, |ui| draw(ui, "Row", 44.0, focus))
                                    .body
                            };
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
                "n12-focus fills {fills} banner {banner}: row {target} {row:?} not whole above {clip}"
            );
            harness.key_press(egui::Key::Enter);
            harness.run_steps(2);
            assert_eq!(
                harness.state().pressed,
                vec![target],
                "n12-enter fills {fills} banner {banner}: exactly the focused row's action"
            );
        }
    }
    /// Lead note for P2 (package R's counterexample, the stat-block pane's `Stop`), and the
    /// review of PR 337 (E). The hang of a quiet `line_start` action paints into the margin left
    /// of the pane, inside the parent's clip; no content moves.
    /// 1. A pane with a 16 px page margin at its left: `Stop` is whole inside the clip; the clip
    ///    reaches the icon action's 28 px hang as far as the parent allows; both actions' words
    ///    start at a plain label's edge.
    /// 2. A pane flush against a `Column`'s splitter: the parent's clip wins; neither action's
    ///    paint nor its hit area reaches into the splitter's grab band (they may be cut).
    /// 3. The text edge of a `Content` pane is the same before and after a `line_start` action
    ///    first draws in it.
    ///
    /// P3a (package R's counterexample: `run-rolls` 02 at 960, `Stop` 45 px wide, 37 shown):
    /// in 1, the pane's own scroll column (its surface, which egui and the gate read as the
    /// column's room) starts at the clip, so it covers the hang, and an icon action whose hang
    /// the room leaves short is whole from the clip. 4. Beside a splitter, in the first pass,
    /// both neighbour orders: everything starts at the reserved band's right edge. 5. No stale
    /// room the pass after a column goes. 6. Two columns in a `Fills` body: the second starts
    /// exactly at the first's reserved lane. 7. A pane its parent cuts never widens the
    /// parent's clip. 8. A real press 2 px outside the clip does not reach `Stop`.
    #[test]
    fn a_line_start_action_keeps_its_gutter_in_a_pane() {
        use crate::design::components::{Action, LINE_START_HANG};
        use egui_kittest::kittest::{NodeT, Queryable};
        struct State {
            action: bool,
            stop: Option<(Rect, Rect)>,
            icon: Option<(Rect, Rect)>,
            label: Rect,
            clip: Rect,
            parent: Rect,
            /// Clicks of `Stop` and the icon action, counted in the frame egui reports them.
            presses: usize,
        }
        impl Default for State {
            fn default() -> Self {
                Self {
                    action: false,
                    stop: None,
                    icon: None,
                    label: Rect::NOTHING,
                    clip: Rect::NOTHING,
                    parent: Rect::NOTHING,
                    presses: 0,
                }
            }
        }
        // The pane: an optional `Stop` and icon action (their rect and hit rect), then a label.
        let pane = |ui: &mut Ui, state: &mut State| {
            let frame = PaneFrame::new(ui);
            let messages = Messages::<NoPress>::new(&frame, "gutter_test", 400.0);
            let action = state.action;
            let drawn = messages
                .pane(ui, Body::Content, |ui| {
                    let mut stop = None;
                    let mut icon = None;
                    let mut presses = 0;
                    if action {
                        let shown = Action::quiet("Stop").line_start().show(ui);
                        presses += usize::from(shown.clicked());
                        stop = Some((shown.rect, shown.interact_rect));
                        let shown = Action::quiet("Add one")
                            .icon(crate::design::icons::ERROR)
                            .line_start()
                            .show(ui);
                        presses += usize::from(shown.clicked());
                        icon = Some((shown.rect, shown.interact_rect));
                    }
                    let label = ui.label("A plain label").rect;
                    (stop, icon, label, ui.clip_rect(), presses)
                })
                .body;
            let presses;
            (state.stop, state.icon, state.label, state.clip, presses) = drawn;
            state.presses += presses;
        };
        // 1 and 3: a 16 px page margin at the pane's left.
        let mut margin = egui_kittest::Harness::builder()
            .with_size(vec2(700.0, 300.0))
            .build_ui_state(
                move |ui, state: &mut State| {
                    let room = ui.available_rect_before_wrap();
                    let inner = room.with_min_x(room.left() + crate::design::tokens::L);
                    ui.scope_builder(egui::UiBuilder::new().max_rect(inner), |ui| pane(ui, state));
                },
                State::default(),
            );
        crate::design::theme::install(&margin.ctx);
        margin.run_steps(3);
        let before = margin.state().label;
        margin.state_mut().action = true;
        margin.run_steps(1);
        let first = margin.state().label;
        margin.run_steps(3);
        let state = margin.state();
        assert!(
            (first.left() - before.left()).abs() <= 0.5
                && (state.label.left() - before.left()).abs() <= 0.5,
            "gutter-still: the text edge {} stays when a line_start action first draws: {} then {}",
            before.left(),
            first.left(),
            state.label.left()
        );
        let (stop, icon) = (
            state.stop.expect("Stop"),
            state.icon.expect("the icon action"),
        );
        // `Stop` hangs 8 px: whole inside the clip. The icon action hangs 28 px, more than this
        // margin: the clip reaches as far as the parent lets it (the window), no further.
        assert!(
            state.clip.expand(0.5).contains_rect(stop.0),
            "gutter-inside: Stop {:?} is whole inside the clip {:?}",
            stop.0,
            state.clip
        );
        let edge = state.label.left();
        assert!(
            (state.clip.left() - (edge - crate::design::components::LINE_START_REACH).max(0.0))
                .abs()
                <= 0.5,
            "gutter-reach: the clip {:?} reaches the icon action's hang left of the text edge {edge}, inside the parent",
            state.clip
        );
        // P3a: the icon action's 28 px hang is longer than the room the window leaves; it stays
        // whole from the clip (it gives up that much of its padding; `components` proves where
        // its words go).
        assert!(
            state.clip.left() > edge - crate::design::components::LINE_START_REACH + 0.5,
            "gutter-short-pressure: the room {:?} is shorter than the icon's hang from {edge}",
            state.clip
        );
        assert!(
            (stop.0.left() + LINE_START_HANG - state.label.left()).abs() <= 0.5
                && (icon.0.left() - state.clip.left()).abs() <= 0.5
                && state.clip.expand(0.5).contains_rect(icon.0),
            "gutter-edge: Stop's words {stop:?} start at the label's edge {:?}; the icon action {icon:?} is whole from the clip {:?}",
            state.label,
            state.clip
        );
        // P3a (the nested scroll): the pane's own scroll column is a surface the gate and egui
        // read as the column's room. It starts where the clip starts, so it covers the hang it
        // paints; it does not end at the text edge. Pressure first: Stop really hangs past it.
        assert!(
            stop.0.left() < edge - 0.5,
            "gutter-pressure: Stop {stop:?} hangs left of the text edge {edge}"
        );
        let surface = pane_records(&margin.ctx)
            .into_iter()
            .find_map(|record| record.viewport())
            .expect("the Content pane's column");
        assert!(
            (surface.left() - state.clip.left()).abs() <= 0.5
                && surface.left() <= stop.0.left() + 0.5,
            "gutter-surface: the column's surface {surface:?} starts at the clip {:?} and covers Stop {:?}",
            state.clip,
            stop.0
        );
        // 4 (P3a, lead point 1): beside a splitter, in the FIRST displayed pass (no settling),
        // both neighbour orders: a pane in the page right of a left column, and a pane inside a
        // right column (the run's panel). The text edge is `S` from the splitter; the column
        // reserved its grab band before any body drew, so the hang's room, the clip, the surface
        // and the press area start exactly at the band's right edge, and `Stop` is whole there.
        let grab = egui::Style::default().interaction.resize_grab_radius_side;
        for right_column in [false, true] {
            let first = egui_kittest::Harness::builder()
                .with_size(vec2(700.0, 300.0))
                .build_ui_state(
                    move |ui, state: &mut State| {
                        // The builder runs the first pass: the theme is in it too.
                        crate::design::theme::install(ui.ctx());
                        let at_edge = |ui: &mut Ui, state: &mut State| {
                            state.parent = ui.clip_rect();
                            let room = ui.available_rect_before_wrap();
                            let inner = room.with_min_x(room.left() + S);
                            ui.scope_builder(egui::UiBuilder::new().max_rect(inner), |ui| {
                                pane(ui, state)
                            });
                        };
                        if right_column {
                            crate::design::Column::right("lookup", 300.0, 200.0..=400.0)
                                .show_inside(ui, |ui| at_edge(ui, state));
                            egui::CentralPanel::default()
                                .frame(egui::Frame::new())
                                .show_inside(ui, |ui| ui.label("Page"));
                        } else {
                            crate::design::Column::left("navigation", 200.0, 150.0..=300.0)
                                .show_inside(ui, |ui| ui.label("Left"));
                            egui::CentralPanel::default()
                                .frame(egui::Frame::new())
                                .show_inside(ui, |ui| at_edge(ui, state));
                        }
                    },
                    State {
                        action: true,
                        ..State::default()
                    },
                );
            // No step: the state is the first displayed pass's (the builder's own pass).
            let state = first.state();
            let (stop, hit) = state.stop.expect("Stop in the first pass");
            // The splitter's x: the right column's left edge, or the page's left edge.
            let splitter = state.parent.left();
            let band = splitter + grab;
            let edge = state.label.left();
            assert!(
                (edge - splitter - S).abs() <= 0.5 && edge - LINE_START_HANG < band - 0.5,
                "gutter-first-pressure ({right_column}): the text edge {edge} is S from the splitter {splitter}, so Stop's hang reaches into its band (to {band})"
            );
            let surface = pane_records(&first.ctx)
                .into_iter()
                .find_map(|record| record.viewport());
            assert!(
                (state.clip.left() - band).abs() <= 0.5
                    && surface.is_none_or(|surface| (surface.left() - band).abs() <= 0.5)
                    && stop.left() >= band - 0.5
                    && state.clip.expand(0.5).contains_rect(stop)
                    && hit.left() >= band - 0.5,
                "gutter-first ({right_column}): in the first pass the clip {:?}, the surface {surface:?}, Stop {stop:?} and its press area {hit:?} start at the band's right edge {band}",
                state.clip
            );
        }
        // 5 (lead point 2): no stale room. A left column's band in one pass; in the next, the
        // column is gone and the pane keeps its text edge: its clip is the plain reach at once.
        let mut stale = egui_kittest::Harness::builder()
            .with_size(vec2(700.0, 300.0))
            .build_ui_state(
                move |ui, state: &mut State| {
                    let column = state.action;
                    state.parent = ui.clip_rect();
                    if column {
                        crate::design::Column::left("navigation", 200.0, 150.0..=300.0)
                            .show_inside(ui, |ui| ui.label("Left"));
                    }
                    egui::CentralPanel::default()
                        .frame(egui::Frame::new())
                        .show_inside(ui, |ui| {
                            let room = ui.available_rect_before_wrap();
                            // The same text edge with and without the column.
                            let inner = room.with_min_x(208.0 + S);
                            ui.scope_builder(egui::UiBuilder::new().max_rect(inner), |ui| {
                                pane(ui, state)
                            });
                        });
                },
                State {
                    action: true,
                    ..State::default()
                },
            );
        crate::design::theme::install(&stale.ctx);
        stale.run_steps(3);
        let with_band = stale.state().clip.left();
        stale.state_mut().action = false;
        stale.step();
        let edge = 208.0 + S;
        let after = stale.state().clip.left();
        assert!(
            with_band > edge - crate::design::components::LINE_START_REACH + 0.5,
            "gutter-stale-pressure: with the column the band limits the room ({with_band})"
        );
        assert!(
            (after - (edge - crate::design::components::LINE_START_REACH)).abs() <= 0.5,
            "gutter-stale: the pass after the column goes, the clip {after} is the plain reach {}, not the old band's {with_band}",
            edge - crate::design::components::LINE_START_REACH
        );
        // 6 (lead point 3; Astra 4): two columns in a `Fills` body, the second `S` right of the
        // first: the second column's surface and hang limit start EXACTLY at the first's lane,
        // and its icon action (28 px hang, 20 px short) is whole there, in both draw orders when
        // the layout owner reserves both lanes first (`reserve_lanes`). Right first without it,
        // the late lane is only recorded (`late_bands`, test builds); nothing protects that pass.
        let two_columns = |right_first: bool, lanes_first: bool| {
            let mut columns = egui_kittest::Harness::builder()
                .with_size(vec2(700.0, 300.0))
                .build_ui_state(
                    move |ui, seen: &mut (Rect, Rect, Rect, Rect, Rect)| {
                        let frame = PaneFrame::new(ui);
                        let messages = Messages::<NoPress>::new(&frame, "gutter_fills", 680.0);
                        *seen = messages
                            .pane(
                                ui,
                                Body::Fills(Fills::for_test(FillsReason::MarkdownEditor)),
                                |ui| {
                                    let room = ui.available_rect_before_wrap();
                                    let left = room.with_max_x(room.left() + 200.0);
                                    let right = room.with_min_x(left.right() + S);
                                    if lanes_first {
                                        reserve_lanes(ui, &[left, right]);
                                    }
                                    let list = |ui: &mut Ui| {
                                        ui.scope_builder(
                                            egui::UiBuilder::new().max_rect(left),
                                            |ui| {
                                                ScrollColumn::new("gutter_list").show(ui, |ui| {
                                                    for row in 0..60 {
                                                        ui.label(format!("Row {row}"));
                                                    }
                                                })
                                            },
                                        );
                                    };
                                    let reader = |ui: &mut Ui| {
                                        ui.scope_builder(
                                            egui::UiBuilder::new().max_rect(right),
                                            |ui| {
                                                ScrollColumn::new("gutter_reader").show(ui, |ui| {
                                                    let shown = Action::quiet("Add one")
                                                        .icon(crate::design::icons::ERROR)
                                                        .line_start()
                                                        .show(ui);
                                                    let clip = ui.clip_rect();
                                                    let limit = Rect::from_min_max(
                                                        pos2(
                                                            hang_limit(ui, Rect::NOTHING),
                                                            clip.top(),
                                                        ),
                                                        clip.max,
                                                    );
                                                    (shown.rect, shown.interact_rect, limit)
                                                })
                                            },
                                        )
                                        .inner
                                    };
                                    let out = if right_first {
                                        let out = reader(ui);
                                        list(ui);
                                        out
                                    } else {
                                        list(ui);
                                        reader(ui)
                                    };
                                    let (rect, hit, clip) = out.inner;
                                    (rect, hit, clip, left, out.inner_rect)
                                },
                            )
                            .body;
                    },
                    (
                        Rect::NOTHING,
                        Rect::NOTHING,
                        Rect::NOTHING,
                        Rect::NOTHING,
                        Rect::NOTHING,
                    ),
                );
            crate::design::theme::install(&columns.ctx);
            columns.run_steps(4);
            (*columns.state(), late_bands(&columns.ctx).len())
        };
        for (right_first, lanes_first) in [(false, false), (false, true), (true, true)] {
            let ((rect, hit, clip, list, surface), late) = two_columns(right_first, lanes_first);
            assert_eq!(
                late, 0,
                "gutter-columns: no late lane when the lanes come first"
            );
            let edge = list.right() + S;
            assert!(
                edge - crate::design::components::LINE_START_REACH < list.right() - 0.5,
                "gutter-columns-pressure: the icon action's hang from {edge} reaches into the list column {list:?}"
            );
            assert!(
                (clip.left() - list.right()).abs() <= 0.5
                    && (surface.left() - list.right()).abs() <= 0.5
                    && (rect.left() - list.right()).abs() <= 0.5
                    && clip.expand(0.5).contains_rect(rect)
                    && (hit.left() - rect.left()).abs() <= 0.5,
                "gutter-columns (right first {right_first}, lanes first {lanes_first}): the limit {clip:?} and surface {surface:?} start exactly at the list's edge {list:?}; the icon action {rect:?} is whole from there (press area {hit:?})"
            );
        }
        let (_, late) = two_columns(true, false);
        assert!(
            late > 0,
            "gutter-columns-late: a lane reserved after the column right of it drew is recorded"
        );
        // 7 (lead point 5): a pane its parent cuts (parent clip from 100, body from 80): the body
        // clip never widens the parent's.
        let mut cut = egui_kittest::Harness::builder()
            .with_size(vec2(700.0, 300.0))
            .build_ui_state(
                move |ui, state: &mut State| {
                    let room = ui.available_rect_before_wrap();
                    ui.scope_builder(
                        egui::UiBuilder::new().max_rect(room.with_min_x(80.0)),
                        |ui| {
                            let mut clip = ui.clip_rect();
                            clip.min.x = 100.0;
                            ui.set_clip_rect(clip);
                            state.parent = ui.clip_rect();
                            pane(ui, state);
                        },
                    );
                },
                State {
                    action: true,
                    ..State::default()
                },
            );
        crate::design::theme::install(&cut.ctx);
        cut.run_steps(3);
        let state = cut.state();
        assert!(
            state.label.left() < state.parent.left() - 0.5,
            "gutter-parent-cut-pressure: the body's edge {:?} is left of the parent's clip {:?}",
            state.label,
            state.parent
        );
        assert!(
            (state.clip.left() - state.parent.left()).abs() <= 0.5,
            "gutter-parent-cut: the body clip {:?} keeps the parent's left {:?}",
            state.clip,
            state.parent
        );
        // Astra 2: the text edge is 20 px outside the clip: `Stop` moves past it, whole, and
        // takes a press on all of itself.
        let (stop, hit) = state.stop.expect("Stop");
        assert!(
            (stop.left() - state.parent.left()).abs() <= 0.5
                && stop.height() >= 24.0
                && stop.width() >= 30.0
                && (hit.left() - stop.left()).abs() <= 0.5
                && (hit.width() - stop.width()).abs() <= 0.5
                && hit.height() >= 24.0,
            "gutter-parent-cut-whole: Stop {stop:?} starts at the clip {:?}, whole, and its press area {hit:?} is all of it",
            state.parent
        );
        // 8 (ruling R19, proofs 1 and 4): with the pointer outside the action's room, ONE input
        // frame moves it onto the action's first visible pixel, presses and releases: exactly one
        // activation. Beside a parent's clip (no other target), and in a real menu (the popup's
        // background takes a press). A press just outside the edge may reach the action by
        // egui's nearest-target rule, as for every control (R19).
        let first_pixel_frame = |menu: bool| {
            let mut press = egui_kittest::Harness::builder()
                .with_size(vec2(700.0, 400.0))
                .build_ui_state(
                    move |ui, state: &mut (Rect, Rect, usize)| {
                        let choice = |ui: &mut Ui, state: &mut (Rect, Rect, usize)| {
                            let shown = Action::quiet("Roll")
                                .density(crate::design::Density::Read)
                                .line_start()
                                .show(ui);
                            state.2 += usize::from(shown.clicked());
                            (state.0, state.1) = (shown.rect, ui.clip_rect());
                        };
                        if menu {
                            let button = crate::design::menu_button(ui, "Roll menu", false);
                            crate::design::menu_popup(&*ui, &button)
                                .open(true)
                                .show(|ui| choice(ui, state));
                        } else {
                            let room = ui.available_rect_before_wrap();
                            ui.scope_builder(
                                egui::UiBuilder::new().max_rect(room.with_min_x(100.0)),
                                |ui| {
                                    let mut clip = ui.clip_rect();
                                    clip.min.x = 100.0;
                                    ui.set_clip_rect(clip);
                                    choice(ui, state);
                                },
                            );
                        }
                    },
                    (Rect::NOTHING, Rect::NOTHING, 0),
                );
            crate::design::theme::install(&press.ctx);
            press.run_steps(4);
            let (choice, clip, _) = *press.state();
            let first = pos2(clip.left() + 0.5, choice.center().y);
            assert!(
                (choice.left() - clip.left()).abs() <= 0.5,
                "gutter-first-pixel-pressure (menu {menu}): the action {choice:?} starts at the clip {clip:?}"
            );
            // The pointer rests outside the room first.
            crate::input_frame::one_pass(
                &mut press,
                [egui::Event::PointerMoved(pos2(
                    clip.left() - 40.0,
                    choice.center().y,
                ))],
            );
            press.run_steps(2);
            let before = press.state().2;
            crate::input_frame::one_pass(
                &mut press,
                [
                    egui::Event::PointerMoved(first),
                    egui::Event::PointerButton {
                        pos: first,
                        button: egui::PointerButton::Primary,
                        pressed: true,
                        modifiers: egui::Modifiers::NONE,
                    },
                    egui::Event::PointerButton {
                        pos: first,
                        button: egui::PointerButton::Primary,
                        pressed: false,
                        modifiers: egui::Modifiers::NONE,
                    },
                ],
            );
            assert_eq!(
                press.state().2 - before,
                1,
                "gutter-first-pixel (menu {menu}): one frame that moves onto {first:?} and clicks activates Roll {choice:?} once"
            );
        };
        first_pixel_frame(false);
        first_pixel_frame(true);
        // 9 (R19, proof 2): a press directly on the scroll lane of the column beside the action
        // goes to the lane (the list scrolls), never to the icon action that hangs toward it.
        let mut lane = egui_kittest::Harness::builder()
            .with_size(vec2(700.0, 300.0))
            .build_ui_state(
                move |ui, seen: &mut (Rect, Rect, f32, usize)| {
                    let frame = PaneFrame::new(ui);
                    let messages = Messages::<NoPress>::new(&frame, "gutter_lane", 680.0);
                    let (list, rect, offset, clicked) = messages
                        .pane(
                            ui,
                            Body::Fills(Fills::for_test(FillsReason::MarkdownEditor)),
                            |ui| {
                                let room = ui.available_rect_before_wrap();
                                let left = room.with_max_x(room.left() + 200.0);
                                let right = room.with_min_x(left.right() + S);
                                reserve_lanes(ui, &[left, right]);
                                let offset = ui
                                    .scope_builder(egui::UiBuilder::new().max_rect(left), |ui| {
                                        ScrollColumn::new("gutter_lane_list").show(ui, |ui| {
                                            for row in 0..60 {
                                                ui.label(format!("Row {row}"));
                                            }
                                        })
                                    })
                                    .inner
                                    .state
                                    .offset
                                    .y;
                                let (rect, clicked) = ui
                                    .scope_builder(egui::UiBuilder::new().max_rect(right), |ui| {
                                        ScrollColumn::new("gutter_lane_reader")
                                            .show(ui, |ui| {
                                                let shown = Action::quiet("Add one")
                                                    .icon(crate::design::icons::ERROR)
                                                    .line_start()
                                                    .show(ui);
                                                (shown.rect, shown.clicked())
                                            })
                                            .inner
                                    })
                                    .inner;
                                (left, rect, offset, clicked)
                            },
                        )
                        .body;
                    seen.0 = list;
                    seen.1 = rect;
                    seen.2 = offset;
                    seen.3 += usize::from(clicked);
                },
                (Rect::NOTHING, Rect::NOTHING, 0.0, 0),
            );
        crate::design::theme::install(&lane.ctx);
        lane.run_steps(4);
        let (list, rect, _, _) = *lane.state();
        let on_lane = pos2(list.right() - 3.0, rect.center().y);
        assert!(
            rect.left() > on_lane.x
                && on_lane.x - (list.right() + S - crate::design::components::LINE_START_REACH)
                    > 0.0,
            "gutter-lane-pressure: the press {on_lane:?} is on the lane, inside the icon action's unrestricted hang, left of the action {rect:?}"
        );
        crate::input_frame::one_pass(
            &mut lane,
            [
                egui::Event::PointerMoved(on_lane),
                egui::Event::PointerButton {
                    pos: on_lane,
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                },
            ],
        );
        crate::input_frame::next_pass(
            &mut lane,
            [egui::Event::PointerMoved(on_lane + vec2(0.0, 60.0))],
        );
        crate::input_frame::next_pass(
            &mut lane,
            [egui::Event::PointerButton {
                pos: on_lane + vec2(0.0, 60.0),
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            }],
        );
        lane.run_steps(2);
        let (_, _, offset, clicked) = *lane.state();
        assert!(
            offset > 0.0 && clicked == 0,
            "gutter-lane: the press on the lane scrolls the list (offset {offset}) and never activates the action ({clicked})"
        );
        // 2: flush against a splitter.
        let mut flush = egui_kittest::Harness::builder()
            .with_size(vec2(700.0, 300.0))
            .build_ui_state(
                move |ui, state: &mut State| {
                    crate::design::Column::left("navigation", 200.0, 150.0..=300.0)
                        .show_inside(ui, |ui| ui.label("Left"));
                    egui::CentralPanel::default()
                        .frame(egui::Frame::new())
                        .show_inside(ui, |ui| {
                            state.parent = ui.clip_rect();
                            pane(ui, state);
                        });
                },
                State {
                    action: true,
                    ..State::default()
                },
            );
        crate::design::theme::install(&flush.ctx);
        flush.run_steps(3);
        let splitter = flush
            .query_all(
                egui_kittest::kittest::By::new()
                    .predicate(|node| node.role() == egui::accesskit::Role::Splitter),
            )
            .find_map(|node| {
                let b = node.accesskit_node().bounding_box()?;
                Some(Rect::from_min_max(
                    pos2(b.x0 as f32, b.y0 as f32),
                    pos2(b.x1 as f32, b.y1 as f32),
                ))
            })
            .expect("the column's splitter");
        let state = flush.state();
        // The parent's clip wins: the hang adds nothing left of it. (Here the parent's clip starts
        // inside the splitter's grab band, so content at the pane's own text edge overlaps the
        // band too: a caller's layout fault, which the gate reports; the pane does not hide it.)
        // Astra 3: the splitter's reserved band crosses the text edge (edge at the splitter,
        // band to splitter + 5): both actions move clear of all of it, paint and press.
        let band = state.parent.left() + egui::Style::default().interaction.resize_grab_radius_side;
        assert!(
            state.label.left() < band - 0.5,
            "gutter-band-pressure: the text edge {:?} is inside the band to {band}",
            state.label
        );
        for (name, (rect, hit)) in [
            ("Stop", state.stop.expect("Stop")),
            ("Add one", state.icon.expect("the icon action")),
        ] {
            assert!(
                state.clip.left() >= state.parent.left() - 0.5
                    && rect.left() >= band - 0.5
                    && hit.left() >= band - 0.5,
                "gutter-band: {name} {rect:?} and its press area {hit:?} start clear of the band to {band} (clip {:?}, splitter {splitter:?})",
                state.clip
            );
            assert!(
                hit.height() >= 24.0,
                "gutter-band-height: {name}'s press area {hit:?} is at least 24 px high"
            );
        }
        // R19, proof 2: a press directly on the band beside `Stop` is the splitter's (its
        // handle is pressed), never `Stop`'s.
        let stop = state.stop.expect("Stop").0;
        let on_band = pos2(band - 2.0, stop.center().y);
        let handle = Id::new("navigation").with("__resize");
        crate::input_frame::one_pass(
            &mut flush,
            [
                egui::Event::PointerMoved(on_band),
                egui::Event::PointerButton {
                    pos: on_band,
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                },
            ],
        );
        let held = flush
            .ctx
            .read_response(handle)
            .is_some_and(|response| response.is_pointer_button_down_on());
        crate::input_frame::next_pass(
            &mut flush,
            [egui::Event::PointerButton {
                pos: on_band,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            }],
        );
        assert!(
            held && flush.state().presses == 0,
            "gutter-band-press: a press at {on_band:?} on the band is the splitter's ({held}), Stop's count {}",
            flush.state().presses
        );
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
        text: crate::design::text_draft::TextDraft,
        save: Option<Rect>,
        caret: Option<Rect>,
        focus: bool,
        /// A caret placed from code (a char index), with focus, after the editor draws.
        set_caret: Option<usize>,
        /// Lay the document out with a face that folds (`Face::Laid { folds: true }`).
        folds: bool,
        /// Each frame's `changed()` of the editor, and the text its galley holds.
        changed: Vec<bool>,
        galley_text: String,
    }

    fn harness(height: f32, state: State) -> egui_kittest::Harness<'static, State> {
        harness_after(height, state, 4)
    }

    /// The harness after `steps` frames: 1 is the first frame shown.
    fn harness_after(
        height: f32,
        state: State,
        steps: usize,
    ) -> egui_kittest::Harness<'static, State> {
        harness_timed(height, state, steps, 1.0 / 4.0)
    }

    /// As [`harness_after`], with `dt` seconds per frame (1/60: a real frame, so an egui
    /// animation is still running two frames later).
    fn harness_timed(
        height: f32,
        state: State,
        steps: usize,
        dt: f32,
    ) -> egui_kittest::Harness<'static, State> {
        let mut harness = egui_kittest::Harness::builder()
            .with_size(vec2(600.0, height))
            .with_step_dt(dt)
            .build_ui_state(
                |ui, state: &mut State| {
                    let want = state.lines;
                    // `lines` 0: the test owns the text (a document, the differential script).
                    if want > 0 && state.text.lines().count() != want {
                        let _ = state.text.replace(
                            (0..want)
                                .map(|line| format!("Line {line}"))
                                .collect::<Vec<_>>()
                                .join("\n"),
                        );
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
                    let set_caret = std::mem::take(&mut state.set_caret);
                    let text = &mut state.text;
                    // Save at the column's right edge, as the app's save row ends.
                    let save_row = |ui: &mut Ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                            ui.button("Save").rect
                        })
                        .inner
                    };
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
                                    let face = if state.folds {
                                        Face::Laid {
                                            layout: laid_plainly,
                                            font: egui::TextStyle::Body.resolve(ui.style()),
                                            folds: true,
                                        }
                                    } else {
                                        Face::Font(egui::TextStyle::Body.resolve(ui.style()))
                                    };
                                    let output = editor.editor(
                                        ui,
                                        FIELD.once(Once::Example),
                                        text,
                                        face,
                                        |edit| edit,
                                    );
                                    state.changed.push(output.response.changed());
                                    state.galley_text.clone_from(&output.galley.job.text);
                                    if focus {
                                        output.response.request_focus();
                                    }
                                    if let Some(at) = set_caret {
                                        output.response.request_focus();
                                        let id = output.response.id;
                                        let mut edit = egui::TextEdit::load_state(ui.ctx(), id)
                                            .unwrap_or_default();
                                        edit.cursor.set_char_range(Some(
                                            egui::text::CCursorRange::one(
                                                egui::text::CCursor::new(at),
                                            ),
                                        ));
                                        edit.store(ui.ctx(), id);
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
        harness.run_steps(steps);
        harness
    }

    /// The footer's rect as the pane placed it in the last pass (the first frame after a resize
    /// included; a response read back can be the frame before's).
    fn footer(harness: &egui_kittest::Harness<'_, State>) -> Option<Rect> {
        pane_record(&harness.ctx, Id::new(LOCAL)).and_then(|record| record.footer())
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

    /// The region's scroll offset as painted: the view's top less the top of the editor's text.
    fn painted_offset(harness: &egui_kittest::Harness<'_, State>) -> f32 {
        harness
            .output()
            .shapes
            .iter()
            .find_map(|clipped| match &clipped.shape {
                egui::Shape::Text(text) if text.galley.job.text.starts_with("Line 0\nLine 1") => {
                    Some(clipped.clip_rect.top() - text.pos.y)
                }
                _ => None,
            })
            .expect("the editor's text is painted")
    }

    /// Layout PR 9 (I10): the editor's fixed frame. All four outside edges are in view, the
    /// frame and Save end at one outside right edge, the inner scroll view (text and lane) ends
    /// at the frame's inner margin (the lane is inside the frame, subtracted once), and the
    /// painted text is inside that view.
    fn frame_whole(harness: &egui_kittest::Harness<'_, State>, case: &str, save: Rect) {
        let frames = editor_frames(&harness.ctx);
        let [record] = frames.as_slice() else {
            panic!("i10-one-{case}: one editor frame, not {frames:?}");
        };
        let frame = record.frame();
        assert!(
            record.whole() && !record.least(),
            "i10-whole-{case}: the frame {frame:?} is whole in its clip {:?}",
            record.clip()
        );
        assert!(
            (frame.right() - save.right()).abs() <= 0.5,
            "i10-edge-{case}: the frame {frame:?} and Save {save:?} share their outside right edge"
        );
        // The painted stroke is that frame, whole in its clip: not a frame inside the scroll.
        let painted: Vec<(Rect, Rect)> = harness
            .output()
            .shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Rect(rect)
                    if rect.stroke.color == super::super::tokens::colors().field_border =>
                {
                    Some((rect.rect, clipped.clip_rect))
                }
                _ => None,
            })
            .collect();
        assert!(
            painted.iter().any(|(rect, clip)| {
                (rect.min - frame.min).length() <= 0.5
                    && (rect.max - frame.max).length() <= 0.5
                    && clip.expand(0.5).contains_rect(*rect)
            }),
            "i10-stroke-{case}: the painted frame strokes {painted:?} include the frame {frame:?}, whole in its clip"
        );
        let inner = frame.shrink(S + super::super::tokens::HAIRLINE_WIDTH);
        let lane = super::super::layout::lane_width();
        // egui's view excludes its bar while the bar shows: it ends at the frame's inside, or
        // one lane before it.
        let view = record.view();
        assert!(
            view.right() <= inner.right() + 0.5
                && view.right() >= inner.right() - lane - 0.5
                && view.top() >= inner.top() - 0.5
                && view.bottom() <= inner.bottom() + 0.5,
            "i10-lane-{case}: the scroll view {view:?} is the frame's inside {inner:?}, its lane included"
        );
        let text = harness
            .output()
            .shapes
            .iter()
            .find_map(|clipped| match &clipped.shape {
                egui::Shape::Text(text) if text.galley.job.text.starts_with("Line 0\nLine 1") => {
                    Some(clipped.clip_rect)
                }
                _ => None,
            })
            .expect("the editor's text is painted");
        // egui paints a text edit's text with a clip one point wider than its rect, and clips a
        // scroll's content `clip_rect_margin` outside its view. The text ends one lane
        // (subtracted once) before the frame's inside right edge.
        let margin = harness.ctx.style().visuals.clip_rect_margin;
        assert!(
            view.expand(margin + 0.5).contains_rect(text)
                && (text.right() - 1.0 - (inner.right() - lane)).abs() <= 0.5,
            "i10-text-{case}: the text (clip {text:?}) is inside the view {view:?}, ending one \
             lane before the frame's inside {inner:?}"
        );
    }

    /// Round 28, P2: a pane's editor region. A short draft: the footer is `S` under the save
    /// row, as for `Body::Content`. A 600-line draft at 300 px: the save row is whole in the room
    /// above the footer, the region scrolls, the caret at the end is in view; the footer gains a
    /// line: the save row stays whole in view, and so does the caret. Growing past the room, the
    /// save row never moves by more than the growth. A body that does not use the region is
    /// `Body::Content`.
    /// PR 343 review: a caret set from code outside the region's view (a broken line's repair)
    /// is in view, centred, after the next pass; then the DM's wheel with the caret unmoved is
    /// never pulled back.
    #[test]
    fn an_editor_region_reveals_a_caret_set_outside_its_view() {
        let mut harness = harness(
            300.0,
            State {
                lines: 200,
                messages: 1,
                region: true,
                ..Default::default()
            },
        );
        let at: usize = (0..150).map(|line| format!("Line {line}").len() + 1).sum();
        harness.state_mut().set_caret = Some(at);
        harness.run_steps(3);
        let view = region_view(&harness);
        let caret = harness.state().caret.expect("a focused caret");
        assert!(
            view.contains_rect(caret),
            "reveal: the caret {caret:?} set at line 150 is in the view {view:?}"
        );
        assert!(
            (caret.center().y - view.center().y).abs() <= caret.height(),
            "reveal-centred: the caret {caret:?} is near the view's centre {view:?}"
        );
        harness.hover_at(view.center());
        harness.event(egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: vec2(0.0, 400.0),
            modifiers: egui::Modifiers::NONE,
        });
        harness.run_steps(4);
        let (wheeled, kept) = (region_view(&harness), harness.state().caret);
        assert!(
            kept.is_some_and(|kept| kept.top() > wheeled.bottom()),
            "reveal-wheel: after the wheel, the unmoved caret {kept:?} stays below the view {wheeled:?}"
        );
    }

    /// PR 343 re-check (P2): a reveal that is still pending when the region's height changes
    /// (a long broken file opened, then the window shrunk before the reveal ran) is computed
    /// from the NEW height: the caret is in the view of the first frame after the resize. One
    /// pass per frame, so the reveal waits for the next frame, as it does when the pass budget
    /// is spent.
    #[test]
    fn a_pending_reveal_uses_the_region_height_of_its_own_pass() {
        let mut harness = harness(
            700.0,
            State {
                lines: 200,
                messages: 1,
                region: true,
                ..Default::default()
            },
        );
        harness
            .ctx
            .options_mut(|options| options.max_passes = std::num::NonZeroUsize::MIN);
        let tall = region_view(&harness);
        let at: usize = (0..150).map(|line| format!("Line {line}").len() + 1).sum();
        harness.state_mut().set_caret = Some(at);
        // Frame 1 places the caret; frame 2 sees it outside the view: a reveal is pending.
        harness.run_steps(2);
        harness.set_size(vec2(600.0, 200.0));
        harness.step();
        let view = region_view(&harness);
        let caret = harness.state().caret.expect("a focused caret");
        assert!(
            view.height() < tall.height() - 100.0,
            "pending-resize-setup: the view shrank from {tall:?} to {view:?}"
        );
        assert!(
            view.contains_rect(caret),
            "pending-resize: the caret {caret:?} at line 150 is in the shrunk view {view:?} \
             (the tall view was {tall:?})"
        );
    }

    /// PR 343 re-check (P3): Shift+Down past the view's bottom edge. The region is the one
    /// writer of its offset: no animated egui target competes with it, so the first frame after the
    /// key and the frame after it (real frame times) show the same offset, with the caret in view.
    #[test]
    fn a_caret_moved_past_the_view_by_a_key_settles_at_once() {
        let mut harness = harness_timed(
            300.0,
            State {
                lines: 200,
                messages: 1,
                region: true,
                ..Default::default()
            },
            4,
            1.0 / 60.0,
        );
        let view = region_view(&harness);
        // The line before the last whole line in the view: two Shift+Down take the caret past
        // the view's bottom.
        let last = ((view.height() / 18.0).floor() as usize).saturating_sub(2);
        let rest = painted_offset(&harness);
        let at: usize = (0..last).map(|line| format!("Line {line}").len() + 1).sum();
        harness.state_mut().set_caret = Some(at);
        harness.run_steps(3);
        let before = harness.state().caret.expect("a focused caret");
        assert!(
            region_view(&harness).contains_rect(before),
            "key-setup: the caret {before:?} starts in the view {view:?}"
        );
        let start = painted_offset(&harness);
        assert!(
            (start - rest).abs() < 0.5,
            "key-setup: the view did not move ({rest} to {start})"
        );
        harness.key_press_modifiers(egui::Modifiers::SHIFT, egui::Key::ArrowDown);
        harness.key_press_modifiers(egui::Modifiers::SHIFT, egui::Key::ArrowDown);
        // The key frames (the first frame shown after the key), then one more frame.
        harness.step();
        let (first, first_offset) = (harness.state().caret, painted_offset(&harness));
        harness.step();
        let (second, second_offset) = (harness.state().caret, painted_offset(&harness));
        let view = region_view(&harness);
        assert!(
            first_offset > start + 18.0 && (first_offset - second_offset).abs() <= 0.5,
            "key-stable: the offset moved down from {start} and stays: {first_offset} then \
             {second_offset}"
        );
        assert!(
            first == second && second.is_some_and(|second| view.contains_rect(second)),
            "key-in-view: the caret {first:?} then {second:?} is in the view {view:?}"
        );
        // Twenty Shift+Down in one frame: the caret goes more than a view past the edge; the
        // region centres it while egui's own ask would stop at the edge, so with egui's animated
        // ask left in place, the offset would go on moving toward that edge.
        let start = second_offset;
        // Twenty physical presses, each with its release, in one input frame (T1: a press with
        // no release is a repeat to egui).
        let key = |pressed| egui::Event::Key {
            key: egui::Key::ArrowDown,
            physical_key: None,
            pressed,
            repeat: false,
            modifiers: egui::Modifiers::SHIFT,
        };
        harness.input_mut().modifiers = egui::Modifiers::SHIFT;
        crate::input_frame::one_pass(&mut harness, (0..20).flat_map(|_| [key(true), key(false)]));
        harness.input_mut().modifiers = egui::Modifiers::NONE;
        let (first, first_offset) = (harness.state().caret, painted_offset(&harness));
        harness.step();
        let (second, second_offset) = (harness.state().caret, painted_offset(&harness));
        let view = region_view(&harness);
        assert!(
            first_offset > start + view.height() && (first_offset - second_offset).abs() <= 0.5,
            "burst-stable: the offset moved down from {start} by more than a view and stays: \
             {first_offset} then {second_offset}"
        );
        assert!(
            first == second && second.is_some_and(|second| view.contains_rect(second)),
            "burst-in-view: the caret {first:?} then {second:?} is in the view {view:?}"
        );
    }

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
        // ... because the content is short, not because the region filled the room.
        let room = pane_record(&short.ctx, Id::new(LOCAL))
            .expect("a record")
            .room();
        assert!(
            at.bottom() < room.bottom() - 4.0 * super::super::tokens::XXL,
            "p2-short-follows: the footer {at:?} follows the short content, far above the room's bottom {room:?}"
        );
        frame_whole(&short, "short", save);
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
        let whole_at = |harness: &egui_kittest::Harness<'_, State>, case: &str, caret: bool| {
            let save = harness.state().save.expect("the save row draws");
            let room = pane_record(&harness.ctx, Id::new(LOCAL))
                .expect("a record")
                .room();
            // Above the footer, or the room's bottom when the footer is empty.
            let at = footer(harness);
            let limit = at.map_or(room.bottom(), |at| at.top() - S);
            assert!(
                save.top() >= room.top() - 0.5 && save.bottom() <= limit + 0.5,
                "p2-save-{case}: the save row {save:?} is whole in the room {room:?} above the footer {at:?}"
            );
            let view = region_view(harness);
            if caret {
                let caret = harness.state().caret.expect("the editor has its caret");
                assert!(
                    view.expand(0.5).contains_rect(caret),
                    "p2-caret-{case}: the caret {caret:?} is in the region's view {view:?}"
                );
            }
            assert!(
                view.bottom() <= save.top() + 0.5 && view.height() < 600.0,
                "p2-region-{case}: the region {view:?} scrolls above the save row {save:?}"
            );
            frame_whole(harness, case, save);
        };
        let whole = |harness: &egui_kittest::Harness<'_, State>, case: &str| {
            whole_at(harness, case, true);
        };
        assert!(
            long.state().text.ends_with("Line 599!"),
            "p2-typed: the caret is at the end"
        );
        whole(&long, "long");
        // The footer gains a line with the caret at the end: the first frame shown keeps the
        // save row whole and the caret in view, and so do the frames after.
        long.state_mut().messages = 2;
        long.run_steps(1);
        whole(&long, "footer-grew-first-frame");
        long.run_steps(3);
        whole(&long, "footer-grew");
        // A resize, both ways: the first frame shown.
        for height in [340.0, 300.0] {
            long.set_size(vec2(600.0, height));
            long.run_steps(1);
            whole(&long, &format!("resize-{height}-first-frame"));
        }
        long.state_mut().messages = 0;
        long.run_steps(4);
        whole(&long, "footer-emptied");
        // Review of PR 799 (P2): narrower, the caret's line wraps below the view. A caret that was
        // in view stays in view, measured in the new layout, in the first frame shown.
        long.event(egui::Event::Text(
            " and then the party walked on along the old road".into(),
        ));
        long.run_steps(4);
        whole(&long, "long-line");
        for width in [300.0, 200.0] {
            long.set_size(vec2(width, 300.0));
            long.run_steps(1);
            whole(&long, &format!("narrower-{width}-first-frame"));
        }
        // Review of PR 825 (P2): only a new layout keeps the caret. The DM's wheel takes a caret
        // that was in view out of it, with no change of layout: nothing pulls it back.
        let caret_out = |harness: &egui_kittest::Harness<'_, State>| {
            let caret = harness.state().caret.expect("the editor has its caret");
            let view = region_view(harness);
            (!view.expand(0.5).contains_rect(caret), caret, view)
        };
        let view = region_view(&long);
        // A touchpad's small steps apply in their own frame (a long wheel step is spread over
        // frames): one frame takes the caret out of view, and nothing moves after it.
        long.input_mut()
            .events
            .push(egui::Event::PointerMoved(view.center()));
        for _ in 0..60 {
            long.input_mut().events.push(egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                delta: vec2(0.0, 7.0),
                modifiers: egui::Modifiers::NONE,
            });
        }
        long.run_steps(4);
        let (out, caret, view) = caret_out(&long);
        assert!(
            out,
            "p2-wheel: the wheel took the caret {caret:?} out of the view {view:?}, and it stays out"
        );
        // A named jump is its pass's one offset, with the caret in view before it, also when the
        // footer changes in the same frame (a new layout): the passes after it keep the jump's
        // place, Top and then Show.
        let jump_pane = pane_record(&long.ctx, Id::new(LOCAL))
            .expect("a record")
            .id();
        long.event(egui::Event::Text("?".into()));
        long.run_steps(4);
        whole(&long, "typed-before-jump");
        long.ctx.data_mut(|data| {
            data.insert_temp(editor_jump_id(jump_pane), EditorJump::Top);
        });
        long.state_mut().messages = 2;
        long.run_steps(4);
        let (out, caret, view) = caret_out(&long);
        let offset = painted_offset(&long);
        assert!(
            offset.abs() <= 2.0 && out,
            "p2-jump-top: the Top jump stays at offset {offset}, the caret {caret:?} out of the \
             view {view:?}"
        );
        long.event(egui::Event::Text("?".into()));
        long.run_steps(4);
        whole(&long, "typed-before-show");
        long.ctx.data_mut(|data| {
            data.insert_temp(
                editor_jump_id(jump_pane),
                EditorJump::Show {
                    top: 900.0,
                    bottom: 918.0,
                },
            );
        });
        long.state_mut().messages = 1;
        long.run_steps(4);
        let (out, caret, view) = caret_out(&long);
        let offset = painted_offset(&long);
        assert!(
            offset <= 909.0 && 909.0 <= offset + view.height() && out,
            "p2-jump-show: the Show jump keeps its place in view at offset {offset}, the caret \
             {caret:?} out of the view {view:?}"
        );

        // The first frame shown on open: the save row whole above the footer.
        let open = harness_after(
            300.0,
            State {
                lines: 600,
                messages: 1,
                region: true,
                ..State::default()
            },
            1,
        );
        whole_at(&open, "open-first-frame", false);

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
        // Each count's first frame shown.
        for lines in 3..30 {
            grow.state_mut().lines = lines;
            grow.run_steps(1);
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
            frame_whole(&grow, &format!("growth-{lines}"), save);
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

    /// A face's layout that does not fold, for a test of the caret part of the key.
    fn laid_plainly(
        ui: &Ui,
        text: &dyn egui::TextBuffer,
        wrap: f32,
    ) -> std::sync::Arc<egui::Galley> {
        let job = egui::text::LayoutJob::simple(
            text.as_str().to_owned(),
            egui::TextStyle::Body.resolve(ui.style()),
            ui.visuals().widgets.inactive.text_color(),
            wrap,
        );
        ui.fonts_mut(|fonts| fonts.layout_job(job))
    }

    /// A harness on a document (layout PR 9b) of `lines` lines, focused, after warm frames;
    /// `folds`: with a face that folds.
    fn document(lines: usize, folds: bool) -> egui_kittest::Harness<'static, State> {
        let words: String = (0..lines)
            .map(|line| format!("Line {line}"))
            .collect::<Vec<_>>()
            .join("\n");
        let mut h = harness(
            400.0,
            State {
                messages: 1,
                region: true,
                focus: true,
                text: crate::design::text_draft::TextDraft::new(words),
                folds,
                ..State::default()
            },
        );
        h.run_steps(4);
        h
    }

    fn made() -> usize {
        KEPT_LAYOUTS_MADE.with(std::cell::Cell::get)
    }

    fn hashed() -> usize {
        crate::design::layout::BOUND_TEXT_BYTES.with(std::cell::Cell::get)
    }

    /// Layout PR 9b (note v2 §3): a document lays out its text once per key. Quiet focused
    /// frames lay out and hash nothing; each part of the key, alone, lays it out again: an edit,
    /// two edits in one frame (the galley holds the draft's words in each frame, also the frame
    /// of the edit, so the layouter reads the live key on each call), the program's
    /// replacement with equal words, another draft in the same field, a resize, and a font atlas
    /// made again with the same fonts (its size and fill can come back equal). A caret move
    /// with the same words lays out again only a face that folds.
    #[test]
    fn a_document_keeps_its_layout_until_its_key_changes() {
        for folds in [false, true] {
            a_document_keeps_its_layout(folds);
        }
    }

    fn a_document_keeps_its_layout(folds: bool) {
        let mut h = document(40, folds);
        let (layouts, bytes) = (made(), hashed());
        h.run_steps(5);
        assert_eq!(
            (made() - layouts, hashed() - bytes),
            (0, 0),
            "quiet: five focused frames lay out and hash no text"
        );
        let relaid =
            |h: &mut egui_kittest::Harness<'static, State>,
             case: &str,
             change: &mut dyn FnMut(&mut egui_kittest::Harness<'static, State>)| {
                let before = made();
                change(h);
                for frame in 1..=3 {
                    h.step();
                    assert_eq!(
                        h.state().galley_text,
                        h.state().text.as_str(),
                        "{case}, folds {folds}, frame {frame}: the galley holds the draft's words"
                    );
                }
                assert!(
                    made() > before,
                    "{case}, folds {folds}: laid out again ({} more)",
                    made() - before
                );
                let quiet = made();
                h.run_steps(3);
                assert_eq!(made(), quiet, "{case}, folds {folds}: quiet again after");
            };
        let up = |h: &mut egui_kittest::Harness<'static, State>| {
            let key = |pressed| egui::Event::Key {
                key: egui::Key::ArrowUp,
                physical_key: None,
                pressed,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            };
            h.input_mut().events.extend([key(true), key(false)]);
        };
        if folds {
            relaid(&mut h, "caret", &mut |h| up(h));
        } else {
            let (before, words) = (made(), h.state().text.key());
            let caret = h.state().caret;
            up(&mut h);
            h.run_steps(3);
            assert_ne!(h.state().caret, caret, "caret: the caret moved");
            assert_eq!(
                (made(), h.state().text.key()),
                (before, words),
                "caret: a plain face keeps its layout when only the caret moves"
            );
        }
        relaid(&mut h, "edit", &mut |h| {
            h.event(egui::Event::Text("!".into()))
        });
        relaid(&mut h, "two-edits", &mut |h| {
            h.input_mut().events.push(egui::Event::Text("a".into()));
            h.input_mut().events.push(egui::Event::Text("b".into()));
        });
        relaid(&mut h, "replace-equal", &mut |h| {
            let doc = &mut h.state_mut().text;
            let words = doc.as_str().to_owned();
            let _ = doc.replace(words);
        });
        relaid(&mut h, "swap", &mut |h| {
            let doc = &mut h.state_mut().text;
            let other = doc.copy_as_new();
            *doc = other;
        });
        relaid(&mut h, "resize", &mut |h| {
            h.set_size(vec2(520.0, 400.0));
        });
        relaid(&mut h, "atlas", &mut |h| {
            // egui makes its font atlas again for a new text alpha mode, and again when it comes
            // back: the same fonts, style, size and fill as before, in a new atlas. The app's own
            // font generation does not change.
            let alpha = h.ctx.style().visuals.text_alpha_from_coverage;
            h.ctx.all_styles_mut(|style| {
                style.visuals.text_alpha_from_coverage = egui::epaint::AlphaFromCoverage::Linear;
            });
            h.step();
            h.ctx
                .all_styles_mut(|style| style.visuals.text_alpha_from_coverage = alpha);
        });
        // Eviction (note v2 §3): a field not drawn in a pass loses its kept galley at the end of
        // that pass, so a closed editor keeps no text; drawn again, it lays out again.
        let kept = |h: &mut egui_kittest::Harness<'static, State>| {
            h.ctx.memory_mut(|memory| {
                egui::cache::CacheTrait::len(memory.caches.cache::<KeptLayouts>())
            })
        };
        assert_eq!(
            kept(&mut h),
            1,
            "folds {folds}: the drawn document keeps its layout"
        );
        h.state_mut().region = false;
        h.step();
        assert_eq!(
            kept(&mut h),
            0,
            "folds {folds}: one pass not drawn drops it"
        );
        h.state_mut().region = true;
        relaid(&mut h, "drawn again", &mut |_| {});
    }

    /// Layout PR 9b (note v2 §6.4): through the real part (guard, kept layouter), a document
    /// answers each event exactly as a plain text does: the same words, caret and `changed()` in
    /// every frame, for typing, paste, an empty paste, Delete at the end, select all and type,
    /// Undo and Redo, and IME preedit, commit and cancel.
    #[test]
    fn a_document_answers_as_a_plain_text_does() {
        let words = "Ash road\nCinder pass";
        // The reference is the field part's plain text (`FieldId::multi_line`, a `String`):
        // egui's own edit with the same guard; the document is `FieldId::editor`.
        struct Field {
            plain: Option<String>,
            doc: crate::design::text_draft::TextDraft,
            changed: Vec<bool>,
        }
        let field = crate::field_id!("editor_region_test_draft").once(Once::Example);
        let make = |doc: bool| {
            let mut h = egui_kittest::Harness::builder()
                .with_size(vec2(400.0, 300.0))
                .build_ui_state(
                    move |ui, state: &mut Field| {
                        let response = match state.plain.as_mut() {
                            Some(plain) => field.multi_line(ui, plain, |edit| edit),
                            None => {
                                let face = Face::Font(egui::TextStyle::Body.resolve(ui.style()));
                                field.editor(ui, &mut state.doc, face, |edit| edit).response
                            }
                        };
                        state.changed.push(response.changed());
                    },
                    Field {
                        plain: (!doc).then(|| words.to_owned()),
                        doc: crate::design::text_draft::TextDraft::new(words),
                        changed: Vec::new(),
                    },
                );
            h.run();
            h.ctx.memory_mut(|memory| memory.request_focus(field.id()));
            h.run();
            h
        };
        let (mut plain, mut doc) = (make(false), make(true));
        let key = |key, modifiers| {
            vec![
                egui::Event::Key {
                    key,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers,
                },
                egui::Event::Key {
                    key,
                    physical_key: None,
                    pressed: false,
                    repeat: false,
                    modifiers,
                },
            ]
        };
        let ime = |event| vec![egui::Event::Ime(event)];
        let script: Vec<(&str, Vec<egui::Event>)> = vec![
            ("end", key(egui::Key::End, egui::Modifiers::COMMAND)),
            ("type", vec![egui::Event::Text("!".into())]),
            ("paste", vec![egui::Event::Paste(" ford".into())]),
            ("empty-paste", vec![egui::Event::Paste(String::new())]),
            (
                "delete-at-end",
                key(egui::Key::Delete, egui::Modifiers::NONE),
            ),
            ("select-all", key(egui::Key::A, egui::Modifiers::COMMAND)),
            ("replace", vec![egui::Event::Text("New".into())]),
            ("undo", key(egui::Key::Z, egui::Modifiers::COMMAND)),
            ("redo", key(egui::Key::Y, egui::Modifiers::COMMAND)),
            ("ime-on", ime(egui::ImeEvent::Enabled)),
            ("preedit", ime(egui::ImeEvent::Preedit("ka".into()))),
            ("commit", ime(egui::ImeEvent::Commit("か".into()))),
            ("preedit-again", ime(egui::ImeEvent::Preedit("ki".into()))),
            ("cancel", ime(egui::ImeEvent::Preedit(String::new()))),
            ("ime-off", ime(egui::ImeEvent::Disabled)),
        ];
        let text = |h: &egui_kittest::Harness<'static, Field>| match &h.state().plain {
            Some(plain) => plain.clone(),
            None => h.state().doc.as_str().to_owned(),
        };
        let caret = |h: &egui_kittest::Harness<'static, Field>| {
            egui::TextEdit::load_state(&h.ctx, field.id())
                .and_then(|state| state.cursor.char_range())
                .map(|range| (range.primary.index, range.secondary.index))
        };
        for (step, events) in script {
            let modifiers = events
                .iter()
                .find_map(|event| match event {
                    egui::Event::Key { modifiers, .. } => Some(*modifiers),
                    _ => None,
                })
                .unwrap_or_default();
            let before = (text(&doc), doc.state().doc.key());
            for h in [&mut plain, &mut doc] {
                h.state_mut().changed.clear();
                h.input_mut().modifiers = modifiers;
                crate::input_frame::one_pass(h, events.clone());
                h.input_mut().modifiers = egui::Modifiers::NONE;
                h.run_steps(1);
            }
            assert_eq!(text(&doc), text(&plain), "{step}: the same words");
            assert_eq!(
                doc.state().changed,
                plain.state().changed,
                "{step}: the same changed() in each frame"
            );
            assert_eq!(caret(&doc), caret(&plain), "{step}: the same caret");
            // Revision stability: the key changes exactly when the words change (a no-op, such
            // as an empty paste or Delete at the end, keeps it and lays out nothing).
            let key = doc.state().doc.key();
            assert_eq!(
                key != before.1,
                text(&doc) != before.0,
                "{step}: a new revision exactly when the words changed"
            );
        }
    }
}
