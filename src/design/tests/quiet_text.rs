//! Layout PR 9b (rulings L8 and M25): the text cost of a quiet frame, measured.
//!
//! A document editor (`pane::document_view` over a `TextDraft`) may cost, per quiet frame,
//! what egui's own text edit costs over the same text, and nothing else that grows with the
//! text. egui's stock costs stay (M25) and are COUNTED here: its `prev_text` copy, its undo
//! copies while focused, its AccessKit text runs while AccessKit is on. The allowance is
//! measured, not modelled: the reference is egui's `TextEdit` over a `String`, in the same
//! `Context` setup, laid out at the document's own wrap width, with a layouter that answers a
//! galley it made once (so the reference has no layout cost of its own).
//!
//! What is MEASURED, per case (focus, AccessKit, four kinds of text: UTF-8 widths, word
//! lengths, empty lines, one long wrapped line):
//! - allocation COUNTS: the document's growth from a small text to a large one equals the
//!   reference's growth exactly (a per-row, per-word or per-line allocation shows here);
//! - gross BYTES (an allocation at its size, a reallocation at its new size): the growth is
//!   equal exactly too (an extra copy, which is one allocation at any size, shows here);
//! - warmed quiet frames are equal to each other, also across egui's undo deadline (one
//!   second; each frame is a quarter second later);
//! - every counted allocation was the measuring thread's.
//!
//! What is COUNTED by the parts, not by the allocator: the bytes the field guard hashed
//! (`layout::BOUND_TEXT_BYTES`) and the layouts the kept layouter made
//! (`pane::KEPT_LAYOUTS_MADE`). Both must stay still on quiet frames. A scan or a compare that
//! allocates nothing is not measured anywhere else: the guard of a `TextDraft` compares keys.
//!
//! Its own process (`harness = false`): a global allocator counts every thread of the process.
use splat_design::{layout, pane, text_draft::TextDraft};
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

struct Counting;

static MEASURING: AtomicBool = AtomicBool::new(false);
static ALL_COUNT: AtomicUsize = AtomicUsize::new(0);
static ALL_BYTES: AtomicUsize = AtomicUsize::new(0);

thread_local! {
    static MINE: Cell<(usize, usize)> = const { Cell::new((0, 0)) };
}

fn note(bytes: usize) {
    if MEASURING.load(Ordering::Relaxed) {
        ALL_COUNT.fetch_add(1, Ordering::Relaxed);
        ALL_BYTES.fetch_add(bytes, Ordering::Relaxed);
        let _ = MINE.try_with(|mine| {
            let (count, total) = mine.get();
            mine.set((count + 1, total + bytes));
        });
    }
}

// SAFETY: each call goes to the system allocator unchanged; the counting touches only
// atomics and a const-initialized thread-local, which never allocate.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        note(layout.size());
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        note(layout.size());
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        note(new_size);
        unsafe { System.realloc(ptr, layout, new_size) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static COUNTING: Counting = Counting;

/// What one measured frame allocated.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Allocated {
    count: usize,
    bytes: usize,
}

fn measured(run: impl FnOnce()) -> Allocated {
    MINE.with(|mine| mine.set((0, 0)));
    ALL_COUNT.store(0, Ordering::Relaxed);
    ALL_BYTES.store(0, Ordering::Relaxed);
    MEASURING.store(true, Ordering::Relaxed);
    run();
    MEASURING.store(false, Ordering::Relaxed);
    let (count, bytes) = MINE.with(Cell::get);
    let mine = Allocated { count, bytes };
    let all = Allocated {
        count: ALL_COUNT.load(Ordering::Relaxed),
        bytes: ALL_BYTES.load(Ordering::Relaxed),
    };
    assert_eq!(mine, all, "every counted allocation was the measuring thread's");
    mine
}

/// Which editor a case draws.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Side {
    /// `pane::document_view` over a `TextDraft`: the part under test.
    Document,
    /// egui's `TextEdit` over a `String` at this wrap width, with a galley made once.
    Stock { wrap: f32 },
}

