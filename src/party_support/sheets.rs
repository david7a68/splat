//! Character viewer, with combat sync on from the time Splat connects to D&D Beyond.
use crate::rolls::Chip;
use crate::{
    auth::Session,
    design::{self, Status, colors, icons},
    party_sync::{Reader, Stored},
};
use eframe::egui;
use serde_json::Value;
use splat::character::EntitySnapshot;
use splat::content::dice_text::{D20, DamagePart, RollLink};
use splat_campaign::encounter_run::{Carry, FightRow, effect::Condition};
use std::time::Duration;
#[derive(Default)]
pub struct Party {
    /// HP from the fights of the session being played, when there are any.
    pub tonight: Option<Carry>,
    pub reader: Reader,
    pub combat: crate::combat_sync::CombatSync,
    epoch: u64,
    connected: bool,
    /// The campaign that `sync_turned_off` is about.
    campaign: Option<uuid::Uuid>,
    /// A read finished with new characters, so their copies are on disk for whoever lists the party.
    pub new_copies: bool,
    /// The DM turned combat sync off here. It stays off until they turn it on.
    sync_turned_off: bool,
    /// Sync was on when the connection ended, so it stopped. The rail says so.
    pub sync_stopped: bool,
    /// Open the combat sync control when the rail links here.
    pub wants_combat_sync: bool,
}
impl Party {
    /// A read of the party is running. The rail shows a spinner beside Connected.
    pub fn busy(&self) -> bool {
        self.reader.loading()
    }

    /// Keeps every sheet current, whichever page is showing. The shell calls this every frame.
    pub fn sync(
        &mut self,
        ctx: &egui::Context,
        epoch: u64,
        campaign: Option<(uuid::Uuid, String, std::path::PathBuf)>,
        session: Option<Session>,
        visible: bool,
    ) {
        self.epoch = epoch;
        self.connected = session.is_some();
        let local = campaign.as_ref().map(|(id, _, _)| *id);
        if local != self.campaign {
            self.campaign = local;
            self.sync_turned_off = false;
        }
        if !self.connected {
            if self.combat.enabled {
                self.sync_stopped = true;
            }
            self.combat.stop();
        }
        let wake = self.connected && campaign.is_some();
        self.reader.sync(ctx, epoch, campaign, self.connected);
        self.reader.on_page = visible;
        self.reader.tick(session.clone(), ctx);
        self.tick_combat(ctx, session, visible);
        if !self.reader.loading() && std::mem::take(&mut self.reader.new_copy) {
            self.new_copies = true;
        }
        if wake {
            ctx.request_repaint_after(Duration::from_secs(1));
        }
    }

    /// Combat sync for the selected character. Away from this page the reader keeps the sheet
    /// current, so this runs only to finish what the DM changed here.
    fn tick_combat(&mut self, ctx: &egui::Context, session: Option<Session>, visible: bool) {
        self.combat.keep_all_unsent();
        let Some((id, campaign)) = self.reader.selected.clone().and_then(|id| {
            let campaign = self.reader.sheets.get(&id)?.source.campaign_id.clone();
            Some((id, campaign))
        }) else {
            self.combat.stop();
            return;
        };
        if session.is_some() && !self.combat.enabled && !self.sync_turned_off {
            self.combat.start();
        }
        if self.combat.enabled {
            self.sync_stopped = false;
        }
        if visible || self.combat.busy() || self.combat.dirty() {
            self.combat.tick(ctx, session, self.epoch, &campaign, &id);
        }
        if self.combat.updated {
            if let Some(record) = self.combat.record().cloned() {
                // Each read gets here on the frame's thread. With the same sheet, only its age
                // changes, and that is not worth a write to disk.
                let same = self
                    .reader
                    .sheets
                    .get(&id)
                    .is_some_and(|old| old.source.content_hash == record.source.content_hash);
                if !same {
                    if let Some(scope) = self.reader.local_scope() {
                        let saved = crate::hermetic::character_cache()
                            .and_then(|cache| cache.save(scope, &record))
                            .map(|_| ());
                        self.reader.stored(Stored::Sheet(id.clone()), saved);
                    }
                    self.reader.save_campaign_copy(&id, &record);
                }
                self.reader.sheets.insert(id, record);
            }
            self.combat.updated = false;
        }
    }

