//! The counting global allocator of the quiet-frame harnesses (layout PR 9b): `quiet_text`
//! (the document editor) and `quiet_reader` (the Reader) share it by `#[path]`. Each harness is a
//! process of its own (`harness = false`), so this allocator sees every thread of it.
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
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
