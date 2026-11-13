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
use std::{hash::Hash, marker::PhantomData, ops::Range};

use eframe::egui::{
    self, Align, Id, Layout, Rect, Response, Sense, TextBuffer, TextEdit, Ui, UiBuilder,
    scroll_area::ScrollAreaOutput, vec2,
};

use super::{
    Action, Density, FORM_WIDTH, L, M, PANEL_MIN_WIDTH, S, Status, Type, XS, colors, extras::space,
    field, named, number_width, placeholder, theme,
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
/// const EDIT: splat::layout::Word = splat::word!("Edit");
/// assert_eq!(EDIT.as_str(), "Edit");
/// ```
///
/// An empty word, one that does not start with a capital, one with a space at its end and one
/// longer than the limit do not compile:
///
/// ```compile_fail,E0080
/// let _ = splat::word!("");
/// ```
///
/// ```compile_fail,E0080
/// let _ = splat::word!("edit");
/// ```
///
/// ```compile_fail,E0080
/// let _ = splat::word!("Edit ");
/// ```
///
/// ```compile_fail,E0080
/// let _ = splat::word!("Add every creature to the fight");
/// ```
///
/// A word made at run time, or held in a constant, is not a literal:
///
/// ```compile_fail
/// let name = String::from("Edit");
/// let _ = splat::word!(name.as_str());
/// ```
///
/// ```compile_fail
/// const NAME: &str = "Edit";
/// let _ = splat::word!(NAME);
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
    /// let _ = splat::layout::TextKey::of("journey");
    /// ```
    ///
    /// ```compile_fail,E0277
    /// let _ = splat::layout::TextKey::of(&format!("row {}", 1));
    /// ```
    ///
    /// No public owner type holds a free text (review 1: `DocOwner` did):
    ///
    /// ```compile_fail,E0422
    /// let _ = splat::layout::TextKey::of(&splat::layout::DocOwner { id: format!("row_{}", 1) });
    /// ```
    ///
    /// A caller's own type cannot become an owner (it would turn any text into a key with no
    /// counted row):
    ///
    /// ```compile_fail,E0277
    /// struct Row(String);
    /// impl splat::layout::TextOwner for Row {
    ///     fn owner_text(&self) -> &str {
    ///         &self.0
    ///     }
    /// }
    /// ```
    pub fn of(owner: &impl TextOwner) -> Self {
        Self(owner.owner_text().to_owned())
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
text_owners!(crate::timeline::Event => |event| &event.id);

macro_rules! row_keys {
    ($($key:ty),*) => {$(
        impl sealed::Sealed for $key {}
        impl RowKey for $key {}
    )*};
}
row_keys!(uuid::Uuid, RowIndex, TextKey);
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
/// use splat::layout::{FieldName, Once, RowIndex};
/// const NAME: FieldName = splat::field_id!("doc_name");
/// const COOKIE: splat::layout::FieldId = splat::field_id!("doc_cookie").once(Once::Settings);
/// fn f(ui: &mut eframe::egui::Ui, text: &mut String) {
///     let _ = NAME.of(RowIndex::from_data(1)).single_line(ui, text, |edit| edit);
///     let _ = COOKIE.single_line(ui, text, |edit| edit);
/// }
/// ```
///
/// A name with no owner choice draws nothing:
///
/// ```compile_fail,E0599
/// fn f(ui: &mut eframe::egui::Ui, text: &mut String) {
///     let _ = splat::field_id!("doc_name").single_line(ui, text, |edit| edit);
/// }
/// ```
///
/// ```compile_fail,E0308
/// fn f(ui: &mut eframe::egui::Ui, text: &mut String) {
///     let _ = splat::layout::line_field(ui, splat::field_id!("doc_name"), "Name", text, "", 80.0);
/// }
/// ```
///
/// A `Once` reason is one of the listed words, not a text:
///
/// ```compile_fail,E0308
/// let _ = splat::field_id!("doc_name").once("Settings");
/// ```
///
/// An id or a place from the layout is not an owner's key:
///
/// ```compile_fail,E0277
/// fn f(ui: &mut eframe::egui::Ui) {
///     let _ = splat::field_id!("doc_name").of(ui.next_auto_id());
/// }
/// ```
///
/// ```compile_fail,E0277
/// fn f(ui: &mut eframe::egui::Ui) {
///     let _ = splat::field_id!("doc_name").of(ui.cursor().min.y as usize);
/// }
/// ```
///
/// A path or a name is not an identity: a save or a rename changes it while the DM types, and
/// the field loses its focus, cursor and Undo. Neither is a key (a path can only be the counted
/// escape `TextKey::from_data`):
///
/// ```compile_fail,E0277
/// let _ = splat::field_id!("doc_name").of(std::path::Path::new("Groups/Wolves.md"));
/// ```
///
/// ```compile_fail,E0277
/// let _ = splat::field_id!("doc_name").of(std::path::PathBuf::from("Groups/Wolves.md"));
/// ```
///
/// ```compile_fail,E0277
/// let _ = splat::field_id!("doc_name").of("Wolves");
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
/// use splat::layout::{FieldName, RowIndex};
/// const NAME: FieldName = splat::field_id!("doc_name");
/// assert_ne!(NAME.of(RowIndex::from_data(1)).id(), NAME.of(RowIndex::from_data(2)).id());
/// ```
///
/// A row index is made only by its named constructor:
///
/// ```compile_fail,E0603
/// let _ = splat::layout::RowIndex(1);
/// ```
///
/// An `egui::Id` is not a [`RowKey`], and neither is a bare text:
///
/// ```compile_fail,E0277
/// const NAME: splat::layout::FieldName = splat::field_id!("doc_name");
/// let _ = NAME.of(eframe::egui::Id::new("place"));
/// ```
///
/// ```compile_fail,E0277
/// const NAME: splat::layout::FieldName = splat::field_id!("doc_name");
/// let _ = NAME.of(format!("{:?}", eframe::egui::Id::new("place")));
/// ```
///
/// An id from the layout is not a `FieldId`:
///
/// ```compile_fail,E0308
/// fn take(_: splat::layout::FieldId) {}
/// take(eframe::egui::Id::new("doc_name"));
/// ```
///
/// ```compile_fail,E0423
/// let _ = splat::layout::FieldId(eframe::egui::Id::new("doc_name"));
/// ```
///
/// ```compile_fail,E0080
/// let _ = splat::field_id!("");
/// ```
///
/// A name held in a constant escapes the lint's scan, so it does not compile:
///
/// ```compile_fail
/// const NAME: &str = "doc_name";
/// let _ = splat::field_id!(NAME);
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
    /// egui's text Undo for this id is dropped before Ctrl+Z can reach it. So another object's
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
        let guard = TextGuard::before(ui, id, text.as_str());
        let response = ui.add(style(TextEdit::singleline(text).id(id)));
        guard.after(ui, text.as_str(), &response);
        record_field(ui, &response);
        response
    }

    /// A multi-line editor with this id, for a caller that reads where the text was laid out
    /// (the galley, its place, the cursor, the clip rect): a Markdown source, a repair editor.
    /// It guards the text history as [`single_line`](Self::single_line) does.
    ///
    /// ```
    /// fn f(ui: &mut eframe::egui::Ui, note: &mut String) {
    ///     const NOTE: splat::layout::FieldName = splat::field_id!("doc_note");
    ///     let id = NOTE.once(splat::layout::Once::Example);
    ///     let editor = id.editor(ui, note, |edit| edit.desired_rows(4));
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
        text: &mut dyn TextBuffer,
        style: impl for<'t> FnOnce(TextEdit<'t>) -> TextEdit<'t>,
    ) -> Editor {
        let id = self.id();
        let guard = TextGuard::before(ui, id, text.as_str());
        let output = style(TextEdit::multiline(text).id(id)).show(ui);
        guard.after(ui, text.as_str(), &output.response);
        record_field(ui, &output.response);
        Editor {
            response: output.response,
            galley: output.galley,
            galley_pos: output.galley_pos,
            text_clip_rect: output.text_clip_rect,
            cursor_range: output.cursor_range,
        }
    }

    /// A search field with this id: the search icon, the hint (what can be searched) and, while
    /// it holds text, a quiet `Clear` after it on its line, inside `width`. `Clear` empties the
    /// text and gives the field its focus back; `cleared` says so, for a caller that resets more
    /// (the selected result). A search field has no form without its `Clear` (N10).
    ///
    /// ```
    /// fn f(ui: &mut eframe::egui::Ui, query: &mut String) {
    ///     use splat::layout::{FieldId, Once};
    ///     const SEARCH: FieldId = splat::field_id!("doc_search").once(Once::PageSearch);
    ///     let search = SEARCH.search(ui, "Search", query, "Search by name…", 240.0);
    ///     let _ = (search.field, search.cleared);
    /// }
    /// ```
    ///
    /// `Clear` is not the caller's to leave out:
    ///
    /// ```compile_fail,E0599
    /// fn f(ui: &mut eframe::egui::Ui, query: &mut String) {
    ///     use splat::layout::{FieldId, Once};
    ///     const SEARCH: FieldId = splat::field_id!("doc_search").once(Once::PageSearch);
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
        let clear = || Action::quiet("Clear").density(Density::Scan);
        let mut cleared = false;
        let id = self.named(name).id();
        let field = ui
            .horizontal(|ui| {
                let room = if text.is_empty() {
                    0.0
                } else {
                    clear().natural_width(ui) + ui.spacing().item_spacing.x
                };
                let guard = TextGuard::before(ui, id, text);
                let field = super::components::search_box(
                    ui,
                    id,
                    name,
                    text,
                    hint,
                    (width - room).max(Density::Scan.control()),
                );
                guard.after(ui, text, &field);
                record_field(ui, &field);
                if !text.is_empty() && clear().show(ui).clicked() {
                    text.clear();
                    field.request_focus();
                    cleared = true;
                }
                field
            })
            .inner;
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
        let guard = TextGuard::before(ui, id, text.as_str());
        let response = ui.add(style(TextEdit::multiline(text).id(id)));
        guard.after(ui, text.as_str(), &response);
        record_field(ui, &response);
        response
    }
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
}