    /// `Sync this sheet with D&D Beyond`, ticked or cleared by the DM (the GPUI panel; egui's
    /// is `show_combat_sync`): sync stays off here until they turn it on.
    pub fn set_combat_sync(&mut self, enabled: bool) {
        if enabled != self.combat.enabled {
            self.combat.set_enabled(enabled);
            self.sync_turned_off = !enabled;
        }
    }

    /// Combat sync on this page found a conflict. The rail says so, wherever the DM is.
    pub fn sync_conflict(&self) -> bool {
        self.combat.enabled && self.combat.state() == crate::combat_sync::SyncState::Conflict
    }

    /// A read of the selected sheet, in D&D Beyond's own fields. Test builds have no route
    /// to a live one, so a visual test shows the sync panel's editable values this way.
    /// For visual tests: swap in a party's members and its read error, and get the old ones back
    /// (an empty party whose read failed, then the party as it was).
    #[cfg(any(test, feature = "test-support"))]
    pub fn swap_members_and_error(
        &mut self,
        members: Vec<crate::party_sync::Member>,
        error: Option<String>,
    ) -> (Vec<crate::party_sync::Member>, Option<String>) {
        (
            std::mem::replace(&mut self.reader.members, members),
            std::mem::replace(&mut self.reader.error, error),
        )
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn simulate_sheet_read(&mut self, max_hp: i64, sheet: &Value) {
        let id = self
            .reader
            .selected
            .as_deref()
            .and_then(|id| id.parse().ok());
        self.combat
            .simulate_sheet(id.expect("a selected character"), max_hp, sheet);
    }

    /// A change made on this page has not reached D&D Beyond.
    pub fn unconfirmed(&self) -> bool {
        self.combat.unconfirmed()
    }

    pub fn connected(&self) -> bool {
        self.connected
    }

    /// The Party page's copy of a character, for the run's side panel.
    pub fn sheet(&self, id: &str) -> Option<&EntitySnapshot> {
        self.reader.sheets.get(id)
    }

    /// Where tonight's fights left this character, if they did.
    pub fn tonight_values(&self, record: &EntitySnapshot) -> Option<(i64, i64)> {
        let carry = self.tonight.as_ref()?;
        carry
            .values
            .get(&record.definition.splat_id)
            .copied()
            .or_else(|| {
                carry
                    .full
                    .then(|| (record.stat("max_hp"), 0))
                    .and_then(|(hp, temp)| Some((hp?, temp)))
            })
    }

    /// The members' identities in a run, in the party bar's order.
    pub fn order(&self) -> Vec<uuid::Uuid> {
        self.reader
            .members
            .iter()
            .filter_map(|member| self.reader.sheets.get(&member.id))
            .map(|record| record.definition.splat_id)
            .collect()
    }
    /// The party has members to show.
    pub fn has_members(&self) -> bool {
        !self.reader.members.is_empty()
    }

    /// Each member at a glance. An open fight's values come first, then tonight's fights,
    /// then the sheet.
    pub fn glances(
        &self,
        run: &std::collections::BTreeMap<uuid::Uuid, FightRow>,
    ) -> Vec<crate::party_bar::Glance> {
        self.reader
            .members
            .iter()
            .map(|member| {
                let record = self.reader.sheets.get(&member.id);
                let in_run = record.and_then(|r| run.get(&r.definition.splat_id));
                let (hp, temp) = in_run
                    .map(|row| (row.hp, row.temporary_hp))
                    .or_else(|| {
                        let (hp, temp) = self.tonight_values(record?)?;
                        Some((Some(hp), temp))
                    })
                    .unwrap_or_else(|| {
                        (
                            record.map(|r| r.initial.hp),
                            record.map_or(0, |r| r.initial.temporary_hp),
                        )
                    });
                let stat = |key| record.and_then(|r| r.stat(key));
                crate::party_bar::Glance {
                    id: member.id.clone(),
                    name: member.name.clone(),
                    portrait: record.and_then(|record| {
                        crate::party_sync::portrait_path(self.reader.local_scope()?, record)
                    }),
                    hp,
                    max_hp: in_run.and_then(|row| row.max_hp).or(stat("max_hp")),
                    temp,
                    ac: stat("armor_class"),
                    perception: stat("passive_perception"),
                    insight: stat("passive_insight"),
                    investigation: stat("passive_investigation"),
                    spell_dc: record.and_then(spell_dc),
                    conditions: in_run.map_or_else(
                        || record.map_or_else(Vec::new, sheet_labels),
                        |row| row.conditions.clone(),
                    ),
                }
            })
            .collect()
    }

    /// Where the bar's numbers come from, as a status.
    pub fn glance_source(&self, from_run: bool) -> crate::party_bar::Source {
        let c = colors();
        if from_run {
            return crate::party_bar::Source {
                icon: icons::RUN,
                word: "From the open fight".to_owned(),
                short: "This fight",
                color: c.muted,
            };
        }
        if let Some(carry) = self
            .tonight
            .as_ref()
            .filter(|carry| carry.full || !carry.values.is_empty())
        {
            return crate::party_bar::Source {
                icon: if carry.values.is_empty() {
                    icons::LONG_REST
                } else {
                    icons::ENCOUNTERS
                },
                word: format!("After {}", carry.from),
                short: if carry.values.is_empty() {
                    "Long rest"
                } else {
                    "Last fight"
                },
                color: c.muted,
            };
        }
        match self
            .reader
            .sheets
            .values()
            .min_by_key(|record| record.source.fetched_at)
        {
            Some(oldest) => {
                let fresh = freshness(oldest, self.connected);
                crate::party_bar::Source {
                    icon: fresh.icon,
                    word: format!("Sheets fetched {}", record_age(oldest)),
                    short: "Sheets",
                    color: fresh.color,
                }
            }
            None => crate::party_bar::Source {
                icon: icons::WARN,
                word: "No sheets yet".to_owned(),
                short: "No sheets",
                color: c.muted,
            },
        }
    }

    /// Open this member's sheet the next time the page shows.
    pub fn choose(&mut self, id: String) {
        self.reader.select(id);
    }
}

/// What the row and side panel know now. A run's values outrank a stored sheet's.
pub struct LiveValues {
    pub hp: Option<i64>,
    pub max_hp: Option<i64>,
    pub temporary_hp: i64,
    pub conditions: Vec<Condition>,
    pub death_saves: splat::character::DeathSaves,
    /// The run already drew its editable block above this page.
    pub death_saves_shown: bool,
}

/// Sync stops only when the connection ends, and starts again when it comes back.
const SYNC_STOPPED: design::outcome::Sentence = crate::sentence!(
    "Combat sync stopped when the connection to D&D Beyond ended. Changes not yet sent did not reach D&D Beyond. Connect in Settings, and sync starts again."
);

/// Combat sync stopped. It waits for the DM to connect, and the rail marks it on every page as a
/// warning, so every place that says it is a warning (R41). This is the one value that the
/// Party footer and the rail read.
pub struct SyncStopped;

impl SyncStopped {
    /// The tone of every place that says it.
    const TONE: design::Tone = design::Tone::Warning;

