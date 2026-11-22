//! Layout PR 9b-R1a (rulings L8 and M25; note 9b §10.6; lead brief Sol 2-004): the Reader's
//! text cost on a quiet frame, counted for the WHOLE host.
//!
//! Each row draws a real reader host, `world::Reader::read_with` (World's reader, and Lookup's
//! compact one), over a campaign note on disk, through `ContentIndex::load`, as the app does:
//! the link index is in the context, the reader draws the note's title, kind line, values and
//! body. Each measured frame first proves that the host drew the note's words. The count runs
//! from the host's entry through paint (`Context::run`, its end of pass included), so a copy or
//! a rebuild of the text anywhere in the host shows, not only one in the shared Reader part.
//!
//! What is MEASURED, per host, with AccessKit off and on, over notes of about 10 KB, 100 KB and
//! 1 MB with the same blocks and links (only the paragraphs grow):
//! - allocation COUNTS: the host's growth from the small note to a large one equals the stock
//!   reference's growth exactly;
//! - gross BYTES (an allocation at its size, a reallocation at its new size): the growth is
//!   equal within 4 KiB (egui rounds some buffers it grows; a copy of a 90 KB or 990 KB text is
//!   far more);
//! - warmed quiet frames are equal to each other;
//! - every counted allocation was the measuring thread's.
//!
//! The stock reference: egui's own label (`Label::ui`) over each of the paragraphs' kept
//! galleys, in the same context setup. With AccessKit on, that is egui's text cost per label
//! (its accessible name and its AccessKit text runs: M25, counted, not removed). With AccessKit
//! off it costs nothing that grows with the text, so neither may the host.
//!
//! What is COUNTED by the part: the Reader's Markdown parses (`markdown_view::MARKDOWN_PARSES`).
//! None on a quiet frame. Each eviction case (a resize, the theme installed again, the font
//! atlas made again, the link index loaded again, a new art index) prepares again exactly as
//! often as its dependency changed, then is quiet again.
//!
//! Layout PR 9b-R2: what a quiet frame lays out does not grow with the note. Per host, with one
//! screen in view (800 px), over notes of 40, 400 and 4,000 blocks (headings, paragraphs and a
//! link every fourth block): the blocks laid out per frame (`markdown_view::BLOCKS_LAID`) are
//! equal at every size, and so are the allocation counts and gross bytes. A block out of view is
//! not laid out, and its links stay controls (AccessKit off: with it on, each control's node is
//! egui's own cost per control, as it is for a label).
//!
//! Its own process (`harness = false`): a global allocator counts every thread of the process.
#[path = "../../design/tests/quiet/counting.rs"]
#[allow(
    dead_code,
    reason = "the shared module’s own bare screen and text edit are the editor harnesses’"
)]
mod counting;

use counting::{Allocated, measured, root_sized, stock_label};
use eframe::egui;
use splat_content_ui::{
    lookup::ContentIndex,
    markdown_view::{self, BLOCKS_LAID, MARKDOWN_PARSES},
    persistence::Campaign,
    world::{self, Reader},
};
use std::sync::Arc;

/// Which reader host a row draws.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Host {
    /// World's reader.
    World,
    /// Lookup's reader of the same entry (`Reader::compact`).
    Lookup,
}

/// Every row sees the whole note: egui reads out a label's words only where the label can be
/// seen, so the host and the stock labels must see the same paragraphs.
const SCREEN_HEIGHT: f32 = 1_000_000.0;

/// The linked rows' view: the host's title and facts, and a part of the first paragraph.
const LINKED_HEIGHT: f32 = 300.0;

/// The first words of the note’s body: the frame proves the host drew them.
const OPENING: &str = "Opening words of the crossing";

