//! One creature added to a fight with no group prepared: found in the campaign, or made up at
//! the table. It joins as a group of one kind does, so its initiative, numbered names, undo and
//! saved state follow the same rules. A creature made up here lives only in this run.
use super::{Command, GroupMember, Phase, Run, RunGroup};
use crate::design;
use serde_yaml_ng::{Mapping, Value};
use splat::content::{
    Entry,
    encounter_group::MAX_COUNT,
    overrides::{Base, Change, MAX_NAME, Overrides},
};
use std::sync::Arc;

const DEFAULT_SIDE: &str = "enemies";
const MATCHES_SHOWN: usize = 5;
const BUTTON_NAME: usize = 24;
const MAX_HP: i64 = 1_000_000;
const MAX_AC: i64 = 1000;

/// A creature of this campaign that a run can add. Linked characters are not among these.
#[derive(Debug)]
pub struct RunCreature {
    pub entry: Arc<Entry>,
    pub lower: String,
    pub detail: String,
    /// The stat block has no hit points, so `detail` shows the `HP not set` fault.
    pub hp_missing: bool,
}

/// The form in the run's Arrivals panel. Never saved.
#[derive(Default)]
pub struct AdHoc {
    pub query: String,
    pub picked: Option<Arc<RunCreature>>,
    pub count: String,
    pub hp: String,
    pub ac: String,
    pub total: String,
    pub side: Option<String>,
    /// The words for which the DM chose Show all.
    pub show_all: Option<String>,
    /// HP takes the keyboard on the next frame.
    pub focus_hp: bool,
}

/// What the form says now, when it can be added.
pub struct Checked {
    /// None for a creature made up here.
    entry: Option<Arc<Entry>>,
    name: String,
    count: u64,
    hp: Option<i64>,
    ac: Option<i64>,
    overrides: Overrides,
    /// What a roll adds. A creature made up here has no bonus to add.
    bonus: Option<i64>,
}

fn number_in(
    text: &str,
    what: &str,
    range: std::ops::RangeInclusive<i64>,
) -> Result<Option<i64>, String> {
    let text = text.trim();
    if text.is_empty() {
        return Ok(None);
    }
    text.parse()
        .ok()
        .filter(|number| range.contains(number))
        .map(Some)
        .ok_or_else(|| format!("{what} must be from {} to {}.", range.start(), range.end()))
}

/// An entry for a creature with no note. It holds only what the DM typed.
fn made_up_entry(name: &str, hp: i64, ac: Option<i64>) -> Entry {
    let stat = |value: i64| {
        let mut block = Mapping::new();
        block.insert("value".into(), value.into());
        Value::Mapping(block)
    };
    let mut stats = Mapping::new();
    stats.insert("max_hp".into(), stat(hp));
    stats.insert("initiative".into(), stat(0));
    if let Some(ac) = ac {
        stats.insert("armor_class".into(), stat(ac));
    }
    let mut fields = Mapping::new();
    fields.insert("splat_schema".into(), 2.into());
    fields.insert("splat_creature".into(), Value::Mapping(Mapping::new()));
    fields.insert("splat_stats".into(), Value::Mapping(stats));
    Entry {
        path: "Made up in this fight".into(),
        id: crate::hermetic::new_uuid(),
        key: None,
        name: name.to_owned(),
        kind: "creature".into(),
        fields: fields.into(),
        body: String::new(),
        body_line: 1,
        inventory: Vec::new(),
        inventory_containers: Vec::new(),
        flags: Vec::new(),
    }
}

impl AdHoc {
    pub fn side(&self) -> &str {
        self.side.as_deref().unwrap_or(DEFAULT_SIDE)
    }

    pub fn total(&self) -> Option<i64> {
        crate::design::parse_signed(&self.total)
    }

    /// The creature chosen, or the one whose whole name was typed.
    fn creature<'a>(&'a self, known: &'a [Arc<RunCreature>]) -> Option<&'a Arc<RunCreature>> {
        self.picked.as_ref().or_else(|| {
            let typed = self.query.trim().to_lowercase();
            known.iter().find(|creature| creature.lower == typed)
        })
    }

    /// What this form would add, or why it cannot be added yet.
    pub fn check(&self, known: &[Arc<RunCreature>]) -> Result<Checked, String> {
        let count = number_in(&self.count, "Count", 1..=MAX_COUNT as i64)?.unwrap_or(1) as u64;
        let hp = number_in(&self.hp, "HP", 1..=MAX_HP)?;
        let ac = number_in(&self.ac, "AC", 0..=MAX_AC)?;
        if let Some(creature) = self.creature(known) {
            let base = Base::of(&creature.entry, self.side());
            if hp.is_none() && base.max_hp.is_none() {
                return Err(format!(
                    "{} has no maximum HP. Type its HP here.",
                    base.name
                ));
            }
            let change = |typed: Option<i64>, was: Option<i64>| {
                typed.filter(|_| typed != was).map(Change::Set)
            };
            let overrides = Overrides {
                max_hp: change(hp, base.max_hp),
                armor_class: change(ac, base.armor_class),
                ..Overrides::default()
            };
            return Ok(Checked {
                entry: Some(creature.entry.clone()),
                name: base.name,
                count,
                hp,
                ac,
                overrides,
                bonus: base.initiative_bonus,
            });
        }
        let name: String = self.query.trim().chars().take(MAX_NAME).collect();
        if name.is_empty() {
            return Err("Find or name a creature first.".into());
        }
        if hp.is_none() {
            return Err(NEEDS_HP.into());
        }
        Ok(Checked {
            entry: None,
            name,
            count,
            hp,
            ac,
            overrides: Overrides::default(),
            bonus: Some(0),
        })
    }