    /// The Party footer's notice, in its tone.
    pub fn notice<A>() -> design::outcome::Notice<A> {
        match Self::TONE {
            design::Tone::Warning => design::outcome::Notice::warning(SYNC_STOPPED),
            _ => design::outcome::Notice::new(SYNC_STOPPED),
        }
    }

    /// The rail's mark, a link to Party, in its tone.
    pub fn rail() -> Status<'static> {
        Status {
            icon: icons::SYNC_OFF,
            word: "Sync stopped",
            color: match Self::TONE {
                design::Tone::Warning => colors().result.warning,
                _ => colors().result.info,
            },
        }
    }
}

/// What a sheet says a character has left, for a DM who plans an encounter around it.
#[derive(Default)]
pub struct SheetNow {
    hp: i64,
    max_hp: i64,
    temporary_hp: i64,
    armor_class: Option<i64>,
    conditions: Vec<String>,
    /// By level. Ordinary and pact slots each have a line.
    slots: Vec<SlotsLeft>,
}
struct SlotsLeft {
    level: i64,
    left: i64,
    max: i64,
}
impl SheetNow {
    pub fn of(record: &EntitySnapshot) -> Self {
        let mut slots: Vec<_> = record
            .definition
            .splat_resources
            .iter()
            .filter(|(_, pool)| pool.kind.as_deref() == Some("spell_slot"))
            .map(|(key, pool)| SlotsLeft {
                level: pool.level.unwrap_or(0),
                left: *record.initial.resources.get(key).unwrap_or(&pool.max),
                max: pool.max,
            })
            .collect();
        slots.sort_unstable_by_key(|slot| (slot.level, slot.left, slot.max));
        Self {
            hp: record.initial.hp,
            max_hp: record.stat("max_hp").unwrap_or(record.initial.hp),
            temporary_hp: record.initial.temporary_hp,
            armor_class: record.stat("armor_class"),
            conditions: sheet_conditions(record),
            slots,
        }
    }
    /// One row of the roster. A word goes with the number, so that it is not only a number.
    pub fn line(&self) -> String {
        let mut parts = vec![format!(
            "HP {}",
            crate::combat_sync::fraction(self.hp, self.max_hp)
        )];
        match design::hp_state(Some(self.hp), Some(self.max_hp)) {
            design::HpState::Zero => parts.push("0 HP".to_owned()),
            design::HpState::Bloodied => parts.push("Bloodied".to_owned()),
            _ => {}
        }
        if self.temporary_hp > 0 {
            parts.push(format!("Temp {}", self.temporary_hp));
        }
        if let Some(ac) = self.armor_class {
            parts.push(format!("AC {ac}"));
        }
        parts.extend(self.conditions.iter().cloned());
        if !self.slots.is_empty() {
            let slots: Vec<_> = self
                .slots
                .iter()
                .map(|slot| {
                    format!(
                        "L{} {}",
                        slot.level,
                        crate::combat_sync::fraction(slot.left, slot.max)
                    )
                })
                .collect();
            parts.push(format!("Slots {}", slots.join(", ")));
        }
        parts.join(" · ")
    }
}

