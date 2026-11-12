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
    /// Each saved character of the campaign's party by the ID of its copy's entry, with its
    /// level from that saved sheet. `None` is a level not known.
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
    /// How many creatures against the party have no XP and no CR.
    pub unknown: u64,
    /// The word and the budgets. `None` when a creature's XP, a level or the table is not
    /// known, or when nothing stands against the party.
    pub rating: Option<Rating>,
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
    /// A character's level from the saved sheet now, by the ID of its copy's entry. A run keeps
    /// a copy of each sheet from when it started; the saved one wins while the campaign has it.
    pub fn level(&self, character: &Entry) -> Option<Option<u64>> {
        let kept = character_level(character)?;
        Some(self.party.get(&character.id).copied().unwrap_or(kept))
    }

    /// The one computation. `party` is the level of each character on the party's side; with
    /// none, the campaign's party stands in. `foes` is the XP of each creature that is not on
    /// the party's side, `None` where it is not known.
    pub fn weigh(
        &self,
        party: &[Option<u64>],
        foes: impl IntoIterator<Item = Option<u64>>,
    ) -> Difficulty {
        let (mut xp, mut unknown, mut any) = (0_u64, 0_u64, false);
        for foe in foes {
            any = true;
            match foe {
                Some(each) => xp = xp.saturating_add(each),
                None => unknown += 1,
            }
        }
        let campaign_party = party.is_empty();
        let saved: Vec<_>;
        let party = if campaign_party {
            saved = self.party.values().copied().collect();
            &saved
        } else {
            party
        };
        Difficulty {
            xp,
            unknown,
            rating: (any && unknown == 0)
                .then(|| self.rate(party, xp, campaign_party))
                .flatten(),
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

impl Rating {
    /// `XP 1,050 of 1,125`: the XP and the budget of the word. Above High has no budget.
    pub fn numbers(&self) -> String {
        let xp = crate::encounter_run::grouped_number(self.xp);
        match self.word {
            Word::AboveHigh => format!("XP {xp}"),
            word => format!(
                "XP {xp} of {}",
                crate::encounter_run::grouped_number(self.budgets[word as usize])
            ),
        }
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
        assert_eq!(rating.numbers(), "XP 750 of 750");
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
            (above.word, above.numbers().as_str()),
            (Word::AboveHigh, "XP 2,400")
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
        // A creature with no XP and no CR keeps its count and the XP that is known.
        let unknown = scale.weigh(&five, [Some(100), None]);
        assert_eq!(
            (unknown.xp, unknown.unknown, unknown.rating),
            (100, 1, None)
        );
        // A level that is not known, or not in the table.
        assert_eq!(scale.weigh(&[Some(3), None], [Some(100)]).rating, None);
        assert_eq!(scale.weigh(&[Some(21)], [Some(100)]).rating, None);
        // No character in the roster and no saved party.
        assert_eq!(scale.weigh(&[], [Some(100)]).rating, None);
        // Nothing stands against the party.
        assert_eq!(scale.weigh(&five, []).rating, None);
        // The rules have no table.
        let no_table = Scale {
            budgets: None,
            party: party_of(&[3]),
        };
        assert_eq!(no_table.weigh(&five, [Some(100)]).rating, None);
        assert_eq!(
            Budgets::of_rule(
                &serde_yaml_ng::from_str("{implementation: dnd2024:other, parameters: {}}")
                    .unwrap()
            ),
            None
        );
    }
}
