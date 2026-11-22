//! Layout PR 9b-C: the text cost of a folded Markdown document's quiet frame, measured.
//!
//! The `Face::Laid` case of `splat-design`'s `quiet_text` (the plain `Face::Font` case lives
//! there). A Session note: `pane::editor_view` over a `TextDraft`, laid out by
//! `markdown_source::layouter` with its folds, then `markdown_source::paint_section_rules` (the
//! heading rules and the `⋯` of each folded `splat:` comment). The reference is egui's own text
//! edit over a `String`, at the document's wrap width, with the same Markdown galley made once
//! and no paint. The document's quiet frame must grow from a small note to a large one exactly
//! as the reference's does, in allocation COUNTS and gross BYTES: its rules and marks are kept
//! per laid-out text and only those in view are painted, so nothing that grows with the note is
//! scanned or allocated. The parts' own counters (`pane::text_work`) stay at zero too.
use eframe::egui;
use splat_content_ui::markdown_source;
use splat_design::{layout, pane, text_draft::TextDraft};
use std::sync::Arc;

#[path = "../../design/tests/quiet/counting.rs"]
mod counting;
use counting::{Allocated, measured, root, stock_text_edit};

const FIELD: layout::FieldId =
    splat_design::field_id!("quiet_markdown_harness").once(layout::Once::Example);

#[derive(Clone, Copy, Debug, PartialEq)]
enum Side {
    Document,
    Stock { wrap: f32 },
}

#[derive(Clone, Copy, Debug)]
struct Case {
    focused: bool,
    accesskit: bool,
}

struct Editor {
    ctx: egui::Context,
    side: Side,
    draft: TextDraft,
    plain: String,
    galley: Option<Arc<egui::Galley>>,
    time: f64,
}

/// The note's face, as a Session note's editor gives it.
fn face(ui: &egui::Ui) -> pane::Face {
    pane::Face::Laid {
        layout: markdown_source::layouter,
        font: splat_design::design::reader::prose_type(ui).font(ui),
        folds: true,
    }
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
        for frame in 0..4 {
            let _ = editor.frame();
            if frame == 0 && case.focused {
                editor
                    .ctx
                    .memory_mut(|memory| memory.request_focus(FIELD.id()));
            }
        }
        editor
    }

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
            shown = self.draw(&mut root(ctx));
        });
        (output, shown)
    }

    fn draw(&mut self, ui: &mut egui::Ui) -> Option<Arc<egui::Galley>> {
        match self.side {
            Side::Document => {
                let face = face(ui);
                let output = pane::editor_view(
                    ui,
                    pane::ViewHeight::Fills,
                    FIELD,
                    &mut self.draft,
                    face,
                    |edit| edit,
                );
                markdown_source::paint_section_rules(ui, &output, Some(self.draft.key()));
                Some(output.galley)
            }
            Side::Stock { wrap } => {
                let plain = &self.plain;
                let galley = self
                    .galley
                    .get_or_insert_with(|| markdown_source::layouter(ui, plain, wrap))
                    .clone();
                let mut layouter =
                    move |_: &egui::Ui, _: &dyn egui::TextBuffer, _: f32| galley.clone();
                stock_text_edit(ui, FIELD.id(), &mut self.plain, wrap, &mut layouter);
                None
            }
        }
    }

    fn quiet(&mut self) -> Allocated {
        let mut shown = None;
        let allocated = measured(|| shown = Some(self.frame()));
        drop(shown);
        allocated
    }
}

/// A folded note of about `bytes` bytes: sections with headings (their rules), and lines with a
/// `splat:` comment each (a fold with its `⋯`).
fn note(bytes: usize) -> String {
    let section = "## On the old road\n\n\
        - [[Wolves]] <!-- splat:check journey=t-road at=24 sum=12345678 --> Keep watch by the stones.\n\
        - Plain words of the note go on and on here, past the bend.\n\n";
    section.repeat(bytes.div_ceil(section.len()))
}

/// A side's quiet frame over `text`: five equal quiet frames, with no text work in the parts.
fn quiet_frame(side: Side, text: &str, case: Case) -> Allocated {
    let mut editor = Editor::new(side, text, case);
    let first = editor.quiet();
    let work = pane::text_work(|| {
        for _ in 0..4 {
            assert_eq!(
                editor.quiet(),
                first,
                "{side:?} {case:?}: the quiet frames are equal"
            );
        }
    });
    // The stock side draws no document editor; the document's editor drew in each frame.
    let drawn = if side == Side::Document { 4 } else { 0 };
    assert!(
        work.drawn >= drawn
            && work
                == pane::TextWork {
                    drawn: work.drawn,
                    ..Default::default()
                },
        "{side:?} {case:?}: no text work: {work:?}"
    );
    first
}

fn both(text: &str, case: Case) -> (Allocated, Allocated) {
    let mut probe = Editor::new(
        Side::Document,
        text,
        Case {
            focused: false,
            accesskit: false,
        },
    );
    let galley = probe.frame().1.expect("the document shows its galley");
    let (wrap, rows) = (galley.job.wrap.max_width, galley.rows.len());
    let stock = Editor::new(Side::Stock { wrap }, text, case);
    let stock_rows = stock.galley.as_ref().map(|galley| galley.rows.len());
    assert_eq!(
        stock_rows,
        Some(rows),
        "the reference has the document's rows"
    );
    (
        quiet_frame(Side::Document, text, case),
        quiet_frame(Side::Stock { wrap }, text, case),
    )
}

fn main() {
    let sizes = [10_000, 100_000, 1_000_000];
    let mut failures = Vec::new();
    for focused in [false, true] {
        for accesskit in [false, true] {
            let case = Case { focused, accesskit };
            let frames: Vec<_> = sizes
                .iter()
                .map(|&bytes| both(&note(bytes), case))
                .collect();
            let (doc_small, stock_small) = frames[0];
            for (&bytes, &(doc, stock)) in sizes.iter().zip(&frames).skip(1) {
                let count = |a: Allocated, b: Allocated| a.count as i64 - b.count as i64;
                let gross = |a: Allocated, b: Allocated| a.bytes as i64 - b.bytes as i64;
                let (doc_count, stock_count) = (count(doc, doc_small), count(stock, stock_small));
                let (doc_bytes, stock_bytes) = (gross(doc, doc_small), gross(stock, stock_small));
                println!(
                    "folded Markdown focused {focused:<5} accesskit {accesskit:<5} {bytes:>7} B: \
                     document +{doc_count} allocations +{doc_bytes} B, stock +{stock_count} +{stock_bytes} B"
                );
                if doc_count != stock_count {
                    failures.push(format!(
                        "{case:?}, {bytes} B: the document's allocation count grows by {doc_count}, stock egui's by {stock_count}"
                    ));
                }
                if doc_bytes != stock_bytes {
                    failures.push(format!(
                        "{case:?}, {bytes} B: the document's gross bytes grow by {doc_bytes}, stock egui's by {stock_bytes}"
                    ));
                }
            }
        }
    }
    assert!(
        failures.is_empty(),
        "quiet_markdown:\n{}",
        failures.join("\n")
    );
    println!("quiet_markdown: ok");
}