/// The party together, as the roster has it. With no sheet to go on, there is no line.
pub fn party_line(sheets: &[&EntitySnapshot], connected: bool) -> Option<String> {
    let oldest = sheets
        .iter()
        .min_by_key(|record| record.source.fetched_at)?;
    let now: Vec<SheetNow> = sheets.iter().map(|record| SheetNow::of(record)).collect();
    let mut parts = vec![format!(
        "Party now: HP {}",
        crate::combat_sync::fraction(
            now.iter().map(|sheet| sheet.hp).sum::<i64>(),
            now.iter().map(|sheet| sheet.max_hp).sum::<i64>(),
        ),
    )];
    let slots = now.iter().flat_map(|sheet| &sheet.slots);
    let (left, max) = slots.fold((0, 0), |(left, max), slot| {
        (left + slot.left, max + slot.max)
    });
    if max > 0 {
        parts.push(format!("Slots {}", crate::combat_sync::fraction(left, max)));
    }
    parts.push(freshness(oldest, connected).word);
    Some(parts.join(" · "))
}

/// How old a sheet is, as a status with words.
pub struct Freshness {
    pub word: String,
    pub icon: &'static str,
    pub color: egui::Color32,
}

pub fn freshness(record: &EntitySnapshot, connected: bool) -> Freshness {
    let age = record_age(record);
    let c = colors().connection;
    if connected {
        Freshness {
            word: format!("Connected · Fetched {age}"),
            icon: icons::ONLINE,
            color: c.connected,
        }
    } else {
        Freshness {
            word: format!("Not connected · Fetched {age}"),
            icon: icons::OFFLINE,
            color: c.offline,
        }
    }
}

pub fn record_age(record: &EntitySnapshot) -> String {
    fetch_age(crate::hermetic::now().saturating_sub(record.source.fetched_at))
}

pub const SHEET_PORTRAIT: f32 = 56.0;

/// The conditions a sheet names, as a run's entries: labels, with no count.
pub fn sheet_labels(record: &EntitySnapshot) -> Vec<Condition> {
    sheet_conditions(record)
        .into_iter()
        .map(Condition::Label)
        .collect()
}

/// The conditions a sheet names.
pub fn sheet_conditions(record: &EntitySnapshot) -> Vec<String> {
    record
        .initial
        .conditions
        .iter()
        .filter_map(|condition| condition["name"].as_str().map(str::to_owned))
        .collect()
}

/// The highest spell save DC on a sheet, for a caster.
pub fn spell_dc(record: &EntitySnapshot) -> Option<i64> {
    array(&record.character.casting["saveDcs"])
        .iter()
        .filter_map(|value| value["value"].as_i64())
        .max()
}

pub fn has_spells(record: &EntitySnapshot) -> bool {
    let d = &record.source.raw;
    d["spells"]
        .as_object()
        .into_iter()
        .flat_map(|o| o.values())
        .any(|v| !array(v).is_empty())
        || array(&d["classSpells"])
            .iter()
            .any(|class| !array(&class["spells"]).is_empty())
}

/// The sheet page with background, details, characteristics, notes and companions.
pub const BACKGROUND_PAGE: usize = 5;
/// The sheet's pages, for Party and the run's side panel. Each index is its place in this list.
pub const SHEET_PAGES: [(usize, &str); 6] = [
    (0, "Overview"),
    (1, "Actions"),
    (2, "Spells"),
    (3, "Equipment"),
    (4, "Features"),
    (BACKGROUND_PAGE, "Background"),
];

