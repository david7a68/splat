//! Encounter difficulty: how a fight's XP compares with the party's XP budget. The budget per
//! character comes from the rule entry that the campaign uses; no table is written here. Every
//! screen that shows the word calls [`Scale::weigh`], and keeps its answer until an input changes.
use eframe::egui;
use serde::Deserialize;
use serde_yaml_ng::Value;
use splat::content::Entry;
use std::collections::BTreeMap;

/// The rule contract that holds the XP budget per character, by level.
pub const CONTRACT: &str = "dnd2024:rules_combat_encounters_step_2_determine_your_xp_budget";

/// Explicit XP wins; a bare CR supplies its standard reward. `None` when the entry has neither.
pub(crate) fn creature_xp(entry: &Entry) -> Option<u64> {
    crate::library::presentation::challenge_text(entry)
        .and_then(splat::content::stat_blocks::challenge_xp)
}

/// A player character's level, from its saved sheet. `None` for a creature that is not a
/// character, or a sheet with no level.
pub(crate) fn character_level(entry: &Entry) -> Option<Option<u64>> {
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
#[derive(Clone, Debug, Default, PartialEq, Eq)]
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
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Missing {
    pub what: Unknown,
    /// Each one by its place in what `Scale::weigh` was given: among the foes, or in the party
    /// (with no character in the roster, the campaign's party in `Scale::party`'s order).
    pub at: Vec<usize>,
    /// Their names, each once, from the caller that knows them (`Difficulty::named`).
    pub names: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
        let letters: Vec<char> = first.chars().collect();
        (1..letters.len())
            .rev()
            .map(|kept| {
                let name: String = letters[..kept].iter().collect();
                format!("{}: {}…{more}", self.short(), name.trim_end())
            })
            .find(|words| fits(words))
            .or_else(|| Some(self.short().to_owned()))
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

#[derive(Clone, Debug, PartialEq, Eq)]
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

/// N7 (ruling R1, T5.7): the one difficulty line of a fight, for the encounter editor, the plan,
/// Ready to run and Arrivals. It holds what stands before the form (the summary's facts and
/// members, a fight's own facts) and the form, and [`Line::fit`] decides what gives way on one
/// line, in one fixed order:
///
/// 1. the word's limit (`(up to 2,800)`);
/// 2. the facts, by whole facts and members: [`Line::fact`]s by their [`Gives`] rank, then the
///    summary from its end, which ends with `…` where it was cut;
/// 3. the form's XP number (`Fight after: Low` keeps its label and its word).
///
/// The word never gives way, and nothing is cut inside its words. A line never takes a second
/// line: when even the word alone does not fit, the word alone is the answer. The caller gives
/// the facts and a measure (`fits`); the order is not the caller's to choose.
pub struct Line<'a> {
    difficulty: &'a Difficulty,
    units: Vec<Unit<'a>>,
    label: Option<&'static str>,
}

/// When a [`Line::fact`] gives way: never, or at a rank (lower first, and before the summary).
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "N7: Arrivals (package R) and the plan (package S) draw their lines with facts and a label when they move"
    )
)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Gives {
    At(u8),
    Never,
}

struct Unit<'a> {
    /// The separator before it on the line: ` · ` between facts, `, ` between members.
    before: &'static str,
    words: &'a str,
    /// `None` for the summary's units, which give way from the end after every ranked fact.
    gives: Option<Gives>,
}

/// What [`Line::fit`] keeps: the words before the form (with `…` where the summary was cut) and
/// the form's parts, the label first.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fitted {
    pub lead: String,
    pub form: Vec<Part>,
    /// Something gave way: hover and the accessible name keep the whole line.
    pub cut: bool,
}

impl Fitted {
    /// The line as plain words.
    pub fn text(&self) -> String {
        let form: String = self.form.iter().map(|(words, _)| words.as_str()).collect();
        if self.lead.is_empty() {
            form
        } else {
            format!("{} · {form}", self.lead)
        }
    }
}

impl<'a> Line<'a> {
    pub fn new(difficulty: &'a Difficulty) -> Self {
        Self {
            difficulty,
            units: Vec::new(),
            label: None,
        }
    }

