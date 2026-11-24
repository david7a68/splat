//! Layout data that the app's views share: an action's checked [`Word`], the place of a title
//! line's actions ([`title_placement`]), an editor's back line ([`EditorHead`]) and the reasons a
//! create form's Create waits for ([`CreateReasons`]).
use std::ops::Range;

use super::{L, S, XS};

/// The longest word an action may have, in characters, so that an action with its icon fits on
/// one line in the narrowest side panel.
pub const MAX_WORD_CHARS: usize = 20;

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
            #[allow(
                clippy::disallowed_methods,
                reason = "owner: the design system checks a word (design::results::OWNED_ALLOWS)"
            )]
            let word = $crate::layout::Word::checked($word);
            word
        }
    };
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

/// The head of an Encounters editor (R38): where its back line goes and the draft that holds it.
/// The view draws the back line from [`back_words`](Self::back_words) and, while a draft holds
/// the editor, that draft's reason ([`Unsaved::reason`] or [`Unsaved::short_reason`]).
///
/// ```
/// # use splat_design::layout::{DraftOwner, EditorHead, EditorKind, Unsaved};
/// struct Owner;
/// impl DraftOwner for Owner {
///     fn unsaved(&self, draft: Unsaved) -> bool { draft == Unsaved::Creature }
/// }
/// let head = EditorHead::new(EditorKind::Group { in_plan: false }, &Owner);
/// assert_eq!(head.back_words(), ("Encounters", "Back to encounters"));
/// assert_eq!(head.held_by(), Some(Unsaved::Creature));
/// ```
#[must_use]
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

    /// The back link's words and its accessible name.
    pub fn back_words(&self) -> (&'static str, &'static str) {
        (self.to.word().as_str(), self.to.name())
    }
}

/// Every reason a create form's Create can be blocked with, so a reason that is not declared
/// does not compile. Declare them with [`create_reasons!`](crate::create_reasons), which checks
/// each reason with package M's `sentence!`; the lint refuses an `impl CreateReasons` by hand,
/// which could leave a reason out of `ALL`.
pub trait CreateReasons: Copy + 'static {
    const ALL: &'static [Self];
    fn words(self) -> &'static str;
}

/// Declare an enum of a create form's reasons, with each variant's words. `ALL` holds every
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

// The status part, public here so its doc tests can name it.
pub use super::status::StatusKind;

#[cfg(test)]
pub(crate) mod bans;
#[cfg(test)]
mod tests;
