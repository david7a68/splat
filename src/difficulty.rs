//! Encounter difficulty: how a fight's XP compares with the party's XP budget. The budget per
//! character comes from the rule entry that the campaign uses; no table is written here. Every
//! screen that shows the word calls [`Scale::weigh`], and keeps its answer until an input changes.
use crate::design::memo::{Key, Measure, Memos, Revision};
use serde::Deserialize;
use serde_yaml_ng::Value;
use splat::content::Entry;
use std::collections::BTreeMap;

/// The rule contract that holds the XP budget per character, by level.
pub const CONTRACT: &str = "dnd2024:rules_combat_encounters_step_2_determine_your_xp_budget";

/// Explicit XP wins; a bare CR supplies its standard reward. `None` when the entry has neither.
pub fn creature_xp(entry: &Entry) -> Option<u64> {
    crate::library::presentation::challenge_text(entry)
        .and_then(splat::content::stat_blocks::challenge_xp)
}

/// A player character's level, from its saved sheet. `None` for a creature that is not a
/// character, or a sheet with no level.
pub fn character_level(entry: &Entry) -> Option<Option<u64>> {
    let sheet = entry.fields.get("splat_character")?;
    Some(sheet.get("level").and_then(Value::as_u64))
}

/// The XP budget per character for each level that the rule's table lists.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Budgets(BTreeMap<u64, [u64; 3]>);

impl Budgets {
    /// The table of a rule (its `splat_rule` value), when the rule is the XP budget's contract
    /// and its table reads.
    pub fn of_rule(rule: &Value) -> Option<Self> {
        #[derive(Deserialize)]
        struct Row {
            level: u64,
            low: u64,
            moderate: u64,
            high: u64,
        }
        if rule.get("implementation")?.as_str()?.trim() != CONTRACT {
            return None;
        }
        let rows: Vec<Row> =
            serde_yaml_ng::from_value(rule.get("parameters")?.get("per_character_xp")?.clone())
                .ok()?;
        let table: BTreeMap<_, _> = rows
            .into_iter()
            .map(|row| (row.level, [row.low, row.moderate, row.high]))
            .collect();
        (!table.is_empty()).then_some(Self(table))
    }
}

/// What a fight is weighed against: the campaign's budget table and its party's levels.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Scale {
    pub budgets: Option<Budgets>,
    /// Every character of the campaign's party, by the ID of its copy's note, with its level
    /// from that saved sheet now. `None` is a level not known: a copy that cannot be read, or
    /// a sheet with no level. A member is never left out because its sheet cannot be read.
    pub party: BTreeMap<uuid::Uuid, Option<u64>>,
}

/// The four words, in rising order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Word {
    Low,
    Moderate,
    High,
    AboveHigh,
}

impl Word {
    pub fn text(self) -> &'static str {
        match self {
            Self::Low => "Low",
            Self::Moderate => "Moderate",
            Self::High => "High",
            Self::AboveHigh => "Above High",
        }
    }
}

/// A fight's XP and, when every input is known, its word.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Difficulty {
    /// The sum of the XP that is known.
    pub xp: u64,
    /// The word and the budgets. `None` when a creature's XP, a level or the table is not
    /// known, or when nothing stands against the party.
    pub rating: Option<Rating>,
    /// How many foes have no XP that is known. Their XP is in no sum; the rewards' XP line
    /// names them (F7).
    pub unknown: u64,
    /// Which input takes the word away, when it is one that is not known: the creatures with
    /// no XP and no CR, or else the characters with no level. `None` when the word shows, or
    /// when nothing the DM could fill in would bring it (no party, no table, no creature).
    pub missing: Option<Missing>,
}

/// The inputs that are not known, so that an opened row can say why it has no word (W1-D2).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Missing {
    pub what: Unknown,
    /// Each one by its place in what `Scale::weigh` was given: among the foes, or in the party
    /// (with no character in the roster, the campaign's party in `Scale::party`'s order).
    pub at: Vec<usize>,
    /// Their names, each once, from the caller that knows them (`Difficulty::named`).
    pub names: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Unknown {
    Xp,
    Level,
}

impl Missing {
    /// `XP not known: Nameless Thing`, or with more names, the first and `and 2 more`. `None`
    /// before the names are known.
    pub fn words(&self) -> Option<String> {
        let first = self.names.first()?;
        Some(match self.names.len() - 1 {
            0 => format!("{}: {first}", self.short()),
            more => format!("{}: {first} and {more} more", self.short()),
        })
    }

    /// The longest form of `words` that `fits`: whole, else the first name shortened from its
    /// end with `…` (the `and 2 more` stays), else `short` where no letter of the name fits
    /// (app round 2, X1). `None` before the names are known.
    pub fn fitted(&self, fits: impl Fn(&str) -> bool) -> Option<String> {
        let whole = self.words()?;
        if fits(&whole) {
            return Some(whole);
        }
        let first = self.names.first()?;
        let more = match self.names.len() - 1 {
            0 => String::new(),
            more => format!(" and {more} more"),
        };
        // The closed recipe, longest first: the first name kept to `kept` letters, then `…`
        // and `and 2 more` whole. The shared selector takes the first that fits.
        let letters: Vec<char> = first.chars().collect();
        let words = |kept: usize| {
            let name: String = letters[..kept].iter().collect();
            format!("{}: {}…{more}", self.short(), name.trim_end())
        };
        crate::design::fitting::first_fit(
            (1..letters.len()).rev(),
            |_| true,
            |kept| Some(words(kept)).filter(|words| fits(words)),
        )
        .map(|(_, words)| words)
        .or_else(|| Some(self.short().to_owned()))
    }

    /// Ruling R31, for every line that shows a roster with this reason, decided by the
    /// MEMBERS, never by comparing names: each member of the roster with whether the line
    /// still shows it. Where every member shown lacks its XP, the line already says whose:
    /// the reason names only the members without XP that it does not show (each name once,
    /// for display). Where a member with a known XP is shown, or none is shown, every name
    /// stays: saying whose input is missing is worth the repeated name.
    pub fn unsaid<'a>(&self, members: impl IntoIterator<Item = (&'a Member, bool)>) -> Self {
        let members: Vec<(&Member, bool)> = members.into_iter().collect();
        let mut shown = members.iter().filter(|(_, shown)| *shown).peekable();
        let says_whose = self.what == Unknown::Xp
            && shown.peek().is_some()
            && shown.all(|(member, _)| member.no_xp);
        if !says_whose {
            return self.clone();
        }
        let mut names: Vec<String> = Vec::new();
        for (member, shown) in &members {
            if member.no_xp && !shown && !names.contains(&member.called) {
                names.push(member.called.clone());
            }
        }
        Self {
            names,
            ..self.clone()
        }
    }

    /// The reason with no names, where a line has no room for them.
    pub fn short(&self) -> &'static str {
        match self.what {
            Unknown::Xp => "XP not known",
            Unknown::Level => "Level not known",
        }
    }
}

