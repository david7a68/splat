//! A new NPC from dice: a name and three attributes. A campaign's own roll tables decide the
//! words, because only the campaign knows how its people are named. Built-in lists fill any
//! gap. Nothing is written until the DM saves the result.
use crate::roll_table::{RollRow, marked_tables, table_rows};
use splat::execution::dice::{Dice, Random};

/// The attributes, in the order a note lists them.
pub const ATTRIBUTES: [&str; 3] = ["Look", "Manner", "Want"];
const NAME_KEY: &str = "npc-name";
const ATTRIBUTE_KEYS: [&str; 3] = ["npc-look", "npc-manner", "npc-want"];

const NAMES: &[&str] = &[
    "Abra Quell",
    "Aldous Venn",
    "Bexley Marrow",
    "Brannoch Tull",
    "Caddis Oake",
    "Corvane Pell",
    "Dessa Holloway",
    "Dunmar Reck",
    "Elowen Strake",
    "Emeric Sallow",
    "Fenna Lark",
    "Garrick Ashlow",
    "Hesper Thorne",
    "Idris Calloway",
    "Jessamy Crane",
    "Kestrin Vale",
    "Lorcan Dray",
    "Maren Gault",
    "Nessa Brock",
    "Oswin Tarn",
    "Perrin Howe",
    "Quilla Stade",
    "Rook Ambry",
    "Sabeth Noor",
    "Tamsin Rill",
    "Ulric Fenn",
    "Vesper Lode",
    "Wren Hallam",
    "Yorick Maddox",
    "Zella Korrin",
];
const LOOKS: &[&str] = &[
    "A burn scar across one palm, kept under a glove",
    "One eye clouded white, and a habit of turning the head to look",
    "Mismatched boots, both of excellent quality",
    "Ink on every fingertip",
    "A broken nose, set badly and worn proudly",
    "A coat with too many pockets, all of them full",
    "Very clean hands in very dirty clothes",
    "Teeth filed flat on one side",
    "A shaved head with a fresh tattoo still healing",
    "A heavy limp, and a walking stick with a steel tip",
    "A ring worn on a cord around the neck",
    "A smell of smoke that never quite washes out",
    "Sleeves rolled to the elbow in any weather",
    "A missing ear, and long hair combed to hide it",
    "Spectacles repaired with wire",
    "An old military coat with the badges cut off",
    "Sun-cracked lips and pale eyes",
    "A small dried flower pinned to the collar",
    "A laugh line on one side of the mouth only",
    "Bitten nails, and rings too large for the fingers",
];
const MANNERS: &[&str] = &[
    "Answers every question with a price",
    "Never finishes a sentence if a nod will do",
    "Calls everyone by the wrong name, on purpose",
    "Counts things under their breath",
    "Stands too close, and doesn't notice",
    "Keeps one hand on a weapon, even when eating",
    "Agrees with everything, and does none of it",
    "Tells the same story, with a new ending each time",
    "Apologizes before giving bad news, and smiles after",
    "Speaks only to the person in charge",
    "Laughs a moment too late",
    "Writes down everything you say",
    "Whispers, so that you have to lean in",
    "Offers food to everyone, and watches them eat it",
    "Corrects small errors, and ignores large ones",
    "Refuses to sit with their back to a door",
    "Talks to their tools as if they were people",
    "Asks the same question twice, to compare the answers",
    "Won't touch money with bare hands",
    "Goes silent when anyone mentions family",
];
const WANTS: &[&str] = &[
    "To clear a debt before the collector finds out where they live",
    "A letter delivered, with no questions about who it is for",
    "To see a rival lose in public, and to be there when it happens",
    "Enough money to leave and never come back",
    "To find out who sold them out",
    "A license, a pardon, or anything else with a seal on it",
    "To get a child out of an apprenticeship",
    "To be asked for advice, just once",
    "A cure, and the name of the person who caused the sickness",
    "To buy back something they pawned",
    "The job of the person above them",
    "A witness who will say that they were somewhere else",
    "To keep a secret for one more week",
    "To hear what happened to someone who went away",
    "To be paid in full, on the day, in front of people",
    "A safe place to hide something heavy",
    "To make a dead friend's name mean something",
    "Revenge, but only if it looks like an accident",
    "To retire, and to choose who takes over",
    "One honest answer from someone in power",
];

