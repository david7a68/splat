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
/// How many matches Show all lists before it scrolls.
const MATCHES_LISTED: usize = 8;
const BUTTON_NAME: usize = 24;
const MAX_HP: i64 = 1_000_000;
const MAX_AC: i64 = 1000;

/// A creature of this campaign that a run can add. Linked characters are not among these.
#[derive(Debug)]
pub(crate) struct RunCreature {
    pub entry: Arc<Entry>,
    pub lower: String,
    pub detail: String,
    /// The stat block has no hit points, so `detail` shows the `HP not set` fault.
    pub hp_missing: bool,
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
    /// The words for which the DM chose Show all.
    pub(super) show_all: Option<String>,
    /// HP takes the keyboard on the next frame.
    pub(super) focus_hp: bool,
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
    matches.sort_by_key(|creature| crate::matching::creature_tier(&creature.lower, typed));
    matches
}

/// A short number field with its word before it. Returns whether Enter was pressed in it.
/// The made-up form's own reason. Its line stays when the DM types the HP.
const NEEDS_HP: &str = "Type its HP. A creature made up here has no stat block.";

/// The made-up form's first fields, `Count` and `Initiative`, share an x: their words take
/// one width, as Correct's label column does.
const FIRST_FIELD_WIDTH: f32 = 56.0;

