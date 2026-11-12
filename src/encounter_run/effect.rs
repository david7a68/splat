//! A condition on one participant of a run: an official condition, or words the DM pinned to
//! it, with the time, uses or recharge the DM gave. One entry holds a condition and its count,
//! so a count cannot outlive its condition. Every reader of a run's conditions takes this type
//! and reads it through these functions.
use serde::{Deserialize, Serialize};
use splat::content::overrides::same_condition;

/// A plain condition or label is a string in the run file, as before effects. An effect with a
/// time, uses or a recharge is one object in the same list.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum Condition {
    Label(String),
    Effect(Effect),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Effect {
    pub name: String,
    #[serde(flatten)]
    pub time: Time,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum Time {
    /// `left` is the number of this participant's turns that it still covers, with the
    /// current one. It goes down when that participant's turn ends, and stops at 0.
    Rounds {
        rounds: u32,
        left: u32,
    },
    /// The words show on each of the participant's turns.
    OnTurn {
        on_turn: bool,
    },
    Uses {
        uses: u32,
        left: u32,
    },
    /// The lowest and highest roll on a d6 that makes it ready again.
    Recharge {
        recharge: [u8; 2],
        spent: bool,
    },
}

/// The most rounds or uses that typed words can give: a day of rounds.
const MOST: u32 = 14_400;

impl Condition {
    /// The condition's name: the official name, or the DM's words without their time.
    pub(crate) fn name(&self) -> &str {
        match self {
            Self::Label(name) => name,
            Self::Effect(effect) => &effect.name,
        }
    }

    pub(crate) fn time(&self) -> Option<Time> {
        match self {
            Self::Label(_) => None,
            Self::Effect(effect) => Some(effect.time),
        }
    }

    /// Words the DM typed, as one entry. A time, a count of uses or a recharge at their end is
    /// read; anything else stays words: `Mage Armor 8 hours` is a label.
    pub(crate) fn read(text: &str) -> Self {
        let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
        match split_time(&text) {
            Some((name, time)) => Self::Effect(Effect {
                name: official_name(name).unwrap_or(name).to_owned(),
                time,
            }),
            None => Self::Label(text),
        }
    }

    /// Two entries for one condition: `poisoned` and `Poisoned` are one.
    pub(crate) fn same(&self, other: &str) -> bool {
        same_condition(self.name(), other)
    }

    /// The whole words, for the hover, the accessible name, the notice line and every reader
    /// outside the run: `Bless 9`, `Bless · last round`, `Legendary actions 2 of 3`.
    pub(crate) fn words(&self) -> String {
        match self.count() {
            Some(count) if count.starts_with(|c: char| c.is_ascii_digit()) && self.rounds() => {
                format!("{} {count}", self.name())
            }
            Some(count) if self.rounds() => format!("{} · {count}", self.name()),
            Some(count) => format!("{} {count}", self.name()),
            None => self.name().to_owned(),
        }
    }

    fn rounds(&self) -> bool {
        matches!(self.time(), Some(Time::Rounds { .. }))
    }

    /// The part after the name that changes: `9`, `last round`, `time is up`, `2 of 3`,
    /// `ready`, `spent`. None for a label and for words shown on each turn.
    pub(crate) fn count(&self) -> Option<String> {
        Some(match self.time()? {
            Time::Rounds { left: 0, .. } => "time is up".to_owned(),
            Time::Rounds { left: 1, .. } => "last round".to_owned(),
            Time::Rounds { left, .. } => left.to_string(),
            Time::OnTurn { .. } => return None,
            Time::Uses { uses, left } => format!("{left} of {uses}"),
            Time::Recharge { spent: false, .. } => "ready".to_owned(),
            Time::Recharge { spent: true, .. } => "spent".to_owned(),
        })
    }

    /// Every form the count can take, so that its place is as wide as the widest one and a
    /// change of count moves no word.
    pub(crate) fn count_forms(&self) -> Vec<String> {
        match self.time() {
            Some(Time::Rounds { rounds, .. }) => vec![
                rounds.to_string(),
                "· last round".to_owned(),
                "· time is up".to_owned(),
            ],
            Some(Time::Uses { uses, .. }) => vec![format!("{uses} of {uses}")],
            Some(Time::Recharge { .. }) => vec!["ready".to_owned(), "spent".to_owned()],
            Some(Time::OnTurn { .. }) | None => Vec::new(),
        }
    }

    /// The words of the notice line on the participant's turn. Only an effect with a time
    /// reminds: a count of rounds, or `on its turn`.
    pub(crate) fn reminder(&self) -> Option<String> {
        matches!(self.time()?, Time::Rounds { .. } | Time::OnTurn { .. }).then(|| self.words())
    }

    /// True for an entry with a time, uses or a recharge: it takes the effects' place on its
    /// row, before plain conditions.
    pub(crate) fn is_effect(&self) -> bool {
        matches!(self, Self::Effect(_))
    }

    /// What a sheet's removal of this condition names: `Frightened, 7 rounds left`.
    pub(crate) fn what_went(&self) -> String {
        match self.time() {
            Some(Time::Rounds { left: 0, .. }) => format!("{}, time was up", self.name()),
            Some(Time::Rounds { left, .. }) => format!(
                "{}, {}",
                self.name(),
                crate::design::count(left as usize, "round left", "rounds left")
            ),
            _ => self.name().to_owned(),
        }
    }
}

/// Tests compare a row's conditions with the words it shows.
#[cfg(test)]
impl PartialEq<str> for Condition {
    fn eq(&self, other: &str) -> bool {
        self.words() == other
    }
}
#[cfg(test)]
impl PartialEq<&str> for Condition {
    fn eq(&self, other: &&str) -> bool {
        self.words() == *other
    }
}
#[cfg(test)]
impl PartialEq<String> for Condition {
    fn eq(&self, other: &String) -> bool {
        self.words() == *other
    }
}

impl From<String> for Condition {
    fn from(name: String) -> Self {
        Self::Label(name)
    }
}

impl From<&str> for Condition {
    fn from(name: &str) -> Self {
        Self::Label(name.to_owned())
    }
}

/// The end of a participant's turn: each count of rounds goes down by one, and stops at 0.
/// Splat removes nothing. An absent participant's count does not go down. Returns the Log's
/// line for each count that reached 0.
pub(super) fn count_down(p: &mut super::Participant) -> Vec<String> {
    if !p.present {
        return Vec::new();
    }
    let mut lines = Vec::new();
    for condition in &mut p.conditions {
        if let Condition::Effect(Effect {
            name,
            time: Time::Rounds { left, .. },
        }) = condition
            && *left > 0
        {
            *left -= 1;
            if *left == 0 {
                lines.push(format!("{}: {name} · time is up.", p.name));
            }
        }
    }
    lines
}

/// The click on a recharge's words: a ready one is spent; a spent one rolls a d6 on the run's
/// dice, and is ready again on a roll in its range.
pub(super) fn recharge(
    state: &mut super::State,
    index: usize,
    at: usize,
) -> Result<String, String> {
    let p = state
        .participants
        .get_mut(index)
        .ok_or("Participant is missing.")?;
    let Some(Condition::Effect(Effect {
        name,
        time: Time::Recharge { recharge, spent },
    })) = p.conditions.get_mut(at)
    else {
        return Err("That effect has no recharge.".into());
    };
    if !*spent {
        *spent = true;
        return Ok(format!("{}: {name} spent.", p.name));
    }
    let rolled = state.random.die(6)?;
    let ready = (u32::from(recharge[0])..=u32::from(recharge[1])).contains(&rolled);
    *spent = !ready;
    Ok(format!(
        "{}: {name}: Rolled {rolled} · {}.",
        p.name,
        if ready { "ready again" } else { "still spent" }
    ))
}

/// The official name for these words, when they are one: `frightened` is `Frightened`.
fn official_name(name: &str) -> Option<&'static str> {
    crate::combat_sync::CONDITIONS
        .iter()
        .map(|(_, official)| *official)
        .find(|official| same_condition(official, name))
}

