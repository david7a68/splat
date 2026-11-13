//! Messages and their words, by type (round 28, package M). A caller gives data: what was done
//! (`Verb` and object, or a `Sentence`), how to undo it (`UndoStep`), which choices follow
//! (`MessageChoice`), or a `Fault`. This module decides the rest: the place, the order, the tone,
//! the form of the words, the weight of each action and the gaps. Nothing here takes a `Tone`, a
//! draw closure or an `Action`, so a caller cannot choose them.
//!
//! The typed places are the only ways to draw a message:
//!
//! - `Footer::outcome`: every result, reason and notice that comes and goes, in its pane's footer.
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
    Action, FooterRank, link, number_width, small_tone_message, tone_message, undo_message_capped,
};
pub use super::components::{Footer, ResultOrder, Tone};
use super::{icons, tokens::*};
use eframe::egui::{Rect, Response, Ui};
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
    /// of the DM's own table. They start with a capital and end with an end mark, whatever came.
    /// Blank words are no sentence: `None`, and the caller says what it knows in its own
    /// checked words. The source lint counts the uses of this escape per file.
    pub fn outside(words: impl Into<String>) -> Option<Self> {
        let words: String = words.into();
        let mut words = words.trim().to_owned();
        let first = words.chars().next()?;
        if first.is_lowercase() {
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

fn ends_with_mark(words: &str) -> bool {
    words
        .trim_end_matches(['”', '’', ')'])
        .ends_with(['.', '?', '!'])
}

/// A sentence whose template is checked at compile time. See `Sentence`.
#[macro_export]
macro_rules! sentence {
    ($template:literal $(,)?) => {{
        const SENTENCE: $crate::outcome::Sentence = $crate::outcome::Sentence::__literal($template);
        SENTENCE
    }};
    ($template:literal, $($values:tt)+) => {{
        const CHECKED: $crate::outcome::Checked = $crate::outcome::check_template($template);
        $crate::outcome::Sentence::__formatted(CHECKED, ::std::format!($template, $($values)+))
    }};
}

// ------------------------------------------------------------------------------------------------
// Verbs and Undo

/// What an action did. A result says its past form (`Removed Take seal.`); its Undo says its
/// present form (`Undo remove Take seal`), from the same value, so the two cannot differ.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verb {
    Add,
    Change,
    Copy,
    Delete,
    Discard,
    Dismiss,
    Link,
    Mark,
    Move,
    Register,
    Reload,
    Remove,
    Rename,
    Replace,
    Restore,
    Write,
}

impl Verb {
    /// The word after `Undo`: `remove`.
    pub const fn word(self) -> &'static str {
        match self {
            Self::Add => "add",
            Self::Change => "change",
            Self::Copy => "copy",
            Self::Delete => "delete",
            Self::Discard => "discard",
            Self::Dismiss => "dismiss",
            Self::Link => "link",
            Self::Mark => "mark",
            Self::Move => "move",
            Self::Register => "register",
            Self::Reload => "reload",
            Self::Remove => "remove",
            Self::Rename => "rename",
            Self::Replace => "replace",
            Self::Restore => "restore",
            Self::Write => "write",
        }
    }

    /// The first word of a result: `Removed`.
    pub const fn past(self) -> &'static str {
        match self {
            Self::Add => "Added",
            Self::Change => "Changed",
            Self::Copy => "Copied",
            Self::Delete => "Deleted",
            Self::Discard => "Discarded",
            Self::Dismiss => "Dismissed",
            Self::Link => "Linked",
            Self::Mark => "Marked",
            Self::Move => "Moved",
            Self::Register => "Registered",
            Self::Reload => "Reloaded",
            Self::Remove => "Removed",
            Self::Rename => "Renamed",
            Self::Replace => "Replaced",
            Self::Restore => "Restored",
            Self::Write => "Wrote",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Step {
    /// `Undo remove`, `Undo remove Take seal`.
    Verb(Verb, Option<String>),
    /// An act the DM named: `Undo: Take seal`.
    Act(String),
}

/// What an Undo takes back, as its label says it. Its fields are private: a label is made only
/// from a `Verb` (and an object), or from the name of an act, and the part that tells two steps
/// apart (a copy number, a folder) is kept whole by every fit.
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

    /// `Undo <verb> <object>`: `Undo remove Take seal`.
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
            label.push_str(object);
        }
        if let Some(part) = &self.tell_apart {
            label.push(' ');
            label.push_str(part);
        }
        label
    }

    /// `Undo remove` or `Undo:`, and the object.
    fn parts(&self) -> (String, Option<&str>) {
        match &self.step {
            Step::Verb(verb, object) => (format!("Undo {}", verb.word()), object.as_deref()),
            Step::Act(label) => ("Undo:".to_owned(), Some(label.as_str())),
        }
    }

    /// The label's words for `width` in the action's font: the whole label when it fits, else
    /// the object shortened by whole words with `…`. `Undo`, the verb and the part that tells
    /// steps apart always stay, also when they alone are wider.
    pub fn fit(&self, ui: &Ui, width: f32) -> String {
        let font = Type::Strong.font(ui);
        let whole = self.label();
        if number_width(ui, &whole, &font) <= width {
            return whole;
        }
        let (head, object) = self.parts();
        let tail = self
            .tell_apart
            .as_ref()
            .map_or(String::new(), |part| format!(" {part}"));
        let words: Vec<&str> = object.map_or(Vec::new(), |o| o.split(' ').collect());
        for keep in (1..words.len()).rev() {
            let shown = format!("{head} {}…{tail}", words[..keep].join(" "));
            if number_width(ui, &shown, &font) <= width {
                return shown;
            }
        }
        format!("{head} …{tail}")
    }
}

