//! One creature added to a fight with no group prepared: found in the campaign, or made up at
//! the table. It joins as a group of one kind does, so its initiative, numbered names, undo and
//! saved state follow the same rules. A creature made up here lives only in this run.
use super::{Command, GroupMember, Phase, Run, RunGroup};
use crate::design::{self, Action, Density, Type, colors, icons};
use eframe::egui;
use serde_yaml_ng::{Mapping, Value};
use splat::content::{
    Entry,
    encounter_group::MAX_COUNT,
    overrides::{Base, Change, MAX_NAME, Overrides},
};
use std::sync::Arc;

const FIELD: &str = "Find or name a creature";
const DEFAULT_SIDE: &str = "enemies";
const MATCHES_SHOWN: usize = 5;
const BUTTON_NAME: usize = 24;
const MAX_HP: i64 = 1_000_000;
const MAX_AC: i64 = 1000;

/// A creature of this campaign that a run can add. Linked characters are not among these.
#[derive(Debug)]
pub(crate) struct RunCreature {
    pub entry: Arc<Entry>,
    pub lower: String,
    pub detail: String,
}

/// The form in the run's Arrivals panel. Never saved.
#[derive(Default)]
pub(super) struct AdHoc {
    pub(super) query: String,
    pub(super) picked: Option<Arc<RunCreature>>,
    pub(super) count: String,
    pub(super) hp: String,
    pub(super) ac: String,
    pub(super) total: String,
    pub(super) side: Option<String>,
}

/// What the form says now, when it can be added.
pub(super) struct Checked {
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
    pub(super) fn side(&self) -> &str {
        self.side.as_deref().unwrap_or(DEFAULT_SIDE)
    }

    pub(super) fn total(&self) -> Option<i64> {
        self.total.trim().parse().ok()
    }

    /// The creature chosen, or the one whose whole name was typed.
    fn creature<'a>(&'a self, known: &'a [Arc<RunCreature>]) -> Option<&'a Arc<RunCreature>> {
        self.picked.as_ref().or_else(|| {
            let typed = self.query.trim().to_lowercase();
            known.iter().find(|creature| creature.lower == typed)
        })
    }

    /// What this form would add, or why it cannot be added yet.
    pub(super) fn check(&self, known: &[Arc<RunCreature>]) -> Result<Checked, String> {
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
            return Err("Type its HP. A creature made up here has no stat block.".into());
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
    pub(super) fn group(&self, checked: Checked) -> Arc<RunGroup> {
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
pub(super) fn matches_for<'a>(
    known: &'a [Arc<RunCreature>],
    typed: &str,
) -> Vec<&'a Arc<RunCreature>> {
    let mut matches: Vec<_> = known
        .iter()
        .filter(|creature| creature.lower.contains(typed))
        .collect();
    matches.sort_by_key(|creature| (creature.lower != typed, !creature.lower.starts_with(typed)));
    matches
}

/// A short number field with its word before it. Returns whether Enter was pressed in it.
fn short_field(
    ui: &mut egui::Ui,
    word: &str,
    name: &str,
    text: &mut String,
    hint: &str,
    width: f32,
) -> bool {
    ui.label(Type::Small.rich(ui, word).color(colors().muted));
    ui.add_space(design::XS);
    let response = design::number_field(ui, name, text, hint, width, Density::Read.control());
    response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))
}

impl Run {
    /// An ended fight shows its summary, with no panel to add from.
    pub(crate) fn takes_arrivals(&self) -> bool {
        !matches!(self.state.phase, Phase::Ended { .. })
    }

    /// The panel for what joins the fight, in place of whatever was open beside the list.
    pub(crate) fn show_arrivals(&mut self) {
        self.panel = super::Panel::Groups;
        self.close_lookup = true;
    }

    /// The sides this run already has, so that a new creature joins one and not a misspelling.
    fn sides_to_join(&self) -> Vec<String> {
        let mut sides = vec![DEFAULT_SIDE.to_owned()];
        for participant in &self.state.participants {
            if !sides.contains(&participant.side) {
                sides.push(participant.side.clone());
            }
        }
        sides
    }

