//! A document's text with its identity and a trusted revision (layout PR 9b, rulings L8 and
//! M25): the field parts know an unchanged document by its key, so a quiet frame hashes, copies,
//! compares and lays out none of it.
//!
//! - **Identity.** Each [`TextDraft`] has its own [`DraftId`], from a counter: two drafts never
//!   share one, whatever their words. A clone is another document, with a new id
//!   ([`TextDraft::copy_as_new`]), so a host's draft that derives `Clone` (an Undo snapshot, a
//!   form's start) never shares a key with the draft it copied.
//! - **Words.** A draft has no `==`: a host compares words with `as_str()`, so each compare of
//!   the whole text is in plain sight (layout PR 9b: a quiet frame should compare none).
//! - **Revision.** Each change of the bytes is a new [`Revision`] (`design::memo`), new in this
//!   run. Per buffer operation:
//!   - `insert_text`: a new revision when it inserted at least one character;
//!   - `delete_char_range`: a new revision when it removed at least one character (egui asks
//!     past the end for Delete at the end; that removes nothing and keeps the key);
//!   - egui's default `replace_with`, `clear` and `take` run through those two, so a clear then
//!     an insert of the same words gives two new revisions: the words end equal, the key does
//!     not;
//!   - [`TextDraft::replace`] (the program's change: a reload, a discard): always a new
//!     revision, also for equal words, so its layout is made again. (The field guard compares
//!     words: other words drop egui's history, equal words keep it, as for a plain text.)
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
//! A clone is a new document; a draft has no `==`:
//!
//! ```
//! let draft = splat_design::text_draft::TextDraft::new("Line 1");
//! assert_ne!(draft.clone().key().draft(), draft.key().draft());
//! ```
//!
//! ```compile_fail,E0369
//! let draft = splat_design::text_draft::TextDraft::new("Line 1");
//! let _ = draft == splat_design::text_draft::TextDraft::new("Line 1");
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

    /// The key as bytes, for a memo keyed by a hash of its inputs: a result kept for one
    /// revision of one draft, with no hash of its words.
    pub fn to_bytes(self) -> [u8; 16] {
        let mut bytes = [0; 16];
        bytes[..8].copy_from_slice(&self.draft.0.to_le_bytes());
        bytes[8..].copy_from_slice(&self.revision.value().to_le_bytes());
        bytes
    }
}

/// The last answer of [`TextDraft::same_as`], kept by the two revisions it compared. A host
/// keeps one per compare it makes on every frame (a draft against its saved text).
pub type KeptCompare = std::cell::Cell<Option<(Revision, Revision, bool)>>;

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// How many whole-text compares the drafts made (`TextDraft::same_words`).
    pub static WORDS_COMPARED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    /// How many bytes of words were copied: a draft's clone, and a host's own counted copies
    /// ([`count_copied`]).
    pub static WORDS_COPIED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// A host copied `bytes` of a document's words (a template's clone): counted in tests, so a
/// quiet frame's copy shows in `pane::text_work`.
pub fn count_copied(bytes: usize) {
    #[cfg(any(test, feature = "test-support"))]
    WORDS_COPIED.with(|count| count.set(count.get() + bytes));
    let _ = bytes;
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
        count_copied(self.text.len());
        Self::new(self.text.clone())
    }

    pub fn key(&self) -> DraftKey {
        self.key
    }

    /// The revision of the words: new in this run for each change, also across drafts.
    pub fn revision(&self) -> Revision {
        self.key.revision
    }

    /// Whether the words are `other`: one compare of the whole text, counted in tests (layout
    /// PR 9b-C: a quiet frame compares none). Kept by revisions in [`same_as`](Self::same_as).
    pub fn same_words(&self, other: &str) -> bool {
        #[cfg(any(test, feature = "test-support"))]
        WORDS_COMPARED.with(|count| count.set(count.get() + 1));
        self.text == other
    }

    /// Whether the words are `words`, the value of revision `other` (a saved text as a
    /// `memo::Revised`, or another draft): compared once per pair of revisions, the answer
    /// kept in `kept`. A quiet frame compares two numbers.
    pub fn same_as(&self, other: Revision, words: &str, kept: &KeptCompare) -> bool {
        let key = (self.revision(), other);
        if let Some((draft, saved, same)) = kept.get()
            && (draft, saved) == key
        {
            return same;
        }
        let same = self.same_words(words);
        kept.set(Some((key.0, key.1, same)));
        same
    }

    pub fn as_str(&self) -> &str {
        &self.text
    }

    /// The program's change of the words (a reload, a discard, an Undo of the program's): the
    /// same document, always a new revision. Gives the words it replaced.
    pub fn replace(&mut self, text: impl Into<String>) -> String {
        let old = std::mem::replace(&mut self.text, text.into());
        self.revise();
        old
    }

    /// The program's addition at the end (a test's typing, an insert from a command): a new
    /// revision when `text` is not empty.
    pub fn push_str(&mut self, text: &str) {
        if !text.is_empty() {
            self.text.push_str(text);
            self.revise();
        }
    }

    /// The program's change of the words in place (an insert from a command, a renamed link):
    /// always a new revision. A host's `&mut String` helper runs here.
    pub fn edit<R>(&mut self, change: impl FnOnce(&mut String) -> R) -> R {
        let result = change(&mut self.text);
        self.revise();
        result
    }

    /// No words: [`replace`](Self::replace) with nothing.
    pub fn clear(&mut self) {
        let _ = self.replace(String::new());
    }

    /// The words, as the draft ends.
    pub fn into_string(self) -> String {
        self.text
    }

    fn revise(&mut self) {
        self.key.revision = Revision::next();
    }
}

/// A copy is another document (a new identity), with the same words.
impl Clone for TextDraft {
    fn clone(&self) -> Self {
        self.copy_as_new()
    }
}

/// A new document of these words.
impl From<String> for TextDraft {
    fn from(text: String) -> Self {
        Self::new(text)
    }
}

/// A new document of these words.
impl From<&str> for TextDraft {
    fn from(text: &str) -> Self {
        Self::new(text)
    }
}

/// The words.
impl std::fmt::Display for TextDraft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.text)
    }
}

/// An empty new document.
impl Default for TextDraft {
    fn default() -> Self {
        Self::new(String::new())
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
        // egui asks past the end (Delete at the end asks for `len..len + 1`), and `String`
        // clamps that to nothing: only a deletion that removed bytes is a new revision.
        let before = self.text.len();
        self.text.delete_char_range(char_range);
        if self.text.len() != before {
            self.revise();
        }
    }

    fn type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<Self>()
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
        draft.delete_char_range(8..9);
        assert_eq!(
            draft.key(),
            start,
            "a delete past the end (Delete at the end) removes nothing and keeps the key"
        );
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
