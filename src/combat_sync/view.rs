//! The combat sync panel's reads and acts, free of any toolkit, so the egui panel
//! (`CombatSync::show`) and the GPUI sync editor (`src/gui/pages/party/sync_editor.rs`, on the
//! Party page's sheet and in the run's Correct tab) share them.
//!
//! [`CombatSync::editor`] says what the panel shows after a read: each value with − and +, its
//! label, its limits and its note, in the egui panel's blocks and order. [`CombatSync::step`],
//! [`CombatSync::set_inspiration`] and [`CombatSync::set_condition`] change one value as the
//! egui panel's controls do: the values' revision moves and the change goes half a second after
//! the DM stops. [`CombatSync::rest`] and its acts are the panel's rests and recovery.
use super::{
    COINS, CONDITIONS, CombatSync, HitDice, MAX_COUNT, MAX_HP_BONUS, RestForm, SheetAction,
    current_hp,
};
use serde_json::Value;
use std::collections::BTreeMap;
use std::ops::RangeInclusive;
use std::time::Instant;

impl CombatSync {
    /// `Sync this sheet with D&D Beyond`: on starts reading the live sheet; off stops.
    pub fn set_enabled(&mut self, enabled: bool) {
        let was = self.enabled;
        self.enabled = enabled;
        if was && !enabled {
            self.stop();
        }
        if !was && enabled {
            self.start();
        }
    }

    /// `Check now`: the next read starts at once.
    pub fn check_now(&mut self) {
        self.refresh_only = true;
        self.next = None;
    }

    /// A live sheet has been read.
    pub fn has_remote(&self) -> bool {
        self.remote.is_some()
    }

    /// The sign-in can edit the live sheet.
    pub fn remote_writable(&self) -> Option<bool> {
        self.remote.as_ref().map(|remote| remote.can_edit)
    }

