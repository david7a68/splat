//! Conflict-aware combat sheet synchronization with D&D Beyond.
//! Writes use source identities and absolute values, followed by a raw read-back.
use crate::auth::Session;
use eframe::egui;
use reqwest::{Method, blocking::Client, redirect::Policy};
use serde_json::{Value, json};
use splat::character::EntitySnapshot;
use std::{
    collections::{BTreeMap, BTreeSet},
    io::Read,
    sync::{
        Arc, Mutex, OnceLock,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    time::{Duration, Instant},
};

const BASE: &str = "https://character-service.dndbeyond.com/character/v5";
const INTERVAL: Duration = Duration::from_secs(15);
/// From the last change, or from leaving a number field, to the exchange that sends it.
const DELAY: Duration = Duration::from_millis(500);
pub const CONDITIONS: [(u64, &str); 15] = [
    (1, "Blinded"),
    (2, "Charmed"),
    (3, "Deafened"),
    (4, "Exhaustion"),
    (5, "Frightened"),
    (6, "Grappled"),
    (7, "Incapacitated"),
    (8, "Invisible"),
    (9, "Paralyzed"),
    (10, "Petrified"),
    (11, "Poisoned"),
    (12, "Prone"),
    (13, "Restrained"),
    (14, "Stunned"),
    (15, "Unconscious"),
];
type Groups = BTreeMap<String, Value>;
#[derive(Clone)]
struct Remote {
    id: u64,
    max_hp: i64,
    can_edit: bool,
    groups: Groups,
    labels: BTreeMap<String, String>,
    record: Option<EntitySnapshot>,
}
fn integer(v: &Value, field: &str) -> Result<i64, String> {
    v.as_i64()
        .filter(|n| *n >= 0)
        .ok_or_else(|| format!("D&D Beyond returned invalid {field}."))
}
impl Remote {
    fn from_raw(id: u64, max_hp: i64, raw: &Value) -> Result<Self, String> {
        if raw["id"].as_u64() != Some(id) || max_hp <= 0 {
            return Err("Character identity or maximum HP is invalid.".into());
        }
        let mut groups = Groups::new();
        let counter = |v: &Value, field: &str| {
            if v.is_null() {
                Ok(0)
            } else {
                integer(v, field)
            }
        };
        if let (Some(removed), Some(temp)) =
            (raw.get("removedHitPoints"), raw.get("temporaryHitPoints"))
        {
            groups.insert("hp".into(), json!({"removedHitPoints":counter(removed,"HP")?,"temporaryHitPoints":counter(temp,"temporary HP")?}));
        }
        if let Some(death) = raw.get("deathSaves") {
            let (success, fail) = if death.is_null() {
                (0, 0)
            } else {
                (
                    counter(
                        death
                            .get("successCount")
                            .ok_or("Missing death save successes.")?,
                        "death saves",
                    )?,
                    counter(
                        death
                            .get("failCount")
                            .ok_or("Missing death save failures.")?,
                        "death saves",
                    )?,
                )
            };
            if success > 3 || fail > 3 {
                return Err("Invalid death save counts.".into());
            }
            groups.insert(
                "death".into(),
                json!({"successCount":success,"failCount":fail}),
            );
        }
        if let Some(conditions) = raw["conditions"].as_array() {
            let mut values = serde_json::Map::new();
            for c in conditions {
                let key = c["id"]
                    .as_u64()
                    .ok_or("A condition has no source ID.")?
                    .to_string();
                if values
                    .insert(key, c.get("level").cloned().unwrap_or(Value::Null))
                    .is_some()
                {
                    return Err("Duplicate condition ID.".into());
                }
            }
            groups.insert("conditions".into(), values.into());
        }
        for (field, group, levels) in [("spellSlots", "slots", 9), ("pactMagic", "pact", 5)] {
            if let Some(slots) = raw[field].as_array() {
                let mut values = serde_json::Map::new();
                for level in 1..=levels {
                    values.insert(format!("level{level}"), 0.into());
                }
                let mut seen = BTreeSet::new();
                for slot in slots {
                    let level = slot["level"].as_u64().ok_or("Invalid spell slot level.")?;
                    if !(1..=levels).contains(&level) || !seen.insert(level) {
                        return Err("Invalid or duplicate slot level.".into());
                    }
                    values.insert(
                        format!("level{level}"),
                        integer(&slot["used"], "slots used")?.into(),
                    );
                }
                groups.insert(group.into(), values.into());
            }
        }
        let mut labels = BTreeMap::new();
        fn resources(
            value: &Value,
            groups: &mut Groups,
            labels: &mut BTreeMap<String, String>,
        ) -> Result<(), String> {
            match value {
                Value::Array(items) => {
                    for item in items {
                        resources(item, groups, labels)?;
                    }
                }
                Value::Object(map) => {
                    if let (Some(id), Some(kind), Some(used)) = (
                        map.get("id").and_then(Value::as_u64),
                        map.get("entityTypeId").and_then(Value::as_u64),
                        map.get("limitedUse").and_then(|v| v.get("numberUsed")),
                    ) {
                        let key = format!("limited/{kind}/{id}");
                        let used = integer(used, "limited uses")?;
                        let group = json!({"id":id,"entityTypeId":kind,"uses":used});
                        if groups.get(&key).is_some_and(|old| old != &group) {
                            return Err("Conflicting resource identities.".into());
                        }
                        groups.insert(key.clone(), group);
                        labels.insert(
                            key,
                            value["name"]
                                .as_str()
                                .or_else(|| value["definition"]["name"].as_str())
                                .unwrap_or("Limited-use resource")
                                .into(),
                        );
                    }
                    for (key, child) in map {
                        if key != "definition" && key != "limitedUse" {
                            resources(child, groups, labels)?;
                        }
                    }
                }
                _ => {}
            }
            Ok(())
        }
        for source in ["actions", "spells", "classSpells"] {
            resources(&raw[source], &mut groups, &mut labels)?;
        }
        Ok(Self {
            id,
            max_hp,
            can_edit: raw["canEdit"] == true,
            groups,
            labels,
            record: None,
        })
    }
}
#[derive(Clone, Debug)]
struct Mutation {
    method: Method,
    route: String,
    body: Value,
    expected: Value,
}
fn mutations(remote: &Remote, key: &str, target: &Value) -> Result<Vec<Mutation>, String> {
    let current = remote
        .groups
        .get(key)
        .ok_or("This combat stat is unavailable on the sheet.")?;
    let mut body = target
        .as_object()
        .cloned()
        .ok_or("Invalid combat values.")?;
    let route = match key {
        "hp" => "/life/hp/damage-taken",
        "death" => "/life/death-saves",
        "slots" => "/spell/slots",
        "pact" => "/spell/pact-magic",
        key if key.starts_with("limited/") => "/action/limited-use",
        "conditions" => "/condition",
        _ => return Err("Unsupported combat field.".into()),
    };
    if key == "conditions" {
        let old = current.as_object().ok_or("Invalid conditions.")?;
        let mut expected = old.clone();
        let mut changes = Vec::new();
        for id in old.keys().filter(|id| !body.contains_key(*id)) {
            let parsed = id.parse::<u64>().map_err(|_| "Invalid condition ID.")?;
            if !CONDITIONS.iter().any(|(i, _)| *i == parsed) {
                return Err("Only recognized conditions can be edited.".into());
            }
            expected.remove(id);
            changes.push(Mutation {
                method: Method::DELETE,
                route: route.into(),
                body: json!({"characterId":remote.id,"id":parsed}),
                expected: expected.clone().into(),
            });
        }
        for (id, level) in &body {
            if old.get(id) == Some(level) {
                continue;
            }
            let parsed = id.parse::<u64>().map_err(|_| "Invalid condition ID.")?;
            if !CONDITIONS.iter().any(|(i, _)| *i == parsed)
                || (parsed == 4 && !level.as_u64().is_some_and(|n| (1..=6).contains(&n)))
                || (parsed != 4 && !level.is_null())
            {
                return Err("Invalid condition or Exhaustion level.".into());
            }
            expected.insert(id.clone(), level.clone());
            changes.push(Mutation {method:Method::PUT,route:route.into(),body:json!({"characterId":remote.id,"id":parsed,"level":level,"totalHp":remote.max_hp}),expected:expected.clone().into()});
        }
        return Ok(changes);
    }
    // Only accept the exact group shape supplied by the provider.
    let old = current.as_object().ok_or("Invalid baseline.")?;
    if body.keys().collect::<Vec<_>>() != old.keys().collect::<Vec<_>>() {
        return Err("Combat stat fields do not match the sheet.".into());
    }
    for (field, value) in &body {
        let n = integer(value, field)?;
        if key == "death" && n > 3 {
            return Err("Death save counts must be zero to three.".into());
        }
        if key == "hp"
            && field == "removedHitPoints"
            && n > remote.max_hp
            && old.get(field) != Some(value)
        {
            return Err("Current HP must be between zero and maximum HP.".into());
        }
        if key.starts_with("limited/")
            && ["id", "entityTypeId"].contains(&field.as_str())
            && old.get(field) != Some(value)
        {
            return Err("Resource identity cannot change.".into());
        }
    }
    body.insert("characterId".into(), remote.id.into());
    Ok(vec![Mutation {
        method: Method::PUT,
        route: route.into(),
        body: body.into(),
        expected: target.clone(),
    }])
}
trait Transport {
    fn read(&mut self) -> Result<Remote, String>;
    fn write(&mut self, mutation: &Mutation) -> Result<(), String>;
}
struct Live {
    session: Session,
    client: Client,
    id: String,
    campaign: String,
    active: Arc<AtomicBool>,
}
impl Transport for Live {
    fn read(&mut self) -> Result<Remote, String> {
        if !self.active.load(Ordering::SeqCst) {
            return Err("Sync stopped.".into());
        }
        let mut record =
            crate::party_sync::fetch(&self.client, &self.session, &self.id, &self.campaign)
                .map_err(|e| e.message)?;
        let mut remote = Remote::from_raw(
            self.id.parse().map_err(|_| "Invalid character ID.")?,
            record.stat("max_hp").ok_or("Missing maximum HP.")?,
            &record.source.raw,
        )?;
        if let Some(hp) = remote.groups.get("hp") {
            record.initial.hp =
                (remote.max_hp - hp["removedHitPoints"].as_i64().unwrap_or(0)).max(0);
            record.initial.temporary_hp = hp["temporaryHitPoints"].as_i64().unwrap_or(0);
        }
        if let Some(death) = remote.groups.get("death") {
            record.initial.death_saves.successes = death["successCount"].as_u64().unwrap_or(0);
            record.initial.death_saves.failures = death["failCount"].as_u64().unwrap_or(0);
        }
        remote.record = Some(record);
        Ok(remote)
    }
    fn write(&mut self, mutation: &Mutation) -> Result<(), String> {
        if !self.active.load(Ordering::SeqCst) {
            return Err("Sync stopped before writing.".into());
        }
        let response = self
            .session
            .authorize(
                self.client
                    .request(mutation.method.clone(), format!("{BASE}{}", mutation.route)),
            )
            .json(&mutation.body)
            .send()
            .map_err(|_| {
                "Write response was not received. Reading the sheet again before any retry."
                    .to_owned()
            })?;
        let status = response.status().as_u16();
        if !(200..300).contains(&status) {
            return Err(match status {
                401 | 403 => {
                    "D&D Beyond denied this write. Reconnect and check character edit permission."
                }
                400 => "D&D Beyond rejected the change; the sheet may have changed elsewhere.",
                429 => "D&D Beyond is limiting writes. Refresh later before retrying.",
                _ => "D&D Beyond could not confirm the write.",
            }
            .into());
        }
        let mut bytes = Vec::new();
        response
            .take(1024 * 1024)
            .read_to_end(&mut bytes)
            .map_err(|_| "Could not read the write response.")?;
        let response: Value =
            serde_json::from_slice(&bytes).map_err(|_| "Unexpected write response.")?;
        if response["success"] != true {
            return Err("D&D Beyond did not accept the change.".into());
        }
        Ok(())
    }
}
struct Outcome {
    remote: Remote,
    blocked: BTreeMap<String, String>,
}
fn exchange(
    transport: &mut impl Transport,
    baseline: Option<&Remote>,
    desired: &Groups,
    blocked: &BTreeSet<String>,
    read_only: bool,
) -> Result<Outcome, String> {
    let mut remote = transport.read()?;
    let mut problems = BTreeMap::new();
    if !read_only && let Some(baseline) = baseline {
        for (key, target) in desired {
            if baseline.groups.get(key) == Some(target)
                || remote.groups.get(key) == Some(target)
                || blocked.contains(key)
            {
                continue;
            }
            if remote.groups.get(key) != baseline.groups.get(key)
                || (key == "hp" && baseline.max_hp != remote.max_hp)
            {
                problems.insert(
                    key.clone(),
                    "Changed on D&D Beyond since your edit. Choose which values to keep.".into(),
                );
                continue;
            }
            if !remote.can_edit {
                problems.insert(
                    key.clone(),
                    "This session cannot edit the character (canEdit is false).".into(),
                );
                continue;
            }
            let changes = match mutations(&remote, key, target) {
                Ok(v) => v,
                Err(e) => {
                    problems.insert(key.clone(), e);
                    continue;
                }
            };
            for change in changes {
                let sent = transport.write(&change);
                remote = transport.read()?;
                if remote.groups.get(key) != Some(&change.expected) {
                    problems.insert(key.clone(),sent.err().unwrap_or_else(||"The read-back did not confirm this change. Review the current sheet before retrying.".into()));
                    break;
                }
            }
        }
    }
    Ok(Outcome {
        remote,
        blocked: problems,
    })
}
struct Pending {
    receiver: mpsc::Receiver<Result<Outcome, String>>,
    sent: Groups,
}
/// What one exchange starts from. `tick` gives it to a worker, which answers on `reply`.
struct Job {
    baseline: Option<Remote>,
    desired: Groups,
    blocked: BTreeSet<String>,
    read_only: bool,
    reply: mpsc::Sender<Result<Outcome, String>>,
}
/// The number field the DM is in, or is dragging. While it lasts, no exchange starts for this
/// character and none is applied, so a part of a number cannot reach the sheet. It has no time
/// limit, as a limit would send that part. egui ends a focus when the field is not drawn or
/// is off, so this ends on the next frame after any way of leaving. It is never saved.
struct NumberField {
    id: egui::Id,
    /// The group and its value when the DM came in. Escape puts it back. A field outside this
    /// sync's panel has none: its value is the run's, and the run takes it back.
    entered: Option<(String, Value)>,
}
/// Escape this frame. A handler can consume the key first, and the raw events still have it.
pub fn escape_pressed(ctx: &egui::Context) -> bool {
    ctx.input(|i| {
        i.raw.events.iter().any(|event| {
            matches!(
                event,
                egui::Event::Key {
                    key: egui::Key::Escape,
                    pressed: true,
                    ..
                }
            )
        })
    })
}
/// The sync states the brief names. Each has a word and an icon on the participant row.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SyncState {
    Off,
    FirstRead,
    Matching,
    Pending,
    Conflict,
    ReadOnly,
    Error,
    /// The session ended under a running sync. It needs a connection and `Sync party`.
    Stopped,
    /// Sync is off, and this row has a change it has not sent.
    NotSent,
}

