//! A document's text with its identity and a trusted revision (layout PR 9b, rulings L8 and
//! M25): the field parts know an unchanged document by its key, so a quiet frame hashes, copies,
//! compares and lays out none of it.
//!
//! - **Identity.** Each [`TextDraft`] has its own [`DraftId`], from a counter: two drafts never
//!   share one, whatever their words. A draft is not `Clone`; [`TextDraft::copy_as_new`] is
//!   another document, with a new id.
//! - **Revision.** Each change of the bytes is a new [`Revision`] (`design::memo`), new in this
//!   run. Per buffer operation:
//!   - `insert_text`: a new revision when it inserted at least one character;
//!   - `delete_char_range`: a new revision when the range was not empty;
//!   - egui's default `replace_with`, `clear` and `take` run through those two, so a clear then
//!     an insert of the same words gives two new revisions: the words end equal, the key does
//!     not;
//!   - [`TextDraft::replace`] (the program's change: a reload, a discard): always a new
//!     revision, also for equal words, so the guard drops egui's history for it.
//!
//!   A no-op event (an empty paste, Delete at the end) keeps the revision, even when egui says
//!   `changed()`: the callers keep egui's `changed()` for their events; the key is only for the
//!   parts' kept work.
//!
//! ```
//! use splat_design::text_draft::TextDraft;
//! let mut draft = TextDraft::new("Line 1");
//! let first = draft.key();
//! assert_eq!(egui::TextBuffer::insert_text(&mut draft, "", 0), 0);
//! assert_eq!(draft.key(), first, "a no-op keeps the key");
//! egui::TextBuffer::insert_text(&mut draft, "!", 6);
//! assert_ne!(draft.key(), first);
//! let copy = draft.copy_as_new();
//! assert_ne!(copy.key().draft(), draft.key().draft());
//! ```
//!
//! A draft cannot be copied by `Clone`:
//!
//! ```compile_fail,E0599
//! let draft = splat_design::text_draft::TextDraft::new("Line 1");
//! let _ = draft.clone();
//! ```
use super::memo::Revision;
use std::sync::atomic::{AtomicU64, Ordering};

/// One document's identity: never shared by two drafts.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct DraftId(u64);

impl DraftId {
    fn next() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        Self(NEXT.fetch_add(1, Ordering::Relaxed))
    }
}

/// A draft's identity and the revision of its words: what the parts key their kept work by.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct DraftKey {
    draft: DraftId,
    revision: Revision,
}

impl DraftKey {
    pub fn draft(self) -> DraftId {
        self.draft
    }
}

/// A document's text, its identity and its revision. See the module comment.
#[derive(Debug)]
pub struct TextDraft {
    key: DraftKey,
    text: String,
}

impl TextDraft {
    /// A new document, with a new identity.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            key: DraftKey {
                draft: DraftId::next(),
                revision: Revision::next(),
            },
            text: text.into(),
        }
    }

    /// Another document with the same words: a new identity.
    pub fn copy_as_new(&self) -> Self {
        Self::new(self.text.clone())
    }

    pub fn key(&self) -> DraftKey {
        self.key
    }

    pub fn as_str(&self) -> &str {
        &self.text
    }

    /// The program's change of the words (a reload, a discard, an Undo of the program's): the
    /// same document, always a new revision.
    pub fn replace(&mut self, text: impl Into<String>) {
        self.text = text.into();
        self.revise();
    }

    fn revise(&mut self) {
        self.key.revision = Revision::next();
    }
}

impl std::ops::Deref for TextDraft {
    type Target = str;
    fn deref(&self) -> &str {
        &self.text
    }
}

impl egui::TextBuffer for TextDraft {
    fn is_mutable(&self) -> bool {
        true
    }

    fn as_str(&self) -> &str {
        &self.text
    }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        let inserted = self.text.insert_text(text, char_index);
        if inserted > 0 {
            self.revise();
        }
        inserted
    }

    fn delete_char_range(&mut self, char_range: std::ops::Range<usize>) {
        if char_range.start < char_range.end {
            self.text.delete_char_range(char_range);
            self.revise();
        }
    }

    fn type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<Self>()
    }
}

/// A draft lent to egui's `TextEdit` for one draw: each change reaches `live`, so the part's
/// layouter reads the live key on every call (egui lays out again after each edit in its own
/// pass, also several in one frame).
pub(in crate::design) struct Lent<'a> {
    pub draft: &'a mut TextDraft,
    pub live: &'a std::cell::Cell<DraftKey>,
}

impl egui::TextBuffer for Lent<'_> {
    fn is_mutable(&self) -> bool {
        true
    }

    fn as_str(&self) -> &str {
        &self.draft.text
    }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        let inserted = self.draft.insert_text(text, char_index);
        self.live.set(self.draft.key);
        inserted
    }

    fn delete_char_range(&mut self, char_range: std::ops::Range<usize>) {
        self.draft.delete_char_range(char_range);
        self.live.set(self.draft.key);
    }

    fn type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<TextDraft>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::TextBuffer;

    /// The revision rule per operation (module comment), and identity.
    #[test]
    fn each_byte_change_is_a_new_revision_and_nothing_else_is() {
        let mut draft = TextDraft::new("Ash road");
        let start = draft.key();
        draft.delete_char_range(3..3);
        assert_eq!(draft.key(), start, "an empty delete keeps the key");
        draft.insert_text("", 0);
        assert_eq!(draft.key(), start, "an empty insert keeps the key");
        draft.delete_char_range(0..3);
        let deleted = draft.key();
        assert_ne!(deleted, start);
        draft.insert_text("Ash", 0);
        assert_eq!(draft.as_str(), "Ash road");
        assert_ne!(
            draft.key(),
            deleted,
            "clear then insert of equal words: a new key"
        );
        assert_ne!(draft.key(), start);
        let before = draft.key();
        draft.replace("Ash road");
        assert_ne!(draft.key(), before, "a program replacement is always new");
        assert_eq!(draft.key().draft(), start.draft(), "the same document");
        let taken = draft.take();
        assert_eq!(taken, "Ash road");
        assert!(draft.as_str().is_empty());
        let other = TextDraft::new("Ash road");
        assert_ne!(other.key().draft(), draft.key().draft());
    }
}
