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
use super::components::FooterRank;
pub use super::components::{ResultOrder, Tone};
pub use super::results::{ByTicket, Gone, Ticket};
use egui::{self, Ui};
use std::borrow::Cow;

/// The footer's messages as plain data, for the GPUI port (`docs/gpui-port.md`).
pub mod plain;

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

    /// The past form inside a sentence, after its object: `ticked` in `“…” ticked in Horn Rings.`
    pub fn past_after(self) -> String {
        self.past().to_lowercase()
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
}

/// The kind of a choice whose words hold data, as `MessageChoice` is for fixed words: a safe
/// choice is bordered; a `Lose` is red with exactly `L` before it, after the safe ones. (A
/// dismissal has no words: it is the close icon, `MessageChoice::Dismiss`.)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChoiceKind {
    Safe,
    Lose,
}

impl ChoiceKind {}

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

    /// Its Undo is locked for as long as it is drawn, for `reason`.
    pub fn undo_blocked(mut self, reason: Sentence) -> Self {
        if let Some(undo) = &mut self.undo {
            undo.2 = Some((reason, true));
        }
        self
    }

    pub fn choice(mut self, choice: Choice<A>) -> Self {
        self.choices.push(choice);
        self
    }

    /// Its Undo is locked while `locked`, for `reason`; the reason's line is kept while open.
    pub fn undo_locked_when(mut self, locked: bool, reason: Sentence) -> Self {
        if let Some(undo) = &mut self.undo {
            undo.2 = Some((reason, locked));
        }
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

    /// Its ticket, for a test of `Results::present`.
    #[cfg(test)]
    pub(in crate::design) fn ticket(&self) -> &Ticket {
        &self.ticket
    }
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
    /// The footer's lines, typed: what each draws, its place in the order and a kept result's
    /// ticket. No line is a closure (layout PR 2b): the footer draws them from this data, in the
    /// order its retained plan holds.
    lines: Vec<FooterLine<A>>,
    frame: std::marker::PhantomData<&'f PaneFrame>,
    /// Whether this frame's footer has its one `answer` (ruling Q-x).
    answered: bool,
    /// The newest act order of this footer's kept lines ([`refusal_rank`]).
    newest: Option<ResultOrder>,
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
}

impl<'f, A: 'static> Messages<'f, A> {
    /// No messages yet, for this frame's pane. `id` and `width` were the egui footer's; nothing
    /// reads them now (TODO(3b): drop them with the toolkit-free context).
    pub fn new(frame: &'f PaneFrame, id: impl std::hash::Hash, width: f32) -> Self {
        let _ = (frame, width);
        drop(id);
        Self {
            lines: Vec::new(),
            frame: std::marker::PhantomData,
            answered: false,
            newest: None,
        }
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
        });
        self
    }

    /// The answer to the DM's last act when it failed (ruling Q-x): a fault that the caller
    /// holds as its newest answer and ends at the next act. It comes first in the footer, before
    /// every kept result, which are answers to older acts. One per footer (a second is refused in
    /// debug builds and tests); Clippy refuses it outside a counted use (`Ban::Answer`).
    ///
    /// `under`: the act was a press of a kept line of this footer (`Press::line`). The egui
    /// footer placed the answer under that line; the plain lines keep the one order, so nothing
    /// reads it now (TODO(3b): drop it).
    pub fn answer(&mut self, fault: FaultMessage<A>, under: Option<PressedLine>) -> &mut Self {
        debug_assert!(
            !self.answered,
            "a footer holds one answer to the DM's last act"
        );
        self.answered = true;
        let _ = under;
        let (_, typed) = Outcome::from(fault).parts();
        self.lines.push(FooterLine {
            rank: FooterRank::Answer,
            order: None,
            typed,
            ticket: None,
            refused: false,
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
}

impl Body {}

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
    /// Its tone (also for a UI that draws a fault itself, the GPUI port).
    pub fn tone(self) -> Tone {
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

/// A notice that came by itself, at the top of a pane's body, in the compact form. It holds no
/// Undo, no choice and no order, so it cannot carry a result's way back; its words can still say
/// anything. `banner`, `row_fault` and `PagePlaceholder` are the body places: they take any `Ui`,
/// so the source lint counts their uses per file (`body-message`), and a new use shows in review.
pub enum Banner<'a> {
    Fault(&'a dyn Fault),
    Notice(Sentence),
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