// ------------------------------------------------------------------------------------------------
// Choices and presses

/// An action that follows a message's words, by what it does. The weight comes from the kind:
/// a `Choice` is bordered, a `Lose` is red with exactly `L` before it, a `Link` is a link. An
/// Undo is not a choice: it is the message's `UndoStep`, always quiet with its icon, and first.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MessageChoice<'a> {
    Choice(&'a str),
    Lose(&'a str),
    Link {
        word: &'a str,
        hover: Option<&'a str>,
    },
}

/// What the DM pressed in a message: its Undo, or its choice at this index.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pressed {
    Undo,
    Choice(usize),
}

// ------------------------------------------------------------------------------------------------
// Outcomes

/// A finished change, in the success tone: `<Verb past> <object>.`, then more sentences. Its
/// Undo is made from the same `Verb` and object (`undo`), so the result and its Undo cannot
/// have two verbs. It needs the order its action issued.
///
/// ```
/// use splat::outcome::{Done, ResultOrder, Verb};
/// let _ = Done::new(ResultOrder::issue(), Verb::Remove, "Take seal").undo();
/// ```
/// ```compile_fail
/// use splat::outcome::{Done, ResultOrder, Verb};
/// let _ = Done::new(ResultOrder::issue(), Verb::Remove, "Take seal").undo("Undo discard");
/// ```
///
/// A result without its action's order does not compile:
///
/// ```compile_fail
/// use splat::outcome::{Done, ResultOrder, Verb};
/// let _ = Done::new(Verb::Remove, "Take seal");
/// ```
#[must_use = "give it to `Footer::outcome`"]
pub struct Done<'a> {
    order: ResultOrder,
    verb: Verb,
    object: String,
    more: Option<Sentence>,
    undo: Option<Option<String>>,
    choices: Vec<MessageChoice<'a>>,
}

impl<'a> Done<'a> {
    pub fn new(order: ResultOrder, verb: Verb, object: impl Into<String>) -> Self {
        Self {
            order,
            verb,
            object: object.into(),
            more: None,
            undo: None,
            choices: Vec::new(),
        }
    }

    /// Sentences after the first: `With it: Bell is done.`
    pub fn more(mut self, more: Sentence) -> Self {
        self.more = Some(match self.more {
            Some(before) => before.then(more),
            None => more,
        });
        self
    }