    /// One fact before the form (`Lost Patrol`, `3 members`), in the line's order.
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "N7: Arrivals (package R) moves to it")
    )]
    pub fn fact(mut self, words: &'a str, gives: Gives) -> Self {
        self.units.push(Unit {
            before: " · ",
            words,
            gives: Some(gives),
        });
        self
    }

    /// A roster's summary (`5 party · 2 × Cinder Wolf, 1 × Ember Cultist · Changed`): its facts
    /// and members give way from its end, whole.
    pub fn summary(mut self, summary: &'a str) -> Self {
        for fact in summary.split(" · ").filter(|f| !f.is_empty()) {
            for (at, member) in fact.split(", ").enumerate() {
                self.units.push(Unit {
                    before: if at == 0 { " · " } else { ", " },
                    words: member,
                    gives: None,
                });
            }
        }
        self
    }

    /// The words before the form's XP (`Fight after: `); they stay with the word.
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "N7: Arrivals (package R) moves to it")
    )]
    pub fn label(mut self, label: &'static str) -> Self {
        self.label = Some(label);
        self
    }

    /// The longest line that `fits` (one line, plain words), in the order this type states.
    /// `None` when the fight has no form (no party, no table, no foe).
    pub fn fit(&self, fits: impl Fn(&str) -> bool) -> Option<Fitted> {
        let label = self.label.map(|label| (label.to_owned(), false));
        let with_label = |parts: Vec<Part>| -> Vec<Part> { label.iter().cloned().chain(parts).collect() };
        // The forms, longest first; the last one is the word with its label only.
        let forms: Vec<Vec<Part>> = match (&self.difficulty.rating, &self.difficulty.missing) {
            (Some(rating), _) => vec![
                with_label(rating.parts(true)),
                with_label(rating.parts(false)),
                with_label(vec![(rating.word.text().to_owned(), true)]),
            ],
            (None, Some(_)) => vec![with_label(self.difficulty.parts()?)],
            (None, None) => return None,
        };
        // The units' order of giving way: ranked facts by rank, then the summary from its end.
        let mut order: Vec<usize> = (0..self.units.len())
            .filter(|at| !matches!(self.units[*at].gives, Some(Gives::Never)))
            .collect();
        order.sort_by_key(|at| match self.units[*at].gives {
            Some(Gives::At(rank)) => (0, rank, 0),
            _ => (1, 0, usize::MAX - at),
        });
        let lead = |gone: &[usize]| -> String {
            let mut text = String::new();
            let mut cut = false;
            for (at, unit) in self.units.iter().enumerate() {
                if gone.contains(&at) {
                    // A cut summary ends with `…`, once, with the separator of its first gone unit.
                    if unit.gives.is_none() && !cut {
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
                text.push_str(unit.words);
            }
            text
        };
        let line = |gone: &[usize], form: &Vec<Part>| Fitted {
            lead: lead(gone),
            form: form.clone(),
            cut: !gone.is_empty() || *form != forms[0],
        };
        // 1. The limit gives way first.
        for form in &forms[..forms.len().min(2)] {
            let fitted = line(&[], form);
            if fits(&fitted.text()) {
                return Some(fitted);
            }
        }
        // 2. Then the facts, in their order, with the form that has no limit.
        let shorter = &forms[forms.len().min(2) - 1];
        for count in 1..=order.len() {
            let fitted = line(&order[..count], shorter);
            if fits(&fitted.text()) {
                return Some(fitted);
            }
        }
        // 3. Then the XP number: the label and the word stay.
        Some(line(&order, &forms[forms.len() - 1]))
    }
}

/// One part of a line in the line's own type: the word in `text`, its separators and numbers
/// in `muted`.
pub(crate) fn append(
    job: &mut egui::text::LayoutJob,
    ui: &egui::Ui,
    kind: crate::design::Type,
    words: &str,
    color: egui::Color32,
) {
    job.append(
        words,
        0.0,
        egui::TextFormat {
            font_id: kind.font(ui),
            extra_letter_spacing: kind.tracking(),
            color,
            ..Default::default()
        },
    );
}

/// The difficulty form's parts at the end of `job`, in `kind`: the word in the text colour, the
/// rest muted. Every line that shows a fight's difficulty draws it with this.
pub(crate) fn append_form(
    job: &mut egui::text::LayoutJob,
    ui: &egui::Ui,
    kind: crate::design::Type,
    parts: &[Part],
) {
    let c = crate::design::colors();
    for (words, word) in parts {
        append(job, ui, kind, words, if *word { c.text } else { c.muted });
    }
}

/// The width of a job on one line.
pub(crate) fn width(ui: &egui::Ui, job: &egui::text::LayoutJob) -> f32 {
    ui.fonts_mut(|fonts| fonts.layout_job(job.clone()).size().x)
}

/// The word's hover and accessible description: the party's budgets.
pub(crate) fn describe(ui: &egui::Ui, response: egui::Response, rating: &Rating) -> egui::Response {
    let line = rating.budget_line();
    ui.ctx()
        .accesskit_node_builder(response.id, |node| node.set_description(line.clone()));
    response.on_hover_text(line)
}

#[cfg(test)]
/// The rule as the campaign's packages hold it, read through the same path the app uses.
pub(crate) fn builtin_budgets() -> Budgets {
    let text = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/builtin/dnd2024-definitions/rules/combat_encounters_step_2_determine_your_xp_budget.md"
    ))
    .unwrap();
    let entry = Entry::parse("rule.md", &text).unwrap();
    Budgets::of_rule(&entry.fields["splat_rule"]).expect("the table in the package reads")
}