/// Compare the sheet with the SRD rules. D&D Beyond stays the source of truth.
pub fn fetch_age(seconds: u64) -> String {
    match seconds {
        0..60 => "just now".into(),
        60..3600 => format!("{} min ago", seconds / 60),
        3600..86400 => format!("{} h ago", seconds / 3600),
        86400..172800 => "1 day ago".into(),
        _ => format!("{} days ago", seconds / 86400),
    }
}
pub fn text(v: &Value) -> &str {
    v.as_str().unwrap_or("—")
}
pub fn array(v: &Value) -> &[Value] {
    v.as_array().map(Vec::as_slice).unwrap_or(&[])
}

/// `DEX` or `dexterity` as the book names it.
fn ability_name(short: &str) -> Option<&'static str> {
    splat::content::dice_text::ABILITIES
        .into_iter()
        .find(|name| short.len() >= 3 && name.to_lowercase().starts_with(&short.to_lowercase()))
}

pub fn val(record: &EntitySnapshot, key: &str) -> String {
    record
        .stat(key)
        .map(|n| n.to_string())
        .unwrap_or("—".into())
}
pub fn signed(record: &EntitySnapshot, key: &str) -> String {
    record.stat(key).map(design::signed).unwrap_or("—".into())
}
pub fn skill_order(record: &EntitySnapshot) -> Vec<usize> {
    let mut order: Vec<_> = (0..record.character.skills.len()).collect();
    order.sort_by(|&a, &b| {
        record.character.skills[a]
            .name
            .cmp(&record.character.skills[b].name)
    });
    order
}
pub fn overview_death_saves(record: &EntitySnapshot, live: Option<&LiveValues>) -> Option<String> {
    if live.is_some_and(|values| values.death_saves_shown) {
        return None;
    }
    let saves = live.map_or(&record.initial.death_saves, |values| &values.death_saves);
    Some(format!(
        "{} {} · {} {}",
        saves.successes,
        if saves.successes == 1 {
            "success"
        } else {
            "successes"
        },
        saves.failures,
        if saves.failures == 1 {
            "failure"
        } else {
            "failures"
        }
    ))
}

pub type SheetEntry = (String, String, String, Option<u64>);