    /// Its Undo is live: `Undo <verb> <object>`.
    pub fn undo(mut self) -> Self {
        self.undo = Some(None);
        self
    }

    /// Its Undo is live, with the part that tells it from another step of the same words.
    pub fn undo_told_apart(mut self, part: impl Into<String>) -> Self {
        self.undo = Some(Some(part.into()));
        self
    }

    pub fn choice(mut self, choice: MessageChoice<'a>) -> Self {
        self.choices.push(choice);
        self
    }

    /// `Removed Take seal.`, then the other sentences.
    pub fn words(&self) -> Sentence {
        let first = Sentence(Cow::Owned(format!(
            "{} {}.",
            self.verb.past(),
            self.object.trim_end_matches('.')
        )));
        match &self.more {
            Some(more) => first.then(more.clone()),
            None => first,
        }
    }
}

/// A finished change in the domain's own words, in the success tone (`Seals for Party changed to
/// 1.`). Its Undo is an `UndoStep`, never words. `Said` is the named escape for a result that is
/// not a verb and an object: nothing ties its words to its Undo's verb, so
/// `Said::new(order, sentence!("Removed Bell.")).undo(UndoStep::of(Verb::Delete, "Bell"))`
/// compiles. Only `Done` holds one verb for both; a reviewer checks each `Said`.
///
/// ```
/// use splat::outcome::{ResultOrder, Said, UndoStep};
/// let _ = Said::new(ResultOrder::issue(), splat::sentence!("Bell is done.")).undo(UndoStep::act("Done"));
/// ```
/// ```compile_fail
/// use splat::outcome::{ResultOrder, Said, UndoStep};
/// let _ = Said::new(ResultOrder::issue(), splat::sentence!("Bell is done.")).undo("Undo: Done");
/// ```
///
/// No outcome takes a tone:
///
/// ```compile_fail
/// use splat::outcome::{ResultOrder, Said, Tone, UndoStep};
/// let _ = Said::new(ResultOrder::issue(), splat::sentence!("Bell is done.")).tone(Tone::Info);
/// ```
#[must_use = "give it to `Footer::outcome`"]
pub struct Said<'a> {
    order: ResultOrder,
    words: Sentence,
    undo: Option<UndoStep>,
    choices: Vec<MessageChoice<'a>>,
}

impl<'a> Said<'a> {
    pub fn new(order: ResultOrder, words: Sentence) -> Self {
        Self {
            order,
            words,
            undo: None,
            choices: Vec::new(),
        }
    }

    pub fn undo(mut self, step: UndoStep) -> Self {
        self.undo = Some(step);
        self
    }

    pub fn choice(mut self, choice: MessageChoice<'a>) -> Self {
        self.choices.push(choice);
        self
    }
}

/// A notice in the info tone: guidance, why other actions wait, or a result that wrote nothing
/// (`Draft discarded.`). With an Undo it is a result, and needs its action's order.
#[must_use = "give it to `Footer::outcome`"]
pub struct Notice<'a> {
    words: Sentence,
    undo: Option<(ResultOrder, UndoStep)>,
    choices: Vec<MessageChoice<'a>>,
}

impl<'a> Notice<'a> {
    pub fn new(words: Sentence) -> Self {
        Self {
            words,
            undo: None,
            choices: Vec::new(),
        }
    }

    pub fn undo(mut self, order: ResultOrder, step: UndoStep) -> Self {
        self.undo = Some((order, step));
        self
    }

    pub fn choice(mut self, choice: MessageChoice<'a>) -> Self {
        self.choices.push(choice);
        self
    }
}

/// A fault as a footer message: its tone comes from its `Severity`, its words from
/// `fault_words`. A fault that left a way back (an event that waits in a draft) has an Undo.
#[must_use = "give it to `Footer::outcome`"]
pub struct FaultMessage<'a> {
    fault: &'a dyn Fault,
    undo: Option<(ResultOrder, UndoStep)>,
    choices: Vec<MessageChoice<'a>>,
}

