//! Layout parts: one place for each layout decision that screens kept making by hand.
//!
//! - [`ScrollColumn`]: a vertical scroll area that always keeps the scroll bar's lane, so no
//!   control moves when the bar comes or goes (family F14). [`sideways_scroll`] is the sideways
//!   table scroll.
//! - [`Slot`] and [`status_slot`]: fixed room for a value that changes in place (F8).
//! - [`TitleLine`] and [`title_placement`]: a title and its actions; every action keeps its word
//!   (F17).
//! - [`InlineCreate`]: the one inline creation layout (F15), built in a fixed order by its type.
//! - [`FieldId`]: a text field's id that does not depend on its place in the layout, so a layout
//!   branch cannot drop the field's focus or cursor (PR 297).
//!
//! `clippy.toml` disallows the raw egui calls these parts replace, outside `src/design/`.
//!
//! Each part's tests (`layout/tests.rs`) prove its rule once. A caller that uses a part, with no
//! layer, clip, `Area` or raw interaction of its own, has no duty left for that rule, except
//! where a part's doc names one.
use std::{borrow::Cow, hash::Hash, marker::PhantomData, ops::Range};

use egui::{
    self, Align, Id, Layout, Rect, Response, Sense, TextBuffer, TextEdit, Ui, UiBuilder,
    scroll_area::ScrollAreaOutput, vec2,
};

use super::{
    Action, Density, FORM_WIDTH, L, LINE_START_HANG, M, PANEL_MIN_WIDTH, S, Status, Type, XL, XS,
    colors, extras::space, field, named, number_width, outcome::Sentence, placeholder, theme,
};

/// The longest word an action may have, in characters. With this limit and an icon, the widest
/// letters fit on one line in the narrowest pane that holds a title line ([`NARROWEST_PANE`]);
/// a part test proves it.
pub const MAX_WORD_CHARS: usize = 20;

/// The text width of the narrowest pane that holds a title line: a side panel (Lookup, Notes,
/// the run's panel) at its minimum width, less its margins. The rail holds no title line.
pub const NARROWEST_PANE: f32 = PANEL_MIN_WIDTH - 2.0 * M;

/// An action's word: a non-empty, sentence-case word or phrase of at most [`MAX_WORD_CHARS`]
/// characters. Make one with [`word!`](crate::word), which takes a string literal and checks it
/// when the program compiles.
///
/// ```
/// const EDIT: splat_design::layout::Word = splat_design::word!("Edit");
/// assert_eq!(EDIT.as_str(), "Edit");
/// ```
///
/// An empty word, one that does not start with a capital, one with a space at its end and one
/// longer than the limit do not compile:
///
/// ```compile_fail,E0080
/// let _ = splat_design::word!("");
/// ```
///
/// ```compile_fail,E0080
/// let _ = splat_design::word!("edit");
/// ```
///
/// ```compile_fail,E0080
/// let _ = splat_design::word!("Edit ");
/// ```
///
/// ```compile_fail,E0080
/// let _ = splat_design::word!("Add every creature to the fight");
/// ```
///
/// A word made at run time, or held in a constant, is not a literal:
///
/// ```compile_fail
/// let name = String::from("Edit");
/// let _ = splat_design::word!(name.as_str());
/// ```
///
/// ```compile_fail
/// const NAME: &str = "Edit";
/// let _ = splat_design::word!(NAME);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Word(&'static str);

impl Word {
    /// Use [`word!`](crate::word). Clippy disallows this call outside `src/design/`, so a call
    /// by hand needs an allow that the lint counts.
    #[doc(hidden)]
    pub const fn checked(word: &'static str) -> Self {
        let bytes = word.as_bytes();
        assert!(!bytes.is_empty(), "A word is not empty.");
        assert!(
            bytes[0].is_ascii_uppercase(),
            "A word starts with a capital."
        );
        assert!(
            !bytes[bytes.len() - 1].is_ascii_whitespace(),
            "A word ends with no space."
        );
        let mut at = 0;
        let mut chars = 0;
        while at < bytes.len() {
            assert!(
                bytes[at] != b'\n' && bytes[at] != b'\t',
                "A word is one line."
            );
            // Count characters, not bytes: a UTF-8 continuation byte starts no character.
            if bytes[at] & 0xC0 != 0x80 {
                chars += 1;
            }
            at += 1;
        }
        assert!(chars <= MAX_WORD_CHARS, "A word has at most 20 characters.");
        Self(word)
    }

    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

/// A [`Word`] from a string literal, checked when the program compiles.
#[macro_export]
macro_rules! word {
    ($word:literal) => {
        const {
            #[allow(clippy::disallowed_methods, reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)")]
            let word = $crate::layout::Word::checked($word);
            word
        }
    };
}

mod sealed {
    pub trait Sealed {}
}

/// A key that tells one field of many apart. Sealed: only the kinds below are keys, and
/// `egui::Id` is not one. That is all the type proves. That a key comes from the row's data, not
/// from its place on screen, is the caller's duty: a number or a text can still be read from the
/// layout. So the two keys that take any number or text are named, and each use is a counted
/// row of `design::results::STAGED_ALLOWS` (`Ban::RowKey`), so a review sees each new one.
pub trait RowKey: sealed::Sealed + Hash {}

/// A row's place in its data (an entry's index in a saved list), as a [`RowKey`]. Give it a
/// data index, not a value read from the layout. Clippy bans `from_data` outside `src/design/`,
/// so each use is a counted row of `design::results::STAGED_ALLOWS` (`Ban::RowKey`) that a
/// reviewer reads.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RowIndex(usize);

impl RowIndex {
    pub const fn from_data(index: usize) -> Self {
        Self(index)
    }
}

/// A text from the row's data (a file name, an entry's name), as a [`RowKey`]. Named, so that a
/// key made with `format!` is seen: Clippy bans `from_data` outside `src/design/`, and each use
/// is a counted row (`Ban::RowKey`). An object whose id is a text is a [`TextOwner`]: key its
/// fields with [`TextKey::of`], which needs no counted row.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TextKey(String);

impl TextKey {
    pub fn from_data(text: impl Into<String>) -> Self {
        Self(text.into())
    }

    /// The key of `owner`'s fields: its own id text. Not banned: the text comes from a listed
    /// owner type's id, never from the layout or a `format!`.
    ///
    /// The compiling twin is the part test `a_timeline_event_keys_its_fields` (a timeline event,
    /// a crate-private type, cannot be named in a doc example).
    ///
    /// A free text is not an owner, and only the types listed in `src/design/layout.rs` can
    /// become one (the trait is sealed):
    ///
    /// ```compile_fail,E0277
    /// let _ = splat_design::layout::TextKey::of("journey");
    /// ```
    ///
    /// ```compile_fail,E0277
    /// let _ = splat_design::layout::TextKey::of(&format!("row {}", 1));
    /// ```
    ///
    /// No public owner type holds a free text (review 1: `DocOwner` did):
    ///
    /// ```compile_fail,E0422
    /// let _ = splat_design::layout::TextKey::of(&splat_design::layout::DocOwner { id: format!("row_{}", 1) });
    /// ```
    ///
    /// A caller's own type cannot become an owner (it would turn any text into a key with no
    /// counted row):
    ///
    /// ```compile_fail,E0277
    /// struct Row(String);
    /// impl splat_design::layout::TextOwner for Row {
    ///     fn owner_text(&self) -> &str {
    ///         &self.0
    ///     }
    /// }
    /// ```
    pub fn of(owner: &impl TextOwner) -> Self {
        Self(owner.owner_text().to_owned())
    }
}

/// A run's participant, the owner of its setup fields (its initiative, its roll mode, the
/// initiative-roll drop-down): the run's id and the participant's key, which stay the same
/// while the run lasts (a rename keeps them). Made only from a run's id and its participant, so
/// neither a place in the layout nor a `format!` can be one, and it needs no counted row. Two
/// runs open at once never share a participant's field. Use it as a field's owner:
/// `field_id!("initiative_mode").of(ParticipantKey::of(run, participant))`, also for a
/// [`Dropdown`](super::popups::Dropdown), whose id is that `FieldId`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ParticipantKey {
    run: uuid::Uuid,
    participant: String,
}

impl ParticipantKey {
    /// The key of `participant`'s fields in the run `run`.
    pub fn of(run: uuid::Uuid, participant: &splat_campaign::encounter_run::Participant) -> Self {
        Self {
            run,
            participant: participant.key.clone(),
        }
    }
}

/// An object whose id is a text (a timeline event's `id`). Its text fields are keyed by that id
/// with [`TextKey::of`]. Sealed: a type becomes an owner only by a line in
/// `src/design/layout.rs` (`text_owners!`), so `String` and `str` never do, and the id is
/// borrowed from the object, so a `format!` cannot be one.
pub trait TextOwner: sealed::Sealed {
    /// The object's id, borrowed from its data.
    fn owner_text(&self) -> &str;
}

macro_rules! text_owners {
    ($($owner:ty => |$it:ident| $id:expr),* $(,)?) => {$(
        impl sealed::Sealed for $owner {}
        impl TextOwner for $owner {
            fn owner_text(&self) -> &str {
                let $it = self;
                $id
            }
        }
    )*};
}
text_owners!(splat_campaign::timeline::Event => |event| &event.id);

macro_rules! row_keys {
    ($($key:ty),*) => {$(
        impl sealed::Sealed for $key {}
        impl RowKey for $key {}
    )*};
}
row_keys!(uuid::Uuid, RowIndex, TextKey, ParticipantKey);
impl<K: RowKey + ?Sized> sealed::Sealed for &K {}
impl<K: RowKey + ?Sized> RowKey for &K {}
impl<K: RowKey> sealed::Sealed for Option<K> {}
impl<K: RowKey> RowKey for Option<K> {}

/// Why a field exists once in the whole app, so its id needs no owner's key. A closed set: a
/// reviewer reads a wrong claim in one word. A field that edits one object of many (an event, a
/// session, a member, a run, a picker of an entry) is never `Once`: key it with
/// [`FieldName::of`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Once {
    /// A setting of the app: there is one settings object, and its fields are its own.
    Settings,
    /// The query of a page's or a popup's own search. The query is not saved object data.
    PageSearch,
    /// The one form that makes a new object. It holds no saved object's text.
    CreateForm,
    /// An example in the design gallery, a doc example or a part test.
    Example,
}

/// A text field's name, from [`field_id!`](crate::field_id). It cannot draw a field: say whose
/// field it is first. [`of`](Self::of) gives the field of one object (its key comes from that
/// object's data); [`once`](Self::once) gives the field that exists once in the app, for one of
/// the [`Once`] reasons.
///
/// ```
/// use splat_design::layout::{FieldName, Once, RowIndex};
/// const NAME: FieldName = splat_design::field_id!("doc_name");
/// const COOKIE: splat_design::layout::FieldId = splat_design::field_id!("doc_cookie").once(Once::Settings);
/// fn f(ui: &mut egui::Ui, text: &mut String) {
///     let _ = NAME.of(RowIndex::from_data(1)).single_line(ui, text, |edit| edit);
///     let _ = COOKIE.single_line(ui, text, |edit| edit);
/// }
/// ```
///
/// A name with no owner choice draws nothing:
///
/// ```compile_fail,E0599
/// fn f(ui: &mut egui::Ui, text: &mut String) {
///     let _ = splat_design::field_id!("doc_name").single_line(ui, text, |edit| edit);
/// }
/// ```
///
/// ```compile_fail,E0308
/// fn f(ui: &mut egui::Ui, text: &mut String) {
///     let _ = splat_design::layout::line_field(ui, splat_design::field_id!("doc_name"), "Name", text, "", 80.0);
/// }
/// ```
///
/// A `Once` reason is one of the listed words, not a text:
///
/// ```compile_fail,E0308
/// let _ = splat_design::field_id!("doc_name").once("Settings");
/// ```
///
/// An id or a place from the layout is not an owner's key:
///
/// ```compile_fail,E0277
/// fn f(ui: &mut egui::Ui) {
///     let _ = splat_design::field_id!("doc_name").of(ui.next_auto_id());
/// }
/// ```
///
/// ```compile_fail,E0277
/// fn f(ui: &mut egui::Ui) {
///     let _ = splat_design::field_id!("doc_name").of(ui.cursor().min.y as usize);
/// }
/// ```
///
/// A path or a name is not an identity: a save or a rename changes it while the DM types, and
/// the field loses its focus, cursor and Undo. Neither is a key (a path can only be the counted
/// escape `TextKey::from_data`):
///
/// ```compile_fail,E0277
/// let _ = splat_design::field_id!("doc_name").of(std::path::Path::new("Groups/Wolves.md"));
/// ```
///
/// ```compile_fail,E0277
/// let _ = splat_design::field_id!("doc_name").of(std::path::PathBuf::from("Groups/Wolves.md"));
/// ```
///
/// ```compile_fail,E0277
/// let _ = splat_design::field_id!("doc_name").of("Wolves");
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FieldName {
    name: &'static str,
}

impl FieldName {
    /// Use [`field_id!`](crate::field_id). Clippy disallows this call outside `src/design/`.
    #[doc(hidden)]
    pub const fn checked(name: &'static str) -> Self {
        let bytes = name.as_bytes();
        assert!(!bytes.is_empty(), "A field id is not empty.");
        let mut at = 0;
        while at < bytes.len() {
            assert!(
                bytes[at].is_ascii_lowercase() || bytes[at] == b'_' || bytes[at].is_ascii_digit(),
                "A field id is snake_case."
            );
            at += 1;
        }
        Self { name }
    }

    /// The field of one object: `key` is the identity of the object whose text the field edits
    /// (a `Uuid`, a [`TextOwner`]'s id), never something the field or a save changes (a path, a
    /// name) and never a place in the layout or a list.
    pub fn of(self, key: impl RowKey) -> FieldId {
        FieldId {
            name: self.name,
            key: Some(Id::new(key)),
        }
    }

    /// The field that exists once in the app, for `reason`.
    pub const fn once(self, reason: Once) -> FieldId {
        let _ = reason;
        FieldId {
            name: self.name,
            key: None,
        }
    }
}

/// A text field's id. It comes from a name in the source and its owner, never from the field's
/// place in the layout, so a branch that changes the layout around the field keeps its focus,
/// cursor and selection. Make one from a [`FieldName`] ([`field_id!`](crate::field_id), which
/// takes a string literal) with [`FieldName::of`] or [`FieldName::once`]; the source lint fails
/// when two `field_id!` names are the same. Two live fields with one id (the same name and key)
/// are the gate's `widget-id-clash` check.
///
/// The named parts ([`line_field`], [`labelled_field`], [`FieldId::search`], [`CreateField`])
/// add the field's visible name to its id, so one name given two labels (one helper that draws
/// three pickers) is three fields. The parts also drop egui's text Undo when the bound text
/// changed since the field last had focus (see [`FieldId::single_line`]).
///
/// ```
/// use splat_design::layout::{FieldName, RowIndex};
/// const NAME: FieldName = splat_design::field_id!("doc_name");
/// assert_ne!(NAME.of(RowIndex::from_data(1)).id(), NAME.of(RowIndex::from_data(2)).id());
/// ```
///
/// A row index is made only by its named constructor:
///
/// ```compile_fail,E0603
/// let _ = splat_design::layout::RowIndex(1);
/// ```
///
/// An `egui::Id` is not a [`RowKey`], and neither is a bare text:
///
/// ```compile_fail,E0277
/// const NAME: splat_design::layout::FieldName = splat_design::field_id!("doc_name");
/// let _ = NAME.of(egui::Id::new("place"));
/// ```
///
/// ```compile_fail,E0277
/// const NAME: splat_design::layout::FieldName = splat_design::field_id!("doc_name");
/// let _ = NAME.of(format!("{:?}", egui::Id::new("place")));
/// ```
///
/// An id from the layout is not a `FieldId`:
///
/// ```compile_fail,E0308
/// fn take(_: splat_design::layout::FieldId) {}
/// take(egui::Id::new("doc_name"));
/// ```
///
/// ```compile_fail,E0423
/// let _ = splat_design::layout::FieldId(egui::Id::new("doc_name"));
/// ```
///
/// ```compile_fail,E0080
/// let _ = splat_design::field_id!("");
/// ```
///
/// A name held in a constant escapes the lint's scan, so it does not compile:
///
/// ```compile_fail
/// const NAME: &str = "doc_name";
/// let _ = splat_design::field_id!(NAME);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FieldId {
    name: &'static str,
    key: Option<Id>,
}

impl FieldId {
    /// The same field for one part of its owner (a line of a tracker, an action of a line).
    pub fn with(self, key: impl RowKey) -> Self {
        let key = match self.key {
            Some(outer) => outer.with(key),
            None => Id::new(key),
        };
        Self {
            key: Some(key),
            ..self
        }
    }

    /// egui's id for this field as [`single_line`](Self::single_line),
    /// [`multi_line`](Self::multi_line) and [`editor`](Self::editor) draw it, to give or test its
    /// focus. A named part adds its name: use [`named_id`](Self::named_id) for those.
    pub fn id(self) -> Id {
        Id::new(("splat_field", self.name, self.key))
    }

    /// egui's id for this field as a named part draws it ([`line_field`], [`labelled_field`],
    /// [`FieldId::search`], [`CreateField`]), with `name`, the visible label or accessible name
    /// the part is given.
    pub fn named_id(self, name: &str) -> Id {
        self.named(name).id()
    }

    /// This field with its visible name in its id. One name with two labels is two fields.
    fn named(self, name: &str) -> Self {
        let key = match self.key {
            Some(outer) => outer.with(("named", name)),
            None => Id::new(("named", name)),
        };
        Self {
            key: Some(key),
            ..self
        }
    }

    /// A one-line field with this id that no shared part draws (a password, a code font).
    /// `style` sets the rest of the `TextEdit`; Clippy refuses another id on it outside
    /// `src/design/`. The field is drawn here, so the gate's `widget-id-clash` check sees it.
    ///
    /// The field parts (`single_line`, `multi_line`, `editor`, `search`, the parts drawn
    /// through them, and `number_field`; not the staged raw fields) guard their text history
    /// (`TextGuard`): when the bound text is not the
    /// text the field held when it last had focus (the program or another object replaced it),
    /// egui's text Undo for this id is dropped before Ctrl+Z can reach it (the cursor stays). So another object's
    /// words cannot come back through Undo, even when the owner's key is missing. One case stays
    /// with the owner's key: both texts are equal (A's text was emptied, B's is empty too).
    ///
    /// Caller duty: do not rewrite the bound text while the field has focus (a trim, a format, a
    /// clear in the frame of `changed()`); keep the typed words and store the cleaned value.
    #[allow(
        clippy::disallowed_methods,
        reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
    )]
    pub fn single_line(
        self,
        ui: &mut Ui,
        text: &mut dyn TextBuffer,
        style: impl for<'t> FnOnce(TextEdit<'t>) -> TextEdit<'t>,
    ) -> Response {
        let id = self.id();
        let guard = TextGuard::before(ui, id, Words::Plain(text.as_str()));
        let response = ui.add(style(TextEdit::singleline(text).id(id)));
        guard.after(ui, Words::Plain(text.as_str()), &response);
        record_field(ui, &response);
        response
    }

    /// A multi-line editor of a document with this id, for a caller that reads where the text
    /// was laid out (the galley, its place, the cursor, the clip rect): a Markdown source, a
    /// repair editor, a note. It guards the text history as [`single_line`](Self::single_line)
    /// does, by the draft's key: a quiet frame hashes no text (layout PR 9b).
    ///
    /// The part lays the text out in `face` and keeps the laid-out text by its whole key (the
    /// draft's identity and revision, the measure, the font atlas's epoch, the visuals, the
    /// face, and the caret for a face that folds): a quiet frame lays out, hashes and copies
    /// none of it in the app. egui's own per-frame text copies stay, counted (ruling M25).
    /// `style` sets the rest of the text edit; a layouter it sets is replaced by the part's.
    ///
    /// ```
    /// use splat_design::{pane::Face, text_draft::TextDraft};
    /// fn f(ui: &mut egui::Ui, note: &mut TextDraft) {
    ///     const NOTE: splat_design::layout::FieldName = splat_design::field_id!("doc_note");
    ///     let id = NOTE.once(splat_design::layout::Once::Example);
    ///     let face = Face::Font(egui::TextStyle::Body.resolve(ui.style()));
    ///     let editor = id.editor(ui, note, face, |edit| edit.desired_rows(4));
    ///     let _ = (editor.response, editor.galley, editor.galley_pos, editor.text_clip_rect);
    ///     let _ = editor.cursor_range;
    /// }
    /// ```
    #[allow(
        clippy::disallowed_methods,
        reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
    )]
    pub fn editor(
        self,
        ui: &mut Ui,
        text: &mut super::text_draft::TextDraft,
        face: super::pane::Face,
        style: impl for<'t> FnOnce(TextEdit<'t>) -> TextEdit<'t>,
    ) -> Editor {
        let id = self.id();
        #[cfg(any(test, feature = "test-support"))]
        DOCUMENTS_DRAWN.with(|count| count.set(count.get() + 1));
        let guard = TextGuard::before(ui, id, Words::Draft(text));
        // The draft is lent to egui for this draw: each change reaches `live`, which the kept
        // layouter reads on every call.
        let live = std::cell::Cell::new(text.key());
        let mut layouter = super::pane::kept_layouter(id, face, &live);
        let mut lent = super::text_draft::Lent {
            draft: text,
            live: &live,
        };
        let output = style(TextEdit::multiline(&mut lent).id(id))
            .layouter(&mut layouter)
            .show(ui);
        guard.after(ui, Words::Draft(lent.draft), &output.response);
        record_field(ui, &output.response);
        Editor {
            frame: output.response.rect,
            view: output.text_clip_rect,
            response: output.response,
            galley: output.galley,
            galley_pos: output.galley_pos,
            text_clip_rect: output.text_clip_rect,
            cursor_range: output.cursor_range,
        }
    }

    /// A search field with this id: the search icon, the hint (what can be searched) and, while
    /// it holds text, a quiet `Clear` inside the field's right end. `Clear` empties the text and
    /// gives the field its focus back; `cleared` says so, for a caller that resets more (the
    /// selected result). A search field has no form without its `Clear` (N10).
    ///
    /// The field's frame is exactly `width` (X2). `Clear`'s place is kept inside the frame's
    /// right end from the first frame (as a `Slot` keeps a value's widest form): the text area is
    /// the frame less that place, with and without text, so nothing moves or narrows while the
    /// DM types or deletes, and the hint has the same room in both states. Below
    /// [`search_min_row`] the frame still keeps to `width`; its text area gives way.
    ///
    /// ```
    /// fn f(ui: &mut egui::Ui, query: &mut String) {
    ///     use splat_design::layout::{FieldId, Once};
    ///     const SEARCH: FieldId = splat_design::field_id!("doc_search").once(Once::PageSearch);
    ///     let search = SEARCH.search(ui, "Search", query, "Search by name…", 240.0);
    ///     let _ = (search.field, search.cleared);
    /// }
    /// ```
    ///
    /// `Clear` is not the caller's to leave out:
    ///
    /// ```compile_fail,E0599
    /// fn f(ui: &mut egui::Ui, query: &mut String) {
    ///     use splat_design::layout::{FieldId, Once};
    ///     const SEARCH: FieldId = splat_design::field_id!("doc_search").once(Once::PageSearch);
    ///     let _ = SEARCH.search(ui, "Search", query, "Search by name…", 240.0).without_clear();
    /// }
    /// ```
    pub fn search(
        self,
        ui: &mut Ui,
        name: &str,
        text: &mut String,
        hint: &str,
        width: f32,
    ) -> Search {
        let mut cleared = false;
        let id = self.named(name).id();
        // Clear's place, kept inside the frame's right end whether or not Clear shows.
        let place = search_clear().natural_width(ui);
        let guard = TextGuard::before(ui, id, Words::Plain(text));
        let field = super::components::search_box(ui, id, name, text, hint, width, place);
        guard.after(ui, Words::Plain(text), &field);
        record_field(ui, &field);
        if !text.is_empty() {
            let frame = field.rect;
            let at = Rect::from_min_max(
                egui::pos2((frame.right() - place).max(frame.left()), frame.top()),
                frame.right_bottom(),
            );
            let mut inside = ui.new_child(
                UiBuilder::new()
                    .max_rect(at)
                    .layout(Layout::left_to_right(Align::Center)),
            );
            let clear = search_clear().show(&mut inside);
            super::components::register_field_action(ui, field.id, clear.id);
            if clear.clicked() {
                text.clear();
                field.request_focus();
                cleared = true;
            }
        }
        Search { field, cleared }
    }

    /// A multi-line field with this id that no shared part draws.
    #[allow(
        clippy::disallowed_methods,
        reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
    )]
    pub fn multi_line(
        self,
        ui: &mut Ui,
        text: &mut dyn TextBuffer,
        style: impl for<'t> FnOnce(TextEdit<'t>) -> TextEdit<'t>,
    ) -> Response {
        let id = self.id();
        let guard = TextGuard::before(ui, id, Words::Plain(text.as_str()));
        let response = ui.add(style(TextEdit::multiline(text).id(id)));
        guard.after(ui, Words::Plain(text.as_str()), &response);
        record_field(ui, &response);
        response
    }
}