pub fn sheet_entries(record: &EntitySnapshot, page: usize) -> Vec<SheetEntry> {
    let d = &record.source.raw;
    let mut entries: Vec<(String, String, String, Option<u64>)> = Vec::new();
    let mut add = |v: &Value, source: &str| {
        let def = if v["definition"].is_object() {
            &v["definition"]
        } else {
            v
        };
        let Some(name) = def["name"].as_str() else {
            return;
        };
        let desc = def["description"]
            .as_str()
            .or_else(|| def["snippet"].as_str())
            .unwrap_or("No description supplied.");
        let source = match source {
            "race" => "Species",
            "class" => "Class",
            "feat" => "Feat",
            other => other,
        };
        let meta = match page {
            3 => format!(
                "Qty {} · {}{}",
                v["quantity"].as_u64().unwrap_or(1),
                if v["equipped"] == true {
                    "Equipped"
                } else {
                    "Carried"
                },
                if v["isAttuned"] == true {
                    " · Attuned"
                } else {
                    ""
                }
            ),
            2 => {
                let mut words = Vec::new();
                if def["level"].as_u64().is_some_and(|level| level > 0)
                    && (v["prepared"] == true || v["alwaysPrepared"] == true)
                {
                    words.push("Prepared");
                }
                words.join(" · ")
            }
            _ => source.into(),
        };
        let mut extra = Vec::new();
        if let Some(dice) = def["damage"]["diceString"].as_str() {
            extra.push(format!("Printed damage: {dice}"));
        }
        if let Some(ac) = def["armorClass"].as_u64() {
            extra.push(format!("Armor value: {ac}"));
        }
        if page == 2 {
            if let Some(range) = def["range"]["rangeValue"].as_u64() {
                extra.push(if range == 0 {
                    format!(
                        "Range: {}",
                        def["range"]["origin"].as_str().unwrap_or("see description")
                    )
                } else {
                    format!("Range: {range} ft")
                });
            }
            if def["concentration"] == true {
                extra.push("Concentration".into());
            }
            if def["ritual"] == true {
                extra.push("Ritual".into());
            }
            if let Some(time) = def["activation"]["activationTime"].as_u64() {
                let unit = match def["activation"]["activationType"].as_u64() {
                    Some(1) => "action",
                    Some(3) => "bonus action",
                    Some(4) => "reaction",
                    Some(6) => "minute",
                    Some(7) => "hour",
                    _ => "(see description)",
                };
                extra.push(format!("Casting time: {time} {unit}"));
            }
        }
        let meta = if page == 2 && def["level"].as_u64() == Some(0) {
            String::new()
        } else if extra.is_empty() {
            meta
        } else {
            [meta, extra.join(" · ")]
                .into_iter()
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join(" · ")
        };
        entries.push((
            name.into(),
            meta,
            splat::character::description_text(desc),
            def["level"].as_u64(),
        ));
    };
    match page {
        BACKGROUND_PAGE => {}
        1 => {
            for (source, values) in d["actions"].as_object().into_iter().flat_map(|o| o.iter()) {
                for v in array(values) {
                    add(v, source);
                }
            }
            for v in array(&d["customActions"]) {
                add(v, "Custom action");
            }
        }
        2 => {
            for (source, values) in d["spells"].as_object().into_iter().flat_map(|o| o.iter()) {
                for v in array(values) {
                    add(v, source);
                }
            }
            for class in array(&d["classSpells"]) {
                for v in array(&class["spells"]) {
                    add(v, "Class spell");
                }
            }
        }
        3 => {
            for v in array(&d["inventory"]) {
                add(v, "");
            }
            for v in array(&d["customItems"]) {
                add(v, "Custom item");
            }
        }
        _ => {
            for v in array(&d["race"]["racialTraits"]) {
                add(v, "Species");
            }
            for class in array(&d["classes"]) {
                for v in array(&class["classFeatures"]) {
                    if v["definition"]["requiredLevel"].as_u64().unwrap_or(0)
                        <= class["level"].as_u64().unwrap_or(0)
                    {
                        add(v, text(&class["definition"]["name"]));
                    }
                }
            }
            for v in array(&d["feats"]) {
                add(v, "Feat");
            }
        }
    }
    if page == 4
        && let (_, Some((_, feature))) = splat::character::profile::background(d)
    {
        entries.push((feature.name, "Background".into(), feature.description, None));
    }
    entries.sort_by(|a, b| {
        if page == 2 {
            a.3.cmp(&b.3).then_with(|| a.0.cmp(&b.0))
        } else {
            a.0.cmp(&b.0)
        }
    });
    if page != 3 && page != 1 {
        entries.dedup();
    }
    entries
}

/// An attack's damage as the sheet prints it: `1d4+3`, or a fixed number.
fn attack_damage(attack: &Value) -> String {
    let value = &attack["damage"]["value"];
    if let Some(n) = value.as_i64() {
        return n.to_string();
    }
    format!(
        "{}{}",
        text(&value["diceString"]),
        value["fixedValue"]
            .as_i64()
            .filter(|n| *n != 0)
            .map(|n| format!("{n:+}"))
            .unwrap_or_default()
    )
}

/// One attack as an action: its bonus with its damage, or its save DC, or its damage alone.
fn attack_chip(attack: &Value) -> Option<Chip> {
    let name = text(&attack["name"]).to_owned();
    let kind = text(&attack["damage"]["type"]);
    let mut words = format!("{name} · {} {kind}", attack_damage(attack));
    // Where it lands, as a stat block says it, so the tile shows it as other tiles do.
    if let Some(lands) = attack_reach(attack) {
        words.push_str(&format!(". {lands}."));
    }
    let parts: Vec<DamagePart> = attack["damage"]["value"]["diceString"]
        .as_str()
        .filter(|_| attack["damage"]["value"].as_i64().is_none())
        .map(|_| DamagePart {
            dice: splat::content::dice_text::plain_dice(&attack_damage(attack)),
            kind: (!kind.is_empty()).then(|| design::reader::human_label(kind)),
            when: None,
        })
        .into_iter()
        .collect();
    let link = if let Some(bonus) = attack["toHit"].as_i64() {
        RollLink::D20 {
            what: D20::Attack,
            bonus,
            label: name,
            damage: parts,
        }
    } else if let (Some(dc), Some(ability)) = (
        attack["saveInfo"]["value"].as_i64(),
        ability_name(text(&attack["saveInfo"]["label"])),
    ) {
        RollLink::SaveDc {
            label: name,
            ability: ability.to_owned(),
            dc,
            half: false,
            damage: parts,
        }
    } else if !parts.is_empty() {
        RollLink::Damage { label: name, parts }
    } else {
        return None;
    };
    Chip::new(link, words)
}