    /// The form as a group of one line, ready to add.
    pub fn group(&self, checked: Checked) -> Arc<RunGroup> {
        let entry = checked.entry.unwrap_or_else(|| {
            Arc::new(made_up_entry(
                &checked.name,
                checked.hp.unwrap_or(1),
                checked.ac,
            ))
        });
        Arc::new(RunGroup {
            id: crate::hermetic::new_uuid(),
            path: String::new(),
            name: checked.name,
            side: self.side().to_owned(),
            members: vec![GroupMember {
                entry,
                count: checked.count,
                overrides: checked.overrides,
            }],
            notes: String::new(),
            xp: 0,
            xp_unknown: Vec::new(),
            unavailable: 0,
            unavailable_members: Vec::new(),
            warnings: Vec::new(),
            problem: None,
            encounter: None,
            encounter_name: None,
        })
    }
}

/// What the words typed find: the whole name first, then names that start with them.
pub(crate) fn matches_for<'a>(
    known: &'a [Arc<RunCreature>],
    typed: &str,
) -> Vec<&'a Arc<RunCreature>> {
    let mut matches: Vec<_> = known
        .iter()
        .filter(|creature| creature.lower.contains(typed))
        .collect();
    matches.sort_by_key(|creature| crate::matching::creature_tier(&creature.lower, typed));
    matches
}

/// A short number field with its word before it. Returns whether Enter was pressed in it.
/// The made-up form's own reason. Its line stays when the DM types the HP.
const NEEDS_HP: &str = "Type its HP. A creature made up here has no stat block.";

impl Run {
    /// An ended fight shows its summary, with no panel to add from.
    pub fn takes_arrivals(&self) -> bool {
        !matches!(self.state.phase, Phase::Ended { .. })
    }

    /// The panel for what joins the fight, in place of whatever was open beside the list.
    pub fn show_arrivals(&mut self) {
        self.panel = super::Panel::Groups;
        self.close_lookup = true;
    }

    /// An NPC with no stat block joins as a creature made up at the table: the form, with its
    /// name filled in and the keyboard in HP.
    pub fn made_up(&mut self, name: &str) {
        self.ad_hoc = AdHoc {
            query: name.to_owned(),
            focus_hp: true,
            ..AdHoc::default()
        };
        self.show_arrivals();
    }

    /// FJ5 (iter-27-R), for Lookup's `Add to fight`: why `stat_block` cannot join this fight now,
    /// or None. The same reasons as Arrivals' Add, for a creature that has no typed values.
    pub fn add_to_fight_reason(&self, stat_block: &Entry) -> Option<String> {
        if !self.takes_arrivals() {
            return Some("This encounter has ended. Undo the ending to continue.".to_owned());
        }
        let base = Base::of(stat_block, DEFAULT_SIDE);
        if base.max_hp.is_none() {
            return Some(format!(
                "{} has no maximum HP. Add it in Arrivals with its HP.",
                base.name
            ));
        }
        base.initiative_bonus.is_none().then(|| {
            format!(
                "{} has no initiative bonus. Add it in Arrivals with a total.",
                base.name
            )
        })
    }

    /// FJ5 (iter-27-R), Lookup's `Add to fight` while a run is open: one of `stat_block` joins
    /// on the enemies' side with its initiative rolled, by the run's own add steps: a creature
    /// as Arrivals adds it (`Command::AddCreature`), an NPC (`npc`) under its own name with this
    /// stat block, as World's form adds it (`Command::AddGroup`). Each is one step, and the
    /// header's Undo (`Undo: add Guard`) takes it away. The run's page is not opened. Returns why
    /// it did not join (`add_to_fight_reason`, or the step's refusal).
    pub fn add_entry_to_fight(
        &mut self,
        stat_block: Arc<Entry>,
        npc: Option<&Entry>,
    ) -> Result<(), String> {
        if let Some(reason) = self.add_to_fight_reason(&stat_block) {
            return Err(reason);
        }
        let command = match npc {
            Some(npc) => Command::AddGroup(
                crate::world::npc_group(npc, &stat_block, DEFAULT_SIDE),
                None,
            ),
            None => {
                let base = Base::of(&stat_block, DEFAULT_SIDE);
                let checked = Checked {
                    entry: Some(stat_block),
                    name: base.name,
                    count: 1,
                    hp: None,
                    ac: None,
                    overrides: Overrides::default(),
                    bonus: base.initiative_bonus,
                };
                Command::AddCreature(AdHoc::default().group(checked), None)
            }
        };
        self.apply(command)
    }

