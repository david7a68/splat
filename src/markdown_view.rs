//! Read-only Markdown presentation. Content cannot execute UI actions, but it can say which note
//! a link names, and a roll table can ask for a roll. Whoever drew it decides what happens next.
use crate::design::{self, Type, colors};
use eframe::egui::util::cache::{ComputerMut, FrameCache};
use eframe::egui::{self, text::LayoutJob};
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use std::{borrow::Cow, sync::Arc};

/// A link to a note, as `world::with_links` writes one.
pub use splat_campaign::note::NOTE_LINK;
/// An embedded image, as `world::with_links` writes one.
pub const FILE_LINK: &str = "splat-file:";

fn clicked_id() -> egui::Id {
    egui::Id::new("markdown_clicked_note")
}

fn heading_id() -> egui::Id {
    egui::Id::new("markdown_wanted_heading")
}

/// The note link the DM pressed, for whoever just drew this Markdown.
pub fn take_note_link(ctx: &egui::Context) -> Option<String> {
    ctx.data_mut(|data| data.remove_temp::<String>(clicked_id()))
}

/// A resolved bold reference pressed in the Reader, for Lookup to open.
pub fn take_lookup_link(ctx: &egui::Context) -> Option<String> {
    ctx.data_mut(|data| data.remove_temp::<String>(egui::Id::new("markdown_clicked_lookup")))
}

/// Give a pressed bold reference to Lookup, as a press in the Reader does ([`take_lookup_link`]).
pub fn send_lookup_link(ctx: &egui::Context, words: String) {
    ctx.data_mut(|data| data.insert_temp(egui::Id::new("markdown_clicked_lookup"), words));
}

/// Ask the next draw to scroll to this heading. `None` ends the request.
pub fn want_heading(ctx: &egui::Context, heading: Option<String>) {
    ctx.data_mut(|data| match heading {
        Some(heading) => data.insert_temp(heading_id(), heading),
        None => data.remove::<String>(heading_id()),
    });
}

/// True once, for the heading a draw was asked to show.
pub fn wanted_heading(ctx: &egui::Context, heading: &str) -> bool {
    let wanted = ctx.data(|data| data.get_temp::<String>(heading_id()));
    if wanted.is_some_and(|wanted| wanted.eq_ignore_ascii_case(heading.trim())) {
        want_heading(ctx, None);
        return true;
    }
    false
}

/// The Reader's Markdown, prepared again in every frame: short words that no document owns
/// (layout PR 9b, Q4: unkeyed and counted). A document's text is [`show_document`]'s.
pub fn show_reader(ui: &mut egui::Ui, markdown: &str) {
    ui.scope(|ui| show_with_density(ui, markdown, false, ShortLinks::Prose, None));
}

/// Which document a reader shows (layout PR 9b-R1a): with its revision, what its prepared
/// blocks are kept for.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum DocumentId {
    /// A World entry's note body, in World's reader or in Lookup's.
    Entry(uuid::Uuid),
    /// A section of a Library entry: the entry's key and the section's place in it.
    Section(String, usize),
    /// A session's recording text: its folder's key, the part shown, and the slice around the
    /// DM's reading mark (before the mark, its line, after it).
    Recording(String, usize, usize),
}

/// A document's text as a reader shows it: whose text it is, and its revision (a reload of its
/// source gives a new one). The reader keeps the text's prepared blocks while the document, its
/// revision and what the preparation read stay the same (the links' index, the art, the fonts
/// and their atlas, the theme, the width and the scale), so a quiet frame parses, resolves and
/// lays out nothing.
pub struct ReaderSource<'a> {
    pub doc: DocumentId,
    pub revision: u64,
    pub text: &'a str,
}

/// The Reader's Markdown of a document, kept prepared for its [`ReaderSource`].
pub fn show_document(ui: &mut egui::Ui, source: ReaderSource<'_>) {
    ui.scope(|ui| show_kept(ui, source));
}

/// A plan row's words, such as a scene's line: the plan row title's type (`Body`) at every
/// width, with the Reader's links and emphasis. Each line is `XL` tall, as a fight's title
/// link is, so the row's second line keeps a fight row's pitch, and no space follows the words.
/// A done scene's words and links take the muted colour; a link keeps its underline.
pub fn show_row_words(ui: &mut egui::Ui, markdown: &str, muted: bool) {
    ui.scope(|ui| {
        show_with_density(
            ui,
            markdown,
            false,
            ShortLinks::Prose,
            Some(RowWords { muted }),
        );
    });
}

/// The Reader's Markdown, as compact field rows.
pub fn show_compact(ui: &mut egui::Ui, markdown: &str) {
    ui.scope(|ui| {
        ui.spacing_mut().item_spacing.y = 3.0;
        show_with_density(ui, markdown, true, ShortLinks::Prose, None);
    });
}

/// A stat block's Skills row, as a compact field row. Each link is a skill's bonus, one entry
/// with the skill's name before it (`Insight +8`), so the row lays its links out as entries.
pub fn show_skills(ui: &mut egui::Ui, markdown: &str) {
    ui.scope(|ui| {
        ui.spacing_mut().item_spacing.y = 3.0;
        show_with_density(ui, markdown, true, ShortLinks::Entries, None);
    });
}

/// Markdown drawn as a plan row's words (`show_row_words`), not as the Reader's prose.
#[derive(Clone, Copy)]
struct RowWords {
    muted: bool,
}

/// How a paragraph lays out a link whose words are narrower than `XL`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum ShortLinks {
    /// The link reserves its spare width in the line, so its target covers no other words.
    #[default]
    Prose,
    /// Each link ends an entry that starts after the last breaking blank before it (a skill's
    /// name and its bonus). The text keeps its own layout, and the link's target extends left
    /// over its own entry.
    Entries,
}

/// A note with its links resolved, which `linked_markdown` keeps from one frame to the next.
#[derive(Clone, Copy)]
struct Linking<'a> {
    index: &'a crate::lookup::ContentIndex,
    markdown: &'a str,
    from: Option<uuid::Uuid>,
}

impl std::hash::Hash for Linking<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.index.generation().hash(state);
        self.markdown.hash(state);
        self.from.hash(state);
    }
}

#[derive(Default)]
struct Linker;

impl ComputerMut<Linking<'_>, Arc<str>> for Linker {
    fn compute(&mut self, linking: Linking<'_>) -> Arc<str> {
        linking
            .index
            .reader_markdown(linking.markdown, linking.from)
            .into()
    }
}

/// Resolving each link searches the index. A reader repaints on each frame of a scroll, so
/// it resolves a note's links again only when the note, the index, or the reader changes.
fn linked_markdown(
    ctx: &egui::Context,
    index: &crate::lookup::ContentIndex,
    markdown: &str,
    from: Option<uuid::Uuid>,
) -> Arc<str> {
    ctx.memory_mut(|memory| {
        memory
            .caches
            .cache::<FrameCache<Arc<str>, Linker>>()
            .get(Linking {
                index,
                markdown,
                from,
            })
    })
}

/// Markdown prepared and drawn in this frame (a reader with no document).
fn show_with_density(
    ui: &mut egui::Ui,
    markdown: &str,
    dense: bool,
    short_links: ShortLinks,
    row: Option<RowWords>,
) {
    let from = reader_document(ui);
    let mut ops = prepare(ui, markdown, dense, row, from);
    let jump = Jump {
        at: Maps::of(&ops).wanted(ui.ctx()),
        kept: None,
    };
    draw(ui, &mut ops, from, short_links, row.is_some(), jump, None);
}

/// The link index a reader resolves its links in, when its host gave one.
fn link_index(ctx: &egui::Context) -> Option<Arc<crate::lookup::ContentIndex>> {
    ctx.data(|data| {
        data.get_temp::<Arc<crate::lookup::ContentIndex>>(egui::Id::new("reader_link_index"))
    })
}

/// Everything a document's prepared blocks were made from (layout PR 9b-R1a; note 9b §5, §10).
#[derive(Clone, Debug, PartialEq)]
struct PreparedKey {
    doc: DocumentId,
    revision: u64,
    /// The document the reader shows, for its links' resolution and their ids.
    from: Option<uuid::Uuid>,
    /// The load of the link index (`ContentIndex::generation`).
    index: Option<u64>,
    /// The art index (`ArtIndex::generation`): which links are figures.
    art: Option<u64>,
    /// The fonts' generation, which the theme's installation also raises.
    fonts: u64,
    /// The font atlas's epoch: a kept galley holds places in it.
    atlas: u64,
    dark: bool,
    /// The scale and the width, in thousandths of a point.
    scale: i64,
    width: i64,
}

impl PreparedKey {
    fn of(ui: &egui::Ui, source: &ReaderSource<'_>, from: Option<uuid::Uuid>) -> Self {
        let ctx = ui.ctx();
        let thousandths = |value: f32| (f64::from(value) * 1000.0).round() as i64;
        Self {
            doc: source.doc.clone(),
            revision: source.revision,
            from,
            index: link_index(ctx).map(|index| index.generation()),
            art: crate::art::generation(ctx),
            fonts: design::theme::font_generation(ctx),
            atlas: design::theme::atlas_epoch(ctx),
            dark: ui.visuals().dark_mode,
            scale: thousandths(ctx.pixels_per_point()),
            width: thousandths(ui.available_width()),
        }
    }
}

/// A document's prepared blocks, for their key, with its maps (layout PR 9b-R1b): its headings
/// and where each block stood in the last draw.
struct Kept {
    key: PreparedKey,
    ops: Vec<Op>,
    maps: Maps,
}

/// A prepared document's maps, complete whatever is in view (note 9b §5): each heading's words
/// and block, and each block's top below the reader's top in the last draw (empty before the
/// first).
#[derive(Default)]
struct Maps {
    headings: Vec<(String, usize)>,
    offsets: Vec<f32>,
    /// The whole document's height in the last draw (layout PR 9b-R2), with `offsets` the room
    /// of every block, so one out of view is not laid out.
    height: Option<f32>,
    /// What else sets a block's height in the last draw: a wide table's room (the pane's right
    /// edge and the clip's), which the key does not hold. When it changes, every block draws.
    room: Option<(Option<f32>, f32)>,
}

impl Maps {
    /// The maps of prepared blocks: their headings.
    fn of(ops: &[Op]) -> Self {
        let mut maps = Self::default();
        for (block, op) in ops.iter().enumerate() {
            if let Op::Jump(words) = op {
                maps.headings.push((words.trim().to_owned(), block));
            }
        }
        maps
    }

    /// The block of the heading a typed jump asks for, if this document has it.
    fn wanted(&self, ctx: &egui::Context) -> Option<usize> {
        let wanted = ctx.data(|data| data.get_temp::<String>(heading_id()))?;
        self.headings
            .iter()
            .find(|(words, _)| wanted.eq_ignore_ascii_case(words))
            .map(|(_, block)| *block)
    }
}

/// Each reader's kept blocks, with the frame it last drew them in. A reader not drawn in the
/// last frame loses them, so a document no reader shows keeps nothing.
#[derive(Clone, Default)]
struct KeptReaders(Arc<std::sync::Mutex<KeptMap>>);

type KeptMap = std::collections::HashMap<egui::Id, (u64, Arc<std::sync::Mutex<Option<Kept>>>)>;

fn show_kept(ui: &mut egui::Ui, source: ReaderSource<'_>) {
    let from = reader_document(ui);
    let key = PreparedKey::of(ui, &source, from);
    let frame = ui.ctx().cumulative_frame_nr();
    let readers = ui.ctx().data_mut(|data| {
        data.get_temp_mut_or_default::<KeptReaders>(egui::Id::new("markdown_kept_readers"))
            .clone()
    });
    let kept = {
        let mut readers = readers
            .0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        readers.retain(|_, (drawn, _)| *drawn + 1 >= frame);
        let entry = readers.entry(ui.id()).or_default();
        entry.0 = frame;
        entry.1.clone()
    };
    let mut kept = kept
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if kept.as_ref().is_none_or(|kept| kept.key != key) {
        let ops = prepare(ui, source.text, false, None, from);
        let maps = Maps::of(&ops);
        *kept = Some(Kept { key, ops, maps });
    }
    if let Some(kept) = kept.as_mut() {
        // A typed jump resolves against the heading map and the heading's offset from the last
        // draw, before any block is drawn (note 9b §5: before any clipping); the first draw
        // after a preparation has no offsets yet, so the heading's block jumps as it draws.
        let mut jump = kept.maps.wanted(ui.ctx());
        let mut kept_jump = None;
        if let Some(block) = jump
            && let Some(offset) = kept.maps.offsets.get(block)
        {
            let top = ui.cursor().top() + offset;
            design::reveal::jump(ui, top, design::reveal::JumpTo::LinkedHeading);
            want_heading(ui.ctx(), None);
            jump = None;
            kept_jump = Some((block, top));
        }
        // The view, from the reader's top (layout PR 9b-R2): a block wholly outside it is not
        // laid out; its room is kept. A new room for wide tables lays every block out again.
        let top = ui.cursor().top();
        let clip = ui.clip_rect();
        let room = (
            ui.ctx()
                .data(|data| data.get_temp::<f32>(egui::Id::new(TABLE_ROOM))),
            clip.right(),
        );
        if kept.maps.room.replace(room) != Some(room) {
            kept.maps.height = None;
        }
        let view = (clip.top() - top, clip.bottom() - top);
        draw(
            ui,
            &mut kept.ops,
            from,
            ShortLinks::Prose,
            false,
            Jump {
                at: jump,
                kept: kept_jump,
            },
            Some((&mut kept.maps.offsets, &mut kept.maps.height, view)),
        );
    }
}

