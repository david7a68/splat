//! The app's state context: what the app's state methods need from the frame, and nothing that
//! draws. The root (`gui::SplatRoot::context`) owns one and passes it to those methods.
//!
//! It holds three things:
//!
//! - **Typed slots** ([`Slot`]): a request that one part leaves for another (a pressed link that
//!   the frame logic routes, a page to open), and a value that a part keeps for a view (a roll
//!   table's state for one reader). A slot names its value's type, so its writer and its reader
//!   cannot disagree on it.
//! - **The pass** ([`StateContext::begin_pass`]): the root starts a pass each time it runs the
//!   frame logic. It moves the frame clock (`frame_clock`) to the time since the app started and
//!   counts the passes ([`StateContext::pass`]).
//! - **Repaint requests** ([`Waker`]): a state method, or a background thread that holds the
//!   context's waker, asks for a frame now or after a delay. The root's poll task reads the waker
//!   ([`Waker::take_due`]) and runs the frame logic when a frame is due.
//!
//! The context is a cheap handle (`Clone` shares it) for one thread, the UI's. A background
//! thread takes only the [`Waker`].
use std::{
    any::{Any, TypeId},
    cell::{Cell, RefCell},
    collections::HashMap,
    hash::{Hash, Hasher},
    marker::PhantomData,
    rc::Rc,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

/// A key for a slot: a name, made distinct by any hashable value (`Key::with`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Key(u64);

impl Key {
    /// A key by its name alone.
    pub const fn new(name: &'static str) -> Self {
        // FNV-1a, so that a slot's key is a constant.
        let bytes = name.as_bytes();
        let mut hash = 0xcbf2_9ce4_8422_2325_u64;
        let mut at = 0;
        while at < bytes.len() {
            hash ^= bytes[at] as u64;
            hash = hash.wrapping_mul(0x0100_0000_01b3);
            at += 1;
        }
        Self(hash)
    }

    /// A key made from this one and `part` (one view's key for a shared name).
    #[must_use]
    pub fn with(self, part: impl Hash) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.0.hash(&mut hasher);
        part.hash(&mut hasher);
        Self(hasher.finish())
    }
}

/// A typed place in the context for one value of `T`.
pub struct Slot<T> {
    key: Key,
    value: PhantomData<fn() -> T>,
}

impl<T> Clone for Slot<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Slot<T> {}

impl<T> std::fmt::Debug for Slot<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Slot").field(&self.key).finish()
    }
}

impl<T> PartialEq for Slot<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<T> Eq for Slot<T> {}

impl<T> Hash for Slot<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

impl<T> Slot<T> {
    /// A slot by its name.
    pub const fn new(name: &'static str) -> Self {
        Self::at(Key::new(name))
    }

    /// A slot at `key`.
    pub const fn at(key: Key) -> Self {
        Self {
            key,
            value: PhantomData,
        }
    }

    /// This slot's own place for `part` (one view's value).
    #[must_use]
    pub fn with(self, part: impl Hash) -> Self {
        Self::at(self.key.with(part))
    }
}

/// Asks for a frame. A clone goes to a background thread, which asks when it has news.
#[derive(Clone, Default)]
pub struct Waker(Arc<Mutex<Option<Instant>>>);

impl Waker {
    /// Ask for a frame now.
    pub fn request_repaint(&self) {
        self.request_repaint_after(Duration::ZERO);
    }

    /// Ask for a frame after `delay` (an hour at most). The soonest request wins.
    pub fn request_repaint_after(&self, delay: Duration) {
        let at = Instant::now() + delay.min(Duration::from_secs(3600));
        let mut due = self.0.lock().unwrap_or_else(|poison| poison.into_inner());
        *due = Some(due.map_or(at, |old| old.min(at)));
    }

    /// Whether a frame is due now; a due frame is taken.
    pub fn take_due(&self) -> bool {
        let mut due = self.0.lock().unwrap_or_else(|poison| poison.into_inner());
        match *due {
            Some(at) if at <= Instant::now() => {
                *due = None;
                true
            }
            _ => false,
        }
    }

    /// Whether any frame is asked for (now or later).
    pub fn pending(&self) -> bool {
        self.0
            .lock()
            .unwrap_or_else(|poison| poison.into_inner())
            .is_some()
    }
}

struct Inner {
    slots: RefCell<HashMap<(TypeId, Key), Box<dyn Any>>>,
    waker: Waker,
    pass: Cell<u64>,
    started: Instant,
}