/// A note body of about `bytes` bytes, with the same blocks and links at every size: a heading,
/// two paragraphs with a link each, a list of two items, a quote and a plain paragraph. Only
/// the paragraphs' words grow.
fn note_body(bytes: usize, linked: bool) -> String {
    let grown = |share: usize| "cinder ash road pass ".repeat((bytes / 6 / 21).max(1) * share / 6);
    // The linked variant grows the links' own words (PR 813 review, P2): each paragraph's words
    // are one link, wrapped over many rows.
    let words = |share: usize| {
        if linked {
            format!("[{}](https://example.com/words)", grown(share))
        } else {
            grown(share)
        }
    };
    format!(
        "{OPENING} {} by [the ford](https://example.com/ford).\n\n\
         ## The ford\n\n\
         The ferry hands wait {} near [the pier](https://example.com/pier) tonight.\n\n\
         - {}\n- The second item {}\n\n\
         > {}\n\n\
         The last words {}\n",
        words(6),
        words(6),
        words(6),
        words(6),
        words(6),
        words(6),
    )
}

/// A note body of `blocks` blocks after the opening words: sections of a heading and three
/// paragraphs, one of them with a link.
fn blocks_body(blocks: usize) -> String {
    let mut body = format!("{OPENING} at the start of the watch.\n\n");
    for part in 0..blocks / 4 {
        body.push_str(&format!(
            "## Part {part}\n\nThe watch of part {part} keeps the road.\n\n\
             Ask [the warden](https://example.com/warden/{part}) of part {part}.\n\n\
             The last words of part {part}.\n\n"
        ));
    }
    body
}

/// A campaign with one note on disk, its loaded index, and a reader host drawing it.
struct Reading {
    ctx: egui::Context,
    reader: Reader,
    index: Arc<ContentIndex>,
    at: usize,
    campaign: Campaign,
    time: f64,
    width: f32,
    height: f32,
    _dir: tempfile::TempDir,
}

impl Reading {
    fn new(host: Host, bytes: usize, accesskit: bool) -> Self {
        Self::with(host, bytes, accesskit, false, SCREEN_HEIGHT)
    }

    fn with(host: Host, bytes: usize, accesskit: bool, linked: bool, height: f32) -> Self {
        Self::with_body(host, &note_body(bytes, linked), accesskit, height)
    }

    fn with_body(host: Host, body: &str, accesskit: bool, height: f32) -> Self {
        let dir = tempfile::tempdir().expect("a folder for the campaign");
        let campaign = Campaign {
            id: uuid::Uuid::new_v4(),
            name: "Sample".into(),
            path: dir.path().into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        // A plain note: World's and Lookup's readers draw its body as one document. (A typed
        // entry draws through the shared entry view, which is not moved yet: note 9b §10.6.)
        let id = world::create_entry(&campaign, "Watch notes", "").expect("a note");
        let first = ContentIndex::load(&[], Some(&campaign)).expect("the index");
        let found = &first.items[first.position(id).expect("the note in the index")];
        assert!(found.is_note(), "the fixture is a plain note");
        let path = dir.path().join(&found.entry.path);
        let mut text = std::fs::read_to_string(&path).expect("the note’s file");
        text.push_str(body);
        std::fs::write(&path, text).expect("the note’s body");
        let index = Arc::new(ContentIndex::load(&[], Some(&campaign)).expect("the index"));
        let at = index.position(id).expect("the note in the index");
        let ctx = egui::Context::default();
        splat_design::design::theme::install(&ctx);
        if !accesskit {
            ctx.disable_accesskit();
        }
        let mut reader = Reader::default();
        reader.compact = host == Host::Lookup;
        let mut reading = Self {
            ctx,
            reader,
            index,
            at,
            campaign,
            time: 0.0,
            width: 900.0,
            height,
            _dir: dir,
        };
        // Warm frames: fonts, the atlas, the preparation.
        for _ in 0..4 {
            let _ = reading.frame();
        }
        reading
    }

    fn input(&mut self) -> egui::RawInput {
        self.time += 0.25;
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(self.width, self.height),
            )),
            time: Some(self.time),
            ..Default::default()
        }
    }

    /// One frame of the host, as the app draws it: the link index in the context, the reader
    /// over the note.
    fn frame(&mut self) -> egui::FullOutput {
        let input = self.input();
        self.frame_with(input)
    }

    /// Tab until the note's long link has the keyboard (egui says so in its output, AccessKit
    /// on or off), then warm frames.
    fn focus_a_link(&mut self) {
        let tab = |pressed| egui::Event::Key {
            key: egui::Key::Tab,
            physical_key: None,
            pressed,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        };
        for _ in 0..60 {
            let mut input = self.input();
            input.events = vec![tab(true), tab(false)];
            let output = self.frame_with(input);
            // The note's long link, not one of the host's own links before the body.
            let link = output.platform_output.events.iter().any(|event| {
                matches!(event, egui::output::OutputEvent::FocusGained(info)
                    if info.typ == egui::WidgetType::Link
                        && info.label.as_deref().is_some_and(|name| name.contains("cinder")))
            });
            if link {
                for _ in 0..4 {
                    let _ = self.frame();
                }
                return;
            }
        }
        panic!("Tab reached no link of the note");
    }

    fn frame_with(&mut self, input: egui::RawInput) -> egui::FullOutput {
        let ctx = self.ctx.clone();
        ctx.run(input, |ctx| {
            ctx.data_mut(|data| {
                data.insert_temp(egui::Id::new("reader_link_index"), self.index.clone());
            });
            let mut ui = root_sized(ctx, egui::vec2(self.width, self.height));
            let _ = self.reader.read_with(
                &mut ui,
                &self.index.items[self.at],
                &self.index,
                Some(&self.campaign),
                None,
            );
        })
    }

    /// One quiet frame, measured, after the host proved it drew the note's words in one layout
    /// pass (a discarded pass would draw the host twice).
    fn quiet(&mut self, what: &str) -> Allocated {
        let mut shown = None;
        let passes = self.ctx.cumulative_pass_nr();
        let allocated = measured(|| shown = Some(self.frame()));
        let output = shown.expect("the frame ran");
        assert_eq!(
            self.ctx.cumulative_pass_nr() - passes,
            1,
            "{what}: a quiet frame runs one layout pass; discarded for {:?}",
            output.platform_output.request_discard_reasons
        );
        assert!(
            painted(&output).iter().any(|text| text.contains(OPENING)),
            "{what}: the real reader host drew the note’s words in the measured frame"
        );
        drop(output);
        allocated
    }
}