/// One block of prepared Markdown, as [`draw`] draws it.
enum Op {
    Gap(design::Gap),
    Rule,
    /// A heading's words: a typed jump to this heading lands here (ruling M18).
    Jump(String),
    /// Images before the words of their block, which are their caption.
    Figures(Vec<std::path::PathBuf>, String),
    Paragraph(Box<Paragraph>),
    QuoteStart,
    QuoteEnd,
    Table(Box<TableBlock>),
    /// A compact reader's `Field`/`Value` table, as field rows.
    Fields(Vec<Vec<String>>),
}

/// A paragraph's words, styled; laid out the first time they are drawn, at the width they get.
struct Paragraph {
    job: Option<LayoutJob>,
    laid: Option<Laid>,
    links: Vec<(std::ops::Range<usize>, LinkTarget)>,
    /// A task's box: whether it is done, how far it starts from its item's mark (after a
    /// number) or, in a plan row, from the words, and its type.
    task: Option<(bool, f32, Type)>,
    /// A list item's mark (a bullet, a number, a task's box) or a plan row's leading icon, in
    /// its own column: its words, then their galley.
    lead: Option<Lead>,
    /// How many lists it is in, in the Reader: its words keep their item's edge. 0 outside a
    /// list and in a plan row's words.
    depth: usize,
    /// The space after it, where the Reader leaves one.
    after: design::Gap,
    /// Where its linked galley was drawn last, from its block's top left (layout PR 9b-R2): a
    /// paragraph out of view registers its links there, below its block's kept top, without
    /// being laid out. (From the block, not the reader: a block above that drew at a new height
    /// moves the block's kept top, and its links with it.)
    origin: Option<egui::Vec2>,
}

/// One level of a list in the Reader (`design::list_item`): room for a bullet, a task's box or a
/// number to 99.
const LIST_STEP: design::Gap = design::Gap::XL;

/// A leading icon's words until they are laid out, then their galley.
enum Lead {
    Words(LayoutJob),
    Laid(Arc<egui::Galley>),
}

/// A paragraph laid out.
enum Laid {
    /// An egui label over its galley: the DM can select its words.
    Plain(Arc<egui::Galley>),
    Linked(Box<LaidLinks>),
}

/// A paragraph with links, laid out: its words (its accessible name), its galley, and each link
/// with its underline, its words' parts, its targets and their bounds at the galley's origin,
/// and the key of its destination, all made once (layout PR 9b-R1b: a quiet frame copies no
/// part and builds no target for a link out of view).
struct LaidLinks {
    text: String,
    galley: Arc<egui::Galley>,
    links: Vec<(std::ops::Range<usize>, LinkTarget)>,
    underlines: Vec<egui::Stroke>,
    words: Vec<Vec<LinkPart>>,
    targets: Vec<Vec<egui::Rect>>,
    bounds: Vec<egui::Rect>,
    /// Each link's destination, as part of its id: a focused link whose destination changed
    /// has another id, so it loses the focus and never acts (note 9b §10.6).
    keys: Vec<u64>,
}

/// A table's cells, its shown columns, and its roll marker.
struct TableBlock {
    rows: Vec<Vec<String>>,
    columns: Vec<usize>,
    /// An inventory table, shown by its item, quantity, equipment and attunement.
    compact: bool,
    rolls: Option<Result<crate::roll_table::Marker, String>>,
}

/// The open quotes while blocks draw: each one's top, and the bottom of the last words drawn in
/// one, where its bar stops.
#[derive(Default)]
struct Quotes {
    tops: Vec<f32>,
    bottom: f32,
}

/// Draw prepared blocks. A paragraph's link ids come from the reader and the block's place,
/// never from its words: stable while the document's preparation stands.
/// A typed jump as the blocks draw (layout PR 9b-R1b): the heading's block where it lands as it
/// draws, and the block and top a jump from its kept offset went to before the blocks drew.
#[derive(Clone, Copy, Default)]
struct Jump {
    at: Option<usize>,
    kept: Option<(usize, f32)>,
}

/// `jump` is where a typed jump lands as the blocks draw, or is corrected; `placed`, a kept
/// document's placement: each block's top below the reader's top, written as the blocks draw,
/// its height, and the view (layout PR 9b-R2).
fn draw(
    ui: &mut egui::Ui,
    ops: &mut [Op],
    from: Option<uuid::Uuid>,
    short_links: ShortLinks,
    row: bool,
    jump: Jump,
    mut placed: Option<Placed<'_>>,
) {
    let reader = egui::Id::new(("markdown_reader", from));
    let mut quotes = Quotes::default();
    let top = ui.cursor().top();
    let origin = egui::pos2(ui.max_rect().left(), top);
    // Layout PR 9b-R2: with every block's room kept from the last draw, a block wholly outside
    // the view is not laid out. Not while the DM holds a selection: egui's label selection
    // reaches only labels that draw, so a selection whose end is out of view stays whole.
    let selecting = ui
        .ctx()
        .plugin::<egui::text_selection::LabelSelectionState>()
        .lock()
        .has_selection();
    let skip = placed.as_ref().and_then(|(offsets, height, view)| {
        (offsets.len() == ops.len() && !selecting)
            .then_some((**height)?)
            .map(|h| (h, *view))
    });
    if let Some((offsets, _, _)) = placed.as_mut() {
        offsets.resize(ops.len(), 0.0);
    }
    // How far this draw's blocks moved from their kept tops (a block drawn again at another
    // height moves the ones after it), and whether the block before this one drew.
    let mut moved = 0.0;
    let mut drew = false;
    for block in 0..ops.len() {
        if let Some((offsets, _, _)) = placed.as_mut() {
            let kept = (skip.is_some() && block < offsets.len()).then(|| offsets[block]);
            if let Some((height, (from, to))) = skip {
                // After a block that drew, this one starts where the reader's cursor is: a block
                // that drew at a new height (a wide table that gains its scroll bar's lane as it
                // comes into view) moves this one, skipped or not (review of PR 882, P2).
                if drew && let Some(kept) = kept {
                    moved = ui.cursor().top() - top - kept;
                }
                let kept_top = kept.unwrap_or_default() + moved;
                let kept_bottom = offsets.get(block + 1).copied().unwrap_or(height) + moved;
                let out = kept_bottom < from || kept_top > to;
                if out && skippable(&ops[block]) {
                    drew = false;
                    offsets[block] = kept_top;
                    skipped(
                        ui,
                        &mut ops[block],
                        (reader, block),
                        origin.x,
                        (top + kept_top, top + kept_bottom),
                        &mut quotes,
                    );
                    continue;
                }
                // Drawn: from its kept top, which the blocks skipped before it left room for.
                leave_room(ui, top + kept_top - ui.cursor().top());
            }
            offsets[block] = ui.cursor().top() - top;
            if let Some(kept) = kept {
                moved = offsets[block] - kept;
            }
            drew = true;
        }
        match &mut ops[block] {
            Op::Gap(gap) => design::gap(ui, *gap),
            Op::Jump(_) => {
                if jump.at == Some(block) {
                    design::reveal::jump(
                        ui,
                        ui.cursor().top(),
                        design::reveal::JumpTo::LinkedHeading,
                    );
                    want_heading(ui.ctx(), None);
                }
                // A jump from the kept offset, where the heading no longer stands (a wide table
                // above it reflowed for a new pane width while the prose kept its measure): the
                // jump again, to where the heading draws now; the newer request replaces it
                // (PR 861 review, P2).
                if let Some((kept, top)) = jump.kept
                    && kept == block
                    && (ui.cursor().top() - top).abs() > 0.5
                {
                    design::reveal::jump(
                        ui,
                        ui.cursor().top(),
                        design::reveal::JumpTo::LinkedHeading,
                    );
                }
            }
            Op::QuoteStart => quotes.tops.push(ui.cursor().top()),
            Op::QuoteEnd => quote_bar(ui, &mut quotes),
            op => {
                // A block with widgets. In a kept document its widgets' ids are its own
                // (layout PR 9b-R2): a block skipped before it moves none of them, so a label the
                // DM starts a selection in keeps its id when every block draws for the selection.
                let own = placed
                    .is_some()
                    .then(|| ui.id().with(("markdown_block", block)));
                in_block(ui, own, |ui| match op {
                    Op::Rule => design::rule(ui),
                    Op::Figures(figures, caption) => {
                        for figure in figures.iter() {
                            crate::art::figure(ui, figure, caption);
                        }
                    }
                    Op::Paragraph(paragraph) => {
                        // The block's top left: its links' kept places are from it.
                        let at = egui::pos2(origin.x, ui.cursor().top());
                        draw_paragraph(
                            ui,
                            paragraph,
                            (reader, block),
                            at,
                            short_links,
                            row,
                            &mut quotes,
                        );
                    }
                    Op::Table(table) => draw_table_block(ui, table),
                    Op::Fields(rows) => show_fields(ui, rows),
                    Op::Gap(_) | Op::Jump(_) | Op::QuoteStart | Op::QuoteEnd => {}
                });
            }
        }
    }
    if let Some((_, height, _)) = placed {
        // The room of the blocks skipped after the last one drawn.
        if let Some((kept, _)) = skip {
            leave_room(ui, top + kept + moved - ui.cursor().top());
        }
        *height = Some(ui.cursor().top() - top);
    }
}

/// Draw a block's widgets: under their own ids (`own`, global) or the reader's. Under its own
/// ids the block draws in a child, and the reader goes on where the child's cursor stopped, as
/// if the block drew in the reader: a block that ends in space adds no item space after it.
fn in_block(ui: &mut egui::Ui, own: Option<egui::Id>, draw: impl FnOnce(&mut egui::Ui)) {
    match own {
        Some(id) => {
            let mut child = ui.new_child(egui::UiBuilder::new().id_salt(id).global_scope(true));
            draw(&mut child);
            let used = child.min_rect();
            let next = child.cursor().top();
            drop(child);
            let spacing = ui.spacing().item_spacing.y;
            let _ = ui.advance_cursor_after_rect(egui::Rect::from_min_max(
                used.min,
                egui::pos2(used.max.x, (next - spacing).max(used.min.y)),
            ));
        }
        None => draw(ui),
    }
}

/// A kept document's placement as it draws: each block's top and the whole height from the last
/// draw, and the view (its top and bottom from the reader's top).
type Placed<'a> = (&'a mut Vec<f32>, &'a mut Option<f32>, (f32, f32));

/// Room of `gap` where blocks were skipped: an empty rect and the spacing after it, as a block
/// laid out there took. Its auto ids are the reader's own: [`show_document`] draws it in a scope,
/// so the widgets after it keep theirs whatever it skips.
fn leave_room(ui: &mut egui::Ui, gap: f32) {
    if gap > 0.0 {
        let spacing = ui.spacing().item_spacing.y;
        let size = egui::vec2(0.0, (gap - spacing).max(0.0));
        let _ = ui.allocate_exact_size(size, egui::Sense::hover());
    }
}

/// Whether a block out of view can go without being laid out: every block but a table with a
/// roll marker, whose control has no kept place yet (a named limit of layout PR 9b-R2).
fn skippable(op: &Op) -> bool {
    !matches!(op, Op::Table(table) if table.rolls.is_some())
}

/// A block out of view (layout PR 9b-R2), at its kept place `(top, bottom)`: nothing is laid
/// out. A quote's start and end still say where its bar goes, and a paragraph's links are still
/// controls, each at its kept first target (from the block's top and the reader's `left`
/// edge), so Tab and a screen reader reach them wherever they are.
fn skipped(
    ui: &mut egui::Ui,
    op: &mut Op,
    links: (egui::Id, usize),
    left: f32,
    (top, bottom): (f32, f32),
    quotes: &mut Quotes,
) {
    match op {
        Op::QuoteStart => quotes.tops.push(top),
        Op::QuoteEnd => quote_bar(ui, quotes),
        Op::Paragraph(paragraph) => {
            if !quotes.tops.is_empty() {
                quotes.bottom = bottom;
            }
            if let (Some(Laid::Linked(laid)), Some(at)) = (&paragraph.laid, paragraph.origin) {
                register_links(
                    ui,
                    laid,
                    block_id(ui.ctx(), links),
                    egui::pos2(left, top) + at,
                );
            }
        }
        _ => {}
    }
}

