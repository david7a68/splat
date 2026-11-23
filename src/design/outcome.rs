//! Messages and their words, by type (round 28, package M). A caller gives data: what was done
//! (`Verb` and object, or a `Sentence`), how to undo it (`UndoStep`), which choices follow
//! (`MessageChoice`), or a `Fault`. This module decides the rest: the place, the order, the tone,
//! the form of the words, the weight of each action and the gaps. Nothing here takes a `Tone`, a
//! draw closure or an `Action`, so a caller cannot choose them.
//!
//! The typed places are the only ways to draw a message:
//!
//! - `Messages` (a pane's footer): every result, reason and notice that comes and goes. Its
//!   entries are owned, typed values built before the body runs, so the body may change the
//!   app; presses come back from `Messages::pane` as the caller's own ids.
//! - `banner`: a notice that came by itself, at the top of a pane's body.
//! - `row_fault`: a fault that belongs to one row or member, drawn where that row is.
//! - `PagePlaceholder`: a page or section whose body cannot show, in place of that body.
//!
//! The old message functions of `components` are `#[deprecated]`. Each file that still calls one
//! holds one `#![allow(deprecated)]`, and the source lint counts those allows and the calls; the
//! counts can only go down. A new caller in a new file fails `check`.
//!
//! Proofs of the rules that do not compile are the `compile_fail` examples on the types below.
//! Stable `rustdoc` does not check a `compile_fail` example's error code, so each one has a twin
//! just before it that differs only in the forbidden part and compiles: the failure can only come
//! from that part.
use super::components::{
    Action, FooterRank, blocked_reason, first_line_centre, icon_button, link, message_choice_row,
    number_width, single_action_message, small_tone_message, tone_message, undo_message_line,
};
pub use super::components::{Footer, ResultOrder, Tone};
pub use super::results::{ByTicket, Gone, Ticket};
use super::{extras::space, icons, tokens::*};
use egui::{self, Rect, Response, Ui};
use std::borrow::Cow;

// ------------------------------------------------------------------------------------------------
// Sentence

/// Words that are code, not the DM's words (round 27 family F2). A sentence template that holds
/// one does not compile.
const CODE_WORDS: [&str; 8] = [
    "identity",
    "cached",
    "in memory",
    "keyring",
    "sandbox",
    "LLM",
    "namespace",
    "UUID",
];

/// Proof that a template passed `check_template`. Only `check_template` makes one.
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Checked(());

const fn is_word_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// Does `word` stand in `text` as a whole word?
const fn holds_word(text: &[u8], word: &[u8]) -> bool {
    if word.len() > text.len() {
        return false;
    }
    let mut at = 0;
    while at + word.len() <= text.len() {
        let mut same = true;
        let mut i = 0;
        while i < word.len() {
            if text[at + i] != word[i] {
                same = false;
                break;
            }
            i += 1;
        }
        let before = at == 0 || !is_word_byte(text[at - 1]);
        let after = at + word.len() == text.len() || !is_word_byte(text[at + word.len()]);
        if same && before && after {
            return true;
        }
        at += 1;
    }
    false
}

/// The rules of a sentence template, checked at compile time by `sentence!`. It starts with a
/// capital letter, a number or a value (`{}`); it ends with `.`, `?` or `!` (a closing quote or
/// bracket may follow); it has no straight apostrophe inside a word (UI words use `’`); and it
/// has no code word (`CODE_WORDS`).
#[doc(hidden)]
pub const fn check_template(template: &str) -> Checked {
    let b = template.as_bytes();
    assert!(!b.is_empty(), "A sentence has words.");
    let first = b[0];
    assert!(
        first.is_ascii_uppercase() || first.is_ascii_digit() || first == b'{' || first >= 0x80,
        "A sentence starts with a capital letter, a number or a value."
    );
    let mut end = b.len();
    loop {
        // A closing curly quote (`”`, `’`) or a bracket may follow the end mark.
        if end >= 3 && b[end - 3] == 0xE2 && b[end - 2] == 0x80 && matches!(b[end - 1], 0x9D | 0x99)
        {
            end -= 3;
        } else if end >= 1 && b[end - 1] == b')' {
            end -= 1;
        } else {
            break;
        }
    }
    assert!(
        end > 0 && matches!(b[end - 1], b'.' | b'?' | b'!'),
        "A sentence ends with a full stop, a question mark or an exclamation mark, also after a value."
    );
    check_words(b);
    Checked(())
}

/// Curly apostrophes and no code word: the rules a sentence template and lead words share.
const fn check_words(b: &[u8]) {
    let mut i = 1;
    while i + 1 < b.len() {
        if b[i] == b'\''
            && (b[i - 1].is_ascii_alphanumeric() || b[i - 1] == b'}')
            && b[i + 1].is_ascii_alphabetic()
        {
            panic!("UI words use the curly apostrophe ’, not a straight one.");
        }
        i += 1;
    }
    let mut w = 0;
    while w < CODE_WORDS.len() {
        assert!(
            !holds_word(b, CODE_WORDS[w].as_bytes()),
            "A sentence uses the DM’s words, not code words (identity, cached, in memory, keyring, sandbox, LLM, namespace, UUID)."
        );
        w += 1;
    }
}

/// A template with no value: `sentence!` makes a constant of it.
#[doc(hidden)]
#[allow(
    clippy::disallowed_methods,
    reason = "the helper of `sentence!`: it checks a template"
)]
pub const fn check_literal(template: &'static str) -> &'static str {
    check_template(template);
    let b = template.as_bytes();
    let mut i = 0;
    while i < b.len() {
        assert!(
            b[i] != b'{',
            "A sentence with values names them after the template: sentence!(\"Removed {{}}.\", name)."
        );
        i += 1;
    }
    template
}

/// One or more whole sentences of UI words. The only ways to make one:
///
/// - `sentence!("Removed {}.", name)` checks the template at compile time (`check_template`):
///   a capital, a number or a value first; an end mark last; curly apostrophes; no code word.
/// - `Sentence::outside(text)` for words that come from outside the app (an error of the system,
///   a parser's message, a row of the DM's table). It gives them an end mark and a capital, and
///   gives `None` for blank words. The source lint counts its uses per file (`outside-words`).
/// - `a.then(b)` joins two sentences.
///
/// What the check cannot see: the values of a template. `sentence!("{}.", error)` checks only
/// `{}.`; an argument may carry outside words (a file's error, a name), lower case or code words.
/// The macro's helpers (`check_template`, `Sentence::__formatted`, `Sentence::__literal`) can
/// build an unchecked sentence; the source lint (`outcome-internal`, rung 3) fails on them
/// outside this file.
///
/// A sentence without its end mark does not compile:
///
/// ```
/// const R: splat_design::outcome::Sentence = splat_design::sentence!("Add a creature first.");
/// ```
/// ```compile_fail
/// const R: splat_design::outcome::Sentence = splat_design::sentence!("Add a creature first");
/// ```
///
/// Nor one that starts with a small letter, has a straight apostrophe or a code word, or ends
/// with a value:
///
/// ```
/// let name = "Cinder Wolf";
/// let _ = splat_design::sentence!("Removed {}.", name);
/// let _ = splat_design::sentence!("The party’s seal is kept.");
/// let _ = splat_design::sentence!("The key is kept.");
/// ```
/// ```compile_fail
/// let _ = splat_design::sentence!("removed the wolf.");
/// ```
/// ```compile_fail
/// let _ = splat_design::sentence!("The party's seal is kept.");
/// ```
/// ```compile_fail
/// let _ = splat_design::sentence!("The keyring is kept.");
/// ```
/// ```compile_fail
/// let name = "Cinder Wolf";
/// let _ = splat_design::sentence!("Removed {}", name);
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Sentence(Cow<'static, str>);

impl Sentence {
    /// `sentence!` with no value.
    #[doc(hidden)]
    #[allow(
        clippy::disallowed_methods,
        reason = "the helper of `sentence!`: it checks its template"
    )]
    pub const fn __literal(template: &'static str) -> Self {
        Self(Cow::Borrowed(check_literal(template)))
    }

    /// `sentence!` with values: `checked` proves the template passed `check_template`.
    #[doc(hidden)]
    pub fn __formatted(checked: Checked, words: String) -> Self {
        let _ = checked;
        Self(Cow::Owned(words))
    }

    /// Words that come from outside the app: an error of the system, a parser's message, a row
    /// of the DM's own table. They end with an end mark, whatever came, and start with a capital,
    /// except that the first letter is left alone when the first white-space-separated word holds
    /// `/` or `\` or ends in a dot-extension (`.` then one or more letters or digits, after a
    /// letter or digit; Unicode letters count). Blank words are no sentence: `None`, and the
    /// caller says what it knows in its own checked words. Words that name a file or a path
    /// anywhere else use [`Sentence::outside_after`], which never changes them. Clippy counts the
    /// uses of this escape (`Ban::Words`).
    pub fn outside(words: impl Into<String>) -> Option<Self> {
        let words: String = words.into();
        let mut words = words.trim().to_owned();
        let first = words.chars().next()?;
        let first_word = words.split_whitespace().next().unwrap_or_default();
        if first.is_lowercase() && !is_data(first_word) {
            words = first.to_uppercase().chain(words.chars().skip(1)).collect();
        }
        if !ends_with_mark(&words) {
            words.push('.');
        }
        Some(Self(Cow::Owned(words)))
    }

    /// Checked words, then words from outside the app exactly as they came, then an end mark if
    /// they have none: `Sentence::outside_after(lead!("Changed on disk:"), "lost notes/session.md")`
    /// is `Changed on disk: lost notes/session.md.`. This is the form of a message that names a
    /// file or a path: the checked words come first, so nothing in the outside words is re-cased.
    /// Blank outside words are no sentence: `None`. Clippy counts its uses (`Ban::Words`), as
    /// `outside`'s.
    pub fn outside_after(lead: Lead, words: impl Into<String>) -> Option<Self> {
        let words: String = words.into();
        let words = words.trim();
        if words.is_empty() {
            return None;
        }
        let mut sentence = format!("{} {words}", lead.0);
        if !ends_with_mark(&sentence) {
            sentence.push('.');
        }
        Some(Self(Cow::Owned(sentence)))
    }

    /// This sentence, then `next`, with one space.
    pub fn then(self, next: Sentence) -> Self {
        Self(Cow::Owned(format!("{} {}", self.0, next.0)))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl PartialEq<str> for Sentence {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<&str> for Sentence {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<String> for Sentence {
    fn eq(&self, other: &String) -> bool {
        self.0 == other.as_str()
    }
}

impl std::fmt::Display for Sentence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Is this first word data that keeps its first letter (M4 gap 3)? It holds `/` or `\`, or,
/// without a trailing `,`, `:` or `;`, it ends in a dot-extension: a `.` after a letter or digit,
/// then one or more letters or digits to its end (`package.md`, `café.md`, `v2.json:`). Not
/// `e.g.` or `removed.`, whose last `.` ends them.
fn is_data(word: &str) -> bool {
    if word.contains(['/', '\\']) {
        return true;
    }
    let word = word.trim_end_matches([',', ':', ';']);
    word.rsplit_once('.').is_some_and(|(stem, extension)| {
        !extension.is_empty()
            && extension.chars().all(char::is_alphanumeric)
            && stem.chars().last().is_some_and(char::is_alphanumeric)
    })
}

/// The checked words that lead outside words ([`Sentence::outside_after`]): `lead!("Changed on
/// disk:")`. Checked at compile time: a capital or a number first, a `:` last, curly
/// apostrophes, no code word.
///
/// ```
/// let _ = splat_design::lead!("Changed on disk:");
/// ```
/// ```compile_fail
/// let _ = splat_design::lead!("changed on disk:");
/// ```
/// ```compile_fail
/// let _ = splat_design::lead!("Changed on disk.");
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Lead(&'static str);

impl Lead {
    /// `lead!`'s check, in a `const`.
    #[doc(hidden)]
    pub const fn __checked(words: &'static str) -> Self {
        let b = words.as_bytes();
        assert!(
            !b.is_empty() && (b[0].is_ascii_uppercase() || b[0].is_ascii_digit() || b[0] >= 0x80),
            "Lead words start with a capital letter or a number."
        );
        assert!(b[b.len() - 1] == b':', "Lead words end with a colon.");
        check_words(b);
        Self(words)
    }
}

/// Checked lead words for [`Sentence::outside_after`]. See [`Lead`].
#[macro_export]
macro_rules! lead {
    ($words:literal) => {{
        #[allow(
            clippy::disallowed_methods,
            reason = "the macro's own check of its words"
        )]
        const LEAD: $crate::outcome::Lead = $crate::outcome::Lead::__checked($words);
        LEAD
    }};
}

fn ends_with_mark(words: &str) -> bool {
    words
        .trim_end_matches(['”', '’', ')'])
        .ends_with(['.', '?', '!'])
}

/// A sentence whose template is checked at compile time. See `Sentence`.
#[macro_export]
macro_rules! sentence {
    ($template:literal $(,)?) => {{
        #[allow(clippy::disallowed_methods, reason = "the macro's own check of its template")]
        const SENTENCE: $crate::outcome::Sentence = $crate::outcome::Sentence::__literal($template);
        SENTENCE
    }};
    ($template:literal, $($values:tt)+) => {{
        #[allow(clippy::disallowed_methods, reason = "the macro's own check of its template")]
        const CHECKED: $crate::outcome::Checked = $crate::outcome::check_template($template);
        #[allow(clippy::disallowed_methods, reason = "the template passed `check_template`")]
        let sentence =
            $crate::outcome::Sentence::__formatted(CHECKED, ::std::format!($template, $($values)+));
        sentence
    }};
}

// ------------------------------------------------------------------------------------------------
// Verbs and Undo

/// What an action did. A result says its past form (`Removed Take seal.`); its Undo says its
/// present form (`Undo remove Take seal`), from the same value, so the two cannot differ. The
/// table also decides, per verb, the word that joins the verb to its object in an Undo
/// (`Undo discard of Bridge Ambush`; none for `Undo remove Take seal`): a caller never writes it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verb {
    Add,
    /// `Undo carry over`, `Carried over …`.
    CarryOver,
    Change,
    Clear,
    Copy,
    Delete,
    Discard,
    Dismiss,
    Link,
    Mark,
    Move,
    Recover,
    Register,
    Reload,
    Remove,
    Rename,
    Replace,
    Restore,
    Roll,
    Separate,
    /// A question ticked in Open questions: `Undo tick`, `… ticked in Horn Rings.`
    Tick,
    Unregister,
    Use,
    Write,
}

impl Verb {
    /// Every verb, for the part's tests of each verb's two forms.
    #[cfg(test)]
    const ALL: [Verb; 24] = [
        Self::Add,
        Self::CarryOver,
        Self::Change,
        Self::Clear,
        Self::Copy,
        Self::Delete,
        Self::Discard,
        Self::Dismiss,
        Self::Link,
        Self::Mark,
        Self::Move,
        Self::Recover,
        Self::Register,
        Self::Reload,
        Self::Remove,
        Self::Rename,
        Self::Replace,
        Self::Restore,
        Self::Roll,
        Self::Separate,
        Self::Tick,
        Self::Unregister,
        Self::Use,
        Self::Write,
    ];

    /// The word after `Undo`: `remove`.
    pub const fn word(self) -> &'static str {
        match self {
            Self::Add => "add",
            Self::CarryOver => "carry over",
            Self::Change => "change",
            Self::Clear => "clear",
            Self::Copy => "copy",
            Self::Delete => "delete",
            Self::Discard => "discard",
            Self::Dismiss => "dismiss",
            Self::Link => "link",
            Self::Mark => "mark",
            Self::Move => "move",
            Self::Recover => "recover",
            Self::Register => "register",
            Self::Reload => "reload",
            Self::Remove => "remove",
            Self::Rename => "rename",
            Self::Replace => "replace",
            Self::Restore => "restore",
            Self::Roll => "roll",
            Self::Separate => "separate",
            Self::Tick => "tick",
            Self::Unregister => "unregister",
            Self::Use => "use",
            Self::Write => "write",
        }
    }

    /// The first word of a result: `Removed`.
    pub const fn past(self) -> &'static str {
        match self {
            Self::Add => "Added",
            Self::CarryOver => "Carried over",
            Self::Change => "Changed",
            Self::Clear => "Cleared",
            Self::Copy => "Copied",
            Self::Delete => "Deleted",
            Self::Discard => "Discarded",
            Self::Dismiss => "Dismissed",
            Self::Link => "Linked",
            Self::Mark => "Marked",
            Self::Move => "Moved",
            Self::Recover => "Recovered",
            Self::Register => "Registered",
            Self::Reload => "Reloaded",
            Self::Remove => "Removed",
            Self::Rename => "Renamed",
            Self::Replace => "Replaced",
            Self::Restore => "Restored",
            Self::Roll => "Rolled",
            Self::Separate => "Separated",
            Self::Tick => "Ticked",
            Self::Unregister => "Unregistered",
            Self::Use => "Used",
            Self::Write => "Wrote",
        }
    }

    /// The past form inside a sentence, after its object: `ticked` in `“…” ticked in Horn Rings.`
    pub fn past_after(self) -> String {
        self.past().to_lowercase()
    }

    /// The word between the verb and its object in an Undo, decided here for every verb (no
    /// wildcard: a new verb must decide). `of` for discard (what was discarded is a draft *of*
    /// the object); `to` for change (the Undo takes back the change *to* the field or note:
    /// `Undo change to Part of`, ruling R16); none for the others.
    pub const fn join(self) -> Option<&'static str> {
        match self {
            Self::Discard => Some("of"),
            Self::Change => Some("to"),
            Self::Add
            | Self::CarryOver
            | Self::Clear
            | Self::Copy
            | Self::Delete
            | Self::Dismiss
            | Self::Link
            | Self::Mark
            | Self::Move
            | Self::Recover
            | Self::Register
            | Self::Reload
            | Self::Remove
            | Self::Rename
            | Self::Replace
            | Self::Restore
            | Self::Roll
            | Self::Separate
            | Self::Tick
            | Self::Unregister
            | Self::Use
            | Self::Write => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Step {
    /// `Undo remove`, `Undo remove Take seal`, `Undo discard of Bridge Ambush`.
    Verb(Verb, Option<String>),
    /// An act the DM named: `Undo: Take seal`.
    Act(String),
    /// The app's own fixed words for a step that is not a verb and an object:
    /// `Undo package registration`. Never cut.
    Words(&'static str),
}

/// What an Undo takes back, typed: a verb, an optional object, and the joining word the verb's
/// table entry gives (`Verb::join`). Its fields are private: a label is made only from a `Verb`
/// (and an object), or from the name of an act, and the part that tells two steps apart (a copy
/// number, a folder) is kept whole by every fit.
///
/// One step has two forms of words, both made here: the accessible name is always the whole
/// form (`label`: `Undo discard of Bridge Ambush`); the visible words are `fit`'s, for the room
/// the footer line has: the whole form when it fits, else, for a verb with a joining word, the
/// short form (`Undo discard`), and for the others the object cut by whole words
/// (`Undo remove Ravenholm…`); a lone Undo of its verb in its footer uses `alone`, which also
/// measures the whole form first and gives up the object only when it does not fit. The message
/// part gives the action the whole form as its name whenever the visible words differ.
///
/// ```
/// let _ = splat_design::outcome::UndoStep::act("Undo it");
/// ```
/// ```compile_fail
/// let _ = splat_design::outcome::UndoStep { label: "Undo it".to_owned() };
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UndoStep {
    step: Step,
    tell_apart: Option<String>,
}

impl UndoStep {
    /// `Undo <verb>`: `Undo add`.
    pub fn verb(verb: Verb) -> Self {
        Self {
            step: Step::Verb(verb, None),
            tell_apart: None,
        }
    }

    /// `Undo <verb> [<join>] <object>`: `Undo remove Take seal`, `Undo discard of Bridge Ambush`.
    pub fn of(verb: Verb, object: impl Into<String>) -> Self {
        Self {
            step: Step::Verb(verb, Some(object.into())),
            tell_apart: None,
        }
    }

    /// An act the DM named, such as a tracker's action: `Undo: Take seal`.
    pub fn act(label: impl Into<String>) -> Self {
        Self {
            step: Step::Act(label.into()),
            tell_apart: None,
        }
    }

    /// The app's own fixed words for a step that is not a verb and an object
    /// (`Undo package registration`), made once as a `const` step that each place showing it
    /// uses: its visible form, its short form and its accessible name are these words, whole at
    /// every width. A literal that does not start with `Undo ` and a word does not compile in a
    /// `const`.
    ///
    /// ```
    /// const STEP: splat_design::outcome::UndoStep = splat_design::outcome::UndoStep::words("Undo source change");
    /// ```
    /// ```compile_fail
    /// const STEP: splat_design::outcome::UndoStep = splat_design::outcome::UndoStep::words("Source change");
    /// ```
    pub const fn words(label: &'static str) -> Self {
        let b = label.as_bytes();
        assert!(
            b.len() > 5
                && b[0] == b'U'
                && b[1] == b'n'
                && b[2] == b'd'
                && b[3] == b'o'
                && b[4] == b' '
                && b[5].is_ascii_lowercase(),
            "an Undo's words start with `Undo ` and a word"
        );
        Self {
            step: Step::Words(label),
            tell_apart: None,
        }
    }

    /// The part that tells this step from another of the same words: `2`, `in Ruins`. A fit
    /// never drops it.
    pub fn tell_apart(mut self, part: impl Into<String>) -> Self {
        self.tell_apart = Some(part.into());
        self
    }

    /// The whole label, also the action's accessible name.
    pub fn label(&self) -> String {
        let (head, object) = self.parts();
        let mut label = head;
        if let Some(object) = object {
            label.push(' ');
            label.push_str(&object);
        }
        label.push_str(&self.tail());
        label
    }

    /// The visible words for `width` in the action's font while no other kept result of its
    /// verb stands in its footer (ruling M9): the whole label when it fits, measured first, as
    /// `fit` does; only when it does not, `Undo tick`, with no object and no part that tells
    /// steps apart, as nothing needs telling apart and its result names the object. A step with
    /// no object keeps its whole label. Its accessible name stays `label`. An act's step
    /// (`Undo: Take seal`) and the app's fixed words are never alone (`counted`).
    pub fn alone(&self, ui: &Ui, width: f32) -> String {
        let whole = self.label();
        match &self.step {
            Step::Verb(verb, Some(_))
                if number_width(ui, &whole, &Type::Strong.font(ui)) > width =>
            {
                format!("Undo {}", verb.word())
            }
            _ => whole,
        }
    }

    /// The verb a footer counts for `alone`: every verb's step, with or without an object, so
    /// `Undo add scene` beside `Undo add` keeps its object.
    fn counted(&self) -> Option<Verb> {
        match &self.step {
            Step::Verb(verb, _) => Some(*verb),
            // The app's fixed words are whole in every footer: no short form to count.
            Step::Act(_) | Step::Words(_) => None,
        }
    }

    /// The short form: `Undo discard`, `Undo remove`, `Undo:`, with the part that tells two
    /// steps apart.
    pub fn short(&self) -> String {
        format!("{}{}", self.parts().0, self.tail())
    }

    /// ` 2`, ` in Ruins`, or nothing.
    fn tail(&self) -> String {
        self.tell_apart
            .as_ref()
            .map_or(String::new(), |part| format!(" {part}"))
    }

    /// `Undo remove` or `Undo:`, and the object with the verb's joining word before it.
    fn parts(&self) -> (String, Option<String>) {
        match &self.step {
            Step::Verb(verb, object) => (
                format!("Undo {}", verb.word()),
                object.as_ref().map(|object| match verb.join() {
                    Some(join) => format!("{join} {object}"),
                    None => object.clone(),
                }),
            ),
            Step::Act(label) => ("Undo:".to_owned(), Some(label.clone())),
            Step::Words(label) => ((*label).to_owned(), None),
        }
    }

    /// The label's visible words for `width` in the action's font: the whole label when it
    /// fits; else, for a verb with a joining word (`Verb::join`), the short form
    /// (`Undo discard`); else the object shortened by whole words with `…`. `Undo`, the verb
    /// and the part that tells steps apart always stay, also when they alone are wider. The
    /// accessible name stays `label` (the message part names the action with it).
    pub fn fit(&self, ui: &Ui, width: f32) -> String {
        let font = Type::Strong.font(ui);
        let whole = self.label();
        if matches!(self.step, Step::Words(_)) || number_width(ui, &whole, &font) <= width {
            return whole;
        }
        if let Step::Verb(verb, Some(_)) = &self.step
            && verb.join().is_some()
        {
            return self.short();
        }
        let (head, object) = self.parts();
        let tail = self.tail();
        let words: Vec<&str> = object
            .as_deref()
            .map_or(Vec::new(), |o| o.split(' ').collect());
        // The most whole words that fit, by bisection: at most `log2` of the word count
        // measurements per draw (the width grows with the words kept).
        let cut = |keep: usize| format!("{head} {}…{tail}", words[..keep].join(" "));
        let fits = |keep: usize| number_width(ui, &cut(keep), &font) <= width;
        if words.len() < 2 || !fits(1) {
            return format!("{head} …{tail}");
        }
        let (mut low, mut high) = (1, words.len() - 1);
        while low < high {
            let mid = (low + high).div_ceil(2);
            if fits(mid) {
                low = mid;
            } else {
                high = mid - 1;
            }
        }
        cut(low)
    }
}

// ------------------------------------------------------------------------------------------------
// Choice words

/// Proof that a choice template passed `check_choice`. Only `check_choice` makes one.
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct CheckedChoice(());

/// The rules of a choice template, checked at compile time by `choice!`: it starts with the
/// app's fixed words (a capital letter first), it holds exactly `values` places `{}` for the
/// objects, no other brace, and at least one fixed word of three letters or more (so an object
/// cannot be the whole label behind a letter: `choice!("A{}", label)` does not compile).
#[doc(hidden)]
pub const fn check_choice(template: &str, values: usize) -> CheckedChoice {
    let b = template.as_bytes();
    assert!(
        !b.is_empty() && b[0].is_ascii_uppercase(),
        "A choice starts with the app's own words, a capital first: choice!(\"Repair {{}}\", name)."
    );
    let (mut i, mut places, mut run, mut word) = (0, 0, 0, false);
    while i < b.len() {
        if b[i] == b'{' {
            assert!(
                i + 1 < b.len() && b[i + 1] == b'}',
                "A choice template has only `{{}}` places."
            );
            places += 1;
            run = 0;
            i += 2;
            continue;
        }
        assert!(b[i] != b'}', "A choice template has only `{{}}` places.");
        if b[i].is_ascii_alphabetic() {
            run += 1;
            if run >= 3 {
                word = true;
            }
        } else {
            run = 0;
        }
        i += 1;
    }
    assert!(
        word,
        "A choice has a fixed word of three letters or more: choice!(\"Repair {{}}\", name)."
    );
    assert!(
        places == values,
        "A choice template has one `{{}}` place for each object."
    );
    CheckedChoice(())
}

/// The words of a choice: the app's fixed words with zero or more objects from the DM's data
/// between them (`Repair {}`, `Move {} before {}`). Made only by `choice!` (a template literal
/// checked at compile time, the objects as values) or from fixed words (`"Open in Library".into()`);
/// no `String` is ever the whole label. The part draws them: when the line is short it cuts the
/// objects (each keeps at least a few letters and `…`), never the fixed words, and it names the
/// action with the whole words.
///
/// The objects are the DM's data (names, paths of their files): the same count as a value of a
/// `sentence!` template, none. Words from the system are not an object (they go in a message's
/// sentence through `Sentence::outside`, `Ban::Words`).
///
/// ```
/// use splat_design::outcome::{Choice, ChoiceWords};
/// let (a, b) = (String::from("Ghost of the Mill"), String::from("Ember Cultist"));
/// let words = splat_design::choice!("Move {} before {}", a, b);
/// assert_eq!(words.label(), "Move Ghost of the Mill before Ember Cultist");
/// let _ = Choice::named(words, 1);
/// let _: ChoiceWords = "Open in Library".into();
/// ```
/// ```compile_fail
/// use splat_design::outcome::Choice;
/// let (a, b) = (String::from("Ghost of the Mill"), String::from("Ember Cultist"));
/// let _ = Choice::named(format!("Move {a} before {b}"), 1);
/// ```
/// ```compile_fail
/// let a = String::from("Ghost of the Mill");
/// let _ = splat_design::choice!("Move {} before {}", a);
/// ```
///
/// A fixed word is a word: an object behind one letter is not choice words (twin: `Add {}`).
///
/// ```
/// let label = String::from("Repair Bridge Ambush");
/// let _ = splat_design::choice!("Add {}", label);
/// ```
/// ```compile_fail
/// let label = String::from("Repair Bridge Ambush");
/// let _ = splat_design::choice!("A{}", label);
/// ```
#[derive(Clone, Debug)]
pub struct ChoiceWords {
    /// The fixed words: one more than the objects.
    fixed: Vec<&'static str>,
    objects: Vec<String>,
    /// Shorter forms of the same choice, longest first (`or_shorter`), shared, with the
    /// revision `or_shorter` gave them: a choice kept by its owner clones them and compares
    /// that revision, not form by form (a clone is the same version).
    shorter: std::sync::Arc<[ChoiceWords]>,
    shorter_revision: Option<super::memo::Revision>,
    /// Its objects must show whole (a file name): words that do not fit wrap, never cut
    /// (`whole_objects`).
    whole: bool,
}

impl PartialEq for ChoiceWords {
    fn eq(&self, other: &Self) -> bool {
        self.fixed == other.fixed
            && self.objects == other.objects
            && self.shorter_revision == other.shorter_revision
            && self.whole == other.whole
    }
}

impl Eq for ChoiceWords {}

/// How many letters a cut object keeps at least.
const OBJECT_KEEPS: usize = 3;

impl From<&'static str> for ChoiceWords {
    fn from(fixed: &'static str) -> Self {
        Self {
            fixed: vec![fixed],
            objects: Vec::new(),
            shorter: std::sync::Arc::from([]),
            shorter_revision: None,
            whole: false,
        }
    }
}

impl ChoiceWords {
    /// `choice!`'s template and values: `checked` proves the template passed `check_choice`.
    #[doc(hidden)]
    pub fn __template(
        checked: CheckedChoice,
        template: &'static str,
        objects: Vec<String>,
    ) -> Self {
        let _ = checked;
        Self {
            fixed: template.split("{}").collect(),
            objects,
            shorter: std::sync::Arc::from([]),
            shorter_revision: None,
            whole: false,
        }
    }

    /// Objects that must show whole, such as a file the repair acts on (M0583): when the words
    /// do not fit their line, they wrap in the action, which grows taller (`Action::wraps`),
    /// instead of being cut. Words that fit stay on one line.
    pub fn whole_objects(mut self) -> Self {
        self.whole = true;
        self
    }

    /// These words, with shorter forms of the same choice for a line too short for them,
    /// longest first, each made by `choice!` too: a form that gives up a whole part, never a
    /// piece of one (Arrivals' Move: `Move Shrine Reserve (Goblin Warrior 3) before Brannoch
    /// Vale`, then `Move Shrine Reserve before Brannoch Vale`, then `Move Shrine… before
    /// Brannoch Vale`). The first form that fits shows whole; when none fits, the last one's
    /// objects are cut as always. The action's name and tip keep these whole words.
    pub fn or_shorter(mut self, forms: impl IntoIterator<Item = ChoiceWords>) -> Self {
        self.shorter = self.shorter.iter().cloned().chain(forms).collect();
        self.shorter_revision = Some(super::memo::Revision::next());
        self
    }

    /// The whole words: the action's accessible name.
    pub fn label(&self) -> String {
        self.shown(usize::MAX)
    }

    /// The words with each object cut to `keep` letters and `…`.
    fn shown(&self, keep: usize) -> String {
        let mut words = String::new();
        for (at, fixed) in self.fixed.iter().enumerate() {
            words.push_str(fixed);
            if let Some(object) = self.objects.get(at) {
                if object.chars().count() > keep {
                    words.extend(object.chars().take(keep));
                    let trimmed = words.trim_end().len();
                    words.truncate(trimmed);
                    words.push('…');
                } else {
                    words.push_str(object);
                }
            }
        }
        words
    }

    /// The visible words: the whole words when `fits` them, else the first shorter form that
    /// does (`or_shorter`), else the longest cut of the last form that does (every object cut to
    /// the same number of letters, at least `OBJECT_KEEPS`). When even the shortest cut does not
    /// fit, the whole words wrap: a fixed word is never cut. The search is a bisection over the
    /// letters kept: at most `log2` of the longest object's length measurements per draw, after
    /// one for each shorter form.
    fn fit(&self, fits: impl Fn(&str) -> bool) -> Fitted {
        let whole = self.label();
        if fits(&whole) {
            return Fitted::Whole(whole);
        }
        if self.whole {
            return Fitted::Wraps(whole);
        }
        for form in self.shorter.iter() {
            let words = form.label();
            if fits(&words) {
                return Fitted::Cut(words, whole);
            }
        }
        let last = self.shorter.last().unwrap_or(self);
        let longest = last
            .objects
            .iter()
            .map(|object| object.chars().count())
            .max()
            .unwrap_or(0);
        if longest <= OBJECT_KEEPS || !fits(&last.shown(OBJECT_KEEPS)) {
            return Fitted::Wraps(whole);
        }
        // The widest cut that fits: the width grows with the letters kept.
        let (mut low, mut high) = (OBJECT_KEEPS, longest - 1);
        while low < high {
            let mid = (low + high).div_ceil(2);
            if fits(&last.shown(mid)) {
                low = mid;
            } else {
                high = mid - 1;
            }
        }
        Fitted::Cut(last.shown(low), whole)
    }

    /// The visible words where `fits` says which words fit, for a caller's test of its forms.
    #[cfg(any(test, feature = "test-support"))]
    pub fn shown_where(&self, fits: impl Fn(&str) -> bool) -> String {
        match self.fit(fits) {
            Fitted::Whole(words) | Fitted::Wraps(words) | Fitted::Cut(words, _) => words,
        }
    }

    /// `fit` for a choice of `kind` in `room`.
    fn fit_choice(&self, ui: &Ui, kind: ChoiceKind, room: f32) -> Fitted {
        self.fit(|words| kind.action(words).natural_width(ui) <= room)
    }
}

/// What a choice's words look like in their room.
enum Fitted {
    /// All the words, on one line.
    Whole(String),
    /// The objects cut (the visible words), and the whole words for the name and the tip.
    Cut(String, String),
    /// All the words, wrapped onto more lines: even the shortest cut did not fit.
    Wraps(String),
}

/// The kind of a choice whose words hold data, as `MessageChoice` is for fixed words: a safe
/// choice is bordered; a `Lose` is red with exactly `L` before it, after the safe ones. (A
/// dismissal has no words: it is the close icon, `MessageChoice::Dismiss`.)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChoiceKind {
    Safe,
    Lose,
}

impl ChoiceKind {
    fn action(self, words: &str) -> Action<'_> {
        match self {
            Self::Safe => Action::secondary(words),
            Self::Lose => Action::destructive(words),
        }
        .density(Density::Read)
    }
}

/// Draw a data choice of `kind` in `room`: whole, cut with the whole words as its name and its
/// tip (on hover and on keyboard focus), or wrapped. `lock` is its reason while it holds.
fn data_choice(
    ui: &mut Ui,
    words: &ChoiceWords,
    kind: ChoiceKind,
    room: f32,
    lock: Option<&str>,
) -> Response {
    match words.fit_choice(ui, kind, room) {
        Fitted::Whole(words) => kind.action(&words).blocked(lock).show(ui),
        Fitted::Wraps(words) => kind.action(&words).wraps().blocked(lock).show(ui),
        Fitted::Cut(shown, whole) => {
            let response = kind.action(&shown).name(&whole).blocked(lock).show(ui);
            cut_tip(response, &whole, lock)
        }
    }
}

/// The whole words of a control whose visible words are cut, on hover and on keyboard focus
/// (`rich_tip`), with its lock's reason under them while it holds.
fn cut_tip(response: Response, whole: &str, lock: Option<&str>) -> Response {
    match lock {
        Some(reason) => super::rich_tip(
            response,
            &[super::TipLine::body(whole), super::TipLine::detail(reason)],
        ),
        None => super::rich_tip(response, &[super::TipLine::body(whole)]),
    }
}

/// Choice words with objects from data: the template's fixed words are checked at compile time
/// (`check_choice`), the objects are values. See `ChoiceWords`.
#[macro_export]
macro_rules! choice {
    ($template:literal, $($value:expr),+ $(,)?) => {{
        #[allow(clippy::disallowed_methods, reason = "the macro's own check of its template")]
        const CHECKED: $crate::outcome::CheckedChoice = $crate::outcome::check_choice($template, [$(::std::stringify!($value)),+].len());
        #[allow(clippy::disallowed_methods, reason = "the template passed `check_choice`")]
        let words = $crate::outcome::ChoiceWords::__template(CHECKED, $template, ::std::vec![$(::std::string::ToString::to_string(&$value)),+]);
        words
    }};
}

// ------------------------------------------------------------------------------------------------
// Choices and presses

/// An action that follows a message's words, by what it does. The weight comes from the kind:
/// a `Choice` is bordered, a `Primary` is filled, a `Lose` is red with exactly `L` before it, a
/// `Link` is a link. An Undo is not a choice: it is the message's `UndoStep`, always quiet with
/// its icon, and first.
///
/// `Primary` is the one act the DM most likely takes next where nothing else on the screen is
/// primary (a recording alert's `Record again`, an empty panel's `New session`). A message has at
/// most one; it stands among the safe choices in the caller's order. Which message of a place
/// holds the screen's one primary is the caller's choice; the others say the same act with
/// `Choice`.
///
/// ```
/// use splat_design::outcome::{Choice, MessageChoice, Notice};
/// let _ = Notice::new(splat_design::sentence!("No session yet."))
///     .choice(Choice::new(MessageChoice::Primary("New session"), 1));
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
///
/// `Dismiss` is the close icon. The part puts it at the right end of the message's first line,
/// centred on that line, whatever its place in the list (N6); a caller gives no icon, name or
/// place. It says what goes with the message ([`Lost`]), so the choice is made on purpose.
///
/// ```
/// use splat_design::outcome::{Choice, Lost, MessageChoice, Notice};
/// let _ = Notice::new(splat_design::sentence!("Sync is off."))
///     .choice(Choice::new(MessageChoice::Dismiss(Lost::Nothing), 1));
/// ```
///
/// What goes is said, never left out:
///
/// ```compile_fail,E0308
/// use splat_design::outcome::{Choice, MessageChoice, Notice};
/// let _ = Notice::new(splat_design::sentence!("Sync is off."))
///     .choice(Choice::new(MessageChoice::Dismiss, 1));
/// ```
pub enum MessageChoice<'a> {
    Choice(&'a str),
    Primary(&'a str),
    Lose(&'a str),
    Link {
        word: &'a str,
        hover: Option<&'a str>,
    },
    Dismiss(Lost),
}

/// The room before a red choice of `width` (`docs/design-system.md`, the destructive choice
/// rule): exactly `L` after the choice before it when it fits on that line; else a line of its own,
/// `M` below the others. `gap` is the item spacing that already follows the choice before it.
fn before_lose(ui: &mut Ui, width: f32, gap: f32) {
    let after = (L - gap).max(0.0);
    if ui.available_size_before_wrap().x >= after + width {
        space(ui, after);
    } else {
        let row_gap = ui.spacing().item_spacing.y;
        ui.spacing_mut().item_spacing.y = M;
        ui.end_row();
        ui.spacing_mut().item_spacing.y = row_gap;
    }
}

/// The close icon's accessible name and hover words.
pub const DISMISS_NAME: &str = "Dismiss this message";

/// What a `Dismiss` lets go with the message (ruling R3).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lost {
    /// Only the app's words: the message can come again from its cause.
    Nothing,
    /// Words the DM typed (a Log line whose entry is gone). The press records
    /// `Done::new(Verb::Dismiss, …).undo(id)` in the pane's `Results` store, so its Undo brings the
    /// words back; nothing is lost without an Undo.
    TypedWords,
}

/// What the DM pressed in a message, inside the part: its Undo, or its choice at this index. A
/// caller gets its own typed id instead ([`Choice`], [`Undo`], [`Paned`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::design) enum Pressed {
    Undo,
    Choice(usize),
}

/// Keep the first activation of a drawing pass: a later one in the same pass is dropped.
fn first<T>(slot: &mut Option<T>, pressed: T) {
    slot.get_or_insert(pressed);
}

/// The one decision for every control of a message (its Undo, each choice, the close icon), in
/// drawing order: an activation goes to `first`. Test builds can also activate a control of the
/// message `words` for one pass (egui gives each activation its own pass), so a test can put
/// several in one.
fn activated(ui: &Ui, slot: &mut Option<Pressed>, clicked: bool, words: &str, what: Pressed) {
    #[cfg(any(test, feature = "test-support"))]
    let clicked = clicked
        || ui.ctx().data(|data| {
            data.get_temp::<Vec<(String, Pressed)>>(egui::Id::new(TEST_ACTIVATIONS))
                .is_some_and(|list| list.iter().any(|(w, p)| w == words && *p == what))
        });
    #[cfg(not(any(test, feature = "test-support")))]
    let _ = (ui, words);
    if clicked {
        first(slot, what);
    }
}

/// Test builds: the controls `activated` treats as pressed in this pass.
#[cfg(any(test, feature = "test-support"))]
const TEST_ACTIVATIONS: &str = "design_test_activations";

/// A pane with no choice and no Undo in its footer: its presses can never happen.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NoPress {}

/// One choice of a message: what it does ([`MessageChoice`], whose kind gives its weight), the
/// id the caller gets back when the DM presses it (one of the pane's own actions), and why it is
/// locked, if it is. A locked choice is drawn as every blocked action is: disabled, with its
/// reason on hover and as its accessible description. Its words are `'static`: a choice holds
/// no borrow of the app.
///
/// A lock has two forms, and the type picks by what the caller knows: `blocked(reason)` takes a
/// reason that is always there (the choice is locked for as long as it is drawn), and a lock
/// that comes and goes at run time is written only as `locked_when(condition, reason)`, which
/// keeps the reason's line while it is open so nothing below moves. An optional reason does
/// not fit `blocked`.
///
/// Two duties stay with the caller, as the types cannot see them: **one stable reason per
/// lock**, and **never build the choice two ways** (`if cond { c.blocked(r) } else { c }`
/// compiles and moves the line when `cond` changes). `locked_when` keeps the room of the
/// reason it is given in each frame: a reason that changes to one of another height moves the
/// line once.
///
/// ```
/// use splat_design::outcome::{Choice, MessageChoice};
/// let waiting = true;
/// let _ = Choice::new(MessageChoice::Choice("Replace"), 1)
///     .locked_when(waiting, splat_design::sentence!("Wait for the read."));
/// let _ = Choice::new(MessageChoice::Choice("Replace"), 1)
///     .blocked(splat_design::sentence!("Wait for the read."));
/// ```
/// ```compile_fail
/// use splat_design::outcome::{Choice, MessageChoice};
/// let waiting = true;
/// let _ = Choice::new(MessageChoice::Choice("Replace"), 1)
///     .blocked(waiting.then_some(splat_design::sentence!("Wait for the read.")));
/// ```
#[must_use = "give it to a message's `choice`"]
pub struct Choice<A> {
    what: MessageChoice<'static>,
    /// A choice whose words hold objects from data (`Choice::named`, `Choice::named_as`).
    named: Option<(ChoiceKind, ChoiceWords)>,
    id: A,
    lock: Lock,
    hover: Option<String>,
    /// What pressing it does, painted in a help line under it (ruling M8).
    help: Option<Sentence>,
}

/// A control's lock: its reason, and whether it holds now. A control made with `locked_when`
/// has its reason even while open, so the part keeps the reason's line for it and a lock that
/// comes or goes moves nothing below; one made with `blocked(None)` has no lock and no line.
type Lock = Option<(Sentence, bool)>;

/// The reason to show now: a lock that holds.
fn holding(lock: &Lock) -> Option<&str> {
    lock.as_ref()
        .filter(|(_, locked)| *locked)
        .map(|(reason, _)| reason.as_str())
}

impl<A> Choice<A> {
    pub fn new(what: MessageChoice<'static>, id: A) -> Self {
        Self {
            what,
            named: None,
            id,
            lock: None,
            hover: None,
            help: None,
        }
    }

    /// A bordered choice whose words hold objects from the DM's data
    /// (`choice!("Move {} before {}", a, b)`). The part cuts the objects when the line is short,
    /// never the fixed words, and names the action with the whole words.
    pub fn named(words: ChoiceWords, id: A) -> Self {
        Self::named_as(ChoiceKind::Safe, words, id)
    }

    /// `named`, of `kind`: `ChoiceKind::Lose` is red, with exactly `L` before it, after the
    /// safe choices (`choice!("Delete {}", name)`).
    pub fn named_as(kind: ChoiceKind, words: ChoiceWords, id: A) -> Self {
        Self {
            what: match kind {
                ChoiceKind::Safe => MessageChoice::Choice(""),
                ChoiceKind::Lose => MessageChoice::Lose(""),
            },
            named: Some((kind, words)),
            id,
            lock: None,
            hover: None,
            help: None,
        }
    }

    /// Words on hover made at run time, owned (a link's path).
    pub fn hover(mut self, words: String) -> Self {
        self.hover = Some(words);
        self
    }

    /// What pressing it does, in a help line under it, as a body fault's [`Repair`] says it
    /// (ruling M8): painted, never only on hover. A message whose choices have help draws each
    /// choice on a line of its own (`action_help`).
    pub fn help(mut self, help: Sentence) -> Self {
        self.help = Some(help);
        self
    }

    /// Locked for as long as it is drawn, for `reason`. A lock that comes and goes is
    /// `locked_when`.
    pub fn blocked(mut self, reason: Sentence) -> Self {
        self.lock = Some((reason, true));
        self
    }

    /// Locked while `locked`, for `reason`. The reason's line is kept while it is open, so a
    /// lock that comes or goes moves no line below it.
    pub fn locked_when(mut self, locked: bool, reason: Sentence) -> Self {
        self.lock = Some((reason, locked));
        self
    }
}

/// A message's Undo: its step, the id the caller gets back when the DM presses it, and why it is
/// locked, if it is (Settings' `Undo register campaign` while a run is live). Its lock has the
/// two forms of [`Choice`]'s: an optional reason does not fit `blocked`.
///
/// ```
/// use splat_design::outcome::{UndoStep, Verb};
/// let live = true;
/// let _ = UndoStep::verb(Verb::Register).pressed_as(1).locked_when(live, splat_design::sentence!("End the run first."));
/// ```
/// ```compile_fail
/// use splat_design::outcome::{UndoStep, Verb};
/// let live = true;
/// let _ = UndoStep::verb(Verb::Register).pressed_as(1).blocked(live.then_some(splat_design::sentence!("End the run first.")));
/// ```
#[must_use = "give it to a result's `undo`"]
pub struct Undo<A> {
    step: UndoStep,
    id: A,
    lock: Lock,
}

impl UndoStep {
    /// This step as an Undo that gives `id` back when pressed.
    pub fn pressed_as<A>(self, id: A) -> Undo<A> {
        Undo {
            step: self,
            id,
            lock: None,
        }
    }
}

impl<A> Undo<A> {
    /// Locked for as long as it is drawn, for `reason`; a lock that comes and goes is
    /// `locked_when`.
    pub fn blocked(mut self, reason: Sentence) -> Self {
        self.lock = Some((reason, true));
        self
    }

    /// Locked while `locked`, for `reason`; the reason's line is kept while it is open.
    pub fn locked_when(mut self, locked: bool, reason: Sentence) -> Self {
        self.lock = Some((reason, locked));
        self
    }
}

// ------------------------------------------------------------------------------------------------
// Outcomes

/// A finished change, in the success tone: `<Verb past> <object>.`, then more sentences. Its
/// Undo is made from the same `Verb` and object (`undo`), so the result and its Undo cannot
/// have two verbs. It takes no order: a result reaches the footer only as a [`KeptLine`], which
/// only a `Results` store's view makes.
///
/// ```
/// use splat_design::outcome::{Done, Verb};
/// let _ = Done::new(Verb::Remove, "Take seal").undo(1);
/// ```
/// ```compile_fail
/// use splat_design::outcome::{Done, Verb};
/// let _ = Done::new(Verb::Remove, "Take seal").undo("Undo discard", 1);
/// ```
///
/// An order made by hand does not go in:
///
/// ```compile_fail
/// use splat_design::outcome::{Done, ResultOrder, Verb};
/// let _ = Done::<u8>::new(ResultOrder::default(), Verb::Remove, "Take seal");
/// ```
///
/// The object may come first, and the result may name where it was done; the Undo then tells
/// itself apart by the same place (Open questions' tick):
///
/// ```
/// use splat_design::outcome::{Done, Verb};
/// let done = Done::new(Verb::Tick, "“Who hired the bandits?”").object_first().within("Horn Rings").undo(1);
/// assert_eq!(done.words().as_str(), "“Who hired the bandits?” ticked in Horn Rings.");
/// ```
///
/// Its Undo's lock has the two forms of [`Choice`]'s: an optional reason does not fit
/// `undo_blocked`.
///
/// ```
/// use splat_design::outcome::{Done, Verb};
/// let live = true;
/// let _ = Done::new(Verb::Remove, "Bell").undo(1).undo_locked_when(live, splat_design::sentence!("End the run first."));
/// let _ = Done::new(Verb::Remove, "Bell").undo(1).undo_blocked(splat_design::sentence!("End the run first."));
/// ```
/// ```compile_fail
/// use splat_design::outcome::{Done, Verb};
/// let live = true;
/// let _ = Done::new(Verb::Remove, "Bell").undo(1).undo_blocked(live.then_some(splat_design::sentence!("End the run first.")));
/// ```
#[must_use = "make a `KeptLine` of it with `Shown::outcome`"]
pub struct Done<A> {
    verb: Verb,
    object: String,
    object_first: bool,
    within: Option<String>,
    more: Option<Sentence>,
    undo: Option<(Option<String>, A, Lock)>,
    choices: Vec<Choice<A>>,
}

impl<A> Done<A> {
    pub fn new(verb: Verb, object: impl Into<String>) -> Self {
        Self {
            verb,
            object: object.into(),
            object_first: false,
            within: None,
            more: None,
            undo: None,
            choices: Vec::new(),
        }
    }

    /// The object first, then the verb: `“…” ticked.` in place of `Ticked “…”.`
    pub fn object_first(mut self) -> Self {
        self.object_first = true;
        self
    }

    /// Where it was done, a name of the DM's data: `… in Horn Rings.` Its Undo tells itself
    /// apart by the same words (`Undo tick “…” in Horn Rings`), which a fit never drops.
    pub fn within(mut self, name: impl Into<String>) -> Self {
        self.within = Some(name.into());
        self
    }

    /// Sentences after the first: `With it: Bell is done.`
    pub fn more(mut self, more: Sentence) -> Self {
        self.more = Some(match self.more {
            Some(before) => before.then(more),
            None => more,
        });
        self
    }

    /// Its Undo is live, `Undo <verb> <object>`, and gives `id` back when pressed.
    pub fn undo(mut self, id: A) -> Self {
        self.undo = Some((None, id, None));
        self
    }

    /// Its Undo is live, with the part that tells it from another step of the same words.
    pub fn undo_told_apart(mut self, part: impl Into<String>, id: A) -> Self {
        self.undo = Some((Some(part.into()), id, None));
        self
    }

    /// Its Undo is locked for as long as it is drawn, for `reason`.
    pub fn undo_blocked(mut self, reason: Sentence) -> Self {
        if let Some(undo) = &mut self.undo {
            undo.2 = Some((reason, true));
        }
        self
    }

    /// Its Undo is locked while `locked`, for `reason`; the reason's line is kept while open.
    pub fn undo_locked_when(mut self, locked: bool, reason: Sentence) -> Self {
        if let Some(undo) = &mut self.undo {
            undo.2 = Some((reason, locked));
        }
        self
    }

    pub fn choice(mut self, choice: Choice<A>) -> Self {
        self.choices.push(choice);
        self
    }

    /// `Removed Take seal.` (or `“…” ticked in Horn Rings.`), then the other sentences.
    pub fn words(&self) -> Sentence {
        let object = self.object.trim_end_matches('.');
        let within = self
            .within
            .as_ref()
            .map_or(String::new(), |name| format!(" in {name}"));
        let first = Sentence(Cow::Owned(if self.object_first {
            format!("{object} {}{within}.", self.verb.past_after())
        } else {
            format!("{} {object}{within}.", self.verb.past())
        }));
        match &self.more {
            Some(more) => first.then(more.clone()),
            None => first,
        }
    }
}

/// A finished change in the domain's own words, in the success tone (`Seals for Party changed to
/// 1.`). Its Undo is an [`Undo`], never words. `Said` is the named escape for a result that is
/// not a verb and an object: nothing ties its words to its Undo's verb, so
/// `Said::new(sentence!("Removed Bell.")).undo(UndoStep::of(Verb::Delete, "Bell").pressed_as(id))`
/// compiles. Only `Done` holds one verb for both; a reviewer checks each `Said`.
///
/// ```
/// use splat_design::outcome::{Said, UndoStep};
/// let _ = Said::new(splat_design::sentence!("Bell is done.")).undo(UndoStep::act("Done").pressed_as(1));
/// ```
/// ```compile_fail
/// use splat_design::outcome::{Said, UndoStep};
/// let _ = Said::<u8>::new(splat_design::sentence!("Bell is done.")).undo("Undo: Done");
/// ```
///
/// No message takes a tone:
///
/// ```compile_fail
/// use splat_design::outcome::{Said, Tone, UndoStep};
/// let _ = Said::<u8>::new(splat_design::sentence!("Bell is done.")).tone(Tone::Info);
/// ```
#[must_use = "make a `KeptLine` of it with `Shown::outcome`"]
pub struct Said<A> {
    words: Sentence,
    undo: Option<Undo<A>>,
    choices: Vec<Choice<A>>,
}

impl<A> Said<A> {
    pub fn new(words: Sentence) -> Self {
        Self {
            words,
            undo: None,
            choices: Vec::new(),
        }
    }

    pub fn undo(mut self, undo: Undo<A>) -> Self {
        self.undo = Some(undo);
        self
    }

    pub fn choice(mut self, choice: Choice<A>) -> Self {
        self.choices.push(choice);
        self
    }
}

/// A notice's tone (M4 gap 2). It has no error: an error is a fault, so it is a
/// [`FaultMessage`] and its tone comes from its `Severity`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NoticeTone {
    /// Guidance, or why other actions wait.
    Info,
    /// What an act the DM can take next will cost (`The current folder keeps … Splat no longer
    /// shows these files.`): nothing is wrong yet, so it is no fault.
    Warning,
}

impl NoticeTone {
    fn tone(self) -> Tone {
        match self {
            Self::Info => Tone::Info,
            Self::Warning => Tone::Warning,
        }
    }
}

/// A notice: guidance or why other actions wait (`new`, the info tone), or a warning before an
/// act (`warning`: what the act will cost, in the warning tone with its bar and icon). A notice
/// with an Undo (a result that wrote nothing, `Draft discarded.`) is a kept result:
/// `KeptOutcome::Notice`.
///
/// Its tone is one of the two its constructors name; no notice takes a `Tone`, and none is an
/// error, which is a fault ([`FaultMessage`]):
///
/// ```
/// use splat_design::outcome::{Notice, NoPress};
/// let _ = Notice::<NoPress>::new(splat_design::sentence!("Sync is off."));
/// let _ = Notice::<NoPress>::warning(splat_design::sentence!("Splat no longer shows these files."));
/// ```
/// ```compile_fail,E0599
/// use splat_design::outcome::{Notice, NoPress};
/// let _ = Notice::<NoPress>::error(splat_design::sentence!("Splat no longer shows these files."));
/// ```
/// ```compile_fail,E0599
/// use splat_design::outcome::{Notice, NoPress, Tone};
/// let _ = Notice::<NoPress>::new(splat_design::sentence!("Sync is off.")).tone(Tone::Error);
/// ```
#[must_use = "give it to `Messages::outcome`"]
pub struct Notice<A> {
    tone: NoticeTone,
    words: Sentence,
    choices: Vec<Choice<A>>,
    file: Option<FileLine>,
}

impl<A> Notice<A> {
    /// Guidance, or why other actions wait: the info tone.
    pub fn new(words: Sentence) -> Self {
        Self {
            tone: NoticeTone::Info,
            words,
            choices: Vec::new(),
            file: None,
        }
    }

    /// A warning before an act: what the act the DM can take next will cost. The warning tone.
    pub fn warning(words: Sentence) -> Self {
        Self {
            tone: NoticeTone::Warning,
            ..Self::new(words)
        }
    }

    pub fn choice(mut self, choice: Choice<A>) -> Self {
        self.choices.push(choice);
        self
    }

    /// The file it is about, on a line under its words ([`FileLine`]).
    pub fn file(mut self, file: FileLine) -> Self {
        self.file = Some(file);
        self
    }
}

/// A fault as a footer message: its tone comes from its `Severity`, its words from
/// `fault_words`, both taken when it is made, so it holds no borrow of the fault. A fault that
/// left a way back (an event that waits in a draft) is a kept result: `KeptOutcome::Fault`.
#[must_use = "give it to `Messages::outcome`"]
pub struct FaultMessage<A> {
    tone: Tone,
    words: Sentence,
    choices: Vec<Choice<A>>,
    file: Option<FileLine>,
    hover: Option<String>,
    help: Vec<Sentence>,
}

impl<A> FaultMessage<A> {
    pub fn new(fault: &dyn Fault) -> Self {
        Self {
            tone: fault.severity().tone(),
            words: fault_words(fault),
            choices: Vec::new(),
            file: None,
            hover: None,
            help: Vec::new(),
        }
    }

    pub fn choice(mut self, choice: Choice<A>) -> Self {
        self.choices.push(choice);
        self
    }

    /// What the system itself said (a recorder's own error), on hover over the fault's words:
    /// the words stay the app's plain ones, and the DM can still read the system's. Data, never
    /// instructions to the DM (N0, ruling M21).
    pub fn hover(mut self, said: String) -> Self {
        self.hover = Some(said);
        self
    }

    /// What the DM can do about it, in a help line under its words (before its file line and
    /// choices), as a [`Choice`]'s help says what the choice does: painted, never only on hover.
    /// Each call is a line of its own, in order (the help, then what the fault blocks).
    pub fn help(mut self, help: Sentence) -> Self {
        self.help.push(help);
        self
    }

    /// The file the fault is in, on a line under its words, with the links that open it
    /// ([`FileLine`]). Several faults of one file: give it to the last.
    pub fn file(mut self, file: FileLine) -> Self {
        self.file = Some(file);
        self
    }
}

/// The links of a [`FileLine`], by their visible words.
pub const EDIT_IN_OBSIDIAN: &str = "Edit in Obsidian";
pub const SHOW_FILE: &str = "Show file";

/// A file a footer message is about (M4 gap 1): its path, as data, and the links that open it
/// (`edit`: `Edit in Obsidian`; `show`: `Show file`). Only a footer message holds one
/// ([`FaultMessage::file`], [`Notice::file`]), so it stays in the footer's fixed region, which
/// scrolls beyond its room (a third of the pane when pinned); it is never drawn in a pane's
/// content.
///
/// The part draws it on a line under the message's words, from their first letter: the path,
/// then the links, on one line when they fit; else the path on its own line and the links on
/// the line under it, wrapped between whole links. The path is never changed: not by
/// `Sentence::outside`, never given a capital, never wrapped inside; a path wider than its line
/// is cut in the middle (`Sessions/Ses…/Timeline.md`), with the whole path in its hover and as
/// its accessible name. Each link is a target at least `XL` (24 px) square; its accessible name
/// holds the relative path (`Show file: Sessions/Timeline.md`), so two files of one name are
/// told apart, and its tip shows the whole path on hover and on keyboard focus. A press opens the
/// file: no id comes back, and the pane's one press is not used. A file that is gone when the
/// link is pressed opens nothing: the link is locked and says `This file is no longer there.`
/// until the file is back.
///
/// `new` checks its paths at run time (rung 5, part test): the root must be absolute and the
/// relative path plain (no root, no `..`). A line that fails shows its path as given and draws no
/// link, so a link never opens a place other than the folder's own file.
///
/// The path is data: it is no sentence, so no sentence form reaches it. Its fields and its draw
/// are private (these examples prove only that, not the checks of `new`):
///
/// ```
/// use splat_design::outcome::{FileLine, Notice, NoPress};
/// let _ = Notice::<NoPress>::new(splat_design::sentence!("Sync is off."))
///     .file(FileLine::new("/vault/Campaign", "Sessions/Timeline.md").edit().show());
/// ```
/// ```compile_fail,E0451
/// use splat_design::outcome::FileLine;
/// let _ = FileLine { root: "/vault".into(), relative: "Timeline.md".into(), openable: true, edit: true, show: true };
/// ```
/// ```compile_fail,E0624
/// use splat_design::outcome::FileLine;
/// fn f(ui: &mut egui::Ui) {
///     FileLine::new("/vault/Campaign", "Timeline.md").show().draw(ui);
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
#[must_use = "give it to a footer message's `file`"]
pub struct FileLine {
    root: std::path::PathBuf,
    relative: std::path::PathBuf,
    /// The root is absolute and the relative path plain: its links may open it.
    openable: bool,
    edit: bool,
    show: bool,
}

/// What a locked file link says: the file was gone when it was pressed.
const FILE_GONE: Sentence = crate::sentence!("This file is no longer there.");

impl FileLine {
    /// The file at `relative` (as the DM knows it, shown as it is) inside the folder `root`.
    /// `root` must be absolute and `relative` plain (no root, no `..`); else the line shows
    /// `relative` and draws no link.
    pub fn new(root: impl AsRef<std::path::Path>, relative: impl AsRef<std::path::Path>) -> Self {
        use std::path::Component;
        let (root, relative) = (root.as_ref(), relative.as_ref());
        let plain = relative.components().next().is_some()
            && relative
                .components()
                .all(|part| matches!(part, Component::Normal(_) | Component::CurDir));
        Self {
            root: root.to_path_buf(),
            relative: relative.to_path_buf(),
            openable: root.is_absolute() && plain,
            edit: false,
            show: false,
        }
    }

    /// With `Edit in Obsidian`.
    pub fn edit(mut self) -> Self {
        self.edit = true;
        self
    }

    /// With `Show file`.
    pub fn show(mut self) -> Self {
        self.show = true;
        self
    }

    /// The file's whole path, when its links may open it.
    fn path(&self) -> Option<std::path::PathBuf> {
        self.openable.then(|| self.root.join(&self.relative))
    }

    /// The relative path as the DM reads it, `/` between its parts.
    fn shown(&self) -> String {
        let parts: Vec<_> = self
            .relative
            .components()
            .map(|part| part.as_os_str().to_string_lossy())
            .collect();
        parts.join("/")
    }

    fn links(&self) -> impl Iterator<Item = &'static str> {
        [(self.edit, EDIT_IN_OBSIDIAN), (self.show, SHOW_FILE)]
            .into_iter()
            .filter_map(|(on, word)| (on && self.openable).then_some(word))
    }

    /// The line's width on one line: its path and its links, as `draw` measures them.
    fn natural_width(&self, ui: &Ui) -> f32 {
        let gap = ui.spacing().item_spacing.x;
        let links: f32 = self
            .links()
            .map(|word| number_width(ui, word, &Type::Body.font(ui)).max(XL) + gap)
            .sum();
        number_width(ui, &self.shown(), &Type::Small.font(ui))
            + if links > 0.0 { L + links } else { 0.0 }
    }

    /// The line, at the cursor, as wide as `ui` allows.
    fn draw(&self, ui: &mut Ui) {
        let path = self.path();
        let relative = self.shown();
        let whole = path
            .as_ref()
            .map_or_else(|| relative.clone(), |path| path.display().to_string());
        let font = Type::Small.font(ui);
        let gap = ui.spacing().item_spacing.x;
        let links: f32 = self
            .links()
            .map(|word| number_width(ui, word, &Type::Body.font(ui)).max(XL) + gap)
            .sum();
        let room = ui.available_width();
        let beside = room - links - L;
        let one_line = links == 0.0 || number_width(ui, &relative, &font) <= beside;
        let shown = cut_middle(
            ui,
            &relative,
            &font,
            if one_line && links > 0.0 {
                beside
            } else {
                room
            },
        );
        let path_label = |ui: &mut Ui| {
            let response = ui.label(Type::Small.rich(ui, shown).color(colors().muted));
            super::components::named(ui, &response, &whole);
            response.on_hover_text(&whole);
        };
        let draw_links = |ui: &mut Ui| {
            let Some(path) = &path else { return };
            // A press on a file that is gone opens nothing; the link stays locked, saying so,
            // until the file is back (looked for only while it is locked).
            let gone_key = egui::Id::new(("design_file_gone", path));
            let gone = ui.ctx().data(|data| data.get_temp::<bool>(gone_key)) == Some(true)
                && !path.exists();
            if !gone {
                ui.ctx().data_mut(|data| data.remove::<bool>(gone_key));
            }
            for word in self.links() {
                let name = format!("{word}: {relative}");
                let tip = [super::popups::TipLine::Detail(Cow::Owned(whole.clone()))];
                let response = if gone {
                    ui.add_enabled_ui(false, |ui| link(ui, word)).inner
                } else {
                    link(ui, word)
                };
                super::components::named(ui, &response, &name);
                let response = super::popups::rich_tip(response, &tip);
                if response.clicked() {
                    if !path.exists() {
                        ui.ctx().data_mut(|data| data.insert_temp(gone_key, true));
                        continue;
                    }
                    let url = if word == EDIT_IN_OBSIDIAN {
                        Some(obsidian_url(path))
                    } else {
                        file_url(path)
                    };
                    if let Some(url) = url {
                        ui.ctx().open_url(egui::OpenUrl::new_tab(url));
                    }
                }
            }
        };
        let row = egui::Layout::left_to_right(egui::Align::Center);
        if one_line {
            ui.with_layout(row, |ui| {
                path_label(ui);
                draw_links(ui);
            });
        } else {
            ui.with_layout(row, path_label);
            ui.with_layout(row.with_main_wrap(true), draw_links);
        }
        if let Some(path) = &path
            && ui
                .ctx()
                .data(|data| data.get_temp::<bool>(egui::Id::new(("design_file_gone", path))))
                == Some(true)
        {
            blocked_reason(ui, Some(FILE_GONE.as_str()));
        }
    }
}

/// `text` whole when it fits in `room`, else cut in the middle (`Campaign/Ses…/Timeline.md`):
/// the most letters that fit, a third of them from its start and the rest from its end, so the
/// file name stays as long as it can.
fn cut_middle(ui: &Ui, text: &str, font: &egui::FontId, room: f32) -> String {
    if number_width(ui, text, font) <= room {
        return text.to_owned();
    }
    let chars: Vec<char> = text.chars().collect();
    let shown = |keep: usize| {
        let head = keep / 3;
        let tail = keep - head;
        let mut words: String = chars[..head].iter().collect();
        words.push('…');
        words.extend(&chars[chars.len() - tail..]);
        words
    };
    // The widest that fits, by bisection: `log2(n)` measurements.
    let (mut fits, mut wide) = (0, chars.len());
    while wide - fits > 1 {
        let keep = (fits + wide) / 2;
        if number_width(ui, &shown(keep), font) <= room {
            fits = keep;
        } else {
            wide = keep;
        }
    }
    shown(fits)
}

/// The link that opens a file in Obsidian: `obsidian://open?path=` and the path, each byte
/// outside `A–Z a–z 0–9 - . _ ~ /` written as `%XX` (a space, `&`, `#`, `%`, `?` and every
/// non-ASCII byte). The one copy of this rule; `world.rs` still has its own (package D).
pub fn obsidian_url(path: &std::path::Path) -> String {
    let encoded: String = path
        .to_string_lossy()
        .bytes()
        .map(|byte| {
            if byte.is_ascii_alphanumeric() || b"-._~/".contains(&byte) {
                (byte as char).to_string()
            } else {
                format!("%{byte:02X}")
            }
        })
        .collect();
    format!("obsidian://open?path={encoded}")
}

/// The `file:` URL of an absolute path, by the platform's rules (`url::Url::from_file_path`: a
/// Windows drive is a path, never a host). `None` for a path that is not absolute.
pub fn file_url(path: &std::path::Path) -> Option<String> {
    url::Url::from_file_path(path).ok().map(String::from)
}

/// A footer message that is not a result: it has no order, and keeps the caller's order among
/// its kind.
///
/// | Variant | Tone | Place |
/// |---|---|---|
/// | `Refused` | warning | reasons |
/// | `Fault` | from its `Severity` | reasons |
/// | `Notice` | info | notices |
///
/// An `Outcome` has no public draw method; it reaches the screen only through
/// `Messages::outcome`. A result is not an `Outcome`:
///
/// ```
/// use splat_design::outcome::{Messages, NoPress, Outcome};
/// fn f(ui: &mut egui::Ui) {
///     let frame = splat_design::outcome::PaneFrame::new(ui);
///     let mut messages = Messages::<NoPress>::new(&frame, "f", 400.0);
///     messages.outcome(Outcome::Refused(splat_design::sentence!("No.")));
///     let _ = messages.pane(ui, splat_design::pane::Body::Content, |_| ());
/// }
/// ```
/// ```compile_fail
/// use splat_design::outcome::{Done, Messages, NoPress, Verb};
/// fn f(ui: &mut egui::Ui) {
///     let frame = splat_design::outcome::PaneFrame::new(ui);
///     let mut messages = Messages::<NoPress>::new(&frame, "f", 400.0);
///     messages.outcome(Done::new(Verb::Remove, "Bell"));
///     let _ = messages.pane(ui, splat_design::pane::Body::Content, |_| ());
/// }
/// ```
/// ```compile_fail
/// fn f(ui: &mut egui::Ui) {
///     splat_design::outcome::Outcome::<u8>::Refused(splat_design::sentence!("No.")).draw(ui);
/// }
/// ```
pub enum Outcome<A> {
    Notice(Notice<A>),
    /// Why an act or Save wrote nothing: it does not fit the files now.
    Refused(Sentence),
    Fault(FaultMessage<A>),
}

impl<A> From<Notice<A>> for Outcome<A> {
    fn from(notice: Notice<A>) -> Self {
        Self::Notice(notice)
    }
}

impl<A> From<FaultMessage<A>> for Outcome<A> {
    fn from(fault: FaultMessage<A>) -> Self {
        Self::Fault(fault)
    }
}

/// A result: what an action did, kept in a `design::Results` store. It reaches the footer only
/// as a [`KeptLine`], which `Shown::outcome` makes from a store's view, so its order is that
/// view's, issued by its store at the action. What the types do not hold, and so stays with the
/// caller: class a result as a result (`Notice::new(sentence!("Removed Bell."))` compiles and
/// needs no store), choose the right owner and store (a caller can take B's view and describe
/// A), and make the words from that view's value. Results go newest first; one with a live Undo
/// leads the footer, one without follows the notices.
///
/// | Variant | Tone |
/// |---|---|
/// | `Done`, `Said` | success |
/// | `Notice` (with its Undo) | info |
/// | `Offer` (its choices, no Undo) | info |
/// | `Fault` (with its Undo) | from its `Severity` |
pub enum KeptOutcome<A> {
    Done(Done<A>),
    Said(Said<A>),
    Notice(Notice<A>, Undo<A>),
    Offer(Offer<A>),
    Fault(FaultMessage<A>, Undo<A>),
}

/// A kept offer: something the DM may do now about a kept entry, in the info tone, with its
/// choice and no Undo (Library's link offer: `Ash is not linked to this campaign.` with `Link
/// to this campaign`). It is a kept line: `Shown::outcome` makes it from a store's view, so its
/// choices' presses come back with that entry's [`Ticket`](super::results::Ticket) and the
/// store acts by it, and a replaced or ended entry's press reaches nothing. It is never a
/// success-tone `Said`: nothing was done yet. It has at least one choice, and no way to hold an
/// Undo.
///
/// ```
/// use splat_design::outcome::{Choice, KeptOutcome, MessageChoice, Offer};
/// let offer: KeptOutcome<u8> = Offer::new(
///     splat_design::sentence!("Ash is not linked to this campaign."),
///     Choice::new(MessageChoice::Choice("Link to this campaign"), 1),
/// )
/// .into();
/// ```
/// ```compile_fail
/// use splat_design::outcome::{Offer, UndoStep, Verb};
/// let _ = Offer::<u8>::new(splat_design::sentence!("Ash is not linked."), todo!())
///     .undo(UndoStep::of(Verb::Change, "Ash").pressed_as(1));
/// ```
#[must_use = "give it to `Shown::outcome`"]
pub struct Offer<A> {
    words: Sentence,
    choices: Vec<Choice<A>>,
}

impl<A> Offer<A> {
    /// The offer's words and its first choice.
    pub fn new(words: Sentence, choice: Choice<A>) -> Self {
        Self {
            words,
            choices: vec![choice],
        }
    }

    /// Another choice, after the ones before it.
    pub fn choice(mut self, choice: Choice<A>) -> Self {
        self.choices.push(choice);
        self
    }
}

impl<A> From<Offer<A>> for KeptOutcome<A> {
    fn from(offer: Offer<A>) -> Self {
        Self::Offer(offer)
    }
}

impl<A> From<Done<A>> for KeptOutcome<A> {
    fn from(done: Done<A>) -> Self {
        Self::Done(done)
    }
}

impl<A> From<Said<A>> for KeptOutcome<A> {
    fn from(said: Said<A>) -> Self {
        Self::Said(said)
    }
}

/// A message as drawn: tone, words, Undo and choices, each with its lock. Owned: it holds no
/// borrow, so a footer of them can sit beside a body that changes the app, and a retained
/// projection (`design::results::Projection`) can keep it across frames as data, and a typed
/// footer's plan compares it with the last one (`PlanLine`).
#[derive(Clone, PartialEq)]
struct Body {
    tone: Tone,
    words: Sentence,
    undo: Option<UndoStep>,
    undo_lock: Lock,
    choices: Vec<MessageChoice<'static>>,
    named: Vec<Option<(ChoiceKind, ChoiceWords)>>,
    locks: Vec<Lock>,
    hovers: Vec<Option<String>>,
    /// Each choice's help line (`Choice::help`), by index.
    helps: Vec<Option<Sentence>>,
    /// The file it is about, on a line under the words (`FileLine`).
    file: Option<FileLine>,
    /// What the system itself said, on hover over the words (`FaultMessage::hover`).
    words_hover: Option<String>,
    /// What the DM can do, in help lines under the words (`FaultMessage::help`).
    help: Vec<Sentence>,
    /// No other kept result of its Undo's verb stands in its footer (`undo_alone`): the Undo
    /// paints `UndoStep::alone` for its room, else `UndoStep::fit`.
    undo_alone: bool,
    /// The place its owner gives it (`BodyMessage::show`: the next widget's id before its words'
    /// scope), which keys its kept Undo words and choices (`body_kept`): one slot per place,
    /// whatever its words. None: the place where it draws.
    place: Option<egui::Id>,
}

/// Whether `undo` is the only kept Undo of its verb among a footer's kept Undos (`kept`): the
/// one rule for every footer and row of messages, so a lone Undo whose whole words do not fit
/// gives up its object (`Undo tick`, its result names it) rather than cut it, and two ticks
/// keep the words that tell them apart.
fn undo_alone(kept: &[Verb], undo: Option<&UndoStep>) -> bool {
    undo.and_then(UndoStep::counted)
        .is_some_and(|verb| kept.iter().filter(|&&kind| kind == verb).count() == 1)
}

/// A body and the caller's ids of its Undo and of each choice, by index.
pub(in crate::design) struct Typed<A> {
    body: Body,
    undo: Option<A>,
    choices: Vec<A>,
}

impl<A: Clone> Clone for Typed<A> {
    fn clone(&self) -> Self {
        Self {
            body: self.body.clone(),
            undo: self.undo.clone(),
            choices: self.choices.clone(),
        }
    }
}

impl<A> Typed<A> {
    /// The bytes of its words, for the work counts of a retained projection.
    pub(in crate::design) fn word_bytes(&self) -> usize {
        self.body.words.as_str().len()
    }

    /// Its words, for a test of a retained projection.
    #[cfg(test)]
    pub(in crate::design) fn words(&self) -> &str {
        self.body.words.as_str()
    }

    /// Whether its Undo's lock holds, for a test of a retained projection.
    #[cfg(test)]
    pub(in crate::design) fn undo_locked(&self) -> bool {
        holding(&self.body.undo_lock).is_some()
    }
}

impl<A> Typed<A> {
    fn new(tone: Tone, words: Sentence, undo: Option<Undo<A>>, choices: Vec<Choice<A>>) -> Self {
        let (undo, undo_lock, undo_id) = match undo {
            Some(undo) => (Some(undo.step), undo.lock, Some(undo.id)),
            None => (None, None, None),
        };
        let mut ids = Vec::with_capacity(choices.len());
        let (mut whats, mut locks, mut hovers) = (Vec::new(), Vec::new(), Vec::new());
        let mut helps = Vec::new();
        let mut named = Vec::new();
        // Safe first: a choice that loses something follows every other (a stable order).
        let mut choices = choices;
        choices.sort_by_key(|choice| matches!(choice.what, MessageChoice::Lose(_)));
        // One primary at most: it is the message's one commit (a test build refuses a second).
        debug_assert!(
            choices
                .iter()
                .filter(|choice| matches!(choice.what, MessageChoice::Primary(_)))
                .count()
                <= 1,
            "a message has at most one primary choice"
        );
        for choice in choices {
            whats.push(choice.what);
            named.push(choice.named);
            locks.push(choice.lock);
            hovers.push(choice.hover);
            helps.push(choice.help);
            ids.push(choice.id);
        }
        Self {
            body: Body {
                tone,
                words,
                undo,
                undo_lock,
                choices: whats,
                named,
                locks,
                hovers,
                helps,
                file: None,
                words_hover: None,
                help: Vec::new(),
                undo_alone: false,
                place: None,
            },
            undo: undo_id,
            choices: ids,
        }
    }

    /// With the file it is about.
    fn with_file(mut self, file: Option<FileLine>) -> Self {
        self.body.file = file;
        self
    }

    /// The same message with each caller's id mapped by `id`.
    fn map<B>(self, id: impl Fn(A) -> B) -> Typed<B> {
        Typed {
            body: self.body,
            undo: self.undo.map(&id),
            choices: self.choices.into_iter().map(id).collect(),
        }
    }

    /// With what the system itself said, on hover over the words.
    fn with_hover(mut self, hover: Option<String>) -> Self {
        self.body.words_hover = hover;
        self
    }

    fn with_help(mut self, help: Vec<Sentence>) -> Self {
        self.body.help = help;
        self
    }

    /// The caller's id of what the part pressed.
    fn id(self, pressed: Pressed) -> Option<A> {
        match pressed {
            Pressed::Undo => self.undo,
            Pressed::Choice(at) => self.choices.into_iter().nth(at),
        }
    }
}

impl<A> Outcome<A> {
    /// The footer's rank, and what it draws.
    fn parts(self) -> (FooterRank, Typed<A>) {
        match self {
            Self::Notice(notice) => (
                FooterRank::Notice,
                Typed::new(notice.tone.tone(), notice.words, None, notice.choices)
                    .with_file(notice.file),
            ),
            Self::Refused(words) => (
                FooterRank::Reason,
                Typed::new(Tone::Warning, words, None, Vec::new()),
            ),
            Self::Fault(fault) => (
                FooterRank::Reason,
                Typed::new(fault.tone, fault.words, None, fault.choices)
                    .with_file(fault.file)
                    .with_hover(fault.hover)
                    .with_help(fault.help),
            ),
        }
    }
}

impl<A> KeptOutcome<A> {
    /// What it draws.
    fn typed(self) -> Typed<A> {
        match self {
            Self::Done(done) => {
                let words = done.words();
                let undo = done.undo.map(|(part, id, lock)| {
                    let step = UndoStep::of(done.verb, done.object.trim_end_matches('.'));
                    let within = done.within.as_ref().map(|name| format!("in {name}"));
                    let part = match (within, part) {
                        (Some(within), Some(part)) => Some(format!("{within} {part}")),
                        (within, part) => within.or(part),
                    };
                    let step = match part {
                        Some(part) => step.tell_apart(part),
                        None => step,
                    };
                    Undo {
                        lock,
                        ..step.pressed_as(id)
                    }
                });
                Typed::new(Tone::Success, words, undo, done.choices)
            }
            Self::Said(said) => Typed::new(Tone::Success, said.words, said.undo, said.choices),
            Self::Notice(notice, undo) => {
                Typed::new(notice.tone.tone(), notice.words, Some(undo), notice.choices)
                    .with_file(notice.file)
            }
            Self::Offer(offer) => Typed::new(Tone::Info, offer.words, None, offer.choices),
            Self::Fault(fault, undo) => {
                Typed::new(fault.tone, fault.words, Some(undo), fault.choices)
                    .with_file(fault.file)
                    .with_hover(fault.hover)
                    .with_help(fault.help)
            }
        }
    }
}

/// This frame's token for one pane: a local value that every kept line and the pane's
/// [`Messages`] borrow. A line therefore cannot outlive the frame it was made in: it cannot be
/// kept in app state and shown after its result was replaced or ended. The body may still take
/// `&mut` of the store's owner, because the store's borrow ends when the line is made.
///
/// The one way around it is to leak a token (`Box::leak`), which a reviewer sees.
pub struct PaneFrame {
    _not_static: std::marker::PhantomData<*const ()>,
}

impl PaneFrame {
    /// A token with no `Ui`, for the design system's own tests.
    #[cfg(test)]
    pub(in crate::design) fn for_test() -> Self {
        Self {
            _not_static: std::marker::PhantomData,
        }
    }

    /// The token for a pane drawn in this frame, on `ui`.
    pub fn new(ui: &Ui) -> Self {
        let _ = ui;
        Self {
            _not_static: std::marker::PhantomData,
        }
    }
}

/// One kept result as a footer line for this frame: what it says, and the store's [`Ticket`]
/// for that entry, which binds its order and its presses to it. Only `Shown::outcome` makes
/// one, from a store's view; that call ends the store's borrow, and the line borrows the pane's
/// [`PaneFrame`] instead. It is not `Clone`, its ticket cannot be read, and it cannot be kept
/// past its frame.
///
/// ```
/// # use splat_design::ownership::{DiskChanged, Ends, Kept, Owner, OwnerChanged, Press, Results};
/// # #[derive(Clone, PartialEq, Eq)] struct Link;
/// # impl Owner for Link { type Scope = str; fn scope(&self) -> &str { "c" } }
/// # struct Saved;
/// # impl Kept for Saved { type Owner = Link; const ENDS: Ends = Ends { owner_changed: OwnerChanged::Ends, disk_changed: DiskChanged::Stays, press: Press::Nothing }; fn owner(&self) -> Link { Link } }
/// use splat_design::outcome::{KeptLine, NoPress, PaneFrame, Said};
/// fn frame(ui: &mut egui::Ui) {
///     let mut store = Results::new();
///     store.record(Saved);
///     let frame = PaneFrame::new(ui);
///     let line: KeptLine<NoPress> = store
///         .get(&Link)
///         .unwrap()
///         .outcome(&frame, |_| Said::new(splat_design::sentence!("Campaign link saved.")).into());
///     // The store is free again while the line lives.
///     store.record(Saved);
///     let _ = line;
/// }
/// ```
///
/// Not without a store, not cloned, its ticket not read, and not kept in app state:
///
/// ```compile_fail
/// use splat_design::outcome::{KeptLine, NoPress};
/// let _ = KeptLine::<NoPress> { ticket: todo!(), typed: todo!(), frame: todo!() };
/// ```
/// ```compile_fail
/// # use splat_design::ownership::{DiskChanged, Ends, Kept, Owner, OwnerChanged, Press, Results};
/// # #[derive(Clone, PartialEq, Eq)] struct Link;
/// # impl Owner for Link { type Scope = str; fn scope(&self) -> &str { "c" } }
/// # struct Saved;
/// # impl Kept for Saved { type Owner = Link; const ENDS: Ends = Ends { owner_changed: OwnerChanged::Ends, disk_changed: DiskChanged::Stays, press: Press::Nothing }; fn owner(&self) -> Link { Link } }
/// use splat_design::outcome::{KeptLine, NoPress, PaneFrame, Said};
/// fn frame(ui: &mut egui::Ui) {
///     let mut store = Results::new();
///     store.record(Saved);
///     let frame = PaneFrame::new(ui);
///     let line: KeptLine<NoPress> = store
///         .get(&Link)
///         .unwrap()
///         .outcome(&frame, |_| Said::new(splat_design::sentence!("Campaign link saved.")).into());
///     let _twice = line.clone();
/// }
/// ```
/// ```compile_fail
/// # use splat_design::ownership::{DiskChanged, Ends, Kept, Owner, OwnerChanged, Press, Results};
/// # #[derive(Clone, PartialEq, Eq)] struct Link;
/// # impl Owner for Link { type Scope = str; fn scope(&self) -> &str { "c" } }
/// # struct Saved;
/// # impl Kept for Saved { type Owner = Link; const ENDS: Ends = Ends { owner_changed: OwnerChanged::Ends, disk_changed: DiskChanged::Stays, press: Press::Nothing }; fn owner(&self) -> Link { Link } }
/// use splat_design::outcome::{KeptLine, NoPress, PaneFrame, Said};
/// fn frame(ui: &mut egui::Ui) {
///     let mut store = Results::new();
///     store.record(Saved);
///     let frame = PaneFrame::new(ui);
///     let line: KeptLine<NoPress> = store
///         .get(&Link)
///         .unwrap()
///         .outcome(&frame, |_| Said::new(splat_design::sentence!("Campaign link saved.")).into());
///     let _ticket = line.ticket;
/// }
/// ```
///
/// A line kept in app state across frames (the review's counterexample) does not compile; the
/// same line used in its frame does (the first example):
///
/// ```compile_fail
/// # use splat_design::ownership::{DiskChanged, Ends, Kept, Owner, OwnerChanged, Press, Results};
/// # #[derive(Clone, PartialEq, Eq)] struct Link;
/// # impl Owner for Link { type Scope = str; fn scope(&self) -> &str { "c" } }
/// # struct Saved;
/// # impl Kept for Saved { type Owner = Link; const ENDS: Ends = Ends { owner_changed: OwnerChanged::Ends, disk_changed: DiskChanged::Stays, press: Press::Nothing }; fn owner(&self) -> Link { Link } }
/// use splat_design::outcome::{KeptLine, NoPress, PaneFrame, Said};
/// struct App { store: Results<Link, Saved>, kept: Option<KeptLine<'static, NoPress>> }
/// fn frame(ui: &mut egui::Ui, app: &mut App) {
///     let frame = PaneFrame::new(ui);
///     app.kept = app
///         .store
///         .get(&Link)
///         .map(|shown| shown.outcome(&frame, |_| Said::new(splat_design::sentence!("Saved.")).into()));
/// }
/// ```
#[must_use = "give it to `Messages::kept`"]
pub struct KeptLine<'f, A> {
    ticket: Ticket,
    typed: Typed<A>,
    /// The order of the entry's refused attempt, while its refusal stands: the line ranks by it
    /// ([`kept_rank`]), and keeps the entry's ticket for its ids and presses.
    refused: Option<ResultOrder>,
    frame: std::marker::PhantomData<&'f PaneFrame>,
}

impl<'f, A> KeptLine<'f, A> {
    /// For `Shown::outcome` (in `results`), which holds the entry's ticket and the refusal of
    /// its last attempt. A refused attempt's line is one answer: the refusal's words first, in
    /// the tone of its diagnosis (`Refusal::tone`, R41); the result's own words follow only while its Undo is live (the caller
    /// gives the Undo only then), because only then does the file still hold what it says; then
    /// the Undo, once.
    pub(in crate::design) fn new(
        frame: &'f PaneFrame,
        ticket: Ticket,
        outcome: KeptOutcome<A>,
        refused: Option<(ResultOrder, Sentence, Tone)>,
    ) -> Self {
        let (typed, refused) = prepare(outcome, refused);
        Self::prepared(frame, ticket, typed, refused)
    }

    /// The same line with each of its caller's ids mapped by `id`: a line that another page's
    /// store made for this pane's footer (the Encounters store's group discard in the Session
    /// plan's footer). Its ticket stays its store's, so its press reaches only that entry.
    pub fn map<B>(self, id: impl Fn(A) -> B) -> KeptLine<'f, B> {
        KeptLine {
            ticket: self.ticket,
            typed: self.typed.map(id),
            refused: self.refused,
            frame: std::marker::PhantomData,
        }
    }

    /// Its ticket, for a test of `Results::present`.
    #[cfg(test)]
    pub(in crate::design) fn ticket(&self) -> &Ticket {
        &self.ticket
    }

    /// A line from what [`prepare`] made: for `Results::present`, which keeps the prepared
    /// message in a retained projection and mints the ticket in the frame that shows it.
    pub(in crate::design) fn prepared(
        _frame: &'f PaneFrame,
        ticket: Ticket,
        typed: Typed<A>,
        refused: Option<ResultOrder>,
    ) -> Self {
        Self {
            ticket,
            typed,
            refused,
            frame: std::marker::PhantomData,
        }
    }
}

/// A kept result's message as its line draws it, with the order of its refused attempt while
/// the refusal stands. A refused attempt's line is one answer: the refusal's words first, in the
/// tone of its diagnosis (`Refusal::tone`, R41); the result's own words follow only while its
/// Undo is live; then the Undo, once. It needs no frame: a retained projection keeps it as data.
pub(in crate::design) fn prepare<A>(
    outcome: KeptOutcome<A>,
    refused: Option<(ResultOrder, Sentence, Tone)>,
) -> (Typed<A>, Option<ResultOrder>) {
    let mut typed = outcome.typed();
    let refused = refused.map(|(order, words, tone)| {
        typed.body.tone = tone;
        let result = std::mem::replace(&mut typed.body.words, words);
        if typed.body.undo.is_some() {
            typed.body.words = typed.body.words.clone().then(result);
        }
        order
    });
    (typed, refused)
}

/// A footer line as the one footer order sees it ([`footer_order`]).
pub(in crate::design) trait Ranked {
    /// Its place, and its act's order (a kept line's; `None` for a standing line).
    fn key(&self) -> (FooterRank, Option<ResultOrder>);
}

/// The one order of a pane's footer ([`Messages`], `Footer::draw_messages`) and of a save row's
/// messages ([`RowMessages`]). Standing lines (faults, warnings, reasons, notices with no act
/// order) keep their places; answers (lines with an act order) stand newest first inside the
/// group of results with a live Undo and inside the group without one: results with a live
/// Undo, then reasons, then notices and results with no Undo. The one line above them all is
/// the footer's newest answer when it is a refusal ([`refusal_rank`]); `Messages::answer`
/// (Campaign) comes before even that.
pub(in crate::design) fn footer_order<T: Ranked>(items: &mut [T]) {
    items.sort_by_key(|item| {
        let (rank, order) = item.key();
        (rank, order.map(std::cmp::Reverse))
    });
}

/// Where a refused attempt's line stands (ruling R42 as amended, Q-x): first in its footer while
/// its attempt is the newest answer there (`newest`: the newest act order of the footer's lines,
/// its own included); else in the group of results with no Undo, by its attempt's order, whether
/// or not its Undo is live, so every newer answer stands above it. It never ends here.
pub(in crate::design) fn refusal_rank(
    attempt: ResultOrder,
    newest: Option<ResultOrder>,
) -> FooterRank {
    if newest.is_none_or(|newest| attempt >= newest) {
        FooterRank::Answer
    } else {
        FooterRank::Notice
    }
}

/// A kept line that is not a refusal: a live Undo before reasons, no Undo after the notices.
fn kept_place(has_undo: bool) -> FooterRank {
    if has_undo {
        FooterRank::Undo
    } else {
        FooterRank::Notice
    }
}

/// A pane footer's message (`Footer`'s own list).
impl<'a> Ranked
    for (
        FooterRank,
        Option<ResultOrder>,
        Cow<'a, str>,
        Box<dyn FnOnce(&mut Ui) + 'a>,
    )
{
    fn key(&self) -> (FooterRank, Option<ResultOrder>) {
        (self.0, self.1)
    }
}

impl<A> Ranked for RowEntry<A> {
    fn key(&self) -> (FooterRank, Option<ResultOrder>) {
        (self.0, self.1)
    }
}

/// What the DM pressed in a pane's footer: the kind of action, as the caller's own id, and for
/// a kept result the store's [`Ticket`] of that entry. The store acts by the ticket
/// (`Results::undo_by`, `use_press_by`), so a press can only reach the entry its line showed;
/// the id names only the kind, and two results with one id cannot be confused.
#[must_use = "act on the press"]
pub struct Press<A> {
    pub id: A,
    pub ticket: Option<Ticket>,
}

/// The footer line a press came from, for an answer to that press (`Messages::answer`): a copy
/// of the line's identity that cannot act, kept by the answer's owner while the answer stands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PressedLine(ResultOrder);

impl<A> Press<A> {
    /// The kept line this press came from, if it came from one.
    pub fn line(&self) -> Option<PressedLine> {
        self.ticket
            .as_ref()
            .map(|ticket| PressedLine(ticket.order()))
    }
}

/// What a pane's body returned, and the one footer press of this frame. At most one press
/// comes back per pane and frame: the first in the footer's drawing order (newest result first,
/// then reasons, then notices) wins, and a second in the same frame is dropped.
#[must_use = "act on `pressed`"]
pub struct Paned<R, A> {
    pub body: R,
    pub pressed: Option<Press<A>>,
}

/// The pane's one press slot: the first press of the frame wins.
struct PressSink<A>(std::rc::Rc<std::cell::RefCell<Option<Press<A>>>>);

impl<A> PressSink<A> {
    fn press(&self, press: Press<A>) {
        let mut slot = self.0.borrow_mut();
        if slot.is_none() {
            *slot = Some(press);
        }
    }
}

/// A pane's footer messages for this frame, owned and typed: built before the body runs, from
/// values that hold no borrow of the app (`'static` words, ids and sentences; a kept result as a
/// [`KeptLine`] of this frame). So the body may change any of the app, also the store a result
/// came from. `pane` draws the body, then the footer (`Footer::pane`'s placement), and gives
/// back the body's value with the press. No message needs a slot for its press. Each kept
/// line's controls take their egui ids from its ticket, so a result added or removed above it
/// moves no focus or pending press onto another result's control.
///
/// A footer entry that borrows the app does not compile:
///
/// ```
/// use splat_design::outcome::{Choice, MessageChoice, Messages, Notice, PaneFrame};
/// #[derive(Clone, Copy)] enum Act { Again }
/// fn f(ui: &mut egui::Ui) -> Option<Act> {
///     let frame = PaneFrame::new(ui);
///     let mut messages = Messages::new(&frame, "f", 400.0);
///     messages.outcome(
///         Notice::new(splat_design::sentence!("Sync is off."))
///             .choice(Choice::new(MessageChoice::Choice("Try again"), Act::Again)),
///     );
///     messages.pane(ui, splat_design::pane::Body::Content, |_| ()).pressed.map(|press| press.id)
/// }
/// ```
/// ```compile_fail
/// use splat_design::outcome::{Choice, MessageChoice, Messages, Notice, PaneFrame};
/// #[derive(Clone, Copy)] enum Act { Again }
/// fn f(ui: &mut egui::Ui, name: String) -> Option<Act> {
///     let frame = PaneFrame::new(ui);
///     let mut messages = Messages::new(&frame, "f", 400.0);
///     messages.outcome(
///         Notice::new(splat_design::sentence!("Sync is off."))
///             .choice(Choice::new(MessageChoice::Choice(&name), Act::Again)),
///     );
///     messages.pane(ui, splat_design::pane::Body::Content, |_| ()).pressed.map(|press| press.id)
/// }
/// ```
///
/// Nor an id that borrows:
///
/// ```compile_fail
/// use splat_design::outcome::{Choice, MessageChoice, Messages, Notice, PaneFrame};
/// fn f<'a>(ui: &mut egui::Ui, app: &'a mut u8) {
///     let frame = PaneFrame::new(ui);
///     let mut messages = Messages::new(&frame, "f", 400.0);
///     messages.outcome(
///         Notice::new(splat_design::sentence!("Sync is off."))
///             .choice(Choice::new(MessageChoice::Choice("Try again"), &*app)),
///     );
///     let _ = messages.pane(ui, splat_design::pane::Body::Content, |_| ());
/// }
/// ```
pub struct Messages<'f, A: 'static> {
    id: egui::Id,
    width: f32,
    /// The footer's lines, typed: what each draws, its place in the order and a kept result's
    /// ticket. No line is a closure (layout PR 2b): the footer draws them from this data, in the
    /// order its retained plan holds.
    lines: Vec<FooterLine<A>>,
    frame: std::marker::PhantomData<&'f PaneFrame>,
    /// Whether this frame's footer has its one `answer` (ruling Q-x).
    answered: bool,
    /// The newest act order of this footer's kept lines ([`refusal_rank`]).
    newest: Option<ResultOrder>,
    /// The body ends at its last whole registered row at its own bottom too ([`Self::rows_end_whole`]).
    rows_end_whole: bool,
}

/// One line of a [`Messages`] footer, as data.
struct FooterLine<A> {
    /// Its place in the footer's one order; a refused attempt's is decided when every line is
    /// in (`refused`).
    rank: FooterRank,
    /// Its act's order (a kept line's; `None` for a standing line).
    order: Option<ResultOrder>,
    typed: Typed<A>,
    /// A kept result's ticket: its controls' ids and its presses.
    ticket: Option<Ticket>,
    /// A refused attempt's line, whose place waits for the footer's newest act order.
    refused: bool,
    /// The answer to a press of a kept line: that line's order (`Messages::answer`).
    under: Option<ResultOrder>,
}

impl<'f, A: 'static> Messages<'f, A> {
    /// No messages yet, for this frame's pane, with a footer `width` wide. `id` keeps its
    /// height, scroll, focus and its retained plan.
    pub fn new(frame: &'f PaneFrame, id: impl std::hash::Hash, width: f32) -> Self {
        let _ = frame;
        Self {
            id: egui::Id::new(id),
            width,
            lines: Vec::new(),
            frame: std::marker::PhantomData,
            answered: false,
            newest: None,
            rows_end_whole: false,
        }
    }

    /// Layout step 11a (SM5, AM16): the body's own bottom is an edge as the footer is, so its
    /// content never stops through a registered row there either: it ends at the last whole row,
    /// with the hairline. For a body of short rows that goes on below its room with no footer
    /// line (a sheet at the window's bottom). Without it only the footer and the session banner
    /// cut at a whole row (N12).
    pub fn rows_end_whole(&mut self) -> &mut Self {
        self.rows_end_whole = true;
        self
    }

    /// A reason or a notice, at the place its variant gives it in the footer's order.
    pub fn outcome(&mut self, outcome: impl Into<Outcome<A>>) -> &mut Self {
        let (rank, typed) = outcome.into().parts();
        self.lines.push(FooterLine {
            rank,
            order: None,
            typed,
            ticket: None,
            refused: false,
            under: None,
        });
        self
    }

    /// The answer to the DM's last act when it failed (ruling Q-x): a fault that the caller
    /// holds as its newest answer and ends at the next act. It comes first in the footer, before
    /// every kept result, which are answers to older acts. One per footer (a second is refused in
    /// debug builds and tests); Clippy refuses it outside a counted use (`Ban::Answer`).
    ///
    /// `under`: the act was a press of a kept line of this footer (`Press::line`). While the
    /// footer follows its content (its top stays and it grows down), the answer stands directly
    /// under that line, so the line the DM pressed keeps its place under the pointer (layout step
    /// 7, Astra's P1 on PR 619); pinned (its bottom stays and it grows up), it comes first. Give
    /// `under` only when no answer stood in this footer before the press: an answer that replaces
    /// one takes its first place, so the lines below move by no more than the change of height.
    pub fn answer(&mut self, fault: FaultMessage<A>, under: Option<PressedLine>) -> &mut Self {
        debug_assert!(
            !self.answered,
            "a footer holds one answer to the DM's last act"
        );
        self.answered = true;
        let (_, typed) = Outcome::from(fault).parts();
        self.lines.push(FooterLine {
            rank: FooterRank::Answer,
            order: None,
            typed,
            ticket: None,
            refused: false,
            under: under.map(|line| line.0),
        });
        self
    }

    /// A kept result of this frame, newest first by the order its store issued.
    pub fn kept(&mut self, line: KeptLine<'f, A>) -> &mut Self {
        self.kept_at(line.ticket, line.typed, line.refused)
    }

    fn kept_at(
        &mut self,
        ticket: Ticket,
        typed: Typed<A>,
        refused: Option<ResultOrder>,
    ) -> &mut Self {
        let order = refused.unwrap_or(ticket.order());
        self.newest = self.newest.max(Some(order));
        self.lines.push(FooterLine {
            rank: kept_place(typed.body.undo.is_some()),
            order: Some(order),
            typed,
            ticket: Some(ticket),
            refused: refused.is_some(),
            under: None,
        });
        self
    }

    /// The staged path for a result whose caller still issues its own `ResultOrder` (Wave 2
    /// moves it into a `Results` store). Clippy refuses it (`Ban::Order`).
    pub fn outcome_at(
        &mut self,
        order: ResultOrder,
        outcome: impl Into<KeptOutcome<A>>,
    ) -> &mut Self {
        self.kept_at(Ticket::of(order), outcome.into().typed(), None)
    }

    /// Every line is in: the refused attempts take their places (`refusal_rank`). Then the
    /// typed footer, and the pane's one press slot.
    fn into_footer(self) -> (Footer<'static>, PressSink<A>) {
        let Self {
            id,
            width,
            mut lines,
            newest,
            rows_end_whole,
            ..
        } = self;
        // Each Undo's short form (`undo_alone`) is the footer's plan's, worked out when it is
        // made (`TypedFooter::draw`).
        for line in &mut lines {
            if line.refused {
                line.rank = refusal_rank(line.order.unwrap_or_default(), newest);
            }
        }
        let sink = PressSink(std::rc::Rc::default());
        let typed = TypedFooter {
            lines,
            sink: PressSink(sink.0.clone()),
            retained: None,
            plan: None,
        };
        let mut footer = Footer::typed(id, width, Box::new(typed));
        footer.whole_rows = rows_end_whole;
        (footer, sink)
    }

    /// The pane: the body (`add`, which may change any of the app), then the footer, placed by
    /// `Footer::pane`. The body's value, and the press.
    pub fn pane<R>(
        self,
        ui: &mut Ui,
        body: super::pane::Body,
        add: impl FnOnce(&mut Ui) -> R,
    ) -> Paned<R, A> {
        let (footer, sink) = self.into_footer();
        let body = footer.pane(ui, body, add);
        Paned {
            body,
            pressed: sink.0.borrow_mut().take(),
        }
    }

    /// `pane` for a body whose acts start or end the footer's lines (a form whose fields, Save
    /// and Discard change what its footer says; `Footer::pane_then`): `add` draws the body; then
    /// `then` gets what it returned and these messages, after the body and before the footer
    /// paints, and adds the lines from the state the body's acts left. So the frame of an act
    /// shows the line it brought and never one it ended, also when egui draws that frame in one
    /// pass. Lines added before stay. What `then` returns, and the press.
    pub fn pane_then<R, T>(
        self,
        ui: &mut Ui,
        body: super::pane::Body,
        add: impl FnOnce(&mut Ui) -> R,
        then: impl FnOnce(R, &mut Self) -> T,
    ) -> Paned<T, A> {
        let (id, width) = (self.id, self.width);
        let mut messages = self;
        let mut sink = None;
        let empty: TypedFooter<A> = TypedFooter {
            lines: Vec::new(),
            sink: PressSink(std::rc::Rc::default()),
            retained: None,
            plan: None,
        };
        let footer = Footer::typed(id, width, Box::new(empty));
        let body = footer.pane_then(ui, body, add, |inner, footer| {
            let out = then(inner, &mut messages);
            let (lines, press) = messages.into_footer();
            footer.take_typed(lines);
            sink = Some(press);
            out
        });
        Paned {
            body,
            pressed: sink.and_then(|sink| sink.0.borrow_mut().take()),
        }
    }

    /// `pane_then` with a `Body::Content` scroll column that behaves as `scroll` says
    /// (`Footer::pane_content_then`).
    pub fn pane_content_then<R, T>(
        self,
        ui: &mut Ui,
        scroll: super::pane::ContentScroll,
        add: impl FnOnce(&mut Ui) -> R,
        then: impl FnOnce(R, &mut Self) -> T,
    ) -> Paned<T, A> {
        let (id, width) = (self.id, self.width);
        let mut messages = self;
        let mut sink = None;
        let empty: TypedFooter<A> = TypedFooter {
            lines: Vec::new(),
            sink: PressSink(std::rc::Rc::default()),
            retained: None,
            plan: None,
        };
        let footer = Footer::typed(id, width, Box::new(empty));
        let body = footer.pane_content_then(ui, scroll, add, |inner, footer| {
            let out = then(inner, &mut messages);
            let (lines, press) = messages.into_footer();
            footer.take_typed(lines);
            sink = Some(press);
            out
        });
        Paned {
            body,
            pressed: sink.and_then(|sink| sink.0.borrow_mut().take()),
        }
    }

    /// `pane` with a `Body::Content` scroll column that behaves as `scroll` says (opens at the
    /// top, holds a pressed control in place: `pane::ContentScroll`).
    pub fn pane_content<R>(
        self,
        ui: &mut Ui,
        scroll: super::pane::ContentScroll,
        add: impl FnOnce(&mut Ui) -> R,
    ) -> Paned<R, A> {
        let (footer, sink) = self.into_footer();
        let body = footer.pane_content(ui, scroll, add);
        Paned {
            body,
            pressed: sink.0.borrow_mut().take(),
        }
    }

    /// `pane` with a `Body::Content` body that holds one editor which grows with its text
    /// (`Footer::pane_editor`, `pane::EditorRegion`).
    pub fn pane_editor<R>(
        self,
        ui: &mut Ui,
        add: impl for<'r> FnOnce(&mut Ui, super::pane::EditorRegion<'r>) -> R,
    ) -> Paned<R, A> {
        let (footer, sink) = self.into_footer();
        let body = footer.pane_editor(ui, add);
        Paned {
            body,
            pressed: sink.0.borrow_mut().take(),
        }
    }
}

/// A [`Messages`] footer as the pane draws it (layout PR 2b, ruling L9): its typed lines, and the
/// footer's retained plan. The plan holds the footer's order, its accessible name, which Undo is
/// the only one of its verb, each kept Undo's words as fitted to its room, the room its lines take
/// and the room each open lock's reason keeps, at one width, scale and style, for one revision of
/// the lines (their data, and the state their file lines draw from). It is made by the pass that
/// draws the lines (the paint is its measure), and only when one of those changed: an unchanged
/// frame sorts, joins, counts verbs, builds and fits no Undo label and lays out no reservation,
/// and the pane knows an unchanged footer's height before it draws. A changed footer's plan is
/// made before the pane places it, by drawing its lines in a context of their own (`measure`,
/// L1c): so the pane knows every footer's height before it draws, and draws its lines once, in
/// their place, from the plan. Each footer's reservations are its own (no shared state): a pane
/// that gives up its reasons' room takes nothing from another. The plans live in
/// one store per context, which drops the plan of a footer that no frame drew since the last one
/// ([`FooterPlans`]).
struct TypedFooter<A: 'static> {
    lines: Vec<FooterLine<A>>,
    sink: PressSink<A>,
    /// The lines' revision, worked out once this pass (`planned`).
    retained: Option<Retained>,
    /// This frame's plan, when the retained one was made from these lines here (`planned`).
    plan: Option<std::sync::Arc<FooterPlan>>,
}

/// What a footer's plan was made from, besides its lines: their revision, the footer's width,
/// the scale and the style (compared by value: a fixed size, whatever the lines hold). A
/// different one makes the plan again.
#[derive(Clone, PartialEq)]
struct PlanEnv {
    lines: super::memo::Revision,
    width: u32,
    scale: u32,
    style: std::sync::Arc<egui::Style>,
    /// The fonts in effect (`font_key`): new fonts, or a family's font replaced, plan again.
    fonts: u64,
    /// The font atlas's epoch (`theme::atlas_epoch`): the plan keeps laid words (its Undos' and
    /// choices'), whose galleys hold places in the atlas, so a new atlas plans again.
    atlas: u64,
    /// The footer followed its content in the last pass (`pane::footer_follows`): an answer to a
    /// pressed line stands under it (`Messages::answer`).
    follows: bool,
}

/// One line as the plan sees it: what decides its place and its size. A press's id is not in
/// it: it changes neither. `file_gone`: its file line says that its file is gone (a link was
/// pressed while the file was not there), which adds a reason line under it.
#[derive(Clone, PartialEq)]
struct PlanLine {
    rank: FooterRank,
    order: Option<ResultOrder>,
    ticket: Option<ResultOrder>,
    under: Option<ResultOrder>,
    body: Body,
    file_gone: bool,
}

impl PlanLine {
    fn same<A>(&self, line: &FooterLine<A>, file_gone: bool) -> bool {
        self.rank == line.rank
            && self.order == line.order
            && self.ticket == line.ticket.as_ref().map(Ticket::order)
            && self.under == line.under
            && self.file_gone == file_gone
            && self.body == line.typed.body
    }
}

/// Whether `body`'s file line says now that its file is gone (`FileLine::draw`'s state).
fn file_gone(ctx: &egui::Context, body: &Body) -> bool {
    body.file
        .as_ref()
        .and_then(FileLine::path)
        .is_some_and(|path| {
            ctx.data(|data| data.get_temp::<bool>(egui::Id::new(("design_file_gone", path))))
                == Some(true)
        })
}

/// A footer's retained plan. See [`TypedFooter`].
#[derive(Clone)]
struct FooterPlan {
    env: PlanEnv,
    /// The lines' indices in the footer's one order (`footer_order`).
    order: Vec<usize>,
    /// The footer's accessible name: every line's words, in that order.
    name: String,
    /// For each line, whether its Undo is the only kept Undo of its verb (`undo_alone`).
    alone: Vec<bool>,
    /// For each line, its Undo's words as fitted to the line's room (`Body::undo_form`).
    forms: std::sync::Arc<[Option<std::sync::Arc<UndoForm>>]>,
    /// For each line, its choices' widths and laid words (`Body::plan_choices`): a pass that does
    /// not measure measures no choice and lays out no action's words.
    choices: std::sync::Arc<[Option<std::sync::Arc<[ChoiceLaid]>>]>,
    /// The room the lines take, without their open locks' reasons.
    bare: f32,
    /// The room the open locks' reasons want (ruling R29).
    reserved: f32,
    /// The room each open lock's reason keeps, in drawing order: a frame that keeps them takes
    /// this room and lays out no reason.
    rooms: std::sync::Arc<[f32]>,
}

impl FooterPlan {
    /// The footer's whole demand: what its lines want, uncapped.
    fn wanted(&self) -> f32 {
        (self.bare + self.reserved + 2.0 * super::components::PART_INSET).ceil()
    }
}

/// A footer's lines as last planned, their revision, its plan, and the frame that last drew it.
#[derive(Clone)]
struct Retained {
    lines: std::sync::Arc<Vec<PlanLine>>,
    revision: super::memo::Revision,
    plan: Option<std::sync::Arc<FooterPlan>>,
    drawn: u64,
}

/// Every typed footer's retained data in one context, by footer id. A footer that no frame drew
/// since the last one has ended (its pane closed, its owner or campaign changed): its data is
/// dropped by the store's sweep, once per frame, when the first footer draws. So the store holds
/// the footers of about two frames, whatever was shown before.
#[derive(Clone, Default)]
struct FooterPlans {
    plans: std::collections::HashMap<egui::Id, Retained>,
    /// The frame of the last sweep.
    swept: Option<u64>,
}

impl FooterPlans {
    /// Once per frame: drop each footer that no frame drew since the last one. It visits each
    /// kept footer once (counted), and its spare room is kept within a few times the footers it
    /// holds, so the scan is as large as the footers in view, not as many as were ever shown.
    fn sweep(&mut self, frame: u64) {
        if self.swept == Some(frame) {
            return;
        }
        self.swept = Some(frame);
        count(|work| work.swept += self.plans.len());
        self.plans.retain(|_, kept| kept.drawn + 1 >= frame);
        if self.plans.capacity() > 4 * self.plans.len().max(8) {
            self.plans.shrink_to_fit();
        }
    }
}

fn plans_id() -> egui::Id {
    egui::Id::new("design_typed_footer_plans")
}

/// The number of footers whose plans one context keeps, for a test of their disposal.
#[cfg(test)]
pub(crate) fn kept_footer_plans(ctx: &egui::Context) -> usize {
    ctx.data_mut(|data| {
        data.get_temp_mut_or_default::<FooterPlans>(plans_id())
            .plans
            .len()
    })
}

/// The work of typed footers on this thread, for tests: counted inside the work.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FooterWork {
    /// Plans made: one sort, one name, one measure each.
    pub plans: usize,
    /// Lines sorted into an order.
    pub sorted: usize,
    /// Bytes of words joined into a footer's name.
    pub named: usize,
    /// Reservations measured in a hidden child.
    pub probes: usize,
    /// Reservations laid out (their reason drawn invisibly to take its room).
    pub laid: usize,
    /// Undo verbs compared to find each line's `undo_alone`.
    pub verbs: usize,
    /// Undo labels built and fitted to their room (`Body::undo_form`).
    pub undo_fits: usize,
    /// Footers visited by the plan store's sweep (`FooterPlans::sweep`).
    pub swept: usize,
    /// Lines compared with the retained ones, and the bytes of their words.
    pub compared: usize,
    pub compared_bytes: usize,
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    pub(crate) static FOOTER_WORK: std::cell::Cell<FooterWork> = const {
        std::cell::Cell::new(FooterWork {
            plans: 0,
            sorted: 0,
            named: 0,
            probes: 0,
            laid: 0,
            verbs: 0,
            undo_fits: 0,
            swept: 0,
            compared: 0,
            compared_bytes: 0,
        })
    };
}

/// Count typed-footer work, in a test build.
#[cfg(any(test, feature = "test-support"))]
fn count(add: impl FnOnce(&mut FooterWork)) {
    FOOTER_WORK.with(|work| {
        let mut now = work.get();
        add(&mut now);
        work.set(now);
    });
}

#[cfg(not(any(test, feature = "test-support")))]
fn count(_: impl FnOnce(&mut FooterWork)) {}

/// This thread's typed-footer work so far.
#[cfg(any(test, feature = "test-support"))]
pub fn footer_work() -> FooterWork {
    FOOTER_WORK.with(std::cell::Cell::get)
}

impl<A: 'static> TypedFooter<A> {
    /// The footer's lines against the retained ones: the same revision while they are the same
    /// (a compare as large as their data: no sort, no measure), else a new one. It also marks
    /// the footer drawn in this frame, and the frame's first footer sweeps the store.
    fn revision(&self, ctx: &egui::Context, id: egui::Id) -> Retained {
        let frame = ctx.cumulative_frame_nr();
        let gone: Vec<bool> = self
            .lines
            .iter()
            .map(|line| file_gone(ctx, &line.typed.body))
            .collect();
        ctx.data_mut(|data| {
            let plans = data.get_temp_mut_or_default::<FooterPlans>(plans_id());
            plans.sweep(frame);
            if let Some(kept) = plans.plans.get_mut(&id) {
                kept.drawn = frame;
                let same = kept.lines.len() == self.lines.len()
                    && kept.lines.iter().zip(&self.lines).zip(&gone).all(
                        |((kept, line), &gone)| {
                            count(|work| {
                                work.compared += 1;
                                work.compared_bytes += line.typed.body.words.as_str().len();
                            });
                            kept.same(line, gone)
                        },
                    );
                if same {
                    return kept.clone();
                }
            }
            let retained = Retained {
                lines: std::sync::Arc::new(
                    self.lines
                        .iter()
                        .zip(&gone)
                        .map(|(line, &file_gone)| PlanLine {
                            rank: line.rank,
                            order: line.order,
                            ticket: line.ticket.as_ref().map(Ticket::order),
                            under: line.under,
                            body: line.typed.body.clone(),
                            file_gone,
                        })
                        .collect(),
                ),
                revision: super::memo::Revision::next(),
                plan: None,
                drawn: frame,
            };
            plans.plans.insert(id, retained.clone());
            retained
        })
    }

    fn env(ui: &Ui, id: egui::Id, lines: super::memo::Revision, width: f32) -> PlanEnv {
        PlanEnv {
            lines,
            width: width.to_bits(),
            scale: ui.ctx().pixels_per_point().to_bits(),
            style: ui.style().clone(),
            fonts: font_key(ui.ctx()),
            atlas: super::theme::atlas_epoch(ui.ctx()),
            follows: super::pane::footer_follows(ui.ctx(), id),
        }
    }

    /// Draw line `at`, with its Undo's words and its reasons' room as the plan decides; its press
    /// goes to the pane's one slot, with a kept line's ticket.
    fn draw_line(
        &self,
        ui: &mut Ui,
        at: usize,
        line: FooterLine<A>,
        alone: bool,
        room: &mut FooterPass,
    ) {
        let FooterLine {
            mut typed, ticket, ..
        } = line;
        typed.body.undo_alone = alone;
        room.line = at;
        let pressed = match &ticket {
            // A global id from the ticket: egui's child ids count the children before them, so
            // a line added above would hand this line's focus and press to another.
            Some(ticket) => {
                // The pane's footer (this `Ui`) and the ticket: one stored entry shown in two
                // panes has two lines with their own ids.
                let pane = ui
                    .ctx()
                    .data(|data| data.get_temp::<egui::Id>(egui::Id::new("design_footer_drawing")));
                let id = egui::Id::new(("design_kept_line", pane, ticket.order()));
                line_ui(ui, Some(id), |ui| {
                    typed.body.draw_in(
                        ui,
                        MessagePlace::Footer,
                        Reasons::Planned(room),
                        IconRoom::Measured,
                    )
                })
            }
            None => line_ui(ui, None, |ui| {
                typed.body.draw_in(
                    ui,
                    MessagePlace::Footer,
                    Reasons::Planned(room),
                    IconRoom::Measured,
                )
            }),
        };
        if let Some(pressed) = pressed
            && let Some(id) = typed.id(pressed)
        {
            self.sink.press(Press { id, ticket });
        }
    }
}

impl<A: 'static> TypedFooter<A> {
    /// The footer's plan at `width`: the retained one while it holds, else one made now
    /// (`measure`) and kept.
    fn plan_for(&mut self, ui: &Ui, id: egui::Id, width: f32) -> std::sync::Arc<FooterPlan> {
        let retained = match self.retained.take() {
            Some(retained) => retained,
            None => self.revision(ui.ctx(), id),
        };
        let env = Self::env(ui, id, retained.revision, width);
        if let Some(plan) = retained.plan.as_ref().filter(|plan| plan.env == env) {
            let plan = plan.clone();
            self.retained = Some(retained);
            return plan;
        }
        let plan = std::sync::Arc::new(self.measure(ui, id, width, env));
        let retained = Retained {
            plan: Some(plan.clone()),
            ..retained
        };
        ui.ctx().data_mut(|data| {
            data.get_temp_mut_or_default::<FooterPlans>(plans_id())
                .plans
                .insert(id, retained.clone());
        });
        self.retained = Some(retained);
        plan
    }

    /// Make the plan of these lines at `width`: their one footer order (`footer_order`), the
    /// footer's name, which Undo is alone of its verb, each Undo's words as fitted, and the room
    /// the lines and their open locks' reasons take, by drawing them as the footer draws them,
    /// in a context of their own with this one's fonts, style and scale (`measuring`). L1c,
    /// finding 2 of PR 452's review: so a changed footer's demand is known before the pane
    /// places it, and its lines are drawn once in this context, in their place. A second draw
    /// here would give each control a second widget and accessible node.
    fn measure(&self, ui: &Ui, id: egui::Id, width: f32, env: PlanEnv) -> FooterPlan {
        let lines = &self.lines;
        let mut order: Vec<usize> = (0..lines.len()).collect();
        order.sort_by_key(|&at| (lines[at].rank, lines[at].order.map(std::cmp::Reverse)));
        // An answer to a press of a kept line, in a footer that follows its content, stands
        // directly under that line, so the pressed line keeps its place (`Messages::answer`).
        if env.follows
            && let Some(answer) = order.iter().position(|&at| lines[at].under.is_some())
            && let Some(pressed) = lines[order[answer]].under
            && let Some(line) = order
                .iter()
                .position(|&at| lines[at].ticket.as_ref().map(Ticket::order) == Some(pressed))
        {
            let moved = order.remove(answer);
            let after = if line > answer { line } else { line + 1 };
            order.insert(after, moved);
        }
        count(|work| {
            work.plans += 1;
            work.sorted += order.len();
        });
        let name = order
            .iter()
            .map(|&at| lines[at].typed.body.words.as_str())
            .filter(|words| !words.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        count(|work| work.named += name.len());
        // The footer's one short-Undo rule: the verbs of its kept Undos, counted once.
        let verbs: Vec<Verb> = lines
            .iter()
            .filter(|line| line.ticket.is_some())
            .filter_map(|line| line.typed.body.undo.as_ref().and_then(UndoStep::counted))
            .collect();
        let alone: Vec<bool> = lines
            .iter()
            .map(|line| {
                count(|work| work.verbs += verbs.len());
                undo_alone(&verbs, line.typed.body.undo.as_ref())
            })
            .collect();
        // Every open lock's reason is laid out and measured (ruling R29): a pass decides from the
        // plan and the footer's room whether they keep it.
        let mut room = FooterPass {
            give_up: false,
            measure: true,
            reserved: 0.0,
            drawn: 0.0,
            rooms: std::sync::Arc::from([]),
            next: 0,
            measured: Vec::new(),
            line: 0,
            forms: std::sync::Arc::from([]),
            fitted: vec![None; lines.len()],
            choices: std::sync::Arc::from([]),
            laid_choices: vec![None; lines.len()],
        };
        // The state the file lines draw from (a file that is gone adds a reason line).
        let gone = file_states(ui.ctx(), lines);
        let content = measuring(ui, &gone, |ui| {
            let rect = Rect::from_min_size(ui.max_rect().min, egui::vec2(width, MEASURE_ROOM));
            let (_, content, _) =
                super::components::scroll_part(ui, rect, id, &name, false, |ui| {
                    for &at in &order {
                        let line = &lines[at];
                        let mut body = line.typed.body.clone();
                        body.undo_alone = alone[at];
                        room.line = at;
                        let scope = line
                            .ticket
                            .as_ref()
                            .map(|ticket| egui::Id::new(("design_kept_line", id, ticket.order())));
                        let _ = line_ui(ui, scope, |ui| {
                            body.draw_in(
                                ui,
                                MessagePlace::Footer,
                                Reasons::Planned(&mut room),
                                IconRoom::Measured,
                            )
                        });
                    }
                });
            content
        });
        FooterPlan {
            env,
            order,
            name,
            alone,
            forms: std::sync::Arc::from(std::mem::take(&mut room.fitted)),
            choices: std::sync::Arc::from(std::mem::take(&mut room.laid_choices)),
            bare: content - room.drawn,
            reserved: room.reserved,
            rooms: std::sync::Arc::from(std::mem::take(&mut room.measured)),
        }
    }
}

/// The state each of `lines`' file lines draws from: its key, and whether it says its file is
/// gone (`FileLine::draw`).
fn file_states<A>(ctx: &egui::Context, lines: &[FooterLine<A>]) -> Vec<(egui::Id, bool)> {
    lines
        .iter()
        .filter_map(|line| line.typed.body.file.as_ref().and_then(FileLine::path))
        .map(|path| {
            let key = egui::Id::new(("design_file_gone", path));
            (
                key,
                ctx.data(|data| data.get_temp::<bool>(key)) == Some(true),
            )
        })
        .collect()
}

/// The state of the same file lines (`file_states`), read again.
fn file_states_of(ctx: &egui::Context, files: &[(egui::Id, bool)]) -> Vec<(egui::Id, bool)> {
    files
        .iter()
        .map(|&(key, _)| {
            (
                key,
                ctx.data(|data| data.get_temp::<bool>(key)) == Some(true),
            )
        })
        .collect()
}

/// A footer line drawn in a scope of its own id (a kept line: its controls' ids come from its
/// ticket), or in `ui`.
fn line_ui<R>(ui: &mut Ui, scope: Option<egui::Id>, add: impl FnOnce(&mut Ui) -> R) -> R {
    match scope {
        Some(id) => ui.scope_builder(egui::UiBuilder::new().id(id), add).inner,
        None => add(ui),
    }
}

/// The height a footer's lines are measured in: more than any footer takes.
const MEASURE_ROOM: f32 = 100_000.0;

/// The fonts in effect in `ctx`, as one key: each family's fonts, and each font's name and data
/// (by its shared data, so new fonts under an old name count). Any `set_fonts` that changes what
/// a family draws with changes it, not only the theme's. Worked out once a pass.
pub fn font_key(ctx: &egui::Context) -> u64 {
    use std::hash::{Hash, Hasher};
    let pass = ctx.cumulative_pass_nr();
    let id = egui::Id::new("design_footer_font_key");
    if let Some((kept, key)) = ctx.data(|data| data.get_temp::<(u64, u64)>(id))
        && kept == pass
    {
        return key;
    }
    let key = ctx.fonts(|view| {
        let fonts = view.definitions();
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        fonts.families.hash(&mut hasher);
        for (name, data) in &fonts.font_data {
            name.hash(&mut hasher);
            std::sync::Arc::as_ptr(data).hash(&mut hasher);
        }
        hasher.finish()
    });
    ctx.data_mut(|data| data.insert_temp(id, (pass, key)));
    key
}

/// The context a footer's lines are measured in (`TypedFooter::measure`), kept in this one's
/// memory, and the fonts it has (`font_key`).
#[derive(Clone)]
struct MeasureContext {
    ctx: egui::Context,
    fonts: u64,
}

/// One pass of the measuring context: `add` gets a `Ui` there with `ui`'s style and this
/// context's fonts and scale, and the file lines' state `gone`. Nothing it draws reaches this
/// context: no widget, no accessible node, no focus and no paint. Its root `Ui` is the one
/// `egui::Ui::new` in the design system: that context has no page, so no host region.
#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
fn measuring<R>(ui: &Ui, gone: &[(egui::Id, bool)], mut add: impl FnMut(&mut Ui) -> R) -> R {
    let main = ui.ctx();
    let key = egui::Id::new("design_footer_measure");
    // The measuring context has the fonts in effect here, whoever set them (`font_key`).
    let fonts = font_key(main);
    let kept = main.data(|data| data.get_temp::<MeasureContext>(key));
    let measure = match kept {
        Some(kept) if kept.fonts == fonts => kept.ctx,
        kept => {
            let ctx = kept.map(|kept| kept.ctx).unwrap_or_default();
            ctx.options_mut(|options| options.max_passes = std::num::NonZeroUsize::MIN);
            ctx.set_fonts(main.fonts(|view| view.definitions().clone()));
            main.data_mut(|data| {
                data.insert_temp(
                    key,
                    MeasureContext {
                        ctx: ctx.clone(),
                        fonts,
                    },
                );
            });
            ctx
        }
    };
    if measure.zoom_factor() != main.zoom_factor() {
        measure.set_zoom_factor(main.zoom_factor());
    }
    measure.data_mut(|data| {
        for &(key, gone) in gone {
            if gone {
                data.insert_temp(key, true);
            } else {
                data.remove::<bool>(key);
            }
        }
    });
    let screen = Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(MEASURE_ROOM, MEASURE_ROOM));
    let mut input = egui::RawInput {
        screen_rect: Some(screen),
        ..Default::default()
    };
    input.viewports.insert(
        egui::ViewportId::ROOT,
        egui::ViewportInfo {
            native_pixels_per_point: main.native_pixels_per_point(),
            ..Default::default()
        },
    );
    let style = ui.style().clone();
    let mut out = None;
    let _ = measure.run(input, |ctx| {
        let mut ui = Ui::new(
            ctx.clone(),
            egui::Id::new("design_footer_measure"),
            egui::UiBuilder::new().max_rect(screen).style(style.clone()),
        );
        out = Some(add(&mut ui));
    });
    out.expect("the measuring context draws one pass")
}

impl<A: 'static> super::components::FooterLines for TypedFooter<A> {
    fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    fn planned(&mut self, ui: &Ui, id: egui::Id, width: f32) -> Option<f32> {
        if self.lines.is_empty() {
            // No footer: nothing to measure (the store still sees it drawn this frame).
            self.retained = Some(self.revision(ui.ctx(), id));
            return None;
        }
        let plan = self.plan_for(ui, id, width);
        let wanted = plan.wanted();
        self.plan = Some(plan);
        Some(wanted)
    }

    fn draw(
        &mut self,
        ui: &mut Ui,
        id: egui::Id,
        rect: Rect,
        cut: bool,
        most: f32,
        stored: f32,
        fit: bool,
    ) -> (Rect, f32) {
        use super::tokens::HAIRLINE_WIDTH;
        if cut && rect.height() > 0.0 {
            // The cut body's last line reads as a list that goes on, not as a drawing fault.
            ui.painter().hline(
                rect.x_range(),
                rect.top() + HAIRLINE_WIDTH / 2.0,
                egui::Stroke::new(HAIRLINE_WIDTH, super::tokens::colors().hairline),
            );
        }
        // The plan: the pane's (`planned`), else one made now for a footer drawn by hand.
        let (plan, made) = match self.plan.take() {
            Some(plan) => (plan, false),
            None => (self.plan_for(ui, id, rect.width()), true),
        };
        let files = file_states(ui.ctx(), &self.lines);
        let mut lines: Vec<Option<FooterLine<A>>> = std::mem::take(&mut self.lines)
            .into_iter()
            .map(Some)
            .collect();
        // Its order, its name, its Undos' forms and its room. Ruling R29: the open locks'
        // reasons keep their room while the footer has room for it where it stands; each takes
        // the room the plan measured, with no reason laid out.
        let mut room = FooterPass {
            give_up: plan.reserved > 0.0 && plan.wanted() > most,
            measure: false,
            reserved: 0.0,
            drawn: 0.0,
            rooms: plan.rooms.clone(),
            next: 0,
            measured: Vec::new(),
            line: 0,
            forms: plan.forms.clone(),
            fitted: Vec::new(),
            choices: plan.choices.clone(),
            laid_choices: Vec::new(),
        };
        let (_, content, part) =
            super::components::scroll_part(ui, rect, id, &plan.name, fit, |ui| {
                for &at in &plan.order {
                    if let Some(line) = lines[at].take() {
                        self.draw_line(ui, at, line, plan.alone[at], &mut room);
                    }
                }
            });
        // The plan was measured in a context of its own (`measure`): drawn here, the lines take
        // the room it says, less the reasons' room they keep. A press in this draw can change
        // what a line shows (a link to a file that is gone adds its reason; the next frame plans
        // the new lines). Any other difference is a fault of that measure: test builds stop at
        // it. Either way the plan takes this paint's room, and the pass is drawn again with it.
        let bare = content - plan.rooms.iter().take(room.next).sum::<f32>();
        let plan = if (bare - plan.bare).abs() >= HAIRLINE_WIDTH {
            debug_assert!(
                files != file_states_of(ui.ctx(), &files),
                "a footer's lines measure {} and paint {bare}",
                plan.bare
            );
            let plan = std::sync::Arc::new(FooterPlan {
                bare,
                ..(*plan).clone()
            });
            ui.ctx().data_mut(|data| {
                if let Some(kept) = data
                    .get_temp_mut_or_default::<FooterPlans>(plans_id())
                    .plans
                    .get_mut(&id)
                {
                    kept.plan = Some(plan.clone());
                }
            });
            ui.ctx()
                .request_discard("a footer's lines paint another room than they measured");
            plan
        } else {
            plan
        };
        let wanted = plan.wanted();
        if made && (wanted - stored).abs() >= HAIRLINE_WIDTH {
            ui.ctx()
                .data_mut(|data| data.insert_temp(id.with("height"), wanted));
            ui.ctx()
                .request_discard("the message footer changed height");
        }
        (part, wanted)
    }
}

/// A save row's own messages (ruling R24): the fault of its act, the warning before its act and
/// its kept result, drawn by `value_block::ValueBlock::messages` `S` under that save row, in a
/// footer's order (results with a live Undo, reasons, notices, then other results; results newest
/// first). The typed `Row` place: they belong to the row above them, so they may move what
/// follows them, never the row or anything above it. Entries are owned and typed as a
/// [`Messages`] footer's are, and a kept line still borrows this frame's [`PaneFrame`].
/// One entry of [`RowMessages`]: its rank and order in the footer's order, what it draws, and
/// a kept result's ticket.
type RowEntry<A> = (
    FooterRank,
    Option<ResultOrder>,
    Typed<A>,
    Option<Ticket>,
    // Whether it is a refused attempt: its place waits for every line (`refusal_rank`).
    bool,
);

#[must_use = "give it to `ValueBlock::messages`"]
pub struct RowMessages<'f, A: 'static> {
    items: Vec<RowEntry<A>>,
    frame: std::marker::PhantomData<&'f PaneFrame>,
}

impl<'f, A: 'static> RowMessages<'f, A> {
    pub fn new(frame: &'f PaneFrame) -> Self {
        let _ = frame;
        Self {
            items: Vec::new(),
            frame: std::marker::PhantomData,
        }
    }

    /// A reason or a notice, at the place its variant gives it.
    pub fn outcome(&mut self, outcome: impl Into<Outcome<A>>) -> &mut Self {
        let (rank, typed) = outcome.into().parts();
        self.items.push((rank, None, typed, None, false));
        self
    }

    /// A kept result of this frame, newest first by the order its store issued.
    pub fn kept(&mut self, line: KeptLine<'f, A>) -> &mut Self {
        let order = line.refused.unwrap_or(line.ticket.order());
        let rank = kept_place(line.typed.body.undo.is_some());
        let refused = line.refused.is_some();
        self.items
            .push((rank, Some(order), line.typed, Some(line.ticket), refused));
        self
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Draw them at the cursor, top down; the first press, as the caller's id.
    pub(in crate::design) fn show(mut self, ui: &mut Ui) -> Option<Press<A>> {
        let newest = self.items.iter().filter_map(|item| item.1).max();
        for (rank, order, _, _, refused) in &mut self.items {
            if let (true, Some(attempt)) = (*refused, *order) {
                *rank = refusal_rank(attempt, newest);
            }
        }
        footer_order(&mut self.items);
        let mut first = None;
        let kept: Vec<Verb> = self
            .items
            .iter()
            .filter(|(_, _, _, ticket, _)| ticket.is_some())
            .filter_map(|(_, _, typed, _, _)| typed.body.undo.as_ref().and_then(UndoStep::counted))
            .collect();
        for (_, _, mut typed, ticket, _) in self.items {
            typed.body.undo_alone = undo_alone(&kept, typed.body.undo.as_ref());
            let pressed = match &ticket {
                // A kept line's ids come from its ticket, as in a footer.
                Some(ticket) => {
                    let id = egui::Id::new(("design_row_kept_line", ticket.order()));
                    ui.scope_builder(egui::UiBuilder::new().id(id), |ui| {
                        typed.body.draw(ui, MessagePlace::Row)
                    })
                    .inner
                }
                None => typed.body.draw(ui, MessagePlace::Row),
            };
            if first.is_none()
                && let Some(pressed) = pressed
                && let Some(id) = typed.id(pressed)
            {
                first = Some(Press { id, ticket });
            }
        }
        first
    }
}

/// N6, the one placement of a message's close icon: `words` draws the message in the room left of
/// the icon (capped at `LIST_MEASURE`) and gives back the y of the centre of its first line; the
/// icon goes at the right end of that line, centred on it. A locked icon is disabled, with its
/// reason. `Body::draw` (a footer's `MessageChoice::Dismiss`) and [`dismiss_on_first_line`] use it.
fn dismiss_at_first_line(
    ui: &mut Ui,
    lock: Option<&str>,
    room: IconRoom,
    words: impl FnOnce(&mut Ui) -> f32,
) -> Response {
    let size = Density::Read.control();
    ui.scope(|ui| {
        ui.set_max_width(ui.available_width().min(LIST_MEASURE));
        let right = ui.max_rect().right();
        let line = ui
            .scope(|ui| {
                ui.set_max_width(ui.available_width() - size - S);
                words(ui)
            })
            .inner;
        let centre = egui::pos2(right - size / 2.0, line);
        let place = Rect::from_center_size(centre, egui::Vec2::splat(size));
        let icon = |ui: &mut Ui| match lock {
            Some(reason) => ui
                .add_enabled_ui(false, |ui| icon_button(ui, icons::CLOSE, DISMISS_NAME))
                .inner
                .on_disabled_hover_text(reason),
            None => icon_button(ui, icons::CLOSE, DISMISS_NAME),
        };
        match room {
            IconRoom::Measured => ui.put(place, icon),
            IconRoom::Overhang => icon(&mut ui.new_child(
                egui::UiBuilder::new().max_rect(place).layout(
                    egui::Layout::centered_and_justified(egui::Direction::TopDown),
                ),
            )),
        }
    })
    .inner
}

/// Whether the close icon's target counts in the message's measured content.
#[derive(Clone, Copy)]
enum IconRoom {
    /// A footer's message: the target is part of the message's rect (unchanged since N6).
    Measured,
    /// A message in a banner with padding: the target, taller than the first line, reaches into
    /// the padding; the message's height stays its words'.
    Overhang,
}

/// A message drawn outside a pane's footer (the session's bottom banner) with its close icon, as
/// a footer's `MessageChoice::Dismiss` places it: at the right end of the words' first line,
/// centred on it (N6). `words` draws the message from the top of its room. The icon's
/// accessible name is [`DISMISS_NAME`]. Returns the icon's response.
pub fn dismiss_on_first_line(ui: &mut Ui, words: impl FnOnce(&mut Ui)) -> Response {
    dismiss_at_first_line(ui, None, IconRoom::Overhang, |ui| {
        let line = ui.cursor().top() + Type::Body.line() / 2.0;
        words(ui);
        line
    })
}

/// Where a message's open locks' reasons take their room (ruling R29), and where its Undo's
/// fitted words come from.
pub(in crate::design) enum Reasons<'r> {
    /// Every place outside a typed footer (a banner, a row, a value block): an open lock's
    /// reason keeps its room, nothing is measured and nothing is counted.
    Body,
    /// A typed footer's own room and Undo words, which its plan decides (layout PR 2b): no
    /// shared state.
    Planned(&'r mut FooterPass),
}

/// One typed footer in one pass: whether its reasons give up their room (its plan's decision),
/// whether this pass measures (makes the plan), the sums it measured, and its lines' Undo words.
pub(in crate::design) struct FooterPass {
    give_up: bool,
    measure: bool,
    reserved: f32,
    drawn: f32,
    /// The plan's room for each open lock's reason, in drawing order, and the next one: a pass
    /// that does not measure takes this room and lays out no reason.
    rooms: std::sync::Arc<[f32]>,
    next: usize,
    /// What a measuring pass found for each, in drawing order: also for a reason whose room
    /// that pass gives up, so a pass after the pane grows takes it.
    measured: Vec<f32>,
    /// The footer line drawn now (its index in the footer's lines).
    line: usize,
    /// The plan's Undo words for each line: a pass that does not measure builds and fits none.
    forms: std::sync::Arc<[Option<std::sync::Arc<UndoForm>>]>,
    /// What a measuring pass fitted for each line.
    fitted: Vec<Option<std::sync::Arc<UndoForm>>>,
    /// The plan's choices for each line (`FooterPlan::choices`).
    choices: std::sync::Arc<[Option<std::sync::Arc<[ChoiceLaid]>>]>,
    /// What a measuring pass laid for each line's choices.
    laid_choices: Vec<Option<std::sync::Arc<[ChoiceLaid]>>>,
}

impl Body {
    /// With a `Dismiss`, the close icon at the right end of the first line, centred on that
    /// line; the words and the other choices keep the room to its left. A locked `Dismiss` is
    /// disabled, with its reason, as every blocked action is.
    fn draw(&self, ui: &mut Ui, place: MessagePlace) -> Option<Pressed> {
        self.draw_in(ui, place, Reasons::Body, IconRoom::Measured)
    }

    /// `draw`, with its reasons' room as `reasons` says, and the close icon's target in the
    /// message's rect (`Measured`) or reaching into the padding around it (`Overhang`).
    fn draw_in(
        &self,
        ui: &mut Ui,
        place: MessagePlace,
        mut reasons: Reasons<'_>,
        icon_room: IconRoom,
    ) -> Option<Pressed> {
        let Some(at) = self
            .choices
            .iter()
            .position(|choice| matches!(choice, MessageChoice::Dismiss(_)))
        else {
            let mut pressed = None;
            self.draw_words(ui, place, &mut pressed, &mut reasons);
            return pressed;
        };
        let lock = self.locks.get(at).and_then(holding);
        let mut pressed = None;
        let icon = dismiss_at_first_line(ui, lock, icon_room, |ui| {
            self.draw_words(ui, place, &mut pressed, &mut reasons)
        });
        // The words and their controls are drawn before the close icon: they come first.
        let words = self.words.as_str();
        activated(ui, &mut pressed, icon.clicked(), words, Pressed::Choice(at));
        pressed
    }

    /// The compact form (`BodyMessage::compact`): the words as a row's message draws them
    /// (`small_tone_message`), the choices in one band after them on their line, as wide as the
    /// choices, and the file line after the words when it fits, else under them at the words.
    /// A fault with help draws, as the full form does, its words, its help lines, its file line,
    /// then its choices on a line of their own at the words.
    fn draw_compact(
        &self,
        ui: &mut Ui,
        place: MessagePlace,
        detail: Option<&str>,
    ) -> Option<Pressed> {
        // Review of PR 592: the compact form draws no choice's help line and no lock's reason
        // line, so a test or debug build refuses a choice with either: draw the full form.
        debug_assert!(
            self.helps.iter().all(Option::is_none) && self.locks.iter().all(Option::is_none),
            "a compact message draws no choice help and no lock reason: draw the full form"
        );
        let gap = ui.spacing().item_spacing.x;
        let words = self.words.as_str();
        let choices: Vec<(usize, MessageChoice<'_>)> =
            self.choices.iter().copied().enumerate().collect();
        // The choices' widths and words, laid once per change (`body_kept`).
        let kept = self.body_kept(ui, "body_choices", 0.0, || self.plan_choices(ui));
        // Review of PR 677: help comes before the file line and the choices, so a fault with
        // help keeps nothing beside its words.
        let helped = !self.help.is_empty();
        let band: f32 = choices
            .iter()
            .filter(|_| !helped)
            .map(|(at, choice)| match choice {
                MessageChoice::Dismiss(_) => Density::Read.control() + gap,
                MessageChoice::Link { word, .. } => compact_link_width(ui, word) + gap,
                choice => self.choice_width(ui, *at, *choice, kept.get(*at)) + gap,
            })
            .sum();
        let file = self.file.as_ref().map(|file| file.natural_width(ui));
        // The words' room on their line; the file line takes room after them only when the
        // words then still fit on one line.
        let (icon, _) = self.tone.parts();
        let inset =
            S + HAIRLINE_WIDTH + number_width(ui, icon, &Type::Small.font(ui)) + 2.0 * gap - XS;
        let one_line = inset + compact_words_width(ui, words);
        // The band follows the words one item gap after them (its sum holds the gaps).
        let room = ui.available_width() - band;
        let beside = file.filter(|file| !helped && one_line + L + file <= room);
        let mut pressed = None;
        let control = Density::Read.control();
        let text_room = (room - beside.map_or(0.0, |file| file + L)).max(control);
        // With a band, the row is one control high: one line of words sits on its centre,
        // wrapped words start at its top (as the close icon does).
        let pad = if band > 0.0 && one_line <= text_room {
            ((control - Type::Small.line()) / 2.0).max(0.0)
        } else {
            0.0
        };
        let row = ui.horizontal_top(|ui| {
            let message = ui
                .allocate_ui_with_layout(
                    egui::vec2(text_room, 0.0),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        // A row message's own gaps, whatever the caller's spacing.
                        ui.spacing_mut().item_spacing.x = S;
                        space(ui, pad);
                        small_tone_message(ui, self.tone, words)
                    },
                )
                .inner;
            if let Some(detail) = detail {
                super::components::named(ui, &message, detail);
                message.clone().on_hover_text(detail);
            }
            if let (Some(width), Some(file)) = (beside, &self.file) {
                space(ui, (L - gap).max(0.0));
                // A box of no height of its own: the file line centres itself in what it gets.
                ui.allocate_ui_with_layout(
                    egui::vec2(width, 0.0),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| file.draw(ui),
                );
            }
            if !helped {
                self.compact_choices(ui, &choices, &kept, &mut pressed);
            }
            message.rect.left()
        });
        // The help under the row, at the words (`FaultMessage::help`), a line each.
        for help in &self.help {
            let inset = row.inner - ui.cursor().left();
            ui.horizontal_top(|ui| {
                space(ui, inset);
                super::components::help(ui, help.as_str());
            });
        }
        if beside.is_none()
            && let Some(file) = &self.file
        {
            let inset = row.inner - ui.cursor().left();
            ui.horizontal_top(|ui| {
                space(ui, inset);
                let width = ui.available_width();
                ui.allocate_ui_with_layout(
                    egui::vec2(width, 0.0),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| file.draw(ui),
                );
            });
        }
        if helped && !choices.is_empty() {
            let inset = row.inner - ui.cursor().left();
            ui.horizontal(|ui| {
                space(ui, inset);
                self.compact_choices(ui, &choices, &kept, &mut pressed);
            });
        }
        register(ui, row.response.rect, place, self.tone);
        pressed
    }

    /// The compact form's choices in order, each as wide as it is and one control high: in the
    /// band after the words, or on their own line under a fault's help and file line.
    fn compact_choices(
        &self,
        ui: &mut Ui,
        choices: &[(usize, MessageChoice<'_>)],
        kept: &[ChoiceLaid],
        pressed: &mut Option<Pressed>,
    ) {
        let control = Density::Read.control();
        let words = self.words.as_str();
        for (at, choice) in choices {
            let clicked = match choice {
                MessageChoice::Dismiss(_) => {
                    let lock = self.locks.get(*at).and_then(holding);
                    match lock {
                        Some(reason) => {
                            ui.add_enabled_ui(false, |ui| {
                                icon_button(ui, icons::CLOSE, DISMISS_NAME)
                            })
                            .inner
                            .on_disabled_hover_text(reason);
                            false
                        }
                        None => icon_button(ui, icons::CLOSE, DISMISS_NAME).clicked(),
                    }
                }
                // A link at the words' size, as the row's own words are.
                MessageChoice::Link { word, hover } => {
                    let lock = self.locks.get(*at).and_then(holding);
                    let owned = self.hovers.get(*at).and_then(Option::as_deref);
                    ui.allocate_ui_with_layout(
                        egui::vec2(compact_link_width(ui, word), control),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            let response = match lock {
                                Some(reason) => ui
                                    .add_enabled_ui(false, |ui| {
                                        super::components::small_link(ui, word)
                                    })
                                    .inner
                                    .on_disabled_hover_text(reason),
                                None => super::components::small_link(ui, word),
                            };
                            match owned.or(*hover) {
                                Some(hover) => response.on_hover_text(hover),
                                None => response,
                            }
                            .clicked()
                        },
                    )
                    .inner
                }
                choice => {
                    let width = self.choice_width(ui, *at, *choice, kept.get(*at));
                    ui.allocate_ui_with_layout(
                        egui::vec2(width, control),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            self.draw_choice(ui, *at, *choice, f32::INFINITY, None, kept.get(*at))
                                .is_some_and(|response| response.clicked())
                        },
                    )
                    .inner
                }
            };
            activated(ui, pressed, clicked, words, Pressed::Choice(*at));
        }
    }

    /// The choices that sit on rows under the words: all but `Dismiss`.
    fn row_choices(&self) -> impl Iterator<Item = (usize, MessageChoice<'_>)> {
        self.choices
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, choice)| !matches!(choice, MessageChoice::Dismiss(_)))
    }

    /// The words' response, with what the system itself said on hover (`words_hover`).
    fn hovered(&self, words: Response) -> Response {
        match &self.words_hover {
            Some(said) => words.on_hover_text(said.as_str()),
            None => words,
        }
    }

    /// The words, capped at `LIST_MEASURE`; an Undo alone beside them when it fits, else under
    /// them at their first letter; with choices, the Undo and then the choices on rows under the
    /// words, from their first letter. A locked Undo or choice is disabled, with its reason as
    /// its accessible description and painted under the message. Also the y of the centre of
    /// the words' first line. Its presses go to `pressed`.
    fn draw_words(
        &self,
        ui: &mut Ui,
        place: MessagePlace,
        pressed: &mut Option<Pressed>,
        reasons: &mut Reasons<'_>,
    ) -> f32 {
        let mut line = ui.cursor().top() + Type::Body.line() / 2.0;
        // The footer's plan keeps the choices' widths and laid words (none outside a footer).
        let planned = self.planned_choices(ui, reasons);
        let planned = planned.as_deref();
        let response = ui
            .scope(|ui| {
                ui.set_max_width(ui.available_width().min(LIST_MEASURE));
                let words = self.words.as_str();
                if self.row_choices().next().is_none() {
                    match &self.undo {
                        None => {
                            line = first_line_centre(
                                &self.hovered(tone_message(ui, self.tone, words)),
                            );
                        }
                        Some(step) => {
                            let form = self.undo_form(ui, step, reasons);
                            let name = (form.shown != form.label).then_some(form.label.as_str());
                            let lock = holding(&self.undo_lock);
                            let (undo, centre) = undo_message_line(
                                ui,
                                self.tone,
                                words,
                                (&form.shown, Some((form.laid.clone(), form.width))),
                                name,
                                lock,
                                true,
                            );
                            activated(ui, pressed, undo, words, Pressed::Undo);
                            line = centre;
                        }
                    }
                    self.file_line(ui);
                    self.reasons(ui, reasons);
                    return;
                }
                let message = self.hovered(tone_message(ui, self.tone, words));
                line = first_line_centre(&message);
                self.file_line(ui);
                let inset = message.rect.left() - ui.cursor().left();
                let gap = ui.spacing().item_spacing.x;
                let undo = self
                    .undo
                    .as_ref()
                    .map(|step| self.undo_form(ui, step, reasons));
                // Undo, then the choices `L` after it, on one line from the words when they fit;
                // else Undo on its line and the choices on the lines under it, from the words.
                let choices: f32 = self
                    .row_choices()
                    .map(|(at, choice)| {
                        let kept = planned.and_then(|planned| planned.get(at));
                        self.choice_width(ui, at, choice, kept) + gap
                    })
                    .sum();
                // Ruling M8: a choice with help stands on a line of its own, its help under it, as
                // a body fault's repair does; then so does every choice of the message.
                let stacked = self.helps.iter().any(Option::is_some);
                let one_line = !stacked
                    && inset + undo.as_ref().map_or(0.0, |form| form.width + L) + choices
                        <= ui.available_width();
                if undo.is_some() || one_line {
                    ui.horizontal(|ui| {
                        space(ui, inset);
                        if let Some(form) = &undo {
                            let (label, shown) = (&form.label, &form.shown);
                            // Its words start at the message's words, its icon in the message's
                            // icon column.
                            let mut action = Action::quiet(shown)
                                .icon(icons::UNDO)
                                .density(Density::Read)
                                .line_start_in_column()
                                .laid(form.laid.clone())
                                .blocked(holding(&self.undo_lock));
                            if shown != label {
                                action = action.name(label);
                            }
                            let mut response = action.show(ui);
                            if shown != label {
                                response = cut_tip(response, label, holding(&self.undo_lock));
                            }
                            let clicked = response.clicked();
                            activated(ui, pressed, clicked, words, Pressed::Undo);
                            space(ui, (L - gap).max(0.0));
                        }
                        if one_line {
                            self.draw_choices(ui, pressed, planned);
                        }
                    });
                }
                if stacked {
                    // The bottom of what the choice before drew: its help line, else itself.
                    let mut below: Option<f32> = None;
                    let mut help_bottom: Option<f32> = None;
                    for (at, choice) in self.row_choices() {
                        // A red choice stands exactly `M` below the line before it, measured from
                        // that line's own bounds; a safe one keeps `L` from the help line before
                        // it, so that help reads as its own choice's (M8).
                        let help_before = help_bottom.take();
                        if let Some(below) = below
                            && self.loses(at, choice)
                        {
                            space(ui, (below + M - ui.cursor().top()).max(0.0));
                        } else {
                            after_help(ui, help_before);
                        }
                        let shown = message_choice_row(ui, self.tone, |ui| {
                            let room = ui.available_width();
                            let kept = planned.and_then(|planned| planned.get(at));
                            self.draw_choice(ui, at, choice, room, None, kept)
                        });
                        let clicked = shown.as_ref().is_some_and(Response::clicked);
                        activated(ui, pressed, clicked, words, Pressed::Choice(at));
                        below = shown.as_ref().map(|response| response.rect.bottom());
                        if let (Some(help), Some(response)) =
                            (self.helps.get(at).and_then(Option::as_ref), &shown)
                        {
                            let lock = self.locks.get(at).and_then(holding);
                            let help = action_help(ui, self.tone, help, response, lock).bottom();
                            below = Some(help);
                            help_bottom = Some(help);
                        }
                    }
                } else if !one_line {
                    ui.horizontal_top(|ui| {
                        space(ui, inset);
                        ui.horizontal_wrapped(|ui| {
                            self.draw_choices(ui, pressed, planned);
                        });
                    });
                }
                self.reasons(ui, reasons);
            })
            .response;
        register(ui, response.rect, place, self.tone);
        line
    }

    /// The file line under the words, from their first letter (`FileLine::draw`).
    fn file_line(&self, ui: &mut Ui) {
        // The help first, under the words (`FaultMessage::help`), then the file line.
        for help in &self.help {
            message_choice_row(ui, self.tone, |ui| {
                super::components::help(ui, help.as_str());
            });
        }
        if let Some(file) = &self.file {
            message_choice_row(ui, self.tone, |ui| {
                ui.vertical(|ui| file.draw(ui));
            });
        }
    }

    /// Why its Undo or a choice is locked, each reason once, in `Small` muted words on a line of
    /// its own at the message words, as every blocked action in the app says why: never only on
    /// hover. The reason of a lock that is open now keeps its place, drawn but not shown, so a
    /// lock that comes or goes moves no line below it.
    fn reasons(&self, ui: &mut Ui, reasons: &mut Reasons<'_>) {
        // Identical reasons are one line, shown when any of their locks holds.
        let mut said: Vec<(&str, bool)> = Vec::new();
        let locks = std::iter::once(&self.undo_lock)
            .filter(|_| self.undo.is_some())
            .chain(&self.locks);
        for (reason, locked) in locks.flatten() {
            let reason = reason.as_str();
            match said.iter_mut().find(|(said, _)| *said == reason) {
                Some((_, any)) => *any |= *locked,
                None => said.push((reason, *locked)),
            }
        }
        // Ruling R29: an open lock's room is a reservation, not content. A footer that has no
        // room for it where it stands (a third of the pane when pinned; the room down to the
        // pane's end below a short body: ruling Q-c) has its reasons give it up. A pass that
        // measures says how much room the reservations want (measured whether or not they take
        // it) and how much they took, so nothing older than that pass decides. A locked reason
        // always shows. A typed footer's room is its own (`Reasons::Planned`, its plan decides
        // and measures only when the plan is made again); a message outside a footer keeps its
        // room and measures and counts nothing (`Reasons::Body`).
        let (give_up, measure) = match reasons {
            Reasons::Body => (false, false),
            Reasons::Planned(room) => (room.give_up, room.measure),
        };
        let add = |reasons: &mut Reasons<'_>, drawn: bool, room: f32| match reasons {
            Reasons::Body => {}
            Reasons::Planned(sums) => {
                if drawn {
                    sums.drawn += room;
                } else {
                    // Each reservation's room, kept whether this pass draws it or gives it up:
                    // a later pass with room for it takes this room and lays out no reason.
                    sums.reserved += room;
                    sums.measured.push(room);
                }
            }
        };
        for (reason, locked) in said {
            if locked {
                message_choice_row(ui, self.tone, |ui| blocked_reason(ui, Some(reason)));
                continue;
            }
            if measure {
                // What the reservation wants at this width, laid out where it would go, in a
                // child that takes no room and paints nothing.
                count(|work| work.probes += 1);
                let mut probe = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(Rect::from_min_size(
                            ui.cursor().min,
                            egui::vec2(ui.available_width(), 10_000.0),
                        ))
                        .layout(*ui.layout())
                        .invisible(),
                );
                message_choice_row(&mut probe, self.tone, |ui| blocked_reason(ui, Some(reason)));
                let wanted = probe.min_rect().height() + ui.spacing().item_spacing.y;
                add(reasons, false, wanted);
            }
            if !give_up {
                // A typed footer's plan keeps each reservation's room: a pass that does not
                // measure takes it and lays out no reason.
                if let Reasons::Planned(sums) = &mut *reasons
                    && !measure
                    && let Some(&kept) = sums.rooms.get(sums.next)
                {
                    sums.next += 1;
                    // An empty rect as tall as the reason was, placed as a widget is: the same
                    // room, and the same spacing after it, as the reason laid out.
                    let height = (kept - ui.spacing().item_spacing.y).max(0.0);
                    let _ = ui.allocate_exact_size(egui::vec2(0.0, height), egui::Sense::hover());
                    continue;
                }
                // A message outside a footer (`Reasons::Body`) keeps each open lock's room by
                // what decides it: the words, the width, the tone, the fonts and the scale. A
                // quiet frame takes the kept room and lays out no reason (issue #527).
                let kept_room = matches!(reasons, Reasons::Body).then(|| {
                    egui::Id::new((
                        "body_reason_room",
                        reason,
                        ui.available_width().to_bits(),
                        std::mem::discriminant(&self.tone),
                        super::theme::font_generation(ui.ctx()),
                        ui.ctx().pixels_per_point().to_bits(),
                    ))
                });
                if let Some(kept) =
                    kept_room.and_then(|key| ui.data(|data| data.get_temp::<f32>(key)))
                {
                    let height = (kept - ui.spacing().item_spacing.y).max(0.0);
                    let _ = ui.allocate_exact_size(egui::vec2(0.0, height), egui::Sense::hover());
                    continue;
                }
                count(|work| work.laid += 1);
                let before = ui.cursor().top();
                message_choice_row(ui, self.tone, |ui| {
                    ui.scope_builder(egui::UiBuilder::new().invisible(), |ui| {
                        blocked_reason(ui, Some(reason));
                    });
                });
                if measure {
                    add(reasons, true, ui.cursor().top() - before);
                }
                if let Some(key) = kept_room {
                    let taken = ui.cursor().top() - before;
                    ui.data_mut(|data| data.insert_temp(key, taken));
                }
            }
        }
    }

    /// Each choice as the footer's plan keeps it (`ChoiceLaid`), in the choices' order.
    fn plan_choices(&self, ui: &Ui) -> std::sync::Arc<[ChoiceLaid]> {
        self.choices
            .iter()
            .copied()
            .enumerate()
            .map(|(at, choice)| {
                let width = self.choice_width(ui, at, choice, None);
                let plain = match (choice, self.named.get(at).and_then(Option::as_ref)) {
                    (MessageChoice::Choice(word), None) => {
                        Some(Action::secondary(word).density(Density::Read))
                    }
                    (MessageChoice::Primary(word), None) => {
                        Some(Action::primary(word).density(Density::Read))
                    }
                    (MessageChoice::Lose(word), None) => {
                        Some(Action::destructive(word).density(Density::Read))
                    }
                    _ => None,
                };
                let action = plain
                    .as_ref()
                    .map_or(0.0, |action| action.natural_width(ui));
                ChoiceLaid {
                    width,
                    action,
                    laid: plain.map(|plain| plain.laid_words(ui, action)),
                }
            })
            .collect()
    }

    /// The choices as laid once per change: a footer's plan's for this line (the kept ones, or
    /// ones laid now and recorded by a measuring pass), else the body message's own (`body_kept`).
    fn planned_choices(
        &self,
        ui: &Ui,
        reasons: &mut Reasons<'_>,
    ) -> Option<std::sync::Arc<[ChoiceLaid]>> {
        let Reasons::Planned(pass) = reasons else {
            return Some(self.body_kept(ui, "body_choices", 0.0, || self.plan_choices(ui)));
        };
        if !pass.measure
            && let Some(Some(kept)) = pass.choices.get(pass.line)
        {
            return Some(kept.clone());
        }
        let laid = self.plan_choices(ui);
        if pass.measure
            && let Some(slot) = pass.laid_choices.get_mut(pass.line)
        {
            *slot = Some(laid.clone());
        }
        Some(laid)
    }

    /// A choice's whole width, with the `L` before a red one: the plan's (`planned`), else measured.
    fn choice_width(
        &self,
        ui: &Ui,
        at: usize,
        choice: MessageChoice<'_>,
        planned: Option<&ChoiceLaid>,
    ) -> f32 {
        if let Some(planned) = planned {
            return planned.width;
        }
        if let Some(Some((kind, words))) = self.named.get(at) {
            let lose = if *kind == ChoiceKind::Lose { L } else { 0.0 };
            return kind.action(&words.label()).natural_width(ui) + lose;
        }
        match choice {
            MessageChoice::Choice(word) => Action::secondary(word)
                .density(Density::Read)
                .natural_width(ui),
            MessageChoice::Primary(word) => Action::primary(word)
                .density(Density::Read)
                .natural_width(ui),
            MessageChoice::Lose(word) => {
                Action::destructive(word)
                    .density(Density::Read)
                    .natural_width(ui)
                    + L
            }
            MessageChoice::Link { word, .. } => {
                number_width(ui, word, &Type::Body.font(ui)).max(XL)
            }
            MessageChoice::Dismiss(_) => 0.0,
        }
    }

    /// The choices, in order, on wrapped lines: a red one keeps exactly `L` from the one before it
    /// on its line, or starts a line of its own `M` below the others (`before_lose`).
    fn draw_choices(
        &self,
        ui: &mut Ui,
        pressed: &mut Option<Pressed>,
        planned: Option<&[ChoiceLaid]>,
    ) {
        let gap = ui.spacing().item_spacing.x;
        for (at, choice) in self.row_choices() {
            let room = ui.max_rect().width();
            let kept = planned.and_then(|planned| planned.get(at));
            let clicked = self
                .draw_choice(ui, at, choice, room, Some(gap), kept)
                .is_some_and(|response| response.clicked());
            activated(
                ui,
                pressed,
                clicked,
                self.words.as_str(),
                Pressed::Choice(at),
            );
        }
    }

    /// Whether the choice at `at` loses something (red).
    fn loses(&self, at: usize, choice: MessageChoice<'_>) -> bool {
        match self.named.get(at).and_then(Option::as_ref) {
            Some((kind, _)) => *kind == ChoiceKind::Lose,
            None => matches!(choice, MessageChoice::Lose(_)),
        }
    }

    /// One choice, in `room`; on a wrapped line (`wrapped`: the item gap), a red one keeps its
    /// `L` or starts a line of its own (`before_lose`). Its response (none for `Dismiss`, which
    /// is not on a row).
    fn draw_choice(
        &self,
        ui: &mut Ui,
        at: usize,
        choice: MessageChoice<'_>,
        room: f32,
        wrapped: Option<f32>,
        planned: Option<&ChoiceLaid>,
    ) -> Option<Response> {
        let lock = self.locks.get(at).and_then(holding);
        // A plain choice paints the words its plan laid.
        let laid = planned.and_then(|planned| planned.laid.clone());
        fn laid_words<'a>(
            action: Action<'a>,
            words: Option<super::components::LaidAction>,
        ) -> Action<'a> {
            match words {
                Some(words) => action.laid(words),
                None => action,
            }
        }
        match (choice, self.named.get(at).and_then(Option::as_ref)) {
            // Words with objects: cut to the row's width, named with the whole words.
            (_, Some((kind, words))) => {
                if let Some(gap) = wrapped
                    && *kind == ChoiceKind::Lose
                    && at > 0
                {
                    before_lose(ui, kind.action(&words.label()).natural_width(ui), gap);
                }
                Some(data_choice(ui, words, *kind, room, lock))
            }
            (MessageChoice::Choice(word), None) => Some(
                laid_words(Action::secondary(word).density(Density::Read), laid.clone())
                    .blocked(lock)
                    .show(ui),
            ),
            (MessageChoice::Primary(word), None) => Some(
                laid_words(Action::primary(word).density(Density::Read), laid.clone())
                    .blocked(lock)
                    .show(ui),
            ),
            (MessageChoice::Lose(word), None) => {
                if let Some(gap) = wrapped
                    && at > 0
                {
                    let width = planned.map_or_else(
                        || {
                            Action::destructive(word)
                                .density(Density::Read)
                                .natural_width(ui)
                        },
                        |planned| planned.action,
                    );
                    before_lose(ui, width, gap);
                }
                Some(
                    laid_words(
                        Action::destructive(word).density(Density::Read),
                        laid.clone(),
                    )
                    .blocked(lock)
                    .show(ui),
                )
            }
            (MessageChoice::Link { word, hover }, None) => match lock {
                // A locked link is drawn disabled, with its reason, as a blocked action is.
                Some(reason) => Some(
                    ui.add_enabled_ui(false, |ui| link(ui, word))
                        .inner
                        .on_disabled_hover_text(reason),
                ),
                None => {
                    let response = link(ui, word);
                    let owned = self.hovers.get(at).and_then(Option::as_deref);
                    Some(match owned.or(hover) {
                        Some(hover) => response.on_hover_text(hover),
                        None => response,
                    })
                }
            },
            (MessageChoice::Dismiss(_), None) => None,
        }
    }

    /// Undo's words for the room after the message's icon, and the action's width with them.
    /// The room is measured before any form is chosen, so a lone Undo shows its whole form
    /// wherever it fits (`UndoStep::alone`). A typed footer's plan keeps them with its key
    /// (the lines as data, the width, the scale, the style): a pass that does not make the plan
    /// builds and fits no label; the pass that makes it records what it fitted.
    fn undo_form(
        &self,
        ui: &Ui,
        step: &UndoStep,
        reasons: &mut Reasons<'_>,
    ) -> std::sync::Arc<UndoForm> {
        if let Reasons::Planned(pass) = reasons
            && !pass.measure
            && let Some(Some(form)) = pass.forms.get(pass.line)
        {
            return form.clone();
        }
        if let Reasons::Body = reasons {
            // The room is the line's (`undo_room`), so it is in the key.
            let room = ui.available_width();
            return self.body_kept(ui, "body_undo_form", room, || self.fit_undo(ui, step));
        }
        let form = self.fit_undo(ui, step);
        if let Reasons::Planned(pass) = reasons
            && pass.measure
            && let Some(slot) = pass.fitted.get_mut(pass.line)
        {
            *slot = Some(form.clone());
        }
        form
    }

    /// Undo's words fitted to the room now, and laid (`undo_form`).
    fn fit_undo(&self, ui: &Ui, step: &UndoStep) -> std::sync::Arc<UndoForm> {
        count(|work| work.undo_fits += 1);
        let label = step.label();
        let room = undo_room(ui, self.tone, &label);
        let shown = if self.undo_alone {
            step.alone(ui, room)
        } else {
            step.fit(ui, room)
        };
        let action = Action::quiet(&shown)
            .icon(icons::UNDO)
            .density(Density::Read);
        let width = action.natural_width(ui);
        let laid = action.laid_words(ui, width);
        std::sync::Arc::new(UndoForm {
            label,
            shown,
            width,
            laid,
        })
    }

    /// What a body message outside a footer (`Reasons::Body`) keeps of `tag`, made by `make` once
    /// per change: of the body itself, the room `width`, the scale, the style (the theme in it),
    /// the fonts and the atlas epoch (the kept words' galleys hold places in the atlas). A quiet
    /// frame measures and lays out nothing. The slot is the message's place (its owner's, else the
    /// next widget's id in its Ui), so a message whose words change takes no new slot; a different
    /// body in it makes the value again. The slots live in one store swept each frame
    /// (`BodyStore`), so a slot no message used in the last frame goes.
    fn body_kept<T: Clone + Send + std::marker::Sync + 'static>(
        &self,
        ui: &Ui,
        tag: &'static str,
        width: f32,
        make: impl FnOnce() -> T,
    ) -> T {
        let slot = egui::Id::new((tag, self.place.unwrap_or_else(|| ui.next_auto_id())));
        let frame = ui.ctx().cumulative_frame_nr();
        let key = BodyKey {
            width: width.to_bits(),
            scale: ui.ctx().pixels_per_point().to_bits(),
            style: ui.style().clone(),
            fonts: font_key(ui.ctx()),
            atlas: super::theme::atlas_epoch(ui.ctx()),
        };
        let kept = ui.data_mut(|data| {
            let store = data.get_temp_mut_or_default::<BodyStore>(body_store_id());
            store.sweep(frame);
            store.slots.get_mut(&slot).map(|(drawn, kept)| {
                *drawn = frame;
                kept.clone()
            })
        });
        if let Some(kept) = kept.and_then(|kept| kept.downcast::<BodyKept<T>>().ok())
            && kept.key == key
            && kept.body == *self
        {
            return kept.value.clone();
        }
        let value = make();
        let kept = std::sync::Arc::new(BodyKept {
            body: self.clone(),
            key,
            value: value.clone(),
        });
        ui.data_mut(|data| {
            data.get_temp_mut_or_default::<BodyStore>(body_store_id())
                .slots
                .insert(slot, (frame, kept));
        });
        value
    }
}

/// Every body message's kept values (`Body::body_kept`), by slot, with the frame that last used
/// each. Swept once per frame: a slot that no message used in the last frame goes, so the store
/// holds only the messages on screen, whatever their words and places do (review of PR 925).
#[derive(Clone, Default)]
struct BodyStore {
    swept: u64,
    slots: std::collections::HashMap<
        egui::Id,
        (
            u64,
            std::sync::Arc<dyn std::any::Any + Send + std::marker::Sync>,
        ),
    >,
}

impl BodyStore {
    /// Drop each slot not used in this frame or the one before, once per frame.
    fn sweep(&mut self, frame: u64) {
        if self.swept != frame {
            self.swept = frame;
            self.slots.retain(|_, (drawn, _)| *drawn + 1 >= frame);
        }
    }
}

fn body_store_id() -> egui::Id {
    egui::Id::new("design_body_kept")
}

/// How many kept values the body messages hold now (`BodyStore`), for a test.
#[cfg(any(test, feature = "test-support"))]
pub fn body_kept_entries(ctx: &egui::Context) -> usize {
    ctx.data_mut(|data| {
        data.get_temp_mut_or_default::<BodyStore>(body_store_id())
            .slots
            .len()
    })
}

/// What decides a body message's kept Undo words and choices besides the body (`body_kept`).
#[derive(Clone, PartialEq)]
struct BodyKey {
    width: u32,
    scale: u32,
    style: std::sync::Arc<egui::Style>,
    fonts: u64,
    atlas: u64,
}

/// A body message's kept value, with the body and the key it was made for.
struct BodyKept<T> {
    body: Body,
    key: BodyKey,
    value: T,
}

/// A message's Undo words for one room: the whole form (its accessible name and tip), the
/// visible form (`UndoStep::fit` or `alone`), and the action's natural width with it.
pub(in crate::design) struct UndoForm {
    label: String,
    shown: String,
    width: f32,
    /// The visible form's words, laid before paint (`Action::laid`): a pass that keeps the form
    /// lays out no Undo words.
    laid: super::components::LaidAction,
}

/// One choice of a footer line as its plan keeps it: its whole width (with the `L` before a red
/// one), its action's own width, and its words laid before paint for a plain choice (`Choice`,
/// `Primary`, `Lose`). A pass that keeps the plan measures no choice and lays out no words.
#[derive(Clone)]
pub(in crate::design) struct ChoiceLaid {
    width: f32,
    action: f32,
    laid: Option<super::components::LaidAction>,
}

/// The room for an Undo's words on a line of its own under a message of `tone`: the line
/// (capped at `LIST_MEASURE`) less the inset of the message's icon and the action's own icon and
/// padding around `label`. The one measure of every message's Undo.
pub(super) fn undo_room(ui: &Ui, tone: Tone, label: &str) -> f32 {
    let action = Action::quiet(label)
        .icon(icons::UNDO)
        .density(Density::Read);
    let chrome = action.natural_width(ui) - number_width(ui, label, &Type::Strong.font(ui));
    let inset = S + 1.0 + number_width(ui, tone.parts().0, &Type::Body.font(ui)) + XS;
    ui.available_width().min(LIST_MEASURE) - inset - chrome
}

// ------------------------------------------------------------------------------------------------
// Faults

/// How bad a fault is. The tone comes from it and from nothing else, so one fault has one tone
/// in every view.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Severity {
    /// A file or a part of one cannot be read now: the error tone.
    CannotRead,
    /// A write failed: the error tone.
    NotSaved,
    /// It reads, but the DM must fix it before it counts: the warning tone.
    NeedsFixing,
}

impl Severity {
    fn tone(self) -> Tone {
        match self {
            Self::CannotRead | Self::NotSaved => Tone::Error,
            Self::NeedsFixing => Tone::Warning,
        }
    }
}

/// A fault that a status line states after its owner's name (an owner line
/// `Ashen Marches · Cannot be read`). The kind gives the words AND the severity, so a status
/// word never meets another kind's tone. It is a [`Fault`]: its row, its message and its status
/// line take one tone from one `severity`.
///
/// ```
/// use splat_design::outcome::{FaultStatus, fault_status};
/// fn f(ui: &mut egui::Ui) { fault_status(ui, "Ashen Marches", FaultStatus::ManifestDeleted); }
/// ```
///
/// The caller gives no severity and no phrase:
///
/// ```compile_fail
/// use splat_design::outcome::{Severity, fault_status};
/// fn f(ui: &mut egui::Ui) {
///     fault_status(ui, "Ashen Marches", Severity::NeedsFixing, "Cannot be read");
/// }
/// ```
///
/// ```compile_fail
/// use splat_design::outcome::{FaultStatus, Severity, fault_status};
/// fn f(ui: &mut egui::Ui) {
///     fault_status(ui, "Ashen Marches", (FaultStatus::CannotRead, Severity::NeedsFixing));
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaultStatus {
    /// The file cannot be read now (not text, no permission, or its text does not parse).
    CannotRead,
    /// A package's manifest was deleted; its entries still read. The DM writes it again.
    ManifestDeleted,
}

impl FaultStatus {
    /// The status words after the owner's name.
    pub fn word(self) -> &'static str {
        match self {
            Self::CannotRead => "Cannot be read",
            Self::ManifestDeleted => "Manifest deleted",
        }
    }

    /// The whole status words: `<owner> · <word>`. For measuring a line before it is drawn.
    pub fn words(self, owner: &str) -> String {
        format!("{owner} · {}", self.word())
    }
}

impl Fault for FaultStatus {
    fn severity(&self) -> Severity {
        match self {
            Self::CannotRead => Severity::CannotRead,
            Self::ManifestDeleted => Severity::NeedsFixing,
        }
    }

    fn cause(&self) -> Sentence {
        match self {
            Self::CannotRead => crate::sentence!("The file cannot be read."),
            Self::ManifestDeleted => crate::sentence!("The package manifest was deleted."),
        }
    }
}

/// A fault's status line after its owner's name, in the fault's tone with its icon.
pub fn fault_status(ui: &mut Ui, owner: &str, fault: FaultStatus) -> Response {
    let (icon, color) = fault.severity().tone().parts();
    let words = fault.words(owner);
    super::status(
        ui,
        super::Status {
            icon,
            word: &words,
            color,
        },
    )
}

/// Where a fault is: a name, and the line of its file when it has one.
pub struct FaultPlace<'a> {
    pub name: Cow<'a, str>,
    pub line: Option<u32>,
}

/// A fault of the DM's files or of a write. Every view draws it through a typed place, which
/// takes its tone from `severity` and its words from `fault_words`. No place takes a `Tone`:
///
/// ```
/// use splat_design::outcome::{Fault, Sentence, Severity, Tone};
/// struct Missing;
/// impl Fault for Missing {
///     fn severity(&self) -> Severity { Severity::NeedsFixing }
///     fn cause(&self) -> Sentence { splat_design::sentence!("The note is gone.") }
/// }
/// fn f(ui: &mut egui::Ui) { splat_design::outcome::row_fault(ui, &Missing); }
/// ```
/// ```compile_fail
/// use splat_design::outcome::{Fault, Sentence, Severity, Tone};
/// struct Missing;
/// impl Fault for Missing {
///     fn severity(&self) -> Severity { Severity::NeedsFixing }
///     fn cause(&self) -> Sentence { splat_design::sentence!("The note is gone.") }
/// }
/// fn f(ui: &mut egui::Ui) { splat_design::outcome::row_fault(ui, &Missing, Tone::Error); }
/// ```
pub trait Fault {
    fn severity(&self) -> Severity;
    /// The name, and line, the fault belongs to; `None` when the cause names it.
    fn place(&self) -> Option<FaultPlace<'_>> {
        None
    }
    fn cause(&self) -> Sentence;
}

/// A fault's one form: `Cinder Wolf · line 14: The HP is missing.`, `Cinder Wolf: …`, or the
/// cause alone.
pub fn fault_words(fault: &dyn Fault) -> Sentence {
    let cause = fault.cause();
    match fault.place() {
        Some(FaultPlace {
            name,
            line: Some(line),
        }) => Sentence(Cow::Owned(format!("{name} · line {line}: {cause}"))),
        Some(FaultPlace { name, line: None }) => Sentence(Cow::Owned(format!("{name}: {cause}"))),
        None => cause,
    }
}

// ------------------------------------------------------------------------------------------------
// The other typed places

/// A fault that belongs to one row or member, drawn where that row is, in the compact form.
pub fn row_fault(ui: &mut Ui, fault: &dyn Fault) -> Response {
    let tone = fault.severity().tone();
    let response = small_tone_message(ui, tone, fault_words(fault).as_str());
    register(ui, response.rect, MessagePlace::Row, tone);
    response
}

/// The height `row_fault` takes for `fault` in a row `width` wide, measured before paint (layout
/// 11c scope C: a recipe's slot for a row fault, such as a tracker line's repeated-field
/// warning). It follows `small_tone_message`: the bar's room (`S` and a hairline), the icon and
/// the item gap, less `XS`, then the words wrapped in the rest of the row; the row is as tall as
/// the taller of the icon and the words.
pub fn row_fault_height(ui: &Ui, fault: &dyn Fault, width: f32) -> f32 {
    let (icon, _) = fault.severity().tone().parts();
    let small = Type::Small.font(ui);
    let icon = ui.fonts_mut(|fonts| {
        fonts.layout_no_wrap(icon.to_owned(), small.clone(), egui::Color32::PLACEHOLDER)
    });
    let used = S + HAIRLINE_WIDTH + icon.size().x + ui.spacing().item_spacing.x - XS;
    let words = ui.fonts_mut(|fonts| {
        fonts.layout(
            fault_words(fault).as_str().to_owned(),
            small,
            egui::Color32::PLACEHOLDER,
            (width - used).max(0.0),
        )
    });
    icon.size().y.max(words.size().y)
}

/// A notice that came by itself, at the top of a pane's body, in the compact form. It holds no
/// Undo, no choice and no order, so it cannot carry a result's way back; its words can still say
/// anything. `banner`, `row_fault` and `PagePlaceholder` are the body places: they take any `Ui`,
/// so the source lint counts their uses per file (`body-message`), and a new use shows in review.
pub enum Banner<'a> {
    Fault(&'a dyn Fault),
    Notice(Sentence),
}

pub fn banner(ui: &mut Ui, banner: Banner<'_>) -> Response {
    let (tone, words) = match banner {
        Banner::Fault(fault) => (fault.severity().tone(), fault_words(fault)),
        Banner::Notice(words) => (Tone::Info, words),
    };
    let response = small_tone_message(ui, tone, words.as_str());
    register(ui, response.rect, MessagePlace::Banner, tone);
    response
}

/// A typed message drawn in a pane's body, outside its footer, where the thing it is about stands
/// (the Run's header notice and its setup warnings, a group row's faults): a [`Notice`] or a
/// [`FaultMessage`], with its choices (a bordered choice, a red one, a link), its close icon
/// (`Dismiss`) and its file line, drawn as a footer line draws them: the same words, tone,
/// order and locks, at the cursor in the full form. A [list row with one repair](Self::row_repair)
/// fits the repair beside the fault. Both forms give back the caller's id of the choice
/// pressed. It holds no Undo and no ticket: a finished change kept in a store goes in its
/// footer (`Messages::kept`), so a `Said` or a `Done` is not one. A body place: `show` is a
/// reviewed suppression (`Ban::Body`).
///
/// ```compile_fail
/// let _ = splat_design::outcome::BodyMessage::<u8>::from(splat_design::outcome::Said::new(splat_design::sentence!("Saved.")));
/// ```
#[must_use = "call show"]
pub struct BodyMessage<A> {
    form: BodyForm<A>,
    compact: bool,
    /// Drawn in a banner with padding: the close icon reaches into it (`in_padding`).
    overhang: bool,
    /// The whole words behind fitted ones: the words' accessible name and hover.
    detail: Option<String>,
}

enum BodyForm<A> {
    Full(Outcome<A>),
    /// A finished change with no way back (`result`). Boxed: a typed message is the largest form.
    Result(Box<Typed<A>>),
    RowRepair {
        tone: Tone,
        words: Sentence,
        label: &'static str,
        id: A,
    },
}

impl<A> From<Notice<A>> for BodyMessage<A> {
    fn from(notice: Notice<A>) -> Self {
        Self::of(BodyForm::Full(notice.into()))
    }
}

impl<A> From<FaultMessage<A>> for BodyMessage<A> {
    fn from(fault: FaultMessage<A>) -> Self {
        Self::of(BodyForm::Full(fault.into()))
    }
}

impl<A> BodyMessage<A> {
    fn of(form: BodyForm<A>) -> Self {
        Self {
            form,
            compact: false,
            overhang: false,
            detail: None,
        }
    }

    /// In a banner with padding (the Sessions bottom banner): the close icon's target, taller
    /// than the words' first line, reaches into the padding, so the message's height stays its
    /// words' and choices', as `dismiss_on_first_line` places it (`IconRoom::Overhang`). Without
    /// it, the target is part of the message's rect, as in a footer.
    pub fn in_padding(mut self) -> Self {
        self.overhang = true;
        self
    }

    /// A list row with one repair. The button shares the fault's line when both fit.
    pub fn row_repair(fault: &dyn Fault, label: &'static str, id: A) -> Self {
        Self::of(BodyForm::RowRepair {
            tone: fault.severity().tone(),
            words: fault_words(fault),
            label,
            id,
        })
    }

    /// A finished change said where it happened (the Run's header band says its last result),
    /// in the success tone, with its choices and no way back: it has no Undo and no ticket, so
    /// a result whose Undo the DM may press stays a kept line in its footer.
    pub fn result(words: Sentence, choices: impl IntoIterator<Item = Choice<A>>) -> Self {
        Self::of(BodyForm::Result(Box::new(Typed::new(
            Tone::Success,
            words,
            None,
            choices.into_iter().collect(),
        ))))
    }

    /// The compact form, at the size of a row's message (`small_message`): `Small` words with
    /// the tone's icon and bar, at its line height and gap; its choices (a link, a bordered
    /// choice, the close icon) in one band one item gap after the words on their line, as wide
    /// as the choices, never under them; its file line after the words on their last row when
    /// it fits, else under them at the words. For a run's setup rows and its header band. A
    /// row repair keeps its own form.
    pub fn compact(mut self) -> Self {
        self.compact = true;
        self
    }

    /// The whole words when the words are fitted ones (a header band's one-line summary): the
    /// words' accessible name and their hover.
    pub fn detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    /// Draw it at the cursor; the caller's id of the choice pressed.
    pub fn show(self, ui: &mut Ui) -> Option<A> {
        let typed = match self.form {
            BodyForm::Full(message) => message.parts().1,
            BodyForm::Result(typed) => *typed,
            BodyForm::RowRepair {
                tone,
                words,
                label,
                id,
            } => {
                let row = single_action_message(ui, tone, words.as_str(), label);
                register(ui, row.response.rect, MessagePlace::Row, tone);
                return row.inner.then_some(id);
            }
        };
        // The controls' ids are the message's own: its place and its words (issue #526). A notice
        // that replaces another in the same place has new ids, so it never takes a press that
        // was meant for the old one's focused control; two equal messages side by side differ
        // by their place.
        let place = ui.next_auto_id();
        let mut typed = typed;
        typed.body.place = Some(place);
        let scope = (place, "body_message", typed.body.words.clone());
        let pressed = ui
            .push_id(scope, |ui| {
                if self.compact {
                    typed
                        .body
                        .draw_compact(ui, MessagePlace::Page, self.detail.as_deref())
                } else {
                    let room = if self.overhang {
                        IconRoom::Overhang
                    } else {
                        IconRoom::Measured
                    };
                    typed
                        .body
                        .draw_in(ui, MessagePlace::Page, Reasons::Body, room)
                }
            })
            .inner?;
        typed.id(pressed)
    }
}

/// A finished change said at the place where it happened, with its Undo, when the act's owner
/// keeps that Undo with its data in place rather than in a footer store: a Session plan row
/// removed from tonight, at the removed row's place, and `Scene added to tonight.` under the
/// Scene field. The success tone and no choices. Its Undo's words are typed (`UndoStep`), and as
/// the only Undo at its place it shows its whole form wherever it fits, else gives up its object
/// whole (ruling M9, `UndoStep::alone`); its accessible name is always the whole form. It gives
/// back the caller's id when the Undo is pressed. A body place: `show` is a reviewed suppression
/// (`Ban::Body`). A result whose owner keeps it in a `Results` store is a kept line in its
/// footer (`Messages::kept`), never this.
#[must_use = "call show"]
pub struct PlacedResult<A> {
    typed: Typed<A>,
}

impl<A> PlacedResult<A> {
    pub fn new(words: Sentence, undo: Undo<A>) -> Self {
        let mut typed = Typed::new(Tone::Success, words, Some(undo), Vec::new());
        typed.body.undo_alone = true;
        Self { typed }
    }

    /// Draw it at the cursor; the caller's id when its Undo is pressed.
    pub fn show(self, ui: &mut Ui) -> Option<A> {
        let pressed = self.typed.body.draw(ui, MessagePlace::Row)?;
        self.typed.id(pressed)
    }

    /// A result's place that keeps one line whether or not a result shows, under a block it
    /// belongs to (a Timeline journey's check result under its table, M0561): one `Read`
    /// control high, capped at `LIST_MEASURE`, so a result that comes or goes moves nothing
    /// below it. The result is its tone's icon and bar, its words cut by whole words to leave
    /// room for its Undo (the whole words on hover and as their accessible name), and its Undo
    /// at the line's end in its whole form. The caller's id when its Undo is pressed.
    pub fn show_line(result: Option<Self>, ui: &mut Ui) -> Option<A> {
        let Some(result) = result else {
            let _ = super::components::result_line(ui, Tone::Success, None);
            return None;
        };
        let body = &result.typed.body;
        // Its Undo's whole form while the words still fit whole beside it, else its short form
        // (M9, `alone`); its accessible name is always the whole form.
        let (whole, short) = body
            .undo
            .as_ref()
            .map(|undo| (undo.label(), undo.alone(ui, 0.0)))
            .unwrap_or_default();
        let pressed = super::components::result_line(
            ui,
            body.tone,
            Some((body.words.as_str(), &whole, &short)),
        );
        if pressed {
            result.typed.id(Pressed::Undo)
        } else {
            None
        }
    }
}

/// A page or section whose body cannot show: its fault in place of the body, with choices that
/// repair it under the words. It takes the fault's words and tone when it is made, and gives
/// back the caller's id of the choice pressed.
#[must_use = "call show"]
pub struct PagePlaceholder<A> {
    tone: Tone,
    words: Sentence,
    choices: Vec<Choice<A>>,
}

impl<A> PagePlaceholder<A> {
    pub fn new(fault: &dyn Fault) -> Self {
        Self {
            tone: fault.severity().tone(),
            words: fault_words(fault),
            choices: Vec::new(),
        }
    }

    pub fn choice(mut self, choice: Choice<A>) -> Self {
        self.choices.push(choice);
        self
    }

    /// Draw it at the cursor; the caller's id of the choice pressed.
    pub fn show(self, ui: &mut Ui) -> Option<A> {
        let typed = Typed::new(self.tone, self.words, None, self.choices);
        let pressed = typed.body.draw(ui, MessagePlace::Page)?;
        typed.id(pressed)
    }
}

/// One repair of a [`BodyFault`]: its [`ChoiceWords`] (the app's fixed words, `Open in Library`,
/// or a `choice!` template with objects from the DM's data, `Repair {}`,
/// `Replace with a basic {}`), a help line under it, a lock, and the caller's id that comes back
/// when it is pressed. The part cuts the objects to the fault's width, never the fixed words, and
/// names the action with the whole words.
///
/// A label made at run time is not choice words; the twin gives the object as a value:
///
/// ```
/// use splat_design::outcome::Repair;
/// let name = String::from("Bridge Ambush");
/// let _ = Repair::new(splat_design::choice!("Repair {}", name), 1);
/// let _ = Repair::new("Open in Library", 2);
/// ```
/// ```compile_fail
/// use splat_design::outcome::Repair;
/// let name = String::from("Bridge Ambush");
/// let _ = Repair::new(format!("Repair {name}"), 1);
/// ```
///
/// Its lock has the two forms of [`Choice`]'s:
///
/// ```
/// use splat_design::outcome::Repair;
/// let unsaved = true;
/// let _ = Repair::new("Replace with a basic manifest", 1)
///     .locked_when(unsaved, splat_design::sentence!("Save or discard campaign changes first."));
/// let _ = Repair::new("Replace with a basic manifest", 1)
///     .blocked(splat_design::sentence!("Save or discard campaign changes first."));
/// ```
/// ```compile_fail
/// use splat_design::outcome::Repair;
/// let unsaved = true;
/// let _ = Repair::new("Replace with a basic", 1)
///     .blocked(unsaved.then_some(splat_design::sentence!("Save or discard campaign changes first.")));
/// ```
///
/// Named escape: a leaked `String` (`String::leak`, `Box::leak`) is `'static`; a reviewer sees it.
#[must_use = "give it to a `BodyFault`'s `repair`"]
pub struct Repair<A> {
    words: ChoiceWords,
    help: Option<Sentence>,
    lock: Lock,
    id: A,
}

impl<A> Repair<A> {
    /// The repair's words, and the id given back when it is pressed.
    pub fn new(words: impl Into<ChoiceWords>, id: A) -> Self {
        Self {
            words: words.into(),
            help: None,
            lock: None,
            id,
        }
    }

    /// What pressing it keeps and what it replaces, in a help line under it.
    pub fn help(mut self, help: Sentence) -> Self {
        self.help = Some(help);
        self
    }

    /// Locked for as long as it is drawn, for `reason`.
    pub fn blocked(mut self, reason: Sentence) -> Self {
        self.lock = Some((reason, true));
        self
    }

    /// Locked while `locked`, for `reason`; the reason's line is kept while it is open.
    pub fn locked_when(mut self, locked: bool, reason: Sentence) -> Self {
        self.lock = Some((reason, locked));
        self
    }
}

/// A compact message's link's width: its `Small` words, at least its `XL` target.
/// A compact message's words as last measured at one place: the words, what they were measured
/// in, and their width.
#[derive(Clone, Default)]
struct CompactWords {
    words: String,
    measure: Option<super::memo::Measure>,
    width: f32,
}

#[cfg(test)]
thread_local! {
    /// How many times a compact message's words were measured (tests).
    static COMPACT_MEASURES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// The compact form's words' own width (review of PR 592): measured once per change of the
/// words or of what they are measured in (font, theme, scale; `memo::Measure`), and kept at the
/// message's place, so an unchanged message copies and lays out nothing in a frame. The words
/// are compared where they are kept.
fn compact_words_width(ui: &Ui, words: &str) -> f32 {
    let font = Type::Small.font(ui);
    let measure = super::memo::Measure::whole(ui, &font);
    let id = ui.next_auto_id().with("design_compact_words");
    let kept = ui.ctx().data_mut(|data| {
        let kept = data.get_temp_mut_or_default::<CompactWords>(id);
        (kept.words == words && kept.measure.as_ref() == Some(&measure)).then_some(kept.width)
    });
    if let Some(width) = kept {
        return width;
    }
    #[cfg(test)]
    COMPACT_MEASURES.with(|count| count.set(count.get() + 1));
    let width = number_width(ui, words, &font);
    ui.ctx().data_mut(|data| {
        data.insert_temp(
            id,
            CompactWords {
                words: words.to_owned(),
                measure: Some(measure),
                width,
            },
        );
    });
    width
}

fn compact_link_width(ui: &Ui, word: &str) -> f32 {
    number_width(ui, word, &Type::Small.font(ui)).max(XL)
}

/// The help line under an action (ruling M8): what pressing it does, in the help words, from
/// the message's words, painted, never only on hover. The one help line of a body fault's
/// [`Repair`] and of a footer [`Choice`]. The help belongs to the action: it is the action's
/// accessible description (after its lock's reason while that holds), and a keyboard focus on
/// the action brings the action and its help into view together (`reveal_with`). The help
/// line's rect.
///
/// It sits `XS` under what its action drew last (the action, or its lock's reason), top-aligned,
/// so it reads as that action's; the next choice keeps `L` from it ([`after_help`]).
fn action_help(
    ui: &mut Ui,
    tone: Tone,
    help: &Sentence,
    action: &Response,
    lock: Option<&str>,
) -> Rect {
    let top = ui.min_rect().bottom().max(action.rect.bottom()) + XS;
    let room = Rect::from_min_max(
        egui::pos2(ui.max_rect().left(), top),
        egui::pos2(ui.max_rect().right(), ui.max_rect().bottom().max(top)),
    );
    let inset = S + 1.0 + number_width(ui, tone.parts().0, &Type::Body.font(ui)) + XS;
    let line = ui
        .scope_builder(
            egui::UiBuilder::new()
                .max_rect(room)
                .layout(egui::Layout::top_down(egui::Align::Min)),
            |ui| {
                ui.horizontal_top(|ui| {
                    space(ui, inset);
                    super::components::help(ui, help.as_str())
                })
                .inner
            },
        )
        .inner;
    let description = match lock {
        Some(reason) => format!("{reason} {}", help.as_str()),
        None => help.as_str().to_owned(),
    };
    ui.ctx()
        .accesskit_node_builder(action.id, |node| node.set_description(description));
    super::components::reveal_with(ui, action.id, action.rect.union(line.rect));
    line.rect
}

/// The space before the next choice of a stack (ruling M8): `L` from the help line of the
/// choice before it (`help_bottom`), so that help reads as its own choice's and not the next
/// one's; nothing after a choice with no help.
fn after_help(ui: &mut Ui, help_bottom: Option<f32>) {
    if let Some(bottom) = help_bottom {
        space(ui, (bottom + L - ui.cursor().top()).max(0.0));
    }
}

/// A message whose one choice opens an inline creation form in its place (Library's "This folder
/// is a copy of …" with `Register as a new package`, then `Register a copy`). Closed, the fault
/// and its choice, drawn as a [`BodyFault`] with one [`Repair`]; a press opens it. Open, the fault
/// stays, the title (if any) and the form follow under it, in place, and the form's Cancel closes
/// it. The caller keeps `open` and gives the form as a typed
/// [`InlineCreate`](super::layout::InlineCreate), never a closure that draws. A body place: `show`
/// is a reviewed suppression (`Ban::Body`).
#[must_use = "call show"]
pub struct FormOpener<'o> {
    tone: Tone,
    words: Sentence,
    choice: ChoiceWords,
    lock: Lock,
    title: Option<&'static str>,
    open: &'o mut bool,
}

/// What the DM did with a [`FormOpener`] this frame.
#[derive(Debug, Default)]
pub struct Opened {
    /// The choice opened the form (it draws from the next frame).
    pub opened: bool,
    /// The open form's response; `None` while it is closed.
    pub form: Option<super::layout::InlineCreateResponse>,
}

impl<'o> FormOpener<'o> {
    /// The fault, the choice's words that open the form, and the caller's open state.
    pub fn new(fault: &dyn Fault, choice: impl Into<ChoiceWords>, open: &'o mut bool) -> Self {
        Self {
            tone: fault.severity().tone(),
            words: fault_words(fault),
            choice: choice.into(),
            lock: None,
            title: None,
            open,
        }
    }

    /// The open form's title, a group heading above it (`Register a copy`).
    pub fn title(mut self, title: &'static str) -> Self {
        self.title = Some(title);
        self
    }

    /// The choice is locked for as long as it is drawn, for `reason`.
    pub fn blocked(mut self, reason: Sentence) -> Self {
        self.lock = Some((reason, true));
        self
    }

    /// The choice is locked while `locked`, for `reason`; the reason's line is kept while open.
    pub fn locked_when(mut self, locked: bool, reason: Sentence) -> Self {
        self.lock = Some((reason, locked));
        self
    }

    /// Draw it at the cursor: closed, the fault and its choice; open, the fault, the title and
    /// `form`.
    pub fn show<R: super::layout::CreateReasons>(
        self,
        ui: &mut Ui,
        form: super::layout::InlineCreate<'_, super::layout::Complete, R>,
    ) -> Opened {
        let Self {
            tone,
            words,
            choice,
            lock,
            title,
            open,
        } = self;
        // Review of PR 612 (P2): when the form closes, the keyboard goes back to the choice that
        // opened it, from the frame after (kept under the opener's own key).
        let give_back = ui.make_persistent_id(("form_opener_focus", words.as_str()));
        if !*open {
            let repair = Repair {
                words: choice,
                help: None,
                lock,
                id: (),
            };
            let (pressed, controls) = BodyFault {
                tone,
                words,
                repairs: vec![repair],
                whole: None,
                help: Vec::new(),
            }
            .draw_repairs(ui);
            let opened = pressed.is_some();
            if ui.data_mut(|data| data.remove_temp::<bool>(give_back)) == Some(true)
                && let Some(choice) = controls.first()
            {
                choice.request_focus();
            }
            *open = opened;
            return Opened { opened, form: None };
        }
        let _ = BodyFault::<()> {
            tone,
            words,
            repairs: Vec::new(),
            whole: None,
            help: Vec::new(),
        }
        .draw(ui);
        if let Some(title) = title {
            super::heading(ui, super::Heading::Group { title, count: 0 });
        }
        let response = form.show(ui);
        if response.cancel {
            *open = false;
            ui.data_mut(|data| data.insert_temp(give_back, true));
        }
        Opened {
            opened: false,
            form: Some(response),
        }
    }
}

/// A fault in a page's body that the DM can repair from where it is (Campaign content's package
/// and dependency faults): the fault's words and tone, then each [`Repair`] on its own line from
/// the words, with its lock's reason and its help line under it. Its tone and words come from the
/// fault when it is made; the press comes back as the caller's id, at most one per draw (the
/// first in drawing order): the first-press slot is per `show` call, not per pane or frame. It
/// registers the fault's rect for the gate (`MessagePlace::Page`), not a whole row: a row that
/// holds it registers itself. A body place: `show` is a reviewed suppression (`Ban::Body`).
#[must_use = "call show"]
pub struct BodyFault<A> {
    tone: Tone,
    words: Sentence,
    repairs: Vec<Repair<A>>,
    /// The whole sentence when the words quote a value cut around `…` (`whole`).
    whole: Option<String>,
    help: Vec<Sentence>,
}

impl<A> BodyFault<A> {
    pub fn new(fault: &dyn Fault) -> Self {
        Self {
            tone: fault.severity().tone(),
            words: fault_words(fault),
            repairs: Vec::new(),
            whole: None,
            help: Vec::new(),
        }
    }

    /// The whole sentence when the fault's words quote a value cut around `…` (World's unread
    /// values, round 25 FD4): the words' hover and accessible description, so a screen reader
    /// and the pointer reach the value whole.
    pub fn whole(mut self, whole: impl Into<String>) -> Self {
        self.whole = Some(whole.into());
        self
    }

    /// What the DM can do about it, in a help line under its words, before its repairs (the
    /// group editor's missing member), as a [`Choice`]'s help says what the choice does. Each
    /// call adds a line of its own, in call order (as `FaultMessage::help`).
    pub fn help(mut self, help: Sentence) -> Self {
        self.help.push(help);
        self
    }

    pub fn repair(mut self, repair: Repair<A>) -> Self {
        self.repairs.push(repair);
        self
    }

    /// Draw it at the cursor, capped at `LIST_MEASURE`; the caller's id of the repair pressed.
    pub fn show(self, ui: &mut Ui) -> Option<A> {
        self.draw(ui)
    }

    /// `show`'s drawing, for the design's own body places (`FormOpener`).
    fn draw(self, ui: &mut Ui) -> Option<A> {
        self.draw_repairs(ui).0
    }

    /// `draw`, with each repair's control, in order (to give one the keyboard back).
    fn draw_repairs(self, ui: &mut Ui) -> (Option<A>, Vec<Response>) {
        let mut controls = Vec::new();
        let Self {
            tone,
            words,
            repairs,
            whole,
            help,
        } = self;
        let mut pressed = None;
        let response = ui
            .scope(|ui| {
                ui.set_max_width(ui.available_width().min(LIST_MEASURE));
                let text = words.as_str();
                let message = tone_message(ui, tone, text);
                if let Some(whole) = whole.filter(|whole| whole != text) {
                    ui.ctx().accesskit_node_builder(message.id, |node| {
                        node.set_description(whole.clone())
                    });
                    let _ = message.on_hover_text(whole);
                }
                for help in &help {
                    message_choice_row(ui, tone, |ui| {
                        super::components::help(ui, help.as_str());
                    });
                }
                let mut help_bottom = None;
                for (at, repair) in repairs.iter().enumerate() {
                    let lock = holding(&repair.lock);
                    after_help(ui, help_bottom.take());
                    let action = message_choice_row(ui, tone, |ui| {
                        let room = ui.available_width();
                        data_choice(ui, &repair.words, ChoiceKind::Safe, room, lock)
                    });
                    activated(
                        ui,
                        &mut pressed,
                        action.clicked(),
                        text,
                        Pressed::Choice(at),
                    );
                    if let Some((reason, locked)) = &repair.lock {
                        message_choice_row(ui, tone, |ui| {
                            if *locked {
                                blocked_reason(ui, Some(reason.as_str()));
                            } else {
                                ui.scope_builder(egui::UiBuilder::new().invisible(), |ui| {
                                    blocked_reason(ui, Some(reason.as_str()));
                                });
                            }
                        });
                    }
                    if let Some(help) = &repair.help {
                        help_bottom = Some(action_help(ui, tone, help, &action, lock).bottom());
                    }
                    controls.push(action);
                }
            })
            .response;
        register(ui, response.rect, MessagePlace::Page, tone);
        let pressed = match pressed {
            Some(Pressed::Choice(at)) => repairs.into_iter().nth(at).map(|repair| repair.id),
            Some(Pressed::Undo) | None => None,
        };
        (pressed, controls)
    }
}

// ------------------------------------------------------------------------------------------------
// The message-rect registry, for the gate (package G)

/// Which typed place drew a message.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MessagePlace {
    Footer,
    Banner,
    Row,
    Page,
}

/// Where test builds keep the rect and place of each message drawn in the last pass.
#[cfg(any(test, feature = "test-support"))]
pub const MESSAGE_RECTS: &str = "design_message_rects";

fn register(ui: &Ui, rect: Rect, place: MessagePlace, tone: Tone) {
    #[cfg(any(test, feature = "test-support"))]
    super::components::keep_for_pass(ui, MESSAGE_RECTS, (rect, place, tone));
    #[cfg(not(any(test, feature = "test-support")))]
    let _ = (ui, rect, place, tone);
}

/// Each message a typed place drew in the last pass that drew one, with its place. Only test
/// builds keep them; the old message functions register nothing.
#[cfg(any(test, feature = "test-support"))]
pub fn message_rects(ctx: &egui::Context) -> Vec<(Rect, MessagePlace, Tone)> {
    ctx.data(|data| {
        data.get_temp::<(u64, Vec<(Rect, MessagePlace, Tone)>)>(egui::Id::new(MESSAGE_RECTS))
    })
    .map(|(_, rects)| rects)
    .unwrap_or_default()
}

/// `{chosen} of {total} <unit> chosen`: the one form of a count of chosen things, in the app's
/// number form (`1,204 of 2,310 entries chosen`; zero is `0`). The phrase lint keeps this form
/// here.
pub fn chosen_count(chosen: usize, total: usize, unit: &str) -> String {
    let (chosen, total) = (
        super::components::grouped_number(chosen),
        super::components::grouped_number(total),
    );
    if unit.is_empty() {
        format!("{chosen} of {total} chosen")
    } else {
        format!("{chosen} of {total} {unit} chosen")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui;
    use egui_kittest::{Harness, kittest::Queryable};

    /// The whole host's text measures and laid action words (`TEXT_MEASURES`,
    /// `ACTION_WORDS_LAID`): every widget of the test, from its entry through paint.
    fn host_work() -> (usize, usize) {
        (
            super::super::components::TEXT_MEASURES.with(std::cell::Cell::get),
            super::super::components::ACTION_WORDS_LAID.with(std::cell::Cell::get),
        )
    }

    #[allow(
        clippy::disallowed_methods,
        reason = "the test calls the helper `sentence!` runs at compile time"
    )]
    fn rejected(template: &'static str) -> bool {
        std::panic::catch_unwind(|| {
            check_template(template);
        })
        .is_err()
    }

    /// The template rules, as `sentence!` applies them at compile time.
    #[test]
    #[allow(
        clippy::disallowed_methods,
        reason = "the test calls the helper `sentence!` runs at compile time"
    )]
    fn a_template_is_a_sentence_in_the_dms_words() {
        for good in [
            "Removed {}.",
            "{name} draft discarded.",
            "3 seals are due.",
            "Is it done?",
            "Saved (2 files).",
            "She said “Go.”",
            "The party’s seal is kept.",
            "Keys are kept.",
        ] {
            assert!(!rejected(good), "{good}");
        }
        for bad in [
            "",
            "removed {}.",
            "Removed {}",
            "Add a creature first",
            "The party's seal.",
            "The keyring is locked.",
            "The value is cached.",
            "Ask the LLM.",
            "Kept in memory.",
        ] {
            assert!(rejected(bad), "{bad:?}");
        }
        assert!(
            std::panic::catch_unwind(|| check_literal("Removed {name}.")).is_err(),
            "a template with values names them"
        );
    }

    #[test]
    #[allow(clippy::disallowed_methods, reason = "the test of the escape itself")]
    fn outside_words_get_a_capital_and_an_end_mark() {
        let outside = |words: &str| Sentence::outside(words).map(|s| s.to_string());
        assert_eq!(
            outside("no such file (os error 2)").as_deref(),
            Some("No such file (os error 2).")
        );
        assert_eq!(outside(" Done. ").as_deref(), Some("Done."));
        assert_eq!(outside("Go “now”").as_deref(), Some("Go “now”."));
        // M4 gap 3. Plain `outside` leaves the first letter alone only when the first
        // white-space-separated word holds `/` or `\\`, or ends in a dot-extension (Unicode).
        // Each case, with what is shown:
        for data in [
            "wrong-package/ holds a different package.", // kept: holds `/`
            "package.md changed on disk.",               // kept: ends in `.md`
            "café.md changed.",                          // kept: `é` is a letter before `.md`
            "notes\\npc.md: no such file.",              // kept: holds `\\`
            "v2.json, line 3: a list opens here.",       // kept: `.json` before the `,`
            "https://example.org is down.",              // kept: a URL holds `/`
        ] {
            assert_eq!(outside(data).as_deref(), Some(data));
        }
        for (came, shown) in [
            // The first word is `lost`: re-cased. A message that names a path uses
            // `outside_after`, below.
            (
                "lost notes/session.md changed.",
                "Lost notes/session.md changed.",
            ),
            // `e.g.` ends with its `.`: no extension, so the capital, as on main.
            (
                "e.g.: the package is missing.",
                "E.g.: the package is missing.",
            ),
            ("removed: the wolf", "Removed: the wolf."),
            ("ends. here", "Ends. here."),
            // Leading white space goes; the rule reads the first word after it.
            (
                "  package.md changed on disk",
                "package.md changed on disk.",
            ),
        ] {
            assert_eq!(outside(came).as_deref(), Some(shown), "{came:?}");
        }
        // `outside_after`: checked lead words first, then the outside words exactly as they came.
        let after = |words: &str| {
            Sentence::outside_after(crate::lead!("Changed on disk:"), words).map(|s| s.to_string())
        };
        assert_eq!(
            after("lost notes/session.md").as_deref(),
            Some("Changed on disk: lost notes/session.md.")
        );
        assert_eq!(
            after(" café.md changed. ").as_deref(),
            Some("Changed on disk: café.md changed.")
        );
        assert_eq!(after("e.g.").as_deref(), Some("Changed on disk: e.g."));
        assert_eq!(after("  "), None);
        // Blank words are no sentence.
        assert_eq!(outside(""), None);
        assert_eq!(outside(" \n\t "), None);
        assert_eq!(
            crate::sentence!("Removed {}.", "Bell")
                .then(crate::sentence!("Undo is kept."))
                .as_str(),
            "Removed Bell. Undo is kept."
        );
    }

    struct Hp(Severity, Option<u32>);
    impl Fault for Hp {
        fn severity(&self) -> Severity {
            self.0
        }
        fn place(&self) -> Option<FaultPlace<'_>> {
            Some(FaultPlace {
                name: "Cinder Wolf".into(),
                line: self.1,
            })
        }
        fn cause(&self) -> Sentence {
            crate::sentence!("The HP is missing.")
        }
    }

    /// One fault form; the tone from the severity alone; the place of each outcome in the order.
    #[test]
    fn a_fault_has_one_form_and_its_tone_from_its_severity() {
        assert_eq!(
            fault_words(&Hp(Severity::NeedsFixing, Some(14))).as_str(),
            "Cinder Wolf · line 14: The HP is missing."
        );
        assert_eq!(
            fault_words(&Hp(Severity::NeedsFixing, None)).as_str(),
            "Cinder Wolf: The HP is missing."
        );
        assert!(Severity::CannotRead.tone() == Tone::Error);
        assert!(Severity::NotSaved.tone() == Tone::Error);
        assert!(Severity::NeedsFixing.tone() == Tone::Warning);
        let fault = Hp(Severity::CannotRead, Some(3));
        let messages = [
            Outcome::<u8>::Refused(crate::sentence!("No.")),
            Outcome::from(Notice::new(crate::sentence!("Wait."))),
            Outcome::from(FaultMessage::new(&fault)),
        ]
        .map(|outcome| {
            let (rank, typed) = outcome.parts();
            (rank, typed.body.tone)
        });
        assert!(
            messages
                == [
                    (FooterRank::Reason, Tone::Warning),
                    (FooterRank::Notice, Tone::Info),
                    (FooterRank::Reason, Tone::Error),
                ],
            "{messages:?}"
        );
        let results = [
            KeptOutcome::from(Done::new(Verb::Remove, "Bell").undo(1u8)),
            KeptOutcome::from(Done::new(Verb::Remove, "Bell")),
            KeptOutcome::Notice(
                Notice::new(crate::sentence!("Draft discarded.")),
                UndoStep::verb(Verb::Discard).pressed_as(2),
            ),
            KeptOutcome::Fault(
                FaultMessage::new(&fault),
                UndoStep::verb(Verb::Add).pressed_as(3),
            ),
        ]
        .map(|outcome| {
            let typed = outcome.typed();
            (typed.body.undo.is_some(), typed.body.tone, typed.undo)
        });
        assert!(
            results
                == [
                    (true, Tone::Success, Some(1)),
                    (false, Tone::Success, None),
                    (true, Tone::Info, Some(2)),
                    (true, Tone::Error, Some(3)),
                ],
            "{results:?}"
        );
    }

    /// Layout 11c scope C: a row fault's measured height (`row_fault_height`) is the height it
    /// paints, at every width and for words of one line and of several, in each tone. The
    /// reference is the painted row's own rect.
    #[test]
    #[allow(
        clippy::disallowed_methods,
        reason = "the test draws row faults to compare their measure"
    )]
    fn a_row_fault_paints_at_its_measured_height() {
        use crate::design::test_harness::ThemedPage as _;
        let long = "line 12: the field `ask` is set twice for one name, so the second line \
                    leaves its amount out and keeps its other actions";
        let mut wrapped = 0;
        for severity in [Severity::NeedsFixing, Severity::CannotRead] {
            for words in ["Not saved.", long] {
                for width in [180.0, 260.0, 340.0, 520.0, 900.0] {
                    let painted = std::rc::Rc::new(std::cell::Cell::new((0.0, 0.0)));
                    let out = painted.clone();
                    let mut h = Harness::builder()
                        .with_size(egui::vec2(width, 400.0))
                        .themed_ui(move |ui| {
                            let fault = Said(severity, words);
                            let measured = row_fault_height(ui, &fault, ui.available_width());
                            let rect = row_fault(ui, &fault).rect;
                            out.set((measured, rect.height()));
                        });
                    h.run_steps(2);
                    let (measured, height) = painted.get();
                    assert!(
                        (measured - height).abs() < 0.5,
                        "row-fault-measure: {severity:?} {width} px {words:?}: measured {measured}, painted {height}"
                    );
                    if height > 30.0 {
                        wrapped += 1;
                    }
                }
            }
        }
        assert!(
            wrapped > 0,
            "row-fault-measure-reached: some width wraps the words"
        );
    }

    /// A fault of the given severity whose cause is the given words, for the measure's proof.
    struct Said(Severity, &'static str);

    impl Fault for Said {
        fn severity(&self) -> Severity {
            self.0
        }
        fn cause(&self) -> Sentence {
            Sentence(Cow::Borrowed(self.1))
        }
    }

    /// One fault drawn in every place has one tone, its severity's, and one form of words.
    #[test]
    #[allow(
        clippy::disallowed_methods,
        reason = "the test draws one fault in every body place"
    )]
    fn one_fault_has_one_tone_in_every_place() {
        use crate::design::test_harness::ThemedPage as _;
        let mut h = Harness::builder()
            .with_size(egui::vec2(600.0, 500.0))
            .themed_ui(|ui| {
                let fault = Hp(Severity::NeedsFixing, Some(14));
                banner(ui, Banner::Fault(&fault));
                row_fault(ui, &fault);
                let _ = PagePlaceholder::<NoPress>::new(&fault).show(ui);
                let _ = BodyFault::<NoPress>::new(&fault).show(ui);
                let frame = PaneFrame::new(ui);
                let mut messages =
                    Messages::<NoPress>::new(&frame, "one_fault", ui.available_width());
                messages.outcome(FaultMessage::new(&fault));
                // M4 gap 2: a warning before an act is a notice in the warning tone, and a
                // plain notice keeps the info tone.
                messages.outcome(Notice::warning(crate::sentence!(
                    "Splat no longer shows these files."
                )));
                messages.outcome(Notice::new(crate::sentence!("Sync is off.")));
                let _ = messages.pane(ui, crate::design::Body::Content, |_| ());
            });
        h.run_steps(4);
        let rects = message_rects(&h.ctx);
        let places: Vec<_> = rects.iter().map(|(_, place, _)| *place).collect();
        assert_eq!(
            places,
            [
                MessagePlace::Banner,
                MessagePlace::Row,
                MessagePlace::Page,
                MessagePlace::Page,
                MessagePlace::Footer,
                MessagePlace::Footer,
                MessagePlace::Footer
            ]
        );
        let tones: Vec<_> = rects.iter().map(|(_, _, tone)| *tone).collect();
        assert_eq!(
            tones,
            [[Tone::Warning; 6].as_slice(), &[Tone::Info]].concat(),
            "{rects:?}"
        );
        // The warning notice paints the warning's bar and icon before its words, as the gate's
        // tone checks (`has_tone_bar`, the icon check) see them.
        let words = h
            .get_by_role_and_label(
                egui::accesskit::Role::Label,
                "Splat no longer shows these files.",
            )
            .rect();
        let (icon, bar) = (super::icons::WARN, colors().result.warning);
        let shapes = &h.output().shapes;
        let painted_icon = shapes.iter().any(|clipped| {
            matches!(&clipped.shape, egui::Shape::Text(text)
                if text.galley.job.text == icon
                    && text.pos.x < words.left()
                    && (text.pos.y - words.top()).abs() <= Type::Body.line())
        });
        let painted_bar = shapes.iter().any(|clipped| {
            matches!(&clipped.shape, egui::Shape::Rect(rect)
                if rect.fill == bar
                    && rect.rect.right() < words.left()
                    && (rect.rect.top() - words.top()).abs() <= 1.0)
        });
        assert!(
            painted_icon && painted_bar,
            "the warning notice's icon {painted_icon} and bar {painted_bar}"
        );
        assert_eq!(
            h.query_all_by_role_and_label(
                egui::accesskit::Role::Label,
                "Cinder Wolf · line 14: The HP is missing."
            )
            .count(),
            5
        );
    }

    /// M4 gap 1: a footer message's file line. The path is data, drawn as it is, from the words'
    /// first letter; its links are targets of at least `XL` whose names hold the relative path,
    /// and a press opens the file without a press of the pane. A path wider than its line is cut
    /// in the middle and keeps its file name, with the whole path as its name, in its hover, and
    /// in each link's tip on keyboard focus. A file gone at the press opens nothing and locks its
    /// links with a reason. A root that is not absolute, or a relative path with a root or `..`,
    /// draws no link. 30 such messages at 960 × 640 keep the footer within a third of the pane,
    /// and the body in view above it (the footer's region scrolls: `Footer`'s own proof).
    #[test]
    fn a_footer_file_line_keeps_its_path_and_opens_its_file() {
        use crate::design::test_harness::Themed as _;
        use egui::accesskit::Role;
        use egui_kittest::kittest::NodeT;
        use std::path::Path;
        const BODY: &str = "The body starts here.";
        const SHORT: &str = "wrong-package/package.md";
        const LONG_A: &str =
            "packages/a/Session 12 at the Northern Bridge Crossing of Ravenholm/package.md";
        const LONG_B: &str =
            "packages/b/Session 12 at the Northern Bridge Crossing of Ravenholm/package.md";
        struct Fixture {
            width: f32,
            count: usize,
            lines: Vec<FileLine>,
            pressed: usize,
        }
        let vault = tempfile::tempdir().unwrap();
        let root = vault.path().join("Ashen Marches");
        for relative in [SHORT, LONG_A, LONG_B] {
            let file = root.join(relative);
            std::fs::create_dir_all(file.parent().unwrap()).unwrap();
            std::fs::write(&file, "x").unwrap();
        }
        let fault = Hp(Severity::NeedsFixing, Some(9));
        let harness = |width: f32, count: usize, lines: Vec<FileLine>| {
            let mut h = Harness::builder()
                .with_size(egui::vec2(960.0, 640.0))
                .themed_ui_state(
                    move |ui, fixture: &mut Fixture| {
                        let fault = Hp(Severity::NeedsFixing, Some(9));
                        ui.set_max_width(fixture.width);
                        let frame = PaneFrame::new(ui);
                        let mut messages =
                            Messages::<u8>::new(&frame, "file_line_footer", ui.available_width());
                        for at in 0..fixture.count {
                            let line = fixture.lines[at % fixture.lines.len()].clone();
                            let message = FaultMessage::new(&fault)
                                .choice(Choice::new(MessageChoice::Choice("Try again"), at as u8));
                            messages.outcome(message.file(line));
                        }
                        let paned = messages.pane(ui, crate::design::Body::Content, |ui| {
                            ui.label(BODY);
                        });
                        if paned.pressed.is_some() {
                            fixture.pressed += 1;
                        }
                    },
                    Fixture {
                        width,
                        count,
                        lines,
                        pressed: 0,
                    },
                );
            h.run_steps(4);
            h
        };
        let line = |relative: &str| FileLine::new(&root, relative).edit().show();
        let painted = |h: &Harness<'_, Fixture>, rect: Rect| -> Vec<String> {
            h.output()
                .shapes
                .iter()
                .filter_map(|clipped| match &clipped.shape {
                    egui::Shape::Text(text)
                        if rect.contains(text.pos + text.galley.size() / 2.0) =>
                    {
                        Some(text.galley.job.text.clone())
                    }
                    _ => None,
                })
                .collect()
        };
        // Shown: painted whole inside its clip (a run cut off by the footer's edge is not).
        let shows = |h: &Harness<'_, Fixture>, words: &str| {
            h.output().shapes.iter().any(|clipped| {
                matches!(&clipped.shape, egui::Shape::Text(text)
                if text.galley.job.text == words
                    && clipped.clip_rect.expand(0.5).contains_rect(
                        Rect::from_min_size(text.pos, text.galley.size())
                    ))
            })
        };
        let footer = |h: &Harness<'_, Fixture>| {
            crate::design::pane::footer_rect(&h.ctx, egui::Id::new("file_line_footer"))
                .expect("the footer")
        };
        let opened =
            |h: &mut Harness<'_, Fixture>| {
                let mut opened: Vec<String> = Vec::new();
                for _ in 0..3 {
                    h.step();
                    opened.extend(h.output().platform_output.commands.iter().filter_map(
                        |command| match command {
                            egui::OutputCommand::OpenUrl(open) => Some(open.url.clone()),
                            _ => None,
                        },
                    ));
                }
                opened
            };
        let whole = |relative: &str| root.join(relative).display().to_string();
        // A node by the name the part gave it (`named`), not the words of its text run.
        let named = |name: String| {
            egui_kittest::kittest::By::new()
                .predicate(move |node| node.data().label() == Some(name.as_str()))
        };
        let words = fault_words(&fault);

        // Wide: the path whole, as it is on disk (no capital), at the words' first letter,
        // then its two links on the same line, named with the relative path.
        let mut h = harness(960.0, 1, vec![line(SHORT)]);
        let said = h.get_by_role_and_label(Role::Label, words.as_str()).rect();
        let path = h
            .query_all(named(whole(SHORT)))
            .next()
            .expect("the path")
            .rect();
        assert_eq!(painted(&h, path), [SHORT], "the path, whole and as it is");
        assert!(
            (path.left() - said.left()).abs() <= 1.0 && path.top() >= said.bottom() - 1.0,
            "the path {path:?} starts under the words {said:?}, at their first letter"
        );
        for word in [EDIT_IN_OBSIDIAN, SHOW_FILE] {
            let name = format!("{word}: {SHORT}");
            let target = h.get_by_label(&name).rect();
            assert!(
                target.width() >= XL && target.height() >= XL,
                "{name}: a target of at least XL, {target:?}"
            );
            assert!(
                target.left() > path.right() && (target.center().y - path.center().y).abs() <= 1.0,
                "{name} {target:?} follows the path {path:?} on its line"
            );
        }
        assert!(
            h.query_by_label(SHOW_FILE).is_none(),
            "a link's name names the file, not only its words"
        );
        h.get_by_label(&format!("{SHOW_FILE}: {SHORT}")).click();
        assert_eq!(
            opened(&mut h),
            [file_url(&root.join(SHORT)).unwrap()],
            "Show file opens the file"
        );
        assert_eq!(
            h.state().pressed,
            0,
            "a link opens its file, not a press of the pane"
        );
        // The file goes before the next press: nothing opens, its links lock and say why, and
        // they open again once it is back. Review of PR 452, 1: the reason is part of the
        // footer's plan, so the footer grows by its line and shows it whole (no inner scroll
        // with room to spare), and gives the room back when the file is back.
        let open = footer(&h);
        std::fs::rename(root.join(SHORT), root.join("moved.md")).unwrap();
        h.get_by_label(&format!("{SHOW_FILE}: {SHORT}")).click();
        assert_eq!(
            opened(&mut h),
            Vec::<String>::new(),
            "a gone file opens nothing"
        );
        h.run_steps(2);
        let gone = footer(&h);
        assert!(
            h.get_by_label(&format!("{SHOW_FILE}: {SHORT}"))
                .accesskit_node()
                .is_disabled()
                && shows(&h, FILE_GONE.as_str()),
            "its links lock, and say why, whole in view"
        );
        assert!(
            gone.height() > open.height() + 1.0 && gone.bottom() <= 640.0,
            "the footer grows by the reason's line: {open:?} -> {gone:?}"
        );
        std::fs::rename(root.join("moved.md"), root.join(SHORT)).unwrap();
        h.run_steps(3);
        assert!(
            !h.get_by_label(&format!("{SHOW_FILE}: {SHORT}"))
                .accesskit_node()
                .is_disabled()
                && !shows(&h, FILE_GONE.as_str()),
            "back: the links open again"
        );
        assert!(
            (footer(&h).height() - open.height()).abs() <= 0.5,
            "back: the footer gives the reason's room back: {open:?} -> {:?}",
            footer(&h)
        );

        // Narrow, two long paths that end in one name: each path takes its own line, cut in the
        // middle, and keeps its file name; its name and hover keep it whole; the links wrap under
        // it. The two lines' links have two names, and keyboard focus on a link shows the whole
        // path.
        let mut h = harness(320.0, 2, vec![line(LONG_A), line(LONG_B)]);
        let said = h
            .query_all_by_role_and_label(Role::Label, words.as_str())
            .next()
            .unwrap()
            .rect();
        let node = h.query_all(named(whole(LONG_A))).next().expect("the path");
        let path = node.rect();
        let shown = painted(&h, path).concat();
        assert!(
            shown.contains('…') && shown.starts_with("packages/") && shown.ends_with("/package.md"),
            "cut in the middle, the file name kept: {shown:?}"
        );
        assert!(path.right() <= 321.0, "inside its line: {path:?}");
        assert!(
            (path.left() - said.left()).abs() <= 1.0,
            "the path {path:?} at the words {said:?}"
        );
        let edit = h
            .get_by_label(&format!("{EDIT_IN_OBSIDIAN}: {LONG_A}"))
            .rect();
        assert!(
            edit.top() >= path.bottom() - 1.0 && (edit.left() - said.left()).abs() <= 1.0,
            "the links wrap under the path, at the words: {edit:?}"
        );
        assert!(
            h.query_by_label(&format!("{EDIT_IN_OBSIDIAN}: {LONG_B}"))
                .is_some(),
            "two files of one name: two link names"
        );
        node.hover();
        h.run_steps(3);
        assert!(shows(&h, &whole(LONG_A)), "the hover shows the whole path");
        // The pointer leaves; only keyboard focus remains.
        h.get_by_label(BODY).hover();
        h.run_steps(3);
        assert!(!shows(&h, &whole(LONG_B)), "no tip before the focus");
        h.get_by_label(&format!("{SHOW_FILE}: {LONG_B}")).focus();
        h.run_steps(3);
        assert!(
            shows(&h, &whole(LONG_B)) && !shows(&h, &whole(LONG_A)),
            "keyboard focus on a link shows its whole path"
        );

        // A root that is not absolute, or a relative path with a root or `..`, draws no link and
        // shows the path as given.
        for refused in [
            FileLine::new("vault", "a.md").edit().show(),
            FileLine::new(&root, "../other/a.md").edit().show(),
            FileLine::new(&root, Path::new("/other/a.md")).edit().show(),
        ] {
            let shown = refused.shown();
            let h = harness(960.0, 1, vec![refused]);
            assert!(
                h.query_by_label_contains(EDIT_IN_OBSIDIAN).is_none()
                    && h.query_by_label_contains(SHOW_FILE).is_none()
                    && h.query_all(named(shown.clone())).next().is_some(),
                "{shown}: no link, the path as given"
            );
        }

        // 30 messages with file lines at 960 × 640: below the short body the footer starts `S`
        // after it and takes the room down to the pane's end (ruling Q-c; a pinned footer would
        // take at most a third), and the body stays in view above it.
        let h = harness(960.0, 30, vec![line(SHORT)]);
        let footer = crate::design::pane::footer_rect(&h.ctx, egui::Id::new("file_line_footer"))
            .expect("the footer is drawn");
        let body = h.get_by_label(BODY).rect();
        let room = crate::design::pane::pane_record(&h.ctx, egui::Id::new("file_line_footer"))
            .expect("the pane's record")
            .room();
        assert!(
            (footer.top() - (body.bottom() + S)).abs() <= 1.0
                && (footer.bottom() - room.bottom()).abs() <= 1.0
                && footer.height() > room.height() / 3.0 + 1.0
                && body.top() >= 0.0,
            "30 file lines: footer {footer:?}, body {body:?}, pane {room:?}"
        );
        let lowest = h
            .query_all(named(whole(SHORT)))
            .map(|node| node.rect().bottom())
            .fold(f32::MIN, f32::max);
        assert!(
            lowest > footer.bottom(),
            "the footer's region scrolls: the last path {lowest} is below its view {footer:?}"
        );
    }

    /// The URLs a file line opens: `file:` by the platform's rules from an absolute path (none
    /// from a relative one), and Obsidian's query with each unsafe byte written as `%XX`.
    #[test]
    fn a_file_line_opens_the_right_place() {
        use std::path::Path;
        let odd = Path::new("/vault/A & B/#é%?.md");
        assert_eq!(
            obsidian_url(odd),
            "obsidian://open?path=/vault/A%20%26%20B/%23%C3%A9%25%3F.md"
        );
        assert_eq!(
            file_url(odd).as_deref(),
            Some("file:///vault/A%20&%20B/%23%C3%A9%25%3F.md")
        );
        assert_eq!(file_url(Path::new("vault/a.md")), None, "never a host");
    }

    /// A result and its Undo say one verb; a fit keeps the verb and the part that tells two
    /// steps apart, and drops whole words of the object.
    #[test]
    fn an_undo_keeps_its_verb_and_what_tells_it_apart() {
        let done = Done::new(Verb::Remove, "Take seal: Keyring +1.")
            .more(crate::sentence!("With it: Bell is done."))
            .undo(1u8);
        assert_eq!(
            done.words().as_str(),
            "Removed Take seal: Keyring +1. With it: Bell is done."
        );
        let typed = KeptOutcome::from(done).typed();
        assert_eq!(
            typed.body.undo.unwrap().label(),
            "Undo remove Take seal: Keyring +1"
        );
        assert_eq!(UndoStep::act("Take seal").label(), "Undo: Take seal");
        assert_eq!(UndoStep::verb(Verb::Add).label(), "Undo add");
        // Ruling R16: a change's Undo joins its object with `to`; a discard's with `of`.
        assert_eq!(
            UndoStep::of(Verb::Change, "Part of").label(),
            "Undo change to Part of"
        );
        assert_eq!(
            UndoStep::of(Verb::Discard, "Bridge Ambush").label(),
            "Undo discard of Bridge Ambush"
        );
        // M3 gap 1: each verb's two forms, from one entry of the table, with its joining word.
        for verb in Verb::ALL {
            let done = Done::new(verb, "Bridge Ambush").undo(0u8);
            let join = verb.join().map_or(String::new(), |join| format!("{join} "));
            assert_eq!(
                done.words().as_str(),
                format!("{} Bridge Ambush.", verb.past()),
                "{verb:?}"
            );
            assert_eq!(
                KeptOutcome::from(done).typed().body.undo.unwrap().label(),
                format!("Undo {} {join}Bridge Ambush", verb.word()),
                "{verb:?}"
            );
            assert_eq!(
                UndoStep::of(verb, "Bell").short(),
                format!("Undo {}", verb.word())
            );
        }
        assert_eq!(
            UndoStep::of(Verb::Discard, "Bridge Ambush").label(),
            "Undo discard of Bridge Ambush"
        );
        assert_eq!(UndoStep::of(Verb::Tick, "“Q?”").label(), "Undo tick “Q?”");
        // The object first, and the place, which the Undo keeps as its tell-apart part.
        let tick = Done::new(Verb::Tick, "“Who hired the bandits?”")
            .object_first()
            .within("Horn Rings")
            .undo(1u8);
        assert_eq!(
            tick.words().as_str(),
            "“Who hired the bandits?” ticked in Horn Rings."
        );
        let step = KeptOutcome::from(tick).typed().body.undo.unwrap();
        assert_eq!(
            step.label(),
            "Undo tick “Who hired the bandits?” in Horn Rings"
        );
        assert_eq!(step.short(), "Undo tick in Horn Rings");
        let ctx = egui::Context::default();
        crate::design::theme::install(&ctx);
        let _ = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let step = UndoStep::of(Verb::Copy, "Ravenholm Ironside Wardens").tell_apart("2");
                let whole = step.label();
                let font = Type::Strong.font(ui);
                let room = number_width(ui, "Undo copy Ravenholm Ironside… 2", &font) + 1.0;
                assert_eq!(step.fit(ui, 10_000.0), whole);
                assert_eq!(step.fit(ui, room), "Undo copy Ravenholm Ironside… 2");
                assert_eq!(step.fit(ui, 1.0), "Undo copy … 2");
                // A verb with a joining word: the whole form, or the short form, never a cut.
                let discard = UndoStep::of(Verb::Discard, "Bridge Ambush");
                let whole = number_width(ui, "Undo discard of Bridge Ambush", &font);
                assert_eq!(
                    discard.fit(ui, whole + 1.0),
                    "Undo discard of Bridge Ambush"
                );
                assert_eq!(discard.fit(ui, whole - 1.0), "Undo discard");
                assert_eq!(discard.fit(ui, 1.0), "Undo discard");
                // Ruling M9: a lone Undo measures its whole form first; only where that does not
                // fit does it give up its object and the part that tells steps apart, never cut.
                let whole = number_width(ui, "Undo copy Ravenholm Ironside Wardens 2", &font);
                assert_eq!(
                    step.alone(ui, whole + 1.0),
                    "Undo copy Ravenholm Ironside Wardens 2"
                );
                assert_eq!(step.alone(ui, whole - 1.0), "Undo copy");
                assert_eq!(step.alone(ui, 1.0), "Undo copy");
            });
        });
    }

    /// M3 gap 1, in the pane's footer: the visible Undo words are `fit`'s for the line's room,
    /// and the accessible name is always the whole form. Narrow: `Undo discard` named
    /// `Undo discard of Bridge Ambush`; a long object of a verb with no joining word is cut in
    /// the visible words only. Wide: the whole form shows. The press reaches its entry.
    /// M-N1.1: a result with its Undo at its own place (`PlacedResult`, a Session plan row
    /// removed from tonight) follows ruling M9 as a footer's lone Undo does. Wide, the whole
    /// form shows; narrow, where it does not fit, `Undo remove` without its object (its words
    /// name it), never a cut object. Its accessible name stays whole, and its press comes back
    /// as the caller's id.
    #[test]
    #[allow(
        clippy::disallowed_methods,
        reason = "the test of the body places `PlacedResult::show` and `BodyMessage::show` themselves"
    )]
    fn a_placed_result_shows_its_undo_whole_where_it_fits() {
        use crate::design::test_harness::Themed as _;
        const OBJECT: &str = "Bridge Ambush at the Ravenholm Ironside Crossing";
        let name = format!("Undo remove {OBJECT}");
        let harness = |width: f32| {
            let mut h = Harness::builder()
                .with_size(egui::vec2(960.0, 300.0))
                .themed_ui_state(
                    move |ui, pressed: &mut Option<u8>| {
                        ui.set_max_width(width);
                        let result = PlacedResult::new(
                            crate::sentence!("Removed “{}” from tonight.", OBJECT),
                            UndoStep::of(Verb::Remove, OBJECT).pressed_as(7_u8),
                        );
                        if let Some(id) = result.show(ui) {
                            *pressed = Some(id);
                        }
                    },
                    None,
                );
            h.run_steps(2);
            h
        };
        let shown = |h: &Harness<'_, Option<u8>>| -> Vec<String> {
            let undo = h.get_by_label(&name).rect();
            h.output()
                .shapes
                .iter()
                .filter_map(|clipped| match &clipped.shape {
                    egui::Shape::Text(text)
                        if text.galley.job.text.starts_with("Undo")
                            && undo.contains(text.pos + text.galley.rect.center().to_vec2()) =>
                    {
                        Some(text.galley.job.text.clone())
                    }
                    _ => None,
                })
                .collect()
        };
        let h = harness(900.0);
        assert_eq!(shown(&h), [name.as_str()], "wide: the whole form");
        let mut h = harness(320.0);
        assert_eq!(
            shown(&h),
            ["Undo remove"],
            "narrow: the object goes whole, never cut"
        );
        h.get_by_label(&name).click();
        h.run_steps(2);
        assert_eq!(
            *h.state(),
            Some(7),
            "its press comes back as the caller's id"
        );
        // M0561 (M-P0.27): the line form keeps one `Read` control's height whether or not a
        // result shows, so a result that comes or goes moves nothing below it; its words cut by
        // whole words before its whole Undo, the whole words on hover and as their name; its
        // Undo's press comes back as the caller's id.
        const WORDS: &str = "12 travel checks rolled for the journey across the Ravenholm hills.";
        let line = |width: f32, shown: bool| {
            let mut h = Harness::builder()
                .with_size(egui::vec2(960.0, 300.0))
                .themed_ui_state(
                    move |ui, state: &mut (Option<u8>, f32)| {
                        ui.set_max_width(width);
                        let result = shown.then(|| {
                            PlacedResult::new(
                                crate::sentence!("{} travel checks rolled for the journey across the Ravenholm hills.", 12),
                                UndoStep::of(Verb::Roll, "12 travel checks").pressed_as(9_u8),
                            )
                        });
                        if let Some(id) = PlacedResult::show_line(result, ui) {
                            state.0 = Some(id);
                        }
                        state.1 = ui.label("Below").rect.top();
                    },
                    (None, 0.0),
                );
            h.run_steps(2);
            h
        };
        let (empty, full) = (line(320.0, false), line(320.0, true));
        assert!(
            (empty.state().1 - full.state().1).abs() < 0.5,
            "m-p0-result-line: what follows stays at {} with or without the result ({})",
            empty.state().1,
            full.state().1
        );
        let words = full
            .query_all(
                egui_kittest::kittest::By::new()
                    .predicate(|node| node.data().label() == Some(WORDS)),
            )
            .next()
            .expect("the words, named whole")
            .rect();
        let undo = full.get_by_label("Undo roll 12 travel checks").rect();
        assert!(
            (words.center().y - undo.center().y).abs() < 2.0 && words.right() <= undo.left(),
            "m-p0-result-line: the cut words and the whole Undo on one line: {words:?} {undo:?}"
        );
        // The words come first: the Undo shows its whole form only while the words still fit
        // whole beside it (here at 900), else its short form (at 320).
        let undo_shown = |h: &Harness<'_, (Option<u8>, f32)>| -> Vec<String> {
            h.output()
                .shapes
                .iter()
                .filter_map(|clipped| match &clipped.shape {
                    egui::Shape::Text(text) if text.galley.job.text.starts_with("Undo") => {
                        Some(text.galley.job.text.clone())
                    }
                    _ => None,
                })
                .collect()
        };
        let wide = line(900.0, true);
        assert_eq!(
            (undo_shown(&full), undo_shown(&wide)),
            (
                vec!["Undo roll".to_owned()],
                vec!["Undo roll 12 travel checks".to_owned()]
            ),
            "m-p0-result-line: the short Undo where the words do not fit whole, the whole one \
             where they do"
        );
        let mut full = full;
        full.get_by_label("Undo roll 12 travel checks").click();
        full.run_steps(2);
        assert_eq!(
            full.state().0,
            Some(9),
            "m-p0-result-line: the Undo's id comes back"
        );
        // The typed message's draw path outside a footer (`Reasons::Body`): a body message keeps
        // its Undo's words and its choices' widths and words by the body and what lays them out
        // (`Body::body_kept`), as a footer's plan does. This host draws only body messages.
        let mut h = Harness::builder()
            .with_size(egui::vec2(480.0, 320.0))
            .themed_ui_state(
                // The result's object; the notice's count; whether the notice moves to a place of
                // its own for each count.
                |ui, (name, count, moving): &mut (&'static str, usize, bool)| {
                    let name = *name;
                    let _ = PlacedResult::new(
                        crate::sentence!("Removed “{}” from tonight.", name),
                        UndoStep::of(Verb::Remove, name).pressed_as(1),
                    )
                    .show(ui);
                    let notice = |ui: &mut Ui| {
                        let _ = BodyMessage::from(
                            Notice::new(crate::sentence!("{} sessions wait.", *count))
                                .choice(Choice::new(MessageChoice::Primary("New session"), 2))
                                .choice(Choice::new(MessageChoice::Choice("Open the plan"), 3)),
                        )
                        .show(ui);
                    };
                    if *moving {
                        ui.push_id(("moving", *count), notice);
                    } else {
                        notice(ui);
                    }
                },
                ("Kaya", 0, false),
            );
        h.run_steps(4);
        assert!(
            h.query_by_label("Undo remove Kaya").is_some()
                && h.query_by_label("New session").is_some()
                && h.query_by_label("Open the plan").is_some(),
            "body-reached: the result draws its Undo and the notice its choices"
        );
        // The whole host, from its entry through paint: a quiet frame measures no text and lays
        // out no action's words. Red: an Undo fitted, or a choice measured or laid, every frame.
        let host = host_work();
        h.run_steps(5);
        assert_eq!(
            host_work(),
            host,
            "body-quiet: an unchanged frame with body messages measures no text and lays no words"
        );
        // A change of the words at the same length: the body is laid again, and the painted
        // Undo words are the new ones (read from the paint, not from the accessible name).
        let painted = |h: &Harness<'_, (&'static str, usize, bool)>, words: &str| {
            h.output().shapes.iter().any(|clipped| {
                matches!(&clipped.shape, egui::Shape::Text(text) if text.galley.job.text == words)
            })
        };
        assert!(painted(&h, "Undo remove Kaya"), "body-paint-reached");
        h.state_mut().0 = "Bram";
        h.run_steps(4);
        assert!(
            painted(&h, "Undo remove Bram") && !painted(&h, "Undo remove Kaya"),
            "body-same-length: the result paints its new Undo words"
        );
        // Review of PR 925 (P2): the kept values are bounded. Fifty different notices in one
        // place take its one slot, each replacing the last: the store's count stays flat. Red: a
        // slot keyed by the words' scope (`BodyMessage::show`'s), one more for each text.
        let entries = |h: &Harness<'_, (&'static str, usize, bool)>| body_kept_entries(&h.ctx);
        let before = entries(&h);
        for count in 1..=50 {
            h.state_mut().1 = count;
            h.run_steps(1);
        }
        assert!(
            before > 0 && painted(&h, "50 sessions wait."),
            "body-store-reached: the fiftieth notice drew, {before} kept values"
        );
        assert_eq!(
            entries(&h),
            before,
            "body-store-place: fifty notices in one place keep one slot"
        );
        // Fifty notices, each at a place of its own: a slot no message used in the last frame
        // goes (`BodyStore::sweep`), so the store holds at most one more than before. Red: no
        // sweep, one more for each place.
        h.state_mut().2 = true;
        for count in 51..=100 {
            h.state_mut().1 = count;
            h.run_steps(1);
        }
        assert!(
            painted(&h, "100 sessions wait.") && entries(&h) <= before + 1,
            "body-store-sweep: fifty places keep at most one more slot than before: {before} then {}",
            entries(&h)
        );
    }

    #[test]
    fn an_undo_shows_its_short_form_when_narrow_and_keeps_its_whole_name() {
        use egui_kittest::kittest::NodeT;
        use pilot::*;
        let visible = |h: &Harness<'_, App>, name: &str| {
            let node = h.get_by_label(name);
            let first = name.split(' ').next().unwrap_or_default();
            let shown: Vec<String> = h
                .output()
                .shapes
                .iter()
                .filter_map(|clipped| match &clipped.shape {
                    egui::Shape::Text(text)
                        if text.galley.job.text.starts_with(first)
                            && node
                                .rect()
                                .contains(text.pos + text.galley.rect.center().to_vec2()) =>
                    {
                        Some(text.galley.job.text.clone())
                    }
                    _ => None,
                })
                .collect();
            (node.accesskit_node().id(), shown)
        };
        // Two Undos of one verb (a lone one follows `UndoStep::alone`, below), so the footer
        // must tell them apart.
        let discard = || {
            let mut app = two();
            app.older_line = OlderLine::Discard("Bridge Ambush");
            app.same_verb = true;
            app
        };
        let mut narrow = discard();
        narrow.narrow = Some(230.0);
        let mut h = harness_sized(narrow, 960.0);
        let (_, shown) = visible(&h, "Undo discard of Bridge Ambush");
        assert_eq!(shown, ["Undo discard"], "narrow: the short form shows");
        h.get_by_label("Undo discard of Bridge Ambush").click();
        h.run_steps(2);
        assert_eq!(h.state().acted, [Ok("Campaign link saved")]);
        let h = harness_sized(discard(), 960.0);
        let (_, shown) = visible(&h, "Undo discard of Bridge Ambush");
        assert_eq!(
            shown,
            ["Undo discard of Bridge Ambush"],
            "wide: the whole form"
        );
        // Two Undos of one verb with different objects: each paints its whole form, so the
        // short form never hides the only difference between two visible Undos (a fight
        // planned from two sessions, lane B1).
        let (_, other) = visible(&h, "Undo discard of Shrine of Cinders");
        assert_eq!(
            other,
            ["Undo discard of Shrine of Cinders"],
            "two of one verb: the other paints its whole form too"
        );
        // Ruling M9 (it replaces round 30 FE11's lone short form): the only Undo of its verb in
        // its footer measures its whole form first. Wide, it paints the whole form; narrow,
        // where that does not fit, `Undo remove` without its object (its result names it),
        // never a cut object. Its accessible name stays whole, and its press reaches its entry.
        let lone = |line| {
            let mut app = two();
            app.older_line = line;
            app
        };
        let h = harness_sized(lone(OlderLine::Discard("Bridge Ambush")), 960.0);
        let (_, shown) = visible(&h, "Undo discard of Bridge Ambush");
        assert_eq!(
            shown,
            ["Undo discard of Bridge Ambush"],
            "lone, wide: the whole form"
        );
        let long = "Bridge Ambush at the Ravenholm Ironside Crossing";
        let name = format!("Undo remove {long}");
        let h = harness_sized(lone(OlderLine::Remove(long)), 960.0);
        assert_eq!(
            visible(&h, &name).1,
            [name.as_str()],
            "lone, wide: a long object whole"
        );
        let mut narrow = lone(OlderLine::Remove(long));
        narrow.narrow = Some(320.0);
        let mut h = harness_sized(narrow, 960.0);
        assert_eq!(
            visible(&h, &name).1,
            ["Undo remove"],
            "lone, narrow: the object goes whole, never cut"
        );
        h.get_by_label(&name).click();
        h.run_steps(2);
        assert_eq!(h.state().acted, [Ok("Campaign link saved")]);
        // Round 30 final review F2: one verb with and without an object in one footer. The
        // step with an object is not alone, so it keeps its object beside the other.
        let mut mixed = two();
        mixed.older_line = OlderLine::MixedAdd;
        let h = harness_sized(mixed, 960.0);
        let (_, scene) = visible(&h, "Undo add scene");
        let (_, bare) = visible(&h, "Undo add");
        assert_eq!(
            (scene, bare),
            (
                vec!["Undo add scene".to_owned()],
                vec!["Undo add".to_owned()]
            ),
            "mixed: Undo add scene beside Undo add"
        );
        // A long object with no joining word beside another Undo of its verb: cut by whole
        // words in the visible words only, at the width where the lone one gives it up.
        let mut narrow = two();
        narrow.older_line = OlderLine::Remove(long);
        narrow.same_verb = true;
        narrow.narrow = Some(320.0);
        let h = harness_sized(narrow, 960.0);
        let (_, shown) = visible(&h, &name);
        assert!(
            shown.len() == 1
                && shown[0].starts_with("Undo remove Bridge")
                && shown[0].ends_with('…'),
            "the object is cut in the visible words: {shown:?}"
        );
        // A footer choice whose words hold two objects from data (package R's counterexample):
        // narrow, each object is cut and the fixed words stay; the name keeps the whole words.
        const MOVE: &str = "Move Ghost of the Mill before Ember Cultist";
        // One result, so the footer shows the whole line without scrolling.
        let one = || {
            let mut app = App::default();
            app.dndbeyond
                .results
                .record(Saved(Campaign("older"), "Campaign link saved"));
            app.moves = Some(["Ghost of the Mill", "Ember Cultist"]);
            app
        };
        let mut narrow = one();
        narrow.narrow = Some(260.0);
        let mut h = harness_sized(narrow, 960.0);
        let (_, shown) = visible(&h, MOVE);
        assert!(
            shown.len() == 1
                && shown[0].starts_with("Move Gho")
                && shown[0].contains("… before Emb")
                && shown[0].ends_with('…'),
            "the objects are cut, the fixed words stay: {shown:?}"
        );
        h.get_by_label(MOVE).click();
        h.run_steps(2);
        assert_eq!(
            h.state().renamed,
            1,
            "its press comes back as the caller's id"
        );
        let h = harness_sized(one(), 960.0);
        assert_eq!(visible(&h, MOVE).1, [MOVE], "wide: the whole words");
        // Migration-32 L2c: a choice with a shorter form (Arrivals' Move) shows that form whole
        // where the whole words do not fit, before any object is cut; its name keeps the whole
        // words. Narrower than the form, the form's objects are cut.
        const WHOLE: &str = "Move Shrine Reserve (Goblin Warrior 3) before Brannoch Vale";
        const SHORT: &str = "Move Shrine Reserve before Brannoch Vale";
        let short = |width: f32| {
            let mut app = one();
            app.moves = Some(["Shrine Reserve (Goblin Warrior 3)", "Brannoch Vale"]);
            app.move_short = Some("Shrine Reserve");
            app.narrow = Some(width);
            harness_sized(app, 960.0)
        };
        assert_eq!(
            visible(&short(900.0), WHOLE).1,
            [WHOLE],
            "room: the whole words"
        );
        assert_eq!(
            visible(&short(380.0), WHOLE).1,
            [SHORT],
            "short of the whole words: the shorter form, whole"
        );
        let cut = visible(&short(240.0), WHOLE).1;
        assert!(
            cut.len() == 1 && cut[0].starts_with("Move Shr") && !cut[0].contains('('),
            "short of the form: the form's objects are cut, never the parenthesis: {cut:?}"
        );

        // Review of PR 332, 3: two objects with one prefix are told apart by the tip, on hover
        // and on keyboard focus, whenever the visible words are cut.
        let painted = |h: &Harness<'_, App>, words: &str| {
            h.output().shapes.iter().any(|clipped| {
                matches!(&clipped.shape, egui::Shape::Text(text) if text.galley.job.text == words)
            })
        };
        const ALIKE: &str = "Move Ghost north before Ghost south";
        let mut narrow = one();
        narrow.moves = Some(["Ghost north", "Ghost south"]);
        narrow.narrow = Some(260.0);
        let mut h = harness_sized(narrow, 960.0);
        h.ctx
            .all_styles_mut(|style| style.interaction.tooltip_delay = 0.0);
        let shown = visible(&h, ALIKE).1;
        assert!(
            shown.len() == 1
                && shown[0].matches('…').count() == 2
                && !shown[0].contains("north")
                && !shown[0].contains("south"),
            "both objects cut alike: {shown:?}"
        );
        assert!(!painted(&h, ALIKE), "no tip before hover or focus");
        let at = h.get_by_label(ALIKE).rect().center();
        h.event(egui::Event::PointerMoved(at));
        h.run_steps(4);
        assert!(painted(&h, ALIKE), "hover shows the whole words");
        h.event(egui::Event::PointerMoved(egui::pos2(1.0, 1.0)));
        h.run_steps(4);
        assert!(!painted(&h, ALIKE), "the tip goes with the pointer");
        h.get_by_label(ALIKE).focus();
        h.run_steps(4);
        assert!(painted(&h, ALIKE), "keyboard focus shows the whole words");

        // Review of PR 332, 2: a red choice with data is drawn red, exactly `L` after the safe
        // choice before it, after the safe ones though it was given first; its press comes back.
        let mut app = one();
        app.moves = Some(["Ash", "Elm"]);
        app.deletes = true;
        let mut h = harness_sized(app, 960.0);
        let (delete, moving) = (
            h.get_by_label("Delete Ash").rect(),
            h.get_by_label("Move Ash before Elm").rect(),
        );
        assert!(
            (delete.top() - moving.top()).abs() < 0.5
                && (delete.left() - moving.right() - L).abs() < 0.5,
            "exactly L after the safe choice: {moving:?} {delete:?}"
        );
        let red = h.output().shapes.iter().any(|clipped| {
            matches!(&clipped.shape, egui::Shape::Text(text)
                if text.galley.job.text == "Delete Ash"
                    && text.galley.job.sections.iter().all(|section| section.format.color == colors().destructive))
        });
        assert!(red, "the destructive colour");
        h.get_by_label("Delete Ash").click();
        h.run_steps(2);
        assert_eq!(h.state().deleted, 1);

        // Ruling M8: a footer choice says what it does in a help line under it, painted, as a
        // body fault's repair does (`action_help`), wide and narrow: each choice on a line of its
        // own from the words, its help under it at its words, the red one exactly `M` below the
        // safe one's help, measured from the help's own bounds, one line wide and wrapped narrow
        // (review of PR 487, 3); each help is its choice's accessible description (review of
        // PR 487, 1); the press still comes back.
        const MOVE_HELP: &str =
            "Elm keeps its place in the notes; Ash moves to the line before it.";
        const DELETE_HELP: &str = "Its file goes to the bin.";
        // Painted whole inside its clip, left of `right`: a line cut off by the footer's edge is
        // not shown.
        let whole = |h: &Harness<'_, App>, words: &str, right: f32| {
            h.output().shapes.iter().any(|clipped| {
                matches!(&clipped.shape, egui::Shape::Text(text)
                if text.galley.job.text == words
                    && text.pos.x < right
                    && clipped.clip_rect.expand(0.5).contains_rect(
                        Rect::from_min_size(text.pos, text.galley.size())
                    ))
            })
        };
        let helped = || {
            let mut app = one();
            app.moves = Some(["Ash", "Elm"]);
            app.deletes = true;
            app.helps = true;
            app
        };
        for narrow in [None, Some(320.0)] {
            let mut app = helped();
            app.narrow = narrow;
            let mut h = harness_sized(app, 960.0);
            let rect = |h: &Harness<'_, App>, name: &str| h.get_by_label(name).rect();
            let help = |h: &Harness<'_, App>, words: &str| {
                h.get_by_role_and_label(egui::accesskit::Role::Label, words)
                    .rect()
            };
            let (moving, delete) = (rect(&h, "Move Ash before Elm"), rect(&h, "Delete Ash"));
            let (move_help, delete_help) = (help(&h, MOVE_HELP), help(&h, DELETE_HELP));
            assert!(
                whole(&h, MOVE_HELP, f32::INFINITY) && whole(&h, DELETE_HELP, f32::INFINITY),
                "{narrow:?}: the help is painted whole, not only a hover"
            );
            assert!(
                move_help.top() >= moving.bottom() - 1.0
                    && (move_help.left() - moving.left()).abs() < 1.0,
                "{narrow:?}: Move's help under it, at its words: {moving:?} {move_help:?}"
            );
            assert_eq!(
                move_help.height() > 1.5 * Type::Small.line(),
                narrow.is_some(),
                "{narrow:?}: Move's help wraps only narrow: {move_help:?}"
            );
            assert!(
                (delete.top() - move_help.bottom() - M).abs() < 0.5
                    && delete_help.top() >= delete.bottom() - 1.0,
                "{narrow:?}: Delete exactly M below Move's help, its help under it: {move_help:?} {delete:?} {delete_help:?}"
            );
            for (name, description) in [
                ("Move Ash before Elm", Some(MOVE_HELP)),
                ("Delete Ash", Some(DELETE_HELP)),
                ("Register as a new campaign", None),
            ] {
                assert_eq!(
                    h.get_by_label(name)
                        .accesskit_node()
                        .description()
                        .as_deref(),
                    description,
                    "{narrow:?}: {name}'s accessible description is its help"
                );
            }
            h.get_by_label("Delete Ash").click();
            h.run_steps(2);
            assert_eq!(h.state().deleted, 1, "{narrow:?}: its press comes back");
        }
        // Review of PR 487, 2: in a short pane whose footer scrolls, Tab to the lowest choice
        // brings it into view with its whole help line, not the choice alone.
        let mut app = helped();
        app.heights = Some([170.0, 760.0]);
        let mut h = harness_sized(app, 960.0);
        h.set_size(egui::vec2(960.0, 800.0));
        h.run_steps(8);
        assert!(
            !whole(&h, DELETE_HELP, 320.0),
            "the short pane's footer scrolls: Delete's help starts out of view"
        );
        let mut reached = false;
        for _ in 0..40 {
            h.key_press(egui::Key::Tab);
            h.run_steps(2);
            let focused = h
                .root()
                .children_recursive()
                .find(|node| node.accesskit_node().is_focused());
            if focused.is_some_and(|node| {
                node.accesskit_node().label().as_deref() == Some("Delete Ash")
                    && node.rect().left() < 320.0
            }) {
                reached = true;
                break;
            }
        }
        assert!(reached, "Tab reaches Delete in the short pane");
        h.run_steps(10);
        assert!(
            whole(&h, "Delete Ash", 320.0) && whole(&h, DELETE_HELP, 320.0),
            "Tab shows Delete and its whole help line"
        );
        // The same in a pane's body (`pane::reveal_focus`): Tab to a body fault's repair under
        // the footer brings it into view above the footer with its whole help line.
        const FAULT_HELP: &str = "The package opens in Library.";
        let mut app = helped();
        app.body_fault = true;
        app.body_tall = [520.0, 0.0];
        app.heights = Some([420.0, 760.0]);
        let mut h = harness_sized(app, 960.0);
        h.set_size(egui::vec2(960.0, 800.0));
        h.run_steps(8);
        assert!(
            !whole(&h, FAULT_HELP, 320.0),
            "the body scrolls: the repair's help starts out of view"
        );
        let mut reached = false;
        for _ in 0..40 {
            h.key_press(egui::Key::Tab);
            h.run_steps(2);
            let focused = h
                .root()
                .children_recursive()
                .find(|node| node.accesskit_node().is_focused());
            if focused.is_some_and(|node| {
                node.accesskit_node().label().as_deref() == Some("Open in Library")
            }) {
                reached = true;
                break;
            }
        }
        assert!(reached, "Tab reaches the body repair");
        h.run_steps(10);
        assert!(
            whole(&h, "Open in Library", 320.0) && whole(&h, FAULT_HELP, 320.0),
            "Tab shows the repair and its whole help line above the footer"
        );
    }

    /// M3 gap 3, in the pilot: a press made on the line before its entry changed in place (in
    /// the press's frame, by the body) still reaches the entry, and the action gets the revised
    /// value; the unrelated result stays, and the next frame shows the revised words.
    #[test]
    fn a_press_made_before_a_revision_reaches_the_revised_entry() {
        use pilot::*;
        let mut h = harness(two());
        let newer = order(h.state(), "newer");
        let at = undo(&h, "older").rect().center();
        pointer(&mut h, at, true);
        h.run_steps(1);
        h.state_mut().in_body = Some(InBody::Revise("older"));
        pointer(&mut h, at, false);
        h.run_steps(1);
        assert_eq!(h.state().acted, [Ok("Campaign link moved")]);
        h.run_steps(2);
        assert_eq!(order(h.state(), "newer"), newer, "B keeps its order");
        assert!(
            h.query_by_role_and_label(egui::accesskit::Role::Label, "Folder saved.")
                .is_some(),
            "B stays"
        );
        // Revised, not pressed: the footer shows the revised words in the same place.
        let mut h = harness(two());
        let top = undo(&h, "older").rect().top();
        h.state_mut().in_body = Some(InBody::Revise("older"));
        h.run_steps(3);
        let label = |h: &Harness<'_, App>, words: &str| {
            h.query_by_role_and_label(egui::accesskit::Role::Label, words)
                .is_some()
        };
        assert!(label(&h, "Campaign link moved."));
        assert!(!label(&h, "Campaign link saved."));
        assert_eq!(undo(&h, "older").rect().top(), top, "the same place");
    }

    /// M3 gap 2: a body fault's repairs. The verb words are the app's; the object comes from
    /// data. A press comes back once, as the caller's id; a locked repair paints its reason and
    /// gives nothing back; a long object is cut inside the fault's width at 320 px while the
    /// accessible name keeps the whole words; each help line sits under its repair.
    /// N0 (ruling M21): a message holds at most one primary choice.
    #[test]
    #[should_panic(expected = "a message has at most one primary choice")]
    #[allow(
        clippy::disallowed_methods,
        reason = "the test of the body place `BodyMessage::show` itself"
    )]
    fn a_message_has_at_most_one_primary_choice() {
        let ctx = egui::Context::default();
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let _ = BodyMessage::from(
                    Notice::new(crate::sentence!("No session yet."))
                        .choice(Choice::new(MessageChoice::Primary("New session"), 1))
                        .choice(Choice::new(MessageChoice::Primary("Record again"), 2)),
                )
                .show(ui);
            });
        });
    }

    #[test]
    #[allow(
        clippy::disallowed_methods,
        reason = "the test of the body places `BodyFault::show` and `BodyMessage::show` themselves"
    )]
    fn a_body_fault_gives_its_repairs_back_by_id() {
        use crate::design::test_harness::Themed as _;
        use crate::design::test_harness::ThemedPage as _;
        use egui_kittest::kittest::NodeT;
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        enum Fix {
            Open,
            Replace,
            Repair(u8),
            Record,
            Log,
            Dismiss,
        }
        #[derive(Default)]
        struct State {
            locked: bool,
            pressed: Vec<Fix>,
            /// The right edge of the fault's 320 px column.
            right: f32,
            /// Issue #526: the warning replaced in its place by another one.
            replaced: bool,
        }
        const PATH: &str =
            "Packages/Ravenholm Ironside Wardens/Encounters/Bridge Ambush/manifest.md";
        const LOCK: Sentence = crate::sentence!("Save or discard campaign changes first.");
        const LONG: &str = "Ravenholm Ironside Wardens of the Northern Bridge Crossing";
        let mut h = Harness::builder()
            .with_size(egui::vec2(960.0, 600.0))
            .themed_ui_state(
                |ui, state: &mut State| {
                    let fault = Hp(Severity::NeedsFixing, None);
                    ui.allocate_ui_with_layout(
                        egui::vec2(320.0, 600.0),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            state.right = ui.max_rect().right();
                            let pressed = BodyFault::new(&fault)
                                .help(crate::sentence!("Check the packages it uses."))
                                .help(crate::sentence!("Or remove it from the group."))
                                .repair(Repair::new("Open in Library", Fix::Open))
                                .repair(
                                    Repair::new(
                                        crate::choice!("Replace with a basic {}", PATH),
                                        Fix::Replace,
                                    )
                                    .help(crate::sentence!("Your entries stay."))
                                    .locked_when(state.locked, LOCK),
                                )
                                .repair(Repair::new(
                                    crate::choice!("Repair {}", "Cinder Wolf"),
                                    Fix::Repair(7),
                                ))
                                .repair(Repair::new(
                                    crate::choice!("Repair {} in Library", LONG),
                                    Fix::Repair(8),
                                ))
                                .show(ui);
                            state.pressed.extend(pressed);
                            // A typed message in the body (`BodyMessage`): a warning with its
                            // primary choice, a link and its close icon.
                            let words = if state.replaced {
                                crate::sentence!("Bram’s sheet changed.")
                            } else {
                                crate::sentence!("Kaya’s sheet changed.")
                            };
                            let pressed = BodyMessage::from(
                                Notice::warning(words)
                                    .choice(Choice::new(
                                        MessageChoice::Primary("Record again"),
                                        Fix::Record,
                                    ))
                                    .choice(Choice::new(
                                        MessageChoice::Link {
                                            word: "Log",
                                            hover: None,
                                        },
                                        Fix::Log,
                                    ))
                                    .choice(Choice::new(
                                        MessageChoice::Dismiss(Lost::Nothing),
                                        Fix::Dismiss,
                                    )),
                            )
                            .show(ui);
                            state.pressed.extend(pressed);
                            // A typed fault with its help line (M-P0 item 9), and a choice whose
                            // lock is open (issue #527: its hidden reason keeps its room).
                            let _ = BodyMessage::<Fix>::from(
                                FaultMessage::new(&Hp(Severity::CannotRead, Some(3)))
                                    .help(crate::sentence!("Read the row again after you fix it."))
                                    .choice(
                                        Choice::new(
                                            MessageChoice::Link {
                                                word: "Read again",
                                                hover: None,
                                            },
                                            Fix::Open,
                                        )
                                        .locked_when(
                                            false,
                                            crate::sentence!("Wait for the sheet to load."),
                                        ),
                                    ),
                            )
                            .show(ui);
                        },
                    );
                },
                State::default(),
            );
        h.run_steps(4);
        // M-P0 item 9: a typed body fault's help line, under its words and before its repairs, at
        // their column; a typed fault message's, under its words, at them.
        let label = |h: &Harness<'_, State>, words: &str| {
            h.get_by_role_and_label(egui::accesskit::Role::Label, words)
                .rect()
        };
        let (words, help, open) = (
            label(&h, "Cinder Wolf: The HP is missing."),
            label(&h, "Check the packages it uses."),
            h.get_by_label("Open in Library").rect(),
        );
        // Review of PR 677: every help line, in call order.
        let second = label(&h, "Or remove it from the group.");
        assert!(
            help.top() >= words.bottom() - 1.0
                && second.top() >= help.bottom() - 0.5
                && second.bottom() <= open.top() + 1.0
                && (help.left() - open.left()).abs() < 1.0
                && (second.left() - open.left()).abs() < 1.0,
            "the fault's help lines under its words, in call order, before its repairs: \
             {words:?} {help:?} {second:?} {open:?}"
        );
        let (words, help) = (
            label(&h, "Cinder Wolf · line 3: The HP is missing."),
            label(&h, "Read the row again after you fix it."),
        );
        assert!(
            help.top() >= words.bottom() - 1.0 && (help.left() - words.left()).abs() < 1.0,
            "the fault message's help under its words, at them: {words:?} {help:?}"
        );
        let name = format!("Replace with a basic {PATH}");
        let replace = h.get_by_label(&name).rect();
        assert!(
            replace.right() <= h.state().right + 0.5 && replace.width() < 320.0,
            "inside the fault's width: {replace:?}"
        );
        let help = h
            .get_by_role_and_label(egui::accesskit::Role::Label, "Your entries stay.")
            .rect();
        assert!(
            help.top() >= replace.bottom() - 1.0,
            "help under its repair"
        );
        // Review of PR 487, 1: the help is its repair's accessible description; a repair with
        // no help has none.
        let description = |h: &Harness<'_, State>, name: &str| {
            h.get_by_label(name).accesskit_node().description()
        };
        assert_eq!(
            description(&h, &name).as_deref(),
            Some("Your entries stay."),
            "the help is the repair's description"
        );
        assert_eq!(description(&h, "Repair Cinder Wolf"), None);
        // The fixed words after a cut object stay; the name keeps the whole words.
        let whole = format!("Repair {LONG} in Library");
        let node = h.get_by_label(&whole).rect();
        let shown: Vec<String> = h
            .output()
            .shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Text(text)
                    if text.galley.job.text.starts_with("Repair")
                        && node.contains(text.pos + text.galley.rect.center().to_vec2()) =>
                {
                    Some(text.galley.job.text.clone())
                }
                _ => None,
            })
            .collect();
        assert!(
            shown.len() == 1
                && shown[0].starts_with("Repair Rav")
                && shown[0].ends_with("… in Library"),
            "the object is cut, the fixed words stay: {shown:?}"
        );
        h.get_by_label("Repair Cinder Wolf").click();
        h.run_steps(2);
        assert_eq!(
            h.state().pressed,
            [Fix::Repair(7)],
            "once, as the caller's id"
        );
        h.get_by_label(&name).click();
        h.run_steps(2);
        assert_eq!(h.state().pressed, [Fix::Repair(7), Fix::Replace]);
        // Locked: disabled, its reason painted, nothing given back; nothing above it moves.
        let open = h.get_by_label("Repair Cinder Wolf").rect().top();
        h.state_mut().locked = true;
        h.state_mut().pressed.clear();
        h.run_steps(3);
        assert!(h.get_by_label(&name).accesskit_node().is_disabled());
        assert!(
            h.query_by_role_and_label(egui::accesskit::Role::Label, LOCK.as_str())
                .is_some()
        );
        assert_eq!(
            description(&h, &name).as_deref(),
            Some("Save or discard campaign changes first. Your entries stay."),
            "locked: the lock's reason, then the help"
        );
        assert_eq!(
            h.get_by_label("Repair Cinder Wolf").rect().top(),
            open,
            "a lock moves nothing"
        );
        h.get_by_label(&name).click();
        h.run_steps(2);
        assert!(h.state().pressed.is_empty());
        // A typed message in the body (`BodyMessage`, M-R0's Run rows): in its notice's tone; its
        // link and its close icon give back the caller's ids, once each.
        let words = h
            .get_by_role_and_label(egui::accesskit::Role::Label, "Kaya’s sheet changed.")
            .rect();
        let tones: Vec<Tone> = message_rects(&h.ctx)
            .into_iter()
            .filter(|(rect, _, _)| rect.contains(words.center()))
            .map(|(_, _, tone)| tone)
            .collect();
        assert_eq!(tones, [Tone::Warning], "the notice's tone");
        // N0 (ruling M21): its primary choice is filled, under the words at their first letter.
        let record = h.get_by_label("Record again").rect();
        let filled = h.output().shapes.iter().any(|clipped| {
            matches!(&clipped.shape, egui::Shape::Rect(rect)
                if rect.fill == colors().accent_fill && rect.rect.expand(1.0).contains_rect(record))
        });
        assert!(
            filled
                && record.top() >= words.bottom() - 1.0
                && (record.left() - words.left()).abs() <= 1.0,
            "n0-primary-choice: the primary is filled, under the words at their first letter: {record:?} {words:?}"
        );
        h.get_by_label("Record again").click();
        h.run_steps(2);
        h.get_by_label("Log").click();
        h.run_steps(2);
        h.get_by_label(DISMISS_NAME).click();
        h.run_steps(2);
        assert_eq!(
            h.state().pressed,
            [Fix::Record, Fix::Log, Fix::Dismiss],
            "each press once"
        );
        // Issue #527: quiet frames lay out no hidden reason; the open lock keeps its room.
        let quiet = footer_work();
        h.run_steps(3);
        assert_eq!(
            footer_work().laid - quiet.laid,
            0,
            "#527: a message outside a footer keeps its hidden reason's room"
        );
        // Issue #526: the warning is replaced in its place while its close icon has focus. Enter
        // reaches no control of the new warning.
        h.get_by_label(DISMISS_NAME).focus();
        h.run_steps(1);
        h.state_mut().replaced = true;
        h.run_steps(1);
        h.key_press(egui::Key::Enter);
        h.run_steps(2);
        assert_eq!(
            h.state().pressed,
            [Fix::Record, Fix::Log, Fix::Dismiss],
            "#526: a replaced warning takes no press meant for the old one's focused control"
        );

        // The compact form (M-P0 item 7: the Run's header band and setup rows), at 900 and in a
        // 320 px column. A warning with `Log` and its close icon: the words at a row message's
        // line, `Log` and the icon beside them on that line in a band as wide as they are, the
        // whole message one control high; each press comes back. A fault with its file line:
        // on the words' row when it fits (900), under them at the words when not (320).
        for width in [900.0, 320.0] {
            let mut h = Harness::builder()
                .with_size(egui::vec2(960.0, 400.0))
                .themed_ui_state(
                    move |ui, pressed: &mut Vec<Fix>| {
                        ui.allocate_ui_with_layout(
                            egui::vec2(width, 400.0),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                let notice = BodyMessage::from(
                                    Notice::warning(crate::sentence!("Kaya’s sheet changed."))
                                        .choice(Choice::new(
                                            MessageChoice::Link {
                                                word: "Log",
                                                hover: None,
                                            },
                                            Fix::Log,
                                        ))
                                        .choice(Choice::new(
                                            MessageChoice::Dismiss(Lost::Nothing),
                                            Fix::Dismiss,
                                        )),
                                )
                                .compact()
                                .show(ui);
                                pressed.extend(notice);
                                let fault = BodyMessage::<Fix>::from(
                                    FaultMessage::new(&Hp(Severity::CannotRead, Some(20)))
                                        .file(FileLine::new("/vault", "Encounters/Shrine.md")),
                                )
                                .compact()
                                .show(ui);
                                pressed.extend(fault);
                                // With help: its help lines, its file line, then its choice.
                                let fault = BodyMessage::<Fix>::from(
                                    FaultMessage::new(&Hp(Severity::CannotRead, Some(21)))
                                        .help(crate::sentence!("Set the HP in the file."))
                                        .help(crate::sentence!("Then read the group again."))
                                        .file(FileLine::new("/vault", "Encounters/Other.md"))
                                        .choice(Choice::new(
                                            MessageChoice::Link {
                                                word: "Read again",
                                                hover: None,
                                            },
                                            Fix::Open,
                                        )),
                                )
                                .compact()
                                .show(ui);
                                pressed.extend(fault);
                            },
                        );
                    },
                    Vec::new(),
                );
            h.run_steps(4);
            let words = h
                .get_by_role_and_label(egui::accesskit::Role::Label, "Kaya’s sheet changed.")
                .rect();
            let (log, close) = (
                h.get_by_label("Log").rect(),
                h.get_by_label(DISMISS_NAME).rect(),
            );
            assert!(
                (words.height() - Type::Small.line()).abs() < 1.0,
                "{width}: the words at a row message's line: {words:?}"
            );
            assert!(
                // The band starts one item gap after the words, its choices one item gap apart:
                // as wide as they are, not stretched to the column.
                (log.left() - words.right() - h.ctx.style().spacing.item_spacing.x).abs() < 1.5
                    && (close.left() - log.right() - h.ctx.style().spacing.item_spacing.x).abs()
                        < 1.5
                    && (log.center().y - words.center().y).abs() < 2.0
                    && (close.center().y - words.center().y).abs() < 2.0,
                "{width}: Log and the close icon beside the words, in a fitted band: {words:?} {log:?} {close:?}"
            );
            let row = message_rects(&h.ctx)
                .into_iter()
                .find(|(rect, _, _)| rect.contains(words.center()))
                .map(|(rect, _, _)| rect)
                .expect("the notice is registered");
            assert!(
                row.height() <= Density::Read.control() + 0.5,
                "{width}: the band is one control high: {row:?}"
            );
            let cause = h
                .get_by_role_and_label(
                    egui::accesskit::Role::Label,
                    "Cinder Wolf · line 20: The HP is missing.",
                )
                .rect();
            // M-P0 item 9, review of PR 677: a compact fault with help draws, under its words and
            // at them, its help lines in call order, its file line, then its choice, at every
            // width, as the full form does.
            let label = |words: &str| {
                h.get_by_role_and_label(egui::accesskit::Role::Label, words)
                    .rect()
            };
            let (helped, first, second) = (
                label("Cinder Wolf · line 21: The HP is missing."),
                label("Set the HP in the file."),
                label("Then read the group again."),
            );
            let other = h
                .query_all(
                    egui_kittest::kittest::By::new().predicate(|node| {
                        node.data().label() == Some("/vault/Encounters/Other.md")
                    }),
                )
                .next()
                .expect("the helped fault's file line")
                .rect();
            let again = h.get_by_label("Read again").rect();
            assert!(
                first.top() >= helped.bottom() - 1.0
                    && second.top() >= first.bottom() - 0.5
                    && other.top() >= second.bottom() - 0.5
                    && again.top() >= other.bottom() - 0.5
                    && [first, second, other, again]
                        .iter()
                        .all(|rect| (rect.left() - helped.left()).abs() < 1.0),
                "{width}: words, help, help, file line, then the choice, at the words: \
                 {helped:?} {first:?} {second:?} {other:?} {again:?}"
            );
            // The path's node by the name the part gave it (`named`).
            let path = h
                .query_all(
                    egui_kittest::kittest::By::new().predicate(|node| {
                        node.data().label() == Some("/vault/Encounters/Shrine.md")
                    }),
                )
                .next()
                .expect("the file line's path")
                .rect();
            assert_eq!(
                (path.center().y - cause.center().y).abs() < 2.0,
                width > 600.0,
                "{width}: the file line on the words' row only where it fits: {cause:?} {path:?}"
            );
            if width < 600.0 {
                assert!(
                    path.top() >= cause.bottom() - 0.5 && (path.left() - cause.left()).abs() < 1.0,
                    "{width}: under the words, at the words: {cause:?} {path:?}"
                );
            }
            // Review of PR 592 (P2, `m-p0-compact-measure`): unchanged frames measure no compact
            // message's words.
            let measured = COMPACT_MEASURES.with(std::cell::Cell::get);
            h.run_steps(3);
            assert_eq!(
                COMPACT_MEASURES.with(std::cell::Cell::get),
                measured,
                "{width}: unchanged frames measure no compact words"
            );
            h.get_by_label("Log").click();
            h.run_steps(2);
            h.get_by_label(DISMISS_NAME).click();
            h.run_steps(2);
            h.get_by_label("Read again").click();
            h.run_steps(2);
            assert_eq!(
                h.state(),
                &[Fix::Log, Fix::Dismiss, Fix::Open],
                "{width}: each press once"
            );
        }

        // Review of PR 592 (`m-p0-compact-refuses`): the compact form draws no choice's help line
        // and no lock's reason line, so a test build refuses a compact message with either.
        for (case, choice) in [
            (
                "help",
                Choice::new(MessageChoice::Choice("Read again"), 1)
                    .help(crate::sentence!("Reads the file again.")),
            ),
            (
                "lock",
                Choice::new(MessageChoice::Choice("Read again"), 1)
                    .blocked(crate::sentence!("Wait for the read.")),
            ),
        ] {
            let mut choice = Some(choice);
            let refused = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let ctx = egui::Context::default();
                let _ = ctx.run(egui::RawInput::default(), |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        if let Some(choice) = choice.take() {
                            let _ = BodyMessage::from(
                                Notice::new(crate::sentence!("The file changed.")).choice(choice),
                            )
                            .compact()
                            .show(ui);
                        }
                    });
                });
            }));
            let message = refused
                .err()
                .and_then(|panic| {
                    panic
                        .downcast_ref::<String>()
                        .cloned()
                        .or_else(|| panic.downcast_ref::<&str>().map(|said| (*said).to_owned()))
                })
                .unwrap_or_default();
            assert!(
                message.contains("a compact message draws no choice help and no lock reason"),
                "m-p0-compact-refuses: a compact message with a choice's {case} is refused: {message:?}"
            );
        }

        // N0 (ruling M21): a fault's words can carry what the system itself said, on hover
        // (`FaultMessage::hover`: a recorder's own error); the words stay the app's.
        const SAID: &str = "The recorder said: device busy.";
        let mut h = Harness::builder()
            .with_size(egui::vec2(600.0, 300.0))
            .themed_ui(|ui| {
                let fault = Hp(Severity::CannotRead, None);
                let _ = BodyMessage::from(
                    FaultMessage::new(&fault)
                        .hover(SAID.to_owned())
                        .choice(Choice::new(MessageChoice::Choice("Record again"), 1_u8)),
                )
                .show(ui);
            });
        h.run_steps(2);
        let fault_words = fault_words(&Hp(Severity::CannotRead, None));
        h.get_by_role_and_label(egui::accesskit::Role::Label, fault_words.as_str())
            .hover();
        h.run_steps(30);
        let said = h.output().shapes.iter().any(|clipped| {
            matches!(&clipped.shape, egui::Shape::Text(text) if text.galley.job.text == SAID)
        });
        assert!(
            said,
            "n0-fault-hover: the words' hover says what the system said"
        );

        // Review of PR 332, 4: narrower than the shortest cut, the words wrap inside the row;
        // every fixed word is painted, none is cut.
        const WHOLE: &str = "Repair Ravenholm in Library";
        let mut h = Harness::builder()
            .with_size(egui::vec2(400.0, 300.0))
            .themed_ui_state(
                |ui, right: &mut f32| {
                    let fault = Hp(Severity::NeedsFixing, None);
                    ui.allocate_ui_with_layout(
                        egui::vec2(150.0, 300.0),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            *right = ui.max_rect().right();
                            let _ = BodyFault::<u8>::new(&fault)
                                .repair(Repair::new(
                                    crate::choice!("Repair {} in Library", "Ravenholm"),
                                    1,
                                ))
                                .show(ui);
                        },
                    );
                },
                0.0,
            );
        h.run_steps(4);
        let node = h.get_by_label(WHOLE).rect();
        let wrapped = h.output().shapes.iter().any(|clipped| {
            matches!(&clipped.shape, egui::Shape::Text(text)
                if text.galley.job.text == WHOLE && text.galley.rows.len() > 1 && !text.galley.elided)
        });
        assert!(wrapped, "the whole words, wrapped, nothing cut");
        assert!(node.right() <= *h.state() + 0.5, "inside the row: {node:?}");

        // M-P0 item 6 (Library's `Register a copy`, M0346): a fault whose one choice opens an
        // inline creation form in its place. Closed, its choice and no form; the choice opens
        // the form under the fault and its title; the form's Cancel closes it; locked, the
        // choice opens nothing.
        #[derive(Default)]
        struct Form {
            open: bool,
            locked: bool,
            name: String,
            opened: usize,
        }
        const REGISTER: crate::design::layout::Word = crate::word!("Register");
        const CANCEL: crate::design::layout::Word = crate::word!("Cancel");
        let mut h = Harness::builder()
            .with_size(egui::vec2(960.0, 600.0))
            .themed_ui_state(
                |ui, form: &mut Form| {
                    let fault = Hp(Severity::NeedsFixing, None);
                    let mut opener =
                        FormOpener::new(&fault, "Register as a new package", &mut form.open)
                            .title("Register a copy");
                    if form.locked {
                        opener = opener.blocked(LOCK);
                    }
                    let shown = opener.show(
                        ui,
                        crate::design::layout::InlineCreate::new()
                            .field(crate::design::layout::CreateField::new(
                                crate::field_id!("test_opener_name")
                                    .once(crate::design::layout::Once::Example),
                                "New package name",
                                &mut form.name,
                            ))
                            .create(
                                crate::design::layout::Create::primary(REGISTER)
                                    .blocked(None::<crate::design::layout::NoReason>),
                            )
                            .kept_line(crate::design::layout::KeptLine::new().cancel(CANCEL, true)),
                    );
                    form.opened += usize::from(shown.opened);
                },
                Form::default(),
            );
        h.run_steps(3);
        // A caption and its field share a name: any node with it.
        let has = |h: &Harness<'_, Form>, name: &str| h.query_all_by_label(name).next().is_some();
        assert!(
            has(&h, "Register as a new package") && !has(&h, "New package name"),
            "closed: the choice, no form"
        );
        h.get_by_label("Register as a new package").click();
        h.run_steps(3);
        assert!(
            h.state().open
                && h.state().opened == 1
                && has(&h, "New package name")
                && has(&h, "Register")
                && !has(&h, "Register as a new package"),
            "the choice opens the form in its place"
        );
        assert!(
            h.query_by_role_and_label(
                egui::accesskit::Role::Label,
                "Cinder Wolf: The HP is missing."
            )
            .is_some(),
            "the fault stays above the open form"
        );
        h.get_by_label("Cancel").click();
        h.run_steps(3);
        assert!(
            !h.state().open && has(&h, "Register as a new package") && !has(&h, "New package name"),
            "Cancel closes the form"
        );
        // Review of PR 612 (P2): the keyboard goes back to the choice that opened the form,
        // after Cancel and after Escape in the form's field.
        let choice_focused = |h: &Harness<'_, Form>| {
            h.get_by_label("Register as a new package")
                .accesskit_node()
                .is_focused()
        };
        assert!(
            choice_focused(&h),
            "after Cancel, the choice has the keyboard"
        );
        h.get_by_label("Register as a new package").click();
        h.run_steps(3);
        h.get_all_by_label("New package name")
            .find(|node| node.accesskit_node().role() == egui::accesskit::Role::TextInput)
            .expect("the form's field")
            .click();
        h.run_steps(2);
        h.key_press(egui::Key::Escape);
        h.run_steps(3);
        assert!(
            !h.state().open && choice_focused(&h),
            "Escape in the field closes the form and gives the keyboard back to the choice"
        );
        h.state_mut().locked = true;
        h.run_steps(2);
        h.get_by_label("Register as a new package").click();
        h.run_steps(3);
        assert!(
            !h.state().open && h.state().opened == 2,
            "locked, the choice opens nothing"
        );
    }

    /// The pane's one press: the first in the footer's drawing order wins, and a second press
    /// in the same frame is dropped.
    /// The chosen count's one form, with the app's digit grouping.
    #[test]
    fn a_chosen_count_groups_its_digits() {
        assert_eq!(chosen_count(0, 999, "entries"), "0 of 999 entries chosen");
        assert_eq!(chosen_count(999, 1000, ""), "999 of 1,000 chosen");
        assert_eq!(
            chosen_count(1204, 2310, "entries"),
            "1,204 of 2,310 entries chosen"
        );
    }

    #[test]
    fn a_pane_gives_back_at_most_one_press_the_first() {
        let sink = PressSink::<u8>(std::rc::Rc::default());
        sink.press(Press {
            id: 1,
            ticket: None,
        });
        sink.press(Press {
            id: 2,
            ticket: None,
        });
        assert_eq!(sink.0.borrow_mut().take().map(|press| press.id), Some(1));
        // Inside one message's drawing pass, the same: the first activation is kept.
        let mut pressed = None;
        first(&mut pressed, Pressed::Undo);
        first(&mut pressed, Pressed::Choice(0));
        assert_eq!(pressed, Some(Pressed::Undo));
        // The whole path: several controls activated in one pass, the pane gives back the first
        // in drawing order (the newer line, then the older; in a line its Undo, its choices,
        // then its close icon).
        use pilot::*;
        let older = "Campaign link saved.";
        let newer = "Folder saved.";
        let mut app = two();
        app.two_choices = true;
        let mut h = harness(app);
        for (activate, expect) in [
            (
                vec![(older, Pressed::Choice(1)), (older, Pressed::Choice(0))],
                (Act::Register, "older"),
            ),
            (
                vec![(older, Pressed::Choice(1)), (older, Pressed::Undo)],
                (Act::Undo, "older"),
            ),
            (
                vec![(newer, Pressed::Choice(0)), (newer, Pressed::Undo)],
                (Act::Undo, "newer"),
            ),
            (
                vec![(older, Pressed::Undo), (newer, Pressed::Choice(0))],
                (Act::Dismiss, "newer"),
            ),
        ] {
            let at = order(h.state(), expect.1);
            h.state_mut().presses.clear();
            h.state_mut().acted.clear();
            let list: Vec<(String, Pressed)> = activate
                .iter()
                .map(|(words, what)| ((*words).to_owned(), *what))
                .collect();
            h.ctx
                .data_mut(|data| data.insert_temp(egui::Id::new(TEST_ACTIVATIONS), list));
            h.step();
            h.ctx.data_mut(|data| {
                data.remove::<Vec<(String, Pressed)>>(egui::Id::new(TEST_ACTIVATIONS))
            });
            assert_eq!(
                h.state().presses,
                [(0, expect.0, at)],
                "activated {activate:?}"
            );
            // An Undo acted: record the entry again for the next case.
            if !h.state().acted.is_empty() {
                *h.state_mut() = {
                    let mut app = two();
                    app.two_choices = true;
                    app
                };
                h.run_steps(2);
            }
        }
    }

    /// The pilot's pane (package C, Campaign → Link and folder), as a fixture: the footer shows
    /// the store's kept results while the body changes the store's owner through `&mut`. Each
    /// press comes back with the store's ticket of the line pressed, and the store acts by it.
    mod pilot {
        use super::super::*;
        use crate::design::results::{
            DiskChanged, Ends, Kept, Owner, OwnerChanged, Press as Kind, Results,
        };
        use egui;
        use egui_kittest::{
            Harness,
            kittest::{NodeT, Queryable},
        };

        #[derive(Clone, PartialEq, Eq, Debug)]
        pub(super) struct Campaign(pub &'static str);
        impl Owner for Campaign {
            type Scope = str;
            fn scope(&self) -> &str {
                "root"
            }
        }
        pub(super) struct Saved(pub Campaign, pub &'static str);
        impl Kept for Saved {
            type Owner = Campaign;
            const ENDS: Ends = Ends {
                owner_changed: OwnerChanged::Ends,
                disk_changed: DiskChanged::Stays,
                press: Kind::Undo,
            };
            fn owner(&self) -> Campaign {
                self.0.clone()
            }
        }
        /// The pilot's `DndBeyond`: the store's owner, which the body changes.
        #[derive(Default)]
        pub(super) struct DndBeyond {
            pub results: Results<Campaign, Saved>,
            pub edits: usize,
        }
        /// Only the kind of action: which result it is, the ticket says.
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub(super) enum Act {
            Undo,
            Register,
            Rename,
            Dismiss,
            Delete,
        }
        /// What a change inside the body does in the next frame drawn.
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub(super) enum InBody {
            Nothing,
            Replace(&'static str),
            End(&'static str),
            Add(&'static str),
            /// M3 gap 3: the owner's kept value changes in place (its file moved).
            Revise(&'static str),
        }
        /// What the older line says (M3 gap 1): the pilot's `Said`, or a typed Undo step.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
        pub(super) enum OlderLine {
            #[default]
            Said,
            /// `<name> draft discarded.` with `Undo discard of <name>`.
            Discard(&'static str),
            /// `Removed <name>.` with `Undo remove <name>`.
            Remove(&'static str),
            /// `Day 2 added.` with `Undo add` (no object), and on the newer line `Scene added.`
            /// with `Undo add scene`: one verb, with and without an object (the Session plan).
            MixedAdd,
            /// `Ash is not linked to this campaign.` with its choice `Link to this campaign`
            /// and no Undo: a kept offer (Library's link offer).
            Offer,
        }
        /// Settings' page lock (a round 26 review repair): long enough to wrap in a narrow pane.
        pub(super) const LOCK: Sentence =
            crate::sentence!("End the run first. Close the run to change campaigns.");
        #[derive(Default)]
        pub(super) struct App {
            pub dndbeyond: DndBeyond,
            pub run_live: bool,
            /// The older line's Undo stays open while the run is live (its choice stays locked,
            /// with the same reason).
            pub older_undo_open: bool,
            pub in_body: Option<InBody>,
            /// The older line has a second choice after `Register as a new campaign`.
            pub two_choices: bool,
            /// Two panes side by side, each showing the same stored entries.
            pub two_panes: bool,
            /// The pane in a column this wide.
            pub narrow: Option<f32>,
            /// Two panes side by side, 320 wide, of these heights.
            pub heights: Option<[f32; 2]>,
            /// Each pane's body is this much taller than its label (a tall body).
            pub body_tall: [f32; 2],
            /// The first pane's body ends with a body fault whose repair has a help line (M8).
            pub body_fault: bool,
            /// The panes' style draws body text in this font family.
            pub family: Option<&'static str>,
            /// No Undo or choice has a lock (no reason, open or held, and no room for one).
            pub no_locks: bool,
            /// The next frame asks egui to draw it again (a second pass), once.
            pub discard: bool,
            /// The panes' owner (a campaign, say): its footers are other instances.
            pub owner: u8,
            /// Passes drawn.
            pub passes: usize,
            /// What the older line says.
            pub older_line: OlderLine,
            /// The newer line has the older line's verb too (about Shrine of Cinders), so two
            /// Undos of one verb stand in the footer.
            pub same_verb: bool,
            /// The older line has a choice whose words hold two objects from data (M3).
            pub moves: Option<[&'static str; 2]>,
            /// The Move choice has a shorter form: its first object given as this.
            pub move_short: Option<&'static str>,
            /// It also has a red choice with data, given before `Move` (the part puts it after).
            pub deletes: bool,
            /// Its `Move` and `Delete` say what they do, in help lines (ruling M8).
            pub helps: bool,
            pub deleted: usize,
            /// What each Undo press did to the store: the words it took back, or `Gone`.
            pub acted: Vec<Result<&'static str, &'static str>>,
            /// Each press: the pane, the kind, and its ticket's order.
            pub presses: Vec<(usize, Act, Option<ResultOrder>)>,
            pub registered: usize,
            pub renamed: usize,
        }

        /// The pilot's `show_active`: it needs the owner by `&mut`.
        #[allow(
            clippy::disallowed_methods,
            reason = "the pilot's by-owner revision, in the frame of its read"
        )]
        fn show_active(
            ui: &mut Ui,
            dndbeyond: &mut DndBeyond,
            change: InBody,
            tall: f32,
            fault: bool,
        ) {
            dndbeyond.edits += 1;
            match change {
                InBody::Nothing => {}
                InBody::Replace(owner) => dndbeyond
                    .results
                    .record(Saved(Campaign(owner), "Campaign link saved again")),
                InBody::End(owner) => dndbeyond.results.owner_changed(&Campaign(owner)),
                InBody::Add(owner) => dndbeyond
                    .results
                    .record(Saved(Campaign(owner), "Folder chosen")),
                InBody::Revise(owner) => {
                    let _ = dndbeyond
                        .results
                        .revise(&Campaign(owner), |saved| saved.1 = "Campaign link moved");
                }
            }
            ui.label("Link and folder");
            if tall > 0.0 {
                let _ = ui.allocate_exact_size(egui::vec2(10.0, tall), egui::Sense::hover());
            }
            if fault {
                let _ = BodyFault::new(&super::Hp(Severity::NeedsFixing, None))
                    .repair(
                        Repair::new("Open in Library", Act::Rename)
                            .help(crate::sentence!("The package opens in Library.")),
                    )
                    .show(ui);
            }
        }

        /// One pane: its footer shows the store's kept lines; its body changes the owner.
        fn one_pane(ui: &mut Ui, app: &mut App, pane: usize) {
            let live = app.run_live;
            let no_locks = app.no_locks;
            if std::mem::take(&mut app.discard) {
                ui.ctx().request_discard("the pilot draws this frame again");
            }
            let older_undo_open = app.older_undo_open;
            let older_line = app.older_line;
            let same_verb = app.same_verb;
            let frame = PaneFrame::new(ui);
            let mut messages =
                Messages::new(&frame, ("pilot", pane, app.owner), ui.available_width());
            for owner in ["older", "newer", "third"] {
                if let Some(shown) = app.dndbeyond.results.get(&Campaign(owner)) {
                    let two_choices = app.two_choices;
                    let moves = app.moves;
                    let move_short = app.move_short;
                    let deletes = app.deletes;
                    let helps = app.helps;
                    let line = shown.outcome(&frame, |saved| {
                        match (owner, older_line) {
                            ("newer", OlderLine::Discard(_)) if same_verb => {
                                return KeptOutcome::Notice(
                                    Notice::new(crate::sentence!(
                                        "Shrine of Cinders draft discarded."
                                    )),
                                    UndoStep::of(Verb::Discard, "Shrine of Cinders")
                                        .pressed_as(Act::Undo),
                                );
                            }
                            ("newer", OlderLine::Remove(_)) if same_verb => {
                                return Done::new(Verb::Remove, "Shrine of Cinders")
                                    .undo(Act::Undo)
                                    .into();
                            }
                            ("newer", OlderLine::MixedAdd) => {
                                return KeptOutcome::Notice(
                                    Notice::new(crate::sentence!("Scene added.")),
                                    UndoStep::of(Verb::Add, "scene").pressed_as(Act::Undo),
                                );
                            }
                            ("older", OlderLine::Offer) => {
                                return Offer::new(
                                    crate::sentence!("Ash is not linked to this campaign."),
                                    Choice::new(
                                        MessageChoice::Choice("Link to this campaign"),
                                        Act::Register,
                                    ),
                                )
                                .into();
                            }
                            (_, OlderLine::Said) | ("newer" | "third", _) => {}
                            ("older", OlderLine::Discard(name)) => {
                                return KeptOutcome::Notice(
                                    Notice::new(crate::sentence!("{} draft discarded.", name)),
                                    UndoStep::of(Verb::Discard, name).pressed_as(Act::Undo),
                                );
                            }
                            (_, OlderLine::Remove(name)) => {
                                return Done::new(Verb::Remove, name).undo(Act::Undo).into();
                            }
                            ("older", OlderLine::MixedAdd) => {
                                return KeptOutcome::Notice(
                                    Notice::new(crate::sentence!("Day 2 added.")),
                                    UndoStep::verb(Verb::Add).pressed_as(Act::Undo),
                                );
                            }
                            _ => {}
                        }
                        let undo = UndoStep::verb(Verb::Change)
                            .tell_apart(owner)
                            .pressed_as(Act::Undo);
                        let undo = if no_locks {
                            undo
                        } else {
                            undo.locked_when(live && !(owner == "older" && older_undo_open), LOCK)
                        };
                        let mut said = Said::new(crate::sentence!("{}.", saved.1)).undo(undo);
                        if owner == "older" {
                            // A choice attached to a kept result.
                            let register = Choice::new(
                                MessageChoice::Choice("Register as a new campaign"),
                                Act::Register,
                            );
                            said = said.choice(if no_locks {
                                register
                            } else {
                                register.locked_when(live, LOCK)
                            });
                            if let Some([a, b]) = moves {
                                if deletes {
                                    let mut delete = Choice::named_as(
                                        ChoiceKind::Lose,
                                        crate::choice!("Delete {}", a),
                                        Act::Delete,
                                    );
                                    if helps {
                                        delete = delete
                                            .help(crate::sentence!("Its file goes to the bin."));
                                    }
                                    said = said.choice(delete);
                                }
                                let words = crate::choice!("Move {} before {}", a, b);
                                let words = match move_short {
                                    Some(short) => words.or_shorter([crate::choice!(
                                        "Move {} before {}",
                                        short,
                                        b
                                    )]),
                                    None => words,
                                };
                                let mut moving = Choice::named(words, Act::Rename);
                                if helps {
                                    // Long enough to wrap in a 320 px pane.
                                    moving = moving.help(crate::sentence!(
                                        "{} keeps its place in the notes; {} moves to the line before it.",
                                        b,
                                        a
                                    ));
                                }
                                said = said.choice(moving);
                            }
                            if two_choices {
                                said = said.choice(Choice::new(
                                    MessageChoice::Choice("Rename the campaign"),
                                    Act::Rename,
                                ));
                            }
                        }
                        if owner == "newer" {
                            // N6: the part places the close icon.
                            said = said.choice(
                                Choice::new(MessageChoice::Dismiss(Lost::Nothing), Act::Dismiss)
                                    .locked_when(live, LOCK),
                            );
                        }
                        said.into()
                    });
                    messages.kept(line);
                }
            }
            let change = if pane == 0 {
                app.in_body.take().unwrap_or(InBody::Nothing)
            } else {
                InBody::Nothing
            };
            // The body changes the store's owner by `&mut` while the footer holds its lines:
            // the gap the pilot found (E0502 with a borrowing entry).
            let tall = app.body_tall[pane];
            let fault = app.body_fault && pane == 0;
            let paned = messages.pane(ui, crate::design::Body::Content, |ui| {
                show_active(ui, &mut app.dndbeyond, change, tall, fault);
                7
            });
            assert_eq!(paned.body, 7);
            if let Some(press) = paned.pressed {
                app.presses
                    .push((pane, press.id, press.ticket.as_ref().map(Ticket::order)));
                match press.id {
                    Act::Undo => {
                        let ticket = press.ticket.expect("a kept line's press has its ticket");
                        app.acted.push(
                            match app.dndbeyond.results.undo_by(ticket, |_| Ok::<_, ()>(())) {
                                Ok((saved, ())) => Ok(saved.1),
                                Err(ByTicket::Gone) => Err("gone"),
                                Err(ByTicket::Failed(())) => Err("failed"),
                            },
                        );
                    }
                    Act::Register => app.registered += 1,
                    Act::Rename => app.renamed += 1,
                    Act::Dismiss => {}
                    Act::Delete => app.deleted += 1,
                }
            }
        }

        pub(super) fn frame(ui: &mut Ui, app: &mut App) {
            app.passes += 1;
            if let Some(family) = app.family {
                use crate::design::tokens::Type;
                ui.style_mut().text_styles.insert(
                    Type::Body.style(),
                    egui::FontId::new(Type::Body.size(), egui::FontFamily::Name(family.into())),
                );
            }
            if let Some(heights) = app.heights {
                ui.horizontal_top(|ui| {
                    for (pane, height) in heights.into_iter().enumerate() {
                        ui.allocate_ui_with_layout(
                            egui::vec2(320.0, height),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                ui.set_max_height(height);
                                one_pane(ui, app, pane);
                            },
                        );
                    }
                });
            } else if app.two_panes {
                ui.columns(2, |columns| {
                    for (pane, column) in columns.iter_mut().enumerate() {
                        one_pane(column, app, pane);
                    }
                });
            } else if let Some(width) = app.narrow {
                let height = ui.available_height();
                ui.allocate_ui_with_layout(
                    egui::vec2(width, height),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| one_pane(ui, app, 0),
                );
            } else {
                one_pane(ui, app, 0);
            }
        }

        /// The order of the stored entry of `owner`.
        pub(super) fn order(app: &App, owner: &'static str) -> Option<ResultOrder> {
            app.dndbeyond
                .results
                .get(&Campaign(owner))
                .map(|shown| shown.order())
        }

        pub(super) fn harness(app: App) -> Harness<'static, App> {
            harness_sized(app, 700.0)
        }

        pub(super) fn harness_sized(app: App, width: f32) -> Harness<'static, App> {
            use crate::design::test_harness::Themed as _;
            let mut h = Harness::builder()
                .with_size(egui::vec2(width, 500.0))
                .themed_ui_state(frame, app);
            h.run_steps(4);
            h
        }

        pub(super) fn label_left(h: &Harness<'_, App>, words: &str) -> f32 {
            h.get_by_role_and_label(egui::accesskit::Role::Label, words)
                .rect()
                .left()
        }

        pub(super) fn undo<'h>(
            h: &'h Harness<'_, App>,
            owner: &'static str,
        ) -> egui_kittest::Node<'h> {
            h.get_by_label(match owner {
                "older" => "Undo change older",
                "newer" => "Undo change newer",
                _ => "Undo change third",
            })
        }

        pub(super) fn disabled(node: &egui_kittest::Node<'_>) -> bool {
            node.accesskit_node().is_disabled()
        }

        pub(super) fn description(node: &egui_kittest::Node<'_>) -> Option<String> {
            node.accesskit_node().description()
        }

        /// The primary button down or up at `at`.
        pub(super) fn pointer(h: &mut Harness<'_, App>, at: egui::Pos2, down: bool) {
            h.event(egui::Event::PointerMoved(at));
            h.event(egui::Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed: down,
                modifiers: egui::Modifiers::NONE,
            });
        }

        /// Two activations in one frame from two sources: the release of a pointer press on
        /// `pointed` (pressed in the frame before), and an AccessKit click on `clicked` in the
        /// same frame (egui takes one AccessKit click per pass).
        pub(super) fn two_activations(h: &mut Harness<'_, App>, pointed: &str, clicked: &str) {
            let at = h.get_by_label(pointed).rect().center();
            let target = h.get_by_label(clicked).accesskit_node().id();
            pointer(h, at, true);
            h.step();
            h.event(egui::Event::AccessKitActionRequest(
                egui::accesskit::ActionRequest {
                    action: egui::accesskit::Action::Click,
                    target,
                    data: None,
                },
            ));
            pointer(h, at, false);
            h.step();
            h.run_steps(2);
        }

        pub(super) fn two() -> App {
            let mut app = App::default();
            // Recorded at the action: the older link first.
            app.dndbeyond
                .results
                .record(Saved(Campaign("older"), "Campaign link saved"));
            app.dndbeyond
                .results
                .record(Saved(Campaign("newer"), "Folder saved"));
            app
        }
    }

    /// Two kept results newest first; each Undo, and a choice on a kept result, come back with
    /// their own ticket, and the store acts on the entry each line showed. Undo starts under its
    /// message's words and stays inside its message.
    #[test]
    fn owned_messages_give_presses_back_by_ticket_beside_a_body_that_changes_the_app() {
        use pilot::*;
        let mut h = harness(two());
        assert!(h.state().dndbeyond.edits > 0, "the body changed the owner");
        let top = |h: &egui_kittest::Harness<'_, App>, words: &str| {
            h.get_by_role_and_label(egui::accesskit::Role::Label, words)
                .rect()
                .top()
        };
        assert!(
            top(&h, "Folder saved.") < top(&h, "Campaign link saved."),
            "the newer result comes first"
        );
        let rects = message_rects(&h.ctx);
        assert!(
            rects.len() == 2
                && rects
                    .iter()
                    .all(|(_, place, _)| *place == MessagePlace::Footer),
            "{rects:?}"
        );
        // The older line has a choice, so its Undo stands under its words, from them, inside it.
        let older = undo(&h, "older").rect();
        let words = label_left(&h, "Campaign link saved.");
        assert!(
            older.left() < words && older.right() > words && rects[1].0.contains_rect(older),
            "Undo starts its line under the words, inside the message: {older:?} {words} {rects:?}"
        );
        undo(&h, "older").click();
        h.run_steps(2);
        assert_eq!(h.state().acted, [Ok("Campaign link saved")]);
        undo(&h, "newer").click();
        h.run_steps(2);
        assert_eq!(
            h.state().acted,
            [Ok("Campaign link saved"), Ok("Folder saved")]
        );
        assert!(h.state().dndbeyond.results.is_empty());
        let mut h = harness(two());
        h.get_by_label("Register as a new campaign").click();
        h.run_steps(2);
        assert_eq!((h.state().registered, h.state().acted.len()), (1, 0));
        // A kept offer (Library's link offer): its words and choice in the info tone, no Undo,
        // after the result whose Undo is live; its press comes back with its entry's ticket.
        let mut app = two();
        app.older_line = OlderLine::Offer;
        let mut h = harness(app);
        const OFFER: &str = "Ash is not linked to this campaign.";
        assert!(
            top(&h, "Folder saved.") < top(&h, OFFER),
            "after the live Undo"
        );
        let offer = h
            .get_by_role_and_label(egui::accesskit::Role::Label, OFFER)
            .rect();
        let tones: Vec<Tone> = message_rects(&h.ctx)
            .into_iter()
            .filter(|(rect, _, _)| rect.contains(offer.center()))
            .map(|(_, _, tone)| tone)
            .collect();
        assert_eq!(tones, [Tone::Info], "the offer is in the info tone");
        assert!(
            h.query_by_label("Undo change older").is_none(),
            "an offer has no Undo"
        );
        let older = order(h.state(), "older");
        h.get_by_label("Link to this campaign").click();
        h.run_steps(2);
        assert_eq!(
            h.state().presses,
            [(0, Act::Register, older)],
            "the offer's press comes back with its entry's ticket"
        );
        assert_eq!((h.state().registered, h.state().acted.len()), (1, 0));
    }

    /// A line made before the body replaced or ended its result: its press reaches nothing, the
    /// other result stays, and the next frame shows no old words.
    #[test]
    fn a_press_on_a_replaced_or_ended_result_reaches_nothing() {
        use pilot::*;
        for change in [InBody::Replace("older"), InBody::End("older")] {
            let mut h = harness(two());
            let at = undo(&h, "older").rect().center();
            pointer(&mut h, at, true);
            h.run_steps(1);
            // In the frame of the release, the body replaces or ends the result first, then the
            // footer draws the line it was given (its old ticket) and takes the press.
            h.state_mut().in_body = Some(change);
            pointer(&mut h, at, false);
            h.run_steps(1);
            // The old line's press reaches nothing: no press, or the store says it is gone.
            let acted = &h.state().acted;
            assert!(
                acted.is_empty() || acted == &[Err("gone")],
                "{change:?}: {acted:?}"
            );
            h.run_steps(3);
            let results = &h.state().dndbeyond.results;
            assert!(results.get(&Campaign("newer")).is_some(), "B stays");
            match change {
                InBody::Replace(_) => assert!(
                    results.get(&Campaign("older")).is_some()
                        && h.query_by_label("Campaign link saved again.").is_some()
                        && h.query_by_label("Campaign link saved.").is_none(),
                    "the replacement shows, and the old words do not"
                ),
                _ => assert!(
                    results.get(&Campaign("older")).is_none()
                        && h.query_by_label("Campaign link saved.").is_none()
                ),
            }
        }
    }

    /// Each line's controls are keyed by its ticket: a result added above moves neither the
    /// keyboard focus nor a pending pointer press onto another result's Undo.
    #[test]
    fn focus_and_a_pending_press_stay_with_their_result() {
        use pilot::*;
        // Focus on the older Undo, then a newer result comes above it: Enter takes back the
        // older one.
        let mut h = harness(two());
        undo(&h, "older").focus();
        h.run_steps(2);
        let focused = h.ctx.memory(|memory| memory.focused());
        assert!(focused.is_some(), "the older Undo has focus");
        h.state_mut().in_body = Some(InBody::Add("third"));
        h.run_steps(4);
        assert!(h.query_by_label("Folder chosen.").is_some());
        assert_eq!(
            h.ctx.memory(|memory| memory.focused()),
            focused,
            "focus stays on the older Undo"
        );
        h.key_press(egui::Key::Enter);
        h.run_steps(2);
        assert_eq!(h.state().acted, [Ok("Campaign link saved")]);
        // A press held on the older Undo while a newer result comes above it: the release acts on
        // the older one or on nothing, never on another result.
        let mut h = harness(two());
        let at = undo(&h, "older").rect().center();
        pointer(&mut h, at, true);
        h.run_steps(1);
        h.state_mut().in_body = Some(InBody::Add("third"));
        h.run_steps(3);
        pointer(&mut h, at, false);
        h.run_steps(2);
        let acted = &h.state().acted;
        assert!(
            acted.is_empty() || acted == &[Ok("Campaign link saved")],
            "{acted:?}"
        );
        assert!(
            h.state()
                .dndbeyond
                .results
                .get(&Campaign("third"))
                .is_some()
                && h.state()
                    .dndbeyond
                    .results
                    .get(&Campaign("newer"))
                    .is_some(),
            "no other result was taken back"
        );
    }

    /// A locked Undo and a locked choice are disabled, say why in painted words (no hover
    /// needed) and in their description, and give nothing back.
    #[test]
    fn a_locked_undo_or_choice_paints_why_and_gives_nothing_back() {
        use pilot::*;
        let mut app = two();
        app.run_live = true;
        let mut h = harness(app);
        let register = h.get_by_label("Register as a new campaign");
        for node in [&register, &undo(&h, "older"), &undo(&h, "newer")] {
            assert!(disabled(node), "it is locked");
            assert_eq!(description(node).as_deref(), Some(LOCK.as_str()));
        }
        // A locked Dismiss (the close icon) is disabled too.
        assert!(
            disabled(&h.get_by_label(DISMISS_NAME)),
            "the close icon is locked"
        );
        assert!(
            h.query_all_by_role_and_label(egui::accesskit::Role::Label, LOCK.as_str())
                .count()
                >= 2,
            "each locked message paints its reason"
        );
        undo(&h, "older").click();
        h.get_by_label("Register as a new campaign").click();
        h.get_by_label(DISMISS_NAME).click();
        h.run_steps(2);
        assert!(
            h.state().acted.is_empty() && h.state().registered == 0 && h.state().presses.is_empty()
        );
        // Two locks of one message share a reason and come apart: the older line's Undo opens
        // while its choice stays locked. The shared reason stays painted for the choice.
        let painted = |h: &Harness<'_, pilot::App>| {
            h.output()
                .shapes
                .iter()
                .filter(|clipped| {
                    matches!(&clipped.shape, egui::Shape::Text(text) if text.galley.job.text == LOCK.as_str())
                })
                .count()
        };
        assert_eq!(painted(&h), 2, "both locked messages paint the reason");
        h.state_mut().older_undo_open = true;
        h.run_steps(2);
        assert!(!disabled(&undo(&h, "older")), "the older Undo is open");
        assert!(disabled(&h.get_by_label("Register as a new campaign")));
        assert_eq!(
            painted(&h),
            2,
            "the locked choice still paints the reason its open Undo shares"
        );
    }

    /// Activations queued for one frame: egui 0.33 delivers one click activation per pass (an
    /// AccessKit click, a pointer release and Enter each landed in their own pass in every
    /// arrangement tried), so each comes back as its own press, with its own kind and its own
    /// line's ticket, and acts only on its own entry. Inside one pass the first activation in
    /// drawing order wins (`a_pane_gives_back_at_most_one_press_the_first`).
    #[test]
    fn activations_queued_for_one_frame_each_reach_their_own_entry() {
        use pilot::*;
        let sorted = |mut presses: Vec<(usize, Act, Option<ResultOrder>)>| {
            presses.sort_by_key(|press| format!("{press:?}"));
            presses
        };
        // The older line's Undo and its choice.
        let mut h = harness(two());
        let older = order(h.state(), "older");
        two_activations(&mut h, "Undo change older", "Register as a new campaign");
        assert_eq!(
            sorted(h.state().presses.clone()),
            sorted(vec![(0, Act::Undo, older), (0, Act::Register, older)])
        );
        assert_eq!(h.state().acted, [Ok("Campaign link saved")]);
        assert!(
            h.state()
                .dndbeyond
                .results
                .get(&Campaign("newer"))
                .is_some()
        );
        // Two choices of one message.
        let mut app = two();
        app.two_choices = true;
        let mut h = harness(app);
        let older = order(h.state(), "older");
        two_activations(&mut h, "Rename the campaign", "Register as a new campaign");
        assert_eq!(
            sorted(h.state().presses.clone()),
            sorted(vec![(0, Act::Rename, older), (0, Act::Register, older)])
        );
        // The newer line's Undo and its close icon.
        let mut h = harness(two());
        let newer = order(h.state(), "newer");
        two_activations(&mut h, "Undo change newer", DISMISS_NAME);
        assert_eq!(
            sorted(h.state().presses.clone()),
            sorted(vec![(0, Act::Undo, newer), (0, Act::Dismiss, newer)])
        );
        assert_eq!(h.state().acted, [Ok("Folder saved")]);
        assert!(
            h.state()
                .dndbeyond
                .results
                .get(&Campaign("older"))
                .is_some()
        );
    }

    /// N6 (package X), on the typed path: the close icon sits at the right end of its message's
    /// first line, centred on it, at least 24 px, and its press comes back as `Dismiss` with
    /// that line's ticket.
    #[test]
    fn the_close_icon_sits_on_its_first_line_and_names_its_entry() {
        use pilot::*;
        let mut h = harness(two());
        let newer = order(h.state(), "newer");
        let dismiss = h.get_by_label(DISMISS_NAME).rect();
        let line = h
            .get_by_role_and_label(egui::accesskit::Role::Label, "Folder saved.")
            .rect();
        assert!(
            (dismiss.center().y - line.center().y).abs() <= 1.0
                && dismiss.left() > line.right()
                && dismiss.height() >= 24.0
                && dismiss.width() >= 24.0,
            "n6: icon {dismiss:?}, first line {line:?}"
        );
        h.get_by_label(DISMISS_NAME).click();
        h.run_steps(2);
        assert_eq!(h.state().presses, [(0, Act::Dismiss, newer)]);
    }

    /// One stored entry shown in two panes: two lines with their own control ids (no clash),
    /// a press comes back from the pane it was made in with the entry's ticket, and focus stays
    /// on its control when a result is added above.
    #[test]
    fn one_entry_in_two_panes_has_two_lines_with_their_own_ids() {
        use egui_kittest::kittest::NodeT;
        use pilot::*;
        let mut app = two();
        app.two_panes = true;
        let mut h = harness_sized(app, 1200.0);
        let ids: Vec<_> = h
            .get_all_by_label("Undo change older")
            .map(|node| node.accesskit_node().id())
            .collect();
        assert!(ids.len() == 2 && ids[0] != ids[1], "{ids:?}");
        let clashes = crate::design::layout::widget_id_clashes(&h.ctx, &h.output().shapes);
        assert!(clashes.is_empty(), "{clashes:?}");
        let older = order(h.state(), "older");
        h.get_all_by_label("Undo change older")
            .nth(1)
            .unwrap()
            .click();
        h.run_steps(2);
        assert_eq!(h.state().presses, [(1, Act::Undo, older)]);
        // Focus on the right pane's newer Undo, then a result comes above it.
        let mut app = two();
        app.two_panes = true;
        let mut h = harness_sized(app, 1200.0);
        h.get_all_by_label("Undo change newer")
            .nth(1)
            .unwrap()
            .focus();
        h.run_steps(2);
        let focused = h.ctx.memory(|memory| memory.focused());
        assert!(focused.is_some());
        h.state_mut().in_body = Some(InBody::Add("third"));
        h.run_steps(4);
        assert_eq!(h.ctx.memory(|memory| memory.focused()), focused);
        h.key_press(egui::Key::Enter);
        h.run_steps(2);
        assert_eq!(h.state().acted, [Ok("Folder saved")]);
        assert_eq!(h.state().presses.first().map(|press| press.0), Some(1));

        // Layout PR 2b (ruling L9): each pane's typed footer keeps its plan. Frames with no
        // change compare the lines and sort, join and measure nothing; a new width and a lock
        // that goes make the plan again, and the footer's room is what a fresh pane measures.
        // Review of PR 452, finding 2 (L1c): already in the first frame of the change, drawn in
        // one pass.
        use crate::design::pane::{Placement, pane_records};
        /// Each pane's footer and where it stands, left to right; where each kept line's words
        /// are painted, and whether whole inside their clip; each Undo's accessible bounds.
        type Shown = (
            Vec<(egui::Rect, Placement)>,
            Vec<(egui::Rect, bool)>,
            Vec<(String, egui::Rect)>,
        );
        fn shown(h: &egui_kittest::Harness<'_, App>) -> Shown {
            fn walk(shape: &egui::Shape, clip: egui::Rect, out: &mut Vec<(egui::Rect, bool)>) {
                match shape {
                    egui::Shape::Vec(shapes) => {
                        shapes.iter().for_each(|shape| walk(shape, clip, out));
                    }
                    egui::Shape::Text(text)
                        if ["Campaign link saved.", "Folder saved."]
                            .contains(&text.galley.text()) =>
                    {
                        let rect = egui::Rect::from_min_size(text.pos, text.galley.size());
                        out.push((rect, clip.expand(0.5).contains_rect(rect)));
                    }
                    _ => {}
                }
            }
            let mut panes: Vec<_> = pane_records(&h.ctx)
                .iter()
                .filter_map(|record| record.footer().map(|rect| (rect, record.placement())))
                .collect();
            panes.sort_by(|a, b| a.0.left().total_cmp(&b.0.left()));
            let mut words = Vec::new();
            for shape in &h.output().shapes {
                walk(&shape.shape, shape.clip_rect, &mut words);
            }
            words.sort_by(|a, b| {
                (a.0.left(), a.0.top())
                    .partial_cmp(&(b.0.left(), b.0.top()))
                    .expect("finite")
            });
            let mut undos: Vec<(String, egui::Rect)> = h
                .query_all_by_role(egui::accesskit::Role::Button)
                .filter_map(|node| {
                    let label = node.accesskit_node().label()?;
                    label
                        .starts_with("Undo change")
                        .then(|| (label.to_owned(), node.rect()))
                })
                .collect();
            undos.sort_by(|a, b| {
                (a.1.left(), a.1.top())
                    .partial_cmp(&(b.1.left(), b.1.top()))
                    .expect("finite")
            });
            (panes, words, undos)
        }
        /// The same, within a hundredth of a point.
        fn same(a: &Shown, b: &Shown) -> bool {
            let near = |a: egui::Rect, b: egui::Rect| {
                (a.min - b.min).length() < 0.01 && (a.max - b.max).length() < 0.01
            };
            a.0.len() == b.0.len()
                && a.1.len() == b.1.len()
                && a.2.len() == b.2.len()
                && a.0
                    .iter()
                    .zip(&b.0)
                    .all(|(a, b)| near(a.0, b.0) && a.1 == b.1)
                && a.1
                    .iter()
                    .zip(&b.1)
                    .all(|(a, b)| near(a.0, b.0) && a.1 == b.1)
                && a.2
                    .iter()
                    .zip(&b.2)
                    .all(|(a, b)| a.0 == b.0 && near(a.1, b.1))
        }
        /// A frame drawn in one pass (egui declines the rerun a changed footer asks for), then
        /// the next frame, in one pass too: what each shows.
        fn one_pass(h: &mut egui_kittest::Harness<'_, App>) -> (Shown, Shown) {
            let passes = h.ctx.options(|options| options.max_passes);
            h.ctx
                .options_mut(|options| options.max_passes = std::num::NonZeroUsize::MIN);
            let before = h.state().passes;
            h.step();
            let first = shown(h);
            h.step();
            let next = shown(h);
            h.ctx.options_mut(|options| options.max_passes = passes);
            assert_eq!(h.state().passes - before, 2, "two frames of one pass");
            (first, next)
        }
        /// A frame drawn in one pass, then a press at `at` decided on that frame's targets: the
        /// pointer goes down in the next frame and up in the one after, one pass each. What the
        /// first frame shows.
        fn press_after_one_pass(h: &mut egui_kittest::Harness<'_, App>, at: egui::Pos2) -> Shown {
            let passes = h.ctx.options(|options| options.max_passes);
            h.ctx
                .options_mut(|options| options.max_passes = std::num::NonZeroUsize::MIN);
            h.step();
            let first = shown(h);
            pointer(h, at, true);
            h.step();
            pointer(h, at, false);
            h.step();
            h.ctx.options_mut(|options| options.max_passes = passes);
            first
        }
        /// A fresh pane of `app` at `size`, settled: what it shows.
        fn settles(app: App, size: egui::Vec2) -> Shown {
            let mut h = harness_sized(app, size.x);
            h.set_size(size);
            h.run_steps(6);
            shown(&h)
        }
        fn panes(no_locks: bool) -> App {
            let mut app = two();
            app.two_panes = true;
            app.no_locks = no_locks;
            app
        }
        let footers = |h: &egui_kittest::Harness<'_, App>| -> Vec<f32> {
            let mut rects: Vec<egui::Rect> = crate::design::pane::pane_records(&h.ctx)
                .iter()
                .filter_map(|record| record.footer())
                .collect();
            rects.sort_by(|a, b| a.left().total_cmp(&b.left()));
            rects.iter().map(egui::Rect::height).collect()
        };
        let fresh = |width: f32, no_locks: bool| {
            let mut app = two();
            app.two_panes = true;
            app.no_locks = no_locks;
            let mut h = harness_sized(app, width);
            h.run_steps(6);
            footers(&h)
        };
        let mut app = two();
        app.two_panes = true;
        let mut h = harness_sized(app, 1200.0);
        h.run_steps(4);
        let quiet = footer_work();
        let host = host_work();
        let frame = h.ctx.cumulative_frame_nr();
        h.run_steps(6);
        let after = footer_work();
        // The whole host, from its entry through paint: the footers' lines (their words, their
        // Undos, their choices) measure no text and lay out no action's words on a quiet frame;
        // the plan keeps the choices' widths and the laid words.
        assert_eq!(
            host_work(),
            host,
            "footer-quiet: an unchanged frame with footer lines measures no text and lays no words"
        );
        let frames = h.ctx.cumulative_frame_nr() - frame;
        assert!(
            after.compared > quiet.compared,
            "the frames reached the footers"
        );
        // Review of PR 452 (re-check, P3): the plan store is swept once per frame, each kept
        // footer visited once, not once per footer that draws.
        let swept = (after.swept - quiet.swept) as u64;
        assert!(
            swept > 0 && swept <= frames * kept_footer_plans(&h.ctx) as u64,
            "the store is swept once per frame: {swept} visits in {frames} frames"
        );
        assert_eq!(
            (
                after.plans - quiet.plans,
                after.sorted - quiet.sorted,
                after.named - quiet.named,
                after.probes - quiet.probes,
                after.verbs - quiet.verbs,
                after.undo_fits - quiet.undo_fits,
                after.laid - quiet.laid,
            ),
            (0, 0, 0, 0, 0, 0, 0),
            "an unchanged frame sorts, names, counts Undo verbs, builds and fits no Undo label \
             and lays out no reservation (each kept Undo, each open lock's reason)"
        );
        let wide = footers(&h);
        assert_eq!(wide, fresh(1200.0, false));
        h.set_size(egui::vec2(700.0, 500.0));
        let (first, next) = one_pass(&mut h);
        let settled = settles(panes(false), egui::vec2(700.0, 500.0));
        assert!(
            same(&first, &settled),
            "the first frame at a new width is where a fresh pane settles: {first:?}, \
             settled {settled:?}"
        );
        assert!(same(&next, &first), "and the next moves nothing: {next:?}");
        h.run_steps(4);
        let replanned = footer_work();
        assert!(
            replanned.plans > after.plans
                && replanned.verbs > after.verbs
                && replanned.undo_fits > after.undo_fits
                && replanned.laid > after.laid,
            "a new width makes the plans again, counting the Undos' verbs, fitting their labels \
             and laying out the reservations once: {after:?} -> {replanned:?}"
        );
        let quiet = footer_work();
        h.run_steps(4);
        let still = footer_work();
        assert_eq!(
            (
                still.verbs - quiet.verbs,
                still.undo_fits - quiet.undo_fits,
                still.laid - quiet.laid
            ),
            (0, 0, 0),
            "and then no more"
        );
        let narrow = fresh(700.0, false);
        assert_ne!(narrow, wide, "the width changes the footers' room");
        assert_eq!(footers(&h), narrow, "at the new width, a fresh pane's room");
        h.state_mut().no_locks = true;
        let (first, next) = one_pass(&mut h);
        let settled = settles(panes(true), egui::vec2(700.0, 500.0));
        assert!(
            same(&first, &settled),
            "the first frame with the locks gone is where a fresh pane settles: {first:?}, \
             settled {settled:?}"
        );
        assert!(same(&next, &first), "and the next moves nothing: {next:?}");
        h.run_steps(4);
        let unlocked = fresh(700.0, true);
        assert_ne!(unlocked, narrow, "the locks' reasons hold room");
        assert_eq!(
            footers(&h),
            unlocked,
            "with the locks gone, a fresh pane's room"
        );

        // A footer that comes, then grows: its first frame, drawn in one pass, is where a fresh
        // pane settles it, its words whole as there; the next frame moves nothing. Beside short
        // bodies it follows them (two panes). In two panes 300 points tall it is pinned: beside a
        // tall body (left), and beside a body that fitted its pane until the footer came (right),
        // not cut short below that body.
        let lines = [("older", "Campaign link saved"), ("newer", "Folder saved")];
        for (heights, tall, size) in [
            (None, [0.0; 2], egui::vec2(1200.0, 500.0)),
            (Some([300.0; 2]), [1000.0, 230.0], egui::vec2(960.0, 800.0)),
        ] {
            let app = |kept: &[(&'static str, &'static str)]| {
                let mut app = App {
                    two_panes: heights.is_none(),
                    heights,
                    body_tall: tall,
                    ..App::default()
                };
                for &(owner, words) in kept {
                    app.dndbeyond.results.record(Saved(Campaign(owner), words));
                }
                app
            };
            let mut h = harness_sized(app(&[]), size.x);
            h.set_size(size);
            h.run_steps(4);
            assert!(shown(&h).0.is_empty(), "no footer yet");
            let fits: Vec<bool> = pane_records(&h.ctx)
                .iter()
                .map(crate::design::pane::PaneRecord::fits)
                .collect();
            for (at, &(owner, words)) in lines.iter().enumerate() {
                h.state_mut()
                    .dndbeyond
                    .results
                    .record(Saved(Campaign(owner), words));
                let (first, next) = one_pass(&mut h);
                let settled = settles(app(&lines[..=at]), size);
                let kept = at + 1;
                assert!(
                    same(&first, &settled),
                    "{heights:?}, {kept} kept: the first frame {first:?}, settled {settled:?}"
                );
                assert!(
                    same(&next, &first),
                    "{heights:?}, {kept} kept: the next frame {next:?}"
                );
                h.run_steps(4);
            }
            // The fixture: where the footers stand, and which body fitted its pane with none.
            let stand: Vec<Placement> = shown(&h).0.iter().map(|(_, stand)| *stand).collect();
            let want = if heights.is_some() {
                ([false, true], [Placement::Pinned; 2])
            } else {
                ([true, true], [Placement::Follows; 2])
            };
            assert_eq!(
                (fits.as_slice(), stand.as_slice()),
                (&want.0[..], &want.1[..])
            );
            // Its controls take presses where they are painted in the frame that brings them: a
            // third result comes, and a press on its Undo where a fresh pane settles it, decided
            // on that first frame's targets, reaches it.
            h.run_steps(2);
            h.state_mut()
                .dndbeyond
                .results
                .record(Saved(Campaign("third"), "Folder chosen"));
            let third = order(h.state(), "third");
            let settled = settles(app(&[lines[0], lines[1], ("third", "Folder chosen")]), size);
            let (_, undo) = settled
                .2
                .iter()
                .find(|(label, _)| label == "Undo change third")
                .expect("the third result's Undo, in the left pane");
            let before = h.state().presses.len();
            let first = press_after_one_pass(&mut h, undo.center());
            assert!(
                same(&first, &settled),
                "{heights:?}, 3 kept: the first frame {first:?}, settled {settled:?}"
            );
            assert_eq!(
                &h.state().presses[before..],
                [(0, Act::Undo, third)],
                "{heights:?}: a press on the new Undo, decided on its first frame's targets"
            );
        }

        // Review of PR 456 (re-review, P2): fonts set after the footers were planned reach their
        // plan and its measure, whoever sets them. Every family's first font replaced by a larger
        // one, then a new family that the panes' style picks for body text: the first frame,
        // drawn in one pass, is where a fresh pane with those fonts settles, and the next frame
        // moves nothing.
        for family in [None, Some("pilot")] {
            let mut h = harness_sized(panes(false), 1200.0);
            h.run_steps(4);
            let mut fonts = h.ctx.fonts(|view| view.definitions().clone());
            let mono = fonts.families[&egui::FontFamily::Monospace][0].clone();
            let mut larger = (*fonts.font_data[&mono]).clone();
            larger.tweak.scale = 1.5;
            fonts
                .font_data
                .insert("pilot larger".into(), std::sync::Arc::new(larger));
            match family {
                None => {
                    for faces in fonts.families.values_mut() {
                        faces.insert(0, "pilot larger".into());
                    }
                }
                Some(family) => {
                    fonts.families.insert(
                        egui::FontFamily::Name(family.into()),
                        vec!["pilot larger".into()],
                    );
                }
            }
            h.ctx.set_fonts(fonts.clone());
            h.state_mut().family = family;
            let (first, next) = one_pass(&mut h);
            let mut fresh = harness_sized(panes(false), 1200.0);
            fresh.ctx.set_fonts(fonts);
            fresh.run_steps(2);
            fresh.state_mut().family = family;
            fresh.run_steps(6);
            let settled = shown(&fresh);
            assert!(
                same(&first, &settled) && settled != shown(&harness_sized(panes(false), 1200.0)),
                "{family:?}: the first frame with the new fonts is where a fresh pane settles, \
                 and the fonts change it: {first:?}, settled {settled:?}"
            );
            assert!(
                same(&next, &first),
                "{family:?}: and the next moves nothing: {next:?}"
            );
        }

        // Review of PR 452, 5: a footer that no frame draws has ended, and its plan with its
        // lines goes; the context keeps the plans of the footers in view, whatever was shown.
        assert_eq!(kept_footer_plans(&h.ctx), 2, "two panes, two plans");
        h.state_mut().two_panes = false;
        h.run_steps(4);
        assert_eq!(
            kept_footer_plans(&h.ctx),
            1,
            "one pane closed: its plan went"
        );
        for owner in 1..=5 {
            h.state_mut().owner = owner;
            h.run_steps(3);
        }
        assert_eq!(
            kept_footer_plans(&h.ctx),
            1,
            "five owners in turn: each one's plan went when it changed"
        );
        // Re-check P3: after many footers end at once, the sweep's scan shrinks with the store
        // (`HashMap::retain` visits its spare room too).
        h.ctx.data_mut(|data| {
            let plans = data.get_temp_mut_or_default::<FooterPlans>(plans_id());
            let kept = plans.plans.values().next().expect("one plan").clone();
            for n in 0..200 {
                let ended = Retained {
                    drawn: 0,
                    ..kept.clone()
                };
                plans
                    .plans
                    .insert(egui::Id::new(("ended footer", n)), ended);
            }
        });
        h.run_steps(3);
        let room = h.ctx.data_mut(|data| {
            data.get_temp_mut_or_default::<FooterPlans>(plans_id())
                .plans
                .capacity()
        });
        assert_eq!(kept_footer_plans(&h.ctx), 1, "the ended footers went");
        assert!(room <= 32, "and the store's spare room with them: {room}");

        // Two panes side by side: a short one whose footer has no room for its reasons'
        // reservations, and a tall one that has. Each decides for itself (no shared state): the
        // tall one keeps the room under `Folder saved.` for its open lock's reason, as two tall
        // panes do; the short one gives it up.
        let gaps = |h: &egui_kittest::Harness<'_, App>| -> Vec<f32> {
            let tops = |words: &str| {
                let mut tops: Vec<(f32, f32)> = h
                    .query_all_by_role_and_label(egui::accesskit::Role::Label, words)
                    .map(|node| (node.rect().left(), node.rect().top()))
                    .collect();
                tops.sort_by(|a, b| a.0.total_cmp(&b.0));
                tops
            };
            tops("Folder saved.")
                .iter()
                .zip(tops("Campaign link saved."))
                .map(|(newer, older)| older.1 - newer.1)
                .collect()
        };
        let panes = |heights: [f32; 2]| {
            let mut app = two();
            app.heights = Some(heights);
            let mut h = harness_sized(app, 960.0);
            h.set_size(egui::vec2(960.0, 800.0));
            h.run_steps(8);
            gaps(&h)
        };
        let tall = panes([760.0, 760.0]);
        let mixed = panes([230.0, 760.0]);
        assert_eq!(
            (tall.len(), mixed.len()),
            (2, 2),
            "two panes: {tall:?} {mixed:?}"
        );
        assert!(
            (mixed[1] - tall[1]).abs() < 0.5,
            "the tall pane keeps its own, as beside another tall pane: {mixed:?} {tall:?}"
        );
        assert!(
            mixed[0] < mixed[1] - 1.0,
            "the short pane gives its reservation up: {mixed:?}"
        );
        // Review of PR 452 (re-check, P2): the short pane gives its reservation up, a result
        // changes while it stays short (its new plan is made with the room given up), then only
        // the pane grows. The plan holds (the same lines, width, scale and style); its quiet
        // frames take each reason's measured room and lay out none, as a pane that was tall
        // throughout does.
        let mut app = two();
        app.heights = Some([230.0, 760.0]);
        let mut h = harness_sized(app, 960.0);
        h.set_size(egui::vec2(960.0, 800.0));
        h.run_steps(8);
        h.state_mut().in_body = Some(InBody::Add("third"));
        h.run_steps(8);
        assert!(
            gaps(&h)[0] < gaps(&h)[1] - 1.0,
            "still short: the reservation is given up"
        );
        h.state_mut().heights = Some([760.0, 760.0]);
        h.run_steps(8);
        let settled = footer_work();
        h.run_steps(6);
        let still = footer_work();
        assert_eq!(
            (still.plans - settled.plans, still.laid - settled.laid),
            (0, 0),
            "grown, the pane's plan holds and its quiet frames lay out no reservation"
        );
        let grown = gaps(&h);
        assert!(
            (grown[0] - grown[1]).abs() < 0.5,
            "grown, it keeps the room as the tall pane does: {grown:?}"
        );

        // Ruling L9 (attack finding 2): a press is acted on once, by its ticket, also when its
        // frame is drawn again (a second pass) and when that rerun is declined (one pass at
        // most). The press is a choice whose line stays (`Register as a new campaign`), so a
        // press kept and given again would act twice.
        for passes in [2_usize, 1] {
            let mut app = two();
            app.two_panes = true;
            let mut h = harness_sized(app, 1200.0);
            h.ctx.options_mut(|options| {
                options.max_passes = std::num::NonZeroUsize::new(passes).expect("one at least");
            });
            let older = order(h.state(), "older");
            // The press's frame asks egui to draw it again.
            h.state_mut().discard = true;
            h.get_all_by_label("Register as a new campaign")
                .nth(1)
                .unwrap()
                .click_accesskit();
            let before = h.state().passes;
            h.step();
            assert_eq!(
                (h.state().passes - before, h.state().discard),
                (passes, false),
                "the press's frame is drawn {passes} times"
            );
            h.run_steps(3);
            assert_eq!(
                (h.state().presses.as_slice(), h.state().registered),
                ([(1, Act::Register, older)].as_slice(), 1),
                "{passes} passes: one press, by its ticket"
            );
        }
    }

    /// At 960 in a narrow pane the lock's reason wraps at whole words under the message words;
    /// a lock that comes and goes moves no line below it (its line is kept while it is open).
    #[test]
    fn a_lock_reason_wraps_under_the_words_and_moves_nothing_when_it_comes() {
        use pilot::*;
        let mut app = two();
        app.narrow = Some(320.0);
        let mut h = harness_sized(app, 960.0);
        let tops = |h: &egui_kittest::Harness<'_, App>| {
            ["Folder saved.", "Campaign link saved."].map(|words| {
                h.get_by_role_and_label(egui::accesskit::Role::Label, words)
                    .rect()
                    .top()
            })
        };
        // Ruling R29: a lock moves nothing where the footer has room for its reservation. With
        // Q-c, this footer follows a short body, so at 960 × 500 it is taller than a third of the
        // pane (a pinned footer's cap) and keeps its reservations: a lock moves nothing there
        // either. The case where it has no room is below, in the pane that shrinks.
        let (room, footer) = crate::design::pane::pane_records(&h.ctx)
            .first()
            .map(|record| (record.room(), record.footer()))
            .expect("the pane's record");
        let open = tops(&h);
        h.state_mut().run_live = true;
        h.run_steps(4);
        assert!(
            footer.is_some_and(|footer| footer.height() > room.height() / 3.0 + 1.0)
                && tops(&h) == open,
            "at 960 x 500 the footer {footer:?} below the short body is taller than a third of \
             the pane {room:?}, and a lock that comes moves no line: {open:?} to {:?}",
            tops(&h)
        );
        h.state_mut().run_live = false;
        h.run_steps(4);
        h.set_size(egui::vec2(960.0, 800.0));
        h.run_steps(6);
        let open = tops(&h);
        h.state_mut().run_live = true;
        h.run_steps(4);
        assert_eq!(tops(&h), open, "a lock that comes moves no line");
        let words = h
            .get_by_role_and_label(egui::accesskit::Role::Label, "Campaign link saved.")
            .rect();
        let reasons: Vec<_> = h
            .query_all_by_role_and_label(egui::accesskit::Role::Label, LOCK.as_str())
            .map(|node| node.rect())
            .collect();
        let line = Type::Small.line();
        assert!(
            !reasons.is_empty()
                && reasons
                    .iter()
                    .all(|reason| (reason.left() - words.left()).abs() <= 1.0
                        && reason.height() >= 2.0 * line - 1.0
                        && reason.right() <= 321.0),
            "the reason wraps under the words: {reasons:?}, words {words:?}"
        );
        h.state_mut().run_live = false;
        h.run_steps(4);
        assert_eq!(tops(&h), open, "a lock that goes moves no line");

        // Ruling R29: the open lock's room is a reservation, never a reason to scroll. Shrink
        // the pane until the footer has less room than it wants (below the short body, the room
        // down to the pane's end: ruling Q-c), so its reservations give way. At each size a wheel
        // over the footer moves its words exactly when a line the DM can see is cut: the
        // invisible room alone never makes it scroll, and a cut line can always be reached.
        let view = |h: &egui_kittest::Harness<'_, App>| {
            crate::design::pane::pane_records(&h.ctx)
                .first()
                .and_then(|record| record.footer())
                .expect("the footer's view")
                .shrink(crate::design::components::PART_INSET)
        };
        let shown = [
            "Folder saved.",
            "Campaign link saved.",
            "Undo change older",
            "Undo change newer",
            "Register as a new campaign",
        ];
        let top = |h: &egui_kittest::Harness<'_, App>, words: &str| {
            h.get_by_role_and_label(egui::accesskit::Role::Label, words)
                .rect()
                .top()
        };
        let wheel = |h: &mut egui_kittest::Harness<'_, App>, at: egui::Pos2, by: f32| {
            h.event(egui::Event::PointerMoved(at));
            h.event(egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                delta: egui::vec2(0.0, by),
                modifiers: egui::Modifiers::NONE,
            });
            h.run_steps(6);
            // No hover tip of a locked control stays over the footer.
            h.event(egui::Event::PointerGone);
            h.run_steps(2);
        };
        let room = view(&h).height();
        let mut capped_whole = None;
        for height in (200..=800).rev().step_by(6) {
            h.set_size(egui::vec2(960.0, height as f32));
            h.run_steps(6);
            let part = view(&h);
            let cut = shown.iter().any(|name| {
                h.query_by_label(name)
                    .is_some_and(|node| node.rect().bottom() > part.bottom() + 0.5)
            });
            h.run_steps(6);
            let before = top(&h, "Folder saved.");
            wheel(&mut h, part.center(), -400.0);
            let after = top(&h, "Folder saved.");
            wheel(&mut h, part.center(), 400.0);
            // The trailing item spacing can pass the edge by a point or two; the reservation
            // is a whole reason line (76 px here).
            let moved = (after - before).abs() > XS;
            assert_eq!(
                moved, cut,
                "at 960 x {height} the footer {part:?} scrolls ({before} to {after}) exactly when \
                 a line the DM can see is cut"
            );
            if part.height() < room - 0.5 && !cut && capped_whole.is_none() {
                capped_whole = Some(height);
            }
        }
        let height = capped_whole
            .expect("a size where the footer has less room than it wants, with its lines whole");
        h.set_size(egui::vec2(960.0, height as f32));
        h.run_steps(6);

        // Locked there, each reason is content. The first message's words and actions stay
        // where they were (its reason comes under them); every reason is there, and a wheel
        // brings each one whole into the footer's view, under its own message's actions.
        let open = tops(&h);
        let actions = ["Undo change newer", "Undo change older"];
        let placed = |h: &egui_kittest::Harness<'_, App>| {
            actions.map(|name| h.get_by_label(name).rect().left_top())
        };
        let first = placed(&h)[0];
        h.state_mut().run_live = true;
        h.run_steps(6);
        assert_eq!(
            tops(&h)[0],
            open[0],
            "with less room than it wants, a lock moves nothing above its reason"
        );
        assert_eq!(placed(&h)[0], first, "the first message's action stays");
        let line = Type::Small.line();
        for (at, action) in actions.iter().enumerate() {
            let reason = |h: &egui_kittest::Harness<'_, App>| {
                let mut reasons: Vec<_> = h
                    .query_all_by_role_and_label(egui::accesskit::Role::Label, LOCK.as_str())
                    .map(|node| node.rect())
                    .collect();
                assert_eq!(reasons.len(), 2, "one reason line per message: {reasons:?}");
                reasons.sort_by(|a, b| a.top().total_cmp(&b.top()));
                reasons[at]
            };
            for _ in 0..40 {
                if reason(&h).bottom() <= view(&h).bottom() + 0.5 {
                    break;
                }
                let centre = view(&h).center();
                wheel(&mut h, centre, -24.0);
            }
            let (shown, part) = (reason(&h), view(&h));
            let control = h.get_by_label(action).rect();
            assert!(
                shown.top() >= part.top() - 0.5
                    && shown.bottom() <= part.bottom() + 0.5
                    && shown.height() >= 2.0 * line - 1.0
                    && control.bottom() <= shown.top(),
                "reason {at} {shown:?} is not whole in the footer's view {part:?} under its \
                 action {control:?}"
            );
        }
        let centre = view(&h).center();
        wheel(&mut h, centre, 4000.0);
        h.state_mut().run_live = false;
        h.run_steps(6);

        // The reservation's room is measured for the messages and the width of this pass: a
        // change of either leaves the footer as a fresh footer in that state draws it, open and
        // locked.
        let sized = |app: App, height: f32| {
            let mut fresh = harness_sized(app, 960.0);
            fresh.set_size(egui::vec2(960.0, height));
            fresh.run_steps(8);
            fresh
        };
        // The reason lines, top to bottom.
        let reasons_of = |h: &egui_kittest::Harness<'_, App>| {
            let mut reasons: Vec<_> = h
                .query_all_by_role_and_label(egui::accesskit::Role::Label, LOCK.as_str())
                .map(|node| node.rect())
                .collect();
            reasons.sort_by(|a, b| a.top().total_cmp(&b.top()));
            reasons
        };
        let same = |h: &mut egui_kittest::Harness<'static, App>,
                    other: &mut egui_kittest::Harness<'static, App>,
                    what: &str,
                    count: usize| {
            let words: Vec<_> = ["Folder saved.", "Campaign link saved."]
                .into_iter()
                .filter(|words| h.query_all_by_label(words).next().is_some())
                .collect();
            let state = |h: &egui_kittest::Harness<'_, App>| {
                (
                    view(h),
                    words.iter().map(|words| top(h, words)).collect::<Vec<_>>(),
                )
            };
            assert_eq!(state(h), state(other), "{what}: open, as a fresh footer");
            for one in [&mut *h, &mut *other] {
                one.state_mut().run_live = true;
                one.run_steps(6);
            }
            assert_eq!(state(h), state(other), "{what}: locked, as a fresh footer");
            let (mine, theirs) = (reasons_of(h), reasons_of(other));
            assert!(
                mine.len() == count && mine == theirs,
                "{what}: locked, {count} reasons as a fresh footer: {mine:?} and {theirs:?}"
            );
            for one in [&mut *h, &mut *other] {
                one.state_mut().run_live = false;
                one.run_steps(6);
            }
        };
        let with = |width: f32, live: bool| {
            let mut app = two();
            app.narrow = Some(width);
            app.run_live = live;
            app
        };
        // Wider, just enough that each reason takes one line instead of two: a measured change
        // of wrapping, on fresh footers, locked, with room, so this footer keeps the state it
        // gave its room up in.
        let narrow = reasons_of(&sized(with(320.0, true), 800.0));
        assert!(
            narrow.len() == 2
                && narrow
                    .iter()
                    .all(|reason| reason.height() >= 2.0 * line - 1.0),
            "at 320 px each of the two reasons takes two lines: {narrow:?}"
        );
        let width = (330..=900)
            .step_by(10)
            .map(|width| width as f32)
            .find(|width| {
                let wide = reasons_of(&sized(with(*width, true), 800.0));
                assert_eq!(wide.len(), 2, "two reasons at {width}: {wide:?}");
                wide.iter().all(|reason| reason.height() < 1.5 * line)
            })
            .expect("a width where each reason takes one line");
        // A cap that holds the wide footer with its reservations, and not the narrow one: a
        // little taller than the wide footer needs, found by shrinking the pane at 320 px.
        let needs = view(&sized(with(width, false), 800.0)).height();
        let narrow_needs = view(&sized(with(320.0, false), 800.0)).height();
        let mut wide_cap = height;
        while wide_cap > 200 {
            h.set_size(egui::vec2(960.0, wide_cap as f32));
            h.run_steps(8);
            if view(&h).height() <= needs + 4.0 {
                break;
            }
            wide_cap -= 2;
        }
        let cap = view(&h).height();
        assert!(
            cap >= needs && cap < narrow_needs,
            "a cap {cap} that holds the wide footer ({needs}) and not the narrow one ({narrow_needs})"
        );
        h.state_mut().narrow = Some(width);
        h.run_steps(8);
        same(
            &mut h,
            &mut sized(with(width, false), wide_cap as f32),
            "wider",
            2,
        );
        h.state_mut().narrow = Some(320.0);
        h.set_size(egui::vec2(960.0, height as f32));
        h.run_steps(8);
        // Back at 320 px, in a pane whose cap is a little taller than one message with its
        // reservation (a fresh footer of that one message, with room, tells how tall): the
        // footer holding both has given its reservations' room up. Then one message goes: the
        // other's reservation has room again, as in a fresh footer of it.
        let alone = || {
            let mut one = App::default();
            one.dndbeyond
                .results
                .record(Saved(Campaign("older"), "Campaign link saved"));
            one.narrow = Some(320.0);
            one
        };
        let needs = view(&sized(alone(), 800.0)).height();
        h.state_mut().narrow = Some(320.0);
        let mut low = height;
        while low > 200 {
            h.set_size(egui::vec2(960.0, low as f32));
            h.run_steps(8);
            if view(&h).height() <= needs + 12.0 {
                break;
            }
            low -= 2;
        }
        assert!(
            view(&h).height() >= needs,
            "a pane whose cap holds one message with its reservation: {} for {needs}",
            view(&h).height()
        );
        h.state_mut()
            .dndbeyond
            .results
            .owner_changed(&Campaign("newer"));
        h.run_steps(8);
        same(
            &mut h,
            &mut sized(alone(), low as f32),
            "one message removed",
            1,
        );
    }
}