/// Mill Road's answer in the scenario's numbers: Above High for 5 at level 3, XP 6,300.
#[cfg(test)]
pub(crate) fn above_high() -> Rating {
    Rating {
        word: Word::AboveHigh,
        budgets: [750, 1125, 2000],
        xp: 6300,
        characters: 5,
        levels: (3, 3),
        campaign_party: false,
    }
}

#[cfg(test)]
pub(crate) fn party_of(levels: &[u64]) -> BTreeMap<uuid::Uuid, Option<u64>> {
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
        let summary = "5 party · 2 × Cinder Wolf, 1 × Ember Cultist";
        let ready = Line::new(&low).summary(summary);
        let at = |room| ready.fit(fits(room)).unwrap().text();
        // Room for all of it.
        let whole = "5 party · 2 × Cinder Wolf, 1 × Ember Cultist · XP 400 · Low (up to 2,800)";
        assert_eq!(at(200), whole);
        assert!(!ready.fit(fits(200)).unwrap().cut);
        // FG1: the limit goes first, before any fact is cut.
        assert_eq!(at(whole.chars().count() - 1), "5 party · 2 × Cinder Wolf, 1 × Ember Cultist · XP 400 · Low");
        // Then the summary from its end, by whole members and facts, with `…` (never `2…`).
        assert_eq!(at(55), "5 party · 2 × Cinder Wolf, … · XP 400 · Low");
        assert_eq!(at(35), "5 party · … · XP 400 · Low");
        // Then the XP number; the word never goes, and nothing takes a second line.
        assert_eq!(at(10), "… · Low");
        assert_eq!(at(1), "… · Low");
        // R1, the contract's counterexample: Arrivals' `Fight after` keeps its label and word;
        // the member count gives way, then the side, then the XP number. The name stays.
        let arrivals = Line::new(&low)
            .fact("Lost Patrol", Gives::Never)
            .fact("3 members", Gives::At(0))
            .fact("Enemies", Gives::At(1))
            .label("Fight after: ");
        let at = |room| arrivals.fit(fits(room)).unwrap().text();
        assert_eq!(
            at(200),
            "Lost Patrol · 3 members · Enemies · Fight after: XP 400 · Low (up to 2,800)"
        );
        assert_eq!(at(60), "Lost Patrol · Enemies · Fight after: XP 400 · Low");
        assert_eq!(at(39), "Lost Patrol · Fight after: XP 400 · Low");
        assert_eq!(at(30), "Lost Patrol · Fight after: Low");
        // A fight with no form has no line; one whose word is missing keeps its reason.
        assert!(Line::new(&Difficulty::default()).summary(summary).fit(fits(200)).is_none());
        let unknown = Difficulty {
            missing: Some(Missing {
                what: Unknown::Xp,
                at: vec![0],
                names: vec!["Nameless Thing".into()],
            }),
            ..Difficulty::default()
        };
        assert_eq!(
            Line::new(&unknown).summary(summary).fit(fits(30)).unwrap().text(),
            "5 party · … · XP not known"
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