/// The smallest row a search field is usable in: its margins, `Clear`'s place and a 24 px text
/// area. Below it the field still keeps inside the width it is given: its text area gives way.
pub fn search_min_row(ui: &Ui) -> f32 {
    use super::components::{SEARCH_LEFT, SEARCH_RIGHT};
    SEARCH_LEFT + SEARCH_RIGHT + search_clear().natural_width(ui) + 24.0
}

/// The search row's `Clear`.
fn search_clear() -> Action<'static> {
    Action::quiet("Clear").density(Density::Scan)
}

/// What [`FieldId::editor`] drew: the field and where its text was laid out. The fields are
/// egui's `TextEditOutput` fields of the same names.
#[must_use = "read `response` for changes and focus"]
pub struct Editor {
    pub response: Response,
    /// The laid-out text.
    pub galley: std::sync::Arc<egui::Galley>,
    /// Where the galley's top left corner was painted.
    pub galley_pos: egui::Pos2,
    /// The rect the text is clipped to (the visible part of the text).
    pub text_clip_rect: Rect,
    /// The cursor or selection, while the field has one.
    pub cursor_range: Option<egui::text::CCursorRange>,
    /// The editor's outside frame: the field itself here; the fixed frame of an editor viewport
    /// (`pane::EditorRegion::editor`, `pane::editor_view`), whose text scrolls inside it.
    pub frame: Rect,
    /// The part of the text in view: the text's own rect here; the viewport's inner scroll view
    /// for an editor viewport.
    pub view: Rect,
}

/// What the bound text was when the field last drew with focus: its length and a hash.
#[derive(Clone, Copy, PartialEq, Eq)]
struct BoundText {
    len: usize,
    hash: u64,
}

/// A field's words, as the guard reads them: a plain text, or a document
/// (`text_draft::TextDraft`), whose key says when its words last changed. The guard compares
/// words in both (another object's text with equal words keeps its history, as a plain text
/// did), but hashes a document's words only when its key changed since its record (layout PR
/// 9b: a quiet frame of a document hashes nothing).
#[derive(Clone, Copy)]
enum Words<'a> {
    Plain(&'a str),
    Draft(&'a super::text_draft::TextDraft),
}

impl Words<'_> {
    fn key(self) -> Option<super::text_draft::DraftKey> {
        match self {
            Self::Plain(_) => None,
            Self::Draft(draft) => Some(draft.key()),
        }
    }

    /// One hash of the words.
    fn bound(self) -> BoundText {
        match self {
            Self::Plain(text) => BoundText::of(text),
            Self::Draft(draft) => BoundText::of_draft(draft),
        }
    }

    /// Whether a record of `bound` read at `key` holds these words. A document whose key is
    /// the record's has the record's words: no hash.
    fn held_by(self, bound: BoundText, key: Option<super::text_draft::DraftKey>) -> bool {
        (key.is_some() && key == self.key()) || bound == self.bound()
    }
}

impl BoundText {
    /// One hash of the text (egui itself copies a focused field's whole text into its Undo
    /// history every frame it has focus, so this adds less than egui does).
    fn of(text: &str) -> Self {
        #[cfg(any(test, feature = "test-support"))]
        {
            BOUND_TEXT_HASHES.with(|count| count.set(count.get() + 1));
            BOUND_TEXT_BYTES.with(|count| count.set(count.get() + text.len()));
        }
        Self {
            len: text.len(),
            hash: egui::util::hash(text),
        }
    }

    /// [`of`](Self::of) a document's words, counted apart: a short field keeps its hash (ruling
    /// Q3), a document's quiet frame hashes none.
    fn of_draft(draft: &super::text_draft::TextDraft) -> Self {
        #[cfg(any(test, feature = "test-support"))]
        DRAFT_TEXT_BYTES.with(|count| count.set(count.get() + draft.len()));
        Self::of(draft)
    }
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// How many times a field part hashed its text, for the cost proof.
    pub(crate) static BOUND_TEXT_HASHES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    /// How many bytes of text the field parts hashed (layout PR 9b's quiet-frame proof).
    pub static BOUND_TEXT_BYTES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    /// How many times a document editor drew (`FieldId::editor`): a quiet-frame proof first
    /// shows that the host's editor was there.
    pub static DOCUMENTS_DRAWN: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    /// How many of those bytes were a document's words (`text_draft::TextDraft`).
    pub static DRAFT_TEXT_BYTES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// How many fields keep a record. The oldest record goes first, and egui's Undo history for
/// its field goes with it, so a field with no record never has old history.
pub(crate) const BOUND_TEXTS: usize = 32;

/// The records, newest last: each field's words, and for a document the key they were read at.
/// At most [`BOUND_TEXTS`].
#[derive(Clone, Default)]
struct BoundTexts(Vec<(Id, BoundText, Option<super::text_draft::DraftKey>)>);

fn bound_texts_id() -> Id {
    Id::new("splat_bound_texts")
}

/// How many records the guard holds, for the part test.
#[cfg(test)]
pub(crate) fn bound_text_count(ctx: &egui::Context) -> usize {
    ctx.data(|data| data.get_temp::<BoundTexts>(bound_texts_id()))
        .map_or(0, |records| records.0.len())
}

/// The field parts' guard of egui's text Undo. egui keeps a field's cursor and Undo history
/// under its id. If the field's text is not the text it held when it last had focus, someone
/// else's history is under this id (another object's, when the caller left out the owner's
/// key; or the program replaced the text): drop it before Ctrl+Z can bring it back.
///
/// When it checks, before the draw:
/// - the field has focus: every frame (egui adds Undo history only then);
/// - the field has a record and no focus: on a frame with a pointer press or a key press, the
///   only frames in which focus can come and Ctrl+Z run inside egui's draw. An idle frame, or
///   one with only pointer moves, hashes nothing.
///
/// A field that is not drawn for a while (its pane closed) keeps its record, so the same check
/// runs when it comes back. A dropped history removes the record. At most [`BOUND_TEXTS`] fields
/// keep a record; the oldest goes first, with its egui history.
///
/// One case passes: the old and new texts are equal (A's text was emptied, B's is empty). The
/// owner's key ([`FieldName::of`]) covers that one. A caller does not rewrite the bound text
/// while the field has focus (a trim, a format, a clear in the frame of `changed()`): the guard
/// takes that for another text and drops the DM's own Undo.
#[must_use]
struct TextGuard {
    id: Id,
    focused_before: bool,
}

impl TextGuard {
    fn before(ui: &Ui, id: Id, text: Words<'_>) -> Self {
        let focused_before = ui.memory(|memory| memory.has_focus(id));
        let check = focused_before
            || ui.input(|input| {
                input.pointer.any_pressed()
                    || input
                        .events
                        .iter()
                        .any(|event| matches!(event, egui::Event::Key { pressed: true, .. }))
            });
        if check && Self::stale(ui, id, text) {
            // The cursor stays: a caller may have set it for this text (select all on open), and
            // egui keeps a cursor inside the text it draws.
            Self::drop_history(ui.ctx(), id);
            Self::forget(ui.ctx(), id);
        }
        Self { id, focused_before }
    }

    fn after(self, ui: &Ui, text: Words<'_>, response: &Response) {
        if !response.has_focus() {
            return;
        }
        // Focus came this frame by a way the check before the draw did not see: egui may have
        // fed the old history with this text.
        if !self.focused_before && Self::stale(ui, self.id, text) {
            Self::drop_history(ui.ctx(), self.id);
        }
        // A document whose key is its record's keeps the record's words, with no hash.
        let ctx = ui.ctx();
        let kept = ctx.data(|data| {
            data.get_temp::<BoundTexts>(bound_texts_id())
                .and_then(|records| {
                    records
                        .0
                        .iter()
                        .find(|(id, _, key)| *id == self.id && key.is_some() && *key == text.key())
                        .map(|(_, bound, _)| *bound)
                })
        });
        let bound = kept.unwrap_or_else(|| text.bound());
        let oldest = ctx.data_mut(|data| {
            let records = &mut data
                .get_temp_mut_or_default::<BoundTexts>(bound_texts_id())
                .0;
            records.retain(|(id, _, _)| *id != self.id);
            records.push((self.id, bound, text.key()));
            (records.len() > BOUND_TEXTS).then(|| records.remove(0).0)
        });
        if let Some(oldest) = oldest {
            Self::drop_history(ctx, oldest);
        }
    }

    /// Whether the field last had focus with another text.
    fn stale(ui: &Ui, id: Id, text: Words<'_>) -> bool {
        let Some((bound, key)) = ui.ctx().data(|data| {
            data.get_temp::<BoundTexts>(bound_texts_id())
                .and_then(|records| {
                    records
                        .0
                        .iter()
                        .find(|(record, _, _)| *record == id)
                        .map(|(_, bound, key)| (*bound, *key))
                })
        }) else {
            return false;
        };
        !text.held_by(bound, key)
    }

    fn forget(ctx: &egui::Context, id: Id) {
        ctx.data_mut(|data| {
            data.get_temp_mut_or_default::<BoundTexts>(bound_texts_id())
                .0
                .retain(|(record, _, _)| *record != id);
        });
    }

    fn drop_history(ctx: &egui::Context, id: Id) {
        if let Some(mut state) = egui::text_edit::TextEditState::load(ctx, id) {
            state.clear_undoer();
            state.store(ctx, id);
        }
    }
}

/// The guarded draw of a field that a part outside this file draws (a number field): `draw`
/// draws the `TextEdit` with `id` and the text.
pub(super) fn guarded_field(
    ui: &mut Ui,
    id: Id,
    text: &mut String,
    draw: impl FnOnce(&mut Ui, &mut String) -> Response,
) -> Response {
    let guard = TextGuard::before(ui, id, Words::Plain(text));
    let response = draw(ui, text);
    guard.after(ui, Words::Plain(text), &response);
    response
}

/// The field with `id` edits its own text by `edit`: a command typed in it takes its words
/// out, words go in at its caret. A guarded field takes a text it did not draw for another
/// object's and drops its Undo history; this edit keeps it, so Ctrl+Z still goes back through
/// what the DM typed before. The guard's record moves to the edited text only when it holds
/// the text before the edit, so a replacement that came first (a reload, another object's
/// text) still drops the history on the next frame. Not for another object's text, or for a
/// whole new text: that history must go.
pub fn edit_in_place(
    ctx: &egui::Context,
    id: Id,
    text: &mut super::text_draft::TextDraft,
    edit: impl FnOnce(&mut String),
) {
    let before = BoundText::of_draft(text);
    text.edit(edit);
    move_bound(ctx, id, before, BoundText::of_draft(text), Some(text.key()));
}

/// A write of the app's own into the words of the field `id`, which the DM may be typing in (a
/// reading's summary, a plan's rewards, a renamed link, all reaching the Notes panel's part of
/// a note): the words become `after`, and the DM's Undo history goes on UNDER the write. Each
/// earlier step gets the same change, so Ctrl+Z takes back what the DM typed and leaves the
/// app's words.
///
/// The change is placed in a step **by the DM's own edits** between that step and now (issue
/// #900), never by equal words and never by a caret: each step is a whole text, so the letters
/// the DM changed between one step and the next are known as a zone (`change_between`,
/// `Write::before`), and the write's place moves back over each zone. A step is kept only when
/// the write's place in it is certain. Where it is not (the write lies in the zone: among
/// letters equal to ones the DM typed or removed, over words the DM replaced, or over a part of
/// the words the DM typed), that step and every older one go. That loses Undo steps, never
/// text: Ctrl+Z never takes the app's words out, never puts a word back on the wrong side of
/// them, and never takes out other words. The DM's Redo steps end at the write, as at any new
/// edit. The guard's record moves as in [`edit_in_place`]. Not for a change the field's owner
/// did not make (a reload of an outside edit): that history must end.
pub fn write_under_history(
    ctx: &egui::Context,
    id: Id,
    text: &mut super::text_draft::TextDraft,
    after: &str,
) {
    let before: Vec<char> = text.as_str().chars().collect();
    edit_in_place(ctx, id, text, |words| {
        words.clear();
        words.push_str(after);
    });
    let Some(mut state) = egui::text_edit::TextEditState::load(ctx, id) else {
        return;
    };
    // The steps, newest first: from a state that is no step, each `undo` gives the step before.
    let mut walk = state.undoer();
    let mut current = (
        egui::text::CCursorRange::one(egui::text::CCursor::new(usize::MAX)),
        String::new(),
    );
    let mut steps = Vec::new();
    while let Some(step) = walk.undo(&current).cloned() {
        steps.push(step.clone());
        current = step;
    }
    let after: Vec<char> = after.chars().collect();
    // The write in the words before it.
    let Some(mut write) = Write::between(&before, &after) else {
        // The same words: no step changes.
        return;
    };
    // From the newest step back: the text after each step's edit.
    let mut newer = before;
    if steps.is_empty() {
        return;
    }
    let mut kept = Vec::new();
    for (cursor, words) in steps {
        let words: Vec<char> = words.chars().collect();
        let edit = change_between(&words, &newer);
        let Some(moved) = write.before(edit, &words, &newer) else {
            break;
        };
        write = moved;
        let place = |cursor: egui::text::CCursor| egui::text::CCursor {
            index: write.place(cursor.index),
            ..cursor
        };
        kept.push((
            egui::text::CCursorRange {
                primary: place(cursor.primary),
                secondary: place(cursor.secondary),
                ..cursor
            },
            write.apply(&words, &after),
        ));
        newer = words;
    }
    // The newest step that is the text as it now stands keeps the field's caret, which the write
    // does not move: egui takes a step with another caret for another state, and the first
    // Ctrl+Z would only move the caret.
    if let (Some((cursor, words)), Some(range)) = (kept.first_mut(), state.cursor.char_range())
        && words.chars().eq(after.iter().copied())
    {
        *cursor = range;
    }
    // A step the write cannot reach, and every older one, is not kept (review of PR 825, P1
    // 825-b): Ctrl+Z to it would take the app's words out.
    let mut rebuilt = egui::util::undoer::Undoer::default();
    for step in kept.iter().rev() {
        rebuilt.add_undo(step);
    }
    state.set_undoer(rebuilt);
    state.store(ctx, id);
}

/// One change of a text, in characters: the `old` characters at `at` became `new` characters.
/// `slack`: typed or removed words that equal letters around them hide can also stand up to
/// that many places later.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Change {
    at: usize,
    old: usize,
    new: usize,
    slack: usize,
}

/// The first and the last place a change from `from` to `to` can stand, with what it takes out
/// and puts in: `(first, last, old, new)`. A change that only adds or only removes characters
/// can stand at several places when equal characters lie around it (`aa` to `a`); any other
/// change is its hull, at one place.
fn change_span(from: &[char], to: &[char]) -> (usize, usize, usize, usize) {
    let short = from.len().min(to.len());
    let head = from
        .iter()
        .zip(to)
        .take_while(|(one, other)| one == other)
        .count();
    let tail = from
        .iter()
        .rev()
        .zip(to.iter().rev())
        .take_while(|(one, other)| one == other)
        .count();
    if head + tail >= short {
        (
            short - tail.min(short),
            head.min(short),
            from.len() - short,
            to.len() - short,
        )
    } else {
        (head, head, from.len() - head - tail, to.len() - head - tail)
    }
}

/// The DM's edit from the step `from` to the text `to` after it. A step is a whole text, so
/// equal characters can hide where words were typed (`a` to `aa`) or removed (`aa` to `a`).
/// The edit then keeps every place it can stand at (`Change::slack`), and a write is placed only
/// when it is certain for all of them (`Write::before`).
///
/// **No caret is evidence of where an edit was** (review of PR 910, P1 910-c): not the field's
/// caret now, and not the caret egui stored with a step. The DM can move the caret after an
/// edit and before the step is made (an arrow key, Home, a click), and a step can hold several
/// edits. A caret taken as the place of removed words put a word the DM got back on the wrong
/// side of the app's words (issue #900).
fn change_between(from: &[char], to: &[char]) -> Change {
    let (first, last, old, new) = change_span(from, to);
    if old == 0 && new == 0 {
        // The same words: no edit (the step is the text as it stands).
        return Change {
            at: 0,
            old,
            new,
            slack: 0,
        };
    }
    Change {
        at: first,
        old,
        new,
        slack: last - first,
    }
}

/// The app's write as it stands in one text: the `old` characters at `at` become the write's
/// new words. The app gives the new words and no range, so the write is the smallest change
/// between the two texts, at the last place it can stand (`Write::between`).
#[derive(Clone, Copy, Debug)]
struct Write {
    at: usize,
    old: usize,
    /// From where in the written text its new words come, and how many.
    from: usize,
    new: usize,
}

impl Write {
    /// The write from `before` to `after`. None for no change.
    fn between(before: &[char], after: &[char]) -> Option<Self> {
        let (_, last, old, new) = change_span(before, after);
        (old > 0 || new > 0).then_some(Self {
            at: last,
            old,
            from: last,
            new,
        })
    }

    /// This write in the text `step` before the DM's `edit` (it stands in `newer`, the text
    /// after the edit). None when its place there is not certain.
    ///
    /// **What is certain.** A step is a whole text. The edit from `step` to `newer` is known
    /// only as the letters between the two texts' common start and common end (with every place
    /// equal letters let it stand at: `Change::slack`). A step can hold several edits, so the
    /// DM's own letters can also be among the letters next to those, wherever a letter there is
    /// one the DM put in or took out: `a` to `a bbb` is `bb` typed at the end and a space typed
    /// before the older `b`, and the texts alone do not say which `b` is the older one. So the
    /// edit's ZONE is those letters and, on each side, every next letter that the DM put in or
    /// took out somewhere in the edit, up to the first letter that is none of them. Outside the
    /// zone every letter is an older letter at a known place.
    ///
    /// - A write wholly outside the zone (it may touch its end) has one place in `step`.
    /// - A write in the zone has one only when the DM typed words and took none out, the zone is
    ///   exactly those words, and the write lies whole in them or holds them whole: it takes
    ///   them with it, and what is left of it stands where they were typed.
    /// - Any other write in the zone has no certain place: over words the DM replaced; over a
    ///   part of the typed words (a renamed link in them, with a summary above, is one change
    ///   from the first changed letter to the last: placed, it would leave the other part as a
    ///   broken piece); among equal letters (issue #900: a write placed there by the words
    ///   after it, or by a caret, took out an older equal word, or left a word that came back
    ///   on the wrong side of the app's words).
    fn before(self, edit: Change, step: &[char], newer: &[char]) -> Option<Self> {
        if edit.old == 0 && edit.new == 0 {
            return Some(self);
        }
        let (start, end) = (self.at, self.at + self.old);
        // The edit's letters in `newer`, and the letters the DM put in or took out.
        let (from, to) = (edit.at, edit.at + edit.slack + edit.new);
        let mut touched: Vec<char> = newer[from..to]
            .iter()
            .chain(&step[edit.at..edit.at + edit.slack + edit.old])
            .copied()
            .collect();
        touched.sort_unstable();
        touched.dedup();
        let touches = |letter: &char| touched.binary_search(letter).is_ok();
        let zone_from = from
            - newer[..from]
                .iter()
                .rev()
                .take_while(|letter| touches(letter))
                .count();
        let zone_to = to
            + newer[to..]
                .iter()
                .take_while(|letter| touches(letter))
                .count();
        if end <= zone_from {
            return Some(self);
        }
        if start >= zone_to {
            // The write moves with the words after the edit.
            let at = self.at + step.len() - newer.len();
            return Some(Self { at, ..self });
        }
        let only_typed = edit.old == 0 && edit.slack == 0 && (zone_from, zone_to) == (from, to);
        let whole = (from >= start && to <= end) || (start >= from && end <= to);
        if !only_typed || !whole {
            return None;
        }
        let at = start.min(from);
        let rest = end.saturating_sub(to);
        Some(Self {
            at,
            old: from - at + rest,
            ..self
        })
    }

    /// `step` with this write: its new words, taken from the written text `after`.
    fn apply(self, step: &[char], after: &[char]) -> String {
        step[..self.at]
            .iter()
            .chain(&after[self.from..self.from + self.new])
            .chain(&step[self.at + self.old..])
            .collect()
    }

    /// Where a caret at `index` of the step stands after this write: one inside the changed
    /// words goes to their end.
    fn place(self, index: usize) -> usize {
        if index <= self.at {
            index
        } else if index >= self.at + self.old {
            index - self.old + self.new
        } else {
            self.at + self.new
        }
    }
}

/// [`edit_in_place`] for a field of a plain `String` (a single-line field).
pub fn edit_line_in_place(
    ctx: &egui::Context,
    id: Id,
    text: &mut String,
    edit: impl FnOnce(&mut String),
) {
    let before = BoundText::of(text);
    edit(text);
    move_bound(ctx, id, before, BoundText::of(text), None);
}

/// The guard's record of field `id` moves from `before` to `after`, when it holds `before`.
fn move_bound(
    ctx: &egui::Context,
    id: Id,
    before: BoundText,
    after: BoundText,
    key: Option<super::text_draft::DraftKey>,
) {
    ctx.data_mut(|data| {
        if let Some(record) = data
            .get_temp_mut_or_default::<BoundTexts>(bound_texts_id())
            .0
            .iter_mut()
            .find(|(record, bound, _)| *record == id && *bound == before)
        {
            (record.1, record.2) = (after, key);
        }
    });
}