#[derive(Clone, Debug, PartialEq)]
struct Table {
    dice: String,
    rows: Vec<RollRow>,
}

impl Table {
    fn roll(&self, random: &mut Random) -> Result<String, String> {
        let value = Dice::parse(&self.dice)?.roll(random, false)?.total;
        self.rows
            .iter()
            .find(|row| (row.min..=row.max).contains(&value))
            .map(|row| row.text.clone())
            .ok_or_else(|| format!("{} rolled {value}, and no row has it.", self.dice))
    }
}

/// A table fault in plain words, with its source marker kept for hover details.
#[derive(Clone, Debug, PartialEq)]
pub struct TableProblem {
    pub path: String,
    pub marker: String,
    pub message: String,
}

/// The campaign's tables for the generator. A part with no table uses its built-in list.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Tables {
    name: Option<Table>,
    attributes: [Option<Table>; 3],
    /// Tables that cannot be used, and why. The DM sees these with the result.
    pub problems: Vec<TableProblem>,
    pub sources: Vec<String>,
}

impl Tables {
    /// Read `(path, body)` notes in the order given. The first table for a key decides it, so
    /// a bad one gives its warning and no later table takes its place by surprise.
    pub fn read<'a>(notes: impl IntoIterator<Item = (&'a str, &'a str)>) -> Self {
        let mut tables = Self::default();
        let mut seen = std::collections::HashSet::new();
        for (path, body) in notes {
            for (marker, rows) in marked_tables(body) {
                let Ok((key, dice)) = marker else {
                    continue;
                };
                let (slot, label) = if key == NAME_KEY {
                    (&mut tables.name, "name".to_owned())
                } else if let Some(at) = ATTRIBUTE_KEYS.iter().position(|k| *k == key) {
                    (&mut tables.attributes[at], ATTRIBUTES[at].to_lowercase())
                } else {
                    continue;
                };
                if !seen.insert(key.clone()) {
                    continue;
                }
                match table_rows(&dice, &rows) {
                    Ok(rows) => {
                        *slot = Some(Table { dice, rows });
                        tables.sources.push(format!("{key}: {path}"));
                    }
                    Err(error) => tables.problems.push(TableProblem {
                        path: path.to_owned(),
                        marker: key.clone(),
                        message: format!(
                            "{path}: {} Built-in {} stand in until it is fixed.",
                            error.replacen("the table", &format!("the {label} table"), 1),
                            if key == NAME_KEY {
                                "names"
                            } else {
                                "attributes"
                            }
                        ),
                    }),
                }
            }
        }
        tables
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Generated {
    pub name: String,
    pub attributes: [String; 3],
}

impl Generated {
    /// The note's text under its title.
    pub fn details(&self) -> String {
        ATTRIBUTES
            .iter()
            .zip(&self.attributes)
            .map(|(label, text)| format!("- **{label}:** {text}\n"))
            .collect()
    }
}

fn pick(list: &[&str], random: &mut Random) -> Result<String, String> {
    let sides = u32::try_from(list.len()).map_err(|e| e.to_string())?;
    let at = random.die(sides)? as usize - 1;
    Ok(list[at].to_owned())
}

fn part(table: Option<&Table>, list: &[&str], random: &mut Random) -> Result<String, String> {
    match table {
        Some(table) => table.roll(random),
        None => pick(list, random),
    }
}

pub fn generate(tables: &Tables, random: &mut Random) -> Result<Generated, String> {
    let [look, manner, want] = &tables.attributes;
    Ok(Generated {
        name: part(tables.name.as_ref(), NAMES, random)?,
        attributes: [
            part(look.as_ref(), LOOKS, random)?,
            part(manner.as_ref(), MANNERS, random)?,
            part(want.as_ref(), WANTS, random)?,
        ],
    })
}

/// Fresh dice each time, as a roll with no run open uses.
pub fn roll(tables: &Tables) -> Result<Generated, String> {
    generate(
        tables,
        &mut Random::seeded(crate::hermetic::new_uuid().as_u128() as u64),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_no_tables_the_built_in_lists_give_a_name_and_three_attributes() {
        let npc = generate(&Tables::default(), &mut Random::seeded(7)).unwrap();
        assert!(NAMES.contains(&npc.name.as_str()));
        assert!(LOOKS.contains(&npc.attributes[0].as_str()));
        assert!(MANNERS.contains(&npc.attributes[1].as_str()));
        assert!(WANTS.contains(&npc.attributes[2].as_str()));
        assert_eq!(
            npc.details().lines().count(),
            3,
            "one line for each attribute"
        );
        assert!(npc.details().starts_with("- **Look:** "));
    }

    #[test]
    fn the_same_dice_give_the_same_npc_and_every_list_entry_can_come_up() {
        let tables = Tables::default();
        assert_eq!(
            generate(&tables, &mut Random::seeded(9)).unwrap(),
            generate(&tables, &mut Random::seeded(9)).unwrap()
        );
        let last = NAMES.len() as u32;
        let picked = pick(NAMES, &mut Random::controlled([last])).unwrap();
        assert_eq!(picked, NAMES[NAMES.len() - 1]);
        let first = pick(NAMES, &mut Random::controlled([1])).unwrap();
        assert_eq!(first, NAMES[0]);
    }

    #[test]
    fn a_campaign_table_replaces_only_its_own_part() {
        let note = "<!-- splat:rolls key=npc-name dice=1d2 -->\n\n| d2 | Name |\n|---|---|\n| 1 | Tally |\n| 2 | Blot |\n\n<!-- splat:rolls key=npc-want dice=1d4 -->\n\n| d4 | Want |\n|---|---|\n| 1-4 | A clean writ |\n";
        let tables = Tables::read([("Names.md", note)]);
        assert!(tables.problems.is_empty(), "{:?}", tables.problems);
        let npc = generate(&tables, &mut Random::seeded(3)).unwrap();
        assert!(["Tally", "Blot"].contains(&npc.name.as_str()));
        assert!(LOOKS.contains(&npc.attributes[0].as_str()));
        assert!(MANNERS.contains(&npc.attributes[1].as_str()));
        assert_eq!(npc.attributes[2], "A clean writ");
    }

    #[test]
    fn a_bad_table_says_so_and_the_list_stands_in() {
        let note = "<!-- splat:rolls key=npc-name dice=1d6 -->\n\n| d6 | Name |\n|---|---|\n| 1 | Tally |\n";
        let tables = Tables::read([("Names.md", note)]);
        assert_eq!(tables.problems.len(), 1);
        assert!(
            tables.problems[0]
                .message
                .contains("Names.md: the name table stops at 1, but 1d6 rolls up to 6.")
        );
        assert_eq!(tables.problems[0].marker, NAME_KEY);
        let npc = generate(&tables, &mut Random::seeded(3)).unwrap();
        assert!(NAMES.contains(&npc.name.as_str()));
    }

    #[test]
    fn the_first_table_decides_its_key_even_when_it_is_bad() {
        let bad = "<!-- splat:rolls key=npc-name dice=1d6 -->\n\n| d6 | Name |\n|---|---|\n| 1 | Tally |\n";
        let good = "<!-- splat:rolls key=npc-name dice=1d2 -->\n\n| d2 | Name |\n|---|---|\n| 1-2 | Later |\n";
        let tables = Tables::read([("A.md", bad), ("B.md", good)]);
        assert_eq!(tables.problems.len(), 1);
        let npc = generate(&tables, &mut Random::seeded(3)).unwrap();
        assert!(NAMES.contains(&npc.name.as_str()), "{}", npc.name);
    }

    #[test]
    fn the_first_table_wins_and_other_keys_are_left_alone() {
        let one = "<!-- splat:rolls key=npc-name dice=1d2 -->\n\n| d2 | Name |\n|---|---|\n| 1-2 | First |\n";
        let two = "<!-- splat:rolls key=npc-name dice=1d2 -->\n\n| d2 | Name |\n|---|---|\n| 1-2 | Second |\n\n<!-- splat:rolls key=gifts dice=1d2 -->\n\n| d2 | Gift |\n|---|---|\n| 1-2 | Knife |\n";
        let tables = Tables::read([("A.md", one), ("B.md", two)]);
        let npc = generate(&tables, &mut Random::seeded(3)).unwrap();
        assert_eq!(npc.name, "First");
        assert!(tables.problems.is_empty());
    }
}