impl Difficulty {
    /// The form (`Rating::parts`), or `XP not known` in the same place when a word is missing
    /// because an input is not known (FB4). `None` when nothing could bring a word (no party,
    /// no table, no foe).
    pub fn parts(&self) -> Option<Vec<Part>> {
        match (&self.rating, &self.missing) {
            (Some(rating), _) => Some(rating.parts(true)),
            (None, Some(missing)) => Some(vec![(missing.short().to_owned(), false)]),
            (None, None) => None,
        }
    }

    /// The form as plain words.
    pub fn form(&self) -> Option<String> {
        Some(self.parts()?.into_iter().map(|(words, _)| words).collect())
    }

    /// The names of the inputs that are not known, from their places. A name that is not
    /// found is left out, and each name is said once.
    pub fn named(mut self, name: impl Fn(Unknown, usize) -> Option<String>) -> Self {
        if let Some(missing) = &mut self.missing {
            for at in &missing.at {
                if let Some(name) = name(missing.what, *at)
                    && !missing.names.contains(&name)
                {
                    missing.names.push(name);
                }
            }
        }
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Rating {
    pub word: Word,
    /// The party's budget for Low, Moderate and High.
    pub budgets: [u64; 3],
    pub xp: u64,
    /// How many characters, at which lowest and highest level.
    pub characters: usize,
    pub levels: (u64, u64),
    /// The roster has no character, so the campaign's party stands in.
    pub campaign_party: bool,
}

impl Scale {
    /// A run participant's level, when it is a character: only from the saved sheet now, by the
    /// ID of its copy's note. A run keeps a copy of each sheet from when it started; that copy
    /// says who is a character, never their level. A sheet that is gone or cannot be read is a
    /// level not known.
    pub fn level(&self, character: &Entry) -> Option<Option<u64>> {
        character_level(character)?;
        Some(self.party.get(&character.id).copied().flatten())
    }

    /// The one computation. `party` is the level of each character on the party's side; with
    /// none, the campaign's party stands in. `foes` is the XP of each creature that is not on
    /// the party's side, `None` where it is not known.
    pub fn weigh(
        &self,
        party: &[Option<u64>],
        foes: impl IntoIterator<Item = Option<u64>>,
    ) -> Difficulty {
        let (mut xp, mut unknown, mut any) = (0_u64, Vec::new(), false);
        for (at, foe) in foes.into_iter().enumerate() {
            any = true;
            match foe {
                Some(each) => xp = xp.saturating_add(each),
                None => unknown.push(at),
            }
        }
        let unknown_count = unknown.len() as u64;
        let campaign_party = party.is_empty();
        let saved: Vec<_>;
        let party = if campaign_party {
            saved = self.party.values().copied().collect();
            &saved
        } else {
            party
        };
        let rating = (any && unknown.is_empty())
            .then(|| self.rate(party, xp, campaign_party))
            .flatten();
        // Only an input that the DM could fill in to bring the word: with a party and a table.
        let levels = || {
            (0..party.len())
                .filter(|at| party[*at].is_none())
                .collect::<Vec<_>>()
        };
        let missing = (any && rating.is_none() && !party.is_empty() && self.budgets.is_some())
            .then(|| match unknown.is_empty() {
                false => (Unknown::Xp, unknown),
                true => (Unknown::Level, levels()),
            })
            .filter(|(_, at)| !at.is_empty())
            .map(|(what, at)| Missing {
                what,
                at,
                names: Vec::new(),
            });
        Difficulty {
            xp,
            rating,
            unknown: unknown_count,
            missing,
        }
    }

    /// The word for a fight whose XP is all known. `None` when the party is empty, a level is
    /// not known or not in the table, or there is no table.
    fn rate(&self, party: &[Option<u64>], xp: u64, campaign_party: bool) -> Option<Rating> {
        if party.is_empty() {
            return None;
        }
        let table = &self.budgets.as_ref()?.0;
        let mut budgets = [0_u64; 3];
        let (mut low, mut high) = (u64::MAX, 0);
        for level in party {
            let level = (*level)?;
            for (sum, each) in budgets.iter_mut().zip(table.get(&level)?) {
                *sum = sum.saturating_add(*each);
            }
            (low, high) = (low.min(level), high.max(level));
        }
        let word = match budgets.iter().position(|budget| xp <= *budget) {
            Some(0) => Word::Low,
            Some(1) => Word::Moderate,
            Some(_) => Word::High,
            None => Word::AboveHigh,
        };
        Some(Rating {
            word,
            budgets,
            xp,
            characters: party.len(),
            levels: (low, high),
            campaign_party,
        })
    }
}

/// One part of the difficulty form, and whether it is the word (drawn in the text colour; the
/// other parts are muted).
pub type Part = (String, bool);

/// T5.7, FB6, FH7: the one form of a fight's difficulty, on every screen that shows it:
/// `XP 400 · Low (up to 2,800)`. The number in brackets is the word's limit; Above High has
/// none. `limit: false` leaves it out, where the line names what the XP is for (Arrivals'
/// `Fight after: XP 450 · Low`).
impl Rating {
    pub fn parts(&self, limit: bool) -> Vec<Part> {
        let xp = crate::encounter_run::grouped_number(self.xp);
        let mut parts = vec![
            (format!("XP {xp} · "), false),
            (self.word.text().to_owned(), true),
        ];
        if limit && self.word != Word::AboveHigh {
            parts.push((
                format!(
                    " (up to {})",
                    crate::encounter_run::grouped_number(self.budgets[self.word as usize])
                ),
                false,
            ));
        }
        parts
    }

    /// The form as plain words.
    pub fn form(&self, limit: bool) -> String {
        self.parts(limit)
            .into_iter()
            .map(|(words, _)| words)
            .collect()
    }

    /// The hover and accessible description of the word:
    /// `Budget for 5 at level 3: Low 750 · Moderate 1,125 · High 2,000`.
    pub fn budget_line(&self) -> String {
        let who = if self.campaign_party {
            format!("the party of {}", self.characters)
        } else {
            self.characters.to_string()
        };
        let levels = match self.levels {
            (low, high) if low == high => format!("level {low}"),
            (low, high) => format!("levels {low} to {high}"),
        };
        let [low, moderate, high] = self.budgets.map(crate::encounter_run::grouped_number);
        format!("Budget for {who} at {levels}: Low {low} · Moderate {moderate} · High {high}")
    }
}

/// N7 (rulings R1, Q-s; T5.7): the one difficulty line of a fight, for the encounter editor, the
/// plan, Ready to run and Arrivals. A caller gives TYPED parts ([`Line::roster`] with a
/// [`Summary`], or [`Line::fight_after`]) and a measure. The order of the words and the order of
/// giving way are this type's, never the caller's. A name is an opaque value: the fit never
/// splits or matches built text, so `Ghost, of the Mill` is one member however it is spelled.
///
/// The words: a group's side, then its member count, then the form (`Enemies · 3 members ·
/// Fight after: XP 450 · Low`); a roster's summary (`5 party · 2 × Cinder Wolf`), then the form.
/// A fight with no word says `XP not known` in the word's place, and names whose input it is
/// only where the line does not already show that member (`1 × Nameless Thing · XP not known`).
///
/// What gives way, first to last:
///
/// - a roster: the party part (`5 party`, the same on every row), then the word's limit, then
///   the form's XP number, then the summary from its end, by whole members and facts, ending
///   with `…` where it was cut (Q-s: the creatures stay longest);
/// - Arrivals: the word's limit, then the member count, then the side, then the XP number
///   (`Fight after: Low` keeps its label and its word; R1).
///
/// The difficulty word and `Fight after:` never give way. A fight with no word gives way as
/// above, then shortens the first name from its end; the names go only where not one of their
/// letters fits. Nothing is cut inside a word, and the answer always fits the measure: where
/// even the shortest form does not, it is cut at whole words with `…`.
pub struct Line<'a> {
    difficulty: &'a Difficulty,
    units: Vec<Unit<'a>>,
    label: Option<&'static str>,
}

/// One creature of a roster's summary: how many, its words as the roster shows them, and the
/// name it is CALLED by (the name a missing-XP reason uses for it). Both come from one
/// constructor, so they cannot drift: a renamed creature shows `Shade (as Nameless Thing)` and
/// is called `Shade`. The names are opaque values; they may hold any characters, commas too.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Member {
    count: usize,
    name: String,
    called: String,
    /// Its XP is not known (R31 asks this of the member, never of a name).
    no_xp: bool,
}