/// The textured shapes (images) a frame painted.
fn images(output: &egui::FullOutput) -> usize {
    fn walk(shape: &egui::Shape) -> usize {
        match shape {
            egui::Shape::Vec(shapes) => shapes.iter().map(walk).sum(),
            egui::Shape::Mesh(mesh) => usize::from(mesh.texture_id != egui::TextureId::default()),
            egui::Shape::Rect(rect) => usize::from(rect.brush.is_some()),
            _ => 0,
        }
    }
    output
        .shapes
        .iter()
        .map(|clipped| walk(&clipped.shape))
        .sum()
}

/// The texts a frame painted.
fn painted(output: &egui::FullOutput) -> Vec<String> {
    fn walk(shape: &egui::Shape, out: &mut Vec<String>) {
        match shape {
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, out)),
            egui::Shape::Text(text) => out.push(text.galley.job.text.clone()),
            _ => {}
        }
    }
    let mut out = Vec::new();
    for clipped in &output.shapes {
        walk(&clipped.shape, &mut out);
    }
    out
}

/// The stock reference: egui's own label over each galley, in a context set up as the host's.
fn stock_labels(galleys: &[Arc<egui::Galley>], accesskit: bool) -> Allocated {
    let ctx = egui::Context::default();
    splat_design::design::theme::install(&ctx);
    if !accesskit {
        ctx.disable_accesskit();
    }
    let mut time = 0.0;
    let mut frame = |ctx: &egui::Context| {
        time += 0.25;
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(900.0, SCREEN_HEIGHT),
            )),
            time: Some(time),
            ..Default::default()
        };
        ctx.run(input, |ctx| {
            let mut ui = root_sized(ctx, egui::vec2(900.0, SCREEN_HEIGHT));
            for galley in galleys {
                stock_label(&mut ui, galley.clone());
            }
        })
    };
    for _ in 0..4 {
        let _ = frame(&ctx);
    }
    let mut shown = None;
    let first = measured(|| shown = Some(frame(&ctx)));
    drop(shown);
    for _ in 0..4 {
        let mut shown = None;
        assert_eq!(
            measured(|| shown = Some(frame(&ctx))),
            first,
            "the stock reference’s quiet frames are equal"
        );
        drop(shown);
    }
    first
}