const SESSION_LOST: &str = "The D&D Beyond session ended, so this sync stopped. Changes since then are kept in Splat and have not been sent. Connect in Settings, then select Sync party. Splat reads the sheet first, and asks before it replaces a value you changed.";
/// Groups the run holds as its own values, so a saved run keeps them.
const KEPT_BY_THE_RUN: [&str; 3] = ["hp", "death", "conditions"];
/// In place of a conflict's message while its sync is stopped: neither choice can run, and
/// what starts this row's sync again depends on the case.
pub const STOPPED_CONNECT_FIRST: &str = "Sync is stopped, so neither choice can run. Connect in Settings, then select Sync party. It reads the sheet and asks again.";
pub const STOPPED_STOP_THEN_SYNC: &str = "Sync is stopped for this row, so neither choice can run. Select Stop sync, then Sync party. It reads the sheet and asks again.";
pub const STOPPED_SYNC_AGAIN: &str = "Sync is stopped, so neither choice can run. Select Sync party. It reads the sheet and asks again.";
const DIFFER_AFTER_READ: &str =
    "Splat and D&D Beyond differ, and you changed this while sync was off. Choose which to keep.";

/// A sheet with this damage and nothing else recorded. Test builds cannot read a live one.
#[cfg(test)]
fn simulated_sheet(character: u64, max_hp: i64, damage: i64, can_edit: bool) -> Remote {
    Remote::from_raw(
        character,
        max_hp,
        &json!({"id":character,"canEdit":can_edit,"removedHitPoints":damage,"temporaryHitPoints":0,"deathSaves":{"successCount":0,"failCount":0},"conditions":[]}),
    )
    .expect("simulated sheet")
}

#[derive(Default)]
pub struct CombatSync {
    pub enabled: bool,
    scope: Option<(u64, String, String)>,
    remote: Option<Remote>,
    desired: Groups,
    blocked: BTreeMap<String, String>,
    pending: Option<Pending>,
    gate: Option<Arc<AtomicBool>>,
    next: Option<Instant>,
    changed: Option<Instant>,
    refresh_only: bool,
    pub error: Option<String>,
    pub updated: bool,
    session_lost: bool,
    /// The last sheet values read before this sync was started again. What the run changed
    /// since then is the DM's own, and the next first read must not replace it unasked.
    matched: Option<Groups>,
    /// Unsent values the run does not hold itself, such as spell slots. They last until the
    /// next first read, where a difference asks. They are not in a saved run.
    unsent: Groups,
    /// A first read arrived, and the run has not compared it with its values yet.
    first_read: bool,
    field: Option<NumberField>,
    /// Sync stopped while the DM was in a panel field. Their number has not reached the run.
    typed_at_stop: bool,
    /// On Party, no run keeps these values, so all that have not gone are this sync's own.
    no_run: bool,
    /// Never talks to D&D Beyond. It only shows the state it was given.
    #[cfg(test)]
    simulated: bool,
}
impl Drop for CombatSync {
    fn drop(&mut self) {
        if let Some(gate) = &self.gate {
            gate.store(false, Ordering::SeqCst);
        }
    }
}
impl CombatSync {
    /// A sync in a state test builds cannot read from a live sheet. It only shows that.
    #[cfg(test)]
    pub fn simulated_state(state: SyncState, character: u64, max_hp: i64, damage: i64) -> Self {
        if state == SyncState::Conflict {
            return Self::simulated_conflict(character, max_hp, damage);
        }
        let mut sync = Self::default();
        sync.simulated = true;
        sync.enabled = state != SyncState::Off;
        if matches!(state, SyncState::FirstRead | SyncState::Off) {
            return sync;
        }
        let read = simulated_sheet(character, max_hp, damage, state != SyncState::ReadOnly);
        sync.desired = read.groups.clone();
        sync.remote = Some(read);
        if state == SyncState::Pending {
            sync.desired.get_mut("hp").expect("HP group")["removedHitPoints"] = (damage + 1).into();
        }
        if state == SyncState::Error {
            sync.error = Some("Could not read this character from D&D Beyond.".into());
        }
        sync
    }

    /// The state after a refresh finds that D&D Beyond and Splat changed the same HP.
    /// Test builds cannot read a live sheet, so a visual test shows the panel this way.
    #[cfg(test)]
    pub fn simulated_conflict(character: u64, max_hp: i64, damage: i64) -> Self {
        let read = simulated_sheet(character, max_hp, damage, true);
        let mut sync = Self::default();
        sync.enabled = true;
        sync.simulated = true;
        sync.remote = Some(read.clone());
        sync.desired = read.groups.clone();
        sync.desired.get_mut("hp").expect("HP group")["removedHitPoints"] = (damage + 4).into();
        let mut changed = read;
        changed.groups.get_mut("hp").expect("HP group")["removedHitPoints"] = (damage + 9).into();
        sync.accept_read(changed);
        sync
    }

    /// A sync that has read this sheet, in a state a live read would bring it to. `local` is
    /// the run's own HP after any change of the DM's, which the sheet has not seen.
    #[cfg(test)]
    pub fn simulated_from(
        state: SyncState,
        character: u64,
        max_hp: i64,
        sheet: &Value,
        local: Value,
    ) -> Self {
        let mut sync = Self::default();
        sync.simulated = true;
        sync.enabled = state != SyncState::Off;
        if matches!(state, SyncState::FirstRead | SyncState::Off) {
            return sync;
        }
        let read = Remote::from_raw(character, max_hp, sheet).expect("simulated sheet");
        sync.desired = read.groups.clone();
        sync.remote = Some(read.clone());
        match state {
            SyncState::Pending => {
                sync.desired.insert("hp".into(), local);
            }
            SyncState::Error => {
                sync.error = Some("Could not read this character from D&D Beyond.".into());
            }
            SyncState::Conflict => {
                sync.desired.insert("hp".into(), local);
                let mut changed = read;
                let hp = changed.groups.get_mut("hp").expect("HP group");
                let damage = hp["removedHitPoints"].as_i64().unwrap_or(0);
                hp["removedHitPoints"] = (damage + 9).into();
                sync.accept_read(changed);
            }
            _ => {}
        }
        sync
    }