/// A field with its word before it. A `label_width` gives the word a fixed column.
#[expect(
    clippy::too_many_arguments,
    reason = "each field differs in each of these"
)]
fn short_field(
    ui: &mut egui::Ui,
    word: &str,
    label_width: Option<f32>,
    name: &str,
    text: &mut String,
    hint: &str,
    width: f32,
    focus: bool,
) -> bool {
    let words = Type::Small.rich(ui, word).color(colors().muted);
    match label_width {
        Some(label_width) => {
            let size = egui::vec2(label_width, Density::Read.control());
            ui.allocate_ui_with_layout(
                size,
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    ui.set_min_size(size);
                    ui.label(words);
                },
            );
        }
        None => {
            ui.label(words);
        }
    }
    design::gap(ui, design::Gap::XS);
    let response = design::number_field(ui, name, text, hint, width, Density::Read.control());
    if focus {
        response.request_focus();
    }
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

    /// An NPC with no stat block joins as a creature made up at the table: the form, with its
    /// name filled in and the keyboard in HP.
    pub(crate) fn made_up(&mut self, name: &str) {
        self.ad_hoc = AdHoc {
            query: name.to_owned(),
            focus_hp: true,
            ..AdHoc::default()
        };
        self.show_arrivals();
    }

    /// FJ5 (iter-27-R), for Lookup's `Add to fight`: why `stat_block` cannot join this fight now,
    /// or None. The same reasons as Arrivals' Add, for a creature that has no typed values.
    pub(crate) fn add_to_fight_reason(&self, stat_block: &Entry) -> Option<String> {
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
    pub(crate) fn add_entry_to_fight(
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
    pub(crate) fn made_up_name(&self) -> Option<&str> {
        (self.panel == super::Panel::Groups && self.ad_hoc.focus_hp)
            .then_some(self.ad_hoc.query.as_str())
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

    /// `block` is the width of Arrivals' group block, when it has groups.
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw egui calls that a design part replaces (design::results::STAGED_ALLOWS)"
    )]
    pub(super) fn add_creature_form(&mut self, ui: &mut egui::Ui, block: Option<f32>) {
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
            // As wide as the group block below, so the panel has one right edge.
            let width = block
                .unwrap_or_else(|| (ui.available_width() - design::M).min(design::LIST_MEASURE));
            let _ = crate::field_id!("run_add_creature_search")
                .once(crate::design::Once::PageSearch)
                .search(
                    ui,
                    FIELD,
                    &mut self.ad_hoc.query,
                    "Find or name a creature…",
                    width,
                );
            let typed = self.ad_hoc.query.trim().to_lowercase();
            // Show all lasts while the words stay the same: new words start a short list again.
            if self.ad_hoc.show_all.as_deref() != Some(typed.as_str()) {
                self.ad_hoc.show_all = None;
            }
            if typed.is_empty() {
                return;
            }
            let matches = matches_for(&known, &typed);
            let all = self.ad_hoc.show_all.is_some();
            // FJ2 (iter-27-R): one match more than the short list shows as a row, not as a
            // `Show all` line of about the same height that costs a click.
            let one_more = matches.len() == MATCHES_SHOWN + 1;
            let shown = if all || one_more {
                matches.len()
            } else {
                MATCHES_SHOWN
            };
            let mut chosen = None;
            let mut rows = |ui: &mut egui::Ui| {
                for creature in matches.iter().take(shown) {
                    // The whole name typed is already the one the button adds.
                    let row = design::row(
                        ui,
                        Density::Scan.row() + design::S,
                        creature.lower == typed,
                        &creature.entry.name,
                        |_, _| {},
                    );
                    // egui does not scroll to a widget that Tab focuses. The list does, so every
                    // match can be reached from the keyboard.
                    if row.gained_focus() {
                        row.scroll_to_me(None);
                    }
                    // A long list lays out only the rows in view. Every row keeps its control and
                    // name, so focus and the accessible list stay whole. The words go in a child
                    // that does not move the cursor, and a hidden row skips the one id that child
                    // takes, so every row has the same pitch and later rows keep their ids.
                    if ui.is_rect_visible(row.rect) {
                        let mut words = ui.new_child(
                            egui::UiBuilder::new()
                                .accessibility_parent(row.id)
                                .max_rect(row.rect.shrink2(egui::vec2(design::S, design::XS))),
                        );
                        words.spacing_mut().item_spacing.y = 0.0;
                        words.label(Type::Body.rich(&words, &creature.entry.name).color(c.text));
                        let parts: Vec<_> = creature.detail.split(" · ").collect();
                        let source = parts.last().copied().filter(|part| {
                            !["CR ", "HP ", "AC "]
                                .iter()
                                .any(|prefix| part.starts_with(prefix))
                        });
                        crate::encounters::detail_columns(
                            &mut words,
                            &creature.detail,
                            source,
                            creature.hp_missing,
                        );
                    } else {
                        ui.skip_ahead_auto_ids(1);
                    }
                    design::named(ui, &row, &format!("Choose {}", creature.entry.name));
                    if row.clicked() {
                        chosen = Some(Arc::clone(creature));
                    }
                }
            };
            if all {
                // A bounded list, so the field above and the form below stay in reach.
                let step = Density::Scan.row() + design::S + ui.spacing().item_spacing.y;
                egui::ScrollArea::vertical()
                    .id_salt("run_creature_matches")
                    .max_height(MATCHES_LISTED as f32 * step)
                    .auto_shrink([false, true])
                    .show(ui, |ui| rows(ui));
            } else {
                rows(ui);
            }
            if !all && matches.len() > MATCHES_SHOWN + 1 {
                let label = format!("Show all {}", matches.len());
                // The rows put their words `S` in, as a quiet action's padding does. So the plain
                // action already starts its words at the rows' words, and its target at their edge.
                if Action::quiet(&label)
                    .density(Density::Read)
                    .hint("List every creature that matches, without more typing")
                    .show(ui)
                    .clicked()
                {
                    self.ad_hoc.show_all = Some(typed.clone());
                }
            }
            if chosen.is_some() {
                self.ad_hoc.picked = chosen;
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
        // Two fixed rows, so that a narrow panel never parts a word from its field. Their
        // first words share one column, so `Count` and `Initiative` fields start at one x and
        // `Side` starts where `HP` does.
        let first_words = ["Count", "Initiative"]
            .iter()
            .map(|word| design::number_width(ui, word, &Type::Small.font(ui)))
            .fold(0.0, f32::max);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = design::XS;
            let form = &mut self.ad_hoc;
            enter |= short_field(
                ui,
                "Count",
                Some(first_words),
                "New creature count",
                &mut form.count,
                "1",
                FIRST_FIELD_WIDTH,
                false,
            );
            design::gap(ui, design::Gap::M);
            enter |= short_field(
                ui,
                "HP",
                None,
                "New creature HP",
                &mut form.hp,
                &hp_hint,
                64.0,
                std::mem::take(&mut form.focus_hp),
            );
            design::gap(ui, design::Gap::M);
            enter |= short_field(
                ui,
                "AC",
                None,
                "New creature AC",
                &mut form.ac,
                &ac_hint,
                52.0,
                false,
            );
        });
        let sides = self.sides_to_join();
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = design::XS;
            enter |= short_field(
                ui,
                "Initiative",
                Some(first_words),
                "New creature initiative total",
                &mut self.ad_hoc.total,
                "Roll",
                FIRST_FIELD_WIDTH,
                false,
            );
            design::gap(ui, design::Gap::M);
            ui.label(Type::Small.rich(ui, "Side").color(c.muted));
            let chosen = self.ad_hoc.side().to_owned();
            let combo = egui::ComboBox::from_id_salt("ad_hoc_side")
                .icon(design::combo_icon)
                .width(110.0)
                .selected_text(design::reader::human_label(&chosen))
                .show_ui(ui, |ui| {
                    for side in &sides {
                        if design::extras::choice_wrapped(
                            ui,
                            *side == chosen,
                            &design::reader::human_label(side),
                            None,
                        )
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
        // A reason that comes and goes keeps its line, as tall as the made-up form's own
        // reason, so Add and the groups below stay still when the DM types the HP.
        let width = ui.available_width();
        let reserved = egui::WidgetText::from(Type::Small.rich(ui, NEEDS_HP))
            .into_galley(
                ui,
                Some(egui::TextWrapMode::Wrap),
                width,
                egui::TextStyle::Body,
            )
            .size()
            .y;
        ui.allocate_ui_with_layout(
            egui::vec2(width, reserved),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                ui.set_min_size(egui::vec2(0.0, reserved));
                if let Some(why) = &blocked {
                    ui.add(egui::Label::new(Type::Small.rich(ui, why).color(c.muted)).wrap());
                }
            },
        );
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