    /// What the panel shows after a read, or `None` before the first one.
    pub fn editor(&self) -> Option<Editor> {
        let remote = self.remote.as_ref()?;
        let max_hp = remote.max_hp;
        let groups = &self.desired.groups;
        let plain = |key: &str, field: &'static str, label: &str, range, note| {
            groups.get(key).map(|group| Step {
                key: key.to_owned(),
                field: Field::Value(field),
                label: label.to_owned(),
                name: label.to_owned(),
                value: group[field].as_i64().unwrap_or(0),
                range,
                note,
            })
        };
        let mut life = Vec::new();
        if let Some(hp) = groups.get("hp") {
            life.push(Row::Step(Step {
                key: "hp".into(),
                field: Field::CurrentHp,
                label: "Current HP".into(),
                name: "Current HP".into(),
                value: current_hp(hp, max_hp),
                range: 0..=max_hp,
                note: Some(format!("of {max_hp}")),
            }));
            life.extend(
                plain(
                    "hp",
                    "temporaryHitPoints",
                    "Temporary HP",
                    0..=MAX_HP_BONUS,
                    None,
                )
                .map(Row::Step),
            );
        }
        if let Some(step) = plain(
            "hp_bonus",
            "bonusHitPoints",
            "Maximum HP modifier",
            -MAX_HP_BONUS..=MAX_HP_BONUS,
            None,
        ) {
            life.push(Row::Step(step));
            // The help sits directly under the stepper it explains, before the death saves.
            life.push(Row::Help(
                "The maximum HP modifier raises or lowers maximum HP, as Aid does.",
            ));
        }
        for (field, label) in [
            ("successCount", "Death save successes"),
            ("failCount", "Death save failures"),
        ] {
            life.extend(plain("death", field, label, 0..=3, None).map(Row::Step));
        }
        if let Some(inspiration) = groups.get("inspiration") {
            life.push(Row::Inspiration(inspiration["inspiration"] == true));
        }
        let conditions = match groups.get("conditions") {
            Some(Value::Object(present)) => Some(Conditions {
                present: CONDITIONS
                    .iter()
                    .map(|(id, name)| (*id, *name, present.contains_key(&id.to_string())))
                    .collect(),
                exhaustion: present.get("4").map(|level| Step {
                    key: "conditions".into(),
                    field: Field::Exhaustion,
                    label: "Exhaustion level".into(),
                    name: "Exhaustion level".into(),
                    value: level.as_i64().unwrap_or(1),
                    range: 1..=6,
                    note: None,
                }),
            }),
            _ => None,
        };
        let mut slots = Vec::new();
        for (key, label, levels) in [("slots", "Spell slots", 9), ("pact", "Pact slots", 5)] {
            for level in 1..=levels {
                if let Some(group) = groups.get(key) {
                    let field = SLOT_FIELDS[level - 1];
                    let label = format!("{label}, level {level}");
                    slots.push(Step {
                        key: key.to_owned(),
                        field: Field::Value(field),
                        name: label.clone(),
                        label,
                        value: group[field].as_i64().unwrap_or(0),
                        range: 0..=99,
                        note: None,
                    });
                }
            }
        }
        let limited = remote
            .named("limited/")
            .filter_map(|(key, label, _)| plain(key, "uses", label, 0..=MAX_COUNT, None))
            .collect();
        let currency = groups.get("currency").map(|_| {
            COINS
                .iter()
                .filter_map(|(field, _, name)| plain("currency", field, name, 0..=MAX_COUNT, None))
                .collect()
        });
        let named = |key: &str, name: &str, steps: Vec<Option<Step>>| Named {
            key: key.to_owned(),
            name: name.to_owned(),
            steps: steps.into_iter().flatten().collect(),
        };
        // Each value of a named group: its small label, and a name that says whose it is.
        let owned = |key: &str,
                     name: &str,
                     field: &'static str,
                     label: &str,
                     range: RangeInclusive<i64>,
                     note: Option<String>| {
            plain(key, field, label, range, note).map(|step| Step {
                name: format!("{name} {}", label.to_lowercase()),
                ..step
            })
        };
        let items: Vec<_> = remote.named("item/").collect();
        let inventory_items = items.len();
        let inventory = items
            .into_iter()
            .filter(|(key, _, _)| groups.contains_key(*key))
            .map(|(key, name, max)| {
                let charges = groups[key].get("charges").is_some().then(|| {
                    owned(
                        key,
                        name,
                        "charges",
                        "Charges spent",
                        0..=MAX_COUNT,
                        max.map(|max| format!("of {max}")),
                    )
                });
                named(
                    key,
                    name,
                    vec![
                        owned(key, name, "quantity", "Quantity", 1..=MAX_COUNT, None),
                        charges.flatten(),
                    ],
                )
            })
            .collect();
        let companions = remote
            .named("creature/")
            .filter(|(key, _, _)| groups.contains_key(*key))
            .map(|(key, name, max)| {
                named(
                    key,
                    name,
                    vec![
                        owned(
                            key,
                            name,
                            "removedHitPoints",
                            "Damage taken",
                            0..=MAX_COUNT,
                            max.map(|max| format!("Stat block average {max} HP")),
                        ),
                        owned(
                            key,
                            name,
                            "temporaryHitPoints",
                            "Temporary HP",
                            0..=MAX_HP_BONUS,
                            None,
                        ),
                    ],
                )
            })
            .collect();
        Some(Editor {
            writable: remote.can_edit,
            life,
            conditions,
            slots,
            limited,
            currency,
            inventory_items,
            inventory,
            companions,
        })
    }

    /// The panel's stepper changed a value to `value`. The stepper keeps to its range, except
    /// that a value already outside it may step back towards it. Nothing changes when the sheet
    /// is read only, the group is gone, or the value is the same.
    pub fn step(&mut self, step: &Step, value: i64) {
        if !self.remote_writable().unwrap_or(false) {
            return;
        }
        let max_hp = self.max_hp().unwrap_or(0);
        let Some(group) = self.desired.groups.get(&step.key) else {
            return;
        };
        let shown = match step.field {
            Field::Value(field) => group[field].as_i64().unwrap_or(0),
            Field::CurrentHp => current_hp(group, max_hp),
            Field::Exhaustion => match group.get("4") {
                Some(level) => level.as_i64().unwrap_or(1),
                None => return,
            },
        };
        let value = value.clamp(
            (*step.range.start()).min(shown),
            (*step.range.end()).max(shown),
        );
        if value == shown {
            return;
        }
        let group = self
            .desired
            .write()
            .get_mut(&step.key)
            .expect("the group is there");
        match step.field {
            Field::Value(field) => group[field] = value.into(),
            Field::CurrentHp => group["removedHitPoints"] = (max_hp - value).into(),
            Field::Exhaustion => group["4"] = value.into(),
        }
        self.changed = Some(Instant::now());
    }

    /// `Has inspiration`.
    pub fn set_inspiration(&mut self, on: bool) {
        if !self.remote_writable().unwrap_or(false) {
            return;
        }
        if let Some(group) = self.desired.groups.get("inspiration")
            && (group["inspiration"] == true) != on
        {
            self.desired.write().get_mut("inspiration").expect("group")["inspiration"] = on.into();
            self.changed = Some(Instant::now());
        }
    }

    /// One sheet condition's box. Exhaustion starts at level 1.
    pub fn set_condition(&mut self, id: u64, present: bool) {
        if !self.remote_writable().unwrap_or(false) {
            return;
        }
        let key = id.to_string();
        let Some(Value::Object(conditions)) = self.desired.groups.get("conditions") else {
            return;
        };
        if conditions.contains_key(&key) == present {
            return;
        }
        let Some(Value::Object(conditions)) = self.desired.write().get_mut("conditions") else {
            return;
        };
        if present {
            conditions.insert(key, if id == 4 { 1.into() } else { Value::Null });
        } else {
            conditions.remove(&key);
        }
        self.changed = Some(Instant::now());
    }

    /// Rests and recovery: the buttons, or the rest the DM is setting up.
    pub fn rest(&self) -> Rest {
        let blocker = self.action_blocker();
        let max_hp = self.max_hp().unwrap_or(0);
        match &self.rest_form {
            None => Rest::Choose {
                blocker,
                restore: self
                    .desired
                    .get("hp")
                    .is_some_and(|hp| current_hp(hp, max_hp) == 0),
            },
            Some(RestForm::Short {
                hit_dice,
                reset_max_hp_bonus,
            }) => Rest::Short {
                dice: self
                    .hit_dice()
                    .iter()
                    .map(|dice| {
                        let left = (dice.level - dice.used).max(0);
                        let label = format!("{} (d{})", dice.name, dice.die);
                        (
                            dice.clone(),
                            Step {
                                key: dice.class.to_string(),
                                field: Field::Value("spend"),
                                name: label.clone(),
                                label,
                                value: hit_dice.get(&dice.class).copied().unwrap_or(0),
                                range: 0..=left,
                                note: Some(format!("{left} of {} left", dice.level)),
                            },
                        )
                    })
                    .collect(),
                reset_max_hp_bonus: *reset_max_hp_bonus,
                blocker,
            },
            Some(RestForm::Long {
                reset_max_hp_bonus,
                reduce_exhaustion,
            }) => Rest::Long {
                reset_max_hp_bonus: *reset_max_hp_bonus,
                reduce_exhaustion: *reduce_exhaustion,
                blocker,
            },
        }
    }

    /// `Short rest…` or `Long rest…`: set the rest up in the panel, before it goes. A long rest
    /// reduces Exhaustion unless the DM says otherwise.
    pub fn open_rest(&mut self, long: bool) {
        if self.action_blocker().is_some() {
            return;
        }
        self.rest_form = Some(if long {
            RestForm::Long {
                reset_max_hp_bonus: false,
                reduce_exhaustion: true,
            }
        } else {
            RestForm::Short {
                hit_dice: BTreeMap::new(),
                reset_max_hp_bonus: false,
            }
        });
    }

    /// The hit dice of one class to spend in the short rest being set up.
    pub fn set_rest_dice(&mut self, class: u64, spend: i64) {
        let left = self
            .hit_dice()
            .iter()
            .find(|dice| dice.class == class)
            .map(|dice| (dice.level - dice.used).max(0));
        if let (Some(left), Some(RestForm::Short { hit_dice, .. })) = (left, &mut self.rest_form) {
            hit_dice.insert(class, spend.clamp(0, left));
        }
    }

    /// `Reset the maximum HP modifier to 0`.
    pub fn set_rest_reset(&mut self, on: bool) {
        match &mut self.rest_form {
            Some(
                RestForm::Short {
                    reset_max_hp_bonus, ..
                }
                | RestForm::Long {
                    reset_max_hp_bonus, ..
                },
            ) => *reset_max_hp_bonus = on,
            None => {}
        }
    }

    /// `Reduce Exhaustion by one level`.
    pub fn set_rest_exhaustion(&mut self, on: bool) {
        if let Some(RestForm::Long {
            reduce_exhaustion, ..
        }) = &mut self.rest_form
        {
            *reduce_exhaustion = on;
        }
    }

    /// `Take short rest` or `Take long rest`: the rest goes with the next exchange, once.
    pub fn take_rest(&mut self) -> Result<(), &'static str> {
        let Some(form) = self.rest_form.clone() else {
            return Err("No rest is set up.");
        };
        self.request(form.into_action())?;
        self.rest_form = None;
        Ok(())
    }

    /// `Cancel`: the rest set up here does not go.
    pub fn cancel_rest(&mut self) {
        self.rest_form = None;
    }

    /// `Restore to life with full HP` or `with 1 HP`.
    pub fn restore_life(&mut self, full: bool) -> Result<(), &'static str> {
        self.request(SheetAction::RestoreLife { full })
    }
}

