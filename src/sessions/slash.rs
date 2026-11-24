//! `/` in a session note opens a short menu of commands. Each command ends with a link
//! where the `/` was typed, so the DM keeps writing and never leaves the note.
use crate::design::icons;

/// The search shows this many results. A longer list belongs in Lookup.
const RESULTS: usize = 8;
/// More letters than any command has: the `/` was not a command.
const LONGEST_TYPED: usize = 16;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Command {
    GenerateNpc,
    Search,
}

const COMMANDS: [Command; 2] = [Command::GenerateNpc, Command::Search];

impl Command {
    fn word(self) -> &'static str {
        match self {
            Self::GenerateNpc => "Generate NPC",
            Self::Search => "Search",
        }
    }

    fn icon(self) -> &'static str {
        match self {
            Self::GenerateNpc => icons::DICE,
            Self::Search => icons::SEARCH,
        }
    }

    fn hint(self) -> &'static str {
        match self {
            Self::GenerateNpc => "Roll a new NPC and link it",
            Self::Search => "Find an entry or a note and link it",
        }
    }

    /// The letters typed after the `/` begin the command, or one of its words.
    fn matches(self, typed: &str) -> bool {
        let typed = typed.to_lowercase();
        let word = self.word().to_lowercase();
        word.starts_with(&typed) || word.split(' ').any(|part| part.starts_with(&typed))
    }
}

/// The `/` before the caret starts a line or a word. In `and/or` or `1/2` it is only a slash.
fn starts_command(body: &str, caret: usize) -> bool {
    let mut before = body
        .chars()
        .take(caret)
        .collect::<Vec<_>>()
        .into_iter()
        .rev();
    before.next() == Some('/') && before.next().is_none_or(char::is_whitespace)
}

/// The GPUI note's `/` menu (`src/gui/pages/sessions/`): the same rules, toolkit-free.
pub mod plain {
    use super::{COMMANDS, Command, LONGEST_TYPED, RESULTS};
    use crate::lookup::ContentIndex;

    /// A command the menu lists: its words, icon and hint.
    pub struct Choice {
        pub word: &'static str,
        pub icon: &'static str,
        pub hint: &'static str,
        pub search: bool,
    }

    /// The `/` just before `caret` (a character count) starts a line or a word.
    pub fn opens_at(body: &str, caret: usize) -> bool {
        super::starts_command(body, caret)
    }

    /// The commands the letters after the `/` match, or `None` when they are no command's
    /// (a new line, too many letters, or no match): the menu closes.
    pub fn commands(typed: &str) -> Option<Vec<Choice>> {
        if typed.contains('\n') || typed.chars().count() > LONGEST_TYPED {
            return None;
        }
        let found: Vec<Choice> = COMMANDS
            .into_iter()
            .filter(|command| command.matches(typed))
            .map(|command| Choice {
                word: command.word(),
                icon: command.icon(),
                hint: command.hint(),
                search: command == Command::Search,
            })
            .collect();
        (!found.is_empty()).then_some(found)
    }

    /// A search's best results: each one's name, detail and link; and how many were found.
    pub fn search(index: &ContentIndex, query: &str) -> (Vec<(String, String, String)>, usize) {
        let wanted = query.trim().to_lowercase();
        if wanted.is_empty() {
            return (Vec::new(), 0);
        }
        let all = index.search(&wanted);
        let found = all
            .iter()
            .copied()
            .take(RESULTS)
            .map(|at| {
                (
                    index.items[at].entry.name.clone(),
                    index.result_detail(at),
                    index.link_to(at),
                )
            })
            .collect();
        (found, all.len())
    }

    /// A new NPC from the campaign's tables, or why none was made.
    pub fn roll(index: &ContentIndex) -> Result<crate::npc_generator::Generated, String> {
        let tables = index.npc_tables();
        crate::npc_generator::roll(&tables)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_slash_is_a_command_only_where_a_word_starts() {
        assert!(starts_command("/", 1));
        assert!(starts_command("Met /", 5));
        assert!(starts_command("Met\n/", 5));
        assert!(!starts_command("and/", 4));
        assert!(!starts_command("Met ", 4));
        assert!(Command::GenerateNpc.matches(""));
        assert!(Command::GenerateNpc.matches("npc"));
        assert!(Command::Search.matches("SEA"));
        assert!(!Command::Search.matches("sea "));
        assert!(!Command::GenerateNpc.matches("x"));
    }
}