/// The words before a time at the end of `text`, and that time. `10 rounds`; `1 minute`, as
/// 10 rounds; `on its turn`; `3 uses`; `recharge 6`, `recharge 5–6` or `recharge 5-6`.
fn split_time(text: &str) -> Option<(&str, Time)> {
    let lower = text.to_lowercase();
    // Lowercase has the same byte lengths for the ASCII words read here, so a cut at a byte of
    // `lower` is a cut of `text`; other words are compared only as they end the text.
    if lower.len() != text.len() {
        return None;
    }
    let words: Vec<&str> = lower.split(' ').collect();
    let name_of = |taken: usize| {
        let kept: usize = words[..words.len() - taken]
            .iter()
            .map(|word| word.len() + 1)
            .sum();
        text[..kept.saturating_sub(1)].trim()
    };
    let number = |word: &str| word.parse::<u32>().ok().filter(|n| (1..=MOST).contains(n));
    let (taken, time) = match words.as_slice() {
        [.., "on", "its", "turn"] => (3, Time::OnTurn { on_turn: true }),
        [.., n, "round" | "rounds"] => {
            let rounds = number(n)?;
            (
                2,
                Time::Rounds {
                    rounds,
                    left: rounds,
                },
            )
        }
        [.., n, "minute" | "minutes"] => {
            let rounds = number(n)?.checked_mul(10).filter(|n| *n <= MOST)?;
            (
                2,
                Time::Rounds {
                    rounds,
                    left: rounds,
                },
            )
        }
        [.., n, "use" | "uses"] => {
            let uses = number(n)?;
            (2, Time::Uses { uses, left: uses })
        }
        [.., "recharge", range] => {
            let (low, high) = match range.split_once(['-', '–']) {
                Some((low, high)) => (low.parse().ok()?, high.parse().ok()?),
                None => (range.parse().ok()?, 6),
            };
            if !(1..=6).contains(&low) || !(low..=6).contains(&high) {
                return None;
            }
            (
                2,
                Time::Recharge {
                    recharge: [low, high],
                    spent: false,
                },
            )
        }
        _ => return None,
    };
    let name = name_of(taken);
    (!name.is_empty()).then_some((name, time))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_words_read_a_time_at_their_end_and_keep_anything_else_as_words() {
        let effect = |name: &str, time| {
            Condition::Effect(Effect {
                name: name.to_owned(),
                time,
            })
        };
        for (typed, read) in [
            (
                "Wis save DC 15 (Hold Person) on its turn",
                effect(
                    "Wis save DC 15 (Hold Person)",
                    Time::OnTurn { on_turn: true },
                ),
            ),
            (
                "frightened 10 rounds",
                effect(
                    "Frightened",
                    Time::Rounds {
                        rounds: 10,
                        left: 10,
                    },
                ),
            ),
            (
                "Bless 1 minute",
                effect(
                    "Bless",
                    Time::Rounds {
                        rounds: 10,
                        left: 10,
                    },
                ),
            ),
            (
                "Fire Breath recharge 5–6",
                effect(
                    "Fire Breath",
                    Time::Recharge {
                        recharge: [5, 6],
                        spent: false,
                    },
                ),
            ),
            (
                "Fire Breath recharge 5-6",
                effect(
                    "Fire Breath",
                    Time::Recharge {
                        recharge: [5, 6],
                        spent: false,
                    },
                ),
            ),
            (
                "Fire Breath recharge 6",
                effect(
                    "Fire Breath",
                    Time::Recharge {
                        recharge: [6, 6],
                        spent: false,
                    },
                ),
            ),
            (
                "Legendary actions 3 uses",
                effect("Legendary actions", Time::Uses { uses: 3, left: 3 }),
            ),
            (
                "Mage Armor 8 hours",
                Condition::Label("Mage Armor 8 hours".into()),
            ),
            ("10 rounds", Condition::Label("10 rounds".into())),
            ("Bless 0 rounds", Condition::Label("Bless 0 rounds".into())),
            (
                "Breath recharge 7",
                Condition::Label("Breath recharge 7".into()),
            ),
            ("Poisoned", Condition::Label("Poisoned".into())),
        ] {
            assert_eq!(Condition::read(typed), read, "{typed}");
        }
    }

    #[test]
    fn a_count_says_its_rounds_then_last_round_then_time_is_up() {
        let mut bless = Condition::read("Bless 10 rounds");
        assert_eq!(bless.words(), "Bless 10");
        assert_eq!(bless.reminder().as_deref(), Some("Bless 10"));
        for (left, words) in [(1, "Bless · last round"), (0, "Bless · time is up")] {
            if let Condition::Effect(Effect {
                time: Time::Rounds { left: l, .. },
                ..
            }) = &mut bless
            {
                *l = left;
            }
            assert_eq!(bless.words(), words);
        }
        assert_eq!(Condition::read("Fire Breath recharge 5").reminder(), None);
        assert_eq!(Condition::from("Poisoned").reminder(), None);
    }

    #[test]
    fn a_run_file_keeps_a_label_as_a_string_and_an_effect_as_one_object() {
        let conditions = vec![
            Condition::from("Poisoned"),
            Condition::read("Frightened 10 rounds"),
            Condition::read("Wis save DC 15 (Hold Person) on its turn"),
            Condition::read("Fire Breath recharge 5–6"),
            Condition::read("Legendary actions 3 uses"),
        ];
        let text = serde_json::to_string(&conditions).unwrap();
        assert_eq!(
            text,
            r#"["Poisoned",{"name":"Frightened","rounds":10,"left":10},{"name":"Wis save DC 15 (Hold Person)","on_turn":true},{"name":"Fire Breath","recharge":[5,6],"spent":false},{"name":"Legendary actions","uses":3,"left":3}]"#
        );
        let read: Vec<Condition> = serde_json::from_str(&text).unwrap();
        assert_eq!(read, conditions);
        // An object this version does not know is not read as another kind.
        assert!(serde_json::from_str::<Condition>(r#"{"name":"Bless","hours":8}"#).is_err());
    }
}