/// The bar of the innermost open quote, from its top to the last words drawn in it.
fn quote_bar(ui: &egui::Ui, quotes: &mut Quotes) {
    if let Some(top) = quotes.tops.pop() {
        let x =
            ui.max_rect().left() + QUOTE_INDENT * quotes.tops.len() as f32 + QUOTE_BAR_WIDTH / 2.0;
        ui.painter().vline(
            x,
            top..=quotes.bottom.max(top),
            egui::Stroke::new(QUOTE_BAR_WIDTH, colors().rule),
        );
    }
}

/// The words waiting to be drawn become a paragraph block.
fn paragraph(ops: &mut Vec<Op>, job: &mut LayoutJob, target: &mut Targets) {
    if job.is_empty() {
        return;
    }
    let mut links = std::mem::take(&mut target.links);
    let task = target.task.take();
    let mut job = std::mem::take(job);
    let lead = target.lead.take().map(|cut| {
        // The icon in its own column; the words wrap at their own left edge.
        let (mut icon, words) = split_job(std::mem::take(&mut job), cut);
        for (range, _) in &mut links {
            *range = range.start - cut..range.end - cut;
        }
        job = words;
        if !links.is_empty() {
            // A link lifts its paragraph's rows to `XL` (`lift_link_rows`): the mark's line
            // with them, so the mark keeps the baseline of the words' first line.
            for section in &mut icon.sections {
                section.format.line_height =
                    section.format.line_height.map(|line| line.max(design::XL));
            }
        }
        Lead::Words(icon)
    });
    ops.push(Op::Paragraph(Box::new(Paragraph {
        job: Some(job),
        laid: None,
        links,
        task,
        lead,
        depth: target.depth,
        after: design::Gap::S,
        origin: None,
    })));
}

/// The space after the block before, when that block is a list's words: `gap` in place of the
/// Reader's `S`. An item of an inner list follows `XS` (it stays with its own list's first
/// item), and a whole list is followed by `M`.
fn gap_after_listed(ops: &mut [Op], gap: design::Gap) {
    if let Some(Op::Paragraph(paragraph)) = ops.last_mut()
        && paragraph.depth > 0
    {
        paragraph.after = gap;
    }
}

/// The images found in a block, before its words.
fn figures_op(ops: &mut Vec<Op>, figures: &mut Vec<std::path::PathBuf>, caption: &LayoutJob) {
    if !figures.is_empty() {
        ops.push(Op::Figures(std::mem::take(figures), caption.text.clone()));
    }
}

/// `job` laid out as an egui label lays it out in `ui` (`Label::layout_in_ui` in a layout that
/// does not wrap horizontally): wrapped at the room it gets, or on one line.
fn lay_out(ui: &egui::Ui, mut job: LayoutJob, wrap: bool) -> Arc<egui::Galley> {
    job.wrap.max_width = if wrap {
        ui.available_width()
    } else {
        f32::INFINITY
    };
    job.halign = ui.layout().horizontal_placement();
    job.justify = ui.layout().horizontal_justify();
    ui.fonts_mut(|fonts| fonts.layout_job(job))
}

/// A paragraph's words, laid out once, in the `Ui` that first draws them.
fn laid<'a>(ui: &egui::Ui, paragraph: &'a mut Paragraph, short_links: ShortLinks) -> &'a Laid {
    paragraph.laid.get_or_insert_with(|| {
        let job = paragraph.job.take().unwrap_or_default();
        let links = std::mem::take(&mut paragraph.links);
        if links.is_empty() {
            Laid::Plain(lay_out(ui, job, true))
        } else {
            Laid::Linked(Box::new(lay_out_links(ui, job, links, short_links)))
        }
    })
}

/// The id of a paragraph's links: its reader and its block's place, never its words. Two
/// readers of one document (or of none) in one pass count up in drawing order, so their links
/// never share an id, and a reader's own ids stay while its preparation stands.
fn block_id(ctx: &egui::Context, (reader, block): (egui::Id, usize)) -> egui::Id {
    #[derive(Clone, Default)]
    struct Seen {
        pass: u64,
        count: std::collections::HashMap<egui::Id, u32>,
    }
    let base = reader.with(("markdown_block", block));
    let pass = ctx.cumulative_pass_nr();
    let nth = ctx.data_mut(|data| {
        let seen = data.get_temp_mut_or_default::<Seen>(egui::Id::new("markdown_block_seen"));
        if seen.pass != pass {
            seen.pass = pass;
            seen.count.clear();
        }
        let nth = seen.count.entry(base).or_default();
        *nth += 1;
        *nth
    });
    base.with(nth)
}

/// One paragraph: its words, its links, a list item's mark or a task's box, inside the quotes
/// and the lists it is in.
fn draw_paragraph(
    ui: &mut egui::Ui,
    paragraph: &mut Paragraph,
    links: (egui::Id, usize),
    origin: egui::Pos2,
    short_links: ShortLinks,
    row: bool,
    quotes: &mut Quotes,
) {
    let (task, depth, after) = (paragraph.task, paragraph.depth, paragraph.after);
    // The mark is laid out once, on one line.
    let mark = paragraph.lead.take().map(|lead| match lead {
        Lead::Words(mut job) => {
            job.wrap.max_width = f32::INFINITY;
            ui.fonts_mut(|fonts| fonts.layout_job(job))
        }
        Lead::Laid(galley) => galley,
    });
    paragraph.lead = mark.clone().map(Lead::Laid);
    let mut at = None;
    let mut words = |ui: &mut egui::Ui| match laid(ui, paragraph, short_links) {
        Laid::Plain(galley) => ui.add(egui::Label::new(galley.clone())).rect,
        Laid::Linked(laid) => {
            let (rect, position) = draw_laid_links(ui, laid, block_id(ui.ctx(), links));
            at = Some(position - origin);
            rect
        }
    };
    let draw = |ui: &mut egui::Ui| {
        // The mark in its own column; the words wrap at their own left edge.
        let rect = if mark.is_some() || depth > 0 {
            let (mark, rect) = design::list_item(ui, depth, LIST_STEP, mark, words);
            mark.map_or(rect, |mark| mark.rect)
        } else {
            words(ui)
        };
        if let Some((done, indent, role)) = task {
            let icon_rect = egui::Rect::from_min_size(
                rect.min + egui::vec2(indent, 0.0),
                egui::vec2(role.size(), role.line()),
            );
            ui.interact(
                icon_rect,
                ui.next_auto_id().with("task-marker"),
                egui::Sense::hover(),
            )
            .widget_info(|| {
                egui::WidgetInfo::labeled(
                    egui::WidgetType::Label,
                    true,
                    if done { "Done" } else { "Open" },
                )
            });
        }
    };
    if quotes.tops.is_empty() {
        draw(ui);
    } else {
        let rect = design::quote(ui, quotes.tops.len(), draw).response.rect;
        quotes.bottom = rect.bottom();
    }
    paragraph.origin = at;
    if !row {
        design::gap(ui, after);
    }
}

/// A table, drawn from its cells.
fn draw_table_block(ui: &mut egui::Ui, table: &TableBlock) {
    let TableBlock {
        rows,
        columns,
        compact,
        rolls,
    } = table;
    let compact = *compact;
    let display = |cell: &'_ str| -> String {
        if compact {
            match cell {
                "Link" => "Item",
                "true" => "Yes",
                "false" => "—",
                other => other,
            }
            .to_owned()
        } else {
            cell.to_owned()
        }
    };
    // A reader may give its tables the pane beyond the prose measure.
    let room = table_room(ui).filter(|width| *width > ui.available_width());
    // The reader's own id: the table's roll state and its field belong to this reader, so the
    // same table in World and in Lookup never shares them.
    let reader = ui.id();
    let draw_table = |ui: &mut egui::Ui| {
        let mut widths = column_widths(ui, rows, columns, &display);
        // A rollable table keeps a slot for the roll mark before every row's range, so the
        // rolled row's range stays at the x of the others. The first column is sized with the
        // slot.
        let mark = (rolls.is_some() && !widths.is_empty()).then(|| {
            ui.painter()
                .layout_no_wrap(
                    format!("{} ", design::icons::DICE),
                    Type::Stat.font(ui),
                    colors().text,
                )
                .size()
                .x
                .ceil()
        });
        if let Some(slot) = mark {
            for (at, row) in rows.iter().enumerate() {
                let label = display(row.get(columns[0]).map(String::as_str).unwrap_or(""));
                let role = if at == 0 { Type::StatLabel } else { Type::Stat };
                let width = ui
                    .painter()
                    .layout_no_wrap(label, role.font(ui), colors().text)
                    .size()
                    .x
                    .ceil()
                    + slot
                    + design::XS;
                widths[0] = widths[0].max(width);
            }
        }
        let matched = rolls
            .as_ref()
            .and_then(|marker| crate::roll_table::matched_row(ui, reader, marker, rows));
        reader_table(ui, rows, columns, &widths, &display, matched, mark);
    };
    match room {
        Some(width) => {
            // The table draws past the measure, but the reader's column keeps it: the rules and
            // prose after the table keep their width.
            let mut area = ui.available_rect_before_wrap();
            area.set_width(width);
            let mut wide = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(area)
                    .layout(egui::Layout::top_down(egui::Align::Min)),
            );
            draw_table(&mut wide);
            let height = wide.min_rect().height();
            ui.allocate_exact_size(
                egui::vec2(ui.available_width(), height),
                egui::Sense::hover(),
            );
        }
        None => draw_table(ui),
    }
    if let Some(marker) = rolls {
        crate::roll_table::roll_control(ui, reader, marker, rows);
    }
    design::gap(ui, design::Gap::S);
}