/// Where a sheet's attack lands, in a stat block's words: `Reach 5 ft` or `Range 80/320 ft`.
/// `None` when the sheet gives no distance.
fn attack_reach(attack: &Value) -> Option<String> {
    let range = &attack["range"];
    let feet = range["value"].as_u64().filter(|feet| *feet > 0)?;
    let reach = range["label"].as_str() == Some("Reach")
        || range["origin"]
            .as_str()
            .is_some_and(|origin| origin == "Melee");
    Some(
        match range["longRangeValue"].as_u64().filter(|long| *long > feet) {
            Some(long) if !reach => format!("Range {feet}/{long} ft"),
            _ if reach => format!("Reach {feet} ft"),
            _ => format!("Range {feet} ft"),
        },
    )
}

/// A character's actions, from its sheet's attacks.
pub fn sheet_chips(record: &EntitySnapshot) -> Vec<Chip> {
    record
        .character
        .attacks
        .iter()
        .filter_map(attack_chip)
        .collect()
}

/// A UTC date and time, so two runs saved on one day can be told apart.
pub fn calendar_time(seconds: u64) -> String {
    format!(
        "{} {:02}:{:02}",
        splat_content_ui::roster::calendar_date(seconds),
        (seconds / 3600) % 24,
        (seconds / 60) % 60
    )
}

#[cfg(test)]
mod sheet_now_tests {
    use super::{EntitySnapshot, SheetNow, SlotsLeft, party_line};
    use serde_json::json;

    #[test]
    fn overview_death_saves_covers_live_stored_hidden_and_all_counts() {
        for successes in 0..=2 {
            for failures in 0..=2 {
                let expected = format!(
                    "{successes} {} · {failures} {}",
                    if successes == 1 {
                        "success"
                    } else {
                        "successes"
                    },
                    if failures == 1 { "failure" } else { "failures" }
                );
                let mut stored = wizard(0);
                stored.initial.death_saves = splat::character::DeathSaves {
                    successes,
                    failures,
                };
                assert_eq!(
                    super::overview_death_saves(&stored, None),
                    Some(expected.clone())
                );
                let mut live = super::LiveValues {
                    hp: Some(1),
                    max_hp: Some(25),
                    temporary_hp: 0,
                    conditions: vec![],
                    death_saves: stored.initial.death_saves.clone(),
                    death_saves_shown: false,
                };
                stored.initial.death_saves.successes = 3;
                stored.initial.death_saves.failures = 3;
                assert_eq!(
                    super::overview_death_saves(&stored, Some(&live)),
                    Some(expected)
                );
                live.hp = Some(0);
                live.death_saves_shown = true;
                assert_eq!(super::overview_death_saves(&stored, Some(&live)), None);
            }
        }
    }

    #[test]
    fn negative_sheet_modifier_uses_true_minus() {
        let mut record = wizard(0);
        record
            .definition
            .splat_stats
            .get_mut("charisma_modifier")
            .unwrap()
            .value = -1;
        assert_eq!(super::signed(&record, "charisma_modifier"), "−1");
    }

    #[test]
    fn overview_skills_are_alphabetical_before_filling_columns() {
        let record = wizard(0);
        let order = super::skill_order(&record);
        let names: Vec<_> = order
            .iter()
            .map(|&at| record.character.skills[at].name.as_str())
            .collect();
        assert_eq!(&names[..3], ["Acrobatics", "Animal Handling", "Arcana"]);
        assert_eq!(names.last(), Some(&"Survival"));
        assert!(names.windows(2).all(|pair| pair[0] < pair[1]));
    }

    fn wizard(fetched: u64) -> EntitySnapshot {
        wizard_with(fetched, json!({}))
    }