    /// The name in the made-up form, while Arrivals shows it.
    #[cfg(test)]
    pub fn made_up_name(&self) -> Option<&str> {
        (self.panel == super::Panel::Groups && self.ad_hoc.focus_hp)
            .then_some(self.ad_hoc.query.as_str())
    }

    /// The sides this run already has, so that a new creature joins one and not a misspelling.
    pub fn sides_to_join(&self) -> Vec<String> {
        let mut sides = vec![DEFAULT_SIDE.to_owned()];
        for participant in &self.state.participants {
            if !sides.contains(&participant.side) {
                sides.push(participant.side.clone());
            }
        }
        sides
    }
}

/// What the Arrivals form's Add says and does now, for any toolkit's form.
pub struct CreatureAdd {
    /// `Add 2 × Goblin at 14`, `Add Goblin and roll`.
    pub label: String,
    /// The whole name, for the accessible name.
    pub name: String,
    /// Why Add cannot add now.
    pub blocked: Option<String>,
    /// The HP and AC fields' placeholders: the chosen stat block's values, or what is needed.
    pub hp_hint: String,
    pub ac_hint: String,
    /// The words name no known creature: it is made up here.
    pub made_up: bool,
}

impl Run {
    /// The Arrivals form's Add, as the form stands (`add_creature_form`).
    pub fn creature_add(&self) -> CreatureAdd {
        let known = self.creatures.clone().unwrap_or_default();
        let creature = self.ad_hoc.creature(&known).cloned();
        let base = creature
            .as_ref()
            .map(|creature| Base::of(&creature.entry, self.ad_hoc.side()));
        let shown = |value: Option<Option<i64>>, none: &str| {
            value
                .flatten()
                .map_or_else(|| none.to_owned(), |value| value.to_string())
        };
        let hp_hint = shown(base.as_ref().map(|base| base.max_hp), "Needed");
        let ac_hint = shown(base.as_ref().map(|base| base.armor_class), "None");
        let checked = self.ad_hoc.check(&known);
        let total = self.ad_hoc.total();
        let may_roll = !self.joins_side(self.ad_hoc.side());
        let name = checked
            .as_ref()
            .map_or_else(|_| "this creature".to_owned(), |it| it.name.clone());
        // Says what it adds. A long name stays in the accessible name, whole.
        let short: String = name.chars().take(BUTTON_NAME).collect();
        let short = checked.as_ref().ok().filter(|it| it.count > 1).map_or_else(
            || short.clone(),
            |it| {
                format!(
                    "{} × {short}",
                    design::count(it.count as usize, "", "").trim()
                )
            },
        );
        let label = match total {
            Some(total) => format!("Add {short} at {total}"),
            None if may_roll => format!("Add {short} and roll"),
            None => format!("Add {short}"),
        };
        let ended = matches!(self.state.phase, Phase::Ended { .. });
        let no_bonus =
            total.is_none() && may_roll && checked.as_ref().is_ok_and(|it| it.bonus.is_none());
        let blocked = match &checked {
            _ if ended => Some("This encounter has ended. Undo the ending to continue.".to_owned()),
            Err(why) => Some(why.clone()),
            Ok(_) if no_bonus => Some("It has no initiative bonus. Type a total.".to_owned()),
            Ok(_) => None,
        };
        CreatureAdd {
            label,
            name,
            blocked,
            hp_hint,
            ac_hint,
            made_up: creature.is_none(),
        }
    }

    /// Add what the Arrivals form says, when it can; the form empties after a join.
    pub fn add_creature_from_form(&mut self) {
        if self.creature_add().blocked.is_some() {
            return;
        }
        let known = self.creatures.clone().unwrap_or_default();
        let total = self.ad_hoc.total();
        let Ok(checked) = self.ad_hoc.check(&known) else {
            return;
        };
        let group = self.ad_hoc.group(checked);
        self.run_command(Command::AddCreature(group, total));
        if self.error.is_none() {
            self.ad_hoc = AdHoc::default();
            self.return_focus = self.lookup_open;
        }
    }

    /// The known creatures that match the form's words, and how many of them show before
    /// `Show all N` (all of them once the DM chose it, or when one more would show).
    pub fn creature_matches(&self) -> (Vec<Arc<RunCreature>>, usize) {
        let known = self.creatures.clone().unwrap_or_default();
        let typed = self.ad_hoc.query.trim().to_lowercase();
        if typed.is_empty() {
            return (Vec::new(), 0);
        }
        let matches: Vec<Arc<RunCreature>> =
            matches_for(&known, &typed).into_iter().cloned().collect();
        let all = self.ad_hoc.show_all.as_deref() == Some(typed.as_str());
        let shown = if all || matches.len() == MATCHES_SHOWN + 1 {
            matches.len()
        } else {
            matches.len().min(MATCHES_SHOWN)
        };
        (matches, shown)
    }
}