/// The blocks of `markdown` as the Reader draws them: the Markdown parsed, its links resolved,
/// its figures found, each paragraph's words styled (layout PR 9b-R1a). Nothing is drawn here:
/// [`draw`] draws the blocks, and lays out each paragraph the first time it draws it.
fn prepare(
    ui: &egui::Ui,
    markdown: &str,
    dense: bool,
    row: Option<RowWords>,
    from: Option<uuid::Uuid>,
) -> Vec<Op> {
    let mut ops = Vec::new();
    let mut quote_depth = 0_usize;
    let prose = if dense {
        Type::Stat
    } else if row.is_some() {
        Type::Body
    } else {
        design::reader::prose_type(ui)
    };
    let line = if row.is_some() {
        design::XL
    } else {
        prose.line()
    };
    // A done scene's words and links are muted; a link keeps its underline.
    let (text_color, link_color) = if row.is_some_and(|row| row.muted) {
        (colors().muted, colors().muted)
    } else {
        (colors().text, colors().accent)
    };
    let mut job = LayoutJob::default();
    let mut heading = None;
    let mut inventory_table = false;
    let mut target = Targets {
        row: row.is_some(),
        ..Targets::default()
    };
    let mut link = None;
    let mut strong_link = None;
    let mut rolls = None;
    let mut in_link = false;
    let mut code = false;
    let mut strong = 0_usize;
    let mut italic = false;
    let mut struck = false;
    let mut lists: Vec<Option<u64>> = Vec::new();
    // The Reader sets an item in by its level, its mark in its own column. A plan row's words
    // are one line of its row: a mark stays in that line, after two spaces for each level.
    let in_row = row.is_some();
    let depth = |lists: usize| if in_row { 0 } else { lists };
    let row_indent = |lists: usize| {
        if in_row {
            "  ".repeat(lists.saturating_sub(1))
        } else {
            String::new()
        }
    };
    let mut table: Option<Vec<Vec<String>>> = None;
    let mut figures: Vec<std::path::PathBuf> = Vec::new();
    let mut in_figure = false;
    let ctx = ui.ctx().clone();
    let index = link_index(&ctx);
    let linked: Arc<str> = index.as_ref().map_or_else(
        || markdown.into(),
        |index| linked_markdown(&ctx, index, markdown, from),
    );
    let source = local_figure_links(&linked, |url| crate::art::link_art(&ctx, url).is_some());
    for (event, range) in Parser::new_ext(
        &source,
        Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TASKLISTS,
    )
    .into_offset_iter()
    {
        // A roll marker belongs to the table right after it, not to a later one.
        if matches!(
            event,
            Event::Start(
                Tag::Paragraph
                    | Tag::Heading { .. }
                    | Tag::List(_)
                    | Tag::CodeBlock(_)
                    | Tag::BlockQuote(_)
            )
        ) {
            rolls = None;
        }
        if matches!(event, Event::Code(_) | Event::Start(Tag::Image { .. })) {
            strong_link = None;
        }
        match event {
            Event::Html(ref text) if text.trim().starts_with("<!-- splat:inventory") => {
                inventory_table = true;
            }
            Event::Html(ref text) if crate::roll_table::rolls_marker(text).is_some() => {
                rolls = crate::roll_table::rolls_marker(text);
            }
            Event::Text(ref text) if in_figure && &**text == "image" => {}
            Event::Start(Tag::Link { dest_url, .. }) if table.is_none() => {
                // An explicit destination wins over the bold words around it.
                strong_link = None;
                in_link = true;
                // A link to an image with a local copy shows the image instead of opening a page.
                let local = crate::art::link_art(&ctx, &dest_url)
                    .or_else(|| dest_url.strip_prefix(FILE_LINK).map(Into::into));
                if let Some(path) = local {
                    figures.push(path);
                    in_figure = true;
                    continue;
                }
                if dest_url.starts_with("splat-missing:") {
                    let icon = format!("{} ", design::icons::WARN);
                    job.append(
                        &icon,
                        0.0,
                        egui::TextFormat {
                            font_id: prose.font(ui),
                            color: colors().result.warning,
                            ..Default::default()
                        },
                    );
                    if target.row && job.text.len() == icon.len() {
                        target.lead = Some(icon.len());
                    }
                    link = Some((job.text.len(), LinkTarget::Missing));
                    continue;
                }
                if let Some(destination) = LinkTarget::from_destination(&dest_url) {
                    link = Some((job.text.len(), destination));
                }
            }
            Event::End(TagEnd::Link) => {
                in_link = false;
                in_figure = false;
                if let Some((start, destination)) = link.take() {
                    target.links.push((start..job.text.len(), destination));
                }
            }
            Event::Start(Tag::Table(_)) => {
                paragraph(&mut ops, &mut job, &mut target);
                table = Some(Vec::new());
            }
            Event::Start(Tag::TableHead | Tag::TableRow) => {
                if let Some(rows) = &mut table {
                    rows.push(Vec::new());
                }
            }
            Event::Start(Tag::TableCell) => {
                if let Some(row) = table.as_mut().and_then(|rows| rows.last_mut()) {
                    row.push(String::new());
                }
            }
            Event::End(TagEnd::Table) => {
                if let Some(mut rows) = table.take() {
                    if dense && rows.first().is_some_and(|row| row == &["Field", "Value"]) {
                        rows.remove(0);
                        ops.push(Op::Fields(rows));
                        inventory_table = false;
                        continue;
                    }
                    let compact = inventory_table
                        && rows
                            .first()
                            .is_some_and(|row| row.iter().any(|cell| cell == "Link"))
                        && rows.iter().skip(1).all(|row| {
                            rows[0]
                                .iter()
                                .position(|cell| cell == "Link")
                                .and_then(|i| row.get(i))
                                .is_some_and(|name| !name.is_empty())
                        });
                    let columns: Vec<_> = if compact {
                        ["Link", "Quantity", "Equipment", "Attuned"]
                            .iter()
                            .filter_map(|name| rows.first()?.iter().position(|cell| cell == name))
                            .collect()
                    } else {
                        (0..rows.first().map_or(0, Vec::len)).collect()
                    };
                    if let Some(Ok(crate::roll_table::Marker { dice, .. })) = &rolls
                        && let Some(header) = rows.first_mut().and_then(|row| row.first_mut())
                        && header.trim().is_empty()
                    {
                        *header = dice
                            .strip_prefix('1')
                            .filter(|value| value.starts_with('d'))
                            .unwrap_or(dice)
                            .to_owned();
                    }
                    ops.push(Op::Table(Box::new(TableBlock {
                        rows,
                        columns,
                        compact,
                        rolls: rolls.take(),
                    })));
                }
                inventory_table = false;
            }
            Event::Text(text) | Event::Code(text) if table.is_some() => {
                if let Some(cell) = table
                    .as_mut()
                    .and_then(|rows| rows.last_mut())
                    .and_then(|row| row.last_mut())
                {
                    cell.push_str(&text);
                }
            }
            Event::Start(Tag::Heading { level, .. }) => {
                paragraph(&mut ops, &mut job, &mut target);
                ops.push(Op::Gap(if dense {
                    design::Gap::S
                } else {
                    match level {
                        pulldown_cmark::HeadingLevel::H1 => design::Gap::S,
                        pulldown_cmark::HeadingLevel::H2 => design::Gap::L,
                        _ => design::Gap::M,
                    }
                }));
                heading = Some(level);
            }
            Event::End(TagEnd::Heading(level)) => {
                // A linked section opens with its heading at the column's top: a typed jump
                // that the column writes as its own offset (ruling M18).
                ops.push(Op::Jump(job.text.clone()));
                figures_op(&mut ops, &mut figures, &job);
                paragraph(&mut ops, &mut job, &mut target);
                if level == pulldown_cmark::HeadingLevel::H2
                    || (!dense && level == pulldown_cmark::HeadingLevel::H3)
                {
                    ops.push(Op::Rule);
                    ops.push(Op::Gap(design::Gap::XS));
                }
                heading = None;
            }
            Event::Start(Tag::CodeBlock(_)) => {
                paragraph(&mut ops, &mut job, &mut target);
                code = true;
            }
            Event::End(TagEnd::CodeBlock) => {
                paragraph(&mut ops, &mut job, &mut target);
                code = false;
            }
            Event::Start(Tag::Strong) => {
                strong += 1;
                if strong == 1 {
                    strong_link = (!in_link && heading.is_none() && table.is_none())
                        .then_some(job.text.len());
                }
            }
            Event::End(TagEnd::Strong) => {
                strong = strong.saturating_sub(1);
                if strong == 0
                    && let Some(start) = strong_link.take()
                {
                    let words = job.text[start..].trim();
                    let table_caption = job.text[..start].trim().is_empty()
                        && source[range.end..].trim_start().starts_with('|');
                    if !(words.is_empty() || words.ends_with(['.', ':']) || table_caption)
                        && index.as_ref().is_some_and(|index| {
                            matches!(
                                index.resolve_link(words, from),
                                crate::world::Go::Item(..) | crate::world::Go::Many(_)
                            )
                        })
                    {
                        for section in &mut job.sections {
                            if section.byte_range.start >= start {
                                section.format.color = link_color;
                                section.format.underline =
                                    egui::Stroke::new(design::HAIRLINE_WIDTH, link_color);
                                if section.format.strikethrough != egui::Stroke::NONE {
                                    section.format.strikethrough.color = link_color;
                                }
                            }
                        }
                        target
                            .links
                            .push((start..job.text.len(), LinkTarget::Lookup(words.to_owned())));
                    }
                }
            }
            Event::Start(Tag::Emphasis) => italic = true,
            Event::End(TagEnd::Emphasis) => italic = false,
            Event::Start(Tag::Strikethrough) => struck = true,
            Event::End(TagEnd::Strikethrough) => struck = false,
            Event::Start(Tag::BlockQuote(_)) => {
                paragraph(&mut ops, &mut job, &mut target);
                ops.push(Op::QuoteStart);
                quote_depth += 1;
            }
            Event::End(TagEnd::BlockQuote(_)) => {
                figures_op(&mut ops, &mut figures, &job);
                paragraph(&mut ops, &mut job, &mut target);
                ops.push(Op::QuoteEnd);
                quote_depth = quote_depth.saturating_sub(1);
            }
            Event::Start(Tag::List(start)) => {
                // An item's own words end before its inner list: they keep their own level.
                paragraph(&mut ops, &mut job, &mut target);
                lists.push(start);
                target.depth = depth(lists.len());
            }
            Event::End(TagEnd::List(_)) => {
                lists.pop();
                target.depth = depth(lists.len());
                if lists.is_empty() && !dense {
                    gap_after_listed(&mut ops, design::Gap::M);
                }
            }
            Event::Start(Tag::Item) => {
                paragraph(&mut ops, &mut job, &mut target);
                if lists.len() > 1 {
                    gap_after_listed(&mut ops, design::Gap::XS);
                }
                let indent = row_indent(lists.len());
                let marker = match lists.last_mut() {
                    Some(Some(number)) => {
                        let marker = format!("{number}. ");
                        *number += 1;
                        marker
                    }
                    _ => "• ".into(),
                };
                job.append(
                    &format!("{indent}{marker}"),
                    0.0,
                    egui::TextFormat {
                        font_id: prose.font(ui),
                        line_height: Some(line),
                        color: colors().muted,
                        ..Default::default()
                    },
                );
                target.lead = (target.depth > 0).then_some(job.text.len());
            }
            Event::TaskListMarker(done) => {
                // The marker follows Item's indentation and bullet, before any prose or links.
                let indent = row_indent(lists.len());
                // A bulleted item trades its bullet for the box; a numbered item keeps its number
                // and the box follows it.
                let bulleted = matches!(lists.last(), Some(None));
                let lead = if bulleted {
                    job = LayoutJob::default();
                    indent.clone()
                } else {
                    std::mem::take(&mut job.text).clone()
                };
                if !bulleted {
                    job = LayoutJob::default();
                    job.append(
                        &lead,
                        0.0,
                        egui::TextFormat {
                            font_id: prose.font(ui),
                            line_height: Some(line),
                            color: colors().muted,
                            ..Default::default()
                        },
                    );
                }
                let icon = if done {
                    egui_phosphor::regular::CHECK_SQUARE
                } else {
                    egui_phosphor::regular::SQUARE
                };
                job.append(
                    &if bulleted {
                        format!("{indent}{icon} ")
                    } else {
                        format!("{icon} ")
                    },
                    0.0,
                    egui::TextFormat {
                        font_id: prose.font(ui),
                        line_height: Some(line),
                        color: colors().muted,
                        ..Default::default()
                    },
                );
                target.lead = (target.depth > 0).then_some(job.text.len());
                target.task = Some((
                    done,
                    design::number_width(ui, &lead, &prose.font(ui)),
                    prose,
                ));
            }
            Event::End(TagEnd::Paragraph | TagEnd::Item) => {
                figures_op(&mut ops, &mut figures, &job);
                paragraph(&mut ops, &mut job, &mut target);
            }
            Event::HardBreak if table.is_none() => job.append(
                "\n",
                0.0,
                egui::TextFormat {
                    font_id: prose.font(ui),
                    line_height: Some(line),
                    ..Default::default()
                },
            ),
            Event::SoftBreak if table.is_none() => job.append(
                if code { "\n" } else { " " },
                0.0,
                egui::TextFormat {
                    font_id: prose.font(ui),
                    line_height: Some(line),
                    ..Default::default()
                },
            ),
            Event::Code(text) if !dense && heading.is_none() => job.append(
                &text,
                0.0,
                egui::TextFormat {
                    font_id: Type::Mono.font(ui),
                    // Mono's own line puts code on the baseline of the serif beside it.
                    line_height: Some(Type::Mono.line()),
                    color: text_color,
                    background: design::reader::inline_code_fill(),
                    ..Default::default()
                },
            ),
            Event::Text(text) | Event::Code(text) => {
                let role = if code {
                    Type::Mono
                } else if let Some(level) = heading {
                    match level {
                        _ if dense => Type::StatLabel,
                        pulldown_cmark::HeadingLevel::H1 => Type::Title,
                        pulldown_cmark::HeadingLevel::H2 | pulldown_cmark::HeadingLevel::H3 => {
                            Type::Section
                        }
                        _ => Type::ProseStrong,
                    }
                } else if strong > 0 {
                    if dense {
                        Type::StatLabel
                    } else if row.is_some() {
                        Type::Strong
                    } else {
                        Type::ProseStrong
                    }
                } else if (italic || quote_depth > 0) && !dense && row.is_none() {
                    Type::ProseItalic
                } else {
                    prose
                };
                // Compact prose keeps the size it was given, whatever its weight.
                let mut font_id = role.font(ui);
                if matches!(role, Type::ProseStrong | Type::ProseItalic) && heading.is_none() {
                    font_id.size = prose.size();
                }
                let color = if matches!(link, Some((_, LinkTarget::Missing))) {
                    colors().muted
                } else if link.is_some() {
                    link_color
                } else {
                    text_color
                };
                job.append(
                    &text,
                    0.0,
                    egui::TextFormat {
                        font_id,
                        line_height: Some(if heading.is_some() || code {
                            role.line()
                        } else {
                            line
                        }),
                        color,
                        underline: if link.is_some()
                            && !matches!(link, Some((_, LinkTarget::Missing)))
                        {
                            egui::Stroke::new(design::HAIRLINE_WIDTH, color)
                        } else {
                            egui::Stroke::NONE
                        },
                        strikethrough: if struck {
                            egui::Stroke::new(1.0, color)
                        } else {
                            egui::Stroke::NONE
                        },
                        // The UI face has no italic of its own: a row's emphasis slants.
                        italics: row.is_some() && italic,
                        ..Default::default()
                    },
                );
            }
            Event::Rule => {
                paragraph(&mut ops, &mut job, &mut target);
                ops.push(Op::Rule);
            }
            _ => {}
        }
    }
    figures_op(&mut ops, &mut figures, &job);
    paragraph(&mut ops, &mut job, &mut target);
    ops
}

