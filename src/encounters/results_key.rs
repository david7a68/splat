//! A draft's results key: the owner of its member and editor results (review 2 of PR 347).
//!
//! The key is shared by every copy of its draft (a clone shares it): the discarded draft that
//! Undo discard restores, the group a removed group's Undo holds. A store entry keeps only a
//! `KeyWatch`, which does not keep the draft's key alive. So an entry is live exactly while some
//! copy of its draft exists, and nothing has to mark the end of a draft: a holder set to `None`,
//! a draft read again from disk (`*self = Self::open(…)`), a campaign change and a dropped Undo
//! all drop the old draft, and with it its key.
#![warn(deprecated)]

use std::sync::{Arc, Weak};
use uuid::Uuid;

/// A draft's results key. A new draft (opened, read again or new) gets a new one.
#[derive(Clone, Debug)]
pub(super) struct ResultsKey(Arc<Uuid>);

impl ResultsKey {
    pub(super) fn new() -> Self {
        Self(Arc::new(crate::hermetic::new_uuid()))
    }
    /// The key's identity, as an owner in a store.
    pub(super) fn id(&self) -> Uuid {
        *self.0
    }
    /// A watch on this key for a store entry: it does not keep the key alive.
    pub(super) fn watch(&self) -> KeyWatch {
        KeyWatch(Arc::downgrade(&self.0))
    }
}

/// A store entry's watch on its draft's key.
pub(super) struct KeyWatch(Weak<Uuid>);

impl KeyWatch {
    /// Whether some copy of the draft still exists.
    pub(super) fn alive(&self) -> bool {
        #[cfg(test)]
        super::KEY_CHECKS.with(|count| count.set(count.get() + 1));
        self.0.strong_count() > 0
    }
}