/// What [`FieldId::search`] drew: the field, and whether `Clear` emptied it this frame.
#[must_use = "read `field` for focus and `cleared` to reset what the search chose"]
pub struct Search {
    pub field: Response,
    pub cleared: bool,
}

/// The text fields the design drew in one pass.
#[derive(Clone, Default)]
struct FieldsDrawn {
    pass: u64,
    fields: Vec<(Id, Rect)>,
}

fn fields_drawn_id() -> Id {
    Id::new("splat_fields_drawn")
}

/// Note a field drawn in this pass, for [`field_id_clashes`].
fn record_field(ui: &Ui, response: &Response) {
    if ui.is_sizing_pass() {
        return;
    }
    let pass = ui.ctx().cumulative_pass_nr();
    ui.ctx().data_mut(|data| {
        let drawn = data.get_temp_mut_or_default::<FieldsDrawn>(fields_drawn_id());
        if drawn.pass != pass {
            drawn.pass = pass;
            drawn.fields.clear();
        }
        drawn.fields.push((response.id, response.rect));
    });
}

/// The ids of the last pass's text fields that another live widget shares: a field drawn twice
/// with one id, or a field whose record in egui another widget with its id took over (egui keeps
/// one record per id, the last one; it warns of a clash only when neither rect holds the other,
/// so this finds the cases it does not warn of). A widget drawn before a field, with the field's
/// id and inside the field's rect, is the one case neither egui nor this sees.
pub fn field_id_clashes(ctx: &egui::Context) -> Vec<Id> {
    let drawn = ctx
        .data(|data| data.get_temp::<FieldsDrawn>(fields_drawn_id()))
        .unwrap_or_default();
    // Only the pass that just ended counts; a record from an older pass is stale.
    if drawn.pass + 1 < ctx.cumulative_pass_nr() {
        return Vec::new();
    }
    let mut clashes = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for (id, _) in &drawn.fields {
        if !seen.insert(*id) {
            clashes.push(*id);
        }
    }
    ctx.viewport(|viewport| {
        for (id, rect) in &drawn.fields {
            if viewport
                .this_pass
                .widgets
                .get(*id)
                .is_some_and(|widget| widget.rect != *rect)
            {
                clashes.push(*id);
            }
        }
    });
    clashes.sort_by_key(|id| id.value());
    clashes.dedup();
    clashes
}

/// The gate's `widget-id-clash` check for one state: egui's clash warnings, read from every
/// painted shape with no visibility filter (a warning at the pane's edge counts), and the text
/// fields that [`field_id_clashes`] finds. Each finding is one line of words.
pub fn widget_id_clashes(
    ctx: &egui::Context,
    shapes: &[egui::epaint::ClippedShape],
) -> Vec<String> {
    fn texts(shape: &egui::Shape, out: &mut Vec<String>) {
        match shape {
            egui::Shape::Text(text) => out.push(text.galley.job.text.clone()),
            egui::Shape::Vec(shapes) => {
                for shape in shapes {
                    texts(shape, out);
                }
            }
            _ => {}
        }
    }
    let mut painted = Vec::new();
    for clipped in shapes {
        texts(&clipped.shape, &mut painted);
    }
    let mut found: Vec<String> = painted
        .into_iter()
        .filter(|text| text.contains("use of widget ID"))
        .collect();
    found.extend(field_id_clashes(ctx).into_iter().map(|id| {
        format!(
            "Two live widgets share the text field id {}",
            id.short_debug_format()
        )
    }));
    found
}

/// A [`FieldName`] from a string literal, checked when the program compiles. Give it its owner
/// with [`FieldName::of`] or [`FieldName::once`] before it can draw.
#[macro_export]
macro_rules! field_id {
    ($name:literal) => {
        const {
            #[allow(clippy::disallowed_methods, reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)")]
            let id = $crate::layout::FieldName::checked($name);
            id
        }
    };
}

/// A short one-line field with an accessible name, a placeholder and a target of at least 26 px.
pub fn line_field(
    ui: &mut Ui,
    id: FieldId,
    name: &str,
    text: &mut String,
    hint: &str,
    width: f32,
) -> Response {
    let hint = placeholder(ui, hint);
    // The desired width excludes the margins.
    let response = id.named(name).single_line(ui, text, |edit| {
        edit.desired_width(width - 2.0 * S)
            .min_size(vec2(width, 26.0))
            .margin(vec2(S, 4.0))
            .hint_text(hint)
    });
    named(ui, &response, name);
    response
}

/// A one-line field with its label above it, and help and an error below it.
pub fn labelled_field(
    ui: &mut Ui,
    id: FieldId,
    label: &str,
    text: &mut String,
    help: Option<&str>,
    error: Option<&str>,
) -> Response {
    field(ui, label, help, error, |ui| {
        let width = ui.available_width().min(FORM_WIDTH) - 2.0 * S;
        form_field(ui, id, label, text, None, width)
    })
}

/// The form field of `labelled_field` and `InlineCreate`, `width` wide without its margins.
fn form_field(
    ui: &mut Ui,
    id: FieldId,
    label: &str,
    text: &mut String,
    hint: Option<&str>,
    width: f32,
) -> Response {
    let hint = hint.map(|hint| placeholder(ui, hint));
    let response = id.named(label).single_line(ui, text, |edit| {
        let edit = edit
            .desired_width(width)
            .min_size(vec2(0.0, Density::Read.control()))
            .margin(vec2(S, 5.0));
        match hint {
            Some(hint) => edit.hint_text(hint),
            None => edit,
        }
    });
    named(ui, &response, label);
    response
}

/// The lane a scroll column always keeps: the app's solid bar and its margin. It comes from the
/// design's scroll style, which `ScrollColumn` sets on its own `Ui`, never from a caller's.
pub fn lane_width() -> f32 {
    theme::scroll_style().allocated_width()
}

/// A vertical scroll area that keeps the scroll bar's lane whether or not the bar shows: its
/// content always ends one lane before the area's right edge, so nothing moves when the bar
/// comes or goes. It sets the app's solid scroll style on its own `Ui`, so a caller's style
/// (floating bars that take no width) cannot remove the lane. The caller has no lane to keep.
///
/// ```
/// fn list(ui: &mut egui::Ui) {
///     splat_design::layout::ScrollColumn::new("list")
///         .max_height(200.0)
///         .show(ui, |ui| ui.label("Row"));
/// }
/// ```
///
/// There is no way to give the lane back:
///
/// ```compile_fail,E0599
/// let _ = splat_design::layout::ScrollColumn::new("list").without_lane();
/// ```
#[must_use = "call show"]
pub struct ScrollColumn {
    id: Id,
    max_height: Option<f32>,
    min_scrolled_height: Option<f32>,
    fit_height: bool,
    offset: Option<f32>,
    sideways: bool,
    full_width: bool,
    hosts_reveals: bool,
    follows_focus: bool,
}

impl ScrollColumn {
    pub fn new(id_salt: impl Hash) -> Self {
        Self {
            id: Id::new(id_salt),
            max_height: None,
            min_scrolled_height: None,
            fit_height: false,
            offset: None,
            sideways: false,
            full_width: false,
            hosts_reveals: false,
            follows_focus: false,
        }
    }

    /// A column outside a pane (the session banner): a control of it that gains keyboard focus
    /// comes whole into its view by the pane's focus rules (`pane::follow_focus`), so Tab never
    /// reaches a control the column hides. In a pane's body the pane is that one arbiter, and
    /// this does nothing.
    pub fn follows_focus(mut self) -> Self {
        self.follows_focus = true;
        self
    }

    /// R46: this column may host a reveal (`design::reveal`). Its programmatic scrolls land in
    /// the pass that handles them and egui stores no target for them, so no older request (a
    /// focus scroll, a caret follow) lands after a reveal ended. A reveal asked from any other
    /// column ends `Unowned`; those columns keep egui's one delayed jump (R43).
    pub fn hosts_reveals(mut self) -> Self {
        self.hosts_reveals = true;
        self
    }

    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = Some(height);
        self
    }

    pub fn min_scrolled_height(mut self, height: f32) -> Self {
        self.min_scrolled_height = Some(height);
        self
    }

    /// As tall as its content up to `max_height`, not the whole height it is given.
    pub fn fit_height(mut self) -> Self {
        self.fit_height = true;
        self
    }

    pub fn vertical_scroll_offset(mut self, offset: f32) -> Self {
        self.offset = Some(offset);
        self
    }

    /// The content takes the column's whole width even when it is narrower: a list whose rows
    /// fill the column. Without it the column gives its width only as the most the content may
    /// take, so a form that narrows itself (`set_max_width`) keeps its own width (K2, gap K-6).
    pub fn full_width(mut self) -> Self {
        self.full_width = true;
        self
    }

    /// Also scroll sideways, for content wider than the pane (a wide table). The vertical
    /// lane stays kept.
    pub fn sideways_too(mut self) -> Self {
        self.sideways = true;
        self
    }

    #[allow(
        clippy::disallowed_methods,
        reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
    )]
    fn area(&self) -> egui::ScrollArea {
        let mut area = if self.sideways {
            egui::ScrollArea::both()
        } else {
            egui::ScrollArea::vertical()
        }
        .id_salt(self.id)
        .auto_shrink([false, self.fit_height])
        .animated(!self.hosts_reveals);
        if let Some(height) = self.max_height {
            area = area.max_height(height);
        }
        if let Some(height) = self.min_scrolled_height {
            area = area.min_scrolled_height(height);
        }
        if let Some(offset) = self.offset {
            area = area.vertical_scroll_offset(offset);
        }
        area
    }

    /// The area for this pass in `ui` (the `Ui` it is shown in), and its persistent key for a
    /// reveal (`design::reveal`): a reveal's offset, written in the last frame, wins once.
    fn area_in(&self, ui: &Ui) -> (egui::ScrollArea, super::reveal::Column) {
        let mut column = super::reveal::Column {
            key: ui.make_persistent_id(("design_reveal_column", self.id)),
            hosts: self.hosts_reveals,
            area: ui.make_persistent_id(self.id),
            applied: None,
        };
        let mut area = self.area();
        // The column's queued request (a region's or a jump: layout 11c) waits while the column's
        // own correction (a held press, `from_top`) sets the offset.
        column.applied = self
            .offset
            .is_none()
            .then(|| super::reveal::take_queued(ui, column.key, column.area))
            .flatten();
        if let Some(offset) = column
            .applied
            .or_else(|| super::reveal::take_offset(ui, column))
        {
            area = area.vertical_scroll_offset(offset);
        }
        (area, column)
    }

    /// Run `add` in a child `Ui` with the app's scroll style, and the content's width: the
    /// width the column is given, less the lane.
    fn styled<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui, f32) -> R) -> R {
        ui.scope(|ui| {
            ui.spacing_mut().scroll = theme::scroll_style();
            let width = (ui.available_width() - lane_width()).max(0.0);
            add(ui, width)
        })
        .inner
    }

    pub fn show<R>(self, ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> ScrollAreaOutput<R> {
        if super::pane::column_in_content(ui) {
            return self.inline(ui, add);
        }
        let (sideways, full, id) = (self.sideways, self.full_width, self.id);
        let follows = self.follows_focus;
        // In a pane's body, the area starts where a `line_start` hang may reach and its content
        // at the text edge (P3a): the surface covers the hang, and nothing moves.
        let Some(reach) = super::pane::column_reach(ui, lane_width()) else {
            let output = Self::styled(ui, |ui, width| {
                let (area, column) = self.area_in(ui);
                area.show_viewport(ui, |ui, viewport| {
                    super::reveal::region(ui, column, viewport, |ui| {
                        if !sideways {
                            ui.set_max_width(width);
                        }
                        if full {
                            ui.set_min_width(width);
                        }
                        super::pane::whole_rows(ui, id, |ui| {
                            super::pane::follow_focus(ui, follows, id, add)
                        })
                    })
                })
            });
            // A column in a child of composite columns adds what it hides to the child's height.
            super::pane::hidden_height(ui, &output);
            return output;
        };
        let output = super::pane::reaching(ui, reach, |ui| {
            Self::styled(ui, |ui, width| {
                let (area, column) = self.area_in(ui);
                area.show_viewport(ui, |ui, viewport| {
                    super::reveal::region(ui, column, viewport, |ui| {
                        if !sideways {
                            ui.set_max_width(width);
                        }
                        if full {
                            ui.set_min_width(width);
                        }
                        super::pane::whole_rows(ui, id, |ui| {
                            super::pane::inset(ui, reach, |ui| {
                                if full {
                                    ui.set_min_width(ui.max_rect().width());
                                }
                                add(ui)
                            })
                        })
                    })
                })
            })
        });
        super::pane::hidden_height(ui, &output);
        output
    }

    /// The scroll of a pane's editor region (`pane::EditorRegion`), the one scroll a `Content`
    /// pane holds: the region refuses a scroll inside it, so it is never refused itself.
    pub(super) fn show_in_region<R>(
        self,
        ui: &mut Ui,
        add: impl FnOnce(&mut Ui) -> R,
    ) -> ScrollAreaOutput<R> {
        Self::styled(ui, |ui, width| {
            let (area, column) = self.area_in(ui);
            area.show_viewport(ui, |ui, viewport| {
                super::reveal::region(ui, column, viewport, |ui| {
                    ui.set_max_width(width);
                    add(ui)
                })
            })
        })
    }

    /// For a long list of rows of one height: only the rows in view are drawn.
    pub fn show_rows<R>(
        self,
        ui: &mut Ui,
        row_height: f32,
        rows: usize,
        add: impl FnOnce(&mut Ui, Range<usize>) -> R,
    ) -> ScrollAreaOutput<R> {
        if super::pane::column_in_content(ui) {
            return self.inline(ui, |ui| add(ui, 0..rows));
        }
        let output = Self::styled(ui, |ui, width| {
            let (full, id) = (self.full_width, self.id);
            self.area().show_rows(ui, row_height, rows, |ui, range| {
                ui.set_max_width(width);
                if full {
                    ui.set_min_width(width);
                }
                super::pane::whole_rows(ui, id, |ui| add(ui, range))
            })
        });
        super::pane::hidden_height(ui, &output);
        output
    }

    /// For content that draws only what is in view of the given viewport.
    pub fn show_viewport<R>(
        self,
        ui: &mut Ui,
        add: impl FnOnce(&mut Ui, Rect) -> R,
    ) -> ScrollAreaOutput<R> {
        if super::pane::column_in_content(ui) {
            // Inline, everything is in view.
            let all =
                Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(f32::INFINITY, f32::INFINITY));
            return self.inline(ui, |ui| add(ui, all));
        }
        let output = Self::styled(ui, |ui, width| {
            let (full, id) = (self.full_width, self.id);
            let (area, column) = self.area_in(ui);
            area.show_viewport(ui, |ui, viewport| {
                super::reveal::region(ui, column, viewport, |ui| {
                    ui.set_max_width(width);
                    if full {
                        ui.set_min_width(width);
                    }
                    super::pane::whole_rows(ui, id, |ui| add(ui, viewport))
                })
            })
        });
        // As `show` and `show_rows`: a column in a child of composite columns adds what it hides
        // to the child's height (a `BlockRegion` draws through here; note C0-parts Part A).
        super::pane::hidden_height(ui, &output);
        output
    }

    /// Inside a `Body::Content` pane, which is a scroll column already, a release build draws the
    /// column's content in place: the pane scrolls it, and no scroll sits inside a scroll. (Debug
    /// builds and tests refuse such a column first, so that its caller is fixed.)
    pub(super) fn inline<R>(
        self,
        ui: &mut Ui,
        add: impl FnOnce(&mut Ui) -> R,
    ) -> ScrollAreaOutput<R> {
        let output = ui.scope(add);
        let rect = output.response.rect;
        ScrollAreaOutput {
            inner: output.inner,
            id: self.id,
            state: Default::default(),
            content_size: rect.size(),
            inner_rect: rect,
        }
    }
}

/// How a [`sideways_scroll`] sizes itself and shows its bar. The default takes the whole width
/// it is given and shows its bar while the content is wider.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Sideways {
    fit: bool,
    no_bar: bool,
}

impl Sideways {
    /// As wide as its content, up to the width it is given (a reader's table).
    pub fn fit(mut self) -> Self {
        self.fit = true;
        self
    }

    /// No bar while `hidden` (a table whose foot is out of view: its bar would float over the
    /// content below the view's edge).
    pub fn without_bar(mut self, hidden: bool) -> Self {
        self.no_bar = hidden;
        self
    }
}

/// A sideways scroll for one wide table or strip: it never scrolls up and down, so it sits
/// inside a page's own `ScrollColumn`. Its bar is solid: it has its own lane below the content
/// and never covers it (a reader's table keeps its last row's text clear of the bar). Its offset is egui's, kept under its own id: a caller
/// writes none (`id_salt` names whose content it is, so another document or surface has its
/// own offset).
#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
pub fn sideways_scroll<R>(
    ui: &mut Ui,
    id_salt: impl Hash,
    view: Sideways,
    add: impl FnOnce(&mut Ui) -> R,
) -> ScrollAreaOutput<R> {
    // The app's solid bar, whatever the caller's style: it takes its own lane.
    ui.scope(|ui| {
        ui.spacing_mut().scroll = theme::scroll_style();
        egui::ScrollArea::horizontal()
            .id_salt(Id::new(id_salt))
            .auto_shrink([view.fit, true])
            .scroll_bar_visibility(if view.no_bar {
                egui::scroll_area::ScrollBarVisibility::AlwaysHidden
            } else {
                egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded
            })
            .show(ui, add)
    })
    .inner
}

/// Fixed room for a text value that changes in place: as wide as the widest example in its
/// type, so the next control never moves when the value changes. A value wider than every
/// example is cut with an ellipsis (the gate's `cut_text` check finds it); it never pushes.
/// The examples are an array whose length is checked when the program compiles, so a slot with
/// none does not compile, at run time too.
///
/// ```
/// use splat_design::layout::{Slot, SlotType};
/// const SAVES: Slot = Slot::new(&["0 of 3", "3 of 3"], SlotType::Small);
/// let _ = Slot::new(&["Low"], SlotType::Body);
/// ```
///
/// ```compile_fail,E0080
/// use splat_design::layout::{Slot, SlotType};
/// let _ = Slot::new(&[], SlotType::Small);
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Slot {
    examples: &'static [&'static str],
    role: SlotType,
}

/// The type a slot's value is set in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlotType {
    Body,
    Strong,
    Small,
    Numeral,
}

impl SlotType {
    fn role(self) -> Type {
        match self {
            Self::Body => Type::Body,
            Self::Strong => Type::Strong,
            Self::Small => Type::Small,
            Self::Numeral => Type::Numeral,
        }
    }
}

impl Slot {
    pub const fn new<const N: usize>(examples: &'static [&'static str; N], role: SlotType) -> Self {
        // Evaluated for each `N` when the program compiles, whether the call is in a constant or
        // not.
        const {
            assert!(N > 0, "A slot has an example value.");
        }
        Self { examples, role }
    }

    /// The slot's width: its widest example's.
    pub fn width(&self, ui: &Ui) -> f32 {
        let font = self.role.role().font(ui);
        self.examples
            .iter()
            .map(|example| number_width(ui, example, &font))
            .fold(0.0, f32::max)
    }

    /// Draw `value` at the slot's start, in `color`. The slot takes its own width, never more.
    pub fn show(&self, ui: &mut Ui, value: &str, color: egui::Color32) -> Response {
        let width = self.width(ui);
        let role = self.role.role();
        let galley = super::one_line(ui, value, role.font(ui), color, width);
        let height = galley.size().y.max(role.line());
        let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
        if ui.is_rect_visible(rect) {
            let top = rect.center().y - galley.size().y / 2.0;
            ui.painter()
                .galley(egui::pos2(rect.left(), top), galley, color);
        }
        response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, value));
        response
    }
}

/// A status (icon, word, colour) in fixed room: as wide as the widest status that `status_of`
/// gives for any of `every`, so a control after it never moves when the state changes. The
/// examples come from the same function that makes the status, so they cannot drift.
pub fn status_slot<K: Copy>(
    ui: &mut Ui,
    every: &[K],
    now: K,
    status_of: impl Fn(K) -> Status<'static>,
) -> Response {
    let font = Type::Small.font(ui);
    let width = every
        .iter()
        .map(|&state| {
            let state = status_of(state);
            number_width(ui, &format!("{} {}", state.icon, state.word), &font)
        })
        .fold(0.0, f32::max);
    let height = Type::Small.line();
    ui.allocate_ui_with_layout(
        vec2(width, height),
        Layout::left_to_right(Align::Center),
        |ui| {
            ui.set_min_size(vec2(width, height));
            ui.set_max_width(width);
            super::status(ui, status_of(now))
        },
    )
    .inner
}

/// The parts that the doc tests of [`BackLink`] and `folder_group_heading` name.
/// Public only for those tests; the app uses them through `design`.
#[doc(hidden)]
pub mod doc_parts {
    pub use crate::design::{
        Action, AddField, Availability, Density, FolderGroup, ModeLock, ReaderToolbar, add_field,
        folder_group_heading, icons,
    };
}

/// A key letter shown on a control (`N`, `U`, `Ctrl+G`): a non-empty literal of the keys list,
/// checked when the program compiles. Make one with [`key!`](crate::key), which takes a string
/// literal; there is no other way, so a key cannot be an empty string chosen at run time.
///
/// ```
/// const NEXT: splat_design::layout::KeyName = splat_design::key!("N");
/// assert_eq!(NEXT.as_str(), "N");
/// ```
///
/// ```compile_fail,E0080
/// let _ = splat_design::key!("");
/// ```
///
/// ```compile_fail,E0080
/// let _ = splat_design::key!(" N");
/// ```
///
/// A key chosen at run time is not a literal, so the macro does not take it:
///
/// ```compile_fail
/// let hide = true;
/// let _ = splat_design::key!(if hide { "" } else { "N" });
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeyName(&'static str);

impl KeyName {
    /// Use [`key!`](crate::key). Clippy disallows this call outside `src/design/`.
    #[doc(hidden)]
    pub const fn checked(key: &'static str) -> Self {
        let bytes = key.as_bytes();
        assert!(!bytes.is_empty(), "A key letter is not empty.");
        assert!(
            !bytes[0].is_ascii_whitespace() && !bytes[bytes.len() - 1].is_ascii_whitespace(),
            "A key letter has no space at its ends."
        );
        Self(key)
    }

    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

/// A [`KeyName`] from a string literal, checked when the program compiles.
#[macro_export]
macro_rules! key {
    ($key:literal) => {
        const {
            #[allow(clippy::disallowed_methods, reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)")]
            let key = $crate::layout::KeyName::checked($key);
            key
        }
    };
}

/// How a title-line action looks. There is no form without its word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TitleWeight {
    Quiet,
    Secondary,
    /// A text link (`TitleAction::link`).
    Link,
}

/// One action on a title line. It always shows its [`Word`]; an icon goes before the word.
///
/// ```
/// use splat_design::layout::TitleAction;
/// let _ = TitleAction::quiet(splat_design::word!("Edit")).name("Edit Cinder Wolf");
/// ```
///
/// A word is a checked `Word`, never a plain string:
///
/// ```compile_fail,E0308
/// let _ = splat_design::layout::TitleAction::quiet("Edit");
/// ```
#[derive(Clone, Copy, Debug)]
#[must_use]
pub struct TitleAction<'a> {
    word: Word,
    weight: TitleWeight,
    icon: Option<&'static str>,
    blocked: Option<&'a str>,
    name: Option<&'a str>,
    key: Option<KeyName>,
    /// Its hover hint (`hint`).
    hint: Option<&'a str>,
    /// It opens a menu (`TitleLine::menu`): a quiet menu button, its caret after the word.
    menu: bool,
}