/// Notes write a captioned image as `*caption* ([image](url))`. Where the image will show in
/// place, the brackets around the link have nothing left to enclose.
fn local_figure_links(markdown: &str, local: impl Fn(&str) -> bool) -> Cow<'_, str> {
    const OPEN: &str = "([image](";
    if !markdown.contains(OPEN) {
        return Cow::Borrowed(markdown);
    }
    let mut output = String::with_capacity(markdown.len());
    let mut rest = markdown;
    while let Some(start) = rest.find(OPEN) {
        let after = &rest[start + OPEN.len()..];
        match after.find("))") {
            Some(end) if local(&after[..end]) => {
                output.push_str(&rest[..start]);
                output.push_str("[image](");
                output.push_str(&after[..=end]);
                rest = &after[end + 2..];
            }
            _ => {
                output.push_str(&rest[..start + OPEN.len()]);
                rest = after;
            }
        }
    }
    output.push_str(rest);
    Cow::Owned(output)
}

const TABLE_GAP: f32 = design::M;

const TABLE_ROOM: &str = "reader_table_right";

/// Lets this reader's tables use the pane beyond the prose measure, up to the pane's right edge
/// as it is here, as a table in a pane narrower than the measure already does. Prose keeps its
/// measure. Call it outside the measure, around the whole reader body.
pub fn with_wide_tables<R>(ui: &mut egui::Ui, add: impl FnOnce(&mut egui::Ui) -> R) -> R {
    let id = egui::Id::new(TABLE_ROOM);
    let right = ui.max_rect().right();
    let before = ui.ctx().data(|data| data.get_temp::<f32>(id));
    ui.ctx().data_mut(|data| data.insert_temp(id, right));
    let inner = add(ui);
    // Another reader drawn later this frame, such as Lookup's, keeps its own width.
    ui.ctx().data_mut(|data| match before {
        Some(before) => data.insert_temp(id, before),
        None => data.remove::<f32>(id),
    });
    inner
}

/// The width a table here may take, when its reader allows wide tables. It ends at the visible
/// pane's edge, so a table that still scrolls has its scroll area under all of its text.
fn table_room(ui: &egui::Ui) -> Option<f32> {
    ui.ctx()
        .data(|data| data.get_temp::<f32>(egui::Id::new(TABLE_ROOM)))
        .map(|right| right.min(ui.clip_rect().right()) - ui.cursor().left())
}

/// The row label has its own column. Only the remaining cells belong to the
/// horizontal scroll area, so its real viewport also supplies the cell padding.
fn reader_table(
    ui: &mut egui::Ui,
    rows: &[Vec<String>],
    columns: &[usize],
    widths: &[f32],
    display: &dyn Fn(&str) -> String,
    matched: Option<usize>,
    mark: Option<f32>,
) {
    if widths.is_empty() {
        return;
    }
    // The roll mark's slot before the first column's words, and the mark itself.
    let slot = mark.unwrap_or(0.0);
    let dice = mark.map(|_| {
        ui.painter().layout_no_wrap(
            design::icons::DICE.to_owned(),
            Type::Stat.font(ui),
            colors().text,
        )
    });
    let cells: Vec<Vec<_>> = rows
        .iter()
        .enumerate()
        .map(|(row, cells)| {
            columns
                .iter()
                .enumerate()
                .map(|(position, column)| {
                    let text = display(cells.get(*column).map(String::as_str).unwrap_or(""));
                    let role = if row == 0 {
                        Type::StatLabel
                    } else {
                        Type::Stat
                    };
                    let width = if position == 0 {
                        widths[0] - slot
                    } else {
                        widths[position]
                    };
                    ui.painter()
                        .layout(text, role.font(ui), colors().text, width)
                })
                .collect()
        })
        .collect();
    let heights: Vec<_> = cells
        .iter()
        .map(|row| {
            row.iter().map(|cell| cell.size().y).fold(0.0, f32::max)
                + design::XS * 2.0
                + design::HAIRLINE_WIDTH
        })
        .collect();
    let height: f32 = heights.iter().sum();
    let width = widths.iter().sum::<f32>() + TABLE_GAP * widths.len().saturating_sub(1) as f32;
    let scrolls = width > ui.available_width();
    if scrolls {
        ui.label(
            Type::Small
                .rich(ui, "More columns: scroll sideways or use Shift+wheel.")
                .color(colors().muted),
        );
    }
    // The sideways scroll bar has its own lane below the last row (the part's solid bar), so it
    // never covers cell text.
    let lane = if scrolls {
        let scroll = ui.spacing().scroll;
        scroll.bar_width + scroll.bar_outer_margin
    } else {
        0.0
    };
    let top = ui.cursor().top();
    let left = ui.cursor().left();
    let outer_clip = ui.clip_rect();
    // While the table's own lane is below view, egui floats the bar up to the reader's bottom
    // edge. There the lane stays at that edge, as the header stays at the top: rows pass under
    // its hairline, so the bar never covers cell text.
    let cut = (scrolls && top < outer_clip.bottom() && top + height + lane > outer_clip.bottom())
        .then(|| (outer_clip.bottom() - lane, outer_clip.bottom()));
    // The header follows the top edge until the final row leaves the reader.
    let header_top = top.max(outer_clip.top()).min(top + height - heights[0]);
    // Round 27 FC3: while the header floats, a row whose text would start under it shows no
    // text, so no cut line of text shows between the header and the next row's rule.
    let cover = (header_top > top).then(|| header_top + heights[0]);
    let mut moving_origin = egui::pos2(left + widths[0] + TABLE_GAP, top);
    let mut moving_clip = outer_clip.intersect(egui::Rect::from_min_max(
        egui::pos2(left + widths[0] + TABLE_GAP, outer_clip.top()),
        outer_clip.max,
    ));
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = TABLE_GAP;
        ui.allocate_ui_with_layout(
            egui::vec2(widths[0], height),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                table_part(
                    ui,
                    &cells,
                    widths,
                    &heights,
                    0..1,
                    matched,
                    (cut, cover),
                    mark_at(slot, &dice),
                );
            },
        );
        if widths.len() > 1 {
            let moving_width = width - widths[0] - TABLE_GAP;
            let mut show = |ui: &mut egui::Ui| {
                ui.set_width(moving_width);
                moving_origin = ui.cursor().min;
                ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                    table_part(
                        ui,
                        &cells,
                        widths,
                        &heights,
                        1..widths.len(),
                        matched,
                        (cut, cover),
                        None,
                    );
                });
            };
            if scrolls {
                ui.style_mut().always_scroll_the_only_direction = false;
                // Each surface and document keeps its own sideways place for this table: the
                // scroll's own id says whose it is, so nothing writes its offset back.
                let document = reader_document(ui);
                let surface = ui
                    .ctx()
                    .data(|data| data.get_temp::<egui::Id>(egui::Id::new("reader_surface")));
                let output = design::sideways_scroll(
                    ui,
                    ("reader_table", surface, document, rows),
                    design::Sideways::default()
                        .fit()
                        .without_bar(top >= outer_clip.bottom()),
                    &mut show,
                );
                moving_clip = outer_clip.intersect(output.inner_rect);
            } else {
                ui.allocate_ui_with_layout(
                    egui::vec2(moving_width, height),
                    egui::Layout::top_down(egui::Align::Min),
                    &mut show,
                );
            }
        }
    });
    let painter = ui.painter().with_clip_rect(outer_clip);
    let rows_end = cut.map_or(top + height, |(lane_top, _)| (top + height).min(lane_top));
    if let Some((lane_top, _)) = cut {
        painter.hline(
            left..=(left + width).min(moving_clip.right()),
            lane_top,
            egui::Stroke::new(design::HAIRLINE_WIDTH, colors().hairline),
        );
    }
    if let Some(header) = cells.first() {
        let header_rect = egui::Rect::from_min_max(
            egui::pos2(left, header_top),
            egui::pos2(
                (left + width).min(if widths.len() > 1 {
                    moving_clip.right()
                } else {
                    outer_clip.right()
                }),
                header_top + heights[0],
            ),
        );
        painter.rect_filled(header_rect, egui::CornerRadius::ZERO, colors().page);
        let fixed = egui::Rect::from_min_size(
            egui::pos2(left, header_top),
            egui::vec2(widths[0], heights[0]),
        );
        painter.rect_filled(fixed, egui::CornerRadius::ZERO, colors().page);
        painter.galley(
            fixed.min + egui::vec2(slot, design::XS),
            header[0].clone(),
            colors().text,
        );
        if widths.len() > 1 {
            let painter = painter.with_clip_rect(moving_clip);
            let strip = egui::Rect::from_min_size(
                egui::pos2(moving_clip.left(), header_top),
                egui::vec2(moving_clip.width(), heights[0]),
            );
            painter.rect_filled(strip, egui::CornerRadius::ZERO, colors().page);
            let mut x = moving_origin.x;
            for column in 1..widths.len() {
                painter.galley(
                    egui::pos2(x, header_top + design::XS),
                    header[column].clone(),
                    colors().text,
                );
                x += widths[column] + TABLE_GAP;
            }
        }
        painter.hline(
            header_rect.x_range(),
            header_rect.bottom(),
            egui::Stroke::new(design::HAIRLINE_WIDTH, colors().hairline),
        );
    }
    // The first column's rule shows only while the other columns scroll under it, and runs
    // through the header too. A table that fits has no column rules.
    if scrolls {
        painter.vline(
            left + widths[0],
            top..=rows_end,
            egui::Stroke::new(design::HAIRLINE_WIDTH, colors().hairline),
        );
    }
}

/// The first column's roll mark slot: its width and the mark, for a rollable table.
type MarkSlot<'a> = Option<(f32, &'a std::sync::Arc<egui::Galley>)>;

fn mark_at(slot: f32, dice: &Option<std::sync::Arc<egui::Galley>>) -> MarkSlot<'_> {
    dice.as_ref().map(|dice| (slot, dice))
}

#[expect(clippy::too_many_arguments)]
fn table_part(
    ui: &mut egui::Ui,
    cells: &[Vec<std::sync::Arc<egui::Galley>>],
    widths: &[f32],
    heights: &[f32],
    columns: std::ops::Range<usize>,
    matched: Option<usize>,
    (cut, cover): (Option<(f32, f32)>, Option<f32>),
    mark: MarkSlot<'_>,
) {
    ui.spacing_mut().item_spacing.y = 0.0;
    // Under the scroll bar's lane (`cut`: its top, and the reader's visible bottom) a row is out
    // of view: it is drawn clipped at the lane, and its cells are not controls.
    let painter = match cut {
        Some((lane_top, _)) => ui.painter().with_clip_rect(
            ui.clip_rect()
                .intersect(egui::Rect::everything_above(lane_top)),
        ),
        None => ui.painter().clone(),
    };
    let width = widths[columns.clone()].iter().sum::<f32>()
        + TABLE_GAP * columns.len().saturating_sub(1) as f32;
    for (row, cells) in cells.iter().enumerate() {
        let (rect, _) =
            ui.allocate_exact_size(egui::vec2(width, heights[row]), egui::Sense::hover());
        if matched == Some(row) {
            let mut fill = rect;
            if columns.start == 0 && columns.end == 1 && widths.len() > 1 {
                fill.max.x += TABLE_GAP;
            }
            painter.rect_filled(fill, egui::CornerRadius::ZERO, colors().selected);
        }
        let mut x = rect.left();
        // `cover`: the floating header's bottom, while it floats. A row whose first line starts
        // above it has lines under the header (row 0 is the header itself).
        let under_header = row > 0 && cover.is_some_and(|cover| rect.top() + design::XS < cover);
        // The rolled row's mark sits in the slot; every row's range starts after it.
        if let Some((slot, dice)) = mark {
            if matched == Some(row) && !under_header {
                painter.galley(
                    egui::pos2(x, rect.top() + design::XS),
                    dice.clone(),
                    colors().text,
                );
            }
            x += slot;
        }
        for column in columns.clone() {
            let cell = egui::Rect::from_min_size(
                egui::pos2(x, rect.top() + design::XS),
                cells[column].size(),
            );
            // Round 27 FC3: under the header, the cell shows from its first whole line below the
            // header's bottom, so no cut line shows and the lines below it stay readable. A
            // cell with no whole line below the header shows nothing.
            let shown_from = if under_header {
                let cover = cover.unwrap_or(f32::NEG_INFINITY);
                cells[column]
                    .rows
                    .iter()
                    .map(|line| cell.top() + line.rect().top())
                    .find(|top| *top >= cover - 0.5)
            } else {
                Some(f32::NEG_INFINITY)
            };
            if let Some(shown_from) = shown_from {
                let below = egui::Rect::everything_below(shown_from);
                if cut.is_some_and(|(lane_top, bottom)| {
                    cell.bottom() > lane_top && cell.top() < bottom
                }) {
                    painter
                        .with_clip_rect(painter.clip_rect().intersect(below))
                        .galley(cell.min, cells[column].clone(), colors().text);
                } else if under_header {
                    // A child Ui keeps the cell a label, clipped above its first whole line.
                    let mut shown = ui.new_child(
                        egui::UiBuilder::new()
                            .max_rect(cell)
                            .layout(egui::Layout::top_down(egui::Align::Min)),
                    );
                    shown.set_clip_rect(shown.clip_rect().intersect(below));
                    shown.add(egui::Label::new(cells[column].clone()));
                } else {
                    ui.place(cell, egui::Label::new(cells[column].clone()));
                }
            }
            x += widths[column] + TABLE_GAP;
        }
        painter.hline(
            rect.x_range(),
            rect.bottom(),
            egui::Stroke::new(design::HAIRLINE_WIDTH, colors().hairline),
        );
    }
}

