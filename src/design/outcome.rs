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
    number_width, small_tone_message, tone_message, undo_message_line,
};
pub use super::components::{Footer, ResultOrder, Tone};
pub use super::results::{ByTicket, Gone, Ticket};
use super::{extras::space, icons, tokens::*};
use eframe::egui::{self, Rect, Response, Ui};
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
    Checked(())
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
/// const R: splat::outcome::Sentence = splat::sentence!("Add a creature first.");
/// ```
/// ```compile_fail
/// const R: splat::outcome::Sentence = splat::sentence!("Add a creature first");
/// ```
///
/// Nor one that starts with a small letter, has a straight apostrophe or a code word, or ends
/// with a value:
///
/// ```
/// let name = "Cinder Wolf";
/// let _ = splat::sentence!("Removed {}.", name);
/// let _ = splat::sentence!("The party’s seal is kept.");
/// let _ = splat::sentence!("The key is kept.");
/// ```
/// ```compile_fail
/// let _ = splat::sentence!("removed the wolf.");
/// ```
/// ```compile_fail
/// let _ = splat::sentence!("The party's seal is kept.");
/// ```
/// ```compile_fail
/// let _ = splat::sentence!("The keyring is kept.");
/// ```
/// ```compile_fail
/// let name = "Cinder Wolf";
/// let _ = splat::sentence!("Removed {}", name);
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
    /// unless their first word is data (`is_data`: a path or a file name, such as
    /// `wrong-package/ holds …` or `package.md changed on disk`), which keeps its letters as they
    /// are on disk. Blank words are no sentence: `None`, and the caller says what it knows in its
    /// own checked words. The source lint counts the uses of this escape per file.
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