#[derive(Clone, Copy, Debug)]
struct Case {
    focused: bool,
    accesskit: bool,
}

const FIELD: layout::FieldId = splat_design::field_id!("quiet_text_harness").once(layout::Once::Example);

struct Editor {
    ctx: egui::Context,
    side: Side,
    draft: TextDraft,
    plain: String,
    galley: Option<Arc<egui::Galley>>,
    time: f64,
}

impl Editor {
    fn new(side: Side, text: &str, case: Case) -> Self {
        let ctx = egui::Context::default();
        splat_design::design::theme::install(&ctx);
        if !case.accesskit {
            ctx.disable_accesskit();
        }
        let mut editor = Self {
            ctx,
            side,
            draft: TextDraft::new(text),
            plain: text.to_owned(),
            galley: None,
            time: 0.0,
        };
        // Warm frames: fonts, the atlas, the layout, focus.
        for frame in 0..4 {
            let _ = editor.frame();
            if frame == 0 && case.focused {
                editor.ctx.memory_mut(|memory| memory.request_focus(FIELD.id()));
            }
        }
        assert_eq!(
            editor.ctx.memory(|memory| memory.has_focus(FIELD.id())),
            case.focused,
            "{side:?} {case:?}: the field has focus as the case says"
        );
        editor
    }

    /// One frame without input, a quarter second after the last.
    fn frame(&mut self) -> (egui::FullOutput, Option<Arc<egui::Galley>>) {
        self.time += 0.25;
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(800.0, 600.0),
            )),
            time: Some(self.time),
            ..Default::default()
        };
        let ctx = self.ctx.clone();
        let mut shown = None;
        let output = ctx.run(input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| shown = self.draw(ui));
        });
        (output, shown)
    }

    fn draw(&mut self, ui: &mut egui::Ui) -> Option<Arc<egui::Galley>> {
        match self.side {
            Side::Document => {
                let editor = pane::document_view(
                    ui,
                    pane::ViewHeight::Fills,
                    FIELD,
                    &mut self.draft,
                    pane::Face::Font(egui::TextStyle::Body),
                    |edit| edit,
                );
                Some(editor.galley)
            }
            Side::Stock { wrap } => {
                let plain = &self.plain;
                let galley = self
                    .galley
                    .get_or_insert_with(|| {
                        let job = egui::text::LayoutJob::simple(
                            plain.clone(),
                            egui::TextStyle::Body.resolve(ui.style()),
                            ui.visuals().widgets.inactive.text_color(),
                            wrap,
                        );
                        ui.fonts_mut(|fonts| fonts.layout_job(job))
                    })
                    .clone();
                let mut layouter =
                    move |_: &egui::Ui, _: &dyn egui::TextBuffer, _: f32| galley.clone();
                #[allow(clippy::disallowed_methods, reason = "the stock reference is egui's own text edit")]
                let _ = egui::TextEdit::multiline(&mut self.plain)
                    .id(FIELD.id())
                    .layouter(&mut layouter)
                    .frame(false)
                    .margin(egui::Vec2::ZERO)
                    .desired_width(wrap)
                    .show(ui);
                None
            }
        }
    }

    /// One quiet frame, measured; its output is dropped after the count stops.
    fn quiet(&mut self) -> Allocated {
        let mut shown = None;
        let allocated = measured(|| shown = Some(self.frame()));
        drop(shown);
        allocated
    }
}

/// About `bytes` bytes of `pattern`, repeated.
fn text(bytes: usize, pattern: &str) -> String {
    pattern.repeat(bytes.div_ceil(pattern.len()))
}

/// Four kinds of text: AccessKit's work for a text edit depends on its rows, its words and its
/// characters' UTF-8 widths.
const PATTERNS: [(&str, &str); 4] = [
    ("prose lines", "Ash road and cinder pass.\n"),
    ("wide UTF-8, empty lines", "かきくけこ — Ünïcödé wörds ⅔\n\n"),
    ("short words", "a b c d e f g h i j k l m n o p\n"),
    (
        "one wrapped line",
        "Onelongwordwithnospacesthatrunsonandonacrossthewholewidthofthefield ",
    ),
];