/// Fit columns to their text without breaking words. A table that cannot fit
/// those widths scrolls within its reader.
fn column_widths(
    ui: &egui::Ui,
    rows: &[Vec<String>],
    columns: &[usize],
    display: &dyn Fn(&str) -> String,
) -> Vec<f32> {
    let font = Type::StatLabel.font(ui);
    let measure = |text: &str| {
        ui.fonts_mut(|fonts| {
            fonts
                .layout_no_wrap(text.to_owned(), font.clone(), colors().text)
                .size()
                .x
                .ceil()
                + design::XS
        })
    };
    let mut natural = vec![1.0_f32; columns.len()];
    let mut floor = vec![1.0_f32; columns.len()];
    for (row_index, row) in rows.iter().enumerate() {
        for (position, &column) in columns.iter().enumerate() {
            let cell = display(row.get(column).map(String::as_str).unwrap_or(""));
            natural[position] = natural[position].max(measure(&cell));
            if row_index == 0 {
                floor[position] = floor[position].max(measure(&cell));
            }
            for word in cell.split_whitespace() {
                floor[position] = floor[position].max(measure(word));
            }
        }
    }
    let gaps = TABLE_GAP * columns.len().saturating_sub(1) as f32;
    let available = (ui.available_width() - gaps - design::XS).max(1.0);
    let total: f32 = natural.iter().sum();
    if total <= available {
        return natural;
    }
    let floors: f32 = floor.iter().sum();
    if floors >= available {
        return floor;
    }
    let share = (available - floors) / (total - floors).max(1.0);
    natural
        .iter()
        .zip(&floor)
        .map(|(natural, floor)| floor + (natural - floor) * share)
        .collect()
}

fn show_fields(ui: &mut egui::Ui, rows: &[Vec<String>]) {
    let available = ui.available_width();
    let label_width = (available * 0.25).clamp(100.0, 145.0);
    let value_width = (available - label_width - 16.0).max(70.0);
    ui.push_id(ui.next_auto_id(), |ui| {
        egui::Grid::new("library_fields")
            .num_columns(2)
            .spacing([design::M, 3.0])
            .min_col_width(0.0)
            .max_col_width(value_width.max(label_width))
            .show(ui, |ui| {
                for row in rows {
                    let field = row
                        .first()
                        .map_or("", String::as_str)
                        .trim()
                        .replace('_', " ");
                    let value = row.get(1).map_or("", String::as_str).trim();
                    let value = display_field_value(value);
                    field_cell(
                        ui,
                        label_width,
                        Type::StatLabel.rich(ui, field).color(colors().text),
                    );
                    field_cell(
                        ui,
                        value_width,
                        Type::Stat.rich(ui, value).color(colors().text),
                    );
                    ui.end_row();
                }
            });
    });
}

fn field_cell(ui: &mut egui::Ui, width: f32, text: egui::RichText) {
    ui.allocate_ui_with_layout(
        egui::vec2(width, 0.0),
        egui::Layout::top_down(egui::Align::Min),
        |ui| {
            ui.set_width(width);
            ui.add(egui::Label::new(text).wrap());
        },
    );
}

fn display_field_value(value: &str) -> String {
    match serde_json::from_str::<serde_json::Value>(value) {
        Ok(serde_json::Value::Array(values)) if values.iter().all(|v| v.is_string()) => values
            .iter()
            .filter_map(|v| v.as_str())
            .collect::<Vec<_>>()
            .join(", "),
        Ok(serde_json::Value::Bool(value)) => if value { "Yes" } else { "No" }.into(),
        _ => value.to_owned(),
    }
}

/// Each level of quote moves its words this far right. Its bar is in that gap.
const QUOTE_INDENT: f32 = design::QUOTE_INDENT;
const QUOTE_BAR_WIDTH: f32 = 2.0;

/// What the words waiting to become a paragraph link to.
#[derive(Default)]
struct Targets {
    links: Vec<(std::ops::Range<usize>, LinkTarget)>,
    task: Option<(bool, f32, Type)>,
    /// A plan row's words: no space follows them.
    row: bool,
    /// The bytes of what starts the words in its own column, so the words' wrapped lines start
    /// at their text edge: in the Reader a list item's mark, in a plan row's words a missing
    /// link's icon (row 49).
    lead: Option<usize>,
    /// How many lists the words are in, in the Reader; 0 in a plan row's words, which keep a
    /// mark in their line.
    depth: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LinkTarget {
    Missing,
    Note(String),
    Lookup(String),
    Url(String),
    /// A dice link in a stat block or a sheet. The run or the preview stream rolls it.
    Roll(splat::content::dice_text::RollLink),
}

impl LinkTarget {
    /// Where a Markdown link's destination goes, if it can be followed.
    pub fn from_destination(destination: &str) -> Option<Self> {
        if let Some(note) = destination.strip_prefix(NOTE_LINK) {
            Some(Self::Note(note.to_owned()))
        } else if let Some(link) =
            splat::content::dice_text::RollLink::from_destination(destination)
        {
            Some(Self::Roll(link))
        } else if destination.starts_with("https://") || destination.starts_with("http://") {
            Some(Self::Url(destination.to_owned()))
        } else {
            None
        }
    }
}

/// A link's underline, or a missing link's dots, sit this far below the words' baseline.
const UNDERLINE_BELOW_BASELINE: f32 = design::HAIRLINE_WIDTH * 2.0;

/// A paragraph with links, laid out once: each short link's spare width reserved, rows with
/// links lifted to a full target, and each link's underline taken out of the words, to be drawn
/// by the paragraph 2 px below the baseline (so a taller row never moves it down to the bottom
/// of the target, near the next line).
fn lay_out_links(
    ui: &egui::Ui,
    mut job: LayoutJob,
    mut links: Vec<(std::ops::Range<usize>, LinkTarget)>,
    short_links: ShortLinks,
) -> LaidLinks {
    // The paragraph's own words, before any display space is added.
    let text = job.text.clone();
    let spare = reserve_short_links(ui, &mut job, &mut links, short_links);
    lift_link_rows(ui, &mut job);
    // A one-line linked paragraph still needs a full target. Set only after `lift_link_rows`:
    // that measures each row, and a first row already raised to `XL` here would not be lifted,
    // so its words would sit at its bottom, while a lifted row keeps them where an unlinked row
    // has them (an uneven pitch, and a larger gap above the paragraph).
    // After the lift every row is at least `XL`, so this is only a floor.
    job.first_row_min_height = job.first_row_min_height.max(design::XL);
    let mut underlines: Vec<egui::Stroke> = vec![egui::Stroke::NONE; links.len()];
    for section in &mut job.sections {
        if section.format.underline == egui::Stroke::NONE {
            continue;
        }
        let range = &section.byte_range;
        if let Some(at) = links
            .iter()
            .position(|(link, _)| link.start <= range.start && range.end <= link.end)
        {
            underlines[at] = section.format.underline;
            section.format.underline = egui::Stroke::NONE;
        }
    }
    let galley = lay_out(ui, job, true);
    let words = link_words(&galley, egui::Pos2::ZERO, &links);
    let targets = link_targets(
        &words,
        &spare,
        egui::Rect::from_min_size(egui::Pos2::ZERO, galley.size()),
    );
    let bounds = words
        .iter()
        .zip(&targets)
        .map(|(parts, rects)| {
            parts
                .iter()
                .map(|part| part.rect)
                .chain(rects.iter().copied())
                .fold(egui::Rect::NOTHING, egui::Rect::union)
        })
        .collect();
    let keys = links
        .iter()
        .map(|(_, destination)| egui::Id::new(format!("{destination:?}")).value())
        .collect();
    LaidLinks {
        text,
        galley,
        links,
        underlines,
        words,
        targets,
        bounds,
        keys,
    }
}

/// A paragraph stays one paragraph. Each link in it is its own control: Tab reaches the
/// links in reading order, Enter or Space follows the focused one, and the pointer presses
/// only a link's own target. `links_id` is the paragraph's block in its reader.
fn draw_laid_links(
    ui: &mut egui::Ui,
    laid: &LaidLinks,
    links_id: egui::Id,
) -> (egui::Rect, egui::Pos2) {
    let LaidLinks {
        text,
        galley,
        links,
        underlines,
        words,
        targets,
        bounds,
        keys,
    } = laid;
    let (position, galley, response) = egui::Label::new(galley.clone())
        .sense(egui::Sense::hover())
        .layout_in_ui(ui);
    // The paragraph reads as plain text, as an unlinked one does. Its links follow as controls.
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, text));
    // Its words, their underlines and their text runs only where they can be seen, as an egui
    // label paints and reads out its own (layout PR 9b-R1a: an unseen paragraph costs nothing
    // that grows with its words). Its links stay controls wherever they are, so Tab reaches them.
    let shown = ui.is_rect_visible(response.rect);
    if shown {
        ui.painter().galley(position, galley.clone(), colors().text);
        egui::text_selection::accesskit_text::update_accesskit_for_text_widget(
            ui.ctx(),
            response.id,
            None,
            egui::accesskit::Role::Label,
            egui::emath::TSTransform::from_translation(position.to_vec2()),
            &galley,
        );
    }
    let at = position.to_vec2();
    let place = |part: &LinkPart| LinkPart {
        left: part.left + at.x,
        right: part.right + at.x,
        y: part.y + at.y,
        rect: part.rect.translate(at),
        row_start: part.row_start,
        entry_left: part.entry_left.map(|left| left + at.x),
    };
    let clip = ui.clip_rect();
    for (index, (range, destination)) in links.iter().enumerate() {
        // A link in view: its underline, a missing link's dots and every part of its target.
        // A link out of view: its first target only, so Tab still reaches it, at the same cost
        // whatever its words.
        let in_view = shown && clip.intersects(bounds[index].translate(at));
        // A link's parts are in row order: only the rows in view draw (a link that wraps over
        // thousands of rows costs what its rows in view cost).
        let rows = |rects: &[egui::Rect]| {
            let from = rects.partition_point(|rect| rect.bottom() + at.y < clip.top());
            let to = rects.partition_point(|rect| rect.top() + at.y <= clip.bottom());
            from..to.max(from)
        };
        // The link's words' rows in view (none when the link is out of view).
        let words_seen = if in_view {
            let parts = &words[index];
            let from = parts.partition_point(|part| part.rect.bottom() + at.y < clip.top());
            let to = parts.partition_point(|part| part.rect.top() + at.y <= clip.bottom());
            from..to.max(from)
        } else {
            0..0
        };
        if in_view {
            for part in words[index][words_seen.clone()].iter().map(place) {
                if underlines[index] != egui::Stroke::NONE {
                    let y = ui
                        .painter()
                        .round_to_pixel_center(part.y + UNDERLINE_BELOW_BASELINE);
                    ui.painter()
                        .hline(part.left..=part.right, y, underlines[index]);
                }
                if matches!(destination, LinkTarget::Missing) {
                    let y = part.y + UNDERLINE_BELOW_BASELINE;
                    let mut dot = part.left;
                    while dot < part.right {
                        ui.painter().circle_filled(
                            egui::pos2(dot, y),
                            design::HAIRLINE_WIDTH / 2.0,
                            colors().muted,
                        );
                        dot += design::XS;
                    }
                }
            }
        }
        // The first part always (Tab reaches the link wherever it is); the others in view.
        let all = &targets[index];
        let seen = if in_view { rows(all) } else { 0..0 };
        let rects = all.iter().enumerate().take(1).chain(
            all.iter()
                .enumerate()
                .skip(seen.start.max(1))
                .take(seen.end.saturating_sub(seen.start.max(1))),
        );
        let words_text = &galley.job.text[range.clone()];
        let id = links_id.with((index, keys[index]));
        // Only the first part of a wrapped link takes focus, so Tab stops once per link.
        let parts: Vec<egui::Response> = rects
            .map(|(part, rect)| {
                let first = part == 0 && !matches!(destination, LinkTarget::Missing);
                let sense = if first {
                    egui::Sense::click()
                } else {
                    egui::Sense::CLICK
                };
                // On the UI's grid, as a target built where it stands would be: a target kept at
                // the galley's origin and moved keeps its whole size.
                let part_response = ui.interact(
                    egui::emath::GuiRounding::round_ui(rect.translate(at)),
                    id.with(part),
                    sense,
                );
                let role = if first {
                    egui::WidgetType::Link
                } else {
                    egui::WidgetType::Label
                };
                // The name is built only when an accessibility reader or a test asks for it.
                part_response.widget_info(|| {
                    egui::WidgetInfo::labeled(role, true, link_name(words_text, destination))
                });
                part_response
            })
            .collect();
        if parts.first().is_some_and(egui::Response::has_focus) {
            // The ring sits `XS / 2` outside the words, so it never covers a letter, even for a
            // link at the start of a line. It stays inside the reader's visible area, which
            // keeps a gutter beside the text for it: only on the rows in view (PR 861 review).
            for part in words[index][words_seen.clone()].iter().map(place) {
                let ring = part.rect.expand2(egui::vec2(design::XS / 2.0, 0.0));
                design::focus_ring(ui, ring.intersect(clip));
            }
        }
        // A click by key or pointer wins over the pointer resting on another row of the link.
        if let Some(pressed) = parts
            .iter()
            .find(|part| part.clicked())
            .or_else(|| parts.iter().find(|part| part.hovered()))
        {
            follow(ui, pressed, destination);
        }
    }
    (response.rect, position)
}