/// What the bound text was when the field last drew with focus: its length and a hash.
#[derive(Clone, Copy, PartialEq, Eq)]
struct BoundText {
    len: usize,
    hash: u64,
}

impl BoundText {
    /// One hash of the text (egui itself copies a focused field's whole text into its Undo
    /// history every frame it has focus, so this adds less than egui does).
    fn of(text: &str) -> Self {
        #[cfg(test)]
        BOUND_TEXT_HASHES.with(|count| count.set(count.get() + 1));
        Self {
            len: text.len(),
            hash: egui::util::hash(text),
        }
    }
}

#[cfg(test)]
thread_local! {
    /// How many times a field part hashed its text, for the cost proof.
    pub(crate) static BOUND_TEXT_HASHES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// How many fields keep a record. The oldest record goes first, and egui's Undo history for
/// its field goes with it, so a field with no record never has old history.
pub(crate) const BOUND_TEXTS: usize = 32;

/// The records, newest last. At most [`BOUND_TEXTS`].
#[derive(Clone, Default)]
struct BoundTexts(Vec<(Id, BoundText)>);

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
    fn before(ui: &Ui, id: Id, text: &str) -> Self {
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
            // The cursor is stale too: it points into the old text.
            Self::drop_history(ui.ctx(), id, true);
            Self::forget(ui.ctx(), id);
        }
        Self { id, focused_before }
    }

    fn after(self, ui: &Ui, text: &str, response: &Response) {
        if !response.has_focus() {
            return;
        }
        // Focus came this frame by a way the check before the draw did not see: egui may have
        // fed the old history with this text. The cursor is new (from the click), so it stays.
        if !self.focused_before && Self::stale(ui, self.id, text) {
            Self::drop_history(ui.ctx(), self.id, false);
        }
        let bound = BoundText::of(text);
        let ctx = ui.ctx();
        let oldest = ctx.data_mut(|data| {
            let records = &mut data
                .get_temp_mut_or_default::<BoundTexts>(bound_texts_id())
                .0;
            records.retain(|(id, _)| *id != self.id);
            records.push((self.id, bound));
            (records.len() > BOUND_TEXTS).then(|| records.remove(0).0)
        });
        if let Some(oldest) = oldest {
            Self::drop_history(ctx, oldest, false);
        }
    }

    /// Whether the field last had focus with another text.
    fn stale(ui: &Ui, id: Id, text: &str) -> bool {
        let Some(bound) = ui.ctx().data(|data| {
            data.get_temp::<BoundTexts>(bound_texts_id())
                .and_then(|records| {
                    records
                        .0
                        .iter()
                        .find(|(record, _)| *record == id)
                        .map(|(_, bound)| *bound)
                })
        }) else {
            return false;
        };
        bound.len != text.len() || bound != BoundText::of(text)
    }

    fn forget(ctx: &egui::Context, id: Id) {
        ctx.data_mut(|data| {
            data.get_temp_mut_or_default::<BoundTexts>(bound_texts_id())
                .0
                .retain(|(record, _)| *record != id);
        });
    }

    fn drop_history(ctx: &egui::Context, id: Id, cursor: bool) {
        if let Some(mut state) = egui::text_edit::TextEditState::load(ctx, id) {
            state.clear_undoer();
            if cursor {
                state.cursor.set_char_range(None);
            }
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
    let guard = TextGuard::before(ui, id, text);
    let response = draw(ui, text);
    guard.after(ui, text, &response);
    response
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
        form_field(ui, id, label, text, width)
    })
}