impl<'a> TitleAction<'a> {
    pub fn quiet(word: Word) -> Self {
        Self {
            word,
            weight: TitleWeight::Quiet,
            icon: None,
            blocked: None,
            name: None,
            key: None,
            hint: None,
            menu: false,
        }
    }

    pub fn secondary(word: Word) -> Self {
        Self {
            weight: TitleWeight::Secondary,
            ..Self::quiet(word)
        }
    }

    /// A text link (`Connect to refresh`): its word in the link colour, a target at least `XL`
    /// wide and one `Read` control high (`title_link`), placed and kept as an action is. It has
    /// no icon, key or lock (M-Y0.2).
    pub fn link(word: Word) -> Self {
        Self {
            weight: TitleWeight::Link,
            ..Self::quiet(word)
        }
    }

    pub fn icon(mut self, icon: &'static str) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn blocked(mut self, reason: Option<&'a str>) -> Self {
        self.blocked = reason;
        self
    }

    /// The accessible name, when the word is shorter than the meaning (`Edit Cinder Wolf`).
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    /// Its keyboard command beside its word (`New entry N`), as an action shows one; the line
    /// measures the action with it.
    pub fn key(mut self, key: KeyName) -> Self {
        self.key = Some(key);
        self
    }

    /// What it does, on hover, when its word does not say it all (`Campaign entries`: "This
    /// campaign’s own entries, in the Library"), as [`Action::hint`]. A blocked action shows its
    /// reason instead.
    pub fn hint(mut self, hint: &'a str) -> Self {
        self.hint = Some(hint);
        self
    }

    /// Its target's width on one line.
    fn natural_width(&self, ui: &Ui) -> f32 {
        if matches!(self.weight, TitleWeight::Link) {
            return number_width(ui, self.word.as_str(), &Type::Body.font(ui))
                .ceil()
                .max(XL);
        }
        if self.menu {
            self.menu_button(&Self::menu_label(self.word))
                .natural_width(ui)
        } else {
            self.action().natural_width(ui)
        }
    }

    /// A menu action's label: its word and the room its caret takes after it.
    fn menu_label(word: Word) -> String {
        format!("{}    ", word.as_str())
    }

    /// A menu action's button (`TitleLine::menu`): its weight, icon, name and lock, at `Read`,
    /// on `label` (`menu_label`), so its caret fits after the word.
    fn menu_button<'l>(&self, label: &'l str) -> Action<'l>
    where
        'a: 'l,
    {
        let mut action = match self.weight {
            TitleWeight::Quiet | TitleWeight::Link => Action::quiet(label),
            TitleWeight::Secondary => Action::secondary(label),
        }
        .density(Density::Read)
        .blocked(self.blocked)
        .name(self.name.unwrap_or(self.word.as_str()));
        if let Some(icon) = self.icon {
            action = action.icon(icon);
        }
        if let Some(hint) = self.hint {
            action = action.hint(hint);
        }
        action
    }

    fn action(&self) -> Action<'a> {
        let mut action = match self.weight {
            TitleWeight::Quiet | TitleWeight::Link => Action::quiet(self.word.as_str()),
            TitleWeight::Secondary => Action::secondary(self.word.as_str()),
        }
        .density(Density::Read)
        .blocked(self.blocked);
        if let Some(icon) = self.icon {
            action = action.icon(icon);
        }
        if let Some(name) = self.name {
            action = action.name(name);
        }
        if let Some(key) = self.key {
            action = action.key(key.as_str());
        }
        if let Some(hint) = self.hint {
            action = action.hint(hint);
        }
        action
    }
}

/// The space between two actions on a title line.
pub const ACTION_GAP: f32 = XS;

/// The space from the title's last letter to the first action's target. A quiet action's
/// words and icon start `S` inside its target, so its icon sits `L` after the title; a bordered
/// action's border sits `L` after it.
pub fn title_lead_in(first_is_quiet: bool) -> f32 {
    if first_is_quiet { L - S } else { L }
}

/// Where a title line's actions go. The rule (round 27, review of PR 300): no action vanishes or
/// loses its word.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TitlePlacement {
    /// First choice: the title and every action on one line, when they fit beside the title.
    OnTitle,
    /// Else every action on one row under the title, when that row fits the line under it; only
    /// when one row cannot fit, the rows wrap at whole actions, filled from the left. Each range
    /// is one row of actions, in order; together they hold every action once.
    Under(Vec<Range<usize>>),
}

/// The one decision of a title line's place, by measure. `title` is the width the title takes
/// (with its context), `lead_in` the space before the first action ([`title_lead_in`]),
/// `widths` each action's target width, `beside` the width the actions may use from the title's
/// left edge on its line, and `under` the width of the line under the title. Actions are
/// [`ACTION_GAP`] apart. An action wider than `under` has a row of its own; none is dropped.
/// Callers measure each width once and draw with the same widths, so the measure and the draw
/// cannot disagree.
pub fn title_placement(
    title: f32,
    lead_in: f32,
    widths: &[f32],
    beside: f32,
    under: f32,
) -> TitlePlacement {
    let row = |widths: &[f32]| {
        widths.iter().sum::<f32>() + ACTION_GAP * widths.len().saturating_sub(1) as f32
    };
    if widths.is_empty() || title + lead_in + row(widths) <= beside {
        return TitlePlacement::OnTitle;
    }
    let mut rows = Vec::new();
    let mut start = 0;
    for end in 1..widths.len() {
        // Close the row before the action that would not fit on it.
        if row(&widths[start..=end]) > under {
            rows.push(start..end);
            start = end;
        }
    }
    rows.push(start..widths.len());
    TitlePlacement::Under(rows)
}

/// A title and its actions, placed by [`title_placement`]: after the title when everything fits
/// on its line; else on one row under the title; else on rows that wrap at whole actions. Every
/// width is measured once, rounded up to a whole point, and the draw gives each piece exactly
/// that width, so the measure and the draw agree. No action is dropped or shown without its
/// word, and a word fits whole in the narrowest pane ([`MAX_WORD_CHARS`]).
///
/// Duty left to the caller: handle each press ([`TitleLineResponse::clicked`]). A caller-side
/// `if` that leaves an action out is not something this part can see: the gate's G2 check
/// ("header actions kept") owns it.
#[must_use = "call show"]
pub struct TitleLine<'a> {
    title: &'a str,
    context: Option<&'a str>,
    role: TitleType,
    actions: Vec<TitleAction<'a>>,
    /// The menus of the menu actions (`menu`), by the action's place.
    menus: Vec<(usize, Menu<'a>)>,
    /// A status link after the actions, and the menu it opens.
    status: Option<StatusMenu<'a>>,
    /// A status link after that one that acts when pressed (`status_press`).
    status_press: Option<Status<'a>>,
    /// The pressed status link's press this frame.
    status_clicked: bool,
    /// Each menu's room, from its control's rect (`menu_room`).
    menu_room: Option<Box<dyn Fn(Rect) -> Rect + 'a>>,
    /// The page's header (`page`): a rule under the line and `L` before the page's body.
    page: bool,
    /// A close icon at the right end of the title's line, and its accessible name.
    close: Option<&'a str>,
    /// A view toggle left of the close icon: its icon, its accessible name, its hint and
    /// whether it is on.
    toggle: Option<(&'static str, &'a str, &'a str, bool)>,
    /// A plain status at the right end of the title's line, left of the toggle (`state`).
    state: Option<(Status<'a>, Vec<Status<'a>>)>,
    /// What the title is, before it (`kind`): the words shown and the accessible name.
    kind: Option<(&'a str, &'a str)>,
    /// A `⋯` menu at the line's right end (`more`): its name and its menu.
    more: Option<(&'a str, Menu<'a>)>,
    /// The actions end at the line's end, before its status and `⋯` (`actions_at_end`).
    at_end: bool,
}

/// A menu's content: the caller draws its own choices.
type Menu<'a> = Box<dyn FnOnce(&mut Ui) + 'a>;

/// A title line's status link and the menu it opens.
type StatusMenu<'a> = (Status<'a>, Menu<'a>);

/// The type of a title line's title.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TitleType {
    /// A page or pane title.
    Title,
    /// A reader's or a panel's title.
    Section,
    /// A form's or a detail's own title in `Heading` type, as `Heading::Panel` (the Timeline
    /// event's title above its form, M0558).
    Panel,
}

/// What a title line did this frame.
#[derive(Clone, Debug, Default)]
pub struct TitleLineResponse {
    pressed: Option<Word>,
    /// The DM pressed the status link of `TitleLine::status_press`.
    pub status_pressed: bool,
    /// The DM pressed the close icon (`TitleLine::close`).
    pub closed: bool,
    /// The DM pressed the view toggle (`TitleLine::toggle`).
    pub toggled: bool,
    /// The actions sit under the title.
    pub below: bool,
    /// The rows of actions under the title (0 when they follow the title).
    pub rows: usize,
}

impl TitleLineResponse {
    /// The DM pressed the action with this word.
    pub fn clicked(&self, word: Word) -> bool {
        self.pressed == Some(word)
    }
}

/// One row of actions that a title line drew under its title (`TitlePlacement::Under`): the
/// title's rect, the row's rect (from its first target's left edge to its last target's right
/// edge, one control high) and how many actions it holds. [`TitleLine`] and the readers' title
/// lines (`world::title_line_actions`) register each row they draw; actions that follow the title
/// on its line register none.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TitleActionRow {
    pub title: Rect,
    pub row: Rect,
    pub actions: usize,
}

/// Where test builds keep each title action row of a pass.
#[cfg(any(test, feature = "test-support"))]
const TITLE_ACTION_ROWS: &str = "design_title_action_rows";

/// Register a row of actions drawn under a title. Only test builds keep it.
pub fn register_title_row(ui: &Ui, row: TitleActionRow) {
    #[cfg(any(test, feature = "test-support"))]
    {
        let pass = ui.ctx().cumulative_pass_nr();
        ui.ctx().data_mut(|data| {
            let kept = data
                .get_temp_mut_or_default::<(u64, Vec<TitleActionRow>)>(Id::new(TITLE_ACTION_ROWS));
            if kept.0 != pass {
                *kept = (pass, Vec::new());
            }
            kept.1.push(row);
        });
    }
    #[cfg(not(any(test, feature = "test-support")))]
    let _ = (ui, row);
}

/// Each action row drawn under a title in the last completed pass, in draw order; empty when
/// that pass drew none (a row never outlives its title line). Package G reads them through
/// `Driver::title_action_rows`.
#[cfg(any(test, feature = "test-support"))]
pub fn title_action_rows(ctx: &egui::Context) -> Vec<TitleActionRow> {
    let last = ctx.cumulative_pass_nr().saturating_sub(1);
    ctx.data(|data| data.get_temp::<(u64, Vec<TitleActionRow>)>(Id::new(TITLE_ACTION_ROWS)))
        .filter(|(pass, _)| *pass == last)
        .map(|(_, rows)| rows)
        .unwrap_or_default()
}

/// A title line's widths, measured once per frame and used by both the placement and the draw.
struct TitleMetrics {
    /// The kind's width before the title, with the gap after it.
    kind: f32,
    title: f32,
    /// The context's width, with the gap before it.
    context: f32,
    lead_in: f32,
    widths: Vec<f32>,
}

impl TitleMetrics {
    fn beside(&self) -> f32 {
        self.kind + self.title + self.context
    }
}

impl<'a> TitleLine<'a> {
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            context: None,
            role: TitleType::Title,
            actions: Vec::new(),
            menus: Vec::new(),
            status: None,
            status_press: None,
            status_clicked: false,
            menu_room: None,
            page: false,
            close: None,
            toggle: None,
            state: None,
            kind: None,
            more: None,
            at_end: false,
        }
    }

    /// The actions end at the right end of their line, before a plain status and the `⋯`, as a
    /// form's header has them (the Timeline event's `Copy link`, which ends at the form's edge
    /// for every event, M0558); placed and kept by the same rule, only right-aligned: on the
    /// title's line when they fit, else on rows under it.
    pub fn actions_at_end(mut self) -> Self {
        self.at_end = true;
        self
    }

    /// What the title is, before it, in muted `Small`, `XS` from the title (a Timeline event's
    /// `⚔ Combat`, or its icon alone for a day or an event with no title of its own, M0558).
    /// `words` is what shows; `name` is its accessible name, and its hover when the words are not
    /// the name (an icon alone), so the kind is never only an icon to a screen reader.
    pub fn kind(mut self, words: &'a str, name: &'a str) -> Self {
        self.kind = Some((words, name));
        self
    }

    /// A `⋯` menu named `name` at the right end of the title's own line, right of a plain status
    /// and left of a view toggle and a close icon, however the actions are placed (the Timeline
    /// event's `More actions for …`, M0558). It opens `menu` under it (`menu_popup`, in the
    /// line's `menu_room`); the title and the actions beside it keep clear of it.
    pub fn more(mut self, name: &'a str, menu: impl FnOnce(&mut Ui) + 'a) -> Self {
        self.more = Some((name, Box::new(menu)));
        self
    }

    pub fn role(mut self, role: TitleType) -> Self {
        self.role = role;
        self
    }

    /// One muted `Small` state, count or date after the title (the Notes panel's note date),
    /// `XS` after it.
    pub fn context(mut self, context: Option<&'a str>) -> Self {
        self.context = context;
        self
    }

    pub fn action(mut self, action: TitleAction<'a>) -> Self {
        self.actions.push(action);
        self
    }

    /// An action that opens `menu` under it (`Packages`), placed and kept as an action is: a
    /// quiet menu button, its caret after its word. A blocked one (`TitleAction::blocked`) is
    /// disabled with its reason on hover and does not open.
    pub fn menu(mut self, action: TitleAction<'a>, menu: impl FnOnce(&mut Ui) + 'a) -> Self {
        self.menus.push((self.actions.len(), Box::new(menu)));
        self.actions.push(TitleAction {
            menu: true,
            ..action
        });
        self
    }

    /// A status link after the actions (`3 files need fixing`, in its tone), placed and kept as
    /// an action is, that opens `menu` under it (`menu_popup`); the menu draws its own choices.
    pub fn status(mut self, status: Status<'a>, menu: impl FnOnce(&mut Ui) + 'a) -> Self {
        self.status = Some((status, Box::new(menu)));
        self
    }

    /// The room of each menu the line opens (a menu action's, the status link's), from where
    /// its control lands (its rect): the column of the page under it, when the title line
    /// spans several panes. The menu keeps inside it, one lane clear of its edges, and chooses
    /// its side there ([`MenuPopup::within`]), so it lies over no splitter or scroll bar of the
    /// panes below. Without it, the room is the line's pane.
    pub fn menu_room(mut self, room: impl Fn(Rect) -> Rect + 'a) -> Self {
        self.menu_room = Some(Box::new(room));
        self
    }

    /// The page's own header, as [`page_header`](super::page_header) draws one: the line at
    /// least a `Scan` control high, then a rule under it and `L` before the page's body.
    pub fn page(mut self) -> Self {
        self.page = true;
        self
    }

    /// A menu under `control`, in its room when the line has one (`menu_room`).
    fn menu_under<'r>(&self, ui: &Ui, control: &'r Response) -> MenuPopup<'r> {
        let popup = super::menu_popup(ui, control);
        match &self.menu_room {
            Some(room) => popup.within(room(control.rect)),
            None => popup,
        }
    }

    /// A close icon named `name` at the right end of the title's own line (a panel's title), on
    /// that line however the actions are placed; the title and the actions beside it keep clear
    /// of it. Its press is [`TitleLineResponse::closed`].
    pub fn close(mut self, name: &'a str) -> Self {
        self.close = Some(name);
        self
    }

    /// A view toggle at the right end of the title's own line, left of a close icon (the
    /// session list's show and hide): an icon target, `name` its accessible name and `hint`
    /// (with its key) on hover; `on` is its pressed state for assistive tools (the list is
    /// shown). It is not an action: it changes what the page shows beside it, never its data,
    /// and it keeps one place however the actions come, go or wrap. The title and the actions
    /// beside it keep clear of it, and the keyboard reaches it after them and before the close
    /// icon, as they stand on the line. Its press is [`TitleLineResponse::toggled`].
    pub fn toggle(mut self, icon: &'static str, name: &'a str, hint: &'a str, on: bool) -> Self {
        self.toggle = Some((icon, name, hint, on));
        self
    }

    /// A status link after the one with a menu (or after the actions), placed and kept as an
    /// action is, that acts when pressed (World's `N notes cannot be read` opens its repair):
    /// [`TitleLineResponse::status_pressed`].
    pub fn status_press(mut self, status: Status<'a>) -> Self {
        self.status_press = Some(status);
        self
    }

    /// The status link's width on one line.
    fn status_width(ui: &Ui, status: &Status<'_>) -> f32 {
        number_width(
            ui,
            &format!("{} {}", status.icon, status.word),
            &Type::Small.font(ui),
        )
        .ceil()
        .max(24.0)
    }

    fn font(&self) -> Type {
        match self.role {
            TitleType::Title => Type::Title,
            TitleType::Section => Type::Section,
            TitleType::Panel => Type::Heading,
        }
    }

    fn metrics(&self, ui: &Ui) -> TitleMetrics {
        TitleMetrics {
            kind: self.kind.map_or(0.0, |(words, _)| {
                number_width(ui, words, &Type::Small.font(ui)).ceil() + XS
            }),
            title: number_width(ui, self.title, &self.font().font(ui)).ceil(),
            context: self.context.map_or(0.0, |context| {
                XS + number_width(ui, context, &Type::Small.font(ui)).ceil()
            }),
            lead_in: title_lead_in(
                self.actions
                    .first()
                    .is_none_or(|action| action.weight == TitleWeight::Quiet),
            ),
            widths: self
                .actions
                .iter()
                .map(|action| action.natural_width(ui).ceil())
                .chain(
                    self.status
                        .as_ref()
                        .map(|(status, _)| Self::status_width(ui, status)),
                )
                .chain(
                    self.status_press
                        .as_ref()
                        .map(|status| Self::status_width(ui, status)),
                )
                .collect(),
        }
    }

    /// Draw one action in exactly its measured target; past the actions, the status link and
    /// its menu.
    fn action_at(&mut self, ui: &mut Ui, at: usize, rect: Rect) -> bool {
        // Past the actions: the status with its menu first, then the pressed one.
        let status = (at >= self.actions.len())
            .then(|| self.status.take())
            .flatten();
        let press = (at >= self.actions.len() && status.is_none())
            .then(|| self.status_press.take())
            .flatten();
        ui.scope_builder(
            UiBuilder::new()
                .max_rect(rect)
                .layout(Layout::left_to_right(Align::Center)),
            |ui| match status {
                Some((status, menu)) => {
                    let link = super::status_link(ui, status);
                    self.menu_under(ui, &link).show(menu);
                    false
                }
                None if let Some(status) = press => {
                    self.status_clicked |= super::status_link(ui, status).clicked();
                    false
                }
                None if self.actions[at].menu => {
                    let action = self.actions[at];
                    let label = TitleAction::menu_label(action.word);
                    let button = action.menu_button(&label).show(ui);
                    let caret = Rect::from_min_max(
                        egui::pos2(button.rect.right() - L - XS, button.rect.top()),
                        button.rect.right_bottom(),
                    );
                    super::combo_icon(ui, caret, ui.style().interact(&button), false);
                    let menu = self
                        .menus
                        .iter()
                        .position(|(place, _)| *place == at)
                        .map(|found| self.menus.swap_remove(found).1);
                    if let Some(menu) = menu.filter(|_| action.blocked.is_none()) {
                        self.menu_under(ui, &button).show(menu);
                    }
                    false
                }
                None if matches!(self.actions[at].weight, TitleWeight::Link) => {
                    // Reviews of PR 730 and 740: a link keeps its hint, as the other forms do;
                    // a blocked one takes no press and says why on hover and to a screen
                    // reader, as a blocked `Action` does.
                    let action = self.actions[at];
                    let link = ui
                        .add_enabled_ui(action.blocked.is_none(), |ui| {
                            super::title_link(ui, action.word.as_str())
                        })
                        .inner;
                    match (action.blocked, action.hint) {
                        (Some(reason), _) => {
                            ui.ctx().accesskit_node_builder(link.id, |node| {
                                node.set_description(reason);
                            });
                            link.on_disabled_hover_text(reason)
                        }
                        (None, Some(hint)) => link.on_hover_text(hint),
                        (None, None) => link,
                    }
                    .clicked()
                }
                None => self.actions[at].action().show(ui).clicked(),
            },
        )
        .inner
    }

    /// The close icon at the right end of the line from `top`, `line_height` high.
    fn close_at(&self, ui: &mut Ui, right: f32, top: f32, line_height: f32) -> bool {
        let Some(name) = self.close else {
            return false;
        };
        let size = Density::Read.control();
        let rect = Rect::from_min_size(
            egui::pos2(right - size, top + (line_height - size) / 2.0),
            vec2(size, size),
        );
        let mut child = ui.new_child(UiBuilder::new().max_rect(rect));
        super::icon_button(&mut child, super::icons::CLOSE, name).clicked()
    }

    /// A plain status at the right end of the title's own line, left of a view toggle and a close
    /// icon (a draft's `Saved` or `Unsaved`, a built-in template's `Read-only`): its icon and word
    /// in its tone, not a control. Unlike [`status`](Self::status) it opens nothing and is not
    /// an action. `now` is the status shown; `every` holds the statuses the line can show (`now`
    /// among them or not), and the status keeps the room of the widest of them, so a change of
    /// its word never reflows the line: the title and the actions beside it keep clear of that
    /// room, and they place the same with any of the statuses.
    pub fn state(mut self, now: Status<'a>, every: &[Status<'a>]) -> Self {
        self.state = Some((now, every.to_vec()));
        self
    }

    /// One status's width: its icon, a space and its word in `Small`, as it is drawn.
    fn status_words_width(ui: &Ui, status: &Status<'_>) -> f32 {
        number_width(
            ui,
            &format!("{} {}", status.icon, status.word),
            &Type::Small.font(ui),
        )
        .ceil()
            + 1.0
    }

    /// The plain status's room: the widest of the statuses the line can show, and the one shown.
    fn state_room(ui: &Ui, (now, every): &(Status<'_>, Vec<Status<'_>>)) -> f32 {
        every
            .iter()
            .chain(std::iter::once(now))
            .map(|status| Self::status_words_width(ui, status))
            .fold(0.0, f32::max)
    }

    /// The plain status, its room's right edge at `right` (left of the toggle), on the line from
    /// `top`, `line_height` high.
    fn state_at(&self, ui: &mut Ui, right: f32, top: f32, line_height: f32) {
        let Some(state) = &self.state else {
            return;
        };
        let width = Self::state_room(ui, state);
        let rect = Rect::from_min_size(egui::pos2(right - width, top), vec2(width, line_height));
        // Left to right in its room: its words start at the room's left edge, whichever status
        // is shown, and end at or before `right`.
        let mut child = ui.new_child(
            UiBuilder::new()
                .max_rect(rect)
                .layout(Layout::left_to_right(Align::Center)),
        );
        let _ = super::status(&mut child, state.0);
    }

    /// The `⋯` menu (`more`), its right edge at `right`, on the line from `top`,
    /// `line_height` high; its menu opens under it.
    fn more_at(&mut self, ui: &mut Ui, right: f32, top: f32, line_height: f32) {
        let Some((name, menu)) = self.more.take() else {
            return;
        };
        let size = Density::Read.control();
        let rect = Rect::from_min_size(
            egui::pos2(right - size, top + (line_height - size) / 2.0),
            vec2(size, size),
        );
        let mut child = ui.new_child(UiBuilder::new().max_rect(rect));
        let button = super::icon_button(&mut child, super::icons::EXPAND, name);
        self.menu_under(&child, &button).show(menu);
    }

    /// The view toggle, its right edge at `right` (left of the close icon), on the line from
    /// `top`, `line_height` high.
    fn toggle_at(&self, ui: &mut Ui, right: f32, top: f32, line_height: f32) -> bool {
        let Some((icon, name, hint, on)) = self.toggle else {
            return false;
        };
        let size = Density::Read.control();
        let rect = Rect::from_min_size(
            egui::pos2(right - size, top + (line_height - size) / 2.0),
            vec2(size, size),
        );
        let mut child = ui.new_child(UiBuilder::new().max_rect(rect));
        super::components::icon_toggle(&mut child, icon, name, hint, on).clicked()
    }

    pub fn show(mut self, ui: &mut Ui) -> TitleLineResponse {
        let metrics = self.metrics(ui);
        let room = ui.available_width();
        // The close icon, the view toggle and the plain status keep their room and a gap each at
        // the title line's right end, in that order from the end; `end` is all of it.
        let end_icon = Density::Read.control() + ACTION_GAP;
        let close = if self.close.is_some() { end_icon } else { 0.0 };
        let toggle = if self.toggle.is_some() { end_icon } else { 0.0 };
        let more = if self.more.is_some() { end_icon } else { 0.0 };
        let state = self
            .state
            .as_ref()
            .map_or(0.0, |state| Self::state_room(ui, state) + ACTION_GAP);
        let end = close + toggle + more + state;
        // Issue #567: a title with no actions that is wider than its line (less the close
        // icon) wraps under nothing, as the `Under` form wraps it; it never paints past the
        // line or under the close icon.
        let placement = if metrics.widths.is_empty() && metrics.beside() > room - end {
            TitlePlacement::Under(Vec::new())
        } else {
            // At the end, rows under the title end before the status and the `⋯` too, so they
            // wrap within that room (review of PR 780).
            title_placement(
                metrics.beside(),
                metrics.lead_in,
                &metrics.widths,
                room - end,
                if self.at_end { room - end } else { room },
            )
        };
        let closed;
        let toggled;
        let height = Density::Read.control();
        let mut pressed = None;
        let font = self.font();
        let title = egui::Label::new(font.rich(ui, self.title).color(colors().text));
        let context = self.context.map(|context| {
            (
                context,
                egui::Label::new(Type::Small.rich(ui, context).color(colors().muted)),
            )
        });
        let kind = self.kind.map(|(words, name)| {
            (
                words,
                name,
                egui::Label::new(Type::Small.rich(ui, words).color(colors().muted)),
            )
        });
        // The kind's accessible name is its name, never its icon's glyph; shown as an icon
        // alone, it also says its name on hover.
        let named_kind = |ui: &Ui, response: Response, words: &str, name: &str| {
            super::named(ui, &response, name);
            if !words.contains(name) {
                let _ = response.on_hover_text(name);
            }
        };
        let rows = match placement {
            TitlePlacement::OnTitle => {
                // A page's header line is at least a `Scan` control high, as `page_header`'s.
                let line_height = height.max(font.line()).max(if self.page {
                    Density::Scan.control()
                } else {
                    0.0
                });
                let (line, _) = ui.allocate_exact_size(vec2(room, line_height), Sense::hover());
                // The line's pieces go in a child that does not allocate in `ui` again.
                let mut ui = ui.new_child(
                    UiBuilder::new()
                        .max_rect(line)
                        .layout(Layout::left_to_right(Align::Center)),
                );
                let ui = &mut ui;
                let mut x = line.left();
                let mut piece = |width: f32| {
                    let rect =
                        Rect::from_min_size(egui::pos2(x, line.top()), vec2(width, line_height));
                    x += width;
                    rect
                };
                // Each piece starts at its rect's left edge, as `page_header`'s did: `ui.put`
                // centres a label, which puts its text's origin mid-piece (iter-24-B FE4 reads
                // that origin as the title's x).
                let at_left = |ui: &mut Ui, rect: Rect, label: egui::Label| {
                    ui.scope_builder(
                        UiBuilder::new()
                            .max_rect(rect)
                            .layout(Layout::left_to_right(Align::Center)),
                        |ui| ui.add(label),
                    )
                    .inner
                };
                if let Some((words, name, kind)) = kind {
                    let shown = at_left(ui, piece(metrics.kind - XS), kind.extend());
                    named_kind(ui, shown, words, name);
                    let _ = piece(XS);
                }
                at_left(ui, piece(metrics.title), title.extend());
                if let Some((words, context)) = context {
                    let _ = piece(XS);
                    at_left(ui, piece(metrics.context - XS), context.extend()).on_hover_text(words);
                }
                // At the end: the actions' row ends where the line's end pieces start.
                let row = metrics.widths.iter().sum::<f32>()
                    + ACTION_GAP * metrics.widths.len().saturating_sub(1) as f32;
                let lead_in = if self.at_end {
                    (line.width() - end - row - metrics.beside()).max(metrics.lead_in)
                } else {
                    metrics.lead_in
                };
                let _ = piece(lead_in);
                for (at, width) in metrics.widths.iter().enumerate() {
                    if at > 0 {
                        let _ = piece(ACTION_GAP);
                    }
                    if self.action_at(ui, at, piece(*width)) {
                        pressed = Some(self.actions[at].word);
                    }
                }
                // Drawn in their order on the line, so the keyboard reaches the toggle first.
                self.state_at(
                    ui,
                    line.right() - close - toggle - more,
                    line.top(),
                    line_height,
                );
                self.more_at(ui, line.right() - close - toggle, line.top(), line_height);
                toggled = self.toggle_at(ui, line.right() - close, line.top(), line_height);
                closed = self.close_at(ui, line.right(), line.top(), line_height);
                0
            }
            TitlePlacement::Under(rows) => {
                let (line_left, line_top) = (ui.cursor().left(), ui.cursor().top());
                let title_rect = ui
                    .horizontal_wrapped(|ui| {
                        ui.set_max_width(room - end);
                        ui.spacing_mut().item_spacing.x = XS;
                        if let Some((words, name, kind)) = kind {
                            let shown = ui.add(kind);
                            named_kind(ui, shown, words, name);
                        }
                        ui.add(title.wrap());
                        if let Some((words, context)) = context {
                            ui.add(context.wrap()).on_hover_text(words);
                        }
                    })
                    .response
                    .rect;
                let right = line_left + room;
                self.state_at(ui, right - close - toggle - more, line_top, font.line());
                self.more_at(ui, right - close - toggle, line_top, font.line());
                toggled = self.toggle_at(ui, right - close, line_top, font.line());
                closed = self.close_at(ui, right, line_top, font.line());
                for row in &rows {
                    let (line, _) = ui.allocate_exact_size(vec2(room, height), Sense::hover());
                    let mut row_ui = ui.new_child(
                        UiBuilder::new()
                            .max_rect(line)
                            .layout(Layout::left_to_right(Align::Center)),
                    );
                    let row_width = row.clone().map(|at| metrics.widths[at]).sum::<f32>()
                        + ACTION_GAP * row.len().saturating_sub(1) as f32;
                    // At the end: rows under the title end where the line's actions end on the
                    // title's line, before the status and the `⋯`, so an action keeps one x.
                    let mut x = if self.at_end {
                        line.right() - end - row_width
                    } else {
                        line.left()
                    };
                    for at in row.clone() {
                        if at > row.start {
                            x += ACTION_GAP;
                        }
                        let width = metrics.widths[at];
                        let rect =
                            Rect::from_min_size(egui::pos2(x, line.top()), vec2(width, height));
                        x += width;
                        if self.action_at(&mut row_ui, at, rect) {
                            pressed = Some(self.actions[at].word);
                        }
                    }
                    register_title_row(
                        ui,
                        TitleActionRow {
                            title: title_rect,
                            row: Rect::from_min_max(line.min, egui::pos2(x, line.bottom())),
                            actions: row.len(),
                        },
                    );
                }
                rows.len()
            }
        };
        if self.page {
            super::rule(ui);
            super::extras::space(ui, L);
        }
        TitleLineResponse {
            pressed,
            status_pressed: self.status_clicked,
            closed,
            toggled,
            below: rows > 0,
            rows,
        }
    }

    /// A back link that starts a line: its word at the text edge, then its icon. See
    /// [`BackLink`].
    pub fn back(word: Word, edge: BackEdge) -> BackLink<'a> {
        BackLink {
            word,
            edge,
            context: None,
            blocked: None,
            name: None,
        }
    }
}