    /// A spell slot spent here and not yet sent.
    #[cfg(test)]
    pub fn simulate_slot_edit(&mut self) {
        let mut slots = serde_json::Map::new();
        for level in 1..=9 {
            slots.insert(format!("level{level}"), 0.into());
        }
        self.remote
            .as_mut()
            .expect("a sheet read")
            .groups
            .insert("slots".into(), slots.clone().into());
        slots.insert("level1".into(), 1.into());
        self.desired.insert("slots".into(), slots.into());
    }

    pub fn stop(&mut self) {
        if let Some(gate) = &self.gate {
            gate.store(false, Ordering::SeqCst);
        }
        let in_panel = self.field.take().is_some_and(|f| f.entered.is_some());
        self.typed_at_stop |= in_panel && self.dirty();
        self.pending = None;
        self.gate = None;
        self.enabled = false;
    }
    /// The session ended. This sync stops, and says so until the DM starts it again.
    pub fn session_ended(&mut self) {
        #[cfg(test)]
        if self.simulated {
            return;
        }
        if self.enabled {
            self.stop();
            self.error = Some(SESSION_LOST.into());
            self.session_lost = true;
        }
    }
    pub fn session_lost(&self) -> bool {
        self.session_lost
    }
    /// Changes D&D Beyond has not confirmed: on their way, or stranded with sync off.
    pub fn unconfirmed(&self) -> bool {
        (self.enabled && (self.busy() || self.dirty())) || self.stranded()
    }
    /// Sync is off, by the session or by the DM, and these changes have not gone.
    pub fn stranded(&self) -> bool {
        !self.enabled && self.dirty()
    }
    /// Off, with a read to compare the run's values with.
    pub fn stopped_after_read(&self) -> bool {
        !self.enabled && self.remote.is_some()
    }
    /// Sync is off, and values only this sync holds have not gone. A close loses them.
    pub fn unsent_only_here(&self) -> bool {
        !self.enabled && !self.only_here().is_empty()
    }
    fn only_here(&self) -> Groups {
        // A number still in a panel field has not reached a run either.
        let all = self.no_run || self.field.as_ref().is_some_and(|f| f.entered.is_some());
        self.desired
            .iter()
            .filter(|(key, value)| {
                (all || !KEPT_BY_THE_RUN.contains(&key.as_str()))
                    && self.remote.as_ref().and_then(|r| r.groups.get(*key)) != Some(*value)
            })
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect()
    }
    /// Closing the app drops the worker and everything only it holds, with sync on or off.
    pub fn loses_on_close(&self) -> bool {
        !self.only_here().is_empty() || !self.unsent.is_empty()
    }
    /// Why a conflict's choices are off. Neither can do what it says with sync stopped: the
    /// run would not change, and nothing would be sent.
    fn choice_blocked(&self, stopped: &'static str) -> Option<&'static str> {
        if self.busy() {
            Some("Wait for the current sheet read.")
        } else if !self.enabled {
            Some(stopped)
        } else {
            None
        }
    }
    fn keep_unsent(&mut self) {
        self.unsent.extend(self.only_here());
    }
    /// What D&D Beyond last confirmed.
    pub fn confirmed(&self) -> Option<&Groups> {
        self.remote.as_ref().map(|remote| &remote.groups)
    }
    /// Let this sync react to the session, as a real one does.
    #[cfg(test)]
    pub fn stop_simulating(&mut self) {
        self.simulated = false;
    }
    pub fn ready(&self) -> bool {
        self.enabled && self.remote.is_some()
    }
    pub fn state(&self) -> SyncState {
        if !self.blocked.is_empty() {
            SyncState::Conflict
        } else if self.stranded() {
            SyncState::NotSent
        } else if self.session_lost {
            SyncState::Stopped
        } else if self.error.is_some() {
            SyncState::Error
        } else if !self.enabled {
            SyncState::Off
        } else {
            match &self.remote {
                None => SyncState::FirstRead,
                Some(remote) if !remote.can_edit => SyncState::ReadOnly,
                Some(_) if self.busy() || self.dirty() => SyncState::Pending,
                Some(_) => SyncState::Matching,
            }
        }
    }
    /// Test builds have no session, and `tick` stops sync without one. Keep it on.
    #[cfg(test)]
    pub fn keep_without_connection(&mut self) {
        self.simulated = true;
    }
    #[cfg(not(test))]
    pub fn keep_without_connection(&mut self) {}
    /// Turn sync on. It reads the sheet before it writes anything.
    pub fn start(&mut self) {
        self.enabled = true;
        self.keep_unsent();
        self.field = None;
        self.typed_at_stop = false;
        if let Some(remote) = self.remote.take() {
            self.matched = Some(remote.groups);
        }
        self.desired.clear();
        self.blocked.clear();
        self.next = None;
        self.error = None;
        self.session_lost = false;
    }
    pub fn busy(&self) -> bool {
        self.pending.is_some()
    }
    pub fn dirty(&self) -> bool {
        self.remote
            .as_ref()
            .is_some_and(|r| r.groups != self.desired)
    }
    pub fn record(&self) -> Option<&EntitySnapshot> {
        if self.enabled {
            self.remote.as_ref()?.record.as_ref()
        } else {
            None
        }
    }
    pub fn max_hp(&self) -> Option<i64> {
        self.remote.as_ref().map(|r| r.max_hp)
    }
    pub fn values(&self) -> &Groups {
        &self.desired
    }
    pub fn set_group(&mut self, key: &str, value: Value) {
        if self.ready() && self.desired.contains_key(key) && self.desired.get(key) != Some(&value) {
            self.desired.insert(key.into(), value);
            self.changed = Some(Instant::now());
        }
    }
    /// Party has no run to keep what this sync holds.
    pub fn without_a_run(&mut self) {
        self.no_run = true;
    }
    /// Values the DM changed that can still go: sync is on, and no conflict holds them.
    pub fn edits_to_send(&self) -> bool {
        self.enabled
            && self.dirty()
            && self.desired.iter().any(|(k, v)| {
                !self.blocked.contains_key(k)
                    && self.remote.as_ref().and_then(|r| r.groups.get(k)) != Some(v)
            })
    }
    /// Every timer has run out: the pause after a change, and the 15-second read.
    #[cfg(test)]
    fn run_out_timers(&mut self) {
        let long = Duration::from_secs(16);
        self.changed = self.changed.map(|t| t - long);
        self.next = self.next.map(|t| t - long);
    }
    /// Every timer has run out. What an exchange would send now, if it may start, is confirmed
    /// by the sheet. For a test of what a run sends, as test builds have no route to a sheet.
    #[cfg(test)]
    pub fn hp_sent_at_once(&mut self, ctx: &egui::Context) -> Option<Value> {
        self.run_out_timers();
        if !self.advance(ctx) || !self.edits_to_send() {
            return None;
        }
        let sent = self.desired["hp"].clone();
        self.remote.as_mut()?.groups = self.desired.clone();
        Some(sent)
    }
    /// The DM is in a number field of this character.
    pub fn in_field(&self) -> bool {
        self.field.is_some()
    }
    /// True once if sync stopped with a number in a panel field. The run takes it first.
    pub fn take_typed_at_stop(&mut self) -> bool {
        std::mem::take(&mut self.typed_at_stop)
    }
    /// Hold this character's exchanges while the DM is in a field outside this panel that
    /// reaches this sync. `Correct HP` in the Record tab is one.
    pub fn hold_for(&mut self, response: &egui::Response) {
        if self.ready()
            && (response.has_focus() || response.dragged())
            && self.field.as_ref().is_none_or(|f| f.id != response.id)
        {
            self.field = Some(NumberField {
                id: response.id,
                entered: None,
            });
        }
    }
    /// True while the DM is in the field, and on the frame they left it with Escape, where a
    /// value is put back. A handler can take Escape first, and then egui sets the typed number
    /// as the field is drawn. So `show` calls this after the fields, as well as `tick`.
    /// When it ends, the usual delay starts, and a read overdue from the wait is not sooner.
    fn in_number_field(&mut self, ctx: &egui::Context) -> bool {
        let Some(field) = &self.field else {
            return false;
        };
        if ctx.memory(|m| m.has_focus(field.id)) || ctx.is_being_dragged(field.id) {
            return true;
        }
        if escape_pressed(ctx) {
            if let Some((key, value)) = field.entered.clone() {
                self.desired.insert(key, value);
            }
            return true;
        }
        self.field = None;
        let now = Instant::now();
        let after_delay = now + DELAY;
        self.changed = Some(now);
        self.next = Some(self.next.map_or(after_delay, |next| next.max(after_delay)));
        false
    }
    /// This frame's sync work, apart from talking to D&D Beyond. True if an exchange is due.
    fn advance(&mut self, ctx: &egui::Context) -> bool {
        if self.in_number_field(ctx) {
            return false;
        }
        if let Some(pending) = &self.pending {
            let result = match pending.receiver.try_recv() {
                Ok(r) => Some(r),
                Err(mpsc::TryRecvError::Disconnected) => {
                    Some(Err("Combat sync stopped unexpectedly.".into()))
                }
                Err(mpsc::TryRecvError::Empty) => None,
            };
            if let Some(result) = result {
                let sent = self.pending.take().unwrap().sent;
                match result {
                    Ok(outcome) => {
                        self.accept(outcome, &sent);
                    }
                    Err(error) => {
                        self.error = Some(error);
                        if let Some(remote) = &self.remote {
                            for (key, value) in &self.desired {
                                if remote.groups.get(key) != Some(value) {
                                    self.blocked.insert(key.clone(),"Write status is uncertain. Refresh and review before retrying.".into());
                                }
                            }
                        }
                    }
                }
                self.next = Some(
                    Instant::now()
                        + if self.error.is_some() {
                            Duration::from_secs(60)
                        } else {
                            INTERVAL
                        },
                );
            }
        }
        let due = self.next.is_none_or(|t| t <= Instant::now())
            || (self.edits_to_send() && self.changed.is_none_or(|t| t.elapsed() >= DELAY));
        self.pending.is_none() && due
    }
    /// Note an exchange as on its way, and what it starts from.
    fn begin_exchange(&mut self) -> Job {
        let (reply, receiver) = mpsc::channel();
        self.pending = Some(Pending {
            receiver,
            sent: self.desired.clone(),
        });
        Job {
            baseline: self.remote.clone(),
            desired: self.desired.clone(),
            blocked: self.blocked.keys().cloned().collect(),
            read_only: std::mem::take(&mut self.refresh_only),
            reply,
        }
    }
    pub fn tick(
        &mut self,
        ctx: &egui::Context,
        session: Option<Session>,
        epoch: u64,
        campaign: &str,
        id: &str,
    ) {
        // Test builds have no route to D&D Beyond. A signed-in test stays on its first read.
        if cfg!(test) {
            let _ = (&session, epoch, campaign, id);
            self.in_number_field(ctx);
            return;
        }
        // Its words and unsent changes stay until the DM starts it again.
        if self.session_lost {
            return;
        }
        let scope = (epoch, campaign.to_owned(), id.to_owned());
        if self.scope.as_ref() != Some(&scope) {
            self.enter_scope(scope);
        }
        if !self.enabled || session.is_none() {
            if self.pending.is_some() || session.is_none() {
                self.stop();
            }
            return;
        }
        if self.advance(ctx) {
            let job = self.begin_exchange();
            let id = id.to_owned();
            let campaign = campaign.to_owned();
            let session = session.unwrap();
            let gate = Arc::new(AtomicBool::new(true));
            self.gate = Some(gate.clone());
            let ctx = ctx.clone();
            std::thread::spawn(move || {
                static LOCKS: OnceLock<Mutex<BTreeMap<String, Arc<Mutex<()>>>>> = OnceLock::new();
                let result = (|| {
                    let lock = LOCKS
                        .get_or_init(Default::default)
                        .lock()
                        .map_err(|_| "Sync lock unavailable.")?
                        .entry(id.clone())
                        .or_default()
                        .clone();
                    let _guard = lock
                        .lock()
                        .map_err(|_| "Character sync lock unavailable.")?;
                    let client = Client::builder()
                        .timeout(Duration::from_secs(20))
                        .redirect(Policy::none())
                        .build()
                        .map_err(|_| "Could not create the sync client.")?;
                    exchange(
                        &mut Live {
                            session,
                            client,
                            id,
                            campaign,
                            active: gate,
                        },
                        job.baseline.as_ref(),
                        &job.desired,
                        &job.blocked,
                        job.read_only,
                    )
                })();
                let _ = job.reply.send(result);
                ctx.request_repaint();
            });
        }
        ctx.request_repaint_after(DELAY);
    }
    /// A new session or character starts clean, apart from what was last matched: that is
    /// how the next read can tell the DM's changes from the sheet's.
    fn enter_scope(&mut self, scope: (u64, String, String)) {
        let enabled = self.enabled;
        let no_run = self.no_run;
        // Another character's values are nothing to this one's sheet.
        let same_character = self.scope.as_ref().is_none_or(|old| old.2 == scope.2);
        self.keep_unsent();
        let unsent = std::mem::take(&mut self.unsent);
        let matched = self
            .remote
            .take()
            .map(|remote| remote.groups)
            .or_else(|| self.matched.take());
        self.stop();
        *self = Self::default();
        self.enabled = enabled;
        self.no_run = no_run;
        self.scope = Some(scope);
        if same_character {
            self.matched = matched;
            self.unsent = unsent;
        }
    }
    /// True once for each first read, for the run to compare with its own values.
    pub fn take_first_read(&mut self) -> bool {
        std::mem::take(&mut self.first_read)
    }
    pub fn take_matched(&mut self) -> Option<Groups> {
        self.matched.take()
    }
    /// The run's value and the sheet's differ, and the DM changed the run's: their choice.
    pub fn hold_conflict(&mut self, key: &str, local: Value) {
        self.desired.insert(key.into(), local);
        self.blocked.insert(key.into(), DIFFER_AFTER_READ.into());
    }
    /// A change the DM recorded while sync was off. It cannot be sent yet, and it counts.
    pub fn note_while_stopped(&mut self, key: &str, local: Value) {
        self.desired.insert(key.into(), local);
    }
    pub fn use_remote(&mut self, key: &str) {
        self.updated = true;
        if let Some(value) = self.remote.as_ref().and_then(|r| r.groups.get(key)) {
            self.desired.insert(key.to_owned(), value.clone());
        } else {
            self.desired.remove(key);
        }
        self.blocked.remove(key);
    }
    pub fn keep_local(&mut self, key: &str) {
        self.blocked.remove(key);
        self.changed = None;
        self.next = None;
    }
    /// A read from D&D Beyond, with the sheet's HP. Test builds have no route to a live one.
    #[cfg(test)]
    pub fn simulate_read(&mut self, character: u64, max_hp: i64, damage: i64) {
        self.accept_read(simulated_sheet(character, max_hp, damage, true));
    }
    /// A read of a sheet given in full, in D&D Beyond's own fields.
    #[cfg(test)]
    pub fn simulate_sheet(&mut self, character: u64, max_hp: i64, sheet: &Value) {
        self.accept_read(Remote::from_raw(character, max_hp, sheet).expect("simulated sheet"));
    }
    #[cfg(test)]
    fn accept_read(&mut self, remote: Remote) {
        let sent = self.desired.clone();
        self.accept(
            Outcome {
                remote,
                blocked: BTreeMap::new(),
            },
            &sent,
        );
    }
    #[cfg(test)]
    pub fn conflict_keys(&self) -> Vec<String> {
        self.blocked.keys().cloned().collect()
    }
    fn accept(&mut self, outcome: Outcome, sent: &Groups) {
        let initial = self.remote.is_none();
        self.first_read |= initial;
        let previous = self
            .remote
            .as_ref()
            .map(|r| r.groups.clone())
            .unwrap_or_default();
        for (key, value) in &outcome.remote.groups {
            let local = self.desired.get(key).cloned();
            let unchanged = local.as_ref() == sent.get(key);
            if initial
                || (unchanged
                    && !outcome.blocked.contains_key(key)
                    && !self.blocked.contains_key(key)
                    && (previous.get(key) == sent.get(key) || sent.get(key) == Some(value)))
            {
                self.desired.insert(key.clone(), value.clone());
            } else if local.as_ref() == Some(value) {
                self.blocked.remove(key);
            } else if previous.get(key) != Some(value)
                && previous.get(key) != local.as_ref()
                && sent.get(key) != Some(value)
                && !outcome.blocked.contains_key(key)
            {
                self.blocked.insert(
                    key.clone(),
                    "Splat and D&D Beyond both changed this. Choose which to keep.".into(),
                );
            }
        }
        // Removed resources are a conflict if an edit is pending, never a write to an old ID.
        for key in previous
            .keys()
            .filter(|k| !outcome.remote.groups.contains_key(*k))
        {
            if self.desired.get(key) != previous.get(key) {
                self.blocked.insert(
                    key.clone(),
                    "This resource is no longer on the sheet.".into(),
                );
            } else {
                self.desired.remove(key);
            }
        }
        self.blocked.extend(outcome.blocked);
        // Back from a stop: an unsent value the sheet does not have is the DM's to settle.
        for (key, value) in std::mem::take(&mut self.unsent) {
            if outcome.remote.groups.get(&key).is_some_and(|v| v != &value) {
                self.hold_conflict(&key, value);
            }
        }
        self.remote = Some(outcome.remote);
        self.error = None;
        self.updated = true;
    }
    /// Each conflict with both values and its two ways out. The run shows this beside
    /// the roster, so it does not stop the table. `stopped` is why the choices are off and what
    /// to do if this sync is stopped. It takes the message's place, in view, as nobody at the
    /// table hovers, and a message that says to choose would be false.
    pub fn conflicts(&mut self, ui: &mut egui::Ui, stopped: &'static str) {
        use crate::design::{self, Action, Density, Type, colors};
        for (key, message) in self.blocked.clone() {
            let c = colors();
            ui.label(
                Type::Heading
                    .rich(
                        ui,
                        format!("{} {} conflict", design::icons::WARN, group_label(&key)),
                    )
                    .color(c.sync.conflict),
            );
            let line = if self.enabled {
                message.as_str()
            } else {
                stopped
            };
            ui.add(egui::Label::new(Type::Small.rich(ui, line).color(c.text)).wrap());
            // The two values side by side, so the difference is what the eye finds.
            egui::Grid::new(("conflict_values", &key))
                .num_columns(2)
                .spacing([design::L, 2.0])
                .show(ui, |ui| {
                    for (label, value) in [
                        ("Splat", self.desired.get(&key)),
                        (
                            "D&D Beyond",
                            self.remote.as_ref().and_then(|r| r.groups.get(&key)),
                        ),
                    ] {
                        ui.label(Type::Small.rich(ui, label).color(colors().muted));
                        ui.label(
                            Type::Strong
                                .rich(ui, describe(&key, value, self.max_hp().unwrap_or(0)))
                                .color(colors().text),
                        );
                        ui.end_row();
                    }
                });
            ui.push_id(&key, |ui| {
                ui.horizontal_wrapped(|ui| {
                    let blocked = self.choice_blocked(stopped);
                    let max = self.max_hp().unwrap_or(0);
                    let sheet = self.remote.as_ref().and_then(|r| r.groups.get(&key));
                    let (use_label, keep_label) = (
                        format!("Use {} from D&D Beyond", short_value(&key, sheet, max)),
                        format!(
                            "Keep {}, send to D&D Beyond",
                            short_value(&key, self.desired.get(&key), max)
                        ),
                    );
                    if Action::secondary(&use_label)
                        .name("Use D&D Beyond values")
                        .density(Density::Read)
                        .blocked(blocked)
                        .show(ui)
                        .clicked()
                    {
                        self.use_remote(&key);
                    }
                    let writable = self
                        .remote
                        .as_ref()
                        .is_some_and(|r| r.can_edit && r.groups.contains_key(&key));
                    if Action::secondary(&keep_label)
                        .name("Keep Splat values and retry")
                        .density(Density::Read)
                        .blocked(blocked.or(
                            (!writable).then_some(
                                "This session cannot edit this character on D&D Beyond.",
                            ),
                        ))
                        .show(ui)
                        .clicked()
                    {
                        self.keep_local(&key);
                    }
                });
            });
        }
    }
    /// `offline` is why sync cannot start now. A sync that is on can always be stopped.
    pub fn show(&mut self, ui: &mut egui::Ui, offline: Option<&str>) {
        let was = self.enabled;
        crate::design::locked(ui, offline.filter(|_| !was), |ui| {
            ui.checkbox(&mut self.enabled, "Sync combat stats with D&D Beyond");
        });
        if was && !self.enabled {
            self.stop();
        }
        if !was && self.enabled {
            self.start();
        }
        if !self.enabled {
            if let Some(error) = &self.error {
                ui.colored_label(crate::design::colors().result.error, error);
            }
            if let Some(offline) = offline {
                ui.weak(offline);
                return;
            }
            ui.weak("Enable to read the live sheet. Edits made here will then update D&D Beyond.");
            return;
        }
        ui.weak("HP, death saves, conditions, spell slots, and supported limited uses · checks every 15 seconds");
        if self.busy() {
            ui.horizontal(|ui| {
                crate::design::spinner(ui);
                ui.label("Checking character…");
            });
        }
        if let Some(e) = &self.error {
            ui.colored_label(crate::design::colors().result.error, e);
        }
        if crate::design::Action::secondary("Refresh / verify")
            .density(crate::design::Density::Read)
            .blocked_if(self.busy(), "Wait for the current sheet read.")
            .show(ui)
            .clicked()
        {
            self.refresh_only = true;
            self.next = None;
        }
        if self.dirty() {
            ui.add(egui::Label::new("Local combat changes pending").wrap());
        } else if self.remote.is_some() {
            ui.add(egui::Label::new("Combat values match the last read").wrap());
        }
        // Sync is on here, so `stopped` is not used. This panel returns early when it is off.
        self.conflicts(ui, STOPPED_SYNC_AGAIN);
        let Some(remote) = &self.remote else {
            ui.label("Waiting for the first sheet read. If it does not come, check the connection in Settings.");
            return;
        };
        if !remote.can_edit {
            ui.label("Read only: D&D Beyond reports that this session cannot edit this character.");
        }
        let max_hp = remote.max_hp;
        let labels = remote.labels.clone();
        let writable = remote.can_edit;
        let before = self.desired.clone();
        // The field the DM is in, if any, and its group.
        let mut in_field = None;
        // A read starting or a conflict coming adds rows above. egui counts rows into a field's
        // id, so without a scope of its own that takes the field the DM is typing in away.
        let values = egui::UiBuilder::new()
            .id_salt(ui.id().with("sheet_values"))
            .global_scope(true);
        ui.scope_builder(values, |ui| {
            ui.add_enabled_ui(writable, |ui| {
                if let Some(hp) = self.desired.get_mut("hp") {
                    ui.horizontal_wrapped(|ui| {
                        let mut current =
                            (max_hp - hp["removedHitPoints"].as_i64().unwrap_or(0)).max(0);
                        ui.label(format!("HP / {max_hp}"));
                        if sheet_number(
                            ui,
                            &mut current,
                            0..=max_hp,
                            "Current HP on the sheet",
                            "hp",
                            &mut in_field,
                        ) {
                            hp["removedHitPoints"] = (max_hp - current).into();
                        }
                        let mut temp = hp["temporaryHitPoints"].as_i64().unwrap_or(0);
                        ui.label("Temporary HP");
                        if sheet_number(
                            ui,
                            &mut temp,
                            0..=1_000_000,
                            "Temporary HP on the sheet",
                            "hp",
                            &mut in_field,
                        ) {
                            hp["temporaryHitPoints"] = temp.into();
                        }
                    });
                }
                if let Some(death) = self.desired.get_mut("death") {
                    ui.horizontal_wrapped(|ui| {
                        ui.label("Death saves");
                        for (field, label) in
                            [("successCount", "Successes"), ("failCount", "Failures")]
                        {
                            let mut n = death[field].as_i64().unwrap_or(0);
                            ui.label(label);
                            if sheet_number(
                                ui,
                                &mut n,
                                0..=3,
                                &format!("Death save {} on the sheet", label.to_lowercase()),
                                "death",
                                &mut in_field,
                            ) {
                                death[field] = n.into();
                            }
                        }
                    });
                }
                crate::design::section(ui, "Sheet conditions", |ui| {
                    if let Some(Value::Object(conditions)) = self.desired.get_mut("conditions") {
                        ui.horizontal_wrapped(|ui| {
                            for (id, name) in CONDITIONS {
                                let key = id.to_string();
                                let mut present = conditions.contains_key(&key);
                                if ui.checkbox(&mut present, name).changed() {
                                    if present {
                                        conditions.insert(
                                            key.clone(),
                                            if id == 4 { 1.into() } else { Value::Null },
                                        );
                                    } else {
                                        conditions.remove(&key);
                                    }
                                }
                                if id == 4 && present {
                                    let mut level = conditions[&key].as_u64().unwrap_or(1);
                                    if sheet_number(
                                        ui,
                                        &mut level,
                                        1..=6,
                                        "Exhaustion level",
                                        "conditions",
                                        &mut in_field,
                                    ) {
                                        conditions.insert(key, level.into());
                                    }
                                }
                            }
                        });
                    }
                });
                crate::design::section(ui, "Spell slots spent", |ui| {
                    for (key, label, levels) in
                        [("slots", "Ordinary slots", 9), ("pact", "Pact slots", 5)]
                    {
                        if let Some(slots) = self.desired.get_mut(key) {
                            ui.label(label);
                            ui.horizontal_wrapped(|ui| {
                                for level in 1..=levels {
                                    let field = format!("level{level}");
                                    let mut used = slots[&field].as_i64().unwrap_or(0);
                                    ui.label(format!("L{level}"));
                                    if sheet_number(
                                        ui,
                                        &mut used,
                                        0..=99,
                                        &format!("{label} spent at level {level}"),
                                        key,
                                        &mut in_field,
                                    ) {
                                        slots[&field] = used.into();
                                    }
                                }
                            });
                        }
                    }
                });
                crate::design::section(ui, "Limited uses spent", |ui| {
                    for (key, label) in labels {
                        if let Some(resource) = self.desired.get_mut(&key) {
                            ui.push_id(&key, |ui| {
                                ui.horizontal_wrapped(|ui| {
                                    ui.label(label);
                                    let mut used = resource["uses"].as_i64().unwrap_or(0);
                                    if sheet_number(
                                        ui,
                                        &mut used,
                                        0..=1_000_000,
                                        "Uses spent",
                                        &key,
                                        &mut in_field,
                                    ) {
                                        resource["uses"] = used.into();
                                    }
                                });
                            });
                        }
                    }
                });
            })
        });
        if self.desired != before {
            self.changed = Some(Instant::now());
        }
        match in_field {
            Some((id, key)) if self.field.as_ref().is_none_or(|field| field.id != id) => {
                let entered = before.get(&key).cloned().map(|value| (key, value));
                self.field = Some(NumberField { id, entered });
            }
            Some(_) => {}
            None => {
                self.in_number_field(ui.ctx());
            }
        }
        ui.weak("Concentration, custom effects, inventory charges, initiative, and DM notes stay local.");
    }
}
/// One number on the sheet. The value counts at each key, so no way of leaving can lose it,
/// and `in_field` notes the field so that it does not go out until the DM leaves. True if it
/// changed.
fn sheet_number<N: egui::emath::Numeric>(
    ui: &mut egui::Ui,
    value: &mut N,
    range: std::ops::RangeInclusive<N>,
    name: &str,
    group: &str,
    in_field: &mut Option<(egui::Id, String)>,
) -> bool {
    let response = ui.add(egui::DragValue::new(value).range(range));
    crate::design::named(ui, &response, name);
    if response.has_focus() || response.dragged() {
        *in_field = Some((response.id, group.to_owned()));
    }
    response.changed()
}
fn describe(key: &str, value: Option<&Value>, max_hp: i64) -> String {
    let Some(value) = value else {
        return "Unavailable".into();
    };
    match key {
        "hp" => format!(
            "{} / {max_hp} HP, {} temporary HP",
            (max_hp - value["removedHitPoints"].as_i64().unwrap_or(0)).max(0),
            value["temporaryHitPoints"]
        ),
        "death" => format!(
            "{} successes, {} failures",
            value["successCount"], value["failCount"]
        ),
        "conditions" => {
            let names: Vec<_> = CONDITIONS
                .iter()
                .filter_map(|(id, name)| {
                    value.get(id.to_string()).map(|level| {
                        if *id == 4 {
                            format!("{name} {level}")
                        } else {
                            (*name).into()
                        }
                    })
                })
                .collect();
            if names.is_empty() {
                "None".into()
            } else {
                names.join(", ")
            }
        }
        "slots" | "pact" => value
            .as_object()
            .map(|slots| {
                slots
                    .iter()
                    .map(|(level, count)| {
                        format!("Level {}: {count} spent", level.trim_start_matches("level"))
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .unwrap_or_default(),
        _ => format!("{} uses spent", value["uses"]),
    }
}
/// A value on a button: the number, for HP, and otherwise which side's values.
fn short_value(key: &str, value: Option<&Value>, max_hp: i64) -> String {
    match (key, value) {
        ("hp", Some(hp)) => format!(
            "{}/{max_hp} HP",
            (max_hp - hp["removedHitPoints"].as_i64().unwrap_or(0)).max(0)
        ),
        _ => "these values".to_owned(),
    }
}

fn group_label(key: &str) -> &str {
    match key {
        "hp" => "HP",
        "death" => "Death saves",
        "conditions" => "Conditions",
        "slots" => "Spell slots",
        "pact" => "Pact slots",
        _ => "Limited-use resource",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn remote() -> Remote {
        Remote::from_raw(123,30,&json!({"id":123,"canEdit":true,"removedHitPoints":5,"temporaryHitPoints":3,"deathSaves":{"successCount":0,"failCount":0},"conditions":[{"id":11,"level":null}],"spellSlots":[{"level":1,"used":1},{"level":2,"used":2}],"pactMagic":[{"level":2,"used":1}],"actions":{"class":[{"id":8,"entityTypeId":9,"name":"Second Wind","limitedUse":{"numberUsed":1}}]},"classSpells":[{"spells":[{"id":20,"entityTypeId":21,"definition":{"id":999,"entityTypeId":998,"name":"Gift spell"},"limitedUse":{"numberUsed":0}}]}]})).unwrap()
    }
    struct Fake {
        remote: Remote,
        writes: Vec<Mutation>,
        no_apply: bool,
        failure_after_apply: bool,
        read_count: usize,
    }
    impl Fake {
        fn new(remote: Remote) -> Self {
            Self {
                remote,
                writes: Vec::new(),
                no_apply: false,
                failure_after_apply: false,
                read_count: 0,
            }
        }
    }
    impl Transport for Fake {
        fn read(&mut self) -> Result<Remote, String> {
            self.read_count += 1;
            Ok(self.remote.clone())
        }
        fn write(&mut self, m: &Mutation) -> Result<(), String> {
            self.writes.push(m.clone());
            let key = match m.route.as_str() {
                "/life/hp/damage-taken" => "hp".into(),
                "/life/death-saves" => "death".into(),
                "/spell/slots" => "slots".into(),
                "/spell/pact-magic" => "pact".into(),
                "/condition" => "conditions".into(),
                "/action/limited-use" => {
                    format!("limited/{}/{}", m.body["entityTypeId"], m.body["id"])
                }
                _ => panic!("Unexpected route"),
            };
            if !self.no_apply {
                self.remote.groups.insert(key, m.expected.clone());
            }
            if self.failure_after_apply {
                Err("Timeout after send".into())
            } else {
                Ok(())
            }
        }
    }
    #[test]
    fn exact_hp_payload_preserves_untouched_damage_when_only_temp_hp_changes() {
        let r = remote();
        let mut desired = r.groups.clone();
        desired.get_mut("hp").unwrap()["temporaryHitPoints"] = 7.into();
        let mut fake = Fake::new(r.clone());
        let outcome = exchange(&mut fake, Some(&r), &desired, &BTreeSet::new(), false).unwrap();
        assert_eq!(fake.writes.len(), 1);
        assert_eq!(
            fake.writes[0].body,
            json!({"characterId":123,"removedHitPoints":5,"temporaryHitPoints":7})
        );
        assert_eq!(fake.read_count, 2);
        assert!(outcome.blocked.is_empty());
        assert_eq!(outcome.remote.groups["hp"], desired["hp"]);
    }
    #[test]
    fn inbound_changes_and_same_field_conflicts_do_not_send_stale_values() {
        let old = remote();
        let mut current = old.clone();
        current.groups.get_mut("hp").unwrap()["removedHitPoints"] = 12.into();
        let mut fake = Fake::new(current.clone());
        let read = exchange(&mut fake, Some(&old), &old.groups, &BTreeSet::new(), false).unwrap();
        assert_eq!(read.remote.groups, current.groups);
        assert!(fake.writes.is_empty());
        let mut desired = old.groups.clone();
        desired.get_mut("hp").unwrap()["removedHitPoints"] = 8.into();
        let result = exchange(&mut fake, Some(&old), &desired, &BTreeSet::new(), false).unwrap();
        assert!(result.blocked.contains_key("hp"));
        assert!(fake.writes.is_empty());
    }
    #[test]
    fn read_only_permission_and_changed_maximum_block_writes() {
        let old = remote();
        let mut target = old.groups.clone();
        target.get_mut("hp").unwrap()["removedHitPoints"] = 8.into();
        let mut denied = old.clone();
        denied.can_edit = false;
        let mut fake = Fake::new(denied);
        assert!(
            exchange(&mut fake, Some(&old), &target, &BTreeSet::new(), false)
                .unwrap()
                .blocked
                .contains_key("hp")
        );
        assert!(fake.writes.is_empty());
        let mut changed = old.clone();
        changed.max_hp = 40;
        let mut fake = Fake::new(changed);
        assert!(
            exchange(&mut fake, Some(&old), &target, &BTreeSet::new(), false)
                .unwrap()
                .blocked
                .contains_key("hp")
        );
        assert!(fake.writes.is_empty());
    }
    #[test]
    fn conditions_use_real_ids_level_and_total_hp_with_separate_delete() {
        let r = remote();
        let target = json!({"4":2,"12":null});
        let changes = mutations(&r, "conditions", &target).unwrap();
        assert_eq!(changes.len(), 3);
        assert_eq!(changes[0].method, Method::DELETE);
        assert_eq!(changes[0].body, json!({"characterId":123,"id":11}));
        let exhaustion = changes.iter().find(|m| m.body["id"] == 4).unwrap();
        assert_eq!(
            exhaustion.body,
            json!({"characterId":123,"id":4,"level":2,"totalHp":30})
        );
        assert_eq!(changes.last().unwrap().expected, target);
        assert!(mutations(&r, "conditions", &json!({"4":7})).is_err());
    }
    #[test]
    fn slot_pools_and_resource_mapping_ids_stay_separate() {
        let r = remote();
        assert_eq!(r.groups["slots"]["level2"], 2);
        assert_eq!(r.groups["pact"]["level2"], 1);
        assert!(r.groups.contains_key("limited/21/20"));
        assert!(!r.groups.contains_key("limited/998/999"));
        let mut desired = r.groups.clone();
        desired.get_mut("slots").unwrap()["level1"] = 2.into();
        desired.get_mut("pact").unwrap()["level2"] = 2.into();
        desired.get_mut("limited/21/20").unwrap()["uses"] = 1.into();
        let mut fake = Fake::new(r.clone());
        assert!(
            exchange(&mut fake, Some(&r), &desired, &BTreeSet::new(), false)
                .unwrap()
                .blocked
                .is_empty()
        );
        assert_eq!(fake.writes.len(), 3);
        assert!(
            fake.writes.iter().any(|m| m.route == "/spell/slots"
                && m.body["level2"] == 2
                && m.body["level9"] == 0)
        );
        assert!(fake.writes.iter().any(|m| m.route == "/action/limited-use"
            && m.body["id"] == 20
            && m.body["entityTypeId"] == 21));
    }
    #[test]
    fn uncertain_writes_are_read_back_and_never_blindly_retried() {
        let old = remote();
        let mut desired = old.groups.clone();
        desired.get_mut("hp").unwrap()["removedHitPoints"] = 8.into();
        let mut fake = Fake::new(old.clone());
        fake.failure_after_apply = true;
        assert!(
            exchange(&mut fake, Some(&old), &desired, &BTreeSet::new(), false)
                .unwrap()
                .blocked
                .is_empty()
        );
        assert_eq!(fake.writes.len(), 1);
        exchange(&mut fake, Some(&old), &desired, &BTreeSet::new(), false).unwrap();
        assert_eq!(fake.writes.len(), 1);
        let mut fake = Fake::new(old.clone());
        fake.no_apply = true;
        assert!(
            exchange(&mut fake, Some(&old), &desired, &BTreeSet::new(), false)
                .unwrap()
                .blocked
                .contains_key("hp")
        );
        assert_eq!(fake.writes.len(), 1);
        exchange(
            &mut fake,
            Some(&old),
            &desired,
            &BTreeSet::from(["hp".into()]),
            false,
        )
        .unwrap();
        assert_eq!(fake.writes.len(), 1);
    }
    #[test]
    fn null_counters_are_zero_but_missing_or_invalid_values_are_not_invented() {
        let r = Remote::from_raw(
            1,
            20,
            &json!({"id":1,"removedHitPoints":null,"temporaryHitPoints":null,"deathSaves":null}),
        )
        .unwrap();
        assert_eq!(
            r.groups["hp"],
            json!({"removedHitPoints":0,"temporaryHitPoints":0})
        );
        assert_eq!(r.groups["death"], json!({"successCount":0,"failCount":0}));
        let missing = Remote::from_raw(1, 20, &json!({"id":1})).unwrap();
        assert!(!missing.groups.contains_key("hp"));
        assert!(
            Remote::from_raw(
                1,
                20,
                &json!({"id":1,"removedHitPoints":-1,"temporaryHitPoints":0})
            )
            .is_err()
        );
    }
    #[test]
    fn initial_connection_only_reads_even_if_given_old_saved_run_values() {
        let old = remote();
        let mut target = old.groups.clone();
        target.get_mut("hp").unwrap()["removedHitPoints"] = 30.into();
        let mut fake = Fake::new(old);
        exchange(&mut fake, None, &target, &BTreeSet::new(), false).unwrap();
        assert!(fake.writes.is_empty());
    }
    #[test]
    fn response_keeps_newer_edits_and_does_not_conflict_with_own_write() {
        let old = remote();
        let mut sync = CombatSync::default();
        sync.remote = Some(old.clone());
        let mut sent = old.groups.clone();
        sent.insert(
            "hp".into(),
            json!({"removedHitPoints":5,"temporaryHitPoints":0}),
        );
        sync.desired = sent.clone();
        sync.desired.get_mut("hp").unwrap()["removedHitPoints"] = 8.into();
        let mut confirmed = old;
        confirmed.groups = sent.clone();
        sync.accept(
            Outcome {
                remote: confirmed,
                blocked: BTreeMap::new(),
            },
            &sent,
        );
        assert_eq!(sync.desired["hp"]["removedHitPoints"], 8);
        assert!(sync.blocked.is_empty());
        assert!(sync.dirty());
    }
    #[test]
    fn a_new_session_keeps_what_was_last_matched() {
        let mut sync = CombatSync::default();
        sync.enabled = true;
        sync.remote = Some(remote());
        let matched = remote().groups;
        sync.enter_scope((2, "1".into(), "123".into()));
        assert!(sync.enabled);
        assert!(sync.remote.is_none());
        assert_eq!(sync.take_matched(), Some(matched.clone()));
        // Started again, then a new session: what was matched still stands.
        sync.remote = Some(remote());
        sync.start();
        sync.enter_scope((3, "1".into(), "123".into()));
        assert_eq!(sync.take_matched(), Some(matched));
    }
    #[test]
    fn refresh_preserves_unsent_edits_and_blocks_external_conflicts() {
        let old = remote();
        let mut sync = CombatSync::default();
        sync.remote = Some(old.clone());
        sync.desired = old.groups.clone();
        sync.desired.get_mut("hp").unwrap()["removedHitPoints"] = 8.into();
        let sent = sync.desired.clone();
        sync.accept(
            Outcome {
                remote: old.clone(),
                blocked: BTreeMap::new(),
            },
            &sent,
        );
        assert_eq!(sync.desired, sent);
        let mut changed = old;
        changed.groups.get_mut("hp").unwrap()["removedHitPoints"] = 12.into();
        sync.accept(
            Outcome {
                remote: changed,
                blocked: BTreeMap::new(),
            },
            &sent,
        );
        assert_eq!(sync.desired, sent);
        assert!(sync.blocked.contains_key("hp"));
    }
    #[test]
    fn render_connected_controls_at_minimum_size() {
        let remote = remote();
        let mut sync = CombatSync::default();
        sync.enabled = true;
        sync.desired = remote.groups.clone();
        sync.remote = Some(remote);
        let ctx = egui::Context::default();
        let before = sync.desired.clone();
        let out = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(700.0, 600.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| sync.show(ui, None));
            },
        );
        assert!(!out.shapes.is_empty());
        assert_eq!(sync.desired, before);
    }
    /// A sync driven as `tick` drives it, frame by frame, with a fake sheet for D&D Beyond. An
    /// exchange runs at once, and its answer lands on the next frame, as a worker's would.
    /// `tick` itself has no route in a test build, so this calls the two parts it is made of.
    struct Rig {
        sync: CombatSync,
        sheet: Fake,
        exchanges: usize,
        /// False: a click on `Log` or on the rail, or another member. The panel is not drawn.
        panel: bool,
        /// A close prompt or a dialog in front.
        off: bool,
        /// Another handler takes Escape before the field is drawn.
        takes_escape: bool,
    }
    const HP_FIELD: &str = "Current HP on the sheet";
    const LOG: &str = "Log";
    fn frame(ui: &mut egui::Ui, rig: &mut Rig) {
        if rig.takes_escape {
            ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape));
        }
        if rig.sync.advance(ui.ctx()) {
            let job = rig.sync.begin_exchange();
            rig.exchanges += 1;
            let result = exchange(
                &mut rig.sheet,
                job.baseline.as_ref(),
                &job.desired,
                &job.blocked,
                job.read_only,
            );
            job.reply.send(result).unwrap();
        }
        // As the tabs do: the click's action runs before the field would be drawn.
        if ui.button(LOG).clicked() {
            rig.panel = false;
        }
        if rig.panel {
            if rig.off {
                ui.disable();
            }
            rig.sync.show(ui, None);
        }
    }
    fn rig() -> egui_kittest::Harness<'static, Rig> {
        let remote = remote();
        let mut sync = CombatSync::default();
        sync.enabled = true;
        sync.desired = remote.groups.clone();
        sync.remote = Some(remote.clone());
        sync.next = Some(Instant::now() + INTERVAL);
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(700.0, 900.0))
            .build_ui_state(
                frame,
                Rig {
                    sync,
                    sheet: Fake::new(remote),
                    exchanges: 0,
                    panel: true,
                    off: false,
                    takes_escape: false,
                },
            );
        harness.run_steps(2);
        harness
    }
    fn pass_time(harness: &mut egui_kittest::Harness<'static, Rig>) {
        harness.state_mut().sync.run_out_timers();
        harness.run_steps(3);
    }
    fn click_hp(harness: &mut egui_kittest::Harness<'static, Rig>) {
        use egui_kittest::kittest::Queryable;
        harness.get_by_label(HP_FIELD).click();
        harness.run_steps(3);
    }
    fn type_text(harness: &mut egui_kittest::Harness<'static, Rig>, text: &str) {
        harness.event(egui::Event::Text(text.to_owned()));
        harness.run_steps(3);
    }
    fn type_21(harness: &mut egui_kittest::Harness<'static, Rig>) {
        click_hp(harness);
        type_text(harness, "2");
        type_text(harness, "1");
    }
    fn removed(harness: &egui_kittest::Harness<'static, Rig>) -> Value {
        harness.state().sync.desired["hp"]["removedHitPoints"].clone()
    }
    fn sheet_damage(harness: &mut egui_kittest::Harness<'static, Rig>, damage: i64) {
        harness
            .state_mut()
            .sheet
            .remote
            .groups
            .get_mut("hp")
            .unwrap()["removedHitPoints"] = damage.into();
    }
    /// After the DM leaves: the number is whole, the hold is over, the usual delay applies, and
    /// then one write goes, with the whole number.
    #[track_caller]
    fn number_goes_out_after_the_delay(harness: &mut egui_kittest::Harness<'static, Rig>) {
        harness.run_steps(3);
        let state = harness.state();
        assert!(!state.sync.in_field(), "the hold ends");
        assert_eq!(removed(harness), 9, "HP is 21");
        assert_eq!(state.sync.state(), SyncState::Pending, "the row says so");
        assert_eq!(state.exchanges, 0, "the delay starts when the DM leaves");
        pass_time(harness);
        let state = harness.state();
        assert_eq!(state.exchanges, 1);
        assert_eq!(state.sheet.writes.len(), 1);
        assert_eq!(state.sheet.writes[0].body["removedHitPoints"], 9);
        assert_eq!(state.sync.state(), SyncState::Matching);
    }
    /// A pause between two digits, far longer than the delay, with the 15-second read due too.
    #[test]
    fn no_exchange_starts_while_the_dm_is_in_the_field() {
        let mut harness = rig();
        click_hp(&mut harness);
        pass_time(&mut harness);
        assert_eq!(
            harness.state().exchanges,
            0,
            "not on focus, with nothing typed"
        );
        type_text(&mut harness, "2");
        assert_eq!(removed(&harness), 28, "the value counts from the first key");
        assert_eq!(harness.state().sync.state(), SyncState::Pending);
        pass_time(&mut harness);
        assert_eq!(harness.state().exchanges, 0, "no exchange after one digit");
        assert!(harness.state().sync.in_field());
        type_text(&mut harness, "1");
        pass_time(&mut harness);
        assert_eq!(harness.state().exchanges, 0, "no exchange after two digits");
        assert!(harness.state().sheet.writes.is_empty());
        assert_eq!(removed(&harness), 9, "HP is 21");
    }
    /// Tab goes to the next field, where the DM is still in a number. The hold ends when the
    /// keyboard reaches something else.
    #[test]
    fn tab_keeps_the_number_and_the_hold_ends_past_the_last_number_field() {
        let mut harness = rig();
        type_21(&mut harness);
        harness.key_press(egui::Key::Tab);
        harness.run_steps(3);
        assert_eq!(removed(&harness), 9, "HP is 21");
        assert!(harness.state().sync.in_field(), "now in Temporary HP");
        pass_time(&mut harness);
        assert_eq!(harness.state().exchanges, 0);
        for _ in 0..3 {
            harness.key_press(egui::Key::Tab);
            harness.run_steps(3);
        }
        number_goes_out_after_the_delay(&mut harness);
    }
    /// Reproduces the order of a click on a tab, on the rail, or on Lookup or Notes where they
    /// hide the panel. The click's action runs first, and the field is not drawn in that frame.
    #[test]
    fn a_click_that_takes_the_field_away_keeps_the_number_and_ends_the_hold() {
        use egui_kittest::kittest::Queryable;
        let mut harness = rig();
        type_21(&mut harness);
        harness.get_by_label(LOG).click();
        number_goes_out_after_the_delay(&mut harness);
        assert!(!harness.state().panel, "the field was not drawn");
    }
    #[test]
    fn a_click_elsewhere_with_the_field_still_drawn_keeps_the_number() {
        use egui_kittest::kittest::Queryable;
        let mut harness = rig();
        type_21(&mut harness);
        harness.get_by_label("Refresh / verify").click();
        harness.run_steps(3);
        assert!(!harness.state().sync.in_field());
        assert_eq!(removed(&harness), 9, "HP is 21");
    }
    /// A whole-number field steps by a quarter and rounds back, so an arrow key changes nothing
    /// and drops nothing, and the hold stands.
    #[test]
    fn an_arrow_key_keeps_the_typed_number() {
        let mut harness = rig();
        type_21(&mut harness);
        harness.key_press(egui::Key::ArrowUp);
        harness.run_steps(3);
        assert_eq!(removed(&harness), 9, "HP is 21");
        assert!(harness.state().sync.in_field());
        harness.key_press(egui::Key::Enter);
        number_goes_out_after_the_delay(&mut harness);
    }
    #[track_caller]
    fn escape_puts_the_value_back(harness: &mut egui_kittest::Harness<'static, Rig>) {
        let before = removed(harness);
        type_21(harness);
        assert_ne!(removed(harness), before);
        harness.key_press(egui::Key::Escape);
        harness.run_steps(3);
        assert_eq!(removed(harness), before, "the value returns");
        assert!(!harness.state().sync.in_field(), "the hold ends");
        assert_eq!(harness.state().sync.state(), SyncState::Matching);
        pass_time(harness);
        assert_eq!(harness.state().exchanges, 1, "the read that waited");
        assert!(harness.state().sheet.writes.is_empty(), "nothing goes out");
    }
    #[test]
    fn escape_puts_the_value_back_and_nothing_goes_out() {
        escape_puts_the_value_back(&mut rig());
    }
    /// `back_out_on_escape`, or Notes or Lookup closing, can take the key before the field is
    /// drawn. egui then sets the typed text as the field is drawn.
    #[test]
    fn escape_puts_the_value_back_when_another_handler_takes_the_key() {
        let mut harness = rig();
        harness.state_mut().takes_escape = true;
        escape_puts_the_value_back(&mut harness);
    }
    /// The hold over, a read finds a changed sheet and a typed number: the existing conflict.
    #[test]
    fn a_changed_sheet_after_the_hold_and_a_typed_number_raise_the_conflict() {
        let mut harness = rig();
        type_21(&mut harness);
        sheet_damage(&mut harness, 12);
        harness.key_press(egui::Key::Enter);
        harness.run_steps(3);
        pass_time(&mut harness);
        let state = harness.state();
        assert_eq!(state.sync.conflict_keys(), vec!["hp"]);
        assert_eq!(state.sync.state(), SyncState::Conflict);
        assert_eq!(state.sync.desired["hp"]["removedHitPoints"], 9, "the DM's");
        assert_eq!(
            state.sync.remote.as_ref().unwrap().groups["hp"]["removedHitPoints"],
            12
        );
        assert!(state.sheet.writes.is_empty());
    }
    /// Nothing typed: the field's old text must not replace what the sheet now has.
    #[test]
    fn a_changed_sheet_with_nothing_typed_is_taken_and_no_write_goes() {
        let mut harness = rig();
        click_hp(&mut harness);
        sheet_damage(&mut harness, 12);
        pass_time(&mut harness);
        assert_eq!(harness.state().exchanges, 0, "the read waits");
        harness.key_press(egui::Key::Enter);
        harness.run_steps(3);
        assert_eq!(removed(&harness), 5, "leaving sets nothing");
        pass_time(&mut harness);
        let state = harness.state();
        assert_eq!(state.exchanges, 1);
        assert_eq!(
            state.sync.desired["hp"]["removedHitPoints"], 12,
            "the sheet's"
        );
        assert!(state.sync.conflict_keys().is_empty());
        assert!(state.sheet.writes.is_empty());
        assert!(!state.sync.dirty());
    }
    /// A worker's answer: a read that found this damage on the sheet.
    fn answer_read(job: Job, damage: i64) {
        let mut changed = remote();
        changed.groups.get_mut("hp").unwrap()["removedHitPoints"] = damage.into();
        job.reply
            .send(Ok(Outcome {
                remote: changed,
                blocked: BTreeMap::new(),
            }))
            .unwrap();
    }
    /// A read on its way as the DM comes in. It lands after they leave, not under the field.
    fn read_on_its_way(harness: &mut egui_kittest::Harness<'static, Rig>, damage: i64) {
        let job = harness.state_mut().sync.begin_exchange();
        sheet_damage(harness, damage);
        // Its words add a row, so wait for the field's new place.
        harness.run_steps(3);
        click_hp(harness);
        answer_read(job, damage);
        harness.run_steps(3);
        assert!(harness.state().sync.in_field());
        let remote = harness.state().sync.remote.as_ref().unwrap();
        assert_eq!(
            remote.groups["hp"]["removedHitPoints"], 5,
            "not applied yet"
        );
    }
    #[test]
    fn a_read_landing_during_focus_with_nothing_typed_is_taken_after_leaving() {
        let mut harness = rig();
        read_on_its_way(&mut harness, 12);
        harness.key_press(egui::Key::Enter);
        harness.run_steps(3);
        assert_eq!(removed(&harness), 12, "the sheet's");
        pass_time(&mut harness);
        let state = harness.state();
        assert!(state.sheet.writes.is_empty());
        assert!(state.sync.conflict_keys().is_empty());
    }
    #[test]
    fn a_read_landing_during_focus_with_a_typed_number_asks() {
        let mut harness = rig();
        read_on_its_way(&mut harness, 12);
        type_text(&mut harness, "2");
        type_text(&mut harness, "1");
        harness.key_press(egui::Key::Enter);
        harness.run_steps(3);
        let state = harness.state();
        assert_eq!(state.sync.conflict_keys(), vec!["hp"]);
        assert_eq!(state.sync.desired["hp"]["removedHitPoints"], 9, "the DM's");
        assert!(state.sheet.writes.is_empty());
    }
    /// Press on the HP field, ready to drag it. Returns where.
    fn press_hp(harness: &mut egui_kittest::Harness<'static, Rig>) -> egui::Pos2 {
        use egui_kittest::kittest::Queryable;
        let start = harness.get_by_label(HP_FIELD).rect().center();
        harness.hover_at(start);
        harness.run_steps(2);
        harness.drag_at(start);
        harness.run_steps(2);
        start
    }
    /// A drag sends at its end, and a pause within it sends nothing.
    #[test]
    fn no_exchange_starts_while_a_field_is_dragged() {
        let mut harness = rig();
        let start = press_hp(&mut harness);
        for step in 1..=3 {
            harness.hover_at(start - egui::vec2(10.0 * step as f32, 0.0));
            harness.run_steps(2);
            assert!(harness.state().sync.in_field(), "step {step}");
            assert_ne!(removed(&harness), 5, "the drag moves the value");
            pass_time(&mut harness);
            assert_eq!(harness.state().exchanges, 0, "step {step}");
        }
        let dragged = removed(&harness);
        harness.drop_at(start - egui::vec2(30.0, 0.0));
        harness.run_steps(3);
        let state = harness.state();
        assert!(!state.sync.in_field(), "the hold ends with the drag");
        assert_eq!(state.exchanges, 0, "the delay starts when the drag ends");
        pass_time(&mut harness);
        let state = harness.state();
        assert_eq!(state.sheet.writes.len(), 1);
        assert_eq!(state.sheet.writes[0].body["removedHitPoints"], dragged);
    }
    /// A read that lands in a drag waits too, and the changed sheet then asks.
    #[test]
    fn a_read_landing_during_a_drag_waits_for_its_end() {
        let mut harness = rig();
        let job = harness.state_mut().sync.begin_exchange();
        harness.run_steps(3);
        let start = press_hp(&mut harness);
        harness.hover_at(start - egui::vec2(20.0, 0.0));
        harness.run_steps(2);
        answer_read(job, 20);
        harness.hover_at(start - egui::vec2(30.0, 0.0));
        harness.run_steps(3);
        let remote = harness.state().sync.remote.as_ref().unwrap();
        assert_eq!(
            remote.groups["hp"]["removedHitPoints"], 5,
            "not applied yet"
        );
        harness.drop_at(start - egui::vec2(30.0, 0.0));
        harness.run_steps(3);
        assert_eq!(harness.state().sync.conflict_keys(), vec!["hp"]);
    }
    /// However long the DM stayed, the delay starts when they leave, with the read long overdue.
    #[test]
    fn the_delay_starts_on_leaving_with_a_read_overdue() {
        let mut harness = rig();
        type_21(&mut harness);
        pass_time(&mut harness);
        harness.key_press(egui::Key::Enter);
        number_goes_out_after_the_delay(&mut harness);
    }
    /// Nothing that ends a sync waits for a focus. A stop does not need the field to end.
    #[test]
    fn stop_and_a_session_end_do_not_wait_for_the_field() {
        let mut harness = rig();
        type_21(&mut harness);
        harness.state_mut().sync.stop();
        assert!(!harness.state().sync.in_field());
        assert_eq!(
            harness.state().sync.state(),
            SyncState::NotSent,
            "the row says so"
        );
        harness.run_steps(3);
        assert_eq!(removed(&harness), 9, "HP is 21");

        let mut harness = rig();
        type_21(&mut harness);
        harness.state_mut().sync.session_ended();
        assert!(!harness.state().sync.in_field());
        assert_eq!(harness.state().sync.state(), SyncState::NotSent);
    }
    /// A close prompt or a dialog turns the field off. egui ends the focus of a field that is
    /// off, so the hold ends, and the number can go while the prompt waits for it.
    #[test]
    fn a_prompt_in_front_of_the_field_ends_the_hold() {
        let mut harness = rig();
        type_21(&mut harness);
        harness.state_mut().off = true;
        number_goes_out_after_the_delay(&mut harness);
    }
    /// With sync off, Party has nothing else to hold an unsent number. Turning sync on asks.
    #[test]
    fn on_party_a_stopped_number_asks_at_the_next_first_read() {
        let mut harness = rig();
        harness.state_mut().sync.without_a_run();
        type_21(&mut harness);
        harness.state_mut().sync.stop();
        let sync = &mut harness.state_mut().sync;
        sync.start();
        sync.accept_read(remote());
        assert_eq!(sync.conflict_keys(), vec!["hp"]);
        assert_eq!(
            sync.values()["hp"]["removedHitPoints"],
            9,
            "the DM's number"
        );
    }
    /// An unsent value belongs to its character, whatever is open next.
    #[test]
    fn another_character_does_not_inherit_an_unsent_value() {
        let mut sync = CombatSync::simulated_state(SyncState::Matching, 11, 10, 0);
        sync.scope = Some((1, "1".into(), "11".into()));
        sync.simulate_slot_edit();
        sync.enter_scope((1, "1".into(), "12".into()));
        assert!(sync.unsent.is_empty());
        let mut sync = CombatSync::simulated_state(SyncState::Matching, 11, 10, 0);
        sync.scope = Some((1, "1".into(), "11".into()));
        sync.simulate_slot_edit();
        sync.enter_scope((2, "1".into(), "11".into()));
        assert!(!sync.unsent.is_empty(), "a new session keeps it");
    }
    /// A read that starts or lands must not take the field the DM is typing in away.
    #[test]
    fn a_read_starting_keeps_the_field_the_dm_is_typing_in() {
        use egui_kittest::{Harness, kittest::Queryable};
        let remote = remote();
        let mut sync = CombatSync::default();
        sync.enabled = true;
        sync.desired = remote.groups.clone();
        sync.remote = Some(remote);
        let mut harness = Harness::builder()
            .with_size(egui::vec2(700.0, 600.0))
            .build_ui_state(|ui, sync: &mut CombatSync| sync.show(ui, None), sync);
        harness.get_by_label(HP_FIELD).click();
        harness.run_steps(3);
        harness.event(egui::Event::Text("2".to_owned()));
        harness.run_steps(3);
        let (_keep_open, receiver) = mpsc::channel();
        let sent = harness.state().desired.clone();
        harness.state_mut().pending = Some(Pending { receiver, sent });
        harness.run_steps(3);
        harness.event(egui::Event::Text("1".to_owned()));
        harness.run_steps(3);
        harness.key_press(egui::Key::Enter);
        harness.run_steps(3);
        assert_eq!(
            harness.state().desired["hp"]["removedHitPoints"],
            9,
            "HP is 21"
        );
    }
    /// While sync is stopped, neither choice can do what it says, so each is off with a reason.
    #[test]
    fn a_conflict_cannot_be_settled_while_sync_is_stopped() {
        let mut sync = CombatSync::simulated_conflict(11, 10, 1);
        assert_eq!(sync.state(), SyncState::Conflict);
        assert_eq!(sync.choice_blocked(STOPPED_SYNC_AGAIN), None);
        sync.stop();
        assert_eq!(sync.state(), SyncState::Conflict, "the conflict stays");
        assert_eq!(
            sync.choice_blocked(STOPPED_CONNECT_FIRST),
            Some(STOPPED_CONNECT_FIRST)
        );
    }
    /// Spell slots are in no saved run. With sync on or off, closing the app loses an unsent one,
    /// and only then does the close say so.
    #[test]
    fn only_values_the_run_does_not_hold_are_lost_on_close() {
        let mut sync = CombatSync::simulated_state(SyncState::Pending, 11, 10, 0);
        assert!(!sync.loses_on_close(), "HP is kept by the run");
        sync.simulate_slot_edit();
        assert!(sync.loses_on_close());
        sync.stop();
        assert!(sync.loses_on_close());
    }
}