impl<'a> FaultMessage<'a> {
    pub fn new(fault: &'a dyn Fault) -> Self {
        Self {
            fault,
            undo: None,
            choices: Vec::new(),
        }
    }

    pub fn undo(mut self, order: ResultOrder, step: UndoStep) -> Self {
        self.undo = Some((order, step));
        self
    }

    pub fn choice(mut self, choice: MessageChoice<'a>) -> Self {
        self.choices.push(choice);
        self
    }
}

/// Everything a pane's footer can hold. The variant decides the tone and the place in the
/// footer's order:
///
/// | Variant | Tone | Place |
/// |---|---|---|
/// | `Done`, `Said` | success | first while its Undo is live, newest first; else after notices |
/// | `Notice` | info | with notices; first, as a result, while its Undo is live |
/// | `Refused` | warning | reasons |
/// | `Fault` | from its `Severity` | reasons; first, as a result, while its Undo is live |
///
/// An `Outcome` has no public draw method; it reaches the screen only through `Footer::outcome`:
///
/// ```
/// fn f(ui: &mut eframe::egui::Ui) {
///     let mut pressed = None;
///     let mut footer = splat::outcome::Footer::new("f", 400.0);
///     footer.outcome(splat::outcome::Outcome::Refused(splat::sentence!("No.")), &mut pressed);
///     let _ = footer.show(ui);
/// }
/// ```
/// ```compile_fail
/// fn f(ui: &mut eframe::egui::Ui) {
///     splat::outcome::Outcome::Refused(splat::sentence!("No.")).draw(ui);
/// }
/// ```
pub enum Outcome<'a> {
    Done(Done<'a>),
    Said(Said<'a>),
    Notice(Notice<'a>),
    /// Why an act or Save wrote nothing: it does not fit the files now.
    Refused(Sentence),
    Fault(FaultMessage<'a>),
}

impl<'a> From<Done<'a>> for Outcome<'a> {
    fn from(done: Done<'a>) -> Self {
        Self::Done(done)
    }
}

impl<'a> From<Said<'a>> for Outcome<'a> {
    fn from(said: Said<'a>) -> Self {
        Self::Said(said)
    }
}

impl<'a> From<Notice<'a>> for Outcome<'a> {
    fn from(notice: Notice<'a>) -> Self {
        Self::Notice(notice)
    }
}

impl<'a> From<FaultMessage<'a>> for Outcome<'a> {
    fn from(fault: FaultMessage<'a>) -> Self {
        Self::Fault(fault)
    }
}

/// A message as drawn: tone, words, Undo and choices.
struct Body<'a> {
    tone: Tone,
    words: Sentence,
    undo: Option<UndoStep>,
    choices: Vec<MessageChoice<'a>>,
}

impl<'a> Outcome<'a> {
    /// The footer's rank and order, and what it draws.
    fn parts(self) -> (FooterRank, Option<ResultOrder>, Body<'a>) {
        let result = |undo: bool| {
            if undo {
                FooterRank::Undo
            } else {
                FooterRank::Notice
            }
        };
        match self {
            Self::Done(done) => {
                let words = done.words();
                let undo = done.undo.map(|part| {
                    let step = UndoStep::of(done.verb, done.object.trim_end_matches('.'));
                    match part {
                        Some(part) => step.tell_apart(part),
                        None => step,
                    }
                });
                (
                    result(undo.is_some()),
                    Some(done.order),
                    Body {
                        tone: Tone::Success,
                        words,
                        undo,
                        choices: done.choices,
                    },
                )
            }
            Self::Said(said) => (
                result(said.undo.is_some()),
                Some(said.order),
                Body {
                    tone: Tone::Success,
                    words: said.words,
                    undo: said.undo,
                    choices: said.choices,
                },
            ),
            Self::Notice(notice) => {
                let (rank, order, undo) = match notice.undo {
                    Some((order, step)) => (FooterRank::Undo, Some(order), Some(step)),
                    None => (FooterRank::Notice, None, None),
                };
                (
                    rank,
                    order,
                    Body {
                        tone: Tone::Info,
                        words: notice.words,
                        undo,
                        choices: notice.choices,
                    },
                )
            }
            Self::Refused(words) => (
                FooterRank::Reason,
                None,
                Body {
                    tone: Tone::Warning,
                    words,
                    undo: None,
                    choices: Vec::new(),
                },
            ),
            Self::Fault(fault) => {
                let (rank, order, undo) = match fault.undo {
                    Some((order, step)) => (FooterRank::Undo, Some(order), Some(step)),
                    None => (FooterRank::Reason, None, None),
                };
                (
                    rank,
                    order,
                    Body {
                        tone: fault.fault.severity().tone(),
                        words: fault_words(fault.fault),
                        undo,
                        choices: fault.choices,
                    },
                )
            }
        }
    }
}