/// The app's state context (see the module's notes).
#[derive(Clone)]
pub struct StateContext(Rc<Inner>);

impl Default for StateContext {
    fn default() -> Self {
        Self::new()
    }
}

impl StateContext {
    pub fn new() -> Self {
        Self(Rc::new(Inner {
            slots: RefCell::default(),
            waker: Waker::default(),
            pass: Cell::new(0),
            started: Instant::now(),
        }))
    }

    /// Start a pass of the frame logic: the frame clock moves to the time since the app started,
    /// and the pass count goes up by one.
    pub fn begin_pass(&self) {
        crate::frame_clock::tick(self.0.started.elapsed().as_secs_f64());
        self.0.pass.set(self.0.pass.get() + 1);
    }

    /// The number of passes started so far: two values are equal within one pass.
    pub fn pass(&self) -> u64 {
        self.0.pass.get()
    }

    /// The waker, for a background thread or the root's poll task.
    pub fn waker(&self) -> Waker {
        self.0.waker.clone()
    }

    /// Ask for a frame now.
    pub fn request_repaint(&self) {
        self.0.waker.request_repaint();
    }

    /// Ask for a frame after `delay`.
    pub fn request_repaint_after(&self, delay: Duration) {
        self.0.waker.request_repaint_after(delay);
    }

    /// A copy of the value in `slot`, if there is one.
    pub fn get<T: Clone + 'static>(&self, slot: Slot<T>) -> Option<T> {
        self.0
            .slots
            .borrow()
            .get(&(TypeId::of::<T>(), slot.key))
            .and_then(|value| value.downcast_ref::<T>())
            .cloned()
    }

    /// Put `value` in `slot`, in place of any value there.
    pub fn set<T: 'static>(&self, slot: Slot<T>, value: T) {
        self.0
            .slots
            .borrow_mut()
            .insert((TypeId::of::<T>(), slot.key), Box::new(value));
    }

    /// Take the value out of `slot`, leaving it empty.
    pub fn take<T: 'static>(&self, slot: Slot<T>) -> Option<T> {
        self.0
            .slots
            .borrow_mut()
            .remove(&(TypeId::of::<T>(), slot.key))
            .and_then(|value| value.downcast::<T>().ok())
            .map(|value| *value)
    }

    /// Put `value` in `slot`, or empty it for `None`.
    pub fn put<T: 'static>(&self, slot: Slot<T>, value: Option<T>) {
        match value {
            Some(value) => self.set(slot, value),
            None => {
                self.take(slot);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_slot_keeps_its_value_by_its_type_and_its_key() {
        const FLAG: Slot<bool> = Slot::new("flag");
        const WORDS: Slot<String> = Slot::new("flag");
        let ctx = StateContext::new();
        assert_eq!(ctx.get(FLAG), None);
        ctx.set(FLAG, true);
        assert_eq!(ctx.get(WORDS), None, "another type has its own place");
        assert_eq!(
            ctx.get(FLAG.with(1)),
            None,
            "another part has its own place"
        );
        assert_eq!(ctx.get(FLAG), Some(true), "get leaves the value");
        assert_eq!(ctx.take(FLAG), Some(true));
        assert_eq!(ctx.get(FLAG), None, "take empties the slot");
        ctx.put(WORDS, Some("a".to_owned()));
        ctx.clone().put(WORDS, None);
        assert_eq!(ctx.get(WORDS), None, "a clone shares the slots");
    }

    #[test]
    fn the_soonest_repaint_request_wins_and_is_taken_once() {
        let ctx = StateContext::new();
        let waker = ctx.waker();
        assert!(!waker.pending());
        ctx.request_repaint_after(Duration::from_secs(60));
        assert!(
            waker.pending() && !waker.take_due(),
            "a later frame is not due yet"
        );
        std::thread::spawn(move || waker.request_repaint())
            .join()
            .unwrap();
        let waker = ctx.waker();
        assert!(waker.take_due(), "a thread's request is due now");
        assert!(
            !waker.take_due() && !waker.pending(),
            "a due frame is taken"
        );
    }

    #[test]
    fn a_pass_moves_the_frame_clock_and_counts() {
        let ctx = StateContext::new();
        crate::frame_clock::set(-1.0);
        ctx.begin_pass();
        assert!(crate::frame_clock::now() >= 0.0);
        assert_eq!(ctx.pass(), 1);
        ctx.begin_pass();
        assert_eq!(ctx.pass(), 2);
    }
}