/// A link's press or hover: a roll's own interaction, or its note or Lookup entry opened.
fn follow(ui: &egui::Ui, pressed: &egui::Response, destination: &LinkTarget) {
    if let LinkTarget::Roll(link) = destination {
        crate::rolls::interact(ui, pressed, link);
        return;
    }
    match press(ui, pressed, destination) {
        Some(LinkTarget::Note(note)) => {
            let from = reader_document(ui);
            send_note_link(ui.ctx(), note.clone(), from);
        }
        Some(LinkTarget::Lookup(words)) => send_lookup_link(ui.ctx(), words.clone()),
        _ => {}
    }
}

/// The links of a paragraph out of view (layout PR 9b-R2), its galley at `at`: each link's first
/// target only, with the id, the role and the name it has when drawn, so Tab and a screen reader
/// reach it wherever it is, at the same cost whatever its words. Its focus ring and the rest of
/// its parts come with its paragraph, which the next pass draws in view (`reveal_focus`).
fn register_links(ui: &egui::Ui, laid: &LaidLinks, links_id: egui::Id, at: egui::Pos2) {
    let at = at.to_vec2();
    for (index, (range, destination)) in laid.links.iter().enumerate() {
        let Some(first) = laid.targets[index].first() else {
            continue;
        };
        let sense = if matches!(destination, LinkTarget::Missing) {
            egui::Sense::CLICK
        } else {
            egui::Sense::click()
        };
        let role = if matches!(destination, LinkTarget::Missing) {
            egui::WidgetType::Label
        } else {
            egui::WidgetType::Link
        };
        let response = ui.interact(
            egui::emath::GuiRounding::round_ui(first.translate(at)),
            links_id.with((index, laid.keys[index])).with(0_usize),
            sense,
        );
        let words = &laid.galley.job.text[range.clone()];
        response
            .widget_info(|| egui::WidgetInfo::labeled(role, true, link_name(words, destination)));
        if response.clicked() {
            follow(ui, &response, destination);
        }
    }
}

/// A link whose words are narrower than `XL` gets `XL` of layout width. A link inside a line has
/// its words in the middle of that space, so the spare width is split before and after them. A
/// link that starts a line keeps its words at the text edge, with all the space after them. A
/// link that punctuation follows, or that ends its line, keeps its words where the text puts
/// them: the punctuation sits directly after its words and the spare width goes after the
/// punctuation. The text after a link starts after its space, so the line wraps if it must. The
/// space is display only: the paragraph's words do not change. Links `XL` or wider keep their
/// layout. In a row of `ShortLinks::Entries` (a Skills row), a link that does not start a line
/// adds no space: its target extends left over its own entry instead.
///
/// Returns, for each link, where its spare width went, so its target grows the same way.
fn reserve_short_links(
    ui: &egui::Ui,
    job: &mut LayoutJob,
    links: &mut [(std::ops::Range<usize>, LinkTarget)],
    short_links: ShortLinks,
) -> Vec<Spare> {
    let mut measure = job.clone();
    measure.wrap.max_width = f32::INFINITY;
    let galley = ui.fonts_mut(|fonts| fonts.layout_job(measure));
    // Each short link, by its index in `links`, and the space it still needs.
    let short: Vec<(usize, f32)> = link_words(&galley, egui::Pos2::ZERO, links)
        .iter()
        .enumerate()
        .filter_map(|(index, parts)| {
            let width: f32 = parts.iter().map(|part| part.right - part.left).sum();
            // Glyphs snap to whole pixels, which can take up to a pixel from the space.
            (!parts.is_empty() && width < design::XL).then_some((index, design::XL - width + 1.0))
        })
        .collect();
    if short.is_empty() {
        return vec![Spare::Split; links.len()];
    }
    // A link at the start of the paragraph or after a line break starts a line. One that wraps to
    // the start of a row is found by laying the paragraph out at the width its label wraps at.
    // Moving a link's space after it can only change the rows after that link, so each pass
    // keeps the links it found.
    let mut starts: Vec<bool> = short
        .iter()
        .map(|(index, _)| {
            let start = links[*index].0.start;
            start == 0 || job.text[..start].ends_with('\n')
        })
        .collect();
    let original = (job.clone(), links.to_vec());
    for _ in 0..4 {
        *job = original.0.clone();
        links.clone_from_slice(&original.1);
        let spare = place_short_links(ui, job, links, &short, &starts, short_links);
        let mut measure = job.clone();
        measure.wrap.max_width = ui.available_width();
        let galley = ui.fonts_mut(|fonts| fonts.layout_job(measure));
        let words = link_words(&galley, egui::Pos2::ZERO, links);
        let mut changed = false;
        for ((index, _), starts) in short.iter().zip(&mut starts) {
            if !*starts && words[*index].first().is_some_and(|part| part.row_start) {
                *starts = true;
                changed = true;
            }
        }
        if !changed {
            return spare;
        }
    }
    *job = original.0;
    links.clone_from_slice(&original.1);
    place_short_links(ui, job, links, &short, &starts, short_links)
}

/// Where a short link's spare width goes, and so the way its target grows from its words.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Spare {
    /// Half before and half after its words: its target is centred on them.
    #[default]
    Split,
    /// After its words and any punctuation that follows them: its last target grows right from
    /// its words.
    After,
    /// None in the line: the link ends an entry, and its first target grows left from its
    /// words over that entry.
    Entry,
}

/// Punctuation that closes a phrase. It sits directly after a short link's words.
fn closes_phrase(character: char) -> bool {
    matches!(
        character,
        ',' | '.'
            | ';'
            | ':'
            | '!'
            | '?'
            | ')'
            | ']'
            | '}'
            | '\''
            | '"'
            | '\u{2019}'
            | '\u{201D}'
            | '\u{2026}'
    )
}

/// Cuts the section that holds byte `at` in two, so that a space can start at `at`.
fn cut_section_at(job: &mut LayoutJob, at: usize) {
    if let Some(index) = job
        .sections
        .iter()
        .position(|section| section.byte_range.start < at && at < section.byte_range.end)
    {
        let mut tail = job.sections[index].clone();
        tail.leading_space = 0.0;
        tail.byte_range = at..tail.byte_range.end;
        job.sections[index].byte_range.end = at;
        job.sections.insert(index + 1, tail);
    }
}

/// Adds each short link's space: half before and half after its words, all after them when it
/// starts a line, ends its line or punctuation follows it, or none for a link that ends an entry
/// (`ShortLinks::Entries`) and does not start a line. Returns, for each link, where its space
/// went.
fn place_short_links(
    ui: &egui::Ui,
    job: &mut LayoutJob,
    links: &mut [(std::ops::Range<usize>, LinkTarget)],
    short: &[(usize, f32)],
    starts: &[bool],
    short_links: ShortLinks,
) -> Vec<Spare> {
    let mut spare = vec![Spare::Split; links.len()];
    // Where each short link ends, and the space that still goes there, in text order.
    let mut spaces: Vec<(usize, f32)> = Vec::new();
    // Where the last short link ended, and the space after it.
    let mut previous: Option<(usize, f32)> = None;
    for (&(index, space), &starts) in short.iter().zip(starts) {
        let range = links[index].0.clone();
        // Punctuation after the link sits directly after its words. The space goes after it.
        let punctuation: usize = job.text[range.end..]
            .chars()
            .take_while(|character| closes_phrase(*character))
            .map(char::len_utf8)
            .sum();
        let end = range.end + punctuation;
        let at_line_end = job.text[end..].starts_with('\n') || end == job.text.len();
        let link_follows = links.iter().any(|(other, _)| other.start == end);
        // A Skills bonus keeps the row's own layout: one blank after its name and one after its
        // comma. Its target covers its own entry instead. A link that must start a line (an
        // entry wider than the line) has no entry before it on its row, so it keeps its space
        // after its words. A link right before another link keeps its space after it, so one
        // underline does not join both.
        if short_links == ShortLinks::Entries && !starts && !link_follows {
            spare[index] = Spare::Entry;
            previous = Some((range.end, 0.0));
            continue;
        }
        // Nothing after the words needs keeping apart from them, so they stay where the text
        // puts them, with all the space after them.
        let held = punctuation > 0 || at_line_end;
        if held {
            spare[index] = Spare::After;
        }
        let own = job.sections.iter_mut().find(|section| {
            section.byte_range.start == range.start && !section.byte_range.is_empty()
        });
        // A row drops the space before its first glyph, so a link that starts a line keeps its
        // words at the text edge.
        let before = match own {
            Some(own) if !starts && !held => {
                own.leading_space += space / 2.0;
                space / 2.0
            }
            _ => 0.0,
        };
        let mut space = space - before;
        // A link right after another short link has the first one's space before it as well as
        // its own. It gets as much space after it, so the pair reads evenly; a blank that
        // follows in the text counts toward it.
        if let Some((_, after)) = previous.filter(|(end, _)| *end == range.start)
            && !at_line_end
            && !link_follows
        {
            let next = job.text[end..].chars().next();
            let blank = next
                .filter(|next| next.is_whitespace())
                .map_or(0.0, |next| {
                    let font = job
                        .sections
                        .iter()
                        .find(|section| section.byte_range.contains(&end))
                        .map(|section| section.format.font_id.clone());
                    font.map_or(0.0, |font| {
                        ui.fonts_mut(|fonts| fonts.glyph_width(&font, next))
                    })
                });
            space += (after - blank).max(0.0);
        }
        previous = Some((range.end, space));
        if punctuation > 0 {
            // The punctuation's own width, and a blank after it, are part of the link's spare
            // width: the target only has to end before the next word.
            let font = job
                .sections
                .iter()
                .find(|section| section.byte_range.contains(&range.end))
                .map(|section| section.format.font_id.clone());
            let blank = job.text[end..]
                .chars()
                .next()
                .filter(|next| next.is_whitespace() && *next != '\n');
            let width: f32 = font.map_or(0.0, |font| {
                ui.fonts_mut(|fonts| {
                    job.text[range.end..end]
                        .chars()
                        .chain(blank)
                        .map(|character| fonts.glyph_width(&font, character))
                        .sum()
                })
            });
            space = (space - width).max(0.0);
            cut_section_at(job, end);
        }
        match job
            .sections
            .iter_mut()
            .find(|section| section.byte_range.start == end && !section.byte_range.is_empty())
        {
            // The text after the link starts after the reserved space.
            Some(next) if !at_line_end && !link_follows => next.leading_space += space,
            // A blank holds the space: where nothing follows on the line, so the paragraph's own
            // width includes it, and before another link, so one underline does not join both.
            _ => spaces.push((end, space)),
        }
    }
    spaces.sort_by_key(|(at, _)| *at);
    if spaces.is_empty() {
        return spare;
    }
    let mut text = String::with_capacity(job.text.len() + spaces.len());
    let mut sections = Vec::with_capacity(job.sections.len() + spaces.len());
    let mut pending = spaces.iter().peekable();
    for section in &job.sections {
        let start = text.len();
        text.push_str(&job.text[section.byte_range.clone()]);
        let mut moved = section.clone();
        moved.byte_range = start..text.len();
        sections.push(moved);
        while let Some(&&(at, space)) = pending.peek() {
            if at > section.byte_range.end {
                break;
            }
            pending.next();
            if at < section.byte_range.end {
                continue;
            }
            let mut format = section.format.clone();
            format.underline = egui::Stroke::NONE;
            format.strikethrough = egui::Stroke::NONE;
            format.background = egui::Color32::TRANSPARENT;
            let blank = ui.fonts_mut(|fonts| fonts.glyph_width(&format.font_id, ' '));
            let start = text.len();
            text.push(' ');
            sections.push(egui::text::LayoutSection {
                leading_space: (space - blank).max(0.0),
                byte_range: start..text.len(),
                format,
            });
        }
    }
    let added = |at: usize, inclusive: bool| {
        spaces
            .iter()
            .filter(|(space, _)| *space < at || (inclusive && *space == at))
            .count()
    };
    for (range, _) in links.iter_mut() {
        *range = range.start + added(range.start, true)..range.end + added(range.end, false);
    }
    job.text = text;
    job.sections = sections;
    spare
}