/// A host's quiet frame over a note of about `bytes` bytes, the stock reference over its kept
/// galleys, and the blocks the frame laid out; the host's five quiet frames are equal and parse
/// nothing.
fn both(
    host: Host,
    bytes: usize,
    accesskit: bool,
    linked: bool,
    focused: bool,
) -> (Allocated, Allocated, usize) {
    let what = format!(
        "{host:?}, AccessKit {accesskit}, linked {linked}, a link focused {focused}, {bytes} B"
    );
    // The linked rows see a part of the first paragraph only, at every size: a link out of view
    // costs the same whatever its words (its parts and targets are built only in view), and the
    // same blocks are laid out (layout PR 9b-R2: a block out of view is not).
    let height = if linked { LINKED_HEIGHT } else { SCREEN_HEIGHT };
    let mut reading = Reading::with(host, bytes, accesskit, linked, height);
    if focused {
        reading.focus_a_link();
    }
    let parses = MARKDOWN_PARSES.with(std::cell::Cell::get);
    let laid = BLOCKS_LAID.with(std::cell::Cell::get);
    let first = reading.quiet(&what);
    let laid = BLOCKS_LAID.with(std::cell::Cell::get) - laid;
    for _ in 0..4 {
        assert_eq!(
            reading.quiet(&what),
            first,
            "{what}: the quiet frames are equal"
        );
    }
    assert_eq!(
        MARKDOWN_PARSES.with(std::cell::Cell::get),
        parses,
        "{what}: the reader parsed nothing on its quiet frames"
    );
    let galleys = markdown_view::kept_galleys(&reading.ctx);
    assert!(
        galleys.len() >= 6,
        "{what}: the reader keeps its paragraphs laid out ({})",
        galleys.len()
    );
    (first, stock_labels(&galleys, accesskit), laid)
}

/// Layout PR 9b-R2: a host's quiet frame over a note of `blocks` blocks with one screen in view:
/// what it costs, and how many blocks it laid out. Its five quiet frames are equal.
fn in_view(host: Host, blocks: usize) -> (Allocated, usize) {
    let what = format!("{host:?}, {blocks} blocks in one screen");
    let mut reading = Reading::with_body(host, &blocks_body(blocks), false, 800.0);
    let laid = BLOCKS_LAID.with(std::cell::Cell::get);
    let first = reading.quiet(&what);
    let laid = BLOCKS_LAID.with(std::cell::Cell::get) - laid;
    for _ in 0..4 {
        let before = BLOCKS_LAID.with(std::cell::Cell::get);
        assert_eq!(
            reading.quiet(&what),
            first,
            "{what}: the quiet frames are equal"
        );
        assert_eq!(
            BLOCKS_LAID.with(std::cell::Cell::get) - before,
            laid,
            "{what}: the quiet frames lay out as many blocks"
        );
    }
    assert!(laid > 0, "{what}: the blocks in view are laid out");
    (first, laid)
}

/// After `change`, the reader prepares again exactly `expected` times, then is quiet again.
fn prepares_again(
    host: Host,
    case: &str,
    expected: usize,
    change: impl FnOnce(&mut Reading),
) -> Option<String> {
    let mut reading = Reading::new(host, 10_000, true);
    let before = MARKDOWN_PARSES.with(std::cell::Cell::get);
    change(&mut reading);
    for _ in 0..4 {
        let _ = reading.frame();
    }
    let made = MARKDOWN_PARSES.with(std::cell::Cell::get) - before;
    let quiet = MARKDOWN_PARSES.with(std::cell::Cell::get);
    let _ = reading.quiet(case);
    let again = MARKDOWN_PARSES.with(std::cell::Cell::get) - quiet;
    (made != expected || again != 0).then(|| {
        format!(
            "{host:?}, {case}: prepared {made} times (expected {expected}), then {again} on a \
             quiet frame"
        )
    })
}

/// A document drawn by the Reader directly, frame by frame: for the cache's content cases,
/// where the document, its revision, the link index and the art each change on their own.
struct Direct {
    ctx: egui::Context,
    time: f64,
    /// The textured shapes (images) the last frame painted.
    images: usize,
}