/// The spell slot fields, by level.
const SLOT_FIELDS: [&str; 9] = [
    "level1", "level2", "level3", "level4", "level5", "level6", "level7", "level8", "level9",
];

/// Which number of a group a stepper changes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Field {
    /// The whole number at this field of the group.
    Value(&'static str),
    /// Current HP, which the sheet keeps as damage taken from its maximum.
    CurrentHp,
    /// The Exhaustion level in the conditions group.
    Exhaustion,
}

/// One value with − and +.
#[derive(Clone, Debug, PartialEq)]
pub struct Step {
    /// The group's key (`hp`, `item/3`), or the class's mapping ID in a short rest.
    pub key: String,
    pub field: Field,
    /// The words beside the stepper: `Current HP`, `Quantity`.
    pub label: String,
    /// The stepper's name, for its − and +: `Decrease Wand of Web charges spent`.
    pub name: String,
    pub value: i64,
    pub range: RangeInclusive<i64>,
    /// Small words after the stepper: `of 62`.
    pub note: Option<String>,
}

/// One row of the Hit points block.
#[derive(Clone, Debug, PartialEq)]
pub enum Row {
    Step(Step),
    /// Help in the value column, under the stepper it explains.
    Help(&'static str),
    /// `Inspiration`, with its `Has inspiration` box.
    Inspiration(bool),
}

/// The sheet's conditions: each official one's box, and the Exhaustion level when it is on.
#[derive(Clone, Debug, PartialEq)]
pub struct Conditions {
    pub present: Vec<(u64, &'static str, bool)>,
    pub exhaustion: Option<Step>,
}

/// A named thing with values of its own: an item, a companion.
#[derive(Clone, Debug, PartialEq)]
pub struct Named {
    pub key: String,
    pub name: String,
    pub steps: Vec<Step>,
}

/// What the panel shows after a read, in its order.
#[derive(Clone, Debug, PartialEq)]
pub struct Editor {
    /// D&D Beyond lets this sign-in edit the sheet. When not, nothing can change.
    pub writable: bool,
    /// `Hit points`: current, temporary, the maximum HP modifier, death saves, inspiration.
    pub life: Vec<Row>,
    /// `Sheet conditions`.
    pub conditions: Option<Conditions>,
    /// `Spell slots spent`: spell slots, then pact slots, level by level.
    pub slots: Vec<Step>,
    /// `Limited uses spent`.
    pub limited: Vec<Step>,
    /// `Currency`, when the sheet has a purse.
    pub currency: Option<Vec<Step>>,
    /// The items the sheet lists, for the section's title.
    pub inventory_items: usize,
    /// `Inventory · N items`: each item's quantity, and the charges spent where it has them.
    pub inventory: Vec<Named>,
    /// `Companions and summons`: each one's damage taken and temporary HP.
    pub companions: Vec<Named>,
}

/// Rests and recovery.
#[derive(Clone, Debug, PartialEq)]
pub enum Rest {
    /// `Short rest…`, `Long rest…`, and at 0 HP the two ways back to life.
    Choose {
        blocker: Option<&'static str>,
        restore: bool,
    },
    /// The short rest set up: each class's hit dice to spend.
    Short {
        dice: Vec<(HitDice, Step)>,
        reset_max_hp_bonus: bool,
        blocker: Option<&'static str>,
    },
    Long {
        reset_max_hp_bonus: bool,
        reduce_exhaustion: bool,
        blocker: Option<&'static str>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn read(can_edit: bool) -> CombatSync {
        let mut sync = CombatSync::default();
        sync.set_enabled(true);
        sync.keep_without_connection();
        sync.simulate_sheet(
            7,
            62,
            &json!({
                "id": 7, "canEdit": can_edit, "removedHitPoints": 4, "temporaryHitPoints": 5,
                "inspiration": true, "bonusHitPoints": 0,
                "deathSaves": {"successCount": 0, "failCount": 0},
                "conditions": [{"id": 4, "level": 2}],
                "spellSlots": [{"level": 1, "used": 1}],
                "currencies": {"cp": 12, "sp": 30, "ep": 0, "gp": 145, "pp": 2},
                "inventory": [
                    {"id": 4, "quantity": 1, "limitedUse": {"maxUses": 3, "numberUsed": 1}, "definition": {"name": "Wand of Web"}}
                ],
                "creatures": [{"id": 5, "name": "Grit", "removedHitPoints": 2, "temporaryHitPoints": 0, "definition": {"name": "Mastiff", "averageHitPoints": 5}}],
                "classes": [{"id": 6, "level": 6, "hitDiceUsed": 2, "definition": {"name": "Fighter", "hitDice": 10}}]
            }),
        );
        sync
    }