/// Is this word data that keeps its letters (M4 gap 3)? A word with a path separator (`/`, `\`)
/// or a file extension: a `.` with a letter or digit on both sides (`package.md`, `v2.json`).
/// Not a word that only ends with a mark (`removed.`, `wolf:`).
fn is_data(word: &str) -> bool {
    let word = word.trim_end_matches([',', ':', ';', '.', '?', '!', '”', '’', ')']);
    let bytes = word.as_bytes();
    word.contains(['/', '\\'])
        || bytes
            .windows(3)
            .any(|w| w[0].is_ascii_alphanumeric() && w[1] == b'.' && w[2].is_ascii_alphanumeric())
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
/// (`Undo remove Ravenholm…`). The message part gives the action the whole form as its name
/// whenever the visible words differ.
///
/// ```
/// let _ = splat::outcome::UndoStep::act("Undo it");
/// ```
/// ```compile_fail
/// let _ = splat::outcome::UndoStep { label: "Undo it".to_owned() };
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
        if number_width(ui, &whole, &font) <= width {
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
/// use splat::outcome::{Choice, ChoiceWords};
/// let (a, b) = (String::from("Ghost of the Mill"), String::from("Ember Cultist"));
/// let words = splat::choice!("Move {} before {}", a, b);
/// assert_eq!(words.label(), "Move Ghost of the Mill before Ember Cultist");
/// let _ = Choice::named(words, 1);
/// let _: ChoiceWords = "Open in Library".into();
/// ```
/// ```compile_fail
/// use splat::outcome::Choice;
/// let (a, b) = (String::from("Ghost of the Mill"), String::from("Ember Cultist"));
/// let _ = Choice::named(format!("Move {a} before {b}"), 1);
/// ```
/// ```compile_fail
/// let a = String::from("Ghost of the Mill");
/// let _ = splat::choice!("Move {} before {}", a);
/// ```
///
/// A fixed word is a word: an object behind one letter is not choice words (twin: `Add {}`).
///
/// ```
/// let label = String::from("Repair Bridge Ambush");
/// let _ = splat::choice!("Add {}", label);
/// ```
/// ```compile_fail
/// let label = String::from("Repair Bridge Ambush");
/// let _ = splat::choice!("A{}", label);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChoiceWords {
    /// The fixed words: one more than the objects.
    fixed: Vec<&'static str>,
    objects: Vec<String>,
}

/// How many letters a cut object keeps at least.
const OBJECT_KEEPS: usize = 3;

impl From<&'static str> for ChoiceWords {
    fn from(fixed: &'static str) -> Self {
        Self {
            fixed: vec![fixed],
            objects: Vec::new(),
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
        }
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

    /// The visible words: the whole words when `fits` them, else the longest cut that does
    /// (every object cut to the same number of letters, at least `OBJECT_KEEPS`). When even the
    /// shortest cut does not fit, the whole words wrap: a fixed word is never cut. The search is
    /// a bisection over the letters kept: at most `log2` of the longest object's length
    /// measurements per draw.
    fn fit(&self, fits: impl Fn(&str) -> bool) -> Fitted {
        let whole = self.label();
        if fits(&whole) {
            return Fitted::Whole(whole);
        }
        let longest = self
            .objects
            .iter()
            .map(|object| object.chars().count())
            .max()
            .unwrap_or(0);
        if longest <= OBJECT_KEEPS || !fits(&self.shown(OBJECT_KEEPS)) {
            return Fitted::Wraps(whole);
        }
        // The widest cut that fits: the width grows with the letters kept.
        let (mut low, mut high) = (OBJECT_KEEPS, longest - 1);
        while low < high {
            let mid = (low + high).div_ceil(2);
            if fits(&self.shown(mid)) {
                low = mid;
            } else {
                high = mid - 1;
            }
        }
        Fitted::Cut(self.shown(low), whole)
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
/// a `Choice` is bordered, a `Lose` is red with exactly `L` before it, a `Link` is a link. An
/// Undo is not a choice: it is the message's `UndoStep`, always quiet with its icon, and first.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
///
/// `Dismiss` is the close icon. The part puts it at the right end of the message's first line,
/// centred on that line, whatever its place in the list (N6); a caller gives no icon, name or
/// place. It says what goes with the message ([`Lost`]), so the choice is made on purpose.
///
/// ```
/// use splat::outcome::{Choice, Lost, MessageChoice, Notice};
/// let _ = Notice::new(splat::sentence!("Sync is off."))
///     .choice(Choice::new(MessageChoice::Dismiss(Lost::Nothing), 1));
/// ```
///
/// What goes is said, never left out:
///
/// ```compile_fail,E0308
/// use splat::outcome::{Choice, MessageChoice, Notice};
/// let _ = Notice::new(splat::sentence!("Sync is off."))
///     .choice(Choice::new(MessageChoice::Dismiss, 1));
/// ```
pub enum MessageChoice<'a> {
    Choice(&'a str),
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
    #[cfg(test)]
    let clicked = clicked
        || ui.ctx().data(|data| {
            data.get_temp::<Vec<(String, Pressed)>>(egui::Id::new(TEST_ACTIVATIONS))
                .is_some_and(|list| list.iter().any(|(w, p)| w == words && *p == what))
        });
    #[cfg(not(test))]
    let _ = (ui, words);
    if clicked {
        first(slot, what);
    }
}

/// Test builds: the controls `activated` treats as pressed in this pass.
#[cfg(test)]
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
/// use splat::outcome::{Choice, MessageChoice};
/// let waiting = true;
/// let _ = Choice::new(MessageChoice::Choice("Replace"), 1)
///     .locked_when(waiting, splat::sentence!("Wait for the read."));
/// let _ = Choice::new(MessageChoice::Choice("Replace"), 1)
///     .blocked(splat::sentence!("Wait for the read."));
/// ```
/// ```compile_fail
/// use splat::outcome::{Choice, MessageChoice};
/// let waiting = true;
/// let _ = Choice::new(MessageChoice::Choice("Replace"), 1)
///     .blocked(waiting.then_some(splat::sentence!("Wait for the read.")));
/// ```
#[must_use = "give it to a message's `choice`"]
pub struct Choice<A> {
    what: MessageChoice<'static>,
    /// A choice whose words hold objects from data (`Choice::named`, `Choice::named_as`).
    named: Option<(ChoiceKind, ChoiceWords)>,
    id: A,
    lock: Lock,
    hover: Option<String>,
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
        }
    }

    /// Words on hover made at run time, owned (a link's path).
    pub fn hover(mut self, words: String) -> Self {
        self.hover = Some(words);
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
/// use splat::outcome::{UndoStep, Verb};
/// let live = true;
/// let _ = UndoStep::verb(Verb::Register).pressed_as(1).locked_when(live, splat::sentence!("End the run first."));
/// ```
/// ```compile_fail
/// use splat::outcome::{UndoStep, Verb};
/// let live = true;
/// let _ = UndoStep::verb(Verb::Register).pressed_as(1).blocked(live.then_some(splat::sentence!("End the run first.")));
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
/// use splat::outcome::{Done, Verb};
/// let _ = Done::new(Verb::Remove, "Take seal").undo(1);
/// ```
/// ```compile_fail
/// use splat::outcome::{Done, Verb};
/// let _ = Done::new(Verb::Remove, "Take seal").undo("Undo discard", 1);
/// ```
///
/// An order made by hand does not go in:
///
/// ```compile_fail
/// use splat::outcome::{Done, ResultOrder, Verb};
/// let _ = Done::<u8>::new(ResultOrder::default(), Verb::Remove, "Take seal");
/// ```
///
/// The object may come first, and the result may name where it was done; the Undo then tells
/// itself apart by the same place (Open questions' tick):
///
/// ```
/// use splat::outcome::{Done, Verb};
/// let done = Done::new(Verb::Tick, "“Who hired the bandits?”").object_first().within("Horn Rings").undo(1);
/// assert_eq!(done.words().as_str(), "“Who hired the bandits?” ticked in Horn Rings.");
/// ```
///
/// Its Undo's lock has the two forms of [`Choice`]'s: an optional reason does not fit
/// `undo_blocked`.
///
/// ```
/// use splat::outcome::{Done, Verb};
/// let live = true;
/// let _ = Done::new(Verb::Remove, "Bell").undo(1).undo_locked_when(live, splat::sentence!("End the run first."));
/// let _ = Done::new(Verb::Remove, "Bell").undo(1).undo_blocked(splat::sentence!("End the run first."));
/// ```
/// ```compile_fail
/// use splat::outcome::{Done, Verb};
/// let live = true;
/// let _ = Done::new(Verb::Remove, "Bell").undo(1).undo_blocked(live.then_some(splat::sentence!("End the run first.")));
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
/// use splat::outcome::{Said, UndoStep};
/// let _ = Said::new(splat::sentence!("Bell is done.")).undo(UndoStep::act("Done").pressed_as(1));
/// ```
/// ```compile_fail
/// use splat::outcome::{Said, UndoStep};
/// let _ = Said::<u8>::new(splat::sentence!("Bell is done.")).undo("Undo: Done");
/// ```
///
/// No message takes a tone:
///
/// ```compile_fail
/// use splat::outcome::{Said, Tone, UndoStep};
/// let _ = Said::<u8>::new(splat::sentence!("Bell is done.")).tone(Tone::Info);
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
/// use splat::outcome::{Notice, NoPress};
/// let _ = Notice::<NoPress>::new(splat::sentence!("Sync is off."));
/// let _ = Notice::<NoPress>::warning(splat::sentence!("Splat no longer shows these files."));
/// ```
/// ```compile_fail,E0599
/// use splat::outcome::{Notice, NoPress};
/// let _ = Notice::<NoPress>::error(splat::sentence!("Splat no longer shows these files."));
/// ```
/// ```compile_fail,E0599
/// use splat::outcome::{Notice, NoPress, Tone};
/// let _ = Notice::<NoPress>::new(splat::sentence!("Sync is off.")).tone(Tone::Error);
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
}

impl<A> FaultMessage<A> {
    pub fn new(fault: &dyn Fault) -> Self {
        Self {
            tone: fault.severity().tone(),
            words: fault_words(fault),
            choices: Vec::new(),
            file: None,
        }
    }

    pub fn choice(mut self, choice: Choice<A>) -> Self {
        self.choices.push(choice);
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
/// scrolls when its messages fill a third of the pane; it is never drawn in a pane's content.
///
/// The part draws it on a line under the message's words, from their first letter: the path,
/// then the links, on one line when they fit; else the path on its own line and the links on
/// the line under it, wrapped between whole links. The path is never changed: not by
/// `Sentence::outside`, never given a capital, never wrapped inside; a path wider than its line
/// is cut in the middle (`Campaign/Ses…/Timeline.md`), with the whole path in its hover and as
/// its accessible name. Each link is a target at least `XL` (24 px) square, and its accessible
/// name names the file (`Show file: Timeline.md`). A press opens the file: no id comes back,
/// and the pane's one press is not used.
///
/// The path is data: it is no sentence, so no sentence form reaches it.
///
/// ```
/// use splat::outcome::{FileLine, Notice, NoPress};
/// let _ = Notice::<NoPress>::new(splat::sentence!("Sync is off."))
///     .file(FileLine::new("/vault/Campaign", "Sessions/Timeline.md").edit().show());
/// ```
/// ```compile_fail,E0451
/// use splat::outcome::FileLine;
/// let _ = FileLine { root: "/vault".into(), relative: "Timeline.md".into(), edit: true, show: true };
/// ```
/// ```compile_fail,E0624
/// use splat::outcome::FileLine;
/// fn f(ui: &mut eframe::egui::Ui) {
///     FileLine::new("/vault/Campaign", "Timeline.md").show().draw(ui);
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
#[must_use = "give it to a footer message's `file`"]
pub struct FileLine {
    root: std::path::PathBuf,
    relative: String,
    edit: bool,
    show: bool,
}

impl FileLine {
    /// The file at `relative` (as the DM knows it, shown as it is) inside the folder `root`.
    pub fn new(root: impl Into<std::path::PathBuf>, relative: impl Into<String>) -> Self {
        Self {
            root: root.into(),
            relative: relative.into(),
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

    fn path(&self) -> std::path::PathBuf {
        self.root.join(&self.relative)
    }

    /// The file's own name, the last part of its path.
    fn name(&self) -> &str {
        self.relative
            .rsplit(['/', '\\'])
            .find(|part| !part.is_empty())
            .unwrap_or(&self.relative)
    }

    fn links(&self) -> impl Iterator<Item = &'static str> {
        [(self.edit, EDIT_IN_OBSIDIAN), (self.show, SHOW_FILE)]
            .into_iter()
            .filter_map(|(on, word)| on.then_some(word))
    }

    /// The accessible name of one of its links: the link's words, then the file.
    fn link_name(&self, word: &str) -> String {
        format!("{word}: {}", self.name())
    }

    /// The line, at the cursor, as wide as `ui` allows.
    fn draw(&self, ui: &mut Ui) {
        let path = self.path();
        let whole = path.display().to_string();
        let font = Type::Small.font(ui);
        let gap = ui.spacing().item_spacing.x;
        let links: f32 = self
            .links()
            .map(|word| number_width(ui, word, &Type::Body.font(ui)).max(XL) + gap)
            .sum();
        let room = ui.available_width();
        let beside = room - links - L;
        let one_line = links == 0.0 || number_width(ui, &self.relative, &font) <= beside;
        let shown = cut_middle(
            ui,
            &self.relative,
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
            for word in self.links() {
                let response = link(ui, word);
                super::components::named(ui, &response, &self.link_name(word));
                if response.clicked() {
                    let url = if word == EDIT_IN_OBSIDIAN {
                        obsidian_url(&path)
                    } else {
                        file_url(&path)
                    };
                    ui.ctx().open_url(egui::OpenUrl::new_tab(url));
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

/// The link that opens a file in Obsidian.
pub fn obsidian_url(path: &std::path::Path) -> String {
    format!("obsidian://open?path={}", encode_path(path))
}

/// The link that shows a file with the system's file handler.
pub fn file_url(path: &std::path::Path) -> String {
    format!("file://{}", encode_path(path))
}

fn encode_path(path: &std::path::Path) -> String {
    path.to_string_lossy()
        .bytes()
        .map(|byte| {
            if byte.is_ascii_alphanumeric() || b"-._~/".contains(&byte) {
                (byte as char).to_string()
            } else {
                format!("%{byte:02X}")
            }
        })
        .collect()
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
/// use splat::outcome::{Messages, NoPress, Outcome};
/// fn f(ui: &mut eframe::egui::Ui) {
///     let frame = splat::outcome::PaneFrame::new(ui);
///     let mut messages = Messages::<NoPress>::new(&frame, "f", 400.0);
///     messages.outcome(Outcome::Refused(splat::sentence!("No.")));
///     let _ = messages.pane(ui, splat::pane::Body::Content, |_| ());
/// }
/// ```
/// ```compile_fail
/// use splat::outcome::{Done, Messages, NoPress, Verb};
/// fn f(ui: &mut eframe::egui::Ui) {
///     let frame = splat::outcome::PaneFrame::new(ui);
///     let mut messages = Messages::<NoPress>::new(&frame, "f", 400.0);
///     messages.outcome(Done::new(Verb::Remove, "Bell"));
///     let _ = messages.pane(ui, splat::pane::Body::Content, |_| ());
/// }
/// ```
/// ```compile_fail
/// fn f(ui: &mut eframe::egui::Ui) {
///     splat::outcome::Outcome::<u8>::Refused(splat::sentence!("No.")).draw(ui);
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
/// | `Fault` (with its Undo) | from its `Severity` |
pub enum KeptOutcome<A> {
    Done(Done<A>),
    Said(Said<A>),
    Notice(Notice<A>, Undo<A>),
    Fault(FaultMessage<A>, Undo<A>),
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
/// borrow, so a footer of them can sit beside a body that changes the app.
struct Body {
    tone: Tone,
    words: Sentence,
    undo: Option<UndoStep>,
    undo_lock: Lock,
    choices: Vec<MessageChoice<'static>>,
    named: Vec<Option<(ChoiceKind, ChoiceWords)>>,
    locks: Vec<Lock>,
    hovers: Vec<Option<String>>,
    /// The file it is about, on a line under the words (`FileLine`).
    file: Option<FileLine>,
}

/// A body and the caller's ids of its Undo and of each choice, by index.
struct Typed<A> {
    body: Body,
    undo: Option<A>,
    choices: Vec<A>,
}

impl<A> Typed<A> {
    fn new(tone: Tone, words: Sentence, undo: Option<Undo<A>>, choices: Vec<Choice<A>>) -> Self {
        let (undo, undo_lock, undo_id) = match undo {
            Some(undo) => (Some(undo.step), undo.lock, Some(undo.id)),
            None => (None, None, None),
        };
        let mut ids = Vec::with_capacity(choices.len());
        let (mut whats, mut locks, mut hovers) = (Vec::new(), Vec::new(), Vec::new());
        let mut named = Vec::new();
        // Safe first: a choice that loses something follows every other (a stable order).
        let mut choices = choices;
        choices.sort_by_key(|choice| matches!(choice.what, MessageChoice::Lose(_)));
        for choice in choices {
            whats.push(choice.what);
            named.push(choice.named);
            locks.push(choice.lock);
            hovers.push(choice.hover);
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
                file: None,
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
                Typed::new(fault.tone, fault.words, None, fault.choices).with_file(fault.file),
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
            Self::Fault(fault, undo) => {
                Typed::new(fault.tone, fault.words, Some(undo), fault.choices).with_file(fault.file)
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
/// # use splat::ownership::{DiskChanged, Ends, Kept, Owner, OwnerChanged, Press, Results};
/// # #[derive(Clone, PartialEq, Eq)] struct Link;
/// # impl Owner for Link { type Scope = str; fn scope(&self) -> &str { "c" } }
/// # struct Saved;
/// # impl Kept for Saved { type Owner = Link; const ENDS: Ends = Ends { owner_changed: OwnerChanged::Ends, disk_changed: DiskChanged::Stays, press: Press::Nothing }; fn owner(&self) -> Link { Link } }
/// use splat::outcome::{KeptLine, NoPress, PaneFrame, Said};
/// fn frame(ui: &mut eframe::egui::Ui) {
///     let mut store = Results::new();
///     store.record(Saved);
///     let frame = PaneFrame::new(ui);
///     let line: KeptLine<NoPress> = store
///         .get(&Link)
///         .unwrap()
///         .outcome(&frame, |_| Said::new(splat::sentence!("Campaign link saved.")).into());
///     // The store is free again while the line lives.
///     store.record(Saved);
///     let _ = line;
/// }
/// ```
///
/// Not without a store, not cloned, its ticket not read, and not kept in app state:
///
/// ```compile_fail
/// use splat::outcome::{KeptLine, NoPress};
/// let _ = KeptLine::<NoPress> { ticket: todo!(), typed: todo!(), frame: todo!() };
/// ```
/// ```compile_fail
/// # use splat::ownership::{DiskChanged, Ends, Kept, Owner, OwnerChanged, Press, Results};
/// # #[derive(Clone, PartialEq, Eq)] struct Link;
/// # impl Owner for Link { type Scope = str; fn scope(&self) -> &str { "c" } }
/// # struct Saved;
/// # impl Kept for Saved { type Owner = Link; const ENDS: Ends = Ends { owner_changed: OwnerChanged::Ends, disk_changed: DiskChanged::Stays, press: Press::Nothing }; fn owner(&self) -> Link { Link } }
/// use splat::outcome::{KeptLine, NoPress, PaneFrame, Said};
/// fn frame(ui: &mut eframe::egui::Ui) {
///     let mut store = Results::new();
///     store.record(Saved);
///     let frame = PaneFrame::new(ui);
///     let line: KeptLine<NoPress> = store
///         .get(&Link)
///         .unwrap()
///         .outcome(&frame, |_| Said::new(splat::sentence!("Campaign link saved.")).into());
///     let _twice = line.clone();
/// }
/// ```
/// ```compile_fail
/// # use splat::ownership::{DiskChanged, Ends, Kept, Owner, OwnerChanged, Press, Results};
/// # #[derive(Clone, PartialEq, Eq)] struct Link;
/// # impl Owner for Link { type Scope = str; fn scope(&self) -> &str { "c" } }
/// # struct Saved;
/// # impl Kept for Saved { type Owner = Link; const ENDS: Ends = Ends { owner_changed: OwnerChanged::Ends, disk_changed: DiskChanged::Stays, press: Press::Nothing }; fn owner(&self) -> Link { Link } }
/// use splat::outcome::{KeptLine, NoPress, PaneFrame, Said};
/// fn frame(ui: &mut eframe::egui::Ui) {
///     let mut store = Results::new();
///     store.record(Saved);
///     let frame = PaneFrame::new(ui);
///     let line: KeptLine<NoPress> = store
///         .get(&Link)
///         .unwrap()
///         .outcome(&frame, |_| Said::new(splat::sentence!("Campaign link saved.")).into());
///     let _ticket = line.ticket;
/// }
/// ```
///
/// A line kept in app state across frames (the review's counterexample) does not compile; the
/// same line used in its frame does (the first example):
///
/// ```compile_fail
/// # use splat::ownership::{DiskChanged, Ends, Kept, Owner, OwnerChanged, Press, Results};
/// # #[derive(Clone, PartialEq, Eq)] struct Link;
/// # impl Owner for Link { type Scope = str; fn scope(&self) -> &str { "c" } }
/// # struct Saved;
/// # impl Kept for Saved { type Owner = Link; const ENDS: Ends = Ends { owner_changed: OwnerChanged::Ends, disk_changed: DiskChanged::Stays, press: Press::Nothing }; fn owner(&self) -> Link { Link } }
/// use splat::outcome::{KeptLine, NoPress, PaneFrame, Said};
/// struct App { store: Results<Link, Saved>, kept: Option<KeptLine<'static, NoPress>> }
/// fn frame(ui: &mut eframe::egui::Ui, app: &mut App) {
///     let frame = PaneFrame::new(ui);
///     app.kept = app
///         .store
///         .get(&Link)
///         .map(|shown| shown.outcome(&frame, |_| Said::new(splat::sentence!("Saved.")).into()));
/// }
/// ```
#[must_use = "give it to `Messages::kept`"]
pub struct KeptLine<'f, A> {
    ticket: Ticket,
    typed: Typed<A>,
    frame: std::marker::PhantomData<&'f PaneFrame>,
}

impl<'f, A> KeptLine<'f, A> {
    /// For `Shown::outcome` (in `results`), which holds the entry's ticket.
    pub(in crate::design) fn new(
        _frame: &'f PaneFrame,
        ticket: Ticket,
        outcome: KeptOutcome<A>,
    ) -> Self {
        Self {
            ticket,
            typed: outcome.typed(),
            frame: std::marker::PhantomData,
        }
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
/// use splat::outcome::{Choice, MessageChoice, Messages, Notice, PaneFrame};
/// #[derive(Clone, Copy)] enum Act { Again }
/// fn f(ui: &mut eframe::egui::Ui) -> Option<Act> {
///     let frame = PaneFrame::new(ui);
///     let mut messages = Messages::new(&frame, "f", 400.0);
///     messages.outcome(
///         Notice::new(splat::sentence!("Sync is off."))
///             .choice(Choice::new(MessageChoice::Choice("Try again"), Act::Again)),
///     );
///     messages.pane(ui, splat::pane::Body::Content, |_| ()).pressed.map(|press| press.id)
/// }
/// ```
/// ```compile_fail
/// use splat::outcome::{Choice, MessageChoice, Messages, Notice, PaneFrame};
/// #[derive(Clone, Copy)] enum Act { Again }
/// fn f(ui: &mut eframe::egui::Ui, name: String) -> Option<Act> {
///     let frame = PaneFrame::new(ui);
///     let mut messages = Messages::new(&frame, "f", 400.0);
///     messages.outcome(
///         Notice::new(splat::sentence!("Sync is off."))
///             .choice(Choice::new(MessageChoice::Choice(&name), Act::Again)),
///     );
///     messages.pane(ui, splat::pane::Body::Content, |_| ()).pressed.map(|press| press.id)
/// }
/// ```
///
/// Nor an id that borrows:
///
/// ```compile_fail
/// use splat::outcome::{Choice, MessageChoice, Messages, Notice, PaneFrame};
/// fn f<'a>(ui: &mut eframe::egui::Ui, app: &'a mut u8) {
///     let frame = PaneFrame::new(ui);
///     let mut messages = Messages::new(&frame, "f", 400.0);
///     messages.outcome(
///         Notice::new(splat::sentence!("Sync is off."))
///             .choice(Choice::new(MessageChoice::Choice("Try again"), &*app)),
///     );
///     let _ = messages.pane(ui, splat::pane::Body::Content, |_| ());
/// }
/// ```
pub struct Messages<'f, A: 'static> {
    footer: Footer<'static>,
    sink: PressSink<A>,
    frame: std::marker::PhantomData<&'f PaneFrame>,
}

impl<'f, A: 'static> Messages<'f, A> {
    /// No messages yet, for this frame's pane, with a footer `width` wide. `id` keeps its
    /// height, scroll and focus.
    pub fn new(frame: &'f PaneFrame, id: impl std::hash::Hash, width: f32) -> Self {
        let _ = frame;
        Self {
            footer: Footer::new(id, width),
            sink: PressSink(std::rc::Rc::default()),
            frame: std::marker::PhantomData,
        }
    }

    /// The draw of one typed message: its press, if any, goes to the pane's one return, with
    /// the kept entry's ticket. A kept line's controls are scoped by its ticket.
    fn draw(&self, typed: Typed<A>, ticket: Option<Ticket>) -> impl FnOnce(&mut Ui) + 'static {
        let sink = PressSink(self.sink.0.clone());
        move |ui| {
            let pressed = match &ticket {
                // A global id from the ticket: egui's child ids count the children before them,
                // so a line added above would hand this line's focus and press to another.
                Some(ticket) => {
                    // The pane's footer (this `Ui`) and the ticket: one stored entry shown in two
                    // panes has two lines with their own ids.
                    let pane = ui.ctx().data(|data| {
                        data.get_temp::<egui::Id>(egui::Id::new("design_footer_drawing"))
                    });
                    let id = egui::Id::new(("design_kept_line", pane, ticket.order()));
                    ui.scope_builder(egui::UiBuilder::new().id(id), |ui| {
                        typed.body.draw(ui, MessagePlace::Footer)
                    })
                    .inner
                }
                None => typed.body.draw(ui, MessagePlace::Footer),
            };
            if let Some(pressed) = pressed
                && let Some(id) = typed.id(pressed)
            {
                sink.press(Press { id, ticket });
            }
        }
    }

    /// A reason or a notice, at the place its variant gives it in the footer's order.
    pub fn outcome(&mut self, outcome: impl Into<Outcome<A>>) -> &mut Self {
        let (rank, typed) = outcome.into().parts();
        let words = Cow::Owned(typed.body.words.as_str().to_owned());
        let draw = self.draw(typed, None);
        self.footer.push(rank, None, words, draw);
        self
    }

    /// A kept result of this frame, newest first by the order its store issued.
    pub fn kept(&mut self, line: KeptLine<'f, A>) -> &mut Self {
        self.kept_at(line.ticket, line.typed)
    }

    fn kept_at(&mut self, ticket: Ticket, typed: Typed<A>) -> &mut Self {
        let has_undo = typed.body.undo.is_some();
        let words = Cow::Owned(typed.body.words.as_str().to_owned());
        let order = ticket.order();
        let draw = self.draw(typed, Some(ticket));
        self.footer.add_kept(order, words, has_undo, draw);
        self
    }

    /// The staged path for a result whose caller still issues its own `ResultOrder` (Wave 2
    /// moves it into a `Results` store). Clippy refuses it (`Ban::Order`).
    pub fn outcome_at(
        &mut self,
        order: ResultOrder,
        outcome: impl Into<KeptOutcome<A>>,
    ) -> &mut Self {
        self.kept_at(Ticket::of(order), outcome.into().typed())
    }

    /// The pane: the body (`add`, which may change any of the app), then the footer, placed by
    /// `Footer::pane`. The body's value, and the press.
    pub fn pane<R>(
        self,
        ui: &mut Ui,
        body: super::pane::Body,
        add: impl FnOnce(&mut Ui) -> R,
    ) -> Paned<R, A> {
        let Self { footer, sink, .. } = self;
        let body = footer.pane(ui, body, add);
        Paned {
            body,
            pressed: sink.0.borrow_mut().take(),
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
        let Self { footer, sink, .. } = self;
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
        let Self { footer, sink, .. } = self;
        let body = footer.pane_editor(ui, add);
        Paned {
            body,
            pressed: sink.0.borrow_mut().take(),
        }
    }
}

impl Body {
    /// With a `Dismiss`, the close icon at the right end of the first line, centred on that
    /// line; the words and the other choices keep the room to its left. A locked `Dismiss` is
    /// disabled, with its reason, as every blocked action is.
    fn draw(&self, ui: &mut Ui, place: MessagePlace) -> Option<Pressed> {
        let Some(at) = self
            .choices
            .iter()
            .position(|choice| matches!(choice, MessageChoice::Dismiss(_)))
        else {
            let mut pressed = None;
            self.draw_words(ui, place, &mut pressed);
            return pressed;
        };
        let size = Density::Read.control();
        let lock = self.locks.get(at).and_then(holding);
        let mut pressed = None;
        ui.scope(|ui| {
            ui.set_max_width(ui.available_width().min(LIST_MEASURE));
            let right = ui.max_rect().right();
            let line = ui
                .scope(|ui| {
                    ui.set_max_width(ui.available_width() - size - S);
                    self.draw_words(ui, place, &mut pressed)
                })
                .inner;
            let centre = egui::pos2(right - size / 2.0, line);
            let icon = ui.put(
                Rect::from_center_size(centre, egui::Vec2::splat(size)),
                |ui: &mut Ui| match lock {
                    Some(reason) => ui
                        .add_enabled_ui(false, |ui| icon_button(ui, icons::CLOSE, DISMISS_NAME))
                        .inner
                        .on_disabled_hover_text(reason),
                    None => icon_button(ui, icons::CLOSE, DISMISS_NAME),
                },
            );
            // The words and their controls are drawn before the close icon: they come first.
            let words = self.words.as_str();
            activated(ui, &mut pressed, icon.clicked(), words, Pressed::Choice(at));
        });
        pressed
    }

    /// The choices that sit on rows under the words: all but `Dismiss`.
    fn row_choices(&self) -> impl Iterator<Item = (usize, MessageChoice<'_>)> {
        self.choices
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, choice)| !matches!(choice, MessageChoice::Dismiss(_)))
    }

    /// The words, capped at `LIST_MEASURE`; an Undo alone beside them when it fits, else under
    /// them at their first letter; with choices, the Undo and then the choices on rows under the
    /// words, from their first letter. A locked Undo or choice is disabled, with its reason as
    /// its accessible description and painted under the message. Also the y of the centre of
    /// the words' first line. Its presses go to `pressed`.
    fn draw_words(&self, ui: &mut Ui, place: MessagePlace, pressed: &mut Option<Pressed>) -> f32 {
        let mut line = ui.cursor().top() + Type::Body.line() / 2.0;
        let response = ui
            .scope(|ui| {
                ui.set_max_width(ui.available_width().min(LIST_MEASURE));
                let words = self.words.as_str();
                if self.row_choices().next().is_none() {
                    match &self.undo {
                        None => {
                            line = first_line_centre(&tone_message(ui, self.tone, words));
                        }
                        Some(step) => {
                            let label = step.label();
                            let shown = self.undo_words(ui, step);
                            let name = (shown != label).then_some(label.as_str());
                            let lock = holding(&self.undo_lock);
                            let (undo, centre) =
                                undo_message_line(ui, self.tone, words, &shown, name, lock, true);
                            activated(ui, pressed, undo, words, Pressed::Undo);
                            line = centre;
                        }
                    }
                    self.file_line(ui);
                    self.reasons(ui);
                    return;
                }
                let message = tone_message(ui, self.tone, words);
                line = first_line_centre(&message);
                self.file_line(ui);
                let inset = message.rect.left() - ui.cursor().left();
                let gap = ui.spacing().item_spacing.x;
                let undo = self.undo.as_ref().map(|step| {
                    let (label, shown) = (step.label(), self.undo_words(ui, step));
                    let width = Action::quiet(&shown)
                        .icon(icons::UNDO)
                        .density(Density::Read)
                        .natural_width(ui);
                    (label, shown, width)
                });
                // Undo, then the choices `L` after it, on one line from the words when they fit;
                // else Undo on its line and the choices on the lines under it, from the words.
                let choices: f32 = self
                    .row_choices()
                    .map(|(at, choice)| self.choice_width(ui, at, choice) + gap)
                    .sum();
                let one_line =
                    inset + undo.as_ref().map_or(0.0, |(_, _, width)| width + L) + choices
                        <= ui.available_width();
                if undo.is_some() || one_line {
                    ui.horizontal(|ui| {
                        space(ui, inset);
                        if let Some((label, shown, _)) = &undo {
                            // Its words start at the message's words, its icon in the message's
                            // icon column.
                            let mut action = Action::quiet(shown)
                                .icon(icons::UNDO)
                                .density(Density::Read)
                                .line_start_in_column()
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
                            self.draw_choices(ui, pressed);
                        }
                    });
                }
                if !one_line {
                    ui.horizontal_top(|ui| {
                        space(ui, inset);
                        ui.horizontal_wrapped(|ui| {
                            self.draw_choices(ui, pressed);
                        });
                    });
                }
                self.reasons(ui);
            })
            .response;
        register(ui, response.rect, place, self.tone);
        line
    }

    /// The file line under the words, from their first letter (`FileLine::draw`).
    fn file_line(&self, ui: &mut Ui) {
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
    fn reasons(&self, ui: &mut Ui) {
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
        for (reason, locked) in said {
            message_choice_row(ui, self.tone, |ui| {
                if locked {
                    blocked_reason(ui, Some(reason));
                } else {
                    ui.scope_builder(egui::UiBuilder::new().invisible(), |ui| {
                        blocked_reason(ui, Some(reason));
                    });
                }
            });
        }
    }

    /// A choice's whole width, with the `L` before a red one.
    fn choice_width(&self, ui: &Ui, at: usize, choice: MessageChoice<'_>) -> f32 {
        if let Some(Some((kind, words))) = self.named.get(at) {
            let lose = if *kind == ChoiceKind::Lose { L } else { 0.0 };
            return kind.action(&words.label()).natural_width(ui) + lose;
        }
        match choice {
            MessageChoice::Choice(word) => Action::secondary(word)
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

    /// The choices, in order: a red one keeps exactly `L` from the one before it. The pressed
    /// choice's index.
    fn draw_choices(&self, ui: &mut Ui, pressed: &mut Option<Pressed>) {
        let gap = ui.spacing().item_spacing.x;
        for (at, choice) in self.row_choices() {
            let lock = self.locks.get(at).and_then(holding);
            let clicked = match (choice, self.named.get(at).and_then(Option::as_ref)) {
                // Words with objects: whole, cut (named and tipped with the whole words) or
                // wrapped to the row's width.
                // Words with objects: cut to the row's width, named with the whole words.
                (_, Some((kind, words))) => {
                    if *kind == ChoiceKind::Lose && at > 0 {
                        space(ui, (L - gap).max(0.0));
                    }
                    data_choice(ui, words, *kind, ui.max_rect().width(), lock).clicked()
                }
                (MessageChoice::Choice(word), None) => Action::secondary(word)
                    .density(Density::Read)
                    .blocked(lock)
                    .show(ui)
                    .clicked(),
                (MessageChoice::Lose(word), None) => {
                    if at > 0 {
                        space(ui, (L - gap).max(0.0));
                    }
                    Action::destructive(word)
                        .density(Density::Read)
                        .blocked(lock)
                        .show(ui)
                        .clicked()
                }
                (MessageChoice::Link { word, hover }, None) => match lock {
                    // A locked link is drawn disabled, with its reason, as a blocked action is.
                    Some(reason) => {
                        ui.add_enabled_ui(false, |ui| link(ui, word))
                            .inner
                            .on_disabled_hover_text(reason);
                        false
                    }
                    None => {
                        let response = link(ui, word);
                        let owned = self.hovers.get(at).and_then(Option::as_deref);
                        match owned.or(hover) {
                            Some(hover) => response.on_hover_text(hover),
                            None => response,
                        }
                        .clicked()
                    }
                },
                (MessageChoice::Dismiss(_), None) => false,
            };
            activated(
                ui,
                pressed,
                clicked,
                self.words.as_str(),
                Pressed::Choice(at),
            );
        }
    }

    /// Undo's label for the room after the message's icon.
    fn undo_words(&self, ui: &Ui, step: &UndoStep) -> String {
        let label = step.label();
        let action = Action::quiet(&label)
            .icon(icons::UNDO)
            .density(Density::Read);
        let chrome = action.natural_width(ui) - number_width(ui, &label, &Type::Strong.font(ui));
        let inset = S + 1.0 + number_width(ui, self.tone.parts().0, &Type::Body.font(ui)) + XS;
        let room = ui.available_width().min(LIST_MEASURE) - inset - chrome;
        step.fit(ui, room)
    }
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

/// Where a fault is: a name, and the line of its file when it has one.
pub struct FaultPlace<'a> {
    pub name: Cow<'a, str>,
    pub line: Option<u32>,
}

/// A fault of the DM's files or of a write. Every view draws it through a typed place, which
/// takes its tone from `severity` and its words from `fault_words`. No place takes a `Tone`:
///
/// ```
/// use splat::outcome::{Fault, Sentence, Severity, Tone};
/// struct Missing;
/// impl Fault for Missing {
///     fn severity(&self) -> Severity { Severity::NeedsFixing }
///     fn cause(&self) -> Sentence { splat::sentence!("The note is gone.") }
/// }
/// fn f(ui: &mut eframe::egui::Ui) { splat::outcome::row_fault(ui, &Missing); }
/// ```
/// ```compile_fail
/// use splat::outcome::{Fault, Sentence, Severity, Tone};
/// struct Missing;
/// impl Fault for Missing {
///     fn severity(&self) -> Severity { Severity::NeedsFixing }
///     fn cause(&self) -> Sentence { splat::sentence!("The note is gone.") }
/// }
/// fn f(ui: &mut eframe::egui::Ui) { splat::outcome::row_fault(ui, &Missing, Tone::Error); }
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
/// use splat::outcome::Repair;
/// let name = String::from("Bridge Ambush");
/// let _ = Repair::new(splat::choice!("Repair {}", name), 1);
/// let _ = Repair::new("Open in Library", 2);
/// ```
/// ```compile_fail
/// use splat::outcome::Repair;
/// let name = String::from("Bridge Ambush");
/// let _ = Repair::new(format!("Repair {name}"), 1);
/// ```
///
/// Its lock has the two forms of [`Choice`]'s:
///
/// ```
/// use splat::outcome::Repair;
/// let unsaved = true;
/// let _ = Repair::new("Replace with a basic manifest", 1)
///     .locked_when(unsaved, splat::sentence!("Save or discard campaign changes first."));
/// let _ = Repair::new("Replace with a basic manifest", 1)
///     .blocked(splat::sentence!("Save or discard campaign changes first."));
/// ```
/// ```compile_fail
/// use splat::outcome::Repair;
/// let unsaved = true;
/// let _ = Repair::new("Replace with a basic", 1)
///     .blocked(unsaved.then_some(splat::sentence!("Save or discard campaign changes first.")));
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
}

impl<A> BodyFault<A> {
    pub fn new(fault: &dyn Fault) -> Self {
        Self {
            tone: fault.severity().tone(),
            words: fault_words(fault),
            repairs: Vec::new(),
        }
    }

    pub fn repair(mut self, repair: Repair<A>) -> Self {
        self.repairs.push(repair);
        self
    }

    /// Draw it at the cursor, capped at `LIST_MEASURE`; the caller's id of the repair pressed.
    pub fn show(self, ui: &mut Ui) -> Option<A> {
        let Self {
            tone,
            words,
            repairs,
        } = self;
        let mut pressed = None;
        let response = ui
            .scope(|ui| {
                ui.set_max_width(ui.available_width().min(LIST_MEASURE));
                let text = words.as_str();
                tone_message(ui, tone, text);
                for (at, repair) in repairs.iter().enumerate() {
                    let clicked = message_choice_row(ui, tone, |ui| {
                        let room = ui.available_width();
                        let lock = holding(&repair.lock);
                        data_choice(ui, &repair.words, ChoiceKind::Safe, room, lock).clicked()
                    });
                    activated(ui, &mut pressed, clicked, text, Pressed::Choice(at));
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
                        message_choice_row(ui, tone, |ui| {
                            super::components::help(ui, help.as_str());
                        });
                    }
                }
            })
            .response;
        register(ui, response.rect, MessagePlace::Page, tone);
        match pressed? {
            Pressed::Choice(at) => repairs.into_iter().nth(at).map(|repair| repair.id),
            Pressed::Undo => None,
        }
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
#[cfg(test)]
pub const MESSAGE_RECTS: &str = "design_message_rects";

fn register(ui: &Ui, rect: Rect, place: MessagePlace, tone: Tone) {
    #[cfg(test)]
    super::components::keep_for_pass(ui, MESSAGE_RECTS, (rect, place, tone));
    #[cfg(not(test))]
    let _ = (ui, rect, place, tone);
}

/// Each message a typed place drew in the last pass that drew one, with its place. Only test
/// builds keep them; the old message functions register nothing.
#[cfg(test)]
pub fn message_rects(ctx: &eframe::egui::Context) -> Vec<(Rect, MessagePlace, Tone)> {
    ctx.data(|data| {
        data.get_temp::<(u64, Vec<(Rect, MessagePlace, Tone)>)>(eframe::egui::Id::new(
            MESSAGE_RECTS,
        ))
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
    use eframe::egui;
    use egui_kittest::{Harness, kittest::Queryable};

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
        // M4 gap 3: a first word that is a path or a file name keeps its letters (Campaign
        // content's `wrong-package/ holds …`, `package.md changed on disk`); words that only
        // end with a mark still get their capital.
        for data in [
            "wrong-package/ holds a different package.",
            "package.md changed on disk.",
            "notes\\npc.md: no such file.",
            "v2.json, line 3: a list opens here.",
        ] {
            assert_eq!(outside(data).as_deref(), Some(data));
        }
        assert_eq!(
            outside("package.md changed on disk").as_deref(),
            Some("package.md changed on disk.")
        );
        assert_eq!(
            outside("removed: the wolf").as_deref(),
            Some("Removed: the wolf.")
        );
        assert_eq!(outside("ends. here").as_deref(), Some("Ends. here."));
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

    /// One fault drawn in every place has one tone, its severity's, and one form of words.
    #[test]
    #[allow(
        clippy::disallowed_methods,
        reason = "the test draws one fault in every body place"
    )]
    fn one_fault_has_one_tone_in_every_place() {
        let mut h = Harness::builder()
            .with_size(egui::vec2(600.0, 500.0))
            .build_ui(|ui| {
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
        crate::design::theme::install(&h.ctx);
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
    /// first letter; its links are targets of at least `XL` whose names name the file, and a
    /// press opens the file without a press of the pane. A path wider than its line is cut in
    /// the middle and keeps its file name, with the whole path as its name and in its hover. 30
    /// such messages at 960 × 640 keep the footer within a third of the pane, and the body in
    /// view above it (the footer's region scrolls: `Footer`'s own proof).
    #[test]
    fn a_footer_file_line_keeps_its_path_and_opens_its_file() {
        use egui::accesskit::Role;
        const BODY: &str = "The body starts here.";
        const ROOT: &str = "/vault/Ashen Marches";
        const SHORT: &str = "wrong-package/package.md";
        const LONG: &str = "Sessions/Session 12 at the Northern Bridge Crossing of Ravenholm/Notes of the wardens/Timeline.md";
        struct Fixture {
            width: f32,
            count: usize,
            relative: &'static str,
            pressed: usize,
        }
        let fault = Hp(Severity::NeedsFixing, Some(9));
        let harness = |size: egui::Vec2, width: f32, count: usize, relative: &'static str| {
            let mut h = Harness::builder().with_size(size).build_ui_state(
                move |ui, fixture: &mut Fixture| {
                    let fault = Hp(Severity::NeedsFixing, Some(9));
                    ui.set_max_width(fixture.width);
                    let frame = PaneFrame::new(ui);
                    let mut messages =
                        Messages::<u8>::new(&frame, "file_line_footer", ui.available_width());
                    for at in 0..fixture.count {
                        let message = FaultMessage::new(&fault)
                            .choice(Choice::new(MessageChoice::Choice("Try again"), at as u8));
                        messages.outcome(
                            message.file(FileLine::new(ROOT, fixture.relative).edit().show()),
                        );
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
                    relative,
                    pressed: 0,
                },
            );
            crate::design::theme::install(&h.ctx);
            h.run_steps(4);
            h
        };
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
        let whole = |relative: &str| format!("{ROOT}/{relative}");
        // A node by the name the part gave it (`named`), not the words of its text run.
        let named = |name: String| {
            egui_kittest::kittest::By::new()
                .predicate(move |node| node.data().label() == Some(name.as_str()))
        };
        let words = fault_words(&fault);

        // Wide: the path whole, as it is on disk (no capital), at the words' first letter,
        // then its two links on the same line.
        let mut h = harness(egui::vec2(960.0, 640.0), 960.0, 1, SHORT);
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
            let name = format!("{word}: package.md");
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
        h.get_by_label("Show file: package.md").click();
        let mut opened: Vec<String> = Vec::new();
        for _ in 0..3 {
            h.step();
            opened.extend(
                h.output()
                    .platform_output
                    .commands
                    .iter()
                    .filter_map(|command| match command {
                        egui::OutputCommand::OpenUrl(open) => Some(open.url.clone()),
                        _ => None,
                    }),
            );
        }
        assert_eq!(
            opened,
            ["file:///vault/Ashen%20Marches/wrong-package/package.md"],
            "Show file opens the file"
        );
        assert_eq!(
            h.state().pressed,
            0,
            "a link opens its file, not a press of the pane"
        );

        // Narrow, with a long path: the path takes its own line, cut in the middle, and keeps
        // its file name; its name and hover keep it whole; the links wrap under it.
        let mut h = harness(egui::vec2(960.0, 640.0), 320.0, 1, LONG);
        let said = h.get_by_role_and_label(Role::Label, words.as_str()).rect();
        let node = h.query_all(named(whole(LONG))).next().expect("the path");
        let path = node.rect();
        let shown = painted(&h, path).concat();
        assert!(
            shown.contains('…')
                && shown.starts_with("Sessions/")
                && shown.ends_with("/Timeline.md"),
            "cut in the middle, the file name kept: {shown:?}"
        );
        assert!(path.right() <= 321.0, "inside its line: {path:?}");
        assert!(
            (path.left() - said.left()).abs() <= 1.0,
            "the path {path:?} at the words {said:?}"
        );
        let show = h.get_by_label("Edit in Obsidian: Timeline.md").rect();
        assert!(
            show.top() >= path.bottom() - 1.0 && (show.left() - said.left()).abs() <= 1.0,
            "the links wrap under the path, at the words: {show:?}"
        );
        node.hover();
        h.run_steps(3);
        assert!(
            h.output()
                .shapes
                .iter()
                .any(|clipped| matches!(&clipped.shape,
                    egui::Shape::Text(text) if text.galley.job.text == whole(LONG))),
            "the hover shows the whole path"
        );

        // 30 messages with file lines at 960 × 640: the footer takes at most a third of the
        // pane, and the body stays in view above it.
        let h = harness(egui::vec2(960.0, 640.0), 960.0, 30, SHORT);
        let footer = crate::design::pane::footer_rect(&h.ctx, egui::Id::new("file_line_footer"))
            .expect("the footer is drawn");
        let body = h.get_by_label(BODY).rect();
        assert!(
            footer.height() <= 640.0 / 3.0 + 1.0
                && body.bottom() <= footer.top()
                && body.top() >= 0.0,
            "30 file lines: footer {footer:?}, body {body:?}"
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
            });
        });
    }

    /// M3 gap 1, in the pane's footer: the visible Undo words are `fit`'s for the line's room,
    /// and the accessible name is always the whole form. Narrow: `Undo discard` named
    /// `Undo discard of Bridge Ambush`; a long object of a verb with no joining word is cut in
    /// the visible words only. Wide: the whole form shows. The press reaches its entry.
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
        let discard = || {
            let mut app = two();
            app.older_line = OlderLine::Discard("Bridge Ambush");
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
        // A long object with no joining word: cut by whole words in the visible words only.
        let long = "Bridge Ambush at the Ravenholm Ironside Crossing";
        let mut narrow = two();
        narrow.older_line = OlderLine::Remove(long);
        narrow.narrow = Some(320.0);
        let h = harness_sized(narrow, 960.0);
        let name = format!("Undo remove {long}");
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
    #[test]
    #[allow(
        clippy::disallowed_methods,
        reason = "the test of the body place `BodyFault::show` itself"
    )]
    fn a_body_fault_gives_its_repairs_back_by_id() {
        use egui_kittest::kittest::NodeT;
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        enum Fix {
            Open,
            Replace,
            Repair(u8),
        }
        #[derive(Default)]
        struct State {
            locked: bool,
            pressed: Vec<Fix>,
            /// The right edge of the fault's 320 px column.
            right: f32,
        }
        const PATH: &str =
            "Packages/Ravenholm Ironside Wardens/Encounters/Bridge Ambush/manifest.md";
        const LOCK: Sentence = crate::sentence!("Save or discard campaign changes first.");
        const LONG: &str = "Ravenholm Ironside Wardens of the Northern Bridge Crossing";
        let mut h = Harness::builder()
            .with_size(egui::vec2(960.0, 600.0))
            .build_ui_state(
                |ui, state: &mut State| {
                    let fault = Hp(Severity::NeedsFixing, None);
                    ui.allocate_ui_with_layout(
                        egui::vec2(320.0, 600.0),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            state.right = ui.max_rect().right();
                            let pressed = BodyFault::new(&fault)
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
                        },
                    );
                },
                State::default(),
            );
        crate::design::theme::install(&h.ctx);
        h.run_steps(4);
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
            h.get_by_label("Repair Cinder Wolf").rect().top(),
            open,
            "a lock moves nothing"
        );
        h.get_by_label(&name).click();
        h.run_steps(2);
        assert!(h.state().pressed.is_empty());

        // Review of PR 332, 4: narrower than the shortest cut, the words wrap inside the row;
        // every fixed word is painted, none is cut.
        const WHOLE: &str = "Repair Ravenholm in Library";
        let mut h = Harness::builder()
            .with_size(egui::vec2(400.0, 300.0))
            .build_ui_state(
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
        crate::design::theme::install(&h.ctx);
        h.run_steps(4);
        let node = h.get_by_label(WHOLE).rect();
        let wrapped = h.output().shapes.iter().any(|clipped| {
            matches!(&clipped.shape, egui::Shape::Text(text)
                if text.galley.job.text == WHOLE && text.galley.rows.len() > 1 && !text.galley.elided)
        });
        assert!(wrapped, "the whole words, wrapped, nothing cut");
        assert!(node.right() <= *h.state() + 0.5, "inside the row: {node:?}");
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
        use eframe::egui;
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
            /// What the older line says.
            pub older_line: OlderLine,
            /// The older line has a choice whose words hold two objects from data (M3).
            pub moves: Option<[&'static str; 2]>,
            /// It also has a red choice with data, given before `Move` (the part puts it after).
            pub deletes: bool,
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
        fn show_active(ui: &mut Ui, dndbeyond: &mut DndBeyond, change: InBody) {
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
        }

        /// One pane: its footer shows the store's kept lines; its body changes the owner.
        fn one_pane(ui: &mut Ui, app: &mut App, pane: usize) {
            let live = app.run_live;
            let older_undo_open = app.older_undo_open;
            let older_line = app.older_line;
            let frame = PaneFrame::new(ui);
            let mut messages = Messages::new(&frame, ("pilot", pane), ui.available_width());
            for owner in ["older", "newer", "third"] {
                if let Some(shown) = app.dndbeyond.results.get(&Campaign(owner)) {
                    let two_choices = app.two_choices;
                    let moves = app.moves;
                    let deletes = app.deletes;
                    let line = shown.outcome(&frame, |saved| {
                        match (owner, older_line) {
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
                            _ => {}
                        }
                        let mut said = Said::new(crate::sentence!("{}.", saved.1)).undo(
                            UndoStep::verb(Verb::Change)
                                .tell_apart(owner)
                                .pressed_as(Act::Undo)
                                .locked_when(live && !(owner == "older" && older_undo_open), LOCK),
                        );
                        if owner == "older" {
                            // A choice attached to a kept result.
                            said = said.choice(
                                Choice::new(
                                    MessageChoice::Choice("Register as a new campaign"),
                                    Act::Register,
                                )
                                .locked_when(live, LOCK),
                            );
                            if let Some([a, b]) = moves {
                                if deletes {
                                    said = said.choice(Choice::named_as(
                                        ChoiceKind::Lose,
                                        crate::choice!("Delete {}", a),
                                        Act::Delete,
                                    ));
                                }
                                said = said.choice(Choice::named(
                                    crate::choice!("Move {} before {}", a, b),
                                    Act::Rename,
                                ));
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
            let paned = messages.pane(ui, crate::design::Body::Content, |ui| {
                show_active(ui, &mut app.dndbeyond, change);
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
            if app.two_panes {
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
            let mut h = Harness::builder()
                .with_size(egui::vec2(width, 500.0))
                .build_ui_state(frame, app);
            crate::design::theme::install(&h.ctx);
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
    }
}