impl Member {
    /// A creature shown and called by one name. `no_xp` is required: its builder says once,
    /// from the source the difficulty weighs (`creature_xp` of its entry), whether its XP is
    /// not known.
    pub fn new(count: usize, name: impl Into<String>, no_xp: bool) -> Self {
        let name = name.into();
        Self {
            count,
            called: name.clone(),
            name,
            no_xp,
        }
    }

    /// A renamed creature (FL2): shown as `Shade (as Nameless Thing)`, called `Shade`.
    pub fn renamed(count: usize, shown: &str, base: &str, no_xp: bool) -> Self {
        Self {
            count,
            name: format!("{shown} (as {base})"),
            called: shown.to_owned(),
            no_xp,
        }
    }

    /// The member as the line shows it: `1,000 × Wolf` (counts grouped, `design::count`).
    pub fn words(&self) -> String {
        format!(
            "{} × {}",
            crate::design::count(self.count, "", ""),
            self.name
        )
    }
}

/// A roster's fact after its members. Typed, so that no caller can put roster words here
/// where the line could not see the members in them.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Fact {
    /// A creature of the roster is changed from its entry.
    Changed,
    /// Built words, for this module's tests of opaque text (`Summary::from_built`).
    #[cfg(any(test, feature = "test-support"))]
    Built(String),
}

impl Fact {
    pub fn words(&self) -> &str {
        match self {
            Self::Changed => "Changed",
            #[cfg(any(test, feature = "test-support"))]
            Self::Built(words) => words,
        }
    }
}

/// A roster's summary as typed parts, in the line's order: the party count, the members, then
/// the roster's other facts (`Changed`).
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Summary {
    pub party: usize,
    pub members: Vec<Member>,
    pub facts: Vec<Fact>,
}

impl Summary {
    /// The whole summary as words, in the line's order, with nothing given way:
    /// `5 party · 2 × Cinder Wolf, 1 × Ember Cultist · Changed`. For a place that shows the
    /// summary as plain words (a row's hover, the plan); a difficulty line takes the parts.
    pub fn text(&self) -> String {
        let mut parts = Vec::new();
        if self.party > 0 {
            parts.push(party_words(self.party));
        }
        if !self.members.is_empty() {
            let members: Vec<String> = self.members.iter().map(Member::words).collect();
            parts.push(members.join(", "));
        }
        parts.extend(self.facts.iter().map(|fact| fact.words().to_owned()));
        parts.join(" · ")
    }

    /// Built words, kept for this module's tests of opaque text. No caller hands built words
    /// to a line: `Line::roster` takes a `Summary`, and `&str` is not one (the Ready row hands
    /// typed parts since G118). Only a leading party count (`5 party`) is read; everything
    /// after it is ONE opaque fact: never parsed into members, never narrowed member by member.
    /// It is shown as it came; after the XP number (Q-s) it shortens from its end where a
    /// separator stands (`, `, ` · `) with `…`, as the line did before typed parts, and goes last. A name such as
    /// `Ghost, 02 × Echo` keeps its words: never reformatted, only cut from the end.
    #[cfg(any(test, feature = "test-support"))]
    pub fn from_built(text: &str) -> Self {
        let (party, rest) = match text.split_once(" · ") {
            Some((first, rest)) => (first, rest),
            None => (text, ""),
        };
        let count = party
            .strip_suffix(" party")
            .filter(|count| !count.is_empty() && count.chars().all(|c| c.is_ascii_digit()))
            .and_then(|count| count.parse().ok());
        let (party, rest) = match count {
            Some(count) => (count, rest),
            None => (0, text),
        };
        Self {
            party,
            members: Vec::new(),
            facts: (!rest.is_empty())
                .then(|| Fact::Built(rest.to_owned()))
                .into_iter()
                .collect(),
        }
    }
}

impl From<&Summary> for Summary {
    fn from(summary: &Summary) -> Self {
        summary.clone()
    }
}

fn party_words(party: usize) -> String {
    crate::design::count(party, "party", "party")
}

pub use splat_content_ui::roster::FightAfter;

/// When a part gives way: its rank among every part of the line. A lower rank goes first.
const PARTY_GOES: usize = 0;
const LIMIT_GOES: usize = 1;
const MEMBERS_GO: usize = 2;
const SIDE_GOES: usize = 3;
const WORTH_GOES: usize = 4;
const NUMBER_GOES: usize = 10;
/// A roster's summary units rank after the XP number, from the summary's end.
const SUMMARY_GOES: usize = 20;

struct Unit<'a> {
    /// The separator before it on the line: ` · ` between facts, `, ` between members.
    before: &'static str,
    words: String,
    /// When it gives way: `None` never, else its rank.
    gives: Option<usize>,
    /// A unit of a summary: the summary ends with `…` where it was cut.
    summary: bool,
    /// The member this unit shows (R31 asks it whether its XP is known).
    member: Option<Member>,
    /// Built words (a fact, not a member): before it goes, it shortens from its end.
    opaque: bool,
    _line: std::marker::PhantomData<&'a ()>,
}

/// One stage of giving way: the parts gone, and an opaque fact shortened from its end.
#[derive(Clone, Debug, Default)]
struct Stage {
    gone: Vec<Give>,
    cut: Option<(usize, String)>,
}

/// One step of giving way.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Give {
    Unit(usize),
    Limit,
    Number,
}