impl<'a> Footer<'a> {
    /// A typed message in this footer, at the place its variant gives it in the footer's order
    /// (see `Outcome`). When the DM presses its Undo or a choice, `pressed` says which, after
    /// `show`. The footer's own tests prove its order, place and press routing; a caller that
    /// uses only this method repeats none of them.
    pub fn outcome(
        &mut self,
        outcome: impl Into<Outcome<'a>>,
        pressed: &'a mut Option<Pressed>,
    ) -> &mut Self {
        let (rank, order, body) = outcome.into().parts();
        let words = Cow::Owned(body.words.as_str().to_owned());
        self.push(rank, order, words, move |ui| {
            if let Some(press) = body.draw(ui, MessagePlace::Footer) {
                *pressed = Some(press);
            }
        })
    }
}

impl Body<'_> {
    /// The words, capped at `LIST_MEASURE`; an Undo alone beside them when it fits, else under
    /// them at their first letter; with choices, the Undo and then the choices on rows under the
    /// words, from their first letter.
    fn draw(&self, ui: &mut Ui, place: MessagePlace) -> Option<Pressed> {
        let mut pressed = None;
        let response = ui
            .scope(|ui| {
                ui.set_max_width(ui.available_width().min(LIST_MEASURE));
                let words = self.words.as_str();
                if self.choices.is_empty() {
                    match &self.undo {
                        None => {
                            tone_message(ui, self.tone, words);
                        }
                        Some(step) => {
                            let label = step.label();
                            let shown = self.undo_words(ui, step);
                            let name = (shown != label).then_some(label.as_str());
                            if undo_message_capped(ui, self.tone, words, &shown, name) {
                                pressed = Some(Pressed::Undo);
                            }
                        }
                    }
                    return;
                }
                let message = tone_message(ui, self.tone, words);
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
                    .choices
                    .iter()
                    .map(|choice| self.choice_width(ui, *choice) + gap)
                    .sum();
                let one_line =
                    inset + undo.as_ref().map_or(0.0, |(_, _, width)| width + L) + choices
                        <= ui.available_width();
                if undo.is_some() || one_line {
                    ui.horizontal(|ui| {
                        ui.add_space(inset);
                        if let Some((label, shown, _)) = &undo {
                            // Its words start at the message's words, its icon in the message's
                            // icon column.
                            let mut action = Action::quiet(shown)
                                .icon(icons::UNDO)
                                .density(Density::Read)
                                .line_start_in_column();
                            if shown != label {
                                action = action.name(label);
                            }
                            if action.show(ui).clicked() {
                                pressed = Some(Pressed::Undo);
                            }
                            ui.add_space((L - gap).max(0.0));
                        }
                        if one_line && let Some(at) = self.draw_choices(ui) {
                            pressed = Some(Pressed::Choice(at));
                        }
                    });
                }
                if !one_line {
                    ui.horizontal_top(|ui| {
                        ui.add_space(inset);
                        ui.horizontal_wrapped(|ui| {
                            if let Some(at) = self.draw_choices(ui) {
                                pressed = Some(Pressed::Choice(at));
                            }
                        });
                    });
                }
            })
            .response;
        register(ui, response.rect, place, self.tone);
        pressed
    }

    /// A choice's whole width, with the `L` before a red one.
    fn choice_width(&self, ui: &Ui, choice: MessageChoice<'_>) -> f32 {
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
        }
    }

    /// The choices, in order: a red one keeps exactly `L` from the one before it. The pressed
    /// choice's index.
    fn draw_choices(&self, ui: &mut Ui) -> Option<usize> {
        let gap = ui.spacing().item_spacing.x;
        let mut pressed = None;
        for (at, choice) in self.choices.iter().enumerate() {
            let clicked = match *choice {
                MessageChoice::Choice(word) => Action::secondary(word)
                    .density(Density::Read)
                    .show(ui)
                    .clicked(),
                MessageChoice::Lose(word) => {
                    if at > 0 {
                        ui.add_space((L - gap).max(0.0));
                    }
                    Action::destructive(word)
                        .density(Density::Read)
                        .show(ui)
                        .clicked()
                }
                MessageChoice::Link { word, hover } => {
                    let response = link(ui, word);
                    match hover {
                        Some(hover) => response.on_hover_text(hover),
                        None => response,
                    }
                    .clicked()
                }
            };
            if clicked {
                pressed = Some(at);
            }
        }
        pressed
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
/// repair it under the words.
#[must_use = "call show"]
pub struct PagePlaceholder<'a> {
    fault: &'a dyn Fault,
    choices: Vec<MessageChoice<'a>>,
}

