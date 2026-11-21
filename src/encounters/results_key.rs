//! A draft's results key: the owner of its member and editor results (review 2 of PR 347).
//!
//! The key is shared by every copy of its draft (a clone shares it): the discarded draft that
//! Undo discard restores, the group a removed group's Undo holds. A store entry keeps only a
//! `KeyWatch`, which does not keep the draft's key alive. So an entry is live exactly while some
//! copy of its draft exists, and nothing has to mark the end of a draft: a holder set to `None`,
//! a draft read again from disk (`*self = Self::open(…)`), a campaign change and a dropped Undo
//! all drop the old draft, and with it its key.
//!
//! The drop of a key is the only signal that a result may be dead: it raises a count
//! (`dead_keys`), and the page ends dead results only when that count changed. A frame with no
//! dropped key does no work, however many results are stored.
#![warn(deprecated)]

use std::sync::{Arc, Weak};
use uuid::Uuid;

thread_local! {
    /// How many results keys this thread dropped. Drafts live on the UI thread, which also
    /// runs the page's `poll`.
    static DEAD_KEYS: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// How many results keys were dropped so far: a change means a result may be dead.
pub(super) fn dead_keys() -> u64 {
    DEAD_KEYS.with(std::cell::Cell::get)
}

/// The value the key's `Arc` holds: its drop, when the last copy of the draft goes, raises
/// `dead_keys`.
#[derive(Debug)]
struct Key(Uuid);

impl Drop for Key {
    fn drop(&mut self) {
        DEAD_KEYS.with(|count| count.set(count.get() + 1));
    }
}

/// A draft's results key. A new draft (opened, read again or new) gets a new one.
#[derive(Clone, Debug)]
pub(super) struct ResultsKey(Arc<Key>);

impl ResultsKey {
    pub(super) fn new() -> Self {
        Self(Arc::new(Key(crate::hermetic::new_uuid())))
    }
    /// The key's identity, as an owner in a store.
    pub(super) fn id(&self) -> Uuid {
        self.0.0
    }
    /// A watch on this key for a store entry: it does not keep the key alive.
    pub(super) fn watch(&self) -> KeyWatch {
        KeyWatch(Arc::downgrade(&self.0))
    }
}

/// A store entry's watch on its draft's key.
pub(super) struct KeyWatch(Weak<Key>);

impl KeyWatch {
    /// Whether some copy of the draft still exists.
    pub(super) fn alive(&self) -> bool {
        #[cfg(any(test, feature = "test-support"))]
        super::KEY_CHECKS.with(|count| count.set(count.get() + 1));
        self.0.strong_count() > 0
    }
}
