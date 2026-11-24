//! Loot in a run. The encounter's loot is its note's `## Loot` list as the run began, and a
//! creature's is what its roster row or group member gave it. A creature's card shows one line
//! of it until the creature drops to 0 HP, then the whole list. The ended view lists the
//! encounter's loot and each downed creature's. A roll's result takes the place of its button,
//! the saved run keeps it, and Undo takes it back.
use super::{Command, Run, State};
use crate::roll_table::TableRoll;
use splat::content::loot::Loot;
pub use splat_session_records::rewards::{Holder, place};
#[cfg(test)]
pub(crate) use splat_session_records::rewards::{Rewards, rewards};
use splat_session_records::rewards::{and_list, holders, note_lines, plain_roll};

use splat_campaign::encounter_run::Found;

/// One loot line to roll, with the table it rolls on.
#[derive(Clone, Debug)]
pub struct LootRoll {
    /// The line's place: see `place`.
    pub place: String,
    pub owner: String,
    pub roll: TableRoll,
    pub times: u32,
}

/// Make each roll on the run's dice, keep what it found, and give the log a line for each.
pub(crate) fn roll(state: &mut State, rolls: Vec<LootRoll>) -> Result<Vec<String>, String> {
    let mut lines = Vec::new();
    for loot in rolls {
        if state.loot.contains_key(&loot.place) {
            return Err("This loot has its roll. Undo it to roll again.".into());
        }
        let mut found = Vec::new();
        for _ in 0..loot.times {
            let (value, text) = loot.roll.draw(&mut state.random)?;
            lines.push(format!(
                "Loot for {}, {}: {value} on {}, {text}",
                loot.owner, loot.roll.key, loot.roll.dice
            ));
            found.push(Found {
                value,
                dice: loot.roll.dice.clone(),
                text,
            });
        }
        state.loot.insert(loot.place, found);
    }
    Ok(lines)
}

/// The Undo words for these rolls. Short, because the ended view's header holds them.
pub(crate) fn describe(rolls: &[LootRoll]) -> String {
    match rolls {
        [one] => step_words(one.times, Some(&one.roll.key)),
        all => step_words(all.iter().map(|roll| roll.times).sum(), None),
    }
}

/// A loot step's words: `loot roll on gems`, `2 loot rolls on gems`, or, for several lines,
/// `3 loot rolls`.
fn step_words(times: u32, key: Option<&str>) -> String {
    let rolls = if times == 1 {
        "loot roll".to_owned()
    } else {
        format!("{times} loot rolls")
    };
    match key {
        Some(key) => format!("{rolls} on {key}"),
        None => rolls,
    }
}

/// `3`, `3 and 4`, `3, 4 and 1`.
fn values(found: &[Found]) -> String {
    let words: Vec<String> = found.iter().map(|f| f.value.to_string()).collect();
    and_list(&words)
}

impl Run {
    /// Everyone with loot: the encounter first, then participants in turn order.
    pub fn holders(&self) -> Vec<Holder> {
        holders(&self.name, self.prep().0, &self.state)
    }

    /// The roll lines of these holders with no result yet, as `(holder, line)`.
    pub fn unrolled<'a>(
        &self,
        holders: impl IntoIterator<Item = &'a Holder>,
    ) -> Vec<(&'a Holder, usize)> {
        let sources = self.content.as_ref().map(|index| index.loot_sources());
        holders
            .into_iter()
            .flat_map(|holder| {
                holder
                    .lines
                    .iter()
                    .enumerate()
                    .filter(|(at, line)| match Loot::parse(line) {
                        // A line whose table cannot roll waits for the DM to fix it. It never
                        // stops the others, and it is not counted as left to roll.
                        Loot::Roll { key, .. } => {
                            !self.state.loot.contains_key(&place(&holder.key, *at))
                                && sources
                                    .as_ref()
                                    .is_none_or(|sources| sources.problem(&key).is_none())
                        }
                        Loot::Text(_) => false,
                    })
                    .map(move |(at, _)| (holder, at))
            })
            .collect()
    }

    /// The rolls for these lines, on the campaign's tables. None if one cannot roll.
    pub fn loot_rolls(&self, picks: &[(Holder, usize)]) -> Result<Vec<LootRoll>, String> {
        picks
            .iter()
            .filter_map(|(holder, at)| match Loot::parse(&holder.lines[*at]) {
                Loot::Roll { times, key } => Some((holder, *at, times, key)),
                Loot::Text(_) => None,
            })
            .map(|(holder, at, times, key)| {
                let roll = match self.content.as_ref() {
                    Some(index) => index.roll_table(&key),
                    None => Err(
                        "The campaign’s notes are still loading. Roll again in a moment.".into(),
                    ),
                }?;
                Ok(LootRoll {
                    place: place(&holder.key, at),
                    owner: holder.name.clone(),
                    roll,
                    times,
                })
            })
            .collect()
    }

    /// Roll these lines as one step.
    pub fn roll_loot(&mut self, picks: &[(Holder, usize)]) {
        match self.loot_rolls(picks) {
            Ok(rolls) if !rolls.is_empty() => self.run_command(Command::Loot(rolls)),
            Ok(_) => {}
            Err(error) => self.error = Some(error),
        }
    }

    /// These holders' loot as a Markdown list, with each roll's result, for a session note.
    pub fn loot_markdown(&self, holders: &[Holder]) -> String {
        let mut out = String::new();
        for holder in holders {
            out.push_str(&format!("**{}**\n", holder.name));
            for (line, _) in note_lines(&self.state, holder) {
                out.push_str(&format!("- {line}\n"));
            }
            out.push('\n');
        }
        out.trim_end().to_owned()
    }
}

/// A loot line's roll in words, `2 × gems`, for a view in any toolkit.
pub fn plain_roll_words(times: u32, key: &str) -> String {
    plain_roll(times, key)
}

/// What a line's rolls came to, `3 and 5`, for a view in any toolkit.
pub fn found_values(found: &[Found]) -> String {
    values(found)
}
