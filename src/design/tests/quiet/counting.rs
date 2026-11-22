//! The counting allocator of the quiet-frame harnesses (layout PR 9b): one per test process,
//! included by `#[path]` (`splat-design`'s `quiet_text`, `splat-content-ui`'s
//! `quiet_markdown`), so both measure the same way. Gross counts and bytes per measured frame,
//! and a check that the measuring thread did all of the counted work.
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

pub struct Counting;

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
pub struct Allocated {
    pub count: usize,
    pub bytes: usize,
}

pub fn measured(run: impl FnOnce()) -> Allocated {
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
    assert_eq!(
        mine, all,
        "every counted allocation was the measuring thread's"
    );
    mine
}

/// The whole screen, with no panel or frame of its own (both sides draw in the same one).
#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
pub fn root(ctx: &egui::Context) -> egui::Ui {
    egui::Ui::new(
        ctx.clone(),
        egui::Id::new("quiet_harness_root"),
        egui::UiBuilder::new().max_rect(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(800.0, 600.0),
        )),
    )
}

/// The stock reference: egui's own text edit, laid out by `layouter`, at `wrap`.
#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
pub fn stock_text_edit(
    ui: &mut egui::Ui,
    id: egui::Id,
    text: &mut String,
    wrap: f32,
    layouter: &mut dyn FnMut(&egui::Ui, &dyn egui::TextBuffer, f32) -> std::sync::Arc<egui::Galley>,
) {
    let _ = egui::TextEdit::multiline(text)
        .id(id)
        .layouter(layouter)
        .frame(false)
        .margin(egui::Vec2::ZERO)
        .desired_width(wrap)
        .show(ui);
}

/// A bare screen of `size`, with no panel or frame of its own (layout PR 9b-R1a: the Reader's
/// harness draws a column as wide as its case, tall enough to see a whole note).
#[allow(
    dead_code,
    reason = "the Reader’s harness (`quiet_reader`) is its only user"
)]
#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
pub fn root_sized(ctx: &egui::Context, size: egui::Vec2) -> egui::Ui {
    egui::Ui::new(
        ctx.clone(),
        egui::Id::new("quiet_harness_root"),
        egui::UiBuilder::new().max_rect(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
    )
}

/// The Reader's stock reference: egui's own label over a galley laid out once.
#[allow(
    dead_code,
    reason = "the Reader’s harness (`quiet_reader`) is its only user"
)]
pub fn stock_label(ui: &mut egui::Ui, galley: std::sync::Arc<egui::Galley>) {
    ui.add(egui::Label::new(galley));
}