impl Direct {
    fn new() -> Self {
        let ctx = egui::Context::default();
        splat_design::design::theme::install(&ctx);
        // The app's image loaders: a figure's local copy loads and paints.
        egui_extras::install_image_loaders(&ctx);
        Self {
            ctx,
            time: 0.0,
            images: 0,
        }
    }

    /// One frame of `text` as document `doc` at `revision`, resolving its links in `index`:
    /// the words it painted.
    fn frame(
        &mut self,
        doc: &markdown_view::DocumentId,
        revision: u64,
        text: &str,
        index: Option<&Arc<ContentIndex>>,
    ) -> String {
        self.time += 0.25;
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(900.0, 800.0),
            )),
            time: Some(self.time),
            ..Default::default()
        };
        let output = self.ctx.clone().run(input, |ctx| {
            let key = egui::Id::new("reader_link_index");
            ctx.data_mut(|data| match index {
                Some(index) => data.insert_temp(key, index.clone()),
                None => data.remove::<Arc<ContentIndex>>(key),
            });
            let mut ui = root_sized(ctx, egui::vec2(900.0, 800.0));
            markdown_view::show_document(
                &mut ui,
                markdown_view::ReaderSource {
                    doc: doc.clone(),
                    revision,
                    text,
                },
            );
        });
        self.images = images(&output);
        painted(&output).join(" | ")
    }

    /// The next quiet frame reuses what the reader prepared: no parse, the same words, the same
    /// images (PR 861 review, P3).
    fn reused(
        &mut self,
        doc: &markdown_view::DocumentId,
        revision: u64,
        text: &str,
        index: Option<&Arc<ContentIndex>>,
        after: &str,
    ) -> bool {
        let (parses, images) = (MARKDOWN_PARSES.with(std::cell::Cell::get), self.images);
        let again = self.frame(doc, revision, text, index);
        MARKDOWN_PARSES.with(std::cell::Cell::get) == parses
            && again == after
            && self.images == images
    }
}