/// Where the body line is shorter than `XL`, every row of a paragraph that holds a link grows to
/// `XL`, so each link's target is `XL` tall and stays on its own row, and the paragraph keeps one
/// line pitch. Every part of a row grows by the same amount, so the words keep their baselines.
/// A paragraph without a link keeps its line height.
fn lift_link_rows(ui: &egui::Ui, job: &mut LayoutJob) {
    // Reader prose is already `XL` tall, so it needs no second layout.
    if job.sections.iter().all(|section| {
        section
            .format
            .line_height
            .is_some_and(|line| line >= design::XL)
    }) {
        return;
    }
    let mut measure = job.clone();
    // The width the paragraph's label wraps at.
    measure.wrap.max_width = ui.available_width();
    let galley = ui.fonts_mut(|fonts| fonts.layout_job(measure));
    let mut lifts: Vec<(std::ops::Range<usize>, f32)> = Vec::new();
    let mut byte_at = 0;
    for row in &galley.rows {
        let start = byte_at;
        byte_at += row
            .glyphs
            .iter()
            .map(|glyph| glyph.chr.len_utf8())
            .sum::<usize>();
        let end = byte_at;
        if row.ends_with_newline {
            byte_at += 1;
        }
        if row.height() < design::XL {
            lifts.push((start..end, design::XL - row.height()));
        }
    }
    if lifts.is_empty() {
        return;
    }
    // Cut the sections at the lifted rows' edges, then lift the parts inside those rows.
    let mut sections = Vec::with_capacity(job.sections.len() + 2 * lifts.len());
    for section in &job.sections {
        let range = section.byte_range.clone();
        let mut cuts: Vec<usize> = lifts
            .iter()
            .flat_map(|(row, _)| [row.start, row.end])
            .filter(|cut| range.start < *cut && *cut < range.end)
            .collect();
        cuts.sort_unstable();
        cuts.dedup();
        cuts.push(range.end);
        let mut from = range.start;
        for cut in cuts {
            let mut part = section.clone();
            if from != range.start {
                part.leading_space = 0.0;
            }
            part.byte_range = from..cut;
            if let Some((_, extra)) = lifts
                .iter()
                .find(|(row, _)| from < cut && row.start <= from && cut <= row.end)
            {
                let line = part.format.line_height.unwrap_or_else(|| {
                    ui.fonts_mut(|fonts| fonts.row_height(&part.format.font_id))
                });
                part.format.line_height = Some(line + extra);
            }
            sections.push(part);
            from = cut;
        }
    }
    job.sections = sections;
}

/// The document the reader being drawn shows, if it shows one. A page with a document sets
/// `reader_document` just before its reader. The first reader to read it takes it, with the pass
/// and the panel it was taken in, so the rest of that page's reader shares it. A reader in
/// another panel or a later pass, with no new value, has no document: it never takes the one
/// that another page left behind (Sessions notes and Lookup's written-note card have none).
fn reader_document(ui: &egui::Ui) -> Option<uuid::Uuid> {
    #[derive(Clone, Copy)]
    struct Taken {
        pass: u64,
        panel: egui::Id,
        document: uuid::Uuid,
    }
    let pass = ui.ctx().cumulative_pass_nr();
    let panel = ui
        .stack()
        .iter()
        .find(|frame| frame.is_panel_ui() || frame.is_area_ui())
        .map_or(egui::Id::NULL, |frame| frame.id);
    let taken_id = egui::Id::new("markdown_reader_document_taken");
    ui.ctx().data_mut(|data| {
        if let Some(document) = data.remove_temp::<uuid::Uuid>(egui::Id::new("reader_document")) {
            data.insert_temp(
                taken_id,
                Taken {
                    pass,
                    panel,
                    document,
                },
            );
            return Some(document);
        }
        data.get_temp::<Taken>(taken_id)
            .filter(|taken| taken.pass == pass && taken.panel == panel)
            .map(|taken| taken.document)
    })
}

/// One row's part of a link's words, on the screen.
#[derive(Clone, Copy, Debug)]
struct LinkPart {
    left: f32,
    right: f32,
    /// The words' baseline area, where a missing link's dots go.
    y: f32,
    /// The whole row's height across the words.
    rect: egui::Rect,
    /// The words are the first on their row, at the text edge.
    row_start: bool,
    /// Where the words' entry starts on their row: the first glyph after the last breaking blank
    /// before them. A no-break space does not end an entry.
    entry_left: Option<f32>,
}

/// Where each link's words are drawn, one part for each row it is on.
fn link_words(
    galley: &egui::Galley,
    position: egui::Pos2,
    links: &[(std::ops::Range<usize>, LinkTarget)],
) -> Vec<Vec<LinkPart>> {
    links
        .iter()
        .map(|(range, _)| {
            let mut parts = Vec::new();
            let mut byte_at = 0;
            for row in &galley.rows {
                let mut run: Option<(f32, f32, f32)> = None;
                let mut row_start = false;
                let mut entry_start = None;
                let mut entry_left = None;
                for (at, glyph) in row.glyphs.iter().enumerate() {
                    let x = position.x + row.pos.x + glyph.pos.x;
                    if range.contains(&byte_at) {
                        row_start |= at == 0;
                        let y = position.y + row.pos.y + glyph.pos.y;
                        if run.is_none() {
                            entry_left = entry_start;
                        }
                        let start = run.map_or(x, |(start, _, _)| start);
                        run = Some((start, x + glyph.advance_width, y));
                    } else if glyph.chr.is_whitespace() && glyph.chr != '\u{a0}' {
                        entry_start = None;
                    } else if entry_start.is_none() {
                        entry_start = Some(x);
                    }
                    byte_at += glyph.chr.len_utf8();
                }
                if let Some((left, right, y)) = run {
                    let row_rect = row.rect().translate(position.to_vec2());
                    parts.push(LinkPart {
                        left,
                        right,
                        y,
                        rect: egui::Rect::from_x_y_ranges(left..=right, row_rect.y_range()),
                        row_start,
                        entry_left,
                    });
                }
                if row.ends_with_newline {
                    byte_at += 1;
                }
            }
            parts
        })
        .collect()
}

/// Each link part's press target: at least `XL` square where the paragraph has room. Inside a
/// line it is centred on the words, as `reserve_short_links` centres them in their space; at the
/// start of a row, and for the last part of a link whose `spare` width is after its words, it
/// grows right from the words. For the first part of a link that ends an entry, it grows left
/// from the words over that entry, and no further than the entry's start. Two targets never share
/// a point: where grown targets meet, they split halfway between the words. The paragraph's own
/// layout stays.
fn link_targets(
    words: &[Vec<LinkPart>],
    spare: &[Spare],
    bounds: egui::Rect,
) -> Vec<Vec<egui::Rect>> {
    let grow = |part: &LinkPart, spare: Spare| {
        let words = part.rect;
        let width = words.width().max(design::XL);
        let left = match (spare, part.entry_left) {
            _ if part.row_start => words.left(),
            (Spare::After, _) => words.left(),
            (Spare::Entry, Some(entry)) => (words.right() - width).max(entry),
            _ => (words.center().x - width / 2.0).max(bounds.left()),
        };
        let mut x = left..=left + width;
        if *x.end() > bounds.right() {
            let left = (bounds.right() - width).max(bounds.left());
            x = left..=bounds.right().max(words.right());
        }
        let height = words.height().max(design::XL);
        let mut top = words.center().y - height / 2.0;
        top = top.min(bounds.bottom() - height).max(bounds.top());
        // Inside the paragraph, so a target never reaches past its own text.
        egui::Rect::from_x_y_ranges(x, top..=top + height).intersect(bounds)
    };
    let mut targets: Vec<Vec<egui::Rect>> = words
        .iter()
        .enumerate()
        .map(|(link, parts)| {
            let spare = spare.get(link).copied().unwrap_or_default();
            parts
                .iter()
                .enumerate()
                .map(|(at, part)| {
                    let own = match spare {
                        Spare::After if at + 1 < parts.len() => Spare::Split,
                        Spare::Entry if at > 0 => Spare::Split,
                        spare => spare,
                    };
                    grow(part, own)
                })
                .collect()
        })
        .collect();
    let all: Vec<(usize, usize)> = words
        .iter()
        .enumerate()
        .flat_map(|(link, parts)| (0..parts.len()).map(move |part| (link, part)))
        .collect();
    for (at, &(a_link, a_part)) in all.iter().enumerate() {
        for &(b_link, b_part) in &all[at + 1..] {
            let shared = targets[a_link][a_part].intersect(targets[b_link][b_part]);
            if shared.width() <= 0.0 || shared.height() <= 0.0 {
                continue;
            }
            let (a_words, b_words) = (words[a_link][a_part].rect, words[b_link][b_part].rect);
            let same_row = a_words.y_range().intersection(b_words.y_range()).span() > 0.0;
            let (first, second, split) = if same_row {
                let (left, right) = if a_words.center().x <= b_words.center().x {
                    ((a_link, a_part), (b_link, b_part))
                } else {
                    ((b_link, b_part), (a_link, a_part))
                };
                let mid = (words[left.0][left.1].rect.right()
                    + words[right.0][right.1].rect.left())
                    / 2.0;
                (left, right, (mid, true))
            } else {
                let (upper, lower) = if a_words.center().y <= b_words.center().y {
                    ((a_link, a_part), (b_link, b_part))
                } else {
                    ((b_link, b_part), (a_link, a_part))
                };
                let mid = (words[upper.0][upper.1].rect.bottom()
                    + words[lower.0][lower.1].rect.top())
                    / 2.0;
                (upper, lower, (mid, false))
            };
            let (mid, sideways) = split;
            if sideways {
                targets[first.0][first.1].max.x = targets[first.0][first.1].max.x.min(mid);
                targets[second.0][second.1].min.x = targets[second.0][second.1].min.x.max(mid);
            } else {
                targets[first.0][first.1].max.y = targets[first.0][first.1].max.y.min(mid);
                targets[second.0][second.1].min.y = targets[second.0][second.1].min.y.max(mid);
            }
        }
    }
    targets
}

/// A link's accessible name: its words, then where it goes.
fn link_name(words: &str, destination: &LinkTarget) -> String {
    match destination {
        LinkTarget::Missing => format!("{words} (not found in this campaign)"),
        LinkTarget::Note(note) => format!("{words} (opens {note})"),
        LinkTarget::Lookup(_) => format!("{words} (opens in Lookup)"),
        LinkTarget::Url(url) => format!("{words} (opens {url})"),
        LinkTarget::Roll(link) => crate::rolls::hint(link),
    }
}

/// Show that the pointer is on a link that can be pressed. A pressed web link opens here.
/// Another pressed link goes back to the caller, which knows where it opens.
pub(crate) fn press<'a>(
    ui: &egui::Ui,
    response: &egui::Response,
    destination: &'a LinkTarget,
) -> Option<&'a LinkTarget> {
    if matches!(destination, LinkTarget::Missing) {
        response.clone().on_hover_text("Not found in this campaign");
        return None;
    }
    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    if let LinkTarget::Url(url) = destination {
        response.clone().on_hover_text(url);
    }
    // A linked world page says what it is before the DM opens it.
    if let LinkTarget::Note(note) = destination
        && response.hovered()
        && let Some(index) = ui.ctx().data(|data| {
            data.get_temp::<Arc<crate::lookup::ContentIndex>>(egui::Id::new("reader_link_index"))
        })
    {
        let from = reader_document(ui);
        if let crate::world::Go::Item(at, _) = index.resolve_link(note, from)
            && !index.items[at].summary().is_empty()
        {
            response.clone().on_hover_text(index.items[at].summary());
        }
    }
    if !response.clicked() {
        return None;
    }
    if let LinkTarget::Url(url) = destination {
        ui.ctx().open_url(egui::OpenUrl::new_tab(url));
        return None;
    }
    Some(destination)
}

/// Give a pressed note link to whichever page takes it next. `from` is the note it is in.
pub fn send_note_link(ctx: &egui::Context, note: String, from: Option<uuid::Uuid>) {
    ctx.data_mut(|data| {
        data.insert_temp(egui::Id::new("clicked_reader_document"), from);
        data.insert_temp(clicked_id(), note);
    });
}

/// A layout job cut at byte `cut` (a section boundary) into its head and its rest.
fn split_job(job: LayoutJob, cut: usize) -> (LayoutJob, LayoutJob) {
    let part = |range: std::ops::Range<usize>| LayoutJob {
        text: job.text[range.clone()].to_owned(),
        sections: job
            .sections
            .iter()
            .filter(|section| {
                section.byte_range.start < range.end && section.byte_range.end > range.start
            })
            .map(|section| egui::text::LayoutSection {
                byte_range: section.byte_range.start.max(range.start) - range.start
                    ..section.byte_range.end.min(range.end) - range.start,
                ..section.clone()
            })
            .collect(),
        wrap: job.wrap.clone(),
        ..job.clone()
    };
    (part(0..cut), part(cut..job.text.len()))
}