/// The icon of a back link: a return arrow, after the word.
const BACK_LINK_ICON: &str = egui_phosphor::regular::ARROW_U_UP_LEFT;

/// What a back link's word lines up with: the text edge of the lines above and below it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BackEdge {
    /// Text that starts at the column's edge (a title, a label, a field). The word starts at the
    /// edge, and the target's padding hangs `S` into the gutter, as `Action::line_start`.
    Text,
    /// A `segmented` row, whose words start `S` inside its edge. The word starts there too, and
    /// the target at the edge.
    Segments,
}

/// A back link at the start of a line (FL7; Session's `Back to recordings`, Timeline's
/// `All events`): one quiet target that holds its [`Word`] and then its icon, centred on the
/// word's line; an optional muted context follows. The word is always shown, and it is the
/// accessible name. The icon never hangs left of the word: the room left of a column's text
/// edge is the gutter (`L` or less, a splitter beyond it), narrower than an icon, so the icon has
/// its place inside the target, after the word. The target is a `Read` control (28 px).
///
/// Duty left to the caller: say what the word lines up with ([`BackEdge`]); the gate's N4 check
/// finds a wrong choice. Handle the press.
///
/// ```
/// # use splat_design::layout::{BackEdge, TitleLine};
/// # fn draw(ui: &mut egui::Ui) {
/// let back = TitleLine::back(splat_design::word!("All events"), BackEdge::Text).show(ui);
/// # let _ = back;
/// # }
/// ```
///
/// A back link has no form without its word: a plain string or a bare icon does not compile.
///
/// ```compile_fail,E0308
/// # use splat_design::layout::{BackEdge, TitleLine};
/// let _ = TitleLine::back("←", BackEdge::Text);
/// ```
#[must_use = "call show"]
pub struct BackLink<'a> {
    word: Word,
    edge: BackEdge,
    context: Option<&'a str>,
    blocked: Option<&'a str>,
    /// The accessible name when it is longer than the word (only [`EditorHead`] sets it).
    name: Option<&'static str>,
}

impl<'a> BackLink<'a> {
    /// One muted `Small` context after the link (the open recording's name).
    pub fn context(mut self, context: &'a str) -> Self {
        self.context = Some(context);
        self
    }

    /// The link cannot be followed now, and why. The word stays.
    pub fn blocked(mut self, reason: Option<&'a str>) -> Self {
        self.blocked = reason;
        self
    }

    /// Draw the link (and its context) on one line. The response is the link's. In a
    /// left-to-right row it draws in that row, so it shares the row's centre with the row's
    /// other controls; elsewhere it makes its own line.
    pub fn show(self, ui: &mut Ui) -> Response {
        if ui.layout().main_dir() == egui::Direction::LeftToRight {
            self.draw(ui)
        } else {
            ui.horizontal(|ui| self.draw(ui)).inner
        }
    }

    fn draw(self, ui: &mut Ui) -> Response {
        let icon_font = Type::Body.font(ui);
        let icon = number_width(ui, BACK_LINK_ICON, &icon_font).ceil();
        let mut action = Action::quiet(self.word.as_str())
            .density(Density::Read)
            .blocked(self.blocked);
        if let Some(name) = self.name {
            action = action.name(name);
        }
        let action = match self.edge {
            BackEdge::Text => action.line_start(),
            BackEdge::Segments => action,
        };
        // The word's target, and room for the icon `XS` after the word.
        let width = action.natural_width(ui).ceil() + XS + icon;
        let response = action.min_width(width).show(ui);
        if ui.is_rect_visible(response.rect) {
            let mut painter = ui.painter().clone();
            if self.blocked.is_some() {
                painter.multiply_opacity(ui.visuals().disabled_alpha());
            }
            painter.text(
                egui::pos2(response.rect.right() - S - icon, response.rect.center().y),
                egui::Align2::LEFT_CENTER,
                BACK_LINK_ICON,
                icon_font,
                colors().text,
            );
        }
        if let Some(context) = self.context {
            ui.label(Type::Small.rich(ui, context).color(colors().muted));
        }
        response
    }
}

/// An unsaved draft that holds an editor, so that its back line cannot be followed. Each one
/// owns the words of its reason (R35: the same value disables Back and gives the painted words),
/// in a long and a short form, both whole sentences.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unsaved {
    /// The group editor's draft.
    Group,
    /// The encounter editor's draft.
    Encounter,
    /// A creature being edited inside an Encounters editor.
    Creature,
}

impl Unsaved {
    #[cfg(test)]
    const ALL: [Self; 3] = [Self::Group, Self::Encounter, Self::Creature];

    /// The reason in its long form: what to do before Back can be followed.
    pub const fn reason(self) -> &'static str {
        match self {
            Self::Group => "Save or discard this group first.",
            Self::Encounter => "Save or discard the open encounter first.",
            Self::Creature => "Save or discard the changes to this creature first.",
        }
    }

    /// The same reason, whole and shorter, for a back line that has no room for the long form.
    pub const fn short_reason(self) -> &'static str {
        match self {
            Self::Group => "Save or discard the group first.",
            Self::Encounter => "Save or discard it first.",
            Self::Creature => "Save or discard the creature first.",
        }
    }
}

/// Where an editor's back line goes. Each destination gives both the painted word (R17) and the
/// accessible name, so the two cannot disagree.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BackTo {
    /// The Encounters page's list.
    Encounters,
    /// The Session page's plan (the group editor's second host).
    Plan,
    /// The session the encounter was opened from.
    Session,
}

impl BackTo {
    #[cfg(test)]
    const ALL: [Self; 3] = [Self::Encounters, Self::Plan, Self::Session];

    /// The painted word.
    pub const fn word(self) -> Word {
        match self {
            Self::Encounters => crate::word!("Encounters"),
            Self::Plan => crate::word!("Plan"),
            Self::Session => crate::word!("Session"),
        }
    }

    /// The accessible name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Encounters => "Back to encounters",
            Self::Plan => "Back to the plan",
            Self::Session => "Back to session",
        }
    }
}

/// The owner of an editor's drafts: it says which drafts are unsaved, from the same values its
/// Save and its guards read. It is implemented once, by the owner, not by the view that draws
/// the head.
pub trait DraftOwner {
    /// Whether `draft` has unsaved changes.
    fn unsaved(&self, draft: Unsaved) -> bool;
}

/// The first unsaved draft of all an owner holds, encounter first, then its group, then a
/// creature: the reason a plan act or a replacement of the open draft is refused, from the same
/// recipe as the back lines.
pub fn first_unsaved(owner: &impl DraftOwner) -> Option<Unsaved> {
    [Unsaved::Encounter, Unsaved::Group, Unsaved::Creature]
        .into_iter()
        .find(|&draft| owner.unsaved(draft))
}

/// The two editors that have a back line, with the host each is drawn in. Each has a fixed
/// recipe of drafts that hold it, in order of priority, and its destination follows from the
/// host.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EditorKind {
    /// The group editor: its group, then a creature open beside it. Drawn in the Session
    /// plan, Back goes to the plan; else to the Encounters list.
    Group {
        /// Drawn as a part of the Session page's plan.
        in_plan: bool,
    },
    /// The encounter editor: its encounter, then a creature open beside it. Opened from a
    /// session, Back goes to that session; else to the Encounters list.
    Encounter {
        /// Opened from a session.
        from_session: bool,
    },
}

impl EditorKind {
    /// The drafts that can hold this editor, first first.
    const fn recipe(self) -> &'static [Unsaved] {
        match self {
            Self::Group { .. } => &[Unsaved::Group, Unsaved::Creature],
            Self::Encounter { .. } => &[Unsaved::Encounter, Unsaved::Creature],
        }
    }

    /// Where Back goes.
    const fn back_to(self) -> BackTo {
        match self {
            Self::Group { in_plan: true } => BackTo::Plan,
            Self::Encounter { from_session: true } => BackTo::Session,
            Self::Group { in_plan: false }
            | Self::Encounter {
                from_session: false,
            } => BackTo::Encounters,
        }
    }
}

/// The head of an Encounters editor (R38): its back line on a line of its own, then the name
/// row that the caller draws, both in one top-down child that this part makes, so the back line
/// is above the name row whatever the parent's layout.
///
/// The back line is ONE row of 28 px in every state: the destination's word at the text edge,
/// then the return arrow, and, while a draft holds the editor, that draft's reason after the
/// link in muted `Small` words: its long form when it fits, else its short form, measured
/// before paint, never wrapped or cut. Both forms fit from the narrowest pane
/// ([`NARROWEST_PANE`], 316 px) up; a part test proves it. So the head adds 28 px and one item
/// gap (34 px) above the name row, and no state reserves more.
///
/// ```
/// # use splat_design::layout::{DraftOwner, EditorHead, EditorKind, Unsaved};
/// # struct Owner;
/// # impl DraftOwner for Owner {
/// #     fn unsaved(&self, _: Unsaved) -> bool { false }
/// # }
/// # fn draw(ui: &mut egui::Ui, owner: &Owner) {
/// let (back, name) = EditorHead::new(EditorKind::Group { in_plan: false }, owner).show(ui, |ui| {
///     ui.horizontal(|ui| ui.label("the name field")).inner
/// });
/// # let _ = (back, name);
/// # }
/// ```
#[must_use = "call show"]
#[derive(Clone, Copy, Debug)]
pub struct EditorHead {
    to: BackTo,
    held: Option<Unsaved>,
}

impl EditorHead {
    /// The head of `editor`, from its owner's destination and drafts.
    pub fn new(editor: EditorKind, owner: &impl DraftOwner) -> Self {
        Self {
            to: editor.back_to(),
            held: editor
                .recipe()
                .iter()
                .copied()
                .find(|&draft| owner.unsaved(draft)),
        }
    }

    /// The draft that holds Back now, if any.
    pub fn held_by(&self) -> Option<Unsaved> {
        self.held
    }

    /// Draw the back line, then `name_row` under it. True when the DM follows the link (never
    /// while it is held), and what `name_row` returned.
    pub fn show<R>(self, ui: &mut Ui, name_row: impl FnOnce(&mut Ui) -> R) -> (bool, R) {
        ui.scope_builder(
            UiBuilder::new().layout(Layout::top_down(Align::Min)),
            |ui| {
                let back = self.back_line(ui);
                (back, name_row(ui))
            },
        )
        .inner
    }

    /// The reason's words at this room: the long form if it fits, else the short one.
    fn reason_words(ui: &Ui, draft: Unsaved, room: f32) -> &'static str {
        let font = Type::Small.font(ui);
        if number_width(ui, draft.reason(), &font) <= room {
            draft.reason()
        } else {
            draft.short_reason()
        }
    }

    /// The room after the link for a reason, at the line's width.
    fn reason_room(ui: &Ui, to: BackTo, width: f32) -> f32 {
        let icon = number_width(ui, BACK_LINK_ICON, &Type::Body.font(ui)).ceil();
        let link = Action::quiet(to.word().as_str())
            .density(Density::Read)
            .line_start()
            .natural_width(ui)
            .ceil()
            + XS
            + icon;
        width - link - L
    }

    fn back_line(self, ui: &mut Ui) -> bool {
        let width = ui.available_width();
        let room = Self::reason_room(ui, self.to, width);
        let reason = self.held.map(|draft| Self::reason_words(ui, draft, room));
        let link = BackLink {
            word: self.to.word(),
            edge: BackEdge::Text,
            context: None,
            blocked: self.held.map(Unsaved::reason),
            name: Some(self.to.name()),
        };
        let gap = ui.spacing().item_spacing.x;
        ui.allocate_ui_with_layout(
            vec2(width, Density::Read.control()),
            Layout::left_to_right(Align::Center),
            |ui| {
                ui.set_min_size(vec2(width, Density::Read.control()));
                ui.set_max_height(Density::Read.control());
                let response = link.draw(ui);
                if let Some(reason) = reason {
                    space(ui, (L - gap).max(0.0));
                    ui.add(
                        egui::Label::new(Type::Small.rich(ui, reason).color(colors().muted))
                            .wrap_mode(egui::TextWrapMode::Extend),
                    );
                }
                response.clicked() && self.held.is_none()
            },
        )
        .inner
    }
}

/// Every reason an [`InlineCreate`]'s Create can be blocked with. The kept line keeps room for
/// the tallest of them, and Create's reason is one of them, so a reason that is not declared
/// does not compile. Declare them with [`create_reasons!`](crate::create_reasons), which checks
/// each reason with package M's `sentence!`; the lint refuses an `impl CreateReasons` by hand,
/// which could leave a reason out of `ALL`.
pub trait CreateReasons: Copy + 'static {
    const ALL: &'static [Self];
    fn words(self) -> &'static str;
}

/// For a form whose Create is never blocked.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NoReason {}

impl CreateReasons for NoReason {
    const ALL: &'static [Self] = &[];
    fn words(self) -> &'static str {
        match self {}
    }
}