/// A side's quiet frame over `text`, after the warm frames; its five quiet frames are equal,
/// and the parts' own text counters stay still over them.
fn quiet_frame(side: Side, text: &str, case: Case) -> Allocated {
    let mut editor = Editor::new(side, text, case);
    let hashed = layout::BOUND_TEXT_BYTES.with(Cell::get);
    let laid = pane::KEPT_LAYOUTS_MADE.with(Cell::get);
    let first = editor.quiet();
    for _ in 0..4 {
        assert_eq!(editor.quiet(), first, "{side:?} {case:?}: the quiet frames are equal");
    }
    assert_eq!(
        layout::BOUND_TEXT_BYTES.with(Cell::get),
        hashed,
        "{side:?} {case:?}: the field guard hashed no text"
    );
    assert_eq!(
        pane::KEPT_LAYOUTS_MADE.with(Cell::get),
        laid,
        "{side:?} {case:?}: the document laid out no text"
    );
    first
}

/// The wrap width and the row count of the document's galley over `text`.
fn document_galley(text: &str) -> (f32, usize) {
    let mut editor = Editor::new(Side::Document, text, Case { focused: false, accesskit: false });
    let galley = editor.frame().1.expect("the document shows its galley");
    (galley.job.wrap.max_width, galley.rows.len())
}

/// The quiet frame of the document and of the stock reference over `text`.
fn both(text: &str, case: Case) -> (Allocated, Allocated) {
    let (wrap, rows) = document_galley(text);
    let mut stock = Editor::new(Side::Stock { wrap }, text, case);
    let stock_rows = stock.galley.as_ref().map(|galley| galley.rows.len());
    assert_eq!(stock_rows, Some(rows), "the reference has the document's rows");
    let _ = stock.quiet();
    (quiet_frame(Side::Document, text, case), quiet_frame(Side::Stock { wrap }, text, case))
}

fn main() {
    let sizes = [10_000, 100_000, 1_000_000];
    let mut failures = Vec::new();
    for (kind, pattern) in PATTERNS {
        for focused in [false, true] {
            for accesskit in [false, true] {
                let case = Case { focused, accesskit };
                let frames: Vec<_> =
                    sizes.iter().map(|&bytes| both(&text(bytes, pattern), case)).collect();
                let (doc_small, stock_small) = frames[0];
                for (&bytes, &(doc, stock)) in sizes.iter().zip(&frames).skip(1) {
                    let count = |a: Allocated, b: Allocated| a.count as i64 - b.count as i64;
                    let gross = |a: Allocated, b: Allocated| a.bytes as i64 - b.bytes as i64;
                    let doc_count = count(doc, doc_small);
                    let stock_count = count(stock, stock_small);
                    let doc_bytes = gross(doc, doc_small);
                    let stock_bytes = gross(stock, stock_small);
                    println!(
                        "{kind:<24} focused {focused:<5} accesskit {accesskit:<5} {:>7} B: \
                         document +{doc_count} allocations +{doc_bytes} B, stock +{stock_count} +{stock_bytes} B \
                         (document {doc:?}, stock {stock:?})",
                        bytes
                    );
                    if doc_count != stock_count {
                        failures.push(format!(
                            "{kind}, {case:?}, {bytes} B: the document's allocation count grows by {doc_count}, stock egui's by {stock_count}"
                        ));
                    }
                    if doc_bytes != stock_bytes {
                        failures.push(format!(
                            "{kind}, {case:?}, {bytes} B: the document's gross bytes grow by {doc_bytes}, stock egui's by {stock_bytes}"
                        ));
                    }
                }
            }
        }
    }
    assert!(failures.is_empty(), "quiet_text:\n{}", failures.join("\n"));
    println!("quiet_text: ok");
}