    pub(super) fn add_creature_form(&mut self, ui: &mut egui::Ui) {
        let c = colors();
        let known = self.creatures.clone().unwrap_or_default();
        if let Some(picked) = self.ad_hoc.picked.clone() {
            ui.horizontal(|ui| {
                ui.label(Type::Strong.rich(ui, &picked.entry.name).color(c.text));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if Action::quiet("Change")
                        .name("Choose another creature")
                        .density(Density::Read)
                        .show(ui)
                        .clicked()
                    {
                        self.ad_hoc = AdHoc::default();
                    }
                });
            });
            ui.add(egui::Label::new(Type::Small.rich(ui, &picked.detail).color(c.muted)).wrap());
        } else {
            let width = ui.available_width() - design::M;
            design::search_field(
                ui,
                FIELD,
                &mut self.ad_hoc.query,
                "Find or name a creature…",
                width,
            );
            let typed = self.ad_hoc.query.trim().to_lowercase();
            if typed.is_empty() {
                return;
            }
            let matches = matches_for(&known, &typed);
            for creature in matches.iter().take(MATCHES_SHOWN) {
                // The whole name typed is already the one the button adds.
                let row = design::row(
                    ui,
                    Density::Scan.row() + design::S,
                    creature.lower == typed,
                    &creature.entry.name,
                    |_, _| {},
                );
                ui.scope_builder(
                    egui::UiBuilder::new()
                        .accessibility_parent(row.id)
                        .max_rect(row.rect.shrink2(egui::vec2(design::S, design::XS))),
                    |ui| {
                        ui.spacing_mut().item_spacing.y = 0.0;
                        ui.label(Type::Body.rich(ui, &creature.entry.name).color(c.text));
                        let parts: Vec<_> = creature.detail.split(" · ").collect();
                        let source = parts.last().copied().filter(|part| {
                            !["CR ", "HP ", "AC "]
                                .iter()
                                .any(|prefix| part.starts_with(prefix))
                        });
                        crate::encounters::detail_columns(ui, &creature.detail, source);
                    },
                );
                design::named(ui, &row, &format!("Choose {}", creature.entry.name));
                if row.clicked() {
                    self.ad_hoc.picked = Some((*creature).clone());
                }
            }
            if matches.len() > MATCHES_SHOWN {
                ui.label(
                    Type::Small
                        .rich(
                            ui,
                            format!("{} more. Keep typing.", matches.len() - MATCHES_SHOWN),
                        )
                        .color(c.muted),
                );
            }
        }
        // A base value is the hint, so that a blank field reads as that value.
        let creature = self.ad_hoc.creature(&known).cloned();
        let base = creature
            .as_ref()
            .map(|creature| Base::of(&creature.entry, self.ad_hoc.side()));
        if creature.is_none() {
            ui.add(
                egui::Label::new(
                    Type::Small
                        .rich(ui, "Made up here. It stays in this fight.")
                        .color(c.muted),
                )
                .wrap(),
            );
        }
        let shown = |value: Option<Option<i64>>, none: &str| {
            value
                .flatten()
                .map_or_else(|| none.to_owned(), |value| value.to_string())
        };
        let hp_hint = shown(base.as_ref().map(|base| base.max_hp), "Needed");
        let ac_hint = shown(base.as_ref().map(|base| base.armor_class), "None");
        let mut enter = false;
        // Two fixed rows, so that a narrow panel never parts a word from its field.
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = design::XS;
            let form = &mut self.ad_hoc;
            enter |= short_field(
                ui,
                "Count",
                "New creature count",
                &mut form.count,
                "1",
                44.0,
            );
            ui.add_space(design::M);
            enter |= short_field(ui, "HP", "New creature HP", &mut form.hp, &hp_hint, 64.0);
            ui.add_space(design::M);
            enter |= short_field(ui, "AC", "New creature AC", &mut form.ac, &ac_hint, 52.0);
        });
        let sides = self.sides_to_join();
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = design::XS;
            enter |= short_field(
                ui,
                "Initiative",
                "New creature initiative total",
                &mut self.ad_hoc.total,
                "Roll",
                56.0,
            );
            ui.add_space(design::M);
            ui.label(Type::Small.rich(ui, "Side").color(c.muted));
            let chosen = self.ad_hoc.side().to_owned();
            let combo = egui::ComboBox::from_id_salt("ad_hoc_side")
                .icon(design::combo_icon)
                .width(110.0)
                .selected_text(design::reader::human_label(&chosen))
                .show_ui(ui, |ui| {
                    for side in &sides {
                        if ui
                            .selectable_label(*side == chosen, design::reader::human_label(side))
                            .clicked()
                        {
                            self.ad_hoc.side = Some(side.clone());
                        }
                    }
                });
            design::named(ui, &combo.response, "Side of the new creature");
        });
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
        if let Some(why) = &blocked {
            ui.add(egui::Label::new(Type::Small.rich(ui, why).color(c.muted)).wrap());
        }
        let pressed = Action::secondary(&label)
            .icon(icons::ADD)
            .density(Density::Read)
            .name(&format!("Add {name} to the initiative list"))
            .blocked(blocked.as_deref())
            .show(ui)
            .clicked();
        if (pressed || enter)
            && blocked.is_none()
            && let Ok(checked) = checked
        {
            let group = self.ad_hoc.group(checked);
            self.run_command(Command::AddCreature(group, total));
            if self.error.is_none() {
                self.ad_hoc = AdHoc::default();
                self.return_focus = self.lookup_open;
            }
        }
    }
}