/// Declare an enum of an [`InlineCreate`]'s reasons, with each variant's words. `ALL` holds every
/// variant, by construction.
///
/// ```
/// splat_design::create_reasons! {
///     pub enum NewEntry {
///         NoName => "Enter a name first.",
///         Taken => "An entry with this name exists. Choose another name.",
///     }
/// }
/// use splat_design::layout::CreateReasons;
/// assert_eq!(NewEntry::ALL.len(), 2);
/// ```
///
/// ```compile_fail,E0080
/// splat_design::create_reasons! {
///     pub enum NewEntry { NoName => "Enter a name first" }
/// }
/// let _ = splat_design::layout::CreateReasons::words(NewEntry::NoName);
/// ```
#[macro_export]
macro_rules! create_reasons {
    ($(#[$meta:meta])* $vis:vis enum $name:ident { $($variant:ident => $words:literal),+ $(,)? }) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        $vis enum $name { $($variant),+ }

        impl $crate::layout::CreateReasons for $name {
            const ALL: &'static [Self] = &[$(Self::$variant),+];
            fn words(self) -> &'static str {
                match self {
                    $(Self::$variant => {
                        // Each reason is a checked sentence (package M's `sentence!`).
                        const _: $crate::outcome::Sentence = $crate::sentence!($words);
                        $words
                    }),+
                }
            }
        }
    };
}

/// How the named Create action looks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CreateWeight {
    Primary,
    Secondary,
}

/// The named Create action of an [`InlineCreate`]. Its word is a [`Word`], so it keeps its width
/// while the DM types; a blocked Create's reason is one of the form's declared reasons.
///
/// ```compile_fail,E0308
/// use splat_design::layout::{Create, NoReason};
/// let _: Create<NoReason> =
///     Create::primary(splat_design::word!("Create")).blocked(Some("Enter a name first."));
/// ```
#[derive(Clone, Copy, Debug)]
#[must_use]
pub struct Create<R: CreateReasons> {
    word: Word,
    weight: CreateWeight,
    blocked: Option<R>,
    key: Option<&'static str>,
}

impl<R: CreateReasons> Create<R> {
    pub fn primary(word: Word) -> Self {
        Self {
            word,
            weight: CreateWeight::Primary,
            blocked: None,
            key: None,
        }
    }

    pub fn secondary(word: Word) -> Self {
        Self {
            weight: CreateWeight::Secondary,
            ..Self::primary(word)
        }
    }

    /// Disable Create and say why on the kept line below the field.
    pub fn blocked(mut self, reason: Option<R>) -> Self {
        self.blocked = reason;
        self
    }

    /// Show Create's key beside its word (`Enter`), so the DM sees how to finish from the
    /// keyboard.
    pub fn key(mut self, key: &'static str) -> Self {
        self.key = Some(key);
        self
    }

    fn action(&self) -> Action<'static> {
        match self.weight {
            CreateWeight::Primary => Action::primary(self.word.as_str()),
            CreateWeight::Secondary => Action::secondary(self.word.as_str()),
        }
        .density(Density::Read)
        .blocked(self.blocked.map(R::words))
        .key(self.key.unwrap_or_default())
    }
}

/// A secondary action beside a [`CreateField`] (`Browse…`). A plain [`Word`] is one that is
/// never locked. A lock that comes and goes is [`locked_when`](Self::locked_when) with its
/// reason, the form the footer part's choices use: the action is disabled, with the reason as
/// its accessible description and hover, and the reason's words on the form's kept line. The
/// kept line keeps the reason's room for the life of the form, so a lock that comes or goes
/// moves nothing.
///
/// ```
/// use splat_design::layout::SideAction;
/// let loading = true;
/// let _ = SideAction::new(splat_design::word!("Browse…"))
///     .locked_when(loading, splat_design::sentence!("Wait until the Library has loaded."));
/// ```
///
/// A lock without its reason does not compile:
///
/// ```compile_fail,E0308
/// use splat_design::layout::SideAction;
/// let _ = SideAction::new(splat_design::word!("Browse…")).locked_when(true, None);
/// ```
///
/// Nor does a reason that is not a checked sentence:
///
/// ```compile_fail,E0308
/// use splat_design::layout::SideAction;
/// let _ = SideAction::new(splat_design::word!("Browse…")).locked_when(true, "Wait.");
/// ```
///
/// Nor an optional lock, which could be written with no reason:
///
/// ```compile_fail,E0599
/// use splat_design::layout::SideAction;
/// let _ = SideAction::new(splat_design::word!("Browse…")).blocked(Some("Wait."));
/// ```
#[derive(Clone, Debug)]
#[must_use]
pub struct SideAction {
    word: Word,
    /// The lock's reason, and whether it holds now.
    lock: Option<(Sentence, bool)>,
}

impl SideAction {
    pub fn new(word: Word) -> Self {
        Self { word, lock: None }
    }

    /// Locked while `locked`, for `reason`. The reason's room on the kept line is kept while
    /// the form is open, so a lock that comes or goes moves nothing below it.
    pub fn locked_when(mut self, locked: bool, reason: Sentence) -> Self {
        self.lock = Some((reason, locked));
        self
    }

    fn reason(&self) -> Option<&str> {
        self.lock
            .as_ref()
            .and_then(|(reason, locked)| locked.then_some(reason.as_str()))
    }

    fn action(&self) -> Action<'_> {
        Action::secondary(self.word.as_str())
            .density(Density::Read)
            .blocked(self.reason())
    }
}

impl From<Word> for SideAction {
    fn from(word: Word) -> Self {
        Self::new(word)
    }
}

/// One text field of an [`InlineCreate`].
#[must_use]
pub struct CreateField<'a> {
    id: FieldId,
    label: &'a str,
    text: &'a mut String,
    help: Option<&'a str>,
    side: Option<SideAction>,
    focus: bool,
    placeholder: Option<&'a str>,
    /// Words the DM may choose instead of typing them (`suggestions`).
    suggestions: Option<&'a [String]>,
}

impl<'a> CreateField<'a> {
    pub fn new(id: FieldId, label: &'a str, text: &'a mut String) -> Self {
        Self {
            id,
            label,
            text,
            help: None,
            side: None,
            focus: false,
            placeholder: None,
            suggestions: None,
        }
    }

    /// Words the DM may choose instead of typing them (a folder that exists): while the field
    /// has the keyboard or its words change, a list under it holds the suggestions that contain
    /// its words; a press fills the field with one. The DM may still type anything.
    pub fn suggestions(mut self, suggestions: &'a [String]) -> Self {
        self.suggestions = Some(suggestions);
        self
    }

    /// The field's hint inside it, in place of a caption above it: `label` is then only the
    /// field's accessible name. For a field whose place says what it is (an add field at the
    /// end of its list).
    pub fn placeholder(mut self, hint: &'a str) -> Self {
        self.placeholder = Some(hint);
        self
    }

    pub fn help(mut self, help: &'a str) -> Self {
        self.help = Some(help);
        self
    }

    /// A secondary action beside the field, before Create when Create follows this field
    /// (`Browse…`): a [`Word`], or a [`SideAction`] with its lock.
    pub fn side_action(mut self, side: impl Into<SideAction>) -> Self {
        self.side = Some(side.into());
        self
    }

    /// Give the field keyboard focus in this frame.
    pub fn focus(mut self, focus: bool) -> Self {
        self.focus = focus;
        self
    }
}

/// A choice (a checkbox) of an [`InlineCreate`], before its last field.
#[must_use]
pub struct CreateChoice<'a> {
    label: &'a str,
    value: &'a mut bool,
    help: Option<&'a str>,
}

impl<'a> CreateChoice<'a> {
    pub fn new(label: &'a str, value: &'a mut bool) -> Self {
        Self {
            label,
            value,
            help: None,
        }
    }

    pub fn help(mut self, help: &'a str) -> Self {
        self.help = Some(help);
        self
    }
}

/// The choices of an [`InlineCreate`]'s drop-down: values and their words, and reasons for
/// choices that cannot be taken. It has no command row: a form's drop-down only chooses, so a
/// press can never be lost (a [`Dropdown`] with `.command` does not fit [`CreateDropdown`]).
///
/// A command does not exist here:
///
/// ```compile_fail,E0599
/// use splat_design::layout::{Once, Selection};
/// let _ = Selection::new(splat_design::field_id!("doc_sel").once(Once::Example), "Template", 0_u8, "Standard")
///     .command(splat_design::word!("New template"));
/// ```
#[must_use]
pub struct Selection<'a, T> {
    id: FieldId,
    name: &'a str,
    choices: Vec<(T, Cow<'a, str>, Option<&'a str>)>,
}

impl<'a, T> Selection<'a, T> {
    /// `name` is the control's accessible name; a selection has at least one choice.
    pub fn new(id: FieldId, name: &'a str, first: T, label: impl Into<Cow<'a, str>>) -> Self {
        Self {
            id,
            name,
            choices: vec![(first, label.into(), None)],
        }
    }

    pub fn choices<L: Into<Cow<'a, str>>>(
        mut self,
        more: impl IntoIterator<Item = (T, L)>,
    ) -> Self {
        self.choices.extend(
            more.into_iter()
                .map(|(value, label)| (value, label.into(), None)),
        );
        self
    }

    /// A choice that cannot be taken now, with its reason on its row.
    pub fn blocked_choice(
        mut self,
        value: T,
        label: impl Into<Cow<'a, str>>,
        reason: &'a str,
    ) -> Self {
        self.choices.push((value, label.into(), Some(reason)));
        self
    }

    fn dropdown(self) -> Option<Dropdown<'a, T, HasChoices>> {
        let mut choices = self.choices.into_iter();
        let (value, label, reason) = choices.next()?;
        let start = Dropdown::new(self.id, self.name);
        let mut dropdown = match reason {
            Some(reason) => start.blocked_choice(value, label, reason),
            None => start.choice(value, label),
        };
        for (value, label, reason) in choices {
            dropdown = match reason {
                Some(reason) => dropdown.blocked_choice(value, label, reason),
                None => dropdown.choice(value, label),
            };
        }
        Some(dropdown)
    }
}

/// A drop-down of an [`InlineCreate`], after a text field: a [`Selection`] (its choices, words
/// and reasons are the caller's) with its label above it. The part gives it the form's width.
/// A form whose last part is a drop-down puts Create and Cancel on an action line of their own
/// under it (see [`InlineCreate`]).
///
/// A drop-down with a command row does not fit (its press would have no place):
///
/// ```compile_fail,E0308
/// use splat_design::layout::{CreateDropdown, Dropdown, Once};
/// let mut value = 0_u8;
/// let _ = CreateDropdown::new(
///     "Template",
///     Dropdown::new(splat_design::field_id!("doc_cmd").once(Once::Example), "Template")
///         .choice(0, "Standard")
///         .command(splat_design::word!("New template")),
///     &mut value,
/// );
/// ```
#[must_use]
pub struct CreateDropdown<'a> {
    label: &'a str,
    help: Option<&'a str>,
    /// The typed drop-down and its value, erased only here, so a caller gives data, not a
    /// drawing closure.
    draw: DrawDropdown<'a>,
}

/// A [`CreateDropdown`]'s typed drop-down, drawn at the form's width.
type DrawDropdown<'a> = Box<dyn FnOnce(&mut Ui, f32) -> Option<Response> + 'a>;

impl<'a> CreateDropdown<'a> {
    pub fn new<T: PartialEq + Clone + 'a>(
        label: &'a str,
        selection: Selection<'a, T>,
        current: &'a mut T,
    ) -> Self {
        Self {
            label,
            help: None,
            draw: Box::new(move |ui, width| {
                selection
                    .dropdown()
                    .map(|dropdown| dropdown.width(width).show(ui, current).response)
            }),
        }
    }

    pub fn help(mut self, help: &'a str) -> Self {
        self.help = Some(help);
        self
    }
}

/// The line below the last field (and below the action line of a form that ends with a
/// drop-down): quiet Cancel at its start when there is something to cancel and Cancel is not on
/// the action line, then a locked side action's reason and Create's blocked reason, or, once
/// nothing blocks Create, the preview. At one width its height is fixed for the life of the
/// form: as tall as a control, or as the declared side reason and the tallest declared reason
/// wrapped beside Cancel at that width, whichever is taller. So nothing below it moves while
/// the DM types or a reason or lock comes and goes. A resize may change its height, as it
/// changes every wrapped text.
#[derive(Clone, Copy, Debug, Default)]
#[must_use]
pub struct KeptLine<'a> {
    cancel: Option<(Word, bool)>,
    cancel_key: Option<&'static str>,
    /// The preview's leading words and its path.
    preview: Option<(&'a str, &'a str)>,
}

impl<'a> KeptLine<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    /// The form's Cancel (or Clear) word, and whether there is something to cancel now. The word
    /// is given whether or not it shows, so the line keeps its room.
    pub fn cancel(mut self, word: Word, shown: bool) -> Self {
        self.cancel = Some((word, shown));
        self
    }

    /// Show Cancel's key beside its word (`Esc`).
    pub fn cancel_key(mut self, key: &'static str) -> Self {
        self.cancel_key = Some(key);
        self
    }

    /// What Create will make (`Will create NPCs/Watchkeeper.md`), shown once nothing blocks it.
    /// It keeps one line: a long one keeps its leading words (those before the first word that
    /// holds a path separator) and gives way in its path; its hover and accessible name keep it
    /// whole.
    pub fn preview(mut self, preview: Option<&'a str>) -> Self {
        self.preview = preview.map(|preview| {
            let path_at = preview
                .split_inclusive(' ')
                .scan(0, |at, word| {
                    let start = *at;
                    *at += word.len();
                    Some((start, word))
                })
                .find(|(_, word)| word.contains(['/', '\\']))
                .map_or(0, |(start, _)| start);
            preview.split_at(path_at)
        });
        self
    }

    /// `preview` with its leading words (`Will add `) and its path given apart, for a path that
    /// may hold no separator (a folder name alone): the leading words always stay.
    pub fn preview_path(mut self, lead: &'a str, path: Option<&'a str>) -> Self {
        self.preview = path.map(|path| (lead, path));
        self
    }
}

enum CreatePart<'a> {
    Text(CreateField<'a>),
    Choice(CreateChoice<'a>),
    Dropdown(CreateDropdown<'a>),
}

/// Typestate of an [`InlineCreate`]: no part yet.
pub struct Empty;
/// Typestate of an [`InlineCreate`]: the last part is a choice; a text field must follow.
pub struct AfterChoice;
/// Typestate of an [`InlineCreate`]: the last part is a text field; Create may follow.
pub struct AfterField;
/// Typestate of an [`InlineCreate`]: the last part is the form's first drop-down, after a text
/// field: a second drop-down, the last choice and field, or Create may follow.
pub struct AfterDropdown;
/// Typestate of an [`InlineCreate`]: the last part is the form's second drop-down: the last
/// choice and field, or Create may follow; a third drop-down does not compile.
pub struct AfterDropdowns;
/// Typestate of an [`InlineCreate`]: a choice after the drop-downs; the last text field follows.
pub struct AfterLateChoice;
/// Typestate of an [`InlineCreate`]: the last text field, after the drop-downs; only Create
/// follows, on that field's line when it fits (Library's New entry: Name, Kind, Destination, a
/// choice, Folder).
pub struct AfterLateField;
/// Typestate of an [`InlineCreate`]: Create is named; the kept line must follow.
pub struct Named;
/// Typestate of an [`InlineCreate`]: complete.
pub struct Complete;

/// The one inline creation layout: the fields (and choices and drop-downs) in order, each with
/// its help; the named Create after the last field on that field's line (after its side
/// action), then the last field's help, then one kept line. Help under an earlier field has `L`
/// before the next label. Nothing moves while the DM types: Create keeps its word, and at one
/// width the kept line keeps its height for the life of the form (a resize may change it, as it
/// changes wrapped text).
///
/// A form whose last part is a drop-down (the New session form's Template) has an action line
/// under the drop-down and its help: Create, then Cancel, at the form's text edge. The kept line
/// follows it, with the reasons or the preview. Everything above the action line has a fixed
/// height and the preview keeps one line below it, so neither a chosen value nor a preview's
/// length moves Create.
///
/// Width: the form takes the column it is drawn in (a reader's measure), at most
/// [`FORM_WIDTH`], and never reaches past the `Ui`'s clip. When a field's side action and
/// Create do not fit beside a field of at least [`CREATE_FIELD_MIN`], they go on an action line
/// under the field, so no target is cut; a long text scrolls inside its field. Cancel on the
/// kept line hangs its padding left of the text edge only when the clip has room for it.
///
/// Keyboard: the fields, then the drop-downs, then Create, then Cancel (drawing order). Enter
/// in a text field presses Create when nothing blocks it; Escape in a text field presses Cancel
/// when Cancel shows. The keyboard leaves the field in both cases. After Create, the caller
/// gives the keyboard to what was made, or back to the form's field when the act failed and the
/// form stays; after Cancel, the caller closes the form.
///
/// The order is in the type: `create` follows a text field or a drop-down, a drop-down follows
/// a text field, `kept_line` follows `create`, and only a complete form has `show`.
///
/// ```
/// use splat_design::layout::{Create, CreateField, InlineCreate, KeptLine};
/// splat_design::create_reasons! { enum Reason { NoName => "Enter a name first." } }
/// fn form(ui: &mut egui::Ui, name: &mut String) {
///     let reason = name.is_empty().then_some(Reason::NoName);
///     let _ = InlineCreate::new()
///         .field(CreateField::new(splat_design::field_id!("doc_form_name").once(splat_design::layout::Once::Example), "Name", name))
///         .create(Create::primary(splat_design::word!("Create")).blocked(reason))
///         .kept_line(KeptLine::new())
///         .show(ui);
/// }
/// ```
///
/// Create before any field does not compile:
///
/// ```compile_fail,E0599
/// use splat_design::layout::{Create, InlineCreate, NoReason};
/// let _ = InlineCreate::new().create(Create::<NoReason>::primary(splat_design::word!("Create")));
/// ```
///
/// Create after a choice, with no field after it, does not compile:
///
/// ```compile_fail,E0599
/// use splat_design::layout::{Create, CreateChoice, InlineCreate, NoReason};
/// let mut used = false;
/// let _ = InlineCreate::new()
///     .choice(CreateChoice::new("Use", &mut used))
///     .create(Create::<NoReason>::primary(splat_design::word!("Create")));
/// ```
///
/// A field after Create does not compile:
///
/// ```compile_fail,E0599
/// use splat_design::layout::{Create, CreateField, InlineCreate, NoReason};
/// let (mut a, mut b) = (String::new(), String::new());
/// let _ = InlineCreate::new()
///     .field(CreateField::new(splat_design::field_id!("doc_a").once(splat_design::layout::Once::Example), "A", &mut a))
///     .create(Create::<NoReason>::primary(splat_design::word!("Create")))
///     .field(CreateField::new(splat_design::field_id!("doc_b").once(splat_design::layout::Once::Example), "B", &mut b));
/// ```
///
/// A form with no kept line has no `show`:
///
/// ```compile_fail,E0599
/// use splat_design::layout::{Create, CreateField, InlineCreate, NoReason};
/// fn form(ui: &mut egui::Ui, name: &mut String) {
///     let _ = InlineCreate::new()
///         .field(CreateField::new(splat_design::field_id!("doc_name").once(splat_design::layout::Once::Example), "Name", name))
///         .create(Create::<NoReason>::primary(splat_design::word!("Create")))
///         .show(ui);
/// }
/// ```
///
/// A form that ends with a drop-down, with Create and Cancel on its action line:
///
/// ```
/// use splat_design::layout::{Create, CreateDropdown, CreateField, InlineCreate, KeptLine, NoReason, Once, Selection};
/// fn form(ui: &mut egui::Ui, name: &mut String, template: &mut u8) {
///     let _ = InlineCreate::new()
///         .field(CreateField::new(splat_design::field_id!("doc_dd_name").once(Once::Example), "Title", name))
///         .dropdown(CreateDropdown::new(
///             "Template",
///             Selection::new(splat_design::field_id!("doc_dd_template").once(Once::Example), "Template", 0, "Standard session")
///                 .choices([(1, "Short notes")]),
///             template,
///         ))
///         .create(Create::<NoReason>::primary(splat_design::word!("Create session")))
///         .kept_line(KeptLine::new().cancel(splat_design::word!("Cancel"), true))
///         .show(ui);
/// }
/// ```
///
/// A form that ends with a drop-down and has no Create has no kept line and no `show`:
///
/// ```compile_fail,E0599
/// use splat_design::layout::{CreateDropdown, CreateField, InlineCreate, KeptLine, NoReason, Once, Selection};
/// fn form(ui: &mut egui::Ui, name: &mut String, template: &mut u8) {
///     let _ = InlineCreate::<splat_design::layout::Empty, NoReason>::new()
///         .field(CreateField::new(splat_design::field_id!("doc_dd_name").once(Once::Example), "Title", name))
///         .dropdown(CreateDropdown::new(
///             "Template",
///             Selection::new(splat_design::field_id!("doc_dd_template").once(Once::Example), "Template", 0, "Standard"),
///             template,
///         ))
///         .kept_line(KeptLine::new())
///         .show(ui);
/// }
/// ```
///
/// A drop-down before any text field does not compile (it follows the name):
///
/// ```compile_fail,E0599
/// use splat_design::layout::{CreateDropdown, InlineCreate, NoReason, Once, Selection};
/// fn form(template: &mut u8) {
///     let _ = InlineCreate::<splat_design::layout::Empty, NoReason>::new().dropdown(CreateDropdown::new(
///         "Template",
///         Selection::new(splat_design::field_id!("doc_dd_template").once(Once::Example), "Template", 0, "Standard"),
///         template,
///     ));
/// }
/// ```
///
/// At most two drop-downs, after the name field; after them, an optional choice and one last
/// text field, which Create follows (Library's New entry: Name, Kind, Destination, a choice,
/// Folder). A third drop-down does not compile:
///
/// ```compile_fail,E0599
/// use splat_design::layout::{CreateDropdown, CreateField, InlineCreate, NoReason, Once, Selection};
/// fn form(name: &mut String, a: &mut u8, b: &mut u8, c: &mut u8) {
///     let _ = InlineCreate::<splat_design::layout::Empty, NoReason>::new()
///         .field(CreateField::new(splat_design::field_id!("doc_dd_name").once(Once::Example), "Title", name))
///         .dropdown(CreateDropdown::new("A", Selection::new(splat_design::field_id!("doc_dd_a").once(Once::Example), "A", 0, "One"), a))
///         .dropdown(CreateDropdown::new("B", Selection::new(splat_design::field_id!("doc_dd_b").once(Once::Example), "B", 0, "One"), b))
///         .dropdown(CreateDropdown::new("C", Selection::new(splat_design::field_id!("doc_dd_c").once(Once::Example), "C", 0, "One"), c));
/// }
/// ```
///
/// Nothing follows the last text field but Create:
///
/// ```compile_fail,E0599
/// use splat_design::layout::{CreateDropdown, CreateField, InlineCreate, NoReason, Once, Selection};
/// fn form(name: &mut String, folder: &mut String, more: &mut String, a: &mut u8) {
///     let _ = InlineCreate::<splat_design::layout::Empty, NoReason>::new()
///         .field(CreateField::new(splat_design::field_id!("doc_dd_name").once(Once::Example), "Title", name))
///         .dropdown(CreateDropdown::new("A", Selection::new(splat_design::field_id!("doc_dd_a").once(Once::Example), "A", 0, "One"), a))
///         .field(CreateField::new(splat_design::field_id!("doc_dd_folder").once(Once::Example), "Folder", folder))
///         .field(CreateField::new(splat_design::field_id!("doc_dd_more").once(Once::Example), "More", more));
/// }
/// ```
///
/// No drop-down after the choice that follows the drop-downs:
///
/// ```compile_fail,E0599
/// use splat_design::layout::{CreateChoice, CreateDropdown, CreateField, InlineCreate, NoReason, Once, Selection};
/// fn form(name: &mut String, used: &mut bool, a: &mut u8, b: &mut u8) {
///     let _ = InlineCreate::<splat_design::layout::Empty, NoReason>::new()
///         .field(CreateField::new(splat_design::field_id!("doc_dd_name").once(Once::Example), "Title", name))
///         .dropdown(CreateDropdown::new("A", Selection::new(splat_design::field_id!("doc_dd_a").once(Once::Example), "A", 0, "One"), a))
///         .choice(CreateChoice::new("Use", used))
///         .dropdown(CreateDropdown::new("B", Selection::new(splat_design::field_id!("doc_dd_b").once(Once::Example), "B", 0, "One"), b));
/// }
/// ```
///
/// A drop-down is a typed [`Selection`], never a closure that draws:
///
/// ```compile_fail,E0277
/// use splat_design::layout::CreateDropdown;
/// let mut value = 0_u8;
/// let _ = CreateDropdown::new("Template", |ui: &mut egui::Ui| ui.label("Mine"), &mut value);
/// ```
#[must_use = "call show"]
pub struct InlineCreate<'a, State, R: CreateReasons> {
    parts: Vec<CreatePart<'a>>,
    create: Option<Create<R>>,
    kept: KeptLine<'a>,
    /// A layer above the form that Escape closes first (a chosen suggestion).
    above: bool,
    /// Create and Cancel on an action line under the last field, as after a drop-down.
    under: bool,
    state: PhantomData<State>,
}

/// What the DM did in an [`InlineCreate`] this frame.
#[derive(Debug, Default)]
pub struct InlineCreateResponse {
    pub create: bool,
    pub cancel: bool,
    /// A field's side action (`Browse…`; a form has one). Never true while it is locked.
    pub side: bool,
    /// Each text field's response, in order.
    pub fields: Vec<Response>,
    /// Each drop-down's control, in order.
    pub dropdowns: Vec<Response>,
    /// Escape closed the layer above the form ([`InlineCreate::escape_closes_above`]) and not
    /// the form: no Cancel, and the field keeps the keyboard.
    pub closed_above: bool,
}

impl<'a, State, R: CreateReasons> InlineCreate<'a, State, R> {
    fn into<Next>(self) -> InlineCreate<'a, Next, R> {
        InlineCreate {
            parts: self.parts,
            create: self.create,
            kept: self.kept,
            above: self.above,
            under: self.under,
            state: PhantomData,
        }
    }
}