    fn life_step(editor: &Editor, label: &str) -> Step {
        editor
            .life
            .iter()
            .find_map(|row| match row {
                Row::Step(step) if step.label == label => Some(step.clone()),
                _ => None,
            })
            .unwrap_or_else(|| panic!("no {label}"))
    }

    #[test]
    fn the_editor_lists_every_value_the_panel_steps() {
        let sync = read(true);
        let editor = sync.editor().expect("a read");
        assert!(editor.writable);
        let current = life_step(&editor, "Current HP");
        assert_eq!(
            (current.value, current.note.as_deref()),
            (58, Some("of 62"))
        );
        assert!(editor.life.contains(&Row::Inspiration(true)));
        assert!(editor.life.iter().any(|row| matches!(row, Row::Help(_))));
        let conditions = editor.conditions.as_ref().expect("conditions");
        assert_eq!(conditions.exhaustion.as_ref().map(|s| s.value), Some(2));
        assert_eq!(editor.slots.len(), 9);
        assert_eq!(editor.currency.as_ref().map(Vec::len), Some(5));
        assert_eq!(editor.inventory_items, 1);
        let wand = &editor.inventory[0];
        assert_eq!(
            wand.steps
                .iter()
                .map(|s| s.name.as_str())
                .collect::<Vec<_>>(),
            ["Wand of Web quantity", "Wand of Web charges spent"]
        );
        assert_eq!(wand.steps[1].note.as_deref(), Some("of 3"));
        assert_eq!(
            editor.companions[0].steps[0].note.as_deref(),
            Some("Stat block average 5 HP")
        );
    }