/// The cache's content cases (PR 813 review, P2): each changes one thing the preparation read,
/// and the next frame must show the new words. A key without that dependency keeps the old ones.
fn content_cases() -> Vec<String> {
    use markdown_view::DocumentId;
    let mut failures = Vec::new();
    let mut check = |case: &str, before: &str, after: &str, ok: bool| {
        if !ok {
            failures.push(format!(
                "{case}: before {before:?}, the next frame {after:?}"
            ));
        }
    };
    let (a, b) = (
        DocumentId::Entry(uuid::Uuid::from_u128(1)),
        DocumentId::Entry(uuid::Uuid::from_u128(2)),
    );
    // A same-length edit: a new revision of the same document.
    let mut reader = Direct::new();
    let mut before = String::new();
    for _ in 0..3 {
        before = reader.frame(&a, 1, "Ash road at dusk.", None);
    }
    let after = reader.frame(&a, 2, "Ash road at dawn.", None);
    let reused = reader.reused(&a, 2, "Ash road at dawn.", None, &after);
    check(
        "a same-length edit (only the revision), then reused",
        &before,
        &after,
        after.contains("dawn") && !after.contains("dusk") && reused,
    );
    // Another document with the same revision.
    let mut reader = Direct::new();
    for _ in 0..3 {
        before = reader.frame(&a, 1, "Words of Hob.", None);
    }
    let after = reader.frame(&b, 1, "Words of Wil.", None);
    let reused = reader.reused(&b, 1, "Words of Wil.", None, &after);
    check(
        "another document with the same revision (only the document), then reused",
        &before,
        &after,
        after.contains("Wil") && !after.contains("Hob") && reused,
    );
    // The link's target renamed, then deleted: only the link index's load changes.
    let warn = splat_design::design::icons::WARN;
    for change in ["renamed", "deleted"] {
        let dir = tempfile::tempdir().expect("a folder for the campaign");
        let campaign = Campaign {
            id: uuid::Uuid::new_v4(),
            name: "Sample".into(),
            path: dir.path().into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        let gate = world::create_entry(&campaign, "Gate", "location").expect("the target");
        let index = Arc::new(ContentIndex::load(&[], Some(&campaign)).expect("the index"));
        let path = dir.path().join(
            &index.items[index.position(gate).expect("the target in the index")]
                .entry
                .path,
        );
        let text = "The [[Gate]] stands.";
        let mut reader = Direct::new();
        for _ in 0..3 {
            before = reader.frame(&a, 1, text, Some(&index));
        }
        if change == "renamed" {
            // The entry's name is in its file: renamed there, and the file with it.
            let words = std::fs::read_to_string(&path).expect("the target’s file");
            std::fs::write(
                path.with_file_name("Gatehouse.md"),
                words.replace("Gate", "Gatehouse"),
            )
            .expect("renamed");
            std::fs::remove_file(&path).expect("the old file");
        } else {
            std::fs::remove_file(&path).expect("deleted");
        }
        let index = Arc::new(ContentIndex::load(&[], Some(&campaign)).expect("the index again"));
        let after = reader.frame(&a, 1, text, Some(&index));
        let reused = reader.reused(&a, 1, text, Some(&index), &after);
        check(
            &format!("the link’s target {change} (only the index’s load), then reused"),
            &before,
            &after,
            !before.contains(warn) && after.contains(warn) && reused,
        );
    }
    // The image: a new art index has a local copy of the linked image, which shows in place.
    let root = tempfile::tempdir().expect("a package folder");
    std::fs::create_dir(root.path().join("Figures")).expect("a figures folder");
    image::DynamicImage::new_rgb8(40, 20)
        .save(root.path().join("Figures/map.png"))
        .expect("an image");
    std::fs::write(
        root.path().join(splat_content_ui::content::art::ART_FILE),
        "```json\n{\"splat_art\":1,\"links\":{\"https://a/map.png\":\"Figures/map.png\"}}\n```\n",
    )
    .expect("the art file");
    let art = Arc::new(splat_content_ui::content::art::ArtIndex::load(
        [root.path()],
    ));
    let text = "*The keep* ([image](https://a/map.png))";
    let mut reader = Direct::new();
    for _ in 0..3 {
        before = reader.frame(&a, 1, text, None);
    }
    reader
        .ctx
        .data_mut(|data| data.insert_temp(egui::Id::new("art_index"), art));
    let shown_before = reader.images;
    let mut after = reader.frame(&a, 1, text, None);
    // The file loads on egui's loader thread: the figure paints once it has.
    for _ in 0..100 {
        if reader.images > 0 {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
        after = reader.frame(&a, 1, text, None);
    }
    let shown_after = reader.images;
    let reused = reader.reused(&a, 1, text, None, &after);
    check(
        &format!(
            "the image’s local copy (only the art index), painted ({shown_before} images, then \
             {shown_after}), then reused"
        ),
        &before,
        &after,
        before.contains("image")
            && !after.contains("image")
            && shown_before == 0
            && shown_after > 0
            && reused,
    );
    failures
}

fn main() {
    let sizes = [10_000, 100_000, 1_000_000];
    let mut failures = Vec::new();
    let rows = [
        (Host::World, false, false, false),
        (Host::World, true, false, false),
        (Host::Lookup, false, false, false),
        (Host::Lookup, true, false, false),
        // The links' own words grow (PR 813 review, P2), AccessKit off: what grows with them
        // is the app's, not egui's text runs.
        (Host::World, false, true, false),
        // The same, with a long link focused (PR 861 review, P2): its ring on its rows in view.
        (Host::World, false, true, true),
    ];
    for (host, accesskit, linked, focused) in rows {
        {
            let frames: Vec<_> = sizes
                .iter()
                .map(|&bytes| both(host, bytes, accesskit, linked, focused))
                .collect();
            let (host_small, stock_small, laid_small) = frames[0];
            for (&bytes, &(reader, stock, laid)) in sizes.iter().zip(&frames).skip(1) {
                if laid != laid_small {
                    failures.push(format!(
                        "{host:?}, AccessKit {accesskit}, linked {linked}, {bytes} B: {laid} blocks \
                         laid out against {laid_small} at the smallest size: the sizes see \
                         different views"
                    ));
                }
                let count = |a: Allocated, b: Allocated| a.count as i64 - b.count as i64;
                let gross = |a: Allocated, b: Allocated| a.bytes as i64 - b.bytes as i64;
                let (host_count, stock_count) =
                    (count(reader, host_small), count(stock, stock_small));
                let (host_bytes, stock_bytes) =
                    (gross(reader, host_small), gross(stock, stock_small));
                println!(
                    "{:<7} accesskit {accesskit:<5} {bytes:>8} B: host +{host_count} allocations \
                     +{host_bytes} B, stock +{stock_count} +{stock_bytes} B (host {reader:?}, \
                     stock {stock:?}, {laid} blocks laid out)",
                    format!("{host:?}")
                );
                if host_count != stock_count {
                    failures.push(format!(
                        "{host:?}, AccessKit {accesskit}, {bytes} B: the host's allocation count \
                         grows by {host_count}, egui's labels' by {stock_count}"
                    ));
                }
                if (host_bytes - stock_bytes).abs() > 4096 {
                    failures.push(format!(
                        "{host:?}, AccessKit {accesskit}, {bytes} B: the host's gross bytes grow \
                         by {host_bytes}, egui's labels' by {stock_bytes}"
                    ));
                }
            }
        }
    }
    for host in [Host::World, Host::Lookup] {
        let frames: Vec<_> = [40, 400, 4_000]
            .into_iter()
            .map(|blocks| (blocks, in_view(host, blocks)))
            .collect();
        let (_, (small, laid_small)) = frames[0];
        for &(blocks, (frame, laid)) in &frames {
            println!(
                "{:<7} {blocks:>5} blocks in one screen: {laid} laid out, {frame:?}",
                format!("{host:?}")
            );
            if laid != laid_small
                || frame.count != small.count
                || frame.bytes.abs_diff(small.bytes) > 4096
            {
                failures.push(format!(
                    "{host:?}, {blocks} blocks in one screen: {laid} blocks laid out and {frame:?}, \
                     against {laid_small} and {small:?} for {} blocks",
                    frames[0].0
                ));
            }
        }
        failures.extend(prepares_again(host, "a resize", 1, |reading| {
            // Below the reader's measure: the note's column gets narrower.
            reading.width = 480.0;
        }));
        failures.extend(prepares_again(
            host,
            "the theme installed again",
            1,
            |reading| {
                // The event under test: the theme installed again on the reader's own context, between its
                // frames (the fonts' generation rises). Counted in `lint_allowlist.txt` (`late-theme`).
                splat_design::design::theme::install(&reading.ctx);
            },
        ));
        failures.extend(prepares_again(
            host,
            "the font atlas made again",
            2,
            |reading| {
                // egui makes its font atlas again for a new text alpha mode, and again when it
                // comes back: the same fonts, style and size, in a new atlas.
                let alpha = reading.ctx.style().visuals.text_alpha_from_coverage;
                reading.ctx.all_styles_mut(|style| {
                    style.visuals.text_alpha_from_coverage =
                        egui::epaint::AlphaFromCoverage::Linear;
                });
                let _ = reading.frame();
                reading
                    .ctx
                    .all_styles_mut(|style| style.visuals.text_alpha_from_coverage = alpha);
            },
        ));
        failures.extend(prepares_again(
            host,
            "the link index loaded again",
            1,
            |reading| {
                reading.index = Arc::new(
                    ContentIndex::load(&[], Some(&reading.campaign)).expect("the index again"),
                );
            },
        ));
        failures.extend(prepares_again(host, "a new art index", 1, |reading| {
            reading.ctx.data_mut(|data| {
                data.insert_temp(
                    egui::Id::new("art_index"),
                    Arc::new(splat_content_ui::content::art::ArtIndex::default()),
                );
            });
        }));
        failures.extend(prepares_again(host, "a new scale", 1, |reading| {
            reading.ctx.set_pixels_per_point(1.5);
        }));
    }
    failures.extend(content_cases());
    assert!(
        failures.is_empty(),
        "quiet_reader:\n{}",
        failures.join("\n")
    );
    println!("quiet_reader: ok");
}