    /// `wizard`, with `extra`'s keys added to its computed sheet.
    fn wizard_with(fetched: u64, extra: serde_json::Value) -> EntitySnapshot {
        let abilities: Vec<_> = ["str", "dex", "con", "int", "wis", "cha"]
            .iter()
            .map(|name| json!({"name": name, "score": 14, "modifier": 2, "save": 4}))
            .collect();
        let skills: Vec<_> = [
            "Athletics", "Acrobatics", "Sleight of Hand", "Stealth", "Arcana", "History",
            "Investigation", "Nature", "Religion", "Animal Handling", "Insight", "Medicine",
            "Perception", "Survival", "Deception", "Intimidation", "Performance", "Persuasion",
        ]
        .iter()
        .map(|name| json!({"name": name, "modifier": 5, "isProficient": true, "isExpert": false, "isHalfProficient": false, "isAdvantage": false, "isDisadvantage": false}))
        .collect();
        let mut computed = json!({"characterId": 123, "name": "Example", "campaign": {"id": 456}, "level": 1, "armorClass": 19, "proficiencyBonus": 2, "initiativeBonus": 7, "passivePerception": 15, "passiveInsight": 12, "passiveInvestigation": 13, "abilities": abilities, "skills": skills, "hitPointInfo": {"maximum": 25, "current": 17, "temp": 4}, "deathSaveInfo": {"successCount": 0, "failCount": 0}, "speeds": [{"name": "Walking", "distance": 40}], "race": {"name": "Goblin"}, "classes": [], "attacks": [], "castingInfo": {"saveDcs": []}, "conditions": [{"name": "Poisoned"}], "inspiration": false});
        if let (Some(computed), Some(extra)) = (computed.as_object_mut(), extra.as_object()) {
            computed.extend(extra.clone());
        }
        EntitySnapshot::from_beyond(
            json!({"id": 123, "name": "Example", "campaign": {"id": 456}, "race": {"sizeId": 3, "creatureTypeId": 11}, "classes": [{"id": 8, "level": 1, "hitDiceUsed": 0, "definition": {"name": "Wizard", "spellRules": {"levelSpellSlots": [[], [2]]}}}], "spellSlots": [{"level": 1, "used": 1}], "notes": {"otherNotes": ""}}),
            computed,
            "456",
            "123",
            fetched,
            fetched,
        )
        .unwrap()
    }

    #[test]
    fn a_sheet_says_what_is_left_and_the_party_line_adds_it_up() {
        let record = wizard(10);
        assert_eq!(
            SheetNow::of(&record).line(),
            "HP 17/25 · Temp 4 · AC 19 · Poisoned · Slots L1 1/2"
        );
        let older = wizard(5);
        let line = party_line(&[&record, &older], false).unwrap();
        assert!(
            line.starts_with("Party now: HP 34/50 · Slots 2/4 · Not connected"),
            "{line}"
        );
        assert_eq!(party_line(&[], true), None);
    }
    #[test]
    fn one_line_says_what_a_character_has_left() {
        let sheet = SheetNow {
            hp: 9,
            max_hp: 38,
            temporary_hp: 5,
            armor_class: Some(16),
            conditions: vec!["Poisoned".into()],
            slots: vec![
                SlotsLeft {
                    level: 1,
                    left: 2,
                    max: 4,
                },
                SlotsLeft {
                    level: 2,
                    left: 0,
                    max: 3,
                },
            ],
        };
        assert_eq!(
            sheet.line(),
            "HP 9/38 · Bloodied · Temp 5 · AC 16 · Poisoned · Slots L1 2/4, L2 0/3"
        );
        let whole = SheetNow {
            hp: 38,
            max_hp: 38,
            ..SheetNow::default()
        };
        assert_eq!(whole.line(), "HP 38/38");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// iter-20-R-FD8: a Party tile says where its attack lands, as Library and run tiles do,
    /// when the sheet gives it; a sheet without a distance keeps one line.
    #[test]
    fn a_sheets_attack_tile_says_where_it_lands() {
        let attack = |range: Value| json!({"name": "Dagger", "toHit": 6, "damage": {"value": {"diceString": "1d4", "fixedValue": 3}, "type": "Piercing"}, "range": range});
        let words = |range: Value| attack_chip(&attack(range)).unwrap().text;
        assert!(words(json!({"label": "Reach", "value": 5})).ends_with(". Reach 5 ft."));
        assert!(words(json!({"origin": "Ranged", "value": 120})).ends_with(". Range 120 ft."));
        assert!(
            words(json!({"origin": "Ranged", "value": 80, "longRangeValue": 320}))
                .ends_with(". Range 80/320 ft.")
        );
        // Must stay true: no distance, no second line, and the numbers are as before.
        for none in [Value::Null, json!({"origin": "Touch", "value": 0})] {
            let text = words(none);
            assert!(!text.contains("Reach") && !text.contains("Range"), "{text}");
            assert!(text.starts_with("Dagger · "), "{text}");
        }
    }
}