impl<R: CreateReasons> InlineCreate<'_, Empty, R> {
    pub fn new() -> Self {
        Self {
            parts: Vec::new(),
            create: None,
            kept: KeptLine::default(),
            above: false,
            under: false,
            state: PhantomData,
        }
    }
}

impl<R: CreateReasons> Default for InlineCreate<'_, Empty, R> {
    fn default() -> Self {
        Self::new()
    }
}

macro_rules! add_parts {
    ($($state:ty),*) => {$(
        impl<'a, R: CreateReasons> InlineCreate<'a, $state, R> {
            pub fn field(mut self, field: CreateField<'a>) -> InlineCreate<'a, AfterField, R> {
                self.parts.push(CreatePart::Text(field));
                self.into()
            }

            pub fn choice(mut self, choice: CreateChoice<'a>) -> InlineCreate<'a, AfterChoice, R> {
                self.parts.push(CreatePart::Choice(choice));
                self.into()
            }
        }
    )*};
}
add_parts!(Empty, AfterChoice, AfterField);

impl<'a, R: CreateReasons> InlineCreate<'a, AfterField, R> {
    /// The form's first drop-down, after the name field (a text field comes first).
    pub fn dropdown(mut self, dropdown: CreateDropdown<'a>) -> InlineCreate<'a, AfterDropdown, R> {
        self.parts.push(CreatePart::Dropdown(dropdown));
        self.into()
    }
}

impl<'a, R: CreateReasons> InlineCreate<'a, AfterDropdown, R> {
    /// The form's second and last drop-down.
    pub fn dropdown(mut self, dropdown: CreateDropdown<'a>) -> InlineCreate<'a, AfterDropdowns, R> {
        self.parts.push(CreatePart::Dropdown(dropdown));
        self.into()
    }
}

macro_rules! add_late_parts {
    ($($state:ty),*) => {$(
        impl<'a, R: CreateReasons> InlineCreate<'a, $state, R> {
            /// A choice after the drop-downs, before the last text field.
            pub fn choice(
                mut self,
                choice: CreateChoice<'a>,
            ) -> InlineCreate<'a, AfterLateChoice, R> {
                self.parts.push(CreatePart::Choice(choice));
                self.into()
            }

            /// The last text field, after the drop-downs: Create follows it on its line.
            pub fn field(mut self, field: CreateField<'a>) -> InlineCreate<'a, AfterLateField, R> {
                self.parts.push(CreatePart::Text(field));
                self.into()
            }
        }
    )*};
}
add_late_parts!(AfterDropdown, AfterDropdowns);

impl<'a, R: CreateReasons> InlineCreate<'a, AfterLateChoice, R> {
    /// The last text field, after the choice: Create follows it on its line.
    pub fn field(mut self, field: CreateField<'a>) -> InlineCreate<'a, AfterLateField, R> {
        self.parts.push(CreatePart::Text(field));
        self.into()
    }
}

macro_rules! add_create {
    ($($state:ty),*) => {$(
        impl<'a, R: CreateReasons> InlineCreate<'a, $state, R> {
            pub fn create(mut self, create: Create<R>) -> InlineCreate<'a, Named, R> {
                self.create = Some(create);
                self.into()
            }
        }
    )*};
}
add_create!(AfterField, AfterDropdown, AfterDropdowns, AfterLateField);

impl<'a, R: CreateReasons> InlineCreate<'a, Named, R> {
    pub fn kept_line(mut self, kept: KeptLine<'a>) -> InlineCreate<'a, Complete, R> {
        self.kept = kept;
        self.into()
    }
}

/// The narrowest a create form's text field is beside its actions (the words a field shows
/// before it scrolls). Narrower, the actions go on a line of their own under the field.
pub const CREATE_FIELD_MIN: f32 = 120.0;

/// The room of a create form: the column it is drawn in, never past the `Ui`'s clip, at most
/// [`FORM_WIDTH`].
fn form_room(ui: &Ui) -> f32 {
    ui.available_width()
        .min(ui.clip_rect().right() - ui.cursor().left())
        .clamp(1.0, FORM_WIDTH)
}

/// The height of `words` in `Small` type, wrapped at `width`.
fn small_height(ui: &Ui, words: &str, width: f32) -> f32 {
    let font = Type::Small.font(ui);
    ui.fonts_mut(|fonts| {
        fonts
            .layout(words.to_owned(), font, colors().muted, width)
            .size()
            .y
    })
}

/// The kept line's fixed height in `room`: a control's, or the declared side reasons and the
/// tallest declared reason, stacked and wrapped beside Cancel, whichever is taller. A reason
/// drawn with Cancel hidden has more room, and a lock that is open draws nothing, so the line
/// is never taller than this.
fn kept_line_height<R: CreateReasons>(
    ui: &Ui,
    room: f32,
    cancel: Option<(Word, Option<&'static str>)>,
    sides: &[&str],
) -> f32 {
    let beside = cancel.map_or(0.0, |(word, key)| {
        cancel_action(word, key).natural_width(ui) + L
    });
    let width = (room - beside).max(1.0);
    let create = R::ALL
        .iter()
        .map(|reason| small_height(ui, reason.words(), width))
        .fold(0.0, f32::max);
    let sides: f32 = sides
        .iter()
        .map(|reason| small_height(ui, reason, width))
        .sum();
    (create + sides).max(Density::Read.control())
}

fn cancel_action(word: Word, key: Option<&'static str>) -> Action<'static> {
    Action::quiet(word.as_str())
        .density(Density::Read)
        .key(key.unwrap_or_default())
}

/// Cancel where it starts a line at the form's text edge: its words at the edge, its padding
/// hanging left of it only where the clip shows it (a reader nested in a pane clips at its own
/// text edge), so its target is never cut. It allocates the same width either way.
fn cancel_at_line_start(ui: &Ui, word: Word, key: Option<&'static str>) -> Action<'static> {
    if ui.cursor().left() - LINE_START_HANG >= ui.clip_rect().left() - 0.5 {
        cancel_action(word, key).line_start()
    } else {
        cancel_action(word, key)
    }
}

/// The input method's composition in one form: the text field it types into. It ends when that
/// field loses the keyboard (a press on another control, the form closed: egui takes the
/// keyboard from a field that is not drawn), when the window loses the keyboard, and when a
/// commit or a `Disabled` event arrives.
/// A field that has not had the keyboard since the frame before never composes.
#[derive(Clone, Copy)]
struct Composing(Id);

/// A text field's own key filter (egui's single-line default), with Escape kept in the field:
/// egui then does not take the keyboard on Escape before the frame's widgets run.
const COMPOSING_FILTER: egui::EventFilter = egui::EventFilter {
    horizontal_arrows: true,
    vertical_arrows: true,
    tab: false,
    escape: true,
};

/// Whole actions, in order, in rows of `room` from the form's text edge: an action that does
/// not fit after the ones before it starts the next row. No action is split or cut to fit a
/// row. The rows depend only on the actions' own widths and the room, never on a field's text,
/// so they do not change while the DM types. `draw` is told whether the action starts its row
/// (at the text edge).
fn action_rows(ui: &mut Ui, room: f32, widths: &[f32], mut draw: impl FnMut(&mut Ui, usize, bool)) {
    let mut rows: Vec<Vec<usize>> = Vec::new();
    let mut used = 0.0;
    for (at, width) in widths.iter().copied().enumerate() {
        match rows.last_mut() {
            Some(row) if used + S + width <= room => {
                row.push(at);
                used += S + width;
            }
            _ => {
                rows.push(vec![at]);
                used = width;
            }
        }
    }
    for row in rows {
        ui.allocate_ui_with_layout(
            vec2(room, Density::Read.control()),
            Layout::left_to_right(Align::Center),
            |ui| {
                ui.spacing_mut().item_spacing.x = S;
                for (place, at) in row.into_iter().enumerate() {
                    draw(ui, at, place == 0);
                }
            },
        );
    }
}

/// A text field's width without its margins in `room`, and whether its side action and Create
/// go beside it: they do while the field keeps [`CREATE_FIELD_MIN`] beside them.
fn field_width<R: CreateReasons>(
    ui: &Ui,
    room: f32,
    side: Option<&SideAction>,
    create: Option<&Create<R>>,
) -> (f32, bool) {
    let side_width = side.map_or(0.0, |side| side.action().natural_width(ui) + S);
    let create_width = create.map_or(0.0, |create| create.action().natural_width(ui) + S);
    let actions = side.is_some() || create.is_some();
    let beside_width = room - side_width - create_width - 2.0 * S;
    (beside_width, !actions || beside_width >= CREATE_FIELD_MIN)
}

/// The width of a form's last text field, margins included, when the form has `room` (its
/// column, at most [`FORM_WIDTH`]): what the field keeps beside Create when Create follows it
/// (`beside`), or all the room when Create goes under it (`None`: a form with
/// [`InlineCreate::actions_under`], or too little room). For a caller that puts words before the
/// form on its line (a stamp) and needs the field's own room.
pub fn create_field_width<R: CreateReasons>(ui: &Ui, room: f32, beside: Option<&Create<R>>) -> f32 {
    let room = room.clamp(1.0, FORM_WIDTH);
    match field_width(ui, room, None, beside) {
        (width, true) => width + 2.0 * S,
        (_, false) => room,
    }
}

/// What one text field's line gave back.
struct FieldLine {
    response: Response,
    side: bool,
    create: bool,
    /// The field's suggestions were open when the frame began: a layer above the form that
    /// Escape closes first.
    suggestions_open: bool,
}

/// One text field with its side action and, when Create follows it, Create: beside the field
/// when the field keeps [`CREATE_FIELD_MIN`] beside them, else on an action line under it. The
/// help goes under the line (and under the action line).
fn field_line<R: CreateReasons>(
    ui: &mut Ui,
    part: CreateField<'_>,
    help: Option<&str>,
    room: f32,
    create: Option<&Create<R>>,
) -> FieldLine {
    let (beside_width, beside) = field_width(ui, room, part.side.as_ref(), create);
    let actions = part.side.is_some() || create.is_some();
    let mut line = FieldLine {
        response: ui.response(),
        side: false,
        create: false,
        suggestions_open: false,
    };
    // The actions in order: the side action, then Create (true for the side action).
    let mut actions_in_order: Vec<bool> = Vec::new();
    if part.side.is_some() {
        actions_in_order.push(true);
    }
    if create.is_some() {
        actions_in_order.push(false);
    }
    let side = part.side.as_ref();
    let action_at = |at: usize| match (actions_in_order[at], side, create) {
        (true, Some(side), _) => Some(side.action()),
        (false, _, Some(create)) => Some(create.action()),
        _ => None,
    };
    let draw_action = |ui: &mut Ui, line: &mut FieldLine, at: usize| {
        let clicked = action_at(at).is_some_and(|action| action.show(ui).clicked());
        if actions_in_order[at] {
            line.side = clicked && side.is_some_and(|side| side.reason().is_none());
        } else {
            line.create = clicked;
        }
    };
    // A field with its hint inside it has no caption: its label is its accessible name.
    let captioned = part.placeholder.is_none();
    let label = part.label;
    let mut body = |ui: &mut Ui| {
        let row = |ui: &mut Ui, add: &mut dyn FnMut(&mut Ui)| {
            ui.allocate_ui_with_layout(
                vec2(room, Density::Read.control()),
                Layout::left_to_right(Align::Center),
                |ui| {
                    ui.spacing_mut().item_spacing.x = S;
                    add(ui);
                },
            );
        };
        if beside {
            let width = if actions {
                beside_width
            } else {
                room - 2.0 * S
            };
            row(ui, &mut |ui| {
                line.response =
                    form_field(ui, part.id, part.label, part.text, part.placeholder, width);
                for at in 0..actions_in_order.len() {
                    draw_action(ui, &mut line, at);
                }
            });
        } else {
            // The actions do not fit beside the field: they take whole rows under it, at the
            // field's left edge, so none is cut at the column's edge.
            line.response = form_field(
                ui,
                part.id,
                part.label,
                part.text,
                part.placeholder,
                room - 2.0 * S,
            );
            let widths: Vec<f32> = (0..actions_in_order.len())
                .map(|at| action_at(at).map_or(0.0, |action| action.natural_width(ui)))
                .collect();
            action_rows(ui, room, &widths, |ui, at, _| {
                draw_action(ui, &mut line, at)
            });
        }
    };
    if captioned {
        field(ui, label, help, None, body);
    } else {
        body(ui);
    }
    if part.focus {
        line.response.request_focus();
    }
    if let Some(suggestions) = part.suggestions {
        line.suggestions_open = suggestion_list(ui, &line.response, part.text, suggestions);
    }
    line
}

/// A create field's suggestions (`CreateField::suggestions`): a menu under the field of those
/// that contain its words, opened when the field takes the keyboard or its words change, closed
/// by a choice, a press outside it, or Escape (which the form then gives as `closed_above`, not
/// Cancel). A choice fills the field. Only an open or opening menu reads the suggestions. Whether
/// the menu was open when the frame began.
fn suggestion_list(
    ui: &mut Ui,
    field: &Response,
    text: &mut String,
    suggestions: &[String],
) -> bool {
    let id = egui::Popup::default_response_id(field);
    let was_open = egui::Popup::is_id_open(ui.ctx(), id);
    let opening = field.gained_focus() || field.changed();
    if !was_open && !opening {
        return false;
    }
    if was_open && ui.input(|input| input.key_pressed(egui::Key::Escape)) {
        egui::Popup::close_id(ui.ctx(), id);
        return true;
    }
    let typed = text.trim().to_lowercase();
    // Owned: the menu's content outlives this frame's borrows.
    let matching: Vec<String> = suggestions
        .iter()
        .filter(|suggestion| suggestion.to_lowercase().contains(&typed))
        .cloned()
        .collect();
    let current = text.clone();
    let command = opening.then_some(egui::SetOpenCommand::Bool(!matching.is_empty()));
    let mut picked = None;
    super::menu_popup(&*ui, field)
        .open_memory(command)
        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
        .show(|ui| {
            for suggestion in matching {
                if super::extras::choice_wrapped(ui, suggestion == current, &suggestion, None)
                    .clicked()
                {
                    picked = Some(suggestion);
                    ui.close();
                }
            }
        });
    if let Some(picked) = picked {
        *text = picked;
    }
    was_open
}

impl<R: CreateReasons> InlineCreate<'_, Complete, R> {
    /// While `above`, a layer shows above the form that Escape closes first (a chosen
    /// suggestion under an add field): Escape then gives `closed_above`, not Cancel. It goes
    /// through the form's input-method guard as every Escape does.
    pub fn escape_closes_above(mut self, above: bool) -> Self {
        self.above = above;
        self
    }

    /// Create, then Cancel, on an action line under the last text field, as a form that ends
    /// with a drop-down has them: the field keeps the whole room, so its hint and its words are
    /// as wide as a field without actions. A form whose kept line can hold nothing (no declared
    /// reason, no side lock, no preview) then has none.
    pub fn actions_under(mut self) -> Self {
        self.under = true;
        self
    }

    pub fn show(self, ui: &mut Ui) -> InlineCreateResponse {
        let room = form_room(ui);
        ui.scope(|ui| {
            // Nothing in the form is wider than its room: a labelled field takes the
            // available width.
            ui.set_max_width(room);
            self.show_in(ui, room)
        })
        .inner
    }

    fn show_in(self, ui: &mut Ui, room: f32) -> InlineCreateResponse {
        let mut out = InlineCreateResponse::default();
        let Some(create) = self.create else {
            return out;
        };
        let count = self.parts.len();
        let ends_with_dropdown = matches!(self.parts.last(), Some(CreatePart::Dropdown(_)));
        // Create and Cancel on an action line of their own, under the last part.
        let action_line = ends_with_dropdown || self.under;
        // Every declared lock reason of a side action keeps its room on the kept line.
        let sides: Vec<(Sentence, bool)> = self
            .parts
            .iter()
            .filter_map(|part| match part {
                CreatePart::Text(field) => field.side.as_ref().and_then(|side| side.lock.clone()),
                _ => None,
            })
            .collect();
        // Composition (an input method's text that is not committed yet) belongs to the one
        // text field it types into. While that field composes, and in any frame with an IME
        // event (its commit), Enter and Escape belong to the input method: they leave this
        // frame's input before any field draws, so egui keeps the field's keyboard (and the
        // commit that follows reaches it), and they give no Create and no Cancel.
        let text_ids: Vec<Id> = self
            .parts
            .iter()
            .filter_map(|part| match part {
                CreatePart::Text(field) => Some(field.id.named_id(field.label)),
                _ => None,
            })
            .collect();
        let ime_key = text_ids.first().map(|id| id.with("inline_create_ime"));
        let focused = ui
            .memory(|memory| memory.focused())
            .filter(|id| text_ids.contains(id));
        // The window lost the keyboard: egui keeps a field's focus then (`has_focus` ignores the
        // window), but the input method's composition ends, and no commit may follow.
        let window_left = ui.input(|input| {
            input.viewport().focused == Some(false)
                || input
                    .events
                    .iter()
                    .any(|event| matches!(event, egui::Event::WindowFocused(false)))
        });
        // The state of the frame before, only while its field still has the keyboard and the
        // window has kept it.
        let was_composing = !window_left
            && ime_key
                .and_then(|key| ui.data(|data| data.get_temp::<Composing>(key)))
                .is_some_and(|Composing(field)| Some(field) == focused);
        let (ime_event, composing) = ui.input(|input| {
            let ime = || {
                input.events.iter().filter_map(|event| match event {
                    egui::Event::Ime(ime) => Some(ime),
                    _ => None,
                })
            };
            let composing = ime().fold(was_composing, |_, ime| {
                matches!(ime, egui::ImeEvent::Enabled | egui::ImeEvent::Preedit(_))
            });
            (
                ime().next().is_some(),
                !window_left && focused.is_some() && composing,
            )
        });
        if focused.is_some() && (was_composing || ime_event) {
            ui.input_mut(|input| {
                input.events.retain(|event| {
                    !matches!(
                        event,
                        egui::Event::Key {
                            key: egui::Key::Enter | egui::Key::Escape,
                            ..
                        }
                    )
                });
            });
        }
        let keys_for_the_form = !was_composing && !ime_event;
        let mut last_help = None;
        // A field's open suggestions are a layer above the form, as a chosen one is.
        let mut suggestions_open = false;
        for (at, part) in self.parts.into_iter().enumerate() {
            let last = at + 1 == count;
            match part {
                CreatePart::Choice(choice) => {
                    ui.checkbox(choice.value, choice.label);
                    if let Some(help) = choice.help {
                        // `L` from the help's last line to the next label, as a field's help has.
                        let bottom = super::help(ui, help).rect.bottom();
                        space(ui, bottom + L - ui.cursor().top());
                    }
                }
                CreatePart::Text(part) if !last => {
                    // The field adds `L` after its help.
                    let help = part.help;
                    let line = field_line::<R>(ui, part, help, room, None);
                    out.side |= line.side;
                    suggestions_open |= line.suggestions_open;
                    out.fields.push(line.response);
                }
                CreatePart::Text(part) => {
                    last_help = part.help;
                    let beside = (!self.under).then_some(&create);
                    let line = field_line(ui, part, None, room, beside);
                    out.side |= line.side;
                    out.create |= line.create;
                    suggestions_open |= line.suggestions_open;
                    out.fields.push(line.response);
                }
                CreatePart::Dropdown(dropdown) => {
                    let help = if last { None } else { dropdown.help };
                    if last {
                        last_help = dropdown.help;
                    }
                    let response = field(ui, dropdown.label, help, None, |ui| {
                        (dropdown.draw)(ui, room)
                    });
                    out.dropdowns.extend(response);
                }
            }
        }
        // The last field's help, under its line (K2, gap K-5): each field with its help, then
        // the kept line.
        if let Some(help) = last_help.take() {
            super::help(ui, help);
        }
        let cancel_shown = matches!(self.kept.cancel, Some((_, true)));
        if action_line {
            // The action line: Create, then Cancel, at the form's text edge. Above it every
            // part has a fixed height, so a chosen value never moves it.
            // Whole actions in rows: in a narrow column Cancel goes under Create.
            let mut widths = vec![create.action().natural_width(ui)];
            if let Some((word, true)) = self.kept.cancel {
                widths.push(cancel_action(word, self.kept.cancel_key).natural_width(ui));
            }
            action_rows(ui, room, &widths, |ui, at, starts_row| {
                if at == 0 {
                    out.create |= create.action().show(ui).clicked();
                } else if let Some((word, true)) = self.kept.cancel {
                    // Cancel that wraps to a row of its own starts the line at the text edge.
                    let action = if starts_row {
                        cancel_at_line_start(ui, word, self.kept.cancel_key)
                    } else {
                        cancel_action(word, self.kept.cancel_key)
                    };
                    out.cancel |= action.show(ui).clicked();
                }
            });
        }
        // Enter in a text field presses Create when nothing blocks it; Escape presses Cancel
        // when Cancel shows. egui takes the keyboard from a one-line field on both keys.
        let (enter, escape) = ui.input(|input| {
            (
                input.key_pressed(egui::Key::Enter),
                input.key_pressed(egui::Key::Escape),
            )
        });
        let left = out.fields.iter().any(Response::lost_focus) && keys_for_the_form;
        if left && enter && create.blocked.is_none() && ui.is_enabled() {
            out.create = true;
        }
        // Escape closes the top layer first: a layer above the form (a chosen suggestion), after
        // the input method's guard, so a composition keeps it too.
        if left && escape && (self.above || suggestions_open) {
            out.closed_above = true;
            if let Some(field) = out.fields.iter().find(|field| field.lost_focus()) {
                field.request_focus();
            }
        } else if left && escape && cancel_shown {
            out.cancel = true;
        }
        // The composition goes on only in the field that had the keyboard at the frame's start
        // and still has it (a press on another control ends it, as egui ends it then). While
        // it goes on, Escape stays in that field in the next pass.
        if let Some(key) = ime_key {
            let field = focused.filter(|id| composing && ui.memory(|memory| memory.has_focus(*id)));
            ui.data_mut(|data| match field {
                Some(field) => data.insert_temp(key, Composing(field)),
                None => data.remove::<Composing>(key),
            });
            if let Some(field) = field {
                ui.memory_mut(|memory| memory.set_focus_lock_filter(field, COMPOSING_FILTER));
            }
        }
        // A form with its actions under its field and nothing that its kept line could ever
        // hold has none.
        if self.under && R::ALL.is_empty() && sides.is_empty() && self.kept.preview.is_none() {
            return out;
        }
        // The kept line: one fixed height for the life of the form, whatever it holds now.
        let kept_cancel = if action_line { None } else { self.kept.cancel };
        let declared: Vec<&str> = sides.iter().map(|(reason, _)| reason.as_str()).collect();
        let height = kept_line_height::<R>(
            ui,
            room,
            kept_cancel.map(|(word, _)| (word, self.kept.cancel_key)),
            &declared,
        );
        let (line, _) = ui.allocate_exact_size(vec2(room, height), Sense::hover());
        // Its content goes in a child that does not allocate in `ui` again.
        let mut kept = ui.new_child(
            UiBuilder::new()
                .max_rect(line)
                .layout(Layout::left_to_right(Align::Min)),
        );
        kept.spacing_mut().item_spacing.x = 0.0;
        if let Some((word, true)) = kept_cancel {
            out.cancel |= cancel_at_line_start(&kept, word, self.kept.cancel_key)
                .show(&mut kept)
                .clicked();
            space(&mut kept, L);
        }
        let mut reasons: Vec<&str> = sides
            .iter()
            .filter(|(_, locked)| *locked)
            .map(|(reason, _)| reason.as_str())
            .collect();
        if let Some(reason) = create.blocked {
            reasons.push(reason.words());
        }
        // Each reason once.
        reasons.dedup();
        if !reasons.is_empty() {
            kept.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                for reason in reasons {
                    let words = Type::Small.rich(ui, reason).color(colors().muted);
                    ui.add(egui::Label::new(words).wrap());
                }
            });
        } else if let Some((lead, path)) = self.kept.preview {
            preview_line(&mut kept, lead, path);
        }
        out
    }
}

/// A preview keeps one line. One that does not fit keeps its leading words (`Will create`) and
/// gives way in its path, from the path's front (`Will create …/modules/copper_coast`), so the
/// final name stays as long as it can; only a final name that alone does not fit is cut at its
/// start too. Words with no path give way at their start. Its hover and accessible name keep it
/// whole.
fn preview_line(ui: &mut Ui, lead: &str, path: &str) {
    let font = Type::Small.font(ui);
    let room = ui.available_width();
    let preview = format!("{lead}{path}");
    // One rendered row: a line break or a tab in the words is a space here (the hover and the
    // accessible name keep the words as given).
    let row = |words: &str| -> String {
        words
            .chars()
            .map(|c| if c.is_control() { ' ' } else { c })
            .collect()
    };
    let (lead, path) = (row(lead), row(path));
    let mut shown = format!("{lead}{path}");
    if number_width(ui, &shown, &font) > room {
        let (lead, path) = (lead.as_str(), path.as_str());
        // Whole folders go from the path's front first (`…/Copper Coast/Campaign.md`).
        let folders = path
            .char_indices()
            .filter(|(_, c)| matches!(c, '/' | '\\'))
            .map(|(at, _)| &path[at..])
            .find(|rest| number_width(ui, &format!("{lead}…{rest}"), &font) <= room);
        shown = match folders {
            Some(rest) => format!("{lead}…{rest}"),
            None => {
                // The final name alone does not fit: it gives way at its start.
                let last = path.rfind(['/', '\\']).map_or(path, |at| &path[at + 1..]);
                let chars: Vec<char> = last.chars().collect();
                let mut cut = format!("{lead}…{last}");
                for from in 0..chars.len() {
                    cut = format!("{lead}…{}", chars[from..].iter().collect::<String>());
                    if number_width(ui, &cut, &font) <= room {
                        break;
                    }
                }
                cut
            }
        };
    }
    let response = ui
        .add(egui::Label::new(Type::Small.rich(ui, shown).color(colors().muted)).extend())
        .on_hover_text(&preview);
    named(ui, &response, &preview);
}

/// A help line that names a path (`Now: …/campaigns/emberfall/Sessions`): one `Small` muted
/// line whatever the path, so a longer or a shorter path never moves what follows it. A path
/// that does not fit gives way from the front at a folder boundary and keeps its last part whole
/// (the path rule); its hover and its accessible description keep it whole.
///
/// The shortened line is kept (`PathLine`, in egui's memory under this `Ui`'s id and the
/// caller's `slot`): its owner is this help line's place. It holds while the path, the lead, the
/// room in whole points, the font and the display scale are the same; any change makes it again,
/// so an unchanged redraw does no shortening work.
pub fn path_help(
    ui: &mut Ui,
    slot: impl std::hash::Hash,
    lead: &str,
    path: &std::path::Path,
) -> Response {
    let font = Type::Small.font(ui);
    // Whole points: a fractional change of the room does not make the line again.
    let room = ui.available_width().floor();
    let scale = ui.ctx().pixels_per_point();
    let key = ui.id().with(("design_path_help", slot));
    let kept = ui
        .ctx()
        .data(|data| data.get_temp::<std::sync::Arc<PathLine>>(key))
        .filter(|line| {
            line.path == path
                && line.lead == lead
                && line.room == room
                && line.font == font
                && line.scale == scale
        });
    let line = match kept {
        Some(line) => line,
        None => {
            let line = std::sync::Arc::new(PathLine::make(ui, lead, path, room, &font, scale));
            ui.ctx()
                .data_mut(|data| data.insert_temp(key, line.clone()));
            line
        }
    };
    let response = ui
        .add(
            egui::Label::new(
                Type::Small
                    .rich(ui, line.shown.as_str())
                    .color(colors().muted),
            )
            .truncate(),
        )
        .on_hover_text(line.whole.as_str());
    ui.ctx()
        .accesskit_node_builder(response.id, |node| node.set_description(line.whole.clone()));
    response
}

/// A `path_help` line as drawn for one path, lead, room, font and display scale.
struct PathLine {
    path: std::path::PathBuf,
    lead: String,
    room: f32,
    font: egui::FontId,
    scale: f32,
    whole: String,
    shown: String,
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    static PATH_CUTS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// How many times `path_help` shortened a path on this thread (test builds).
#[cfg(any(test, feature = "test-support"))]
pub fn path_cuts() -> usize {
    PATH_CUTS.with(std::cell::Cell::get)
}

impl PathLine {
    fn make(
        ui: &Ui,
        lead: &str,
        path: &std::path::Path,
        room: f32,
        font: &egui::FontId,
        scale: f32,
    ) -> Self {
        #[cfg(any(test, feature = "test-support"))]
        PATH_CUTS.with(|cuts| cuts.set(cuts.get() + 1));
        let text = path.display().to_string();
        let whole = format!("{lead}{text}");
        let one_row = |text: &str| -> String {
            text.chars()
                .map(|c| if c.is_control() { ' ' } else { c })
                .collect()
        };
        let mut shown = one_row(&whole);
        if number_width(ui, &shown, font) > room {
            // Each cut starts at a separator, so the shown path starts with a whole folder; the
            // last cut keeps the last part whole when nothing shorter fits.
            for (at, _) in text.char_indices().filter(|(_, c)| *c == '/' || *c == '\\') {
                shown = one_row(&format!("{lead}…{}", &text[at..]));
                if number_width(ui, &shown, font) <= room {
                    break;
                }
            }
        }
        Self {
            path: path.to_owned(),
            lead: lead.to_owned(),
            room,
            font: font.clone(),
            scale,
            whole,
            shown,
        }
    }
}

/// A menu that opens under `response`, for a row or a control with more actions. The one place
/// a menu popup is made outside `row_menu`, so a placement rule for menus has one home.
///
/// Its room (X2): the menu stays inside the pane that `ui` draws in (its clip), one scroll
/// lane (`lane_width`, at least `S`) clear of the pane's edges, so it lies over neither a
/// splitter nor a scroll bar. It starts at the control's left edge and may reach that inner
/// edge; when the control's right edge leaves more room on its left, it ends there instead.
///
/// Its width (X2): the part decides it on every draw, not egui's saved size. A menu of
/// choices is as wide as its widest choice's words this frame, at most the room (the room is
/// measured again on every draw, so a pane made narrower while the menu was closed still holds
/// it). When the choices change while it is open, it measures again in the same frame. Other
/// content is cut at the room. Close and reopen measure afresh.
///
/// How a caller moves a raw `egui::Popup::menu` of choices here: replace
/// `egui::Popup::menu(&button).show(|ui| …)` with `design::menu_popup(ui, &button).show(|ui| …)`,
/// keep the same `choice`/`choice_wrapped` rows inside, and remove the row's `Ban::Layout` count.
pub fn menu_popup<'a, P: MenuPane + ?Sized>(pane: &P, response: &'a Response) -> MenuPopup<'a> {
    let pane = pane.pane();
    let (align, room) = menu_side(pane, response.rect);
    MenuPopup {
        id: egui::Popup::default_response_id(response),
        popup: menu_popup_raw(MenuAnchor::Control(response))
            .align(align)
            .width(room),
        room,
        pane,
        control: response.rect,
    }
}

/// A menu that opens at `anchor`, a place in `ui`'s pane that is not a control (Sessions' `/`
/// menu opens at the note's text cursor, M0536), named by `id`. Its room, side, width, keys and
/// close rules are [`menu_popup`]'s: it stays inside `ui`'s pane, one scroll lane clear of the
/// pane's edges, so it lies over neither a splitter nor a scroll bar, and it opens above the
/// anchor when there is no room below. No click opens it: the caller opens it with
/// [`MenuPopup::open_memory`] or [`MenuPopup::open`], and closes it with
/// `egui::Popup::close_id(ctx, id)` as egui's own menus close.
pub fn menu_popup_at(ui: &Ui, id: Id, anchor: Rect) -> MenuPopup<'static> {
    let pane = ui.pane();
    let (align, room) = menu_side(pane, anchor);
    MenuPopup {
        id,
        popup: menu_popup_raw(MenuAnchor::At {
            id,
            ctx: ui.ctx().clone(),
            rect: anchor,
            layer: ui.layer_id(),
        })
        .align(align)
        .width(room),
        room,
        pane,
        control: anchor,
    }
}

/// What a menu opens beside: a control (it toggles the menu when pressed), or a place.
enum MenuAnchor<'a> {
    Control(&'a Response),
    At {
        id: Id,
        ctx: egui::Context,
        rect: Rect,
        layer: egui::LayerId,
    },
}

/// Where a menu's room comes from: the `Ui` its control is drawn in (that pane's clip), or,
/// for a caller with no `Ui`, the `Context` (the screen).
pub trait MenuPane: menu_pane::Sealed {
    fn pane(&self) -> Rect;
}

mod menu_pane {
    pub trait Sealed {}
    impl Sealed for egui::Ui {}
    impl Sealed for egui::Context {}
}

impl MenuPane for Ui {
    fn pane(&self) -> Rect {
        self.clip_rect()
    }
}

impl MenuPane for egui::Context {
    fn pane(&self) -> Rect {
        self.content_rect()
    }
}

/// The side a menu opens on beside `control` in `pane`, and its room there: rightward from the
/// control's left edge, unless its right edge leaves more room leftward.
fn menu_side(pane: Rect, control: Rect) -> (egui::RectAlign, f32) {
    let rightward = menu_room(pane, control, egui::RectAlign::BOTTOM_START);
    let leftward = menu_room(pane, control, egui::RectAlign::BOTTOM_END);
    if rightward >= leftward {
        (egui::RectAlign::BOTTOM_START, rightward)
    } else {
        (egui::RectAlign::BOTTOM_END, leftward)
    }
}

/// A menu's room in `pane` beside `control` for `align`: from the control's left edge to the
/// pane's inner right edge (or from its right edge leftward), one lane clear of the edges. It is
/// never wider than that, also when that is narrower than a control (issue #549): a narrow
/// room makes a narrow menu, and a room with no space for the menu's frame shows none.
fn menu_room(pane: Rect, control: Rect, align: egui::RectAlign) -> f32 {
    let clear = lane_width().max(S);
    let room = if align.parent().x() == egui::Align::Max {
        control.right() - (pane.left() + clear)
    } else {
        pane.right() - clear - control.left()
    };
    room.max(0.0)
}

#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
fn menu_popup_raw(anchor: MenuAnchor<'_>) -> egui::Popup<'_> {
    match anchor {
        MenuAnchor::Control(response) => egui::Popup::menu(response),
        // `Popup::menu`'s form without its toggle: a menu's kind, layout, style and no gap.
        MenuAnchor::At {
            id,
            ctx,
            rect,
            layer,
        } => egui::Popup::new(id, ctx, rect, layer)
            .kind(egui::PopupKind::Menu)
            .layout(egui::Layout::top_down_justified(egui::Align::Min))
            .style(egui::containers::menu::menu_style)
            .gap(0.0),
    }
}

/// The submenus of a menu that a click inside does not close ([`MenuPopup::show_submenus`]).
pub struct Submenus {
    _by_its_menu: (),
}

impl Submenus {
    /// Its submenus, each a row of the menu with its word and caret, whose choices open under it
    /// in the menu, each on its own; `contents(ui, at)` draws an open one's choices, and `width`
    /// is the widest submenu's choices, so the menu keeps one width.
    pub fn show<W: AsRef<str>>(
        &mut self,
        ui: &mut Ui,
        id: impl std::hash::Hash,
        width: f32,
        words: &[W],
        contents: impl FnMut(&mut Ui, usize),
    ) {
        super::components::menu_submenus(ui, id, width, words, contents);
    }
}

/// A menu from [`menu_popup`]: `show` draws its content at the width the part decides.
#[must_use = "call `show` to draw the menu"]
pub struct MenuPopup<'a> {
    popup: egui::Popup<'a>,
    id: Id,
    /// The menu's largest outer width: its room in the pane.
    room: f32,
    pane: Rect,
    control: Rect,
}

impl MenuPopup<'_> {
    /// Open it at a side of its control; its room is measured again for that side.
    pub fn align(mut self, align: egui::RectAlign) -> Self {
        self.room = menu_room(self.pane, self.control, align);
        self.popup = self.popup.align(align).width(self.room);
        self
    }

    /// A smaller room owned by the control's layout (as [`Dropdown::within`](super::popups::Dropdown::within)):
    /// a column of a header that spans two panes, the list beside a splitter. The part
    /// intersects it with the pane, then chooses the menu's side and width inside it again, one
    /// lane clear of its edges. It cannot widen the pane's room. A control that reaches past
    /// the room's inner edges anchors the menu at those edges, so that the menu never starts
    /// past the room (issue #549); a room too narrow for the menu's frame shows none.
    pub fn within(mut self, room: Rect) -> Self {
        self.pane = self.pane.intersect(room);
        let clear = lane_width().max(S);
        let (left, right) = (self.pane.left() + clear, self.pane.right() - clear);
        self.control.min.x = self.control.min.x.max(left).min(right);
        self.control.max.x = self.control.max.x.min(right).max(left);
        self.popup = self
            .popup
            .anchor(egui::PopupAnchor::ParentRect(self.control));
        let (align, width) = menu_side(self.pane, self.control);
        self.room = width;
        self.popup = self.popup.align(align).width(width);
        self
    }

    /// Open or close it by the caller's flag (egui's `Popup::open`).
    pub fn open(mut self, open: bool) -> Self {
        self.popup = self.popup.open(open);
        self
    }

    /// Open, close or toggle it by a command (egui's `Popup::open_memory`).
    pub fn open_memory(mut self, command: Option<egui::SetOpenCommand>) -> Self {
        self.popup = self.popup.open_memory(command);
        self
    }

    /// The width egui first measures the menu in (egui's `Popup::width`); at most its room. The
    /// part still decides the width it draws at.
    pub fn width(mut self, width: f32) -> Self {
        self.popup = self.popup.width(width.min(self.room));
        self
    }

    /// The menu's size when it last drew, if it did.
    pub fn get_expected_size(&self) -> Option<egui::Vec2> {
        self.popup.get_expected_size()
    }

    /// When a click closes the menu (egui's `PopupCloseBehavior`).
    pub fn close_behavior(mut self, behavior: egui::PopupCloseBehavior) -> Self {
        self.popup = self.popup.close_behavior(behavior);
        self
    }

    /// A menu of several submenus (M0051, the creature Filter's facts): `content` draws the
    /// menu, and its submenus through [`Submenus::show`]. Its rows and choices keep it open; a
    /// click outside it or Escape closes it. Only this menu makes a [`Submenus`], so no menu that
    /// a click inside closes holds one.
    pub fn show_submenus<R>(
        self,
        content: impl FnOnce(&mut Ui, &mut Submenus) -> R,
    ) -> Option<egui::InnerResponse<R>> {
        self.close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
            .show(|ui| content(ui, &mut Submenus { _by_its_menu: () }))
    }

    /// Draw the menu while it is open.
    pub fn show<R>(self, content: impl FnOnce(&mut Ui) -> R) -> Option<egui::InnerResponse<R>> {
        let Self {
            popup, id, room, ..
        } = self;
        let ctx = popup.ctx().clone();
        let key = id.with("splat_menu_width");
        let style = ctx.style();
        let frame = style.spacing.menu_margin.sum().x + 2.0 * style.visuals.window_stroke.width;
        // A room that cannot hold the frame and anything in it shows no menu, rather than one
        // drawn past the room (issue #549). It opens when the room is there again.
        if room <= frame {
            ctx.data_mut(|data| data.remove_temp::<f32>(key));
            return None;
        }
        let shown = popup.show(|ui| {
            let inner_room = (room - frame).max(0.0);
            let decided = ui.data(|data| data.get_temp::<f32>(key));
            let width = decided.unwrap_or(inner_room).min(inner_room);
            let top = ui.cursor().min;
            let area = Rect::from_min_size(top, vec2(width, ui.ctx().content_rect().height()));
            // At an opening the part measures: a sizing pass, in which widgets take their own
            // size (egui keeps a reopened menu's old size and gives it no sizing pass of its own).
            let mut builder = UiBuilder::new().max_rect(area).layout(*ui.layout());
            if decided.is_none() {
                builder = builder.sizing_pass().invisible();
            }
            let mut child = ui.new_child(builder);
            child.set_clip_rect(child.clip_rect().intersect(area));
            let outer = ui.data_mut(|data| data.remove_temp::<MenuMeasure>(menu_measure_id()));
            ui.data_mut(|data| data.insert_temp(menu_measure_id(), MenuMeasure::default()));
            let inner = content(&mut child);
            let measured = ui
                .data_mut(|data| data.remove_temp::<MenuMeasure>(menu_measure_id()))
                .unwrap_or_default();
            if let Some(outer) = outer {
                ui.data_mut(|data| data.insert_temp(menu_measure_id(), outer));
            }
            let used = child.min_rect();
            let wanted = if measured.any {
                measured.widest
            } else {
                used.width()
            };
            let next = wanted.min(inner_room);
            // A new width (an opening, changed choices, a narrower pane) draws the pass again.
            // This pass, which egui discards, already allocates the new width: egui saves the
            // menu's size at the end of a pass and places the next pass with it, so the drawn
            // pass is anchored with the size it draws at.
            let remeasure = (next - width).abs() > 0.5 || decided.is_none();
            let allocated = if remeasure { next } else { width };
            ui.allocate_rect(
                Rect::from_min_size(top, vec2(allocated, used.height())),
                Sense::hover(),
            );
            if remeasure {
                ui.data_mut(|data| data.insert_temp(key, next));
                ui.ctx().request_discard("a menu measured its width");
            }
            inner
        });
        if shown.is_none() {
            ctx.data_mut(|data| data.remove_temp::<f32>(key));
        }
        shown
    }
}

/// The widest choice drawn in the menu being drawn, so the menu can size itself to its choices.
#[derive(Clone, Copy, Default)]
struct MenuMeasure {
    widest: f32,
    any: bool,
}

fn menu_measure_id() -> Id {
    Id::new("splat_menu_measure")
}

/// Draw `add` as one block of the menu being drawn, `width` wide (a menu's submenus, with their
/// wrapped choices and their scroll lane, M0051): the choices in it note no width of their own,
/// and the block notes its whole width, so the menu is as wide as the block (review of PR 759).
pub(super) fn menu_block<R>(ui: &mut Ui, width: f32, add: impl FnOnce(&mut Ui) -> R) -> R {
    let outer = ui.data_mut(|data| data.remove_temp::<MenuMeasure>(menu_measure_id()));
    let inner = add(ui);
    if let Some(mut outer) = outer {
        outer.widest = outer.widest.max(width);
        outer.any = true;
        ui.data_mut(|data| data.insert_temp(menu_measure_id(), outer));
    }
    inner
}

/// A choice notes its words' width (`natural`) for the menu it is drawn in, if any.
pub(super) fn note_choice_width(ui: &Ui, natural: f32) {
    ui.data_mut(|data| {
        if let Some(mut measure) = data.get_temp::<MenuMeasure>(menu_measure_id()) {
            measure.widest = measure.widest.max(natural);
            measure.any = true;
            data.insert_temp(menu_measure_id(), measure);
        }
    });
}

// The popup and status parts, public here so their doc tests can name them
// (`splat::layout::Dropdown`).
pub use super::popups::{
    Dropdown, DropdownResponse, Ghost, HasChoices, ImageSize, NoChoice, TIP_WIDTH, TipLine,
    drag_source, fitted, image_viewer, rich_tip,
};
pub use super::status::{KindStatus, StatusKind, Tinted};

#[cfg(test)]
pub(crate) mod bans;
#[cfg(test)]
mod tests;