/// What [`Line::fit`] keeps: the words before the form (with `…` where the summary was cut) and
/// the form's parts, the label first.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fitted {
    pub lead: String,
    pub form: Vec<Part>,
    /// Something gave way: hover and the accessible name keep the whole line.
    pub cut: bool,
    /// The protected answer is wider than its room: the caller wraps it at word boundaries
    /// and never cuts it (`Line::fit_or_wrap`).
    pub wrap: bool,
}

impl Fitted {
    /// The line as plain words.
    pub fn text(&self) -> String {
        let form: String = self.form.iter().map(|(words, _)| words.as_str()).collect();
        match (self.lead.is_empty(), form.is_empty()) {
            (true, _) => form,
            (false, true) => self.lead.clone(),
            (false, false) => format!("{} · {form}", self.lead),
        }
    }
}

/// The longest start of `text`, cut at a whole word, that fits with `…` after it; `…` alone
/// when not one word does.
fn cut_at_words(text: &str, fits: &impl Fn(&str) -> bool) -> String {
    let words: Vec<&str> = text.split(' ').collect();
    // The closed recipe, longest first: the first `kept` words with no separator left at
    // their end, then `…`. The shared selector takes the first that fits.
    let cut = |kept: usize| {
        let kept = words[..kept]
            .join(" ")
            .trim_end_matches(['·', ',', ' ', '…'])
            .to_owned();
        (!kept.is_empty()).then(|| format!("{kept}…"))
    };
    crate::design::fitting::first_fit(
        (1..words.len()).rev(),
        |_| true,
        |kept| cut(kept).filter(|cut| fits(cut)),
    )
    .map_or_else(|| "…".to_owned(), |(_, cut)| cut)
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// Test builds: how many lines were built and how many candidate forms were measured, so
    /// a test can see that an unchanged frame does neither (point 3 of the round 29 review).
    pub static WORK: std::cell::Cell<(usize, usize)> = const { std::cell::Cell::new((0, 0)) };
}

fn count_work(built: usize, measured: usize) {
    #[cfg(any(test, feature = "test-support"))]
    WORK.with(|work| {
        let (b, m) = work.get();
        work.set((b + built, m + measured));
    });
    let _ = (built, measured);
}

impl<'a> Line<'a> {
    /// A roster's line: its summary's typed parts (which may be empty: the editor's heading says
    /// the rest), then the form.
    pub fn roster(difficulty: &'a Difficulty, summary: impl Into<Summary>) -> Self {
        count_work(1, 0);
        let summary = summary.into();
        let mut units = Vec::new();
        if summary.party > 0 {
            units.push(Unit {
                before: " · ",
                words: party_words(summary.party),
                gives: Some(PARTY_GOES),
                summary: false,
                member: None,
                opaque: false,
                _line: std::marker::PhantomData,
            });
        }
        for (at, member) in summary.members.iter().enumerate() {
            units.push(Unit {
                before: if at == 0 { " · " } else { ", " },
                words: member.words(),
                gives: None,
                summary: true,
                member: Some(member.clone()),
                opaque: false,
                _line: std::marker::PhantomData,
            });
        }
        for fact in &summary.facts {
            units.push(Unit {
                before: " · ",
                words: fact.words().to_owned(),
                gives: None,
                summary: true,
                member: None,
                opaque: true,
                _line: std::marker::PhantomData,
            });
        }
        // From its end: a later unit gives way first.
        let count = units.len();
        for (at, unit) in units.iter_mut().enumerate() {
            if unit.summary {
                unit.gives = Some(SUMMARY_GOES + count - at);
            }
        }
        Self {
            difficulty,
            units,
            label: None,
        }
    }

    /// Arrivals' line (R1): `Enemies · 3 members · Fight after: XP 450 · Low`, after the
    /// group's name. The label is part of this form; a caller cannot leave it out. A fight with
    /// no form (no party, no table) keeps the group's own facts: `Enemies · 3 members · XP 150`.
    pub fn fight_after(difficulty: &'a Difficulty, parts: FightAfter<'a>) -> Self {
        count_work(1, 0);
        let fact = |words: &str, gives| Unit {
            before: " · ",
            words: words.to_owned(),
            gives,
            summary: false,
            member: None,
            opaque: false,
            _line: std::marker::PhantomData,
        };
        let worth = if difficulty.parts().is_some() {
            ""
        } else {
            parts.worth
        };
        Self {
            difficulty,
            units: vec![
                fact(parts.side, Some(SIDE_GOES)),
                fact(parts.members, Some(MEMBERS_GO)),
                fact(worth, Some(WORTH_GOES)),
            ]
            .into_iter()
            .filter(|unit| !unit.words.is_empty())
            .collect(),
            label: Some("Fight after: "),
        }
    }

    /// Every stage of giving way, in rank order. A part goes whole, except an opaque fact
    /// (built words, `Summary::from_built`): before it goes, it shortens from its end where a
    /// separator stands, with `…`, as the line did before typed parts (third review, point 2).
    fn stages(&self) -> Vec<Stage> {
        let rated = self.difficulty.rating.is_some();
        let mut ranked: Vec<(usize, Give)> = self
            .units
            .iter()
            .enumerate()
            .filter_map(|(at, unit)| Some((unit.gives?, Give::Unit(at))))
            .collect();
        if rated {
            ranked.push((LIMIT_GOES, Give::Limit));
            ranked.push((NUMBER_GOES, Give::Number));
        }
        ranked.sort_by_key(|(rank, _)| *rank);
        let order: Vec<Give> = ranked.into_iter().map(|(_, give)| give).collect();
        let mut stages = Vec::new();
        for count in 0..=order.len() {
            let gone = order[..count].to_vec();
            stages.push(Stage {
                gone: gone.clone(),
                cut: None,
            });
            if let Some(Give::Unit(at)) = order.get(count)
                && self.units[*at].opaque
            {
                // Cut only where the words have a separator (`, ` or ` · `), the places the line
                // cut them before typed parts: nothing is read as a member, nothing is cut inside
                // a name's words, and the separator stays before `…` (`1 × Goblin, …`).
                let words = &self.units[*at].words;
                let mut ends: Vec<(usize, &str)> = words
                    .match_indices(", ")
                    .chain(words.match_indices(" · "))
                    .collect();
                ends.sort_by_key(|(end, _)| std::cmp::Reverse(*end));
                for (end, separator) in ends {
                    stages.push(Stage {
                        gone: gone.clone(),
                        cut: Some((*at, format!("{}{} …", &words[..end], separator.trim_end()))),
                    });
                }
            }
        }
        stages
    }

    /// The words before the form, at this stage.
    fn lead(&self, stage: &Stage) -> String {
        let mut text = String::new();
        let mut cut = false;
        for (at, unit) in self.units.iter().enumerate() {
            if stage.gone.contains(&Give::Unit(at)) {
                if unit.summary && !cut {
                    cut = true;
                    if !text.is_empty() {
                        text.push_str(unit.before.trim_end());
                        text.push(' ');
                    }
                    text.push('…');
                }
                continue;
            }
            if !text.is_empty() {
                text.push_str(unit.before);
            }
            match &stage.cut {
                Some((shortened, words)) if *shortened == at => {
                    cut = true;
                    text.push_str(words);
                }
                _ => text.push_str(&unit.words),
            }
        }
        text
    }