/// The form field of `labelled_field` and `InlineCreate`, `width` wide without its margins.
fn form_field(ui: &mut Ui, id: FieldId, label: &str, text: &mut String, width: f32) -> Response {
    let response = id.named(label).single_line(ui, text, |edit| {
        edit.desired_width(width)
            .min_size(vec2(0.0, Density::Read.control()))
            .margin(vec2(S, 5.0))
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
/// fn list(ui: &mut eframe::egui::Ui) {
///     splat::layout::ScrollColumn::new("list")
///         .max_height(200.0)
///         .show(ui, |ui| ui.label("Row"));
/// }
/// ```
///
/// There is no way to give the lane back:
///
/// ```compile_fail,E0599
/// let _ = splat::layout::ScrollColumn::new("list").without_lane();
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
        }
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
        .auto_shrink([false, self.fit_height]);
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
        let (sideways, full) = (self.sideways, self.full_width);
        Self::styled(ui, |ui, width| {
            self.area().show(ui, |ui| {
                if !sideways {
                    ui.set_max_width(width);
                }
                if full {
                    ui.set_min_width(width);
                }
                add(ui)
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
        Self::styled(ui, |ui, width| {
            let full = self.full_width;
            self.area().show_rows(ui, row_height, rows, |ui, range| {
                ui.set_max_width(width);
                if full {
                    ui.set_min_width(width);
                }
                add(ui, range)
            })
        })
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
        Self::styled(ui, |ui, width| {
            let full = self.full_width;
            self.area().show_viewport(ui, |ui, viewport| {
                ui.set_max_width(width);
                if full {
                    ui.set_min_width(width);
                }
                add(ui, viewport)
            })
        })
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

/// A sideways scroll for one wide table or strip: it never scrolls up and down, so it sits
/// inside a page's own `ScrollColumn`.
#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
pub fn sideways_scroll<R>(
    ui: &mut Ui,
    id_salt: impl Hash,
    add: impl FnOnce(&mut Ui) -> R,
) -> ScrollAreaOutput<R> {
    egui::ScrollArea::horizontal()
        .id_salt(Id::new(id_salt))
        .auto_shrink([false, true])
        .show(ui, add)
}

/// Fixed room for a text value that changes in place: as wide as the widest example in its
/// type, so the next control never moves when the value changes. A value wider than every
/// example is cut with an ellipsis (the gate's `cut_text` check finds it); it never pushes.
/// The examples are an array whose length is checked when the program compiles, so a slot with
/// none does not compile, at run time too.
///
/// ```
/// use splat::layout::{Slot, SlotType};
/// const SAVES: Slot = Slot::new(&["0 of 3", "3 of 3"], SlotType::Small);
/// let _ = Slot::new(&["Low"], SlotType::Body);
/// ```
///
/// ```compile_fail,E0080
/// use splat::layout::{Slot, SlotType};
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

/// How a title-line action looks. There is no form without its word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TitleWeight {
    Quiet,
    Secondary,
}

/// One action on a title line. It always shows its [`Word`]; an icon goes before the word.
///
/// ```
/// use splat::layout::TitleAction;
/// let _ = TitleAction::quiet(splat::word!("Edit")).name("Edit Cinder Wolf");
/// ```
///
/// A word is a checked `Word`, never a plain string:
///
/// ```compile_fail,E0308
/// let _ = splat::layout::TitleAction::quiet("Edit");
/// ```
#[derive(Clone, Copy, Debug)]
#[must_use]
pub struct TitleAction<'a> {
    word: Word,
    weight: TitleWeight,
    icon: Option<&'static str>,
    blocked: Option<&'a str>,
    name: Option<&'a str>,
}

impl<'a> TitleAction<'a> {
    pub fn quiet(word: Word) -> Self {
        Self {
            word,
            weight: TitleWeight::Quiet,
            icon: None,
            blocked: None,
            name: None,
        }
    }

    pub fn secondary(word: Word) -> Self {
        Self {
            weight: TitleWeight::Secondary,
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

    fn action(&self) -> Action<'a> {
        let mut action = match self.weight {
            TitleWeight::Quiet => Action::quiet(self.word.as_str()),
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
}

/// The type of a title line's title.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TitleType {
    /// A page or pane title.
    Title,
    /// A reader's or a panel's title.
    Section,
}

/// What a title line did this frame.
#[derive(Clone, Debug, Default)]
pub struct TitleLineResponse {
    pressed: Option<Word>,
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
#[cfg(test)]
const TITLE_ACTION_ROWS: &str = "design_title_action_rows";

/// Register a row of actions drawn under a title. Only test builds keep it.
pub fn register_title_row(ui: &Ui, row: TitleActionRow) {
    #[cfg(test)]
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
    #[cfg(not(test))]
    let _ = (ui, row);
}

/// Each action row drawn under a title in the last completed pass, in draw order; empty when
/// that pass drew none (a row never outlives its title line). Package G reads them through
/// `Driver::title_action_rows`.
#[cfg(test)]
pub fn title_action_rows(ctx: &egui::Context) -> Vec<TitleActionRow> {
    let last = ctx.cumulative_pass_nr().saturating_sub(1);
    ctx.data(|data| data.get_temp::<(u64, Vec<TitleActionRow>)>(Id::new(TITLE_ACTION_ROWS)))
        .filter(|(pass, _)| *pass == last)
        .map(|(_, rows)| rows)
        .unwrap_or_default()
}

/// A title line's widths, measured once per frame and used by both the placement and the draw.
struct TitleMetrics {
    title: f32,
    /// The context's width, with the gap before it.
    context: f32,
    lead_in: f32,
    widths: Vec<f32>,
}

impl TitleMetrics {
    fn beside(&self) -> f32 {
        self.title + self.context
    }
}

impl<'a> TitleLine<'a> {
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            context: None,
            role: TitleType::Title,
            actions: Vec::new(),
        }
    }

    pub fn role(mut self, role: TitleType) -> Self {
        self.role = role;
        self
    }

    /// One muted `Small` state or count after the title, `XS` after it.
    pub fn context(mut self, context: Option<&'a str>) -> Self {
        self.context = context;
        self
    }

    pub fn action(mut self, action: TitleAction<'a>) -> Self {
        self.actions.push(action);
        self
    }

    fn font(&self) -> Type {
        match self.role {
            TitleType::Title => Type::Title,
            TitleType::Section => Type::Section,
        }
    }

    fn metrics(&self, ui: &Ui) -> TitleMetrics {
        TitleMetrics {
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
                .map(|action| action.action().natural_width(ui).ceil())
                .collect(),
        }
    }

    /// Draw one action in exactly its measured target.
    fn action_at(&self, ui: &mut Ui, at: usize, rect: Rect) -> bool {
        ui.scope_builder(
            UiBuilder::new()
                .max_rect(rect)
                .layout(Layout::left_to_right(Align::Center)),
            |ui| self.actions[at].action().show(ui).clicked(),
        )
        .inner
    }

    pub fn show(self, ui: &mut Ui) -> TitleLineResponse {
        let metrics = self.metrics(ui);
        let room = ui.available_width();
        let placement = title_placement(
            metrics.beside(),
            metrics.lead_in,
            &metrics.widths,
            room,
            room,
        );
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
        let rows = match placement {
            TitlePlacement::OnTitle => {
                let line_height = height.max(font.line());
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
                ui.put(piece(metrics.title), title.extend());
                if let Some((words, context)) = context {
                    let _ = piece(XS);
                    ui.put(piece(metrics.context - XS), context.extend())
                        .on_hover_text(words);
                }
                let _ = piece(metrics.lead_in);
                for (at, width) in metrics.widths.iter().enumerate() {
                    if at > 0 {
                        let _ = piece(ACTION_GAP);
                    }
                    if self.action_at(ui, at, piece(*width)) {
                        pressed = Some(self.actions[at].word);
                    }
                }
                0
            }
            TitlePlacement::Under(rows) => {
                let title_rect = ui
                    .horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing.x = XS;
                        ui.add(title.wrap());
                        if let Some((words, context)) = context {
                            ui.add(context.wrap()).on_hover_text(words);
                        }
                    })
                    .response
                    .rect;
                for row in &rows {
                    let (line, _) = ui.allocate_exact_size(vec2(room, height), Sense::hover());
                    let mut row_ui = ui.new_child(
                        UiBuilder::new()
                            .max_rect(line)
                            .layout(Layout::left_to_right(Align::Center)),
                    );
                    let mut x = line.left();
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
        TitleLineResponse {
            pressed,
            below: rows > 0,
            rows,
        }
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
/// splat::create_reasons! {
///     pub enum NewEntry {
///         NoName => "Enter a name first.",
///         Taken => "An entry with this name exists. Choose another name.",
///     }
/// }
/// use splat::layout::CreateReasons;
/// assert_eq!(NewEntry::ALL.len(), 2);
/// ```
///
/// ```compile_fail,E0080
/// splat::create_reasons! {
///     pub enum NewEntry { NoName => "Enter a name first" }
/// }
/// let _ = splat::layout::CreateReasons::words(NewEntry::NoName);
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
/// use splat::layout::{Create, NoReason};
/// let _: Create<NoReason> =
///     Create::primary(splat::word!("Create")).blocked(Some("Enter a name first."));
/// ```
#[derive(Clone, Copy, Debug)]
#[must_use]
pub struct Create<R: CreateReasons> {
    word: Word,
    weight: CreateWeight,
    blocked: Option<R>,
}

impl<R: CreateReasons> Create<R> {
    pub fn primary(word: Word) -> Self {
        Self {
            word,
            weight: CreateWeight::Primary,
            blocked: None,
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

    fn action(&self) -> Action<'static> {
        match self.weight {
            CreateWeight::Primary => Action::primary(self.word.as_str()),
            CreateWeight::Secondary => Action::secondary(self.word.as_str()),
        }
        .density(Density::Read)
        .blocked(self.blocked.map(R::words))
    }
}

/// One text field of an [`InlineCreate`].
#[must_use]
pub struct CreateField<'a> {
    id: FieldId,
    label: &'a str,
    text: &'a mut String,
    help: Option<&'a str>,
    side: Option<Word>,
    focus: bool,
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
        }
    }

    pub fn help(mut self, help: &'a str) -> Self {
        self.help = Some(help);
        self
    }

    /// A secondary action beside the field, before Create (`Browse…`).
    pub fn side_action(mut self, word: Word) -> Self {
        self.side = Some(word);
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

/// The line below the last field: quiet Cancel at its start when there is something to cancel,
/// then Create's blocked reason, or, once nothing blocks Create, the preview. At one width its
/// height is fixed for the life of the form: as tall as a control, or as the tallest declared
/// reason wrapped beside Cancel at that width, whichever is taller. So nothing below it moves
/// while the DM types or a reason comes and goes. A resize may change its height, as it changes
/// every wrapped text.
#[derive(Clone, Copy, Debug, Default)]
#[must_use]
pub struct KeptLine<'a> {
    cancel: Option<(Word, bool)>,
    preview: Option<&'a str>,
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

    /// What Create will make (`Will create NPCs/Watchkeeper.md`), shown once nothing blocks it.
    /// It keeps one line: a long one gives way at its start, and its hover and accessible name
    /// keep it whole.
    pub fn preview(mut self, preview: Option<&'a str>) -> Self {
        self.preview = preview;
        self
    }
}

enum CreatePart<'a> {
    Text(CreateField<'a>),
    Choice(CreateChoice<'a>),
}

/// Typestate of an [`InlineCreate`]: no part yet.
pub struct Empty;
/// Typestate of an [`InlineCreate`]: the last part is a choice; a text field must follow.
pub struct AfterChoice;
/// Typestate of an [`InlineCreate`]: the last part is a text field; Create may follow.
pub struct AfterField;
/// Typestate of an [`InlineCreate`]: Create is named; the kept line must follow.
pub struct Named;
/// Typestate of an [`InlineCreate`]: complete.
pub struct Complete;

/// The one inline creation layout: the fields (and choices) in order, each with its help; the
/// named Create after the last field on that field's line (after its side action), then the last
/// field's help, then one kept line. Help under an earlier field has `L` before the next label.
/// Nothing moves while the DM types: Create keeps its word, and at one width the kept line keeps
/// its height for the life of the form (a resize may change it, as it changes wrapped text).
///
/// The order is in the type: `create` follows a text field, `kept_line` follows `create`, and
/// only a complete form has `show`.
///
/// ```
/// use splat::layout::{Create, CreateField, InlineCreate, KeptLine};
/// splat::create_reasons! { enum Reason { NoName => "Enter a name first." } }
/// fn form(ui: &mut eframe::egui::Ui, name: &mut String) {
///     let reason = name.is_empty().then_some(Reason::NoName);
///     let _ = InlineCreate::new()
///         .field(CreateField::new(splat::field_id!("doc_form_name").once(splat::layout::Once::Example), "Name", name))
///         .create(Create::primary(splat::word!("Create")).blocked(reason))
///         .kept_line(KeptLine::new())
///         .show(ui);
/// }
/// ```
///
/// Create before any field does not compile:
///
/// ```compile_fail,E0599
/// use splat::layout::{Create, InlineCreate, NoReason};
/// let _ = InlineCreate::new().create(Create::<NoReason>::primary(splat::word!("Create")));
/// ```
///
/// Create after a choice, with no field after it, does not compile:
///
/// ```compile_fail,E0599
/// use splat::layout::{Create, CreateChoice, InlineCreate, NoReason};
/// let mut used = false;
/// let _ = InlineCreate::new()
///     .choice(CreateChoice::new("Use", &mut used))
///     .create(Create::<NoReason>::primary(splat::word!("Create")));
/// ```
///
/// A field after Create does not compile:
///
/// ```compile_fail,E0599
/// use splat::layout::{Create, CreateField, InlineCreate, NoReason};
/// let (mut a, mut b) = (String::new(), String::new());
/// let _ = InlineCreate::new()
///     .field(CreateField::new(splat::field_id!("doc_a").once(splat::layout::Once::Example), "A", &mut a))
///     .create(Create::<NoReason>::primary(splat::word!("Create")))
///     .field(CreateField::new(splat::field_id!("doc_b").once(splat::layout::Once::Example), "B", &mut b));
/// ```
///
/// A form with no kept line has no `show`:
///
/// ```compile_fail,E0599
/// use splat::layout::{Create, CreateField, InlineCreate, NoReason};
/// fn form(ui: &mut eframe::egui::Ui, name: &mut String) {
///     let _ = InlineCreate::new()
///         .field(CreateField::new(splat::field_id!("doc_name").once(splat::layout::Once::Example), "Name", name))
///         .create(Create::<NoReason>::primary(splat::word!("Create")))
///         .show(ui);
/// }
/// ```
#[must_use = "call show"]
pub struct InlineCreate<'a, State, R: CreateReasons> {
    parts: Vec<CreatePart<'a>>,
    create: Option<Create<R>>,
    kept: KeptLine<'a>,
    state: PhantomData<State>,
}

/// What the DM did in an [`InlineCreate`] this frame.
#[derive(Debug, Default)]
pub struct InlineCreateResponse {
    pub create: bool,
    pub cancel: bool,
    /// The last field's side action (`Browse…`).
    pub side: bool,
    /// Each text field's response, in order.
    pub fields: Vec<Response>,
}

impl<'a, State, R: CreateReasons> InlineCreate<'a, State, R> {
    fn into<Next>(self) -> InlineCreate<'a, Next, R> {
        InlineCreate {
            parts: self.parts,
            create: self.create,
            kept: self.kept,
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
    pub fn create(mut self, create: Create<R>) -> InlineCreate<'a, Named, R> {
        self.create = Some(create);
        self.into()
    }
}

impl<'a, R: CreateReasons> InlineCreate<'a, Named, R> {
    pub fn kept_line(mut self, kept: KeptLine<'a>) -> InlineCreate<'a, Complete, R> {
        self.kept = kept;
        self.into()
    }
}

/// The kept line's fixed height in `room`: a control's, or the tallest declared reason's
/// wrapped beside Cancel, whichever is taller. A reason drawn with Cancel hidden has more
/// room, so it is never taller than this.
fn kept_line_height<R: CreateReasons>(ui: &Ui, room: f32, cancel: Option<Word>) -> f32 {
    let beside = cancel.map_or(0.0, |word| cancel_action(word).natural_width(ui) + L);
    let width = (room - beside).max(1.0);
    let font = Type::Small.font(ui);
    R::ALL
        .iter()
        .map(|reason| {
            ui.fonts_mut(|fonts| {
                fonts
                    .layout(
                        reason.words().to_owned(),
                        font.clone(),
                        colors().muted,
                        width,
                    )
                    .size()
                    .y
            })
        })
        .fold(Density::Read.control(), f32::max)
}

fn cancel_action(word: Word) -> Action<'static> {
    Action::quiet(word.as_str()).density(Density::Read)
}

impl<R: CreateReasons> InlineCreate<'_, Complete, R> {
    pub fn show(self, ui: &mut Ui) -> InlineCreateResponse {
        let mut out = InlineCreateResponse::default();
        let Some(create) = self.create else {
            return out;
        };
        let room = ui.available_width().min(FORM_WIDTH);
        let count = self.parts.len();
        let mut last_help = None;
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
                    let response =
                        labelled_field(ui, part.id, part.label, part.text, part.help, None);
                    if part.focus {
                        response.request_focus();
                    }
                    out.fields.push(response);
                }
                CreatePart::Text(part) => {
                    last_help = part.help;
                    let side = part.side;
                    let side_width = side.map_or(0.0, |word| {
                        Action::secondary(word.as_str())
                            .density(Density::Read)
                            .natural_width(ui)
                            + S
                    });
                    let create_width = create.action().natural_width(ui) + S;
                    field(ui, part.label, None, None, |ui| {
                        ui.allocate_ui_with_layout(
                            vec2(room, Density::Read.control()),
                            Layout::left_to_right(Align::Center),
                            |ui| {
                                ui.spacing_mut().item_spacing.x = S;
                                let width = (room - side_width - create_width - 2.0 * S).max(120.0);
                                let response =
                                    form_field(ui, part.id, part.label, part.text, width);
                                if part.focus {
                                    response.request_focus();
                                }
                                out.fields.push(response);
                                if let Some(word) = side {
                                    out.side = Action::secondary(word.as_str())
                                        .density(Density::Read)
                                        .show(ui)
                                        .clicked();
                                }
                                out.create = create.action().show(ui).clicked();
                            },
                        );
                    });
                }
            }
        }
        // The last field's help, under its line (K2, gap K-5): each field with its help, then
        // the kept line.
        if let Some(help) = last_help.take() {
            super::help(ui, help);
        }
        // The kept line: one fixed height for the life of the form, whatever it holds now.
        let cancel = self.kept.cancel.map(|(word, _)| word);
        let height = kept_line_height::<R>(ui, room, cancel);
        let (line, _) = ui.allocate_exact_size(vec2(room, height), Sense::hover());
        // Its content goes in a child that does not allocate in `ui` again.
        let mut kept = ui.new_child(
            UiBuilder::new()
                .max_rect(line)
                .layout(Layout::left_to_right(Align::Min)),
        );
        kept.spacing_mut().item_spacing.x = 0.0;
        if let Some((word, true)) = self.kept.cancel {
            out.cancel = cancel_action(word).line_start().show(&mut kept).clicked();
            space(&mut kept, L);
        }
        if let Some(reason) = create.blocked {
            let words = Type::Small
                .rich(&kept, reason.words())
                .color(colors().muted);
            kept.add(egui::Label::new(words).wrap());
        } else if let Some(preview) = self.kept.preview {
            preview_line(&mut kept, preview);
        }
        out
    }
}

/// A preview keeps one line: one that does not fit gives way at its start
/// (`…/modules/copper_coast`); its hover and accessible name keep it whole.
fn preview_line(ui: &mut Ui, preview: &str) {
    let font = Type::Small.font(ui);
    let room = ui.available_width();
    let mut shown = preview.to_owned();
    if number_width(ui, &shown, &font) > room {
        let chars: Vec<char> = preview.chars().collect();
        let mut from = 0;
        while from < chars.len() {
            shown = format!("…{}", chars[from..].iter().collect::<String>());
            if number_width(ui, &shown, &font) <= room {
                break;
            }
            from += 1;
        }
    }
    let response = ui
        .add(egui::Label::new(Type::Small.rich(ui, shown).color(colors().muted)).extend())
        .on_hover_text(preview);
    named(ui, &response, preview);
}

/// A menu that opens under `response`, for a row or a control with more actions. The one place
/// a menu popup is made outside `row_menu`, so a placement rule for menus has one home.
#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
pub fn menu_popup(response: &Response) -> egui::Popup<'_> {
    egui::Popup::menu(response)
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
