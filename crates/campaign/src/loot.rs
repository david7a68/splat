//! Content that a loot line or tracker can refer to.
use crate::content::loot::Loot;
use crate::roll_table::{Table, TableRoll, tables_in};

/// An item from the Library that a loot line can link to.
#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    pub name: String,
    pub lower: String,
    /// The line a loot list keeps: a link that Splat and Obsidian both follow.
    pub link: String,
    pub detail: String,
}

/// What the editor can offer. Built once for each load of the campaign's index.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Sources {
    pub tables: Vec<Table>,
    pub items: Vec<Item>,
}

impl Sources {
    /// The marked tables in `(path, body)` notes, in that order. The first table with a key
    /// decides it, so a bad one says why and no later table takes its place by surprise.
    pub fn tables_from<'a>(notes: impl IntoIterator<Item = (&'a str, &'a str)>) -> Vec<Table> {
        let mut tables = tables_in(notes);
        tables.sort_by(|a, b| a.key.cmp(&b.key));
        tables
    }

    /// A roll on the table with this key, or why there can be none.
    pub fn roll(&self, key: &str) -> Result<TableRoll, String> {
        if let Some(problem) = self.problem(key) {
            return Err(problem);
        }
        self.table(key)
            .ok_or("No roll table has this key.")?
            .roll(None)
    }

    pub fn table(&self, key: &str) -> Option<&Table> {
        self.tables.iter().find(|table| table.key == key)
    }

    /// Why a roll on this key cannot happen, or nothing.
    pub fn problem(&self, key: &str) -> Option<String> {
        match self.table(key) {
            None => Some(format!("No roll table has the key “{key}”.")),
            Some(table) => table.fault(),
        }
    }

    /// The keys among these lines that cannot roll.
    pub fn missing<'a>(&self, lines: impl IntoIterator<Item = &'a str>) -> Vec<String> {
        let mut keys = Vec::new();
        for line in lines {
            if let Loot::Roll { key, .. } = Loot::parse(line)
                && self.problem(&key).is_some()
                && !keys.contains(&key)
            {
                keys.push(key);
            }
        }
        keys
    }
}