    /// Each member of the line with whether it is still on the line at `stage` (R31).
    fn members<'s>(&'s self, stage: &'s Stage) -> impl Iterator<Item = (&'s Member, bool)> + 's {
        self.units.iter().enumerate().filter_map(move |(at, unit)| {
            let member = unit.member.as_ref()?;
            Some((member, !stage.gone.contains(&Give::Unit(at))))
        })
    }

    fn with_label(&self, parts: Vec<Part>) -> Vec<Part> {
        self.label
            .map(|label| (label.to_owned(), false))
            .into_iter()
            .chain(parts)
            .collect()
    }

    /// The smallest COMPLETE line, every part that can give way gone: `Fight after: Low`, or
    /// `Fight after: XP not known` (point 2: the room a heading reserves before its title).
    /// `None` as `fit`.
    pub fn shortest(&self) -> Option<Fitted> {
        let last = self.stages().pop()?;
        let lead = self.lead(&last);
        let form = match (&self.difficulty.rating, &self.difficulty.missing) {
            (Some(rating), _) => self.with_label(vec![(rating.word.text().to_owned(), true)]),
            (None, Some(missing)) => self.with_label(vec![(missing.short().to_owned(), false)]),
            (None, None) if self.label.is_some() => Vec::new(),
            (None, None) => return None,
        };
        Some(Fitted {
            lead,
            form,
            cut: !last.gone.is_empty(),
            wrap: false,
        })
    }

    /// Point 3 of the third review (R1 at its boundary): the protected answer is never cut
    /// with `…`. Where even [`Line::shortest`] does not fit, that answer is given whole with
    /// `wrap`: the caller lays it out at its room, wrapping at word boundaries. Else as `fit`.
    pub fn fit_or_wrap(&self, fits: impl Fn(&str) -> bool) -> Option<Fitted> {
        if let Some(shortest) = self.shortest()
            && self.label.is_some()
            && !fits(&shortest.text())
        {
            count_work(0, 1);
            return Some(Fitted {
                wrap: true,
                cut: true,
                ..shortest
            });
        }
        self.fit(fits)
    }

    /// The longest line that `fits` (one line, plain words), in the order this type states.
    /// `None` when the fight has no form (no party, no table, no foe) and the line has no facts
    /// of its own to say instead (a roster).
    pub fn fit(&self, fits: impl Fn(&str) -> bool) -> Option<Fitted> {
        let fits = |text: &str| {
            count_work(0, 1);
            fits(text)
        };
        let stages = self.stages();
        let last = stages.last().cloned().unwrap_or_default();
        let line = |stage: &Stage, form: Vec<Part>, cut: bool| Fitted {
            lead: self.lead(stage),
            form,
            cut: cut || !stage.gone.is_empty() || stage.cut.is_some(),
            wrap: false,
        };
        let fallback = |fitted: Fitted| {
            if fits(&fitted.text()) {
                return fitted;
            }
            Fitted {
                lead: String::new(),
                form: vec![(cut_at_words(&fitted.text(), &fits), false)],
                cut: true,
                wrap: false,
            }
        };
        match (&self.difficulty.rating, &self.difficulty.missing) {
            (Some(rating), _) => {
                let form = |stage: &Stage| {
                    self.with_label(if stage.gone.contains(&Give::Number) {
                        vec![(rating.word.text().to_owned(), true)]
                    } else {
                        rating.parts(!stage.gone.contains(&Give::Limit))
                    })
                };
                for stage in &stages {
                    let fitted = line(stage, form(stage), false);
                    if fits(&fitted.text()) {
                        return Some(fitted);
                    }
                }
                Some(fallback(line(&last, form(&last), true)))
            }
            (None, Some(missing)) => {
                // R31 (AB4, FB5, SB5): where every creature still shown lacks its XP, they are
                // not named again; beside a known one, the reason keeps the names.
                let unsaid = |stage: &Stage| missing.unsaid(self.members(stage));
                let reason = |words: String| self.with_label(vec![(words, false)]);
                let whole = |stage: &Stage| {
                    unsaid(stage)
                        .words()
                        .unwrap_or_else(|| missing.short().to_owned())
                };
                // The names whole (the first and a count), the facts giving way first.
                for stage in &stages {
                    let fitted = line(stage, reason(whole(stage)), false);
                    if fits(&fitted.text()) {
                        return Some(fitted);
                    }
                }
                // Then the first name shortened from its end, with the fewest facts.
                let compose = |words: &str| line(&last, reason(words.to_owned()), false).text();
                let words = unsaid(&last)
                    .fitted(|words| fits(&compose(words)))
                    .unwrap_or_else(|| missing.short().to_owned());
                let cut = words != whole(&last);
                Some(fallback(line(&last, reason(words), cut)))
            }
            (None, None) if self.label.is_some() => {
                // No form: the facts alone, in the same order of giving way.
                for stage in &stages {
                    let fitted = line(stage, Vec::new(), false);
                    if fits(&fitted.text()) {
                        return Some(fitted);
                    }
                }
                Some(fallback(line(&last, Vec::new(), true)))
            }
            (None, None) => None,
        }
    }
}

/// Point 3 and point 1 of the round 29 reviews: the fitted lines of one owner, kept between
/// frames. The OWNER holds it (a loaded run keeps one for its Arrivals), so it dies with that
/// owner and no other run can read its answers. Each line is kept under its own id with one
/// key (ruling M6): the owner's revision `C` of what the line says, and the [`Measure`] it was
/// fitted in (its room, its font, the theme's font generation and the display scale). On a
/// frame where the key is unchanged, the line is not built and nothing is measured; a new key
/// replaces the entry. The owner ends an entry with [`Kept::retain`] when it no longer shows
/// its line, or ends them all by dropping the `Kept`.
///
/// The kept answer is a fitted line (`V`'s default), or an owner's own answer for a line it
/// lays out itself (the session plan's row keeps its laid-out line).
pub struct Kept<K, V: Sync = Option<Fitted>, C: Key = Revision> {
    fits: Memos<K, (C, Measure), V>,
    shortest: Memos<K, (C, Measure), f32>,
    widest: Memos<K, (C, Measure), Option<f32>>,
}

impl<K, V: Sync, C: Key> Default for Kept<K, V, C> {
    fn default() -> Self {
        Self {
            fits: Memos::default(),
            shortest: Memos::default(),
            widest: Memos::default(),
        }
    }
}

impl<K: std::hash::Hash + Eq + Clone, C: Key> Kept<K, Option<Fitted>, C> {
    #[cfg(any(test, feature = "test-support"))]
    pub fn seed(&mut self, id: K) {
        self.fits.seed(id.clone());
        self.shortest.seed(id);
    }
}