    #[test]
    fn a_step_changes_one_value_and_moves_the_revision() {
        let mut sync = read(true);
        let editor = sync.editor().unwrap();
        let before = sync.values_revision();
        sync.step(&life_step(&editor, "Current HP"), 50);
        assert_eq!(sync.values()["hp"]["removedHitPoints"], 12);
        assert_ne!(sync.values_revision(), before);
        assert!(sync.dirty());
        // Past the maximum, the stepper stops at it.
        sync.step(&life_step(&editor, "Current HP"), 80);
        assert_eq!(sync.values()["hp"]["removedHitPoints"], 0);
        let gold = editor.currency.as_ref().unwrap()[1].clone();
        sync.step(&gold, 146);
        assert_eq!(sync.values()["currency"]["gp"], 146);
        let exhaustion = editor
            .conditions
            .as_ref()
            .unwrap()
            .exhaustion
            .clone()
            .unwrap();
        sync.step(&exhaustion, 9);
        assert_eq!(sync.values()["conditions"]["4"], 6);
        sync.set_condition(4, false);
        assert!(sync.values()["conditions"].get("4").is_none());
        sync.set_condition(1, true);
        assert!(sync.values()["conditions"].get("1").is_some());
        sync.set_inspiration(false);
        assert_eq!(sync.values()["inspiration"]["inspiration"], false);
        // The same value is no change.
        let same = sync.values_revision();
        sync.step(&gold, 146);
        assert_eq!(sync.values_revision(), same);
    }