impl<'a> PagePlaceholder<'a> {
    pub fn new(fault: &'a dyn Fault) -> Self {
        Self {
            fault,
            choices: Vec::new(),
        }
    }

    pub fn choice(mut self, choice: MessageChoice<'a>) -> Self {
        self.choices.push(choice);
        self
    }

    /// Draw it at the cursor; the pressed choice.
    pub fn show(self, ui: &mut Ui) -> Option<Pressed> {
        Body {
            tone: self.fault.severity().tone(),
            words: fault_words(self.fault),
            undo: None,
            choices: self.choices,
        }
        .draw(ui, MessagePlace::Page)
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

/// `{chosen} of {total} <unit> chosen`: the one form of a count of chosen things. The phrase
/// lint keeps this form here.
pub fn chosen_count(chosen: usize, total: usize, unit: &str) -> String {
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

    fn rejected(template: &'static str) -> bool {
        std::panic::catch_unwind(|| {
            check_template(template);
        })
        .is_err()
    }

    /// The template rules, as `sentence!` applies them at compile time.
    #[test]
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
    fn outside_words_get_a_capital_and_an_end_mark() {
        let outside = |words: &str| Sentence::outside(words).map(|s| s.to_string());
        assert_eq!(
            outside("no such file (os error 2)").as_deref(),
            Some("No such file (os error 2).")
        );
        assert_eq!(outside(" Done. ").as_deref(), Some("Done."));
        assert_eq!(outside("Go “now”").as_deref(), Some("Go “now”."));
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
        let order = ResultOrder::issue();
        let ranks = [
            Outcome::from(Done::new(order, Verb::Remove, "Bell").undo()),
            Outcome::from(Done::new(order, Verb::Remove, "Bell")),
            Outcome::Refused(crate::sentence!("No.")),
            Outcome::from(Notice::new(crate::sentence!("Wait."))),
            Outcome::from(FaultMessage::new(&fault)),
            Outcome::from(FaultMessage::new(&fault).undo(order, UndoStep::verb(Verb::Add))),
        ]
        .map(|outcome| {
            let (rank, order, body) = outcome.parts();
            (rank, order.is_some(), body.tone)
        });
        assert!(
            ranks
                == [
                    (FooterRank::Undo, true, Tone::Success),
                    (FooterRank::Notice, true, Tone::Success),
                    (FooterRank::Reason, false, Tone::Warning),
                    (FooterRank::Notice, false, Tone::Info),
                    (FooterRank::Reason, false, Tone::Error),
                    (FooterRank::Undo, true, Tone::Error),
                ],
            "{:?}",
            ranks.map(|(rank, order, _)| (rank, order))
        );
    }

    /// One fault drawn in every place has one tone, its severity's, and one form of words.
    #[test]
    fn one_fault_has_one_tone_in_every_place() {
        let mut h = Harness::builder()
            .with_size(egui::vec2(600.0, 500.0))
            .build_ui(|ui| {
                let fault = Hp(Severity::NeedsFixing, Some(14));
                banner(ui, Banner::Fault(&fault));
                row_fault(ui, &fault);
                let _ = PagePlaceholder::new(&fault).show(ui);
                let mut pressed = None;
                let mut footer = Footer::new("one_fault", ui.available_width());
                footer.outcome(FaultMessage::new(&fault), &mut pressed);
                let _ = footer.show(ui);
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
                MessagePlace::Footer
            ]
        );
        assert!(
            rects.iter().all(|(_, _, tone)| *tone == Tone::Warning),
            "{rects:?}"
        );
        assert_eq!(
            h.query_all_by_role_and_label(
                egui::accesskit::Role::Label,
                "Cinder Wolf · line 14: The HP is missing."
            )
            .count(),
            4
        );
    }

    /// A result and its Undo say one verb; a fit keeps the verb and the part that tells two
    /// steps apart, and drops whole words of the object.
    #[test]
    fn an_undo_keeps_its_verb_and_what_tells_it_apart() {
        let done = Done::new(ResultOrder::issue(), Verb::Remove, "Take seal: Keyring +1.")
            .more(crate::sentence!("With it: Bell is done."))
            .undo();
        assert_eq!(
            done.words().as_str(),
            "Removed Take seal: Keyring +1. With it: Bell is done."
        );
        let (_, _, body) = Outcome::from(done).parts();
        assert_eq!(
            body.undo.unwrap().label(),
            "Undo remove Take seal: Keyring +1"
        );
        assert_eq!(UndoStep::act("Take seal").label(), "Undo: Take seal");
        assert_eq!(UndoStep::verb(Verb::Add).label(), "Undo add");
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
            });
        });
    }

    /// The footer's typed message: a press on its Undo and on its choice reach the caller, the
    /// words are the footer's name, and the registry holds its rect with its place.
    #[test]
    fn a_footer_outcome_reports_its_presses_and_registers_its_rect() {
        #[derive(Default)]
        struct State {
            pressed: Vec<Pressed>,
        }
        let mut h = Harness::builder()
            .with_size(egui::vec2(600.0, 400.0))
            .build_ui_state(
                |ui, state: &mut State| {
                    let mut pressed = None;
                    let order = ResultOrder::default();
                    let mut footer = Footer::new("outcome_test", ui.available_width());
                    footer.outcome(
                        Said::new(order, crate::sentence!("Seals for Party changed to 1."))
                            .undo(UndoStep::act("Take seal"))
                            .choice(MessageChoice::Choice("Add to Timeline")),
                        &mut pressed,
                    );
                    let bottom = footer.show(ui);
                    ui.allocate_space(egui::vec2(10.0, (bottom - ui.cursor().top()).max(0.0)));
                    state.pressed.extend(pressed);
                },
                State::default(),
            );
        crate::design::theme::install(&h.ctx);
        h.run_steps(4);
        let rects = message_rects(&h.ctx);
        assert!(
            rects.len() == 1 && rects[0].1 == MessagePlace::Footer,
            "{rects:?}"
        );
        let words = h
            .get_by_role_and_label(
                egui::accesskit::Role::Label,
                "Seals for Party changed to 1.",
            )
            .rect()
            .left();
        let undo = h.get_by_label("Undo: Take seal").rect();
        assert!(
            undo.left() < words && rects[0].0.contains_rect(undo),
            "Undo starts its line under the words, inside the message: {undo:?} {words} {rects:?}"
        );
        h.get_by_label("Undo: Take seal").click();
        h.run_steps(2);
        h.get_by_label("Add to Timeline").click();
        h.run_steps(2);
        assert_eq!(h.state().pressed, [Pressed::Undo, Pressed::Choice(0)]);
    }
}