impl<K: std::hash::Hash + Eq + Clone, V: Clone + Sync, C: Key> Kept<K, V, C> {
    /// Test builds: whether a line is kept for `id`.
    #[cfg(any(test, feature = "test-support"))]
    pub fn holds(&self, id: &K) -> bool {
        self.fits.holds(id) || self.shortest.holds(id) || self.widest.holds(id)
    }

    /// Drop the lines whose owner no longer shows them (a group no longer offered).
    pub fn retain(&mut self, keep: impl Fn(&K) -> bool) {
        self.fits.retain(&keep);
        self.shortest.retain(&keep);
        self.widest.retain(&keep);
    }
}

/// The rule as the campaign's packages hold it, read through the same path the app uses.
#[cfg(any(test, feature = "test-support"))]
pub fn builtin_budgets() -> Budgets {
    let text = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../builtin/dnd2024-definitions/rules/combat_encounters_step_2_determine_your_xp_budget.md"
    ))
    .unwrap();
    let entry = Entry::parse("rule.md", &text).unwrap();
    Budgets::of_rule(&entry.fields["splat_rule"]).expect("the table in the package reads")
}

#[cfg(any(test, feature = "test-support"))]
pub fn party_of(levels: &[u64]) -> BTreeMap<uuid::Uuid, Option<u64>> {
    levels
        .iter()
        .map(|level| (uuid::Uuid::new_v4(), Some(*level)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// N7, R1, FG1: what gives way on a difficulty line, and in which order. A plain width per
    /// character stands in for the font.
    #[test]
    fn a_difficulty_line_gives_way_in_one_order() {
        let fits = |room: usize| move |text: &str| text.chars().count() <= room;
        let low = Difficulty {
            xp: 400,
            rating: Some(Rating {
                word: Word::Low,
                budgets: [2800, 4200, 6400],
                xp: 400,
                characters: 5,
                levels: (3, 3),
                campaign_party: false,
            }),
            ..Difficulty::default()
        };
        // Typed parts (point 1): the party count and each member with its count and name.
        let typed = |party: usize, members: &[(usize, &str)]| Summary {
            party,
            members: members
                .iter()
                // The fixture's creatures with no XP: Nameless Thing and Wisp.
                .map(|(count, name)| {
                    Member::new(*count, *name, ["Nameless Thing", "Wisp"].contains(name))
                })
                .collect(),
            facts: Vec::new(),
        };
        let summary = typed(5, &[(2, "Cinder Wolf"), (1, "Ember Cultist")]);
        let ready = Line::roster(&low, &summary);
        let at = |room| ready.fit(fits(room)).unwrap().text();
        // Room for all of it.
        let whole = "5 party · 2 × Cinder Wolf, 1 × Ember Cultist · XP 400 · Low (up to 2,800)";
        assert_eq!(at(200), whole);
        assert!(!ready.fit(fits(200)).unwrap().cut);
        // Q-s (FB4): `5 party`, the same on every row, goes first; then the limit (FG1); then
        // the XP number. The creatures stay longest: the row answers "what can I run?".
        let steps = [
            "2 × Cinder Wolf, 1 × Ember Cultist · XP 400 · Low (up to 2,800)",
            "2 × Cinder Wolf, 1 × Ember Cultist · XP 400 · Low",
            "2 × Cinder Wolf, 1 × Ember Cultist · Low",
        ];
        assert_eq!(at(whole.chars().count() - 1), steps[0]);
        for (step, next) in steps.iter().zip(&steps[1..]) {
            assert_eq!(at(step.chars().count() - 1), *next);
        }
        // Then the summary from its end, by whole members and facts, with `…` (never `2…`).
        assert_eq!(at(39), "2 × Cinder Wolf, … · Low");
        assert_eq!(at(20), "… · Low");
        // Review of PR 327 (c): the answer always fits; where the shortest form does not, it is
        // cut at whole words with `…`. A 40 px width (here 4 characters) stays inside it.
        for room in [4, 1] {
            let text = at(room);
            assert!(text.chars().count() <= room, "{room}: {text:?}");
        }
        // Only a first fact that counts the party is the party fact.
        let named_party = Line::roster(
            &low,
            Summary::from_built("Party Crashers · 2 × Cinder Wolf"),
        );
        assert_eq!(
            named_party.fit(fits(52)).unwrap().text(),
            "Party Crashers · 2 × Cinder Wolf · XP 400 · Low"
        );
        // R1, the contract's counterexample: Arrivals' `Fight after` keeps its label and word;
        // the member count gives way, then the side, then the XP number. The name stays. The
        // order and the label are the type's: `fight_after` takes typed parts and no rank.
        // FJ2, FC7, FK8, FN3: the facts read side, then count, as the Encounters list and the
        // plan say them (`FightAfter::facts`), in every form of the heading.
        let patrol = || FightAfter {
            side: "Enemies",
            members: "3 members",
            worth: "XP 150",
        };
        assert_eq!(patrol().facts(), "Enemies · 3 members · XP 150");
        let arrivals = Line::fight_after(&low, patrol());
        let at = |room| arrivals.fit(fits(room)).unwrap().text();
        assert_eq!(
            at(200),
            "Enemies · 3 members · Fight after: XP 400 · Low (up to 2,800)"
        );
        assert_eq!(at(60), "Enemies · 3 members · Fight after: XP 400 · Low");
        assert_eq!(at(46), "Enemies · Fight after: XP 400 · Low");
        assert_eq!(at(34), "Fight after: XP 400 · Low");
        assert_eq!(at(24), "Fight after: Low");
        // SB1 (iter-29 G57): no form of the line holds the group's name, also where even the
        // shortest form is cut: the heading draws the name once.
        for room in [200, 60, 46, 34, 24, 10, 3] {
            assert!(!at(room).contains("Patrol"), "{room}: {}", at(room));
        }
        // A fight with no form has no line on a roster.
        assert!(
            Line::roster(&Difficulty::default(), &summary)
                .fit(fits(200))
                .is_none()
        );
        // In Arrivals, a fight with no form (no party, no table) keeps the group's own facts,
        // in the same order; with a form, the group's own XP never shows beside the fight's.
        let none = Difficulty::default();
        let facts = Line::fight_after(&none, patrol());
        assert_eq!(
            facts.fit(fits(200)).unwrap().text(),
            "Enemies · 3 members · XP 150"
        );
        assert_eq!(facts.fit(fits(20)).unwrap().text(), "Enemies · XP 150");
        assert!(!at(200).contains("XP 150"));
        // Review of PR 327 (b): a mixed roster's reason keeps whose input is missing: whole
        // where it fits, the facts going first, then the first name shortened; never `XP not
        // known` alone while a letter of the name fits.
        let unknown = Difficulty {
            missing: Some(Missing {
                what: Unknown::Xp,
                at: vec![2],
                names: vec!["Nameless Thing".into(), "Wisp".into()],
            }),
            ..Difficulty::default()
        };
        let mixed = Line::roster(&unknown, &summary);
        let at = |room| mixed.fit(fits(room)).unwrap().text();
        assert_eq!(
            at(200),
            "5 party · 2 × Cinder Wolf, 1 × Ember Cultist · XP not known: Nameless Thing and 1 more"
        );
        assert_eq!(
            at(80),
            "2 × Cinder Wolf, 1 × Ember Cultist · XP not known: Nameless Thing and 1 more"
        );
        assert_eq!(
            at(64),
            "2 × Cinder Wolf, … · XP not known: Nameless Thing and 1 more"
        );
        assert_eq!(at(45), "… · XP not known: Nameless Thing and 1 more");
        assert_eq!(at(32), "… · XP not known: Na… and 1 more");
        // AB4, FB5, SB5: a name that the summary shows is not said again. Where the summary is
        // cut so that it no longer shows the name, the reason names it.
        let lone = Difficulty {
            missing: Some(Missing {
                what: Unknown::Xp,
                at: vec![0],
                names: vec!["Nameless Thing".into()],
            }),
            ..Difficulty::default()
        };
        let visitor = Line::roster(&lone, typed(5, &[(1, "Nameless Thing")]));
        let at = |room| visitor.fit(fits(room)).unwrap().text();
        assert_eq!(at(200), "5 party · 1 × Nameless Thing · XP not known");
        assert_eq!(at(36), "1 × Nameless Thing · XP not known");
        assert_eq!(at(32), "… · XP not known: Nameless Thing");
        assert!(!visitor.fit(fits(200)).unwrap().cut);
        // R31: beside a creature with a known XP (Ox), the reason keeps every name: it says
        // whose input is missing.
        let two = Line::roster(&unknown, typed(5, &[(1, "Nameless Thing"), (1, "Ox")]));
        assert_eq!(
            two.fit(fits(200)).unwrap().text(),
            "5 party · 1 × Nameless Thing, 1 × Ox · XP not known: Nameless Thing and 1 more"
        );
        // Every creature shown lacks its XP: the line says whose; the one it no longer shows
        // is named.
        let both = Line::roster(&unknown, typed(5, &[(1, "Nameless Thing"), (1, "Wisp")]));
        let at = |room| both.fit(fits(room)).unwrap().text();
        assert_eq!(
            at(200),
            "5 party · 1 × Nameless Thing, 1 × Wisp · XP not known"
        );
        assert_eq!(at(42), "1 × Nameless Thing, … · XP not known: Wisp");
        // FK3, SK3, FJ3 (R1): a joining group whose fight has no word still says `Fight after`,
        // with `XP not known` in the word's place and whose XP it is.
        let ghost = Difficulty {
            missing: Some(Missing {
                what: Unknown::Xp,
                at: vec![3],
                names: vec!["Ghost of the Mill".into()],
            }),
            ..Difficulty::default()
        };
        let joining = Line::fight_after(&ghost, patrol());
        assert_eq!(
            joining.fit(fits(200)).unwrap().text(),
            "Enemies · 3 members · Fight after: XP not known: Ghost of the Mill"
        );
        assert_eq!(
            joining.fit(fits(44)).unwrap().text(),
            "Fight after: XP not known: Ghost of the Mill"
        );
        // Point 1 (round 29 review): typed parts. A name is an opaque value: `Ghost, of the
        // Mill` is one member, said once, and narrowing never halves it.
        let comma = Summary {
            party: 5,
            members: vec![
                Member::new(2, "Cinder Wolf", false),
                Member::new(1, "Ghost, of the Mill", true),
            ],
            facts: Vec::new(),
        };
        let ghost = Difficulty {
            missing: Some(Missing {
                what: Unknown::Xp,
                at: vec![2],
                names: vec!["Ghost, of the Mill".into()],
            }),
            ..Difficulty::default()
        };
        let typed = Line::roster(&ghost, &comma);
        let at = |room| typed.fit(fits(room)).unwrap().text();
        // R31: Cinder Wolf's XP is known, so the reason names the ghost.
        assert_eq!(
            at(200),
            "5 party · 2 × Cinder Wolf, 1 × Ghost, of the Mill · XP not known: Ghost, of the Mill"
        );
        for room in (1..=80).rev() {
            let text = at(room);
            assert!(text.chars().count() <= room, "{room}: {text}");
            assert!(
                text.matches("Ghost").count() <= 1 + usize::from(text.contains("1 × Ghost")),
                "the name once in the reason at {room}: {text}"
            );
            assert!(
                !text.contains("1 × Ghost,…") && !text.contains("Ghost, …"),
                "never half a member at {room}: {text}"
            );
            if text.contains("1 × Ghost") {
                assert!(text.contains("1 × Ghost, of the Mill"), "{room}: {text}");
            }
        }
        assert_eq!(
            at(53),
            "2 × Cinder Wolf, … · XP not known: Ghost, of the Mill"
        );
        assert_eq!(at(40), "… · XP not known: Ghost, of the Mill");
        // Point 2 of the second review: BUILT words are opaque. `Ghost, 02 × Echo` is never read
        // as members: its words stay as they came at full width, and narrowing never halves it
        // (it gives way whole, after `5 party`, the limit and the XP number).
        let built = Line::roster(&low, Summary::from_built("5 party · 1 × Ghost, 02 × Echo"));
        let at = |room| built.fit(fits(room)).unwrap().text();
        assert_eq!(
            at(200),
            "5 party · 1 × Ghost, 02 × Echo · XP 400 · Low (up to 2,800)"
        );
        // Never reformatted: whatever of the built words shows is their own start, cut where a
        // separator stands, with `…` (point 2 of the third review), and `02` stays `02`.
        let words = "1 × Ghost, 02 × Echo";
        for room in (1..=70).rev() {
            let text = at(room);
            let shown = text
                .split(" · ")
                .find(|part| part.starts_with("1 ×"))
                .unwrap_or("");
            assert!(
                shown.is_empty()
                    || shown == words
                    || shown
                        .strip_suffix('…')
                        .is_some_and(|cut| words.starts_with(cut.trim_end())),
                "built words at {room}: {text}"
            );
            assert!(!text.contains(" 2 × Echo"), "{room}: {text}");
        }
        assert_eq!(at(40), "1 × Ghost, 02 × Echo · XP 400 · Low");
        assert_eq!(at(30), "1 × Ghost, 02 × Echo · Low");
        assert_eq!(at(20), "1 × Ghost, … · Low");
        // Point 2 of the third review: built words shorten FROM THEIR END; a useful start stays
        // (the leading creature), as before typed parts. They go whole only where it does not fit.
        let long = Line::roster(
            &low,
            Summary::from_built("5 party · 1 × Goblin, 1 × Ashen Warden of the Ninth Gate"),
        );
        let at = |room| long.fit(fits(room)).unwrap().text();
        assert_eq!(at(30), "1 × Goblin, … · Low");
        assert_eq!(at(7), "… · Low");
        // Point 3 of the third review: the protected answer is never cut with `…`. Where even
        // its smallest form is wider than the room, it is given whole, to wrap at words.
        let arrivals = Line::fight_after(&low, patrol());
        let narrow = arrivals.fit_or_wrap(fits(10)).unwrap();
        assert_eq!(
            (narrow.text().as_str(), narrow.wrap),
            ("Fight after: Low", true)
        );
        let room = arrivals.fit_or_wrap(fits(16)).unwrap();
        assert_eq!(
            (room.text().as_str(), room.wrap),
            ("Fight after: Low", false)
        );
        for width in 1..=80 {
            let fitted = arrivals.fit_or_wrap(fits(width)).unwrap();
            assert!(
                fitted.text().contains("Fight after: ") && !fitted.text().contains('…'),
                "{width}: {}",
                fitted.text()
            );
        }
        // Point 2: the smallest complete line, every part that can give way gone.
        assert_eq!(
            Line::fight_after(&low, patrol()).shortest().unwrap().text(),
            "Fight after: Low"
        );
        assert_eq!(
            Line::fight_after(&ghost, patrol())
                .shortest()
                .unwrap()
                .text(),
            "Fight after: XP not known"
        );
    }

    #[test]
    fn the_word_compares_the_xp_with_the_party_budget_from_the_rules() {
        let scale = Scale {
            budgets: Some(builtin_budgets()),
            party: party_of(&[3; 5]),
        };
        let weigh =
            |party: &[Option<u64>], foes: &[Option<u64>]| scale.weigh(party, foes.iter().copied());
        let five = [Some(3); 5];
        // The design's party: 5 characters at level 3. Each budget is the sum of the table's
        // value for each character, and each word holds up to its budget.
        let low = weigh(&five, &[Some(450), Some(100), Some(200)]);
        let rating = low.rating.as_ref().unwrap();
        assert_eq!(
            (rating.word, rating.budgets),
            (Word::Low, [750, 1125, 2000])
        );
        // FB6, FH7: one form, with the word's limit named.
        assert_eq!(rating.form(true), "XP 750 · Low (up to 750)");
        assert_eq!(rating.form(false), "XP 750 · Low");
        assert_eq!(low.form().as_deref(), Some("XP 750 · Low (up to 750)"));
        assert_eq!(
            rating.budget_line(),
            "Budget for 5 at level 3: Low 750 · Moderate 1,125 · High 2,000"
        );
        let word = |party: &[Option<u64>], foes: &[Option<u64>]| {
            weigh(party, foes).rating.map(|rating| rating.word)
        };
        assert_eq!(word(&five, &[Some(751)]), Some(Word::Moderate));
        assert_eq!(word(&five, &[Some(2000)]), Some(Word::High));
        let above = weigh(&five, &[Some(2400)]).rating.unwrap();
        assert_eq!(
            (above.word, above.form(true).as_str()),
            (Word::AboveHigh, "XP 2,400 · Above High")
        );
        // Mixed levels, and a roster with no character, say so in the budget line.
        let mixed = weigh(&[Some(3), Some(4)], &[Some(50)]).rating.unwrap();
        assert_eq!(mixed.budgets, [400, 600, 900]);
        assert!(
            mixed
                .budget_line()
                .starts_with("Budget for 2 at levels 3 to 4:")
        );
        let stand_in = weigh(&[], &[Some(50)]).rating.unwrap();
        assert!(
            stand_in
                .budget_line()
                .starts_with("Budget for the party of 5 at level 3:")
        );
    }

    #[test]
    fn nothing_shows_when_an_input_is_not_known() {
        let scale = Scale {
            budgets: Some(builtin_budgets()),
            party: BTreeMap::new(),
        };
        let five = [Some(3); 5];
        // A creature with no XP and no CR keeps its place and the XP that is known.
        let unknown = scale.weigh(&five, [Some(100), None, None]);
        let missing = |what, at: &[usize]| {
            Some(Missing {
                what,
                at: at.to_vec(),
                names: Vec::new(),
            })
        };
        // FB4: a fight with no word says so in the word's place.
        assert_eq!(unknown.form().as_deref(), Some("XP not known"));
        assert_eq!(
            (unknown.xp, unknown.rating, unknown.missing),
            (100, None, missing(Unknown::Xp, &[1, 2]))
        );
        // W1-D2: the opened row names whose input is not known, each name once.
        let named = scale
            .weigh(&five, [Some(100), None, None, None])
            .named(|_, at| Some(["Ox", "Nameless Thing", "Nameless Thing", "Wisp"][at].into()));
        assert_eq!(
            named
                .missing
                .as_ref()
                .and_then(|missing| missing.words())
                .as_deref(),
            Some("XP not known: Nameless Thing and 1 more")
        );
        // X1: with less room, the name shortens from its end, and goes only when no letter fits.
        let fitted = |room: usize| {
            named
                .missing
                .as_ref()
                .and_then(|missing| missing.fitted(|words| words.chars().count() <= room))
        };
        assert_eq!(
            fitted(39).as_deref(),
            Some("XP not known: Nameless Thing and 1 more")
        );
        assert_eq!(
            fitted(38).as_deref(),
            Some("XP not known: Nameless Thi… and 1 more")
        );
        assert_eq!(
            fitted(34).as_deref(),
            Some("XP not known: Nameless… and 1 more")
        );
        assert_eq!(fitted(27).as_deref(), Some("XP not known: N… and 1 more"));
        assert_eq!(fitted(26).as_deref(), Some("XP not known"));
        // A level that is not known, or not in the table.
        let level = scale.weigh(&[Some(3), None], [Some(100)]);
        assert_eq!(
            (level.rating.clone(), level.missing.clone()),
            (None, missing(Unknown::Level, &[1]))
        );
        let named =
            level.named(|what, at| (what == Unknown::Level && at == 1).then(|| "Mira".into()));
        assert_eq!(
            named.missing.and_then(|missing| missing.words()).as_deref(),
            Some("Level not known: Mira")
        );
        assert_eq!(scale.weigh(&[Some(21)], [Some(100)]).rating, None);
        // A level outside the table is not a level that is not known: no reason to give.
        assert_eq!(scale.weigh(&[Some(21)], [Some(100)]).missing, None);
        // No character in the roster and no saved party.
        assert_eq!(scale.weigh(&[], [Some(100)]).rating, None);
        // Nothing stands against the party.
        assert_eq!(scale.weigh(&five, []).rating, None);
        // Outside the rule: nothing the DM could fill in brings a word, so nothing shows.
        assert_eq!(scale.weigh(&five, []).form(), None);
        assert_eq!(scale.weigh(&[Some(21)], [Some(100)]).form(), None);
        // The rules have no table.
        let no_table = Scale {
            budgets: None,
            party: party_of(&[3]),
        };
        assert_eq!(no_table.weigh(&five, [Some(100)]).rating, None);
        // No word could show without the table, so no input is named as the reason.
        assert_eq!(no_table.weigh(&[None], [None]).missing, None);
        assert_eq!(
            Budgets::of_rule(
                &serde_yaml_ng::from_str("{implementation: dnd2024:other, parameters: {}}")
                    .unwrap()
            ),
            None
        );
    }
}