    #[test]
    fn a_read_only_sheet_changes_nothing() {
        let mut sync = read(false);
        let editor = sync.editor().unwrap();
        assert!(!editor.writable);
        sync.step(&life_step(&editor, "Current HP"), 10);
        sync.set_inspiration(false);
        sync.set_condition(1, true);
        assert!(!sync.dirty());
        sync.open_rest(true);
        assert!(matches!(
            sync.rest(),
            Rest::Choose {
                blocker: Some(_),
                ..
            }
        ));
    }

    #[test]
    fn a_rest_is_set_up_then_taken_once_or_cancelled() {
        let mut sync = read(true);
        assert!(matches!(
            sync.rest(),
            Rest::Choose {
                blocker: None,
                restore: false
            }
        ));
        sync.open_rest(false);
        sync.set_rest_dice(6, 9);
        sync.set_rest_reset(true);
        let Rest::Short {
            dice,
            reset_max_hp_bonus,
            ..
        } = sync.rest()
        else {
            panic!("a short rest");
        };
        assert_eq!(dice[0].1.value, 4, "at most the dice left");
        assert_eq!(dice[0].1.note.as_deref(), Some("4 of 6 left"));
        assert!(reset_max_hp_bonus);
        sync.cancel_rest();
        assert!(matches!(sync.rest(), Rest::Choose { .. }));
        sync.open_rest(true);
        sync.set_rest_exhaustion(false);
        assert!(matches!(
            sync.rest(),
            Rest::Long {
                reduce_exhaustion: false,
                ..
            }
        ));
        assert_eq!(sync.take_rest(), Ok(()));
        assert!(matches!(
            sync.rest(),
            Rest::Choose {
                blocker: Some("Wait for the current sheet read."),
                ..
            }
        ));
    }
}
