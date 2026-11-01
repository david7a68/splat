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
        let read = Remote::from_raw(
            character,
            max_hp,
            &json!({"id":character,"canEdit":state != SyncState::ReadOnly,"removedHitPoints":damage,"temporaryHitPoints":0,"deathSaves":{"successCount":0,"failCount":0},"conditions":[]}),
        )
        .expect("simulated sheet");
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
        let read = Remote::from_raw(
            character,
            max_hp,
            &json!({"id":character,"canEdit":true,"removedHitPoints":damage,"temporaryHitPoints":0,"deathSaves":{"successCount":0,"failCount":0},"conditions":[]}),
        )
        .expect("simulated sheet");
        let mut sync = Self::default();
        sync.enabled = true;
        sync.simulated = true;
        sync.remote = Some(read.clone());
        sync.desired = read.groups.clone();
        sync.desired.get_mut("hp").expect("HP group")["removedHitPoints"] = (damage + 4).into();
        let sent = sync.desired.clone();
        let mut changed = read;
        changed.groups.get_mut("hp").expect("HP group")["removedHitPoints"] = (damage + 9).into();
        sync.accept(
            Outcome {
                remote: changed,
                blocked: BTreeMap::new(),
            },
            &sent,
        );
        sync
    }

    pub fn stop(&mut self) {
        if let Some(gate) = &self.gate {
            gate.store(false, Ordering::SeqCst);
        }
        self.pending = None;
        self.gate = None;
        self.enabled = false;
    }
    pub fn ready(&self) -> bool {
        self.enabled && self.remote.is_some()
    }
    pub fn state(&self) -> SyncState {
        if !self.blocked.is_empty() {
            SyncState::Conflict
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
        self.remote = None;
        self.desired.clear();
        self.blocked.clear();
        self.next = None;
        self.error = None;
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
    pub fn tick(
        &mut self,
        ctx: &egui::Context,
        session: Option<Session>,
        epoch: u64,
        campaign: &str,
        id: &str,
    ) {
        #[cfg(test)]
        if self.simulated {
            return;
        }
        let scope = (epoch, campaign.to_owned(), id.to_owned());
        if self.scope.as_ref() != Some(&scope) {
            let enabled = self.enabled;
            self.stop();
            *self = Self::default();
            self.enabled = enabled;
            self.scope = Some(scope);
        }
        if !self.enabled || session.is_none() {
            if self.pending.is_some() || session.is_none() {
                self.stop();
            }
            return;
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
        let pending_edits = self.dirty()
            && self.desired.iter().any(|(k, v)| {
                !self.blocked.contains_key(k)
                    && self.remote.as_ref().and_then(|r| r.groups.get(k)) != Some(v)
            });
        let due = self.next.is_none_or(|t| t <= Instant::now())
            || (pending_edits
                && self
                    .changed
                    .is_none_or(|t| t.elapsed() >= Duration::from_millis(500)));
        if self.pending.is_none() && due {
            let baseline = self.remote.clone();
            let desired = self.desired.clone();
            let blocked = self.blocked.keys().cloned().collect();
            let read_only = std::mem::take(&mut self.refresh_only);
            let id = id.to_owned();
            let campaign = campaign.to_owned();
            let session = session.unwrap();
            let gate = Arc::new(AtomicBool::new(true));
            self.gate = Some(gate.clone());
            let (tx, rx) = mpsc::channel();
            self.pending = Some(Pending {
                receiver: rx,
                sent: desired.clone(),
            });
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
                        baseline.as_ref(),
                        &desired,
                        &blocked,
                        read_only,
                    )
                })();
                let _ = tx.send(result);
                ctx.request_repaint();
            });
        }
        ctx.request_repaint_after(Duration::from_millis(500));
    }
    fn accept(&mut self, outcome: Outcome, sent: &Groups) {
        let initial = self.remote.is_none();
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
                    "The live sheet and local values both changed. Choose which to keep.".into(),
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
        self.remote = Some(outcome.remote);
        self.error = None;
        self.updated = true;
    }
    /// Each conflict with both values and its two ways out. The run shows this beside
    /// the roster, so it does not stop the table.
    pub fn conflicts(&mut self, ui: &mut egui::Ui) {
        use crate::design::{self, Action, Density, Tone, Type, colors};
        for (key, message) in self.blocked.clone() {
            design::message(
                ui,
                Tone::Warning,
                &format!("{}: {message}", group_label(&key)),
            );
            for (label, value) in [
                ("Splat", self.desired.get(&key)),
                (
                    "D&D Beyond",
                    self.remote.as_ref().and_then(|r| r.groups.get(&key)),
                ),
            ] {
                ui.label(
                    Type::Body
                        .rich(
                            ui,
                            format!(
                                "{label}: {}",
                                describe(&key, value, self.max_hp().unwrap_or(0))
                            ),
                        )
                        .color(colors().text),
                );
            }
            ui.push_id(&key, |ui| {
                ui.horizontal_wrapped(|ui| {
                    let busy = self.busy().then_some("Wait for the current sheet read.");
                    if Action::secondary("Use D&D Beyond values")
                        .density(Density::Read)
                        .blocked(busy)
                        .show(ui)
                        .clicked()
                    {
                        if let Some(value) = self.remote.as_ref().and_then(|r| r.groups.get(&key)) {
                            self.desired.insert(key.clone(), value.clone());
                        } else {
                            self.desired.remove(&key);
                        }
                        self.blocked.remove(&key);
                    }
                    let writable = self
                        .remote
                        .as_ref()
                        .is_some_and(|r| r.can_edit && r.groups.contains_key(&key));
                    if Action::quiet("Keep Splat values and retry")
                        .density(Density::Read)
                        .blocked(busy.or(
                            (!writable).then_some(
                                "This session cannot edit this character on D&D Beyond.",
                            ),
                        ))
                        .show(ui)
                        .clicked()
                    {
                        self.blocked.remove(&key);
                        self.changed = None;
                        self.next = None;
                    }
                });
            });
        }
    }
    pub fn show(&mut self, ui: &mut egui::Ui) {
        let was = self.enabled;
        ui.checkbox(&mut self.enabled, "Sync combat stats with D&D Beyond");
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
            ui.weak("Enable to read the live sheet. Edits made here will then update D&D Beyond.");
            return;
        }
        ui.weak("HP, death saves, conditions, spell slots, and supported limited uses · checks every 15 seconds");
        if self.busy() {
            ui.horizontal(|ui| {
                ui.spinner();
                ui.label("Checking character…");
            });
        }
        if let Some(e) = &self.error {
            ui.colored_label(crate::design::colors().result.error, e);
        }
        ui.horizontal(|ui| {
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
                ui.label("Local combat changes pending");
            } else if self.remote.is_some() {
                ui.label("Combat values match the last read");
            }
        });
        self.conflicts(ui);
        let Some(remote) = &self.remote else {
            ui.label("Connect in Settings and wait for the first sheet read.");
            return;
        };
        if !remote.can_edit {
            ui.label("Read only: D&D Beyond reports that this session cannot edit this character.");
        }
        let max_hp = remote.max_hp;
        let labels = remote.labels.clone();
        let writable = remote.can_edit;
        let before = self.desired.clone();
        ui.add_enabled_ui(writable, |ui| {
            if let Some(hp) = self.desired.get_mut("hp") {
                ui.horizontal_wrapped(|ui| {
                    let mut current =
                        (max_hp - hp["removedHitPoints"].as_i64().unwrap_or(0)).max(0);
                    ui.label(format!("HP / {max_hp}"));
                    if ui
                        .add(egui::DragValue::new(&mut current).range(0..=max_hp))
                        .changed()
                    {
                        hp["removedHitPoints"] = (max_hp - current).into();
                    }
                    let mut temp = hp["temporaryHitPoints"].as_i64().unwrap_or(0);
                    ui.label("Temporary HP");
                    if ui
                        .add(egui::DragValue::new(&mut temp).range(0..=1_000_000))
                        .changed()
                    {
                        hp["temporaryHitPoints"] = temp.into();
                    }
                });
            }
            if let Some(death) = self.desired.get_mut("death") {
                ui.horizontal_wrapped(|ui| {
                    ui.label("Death saves");
                    for (field, label) in [("successCount", "Successes"), ("failCount", "Failures")]
                    {
                        let mut n = death[field].as_i64().unwrap_or(0);
                        ui.label(label);
                        if ui.add(egui::DragValue::new(&mut n).range(0..=3)).changed() {
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
                                if ui
                                    .add(egui::DragValue::new(&mut level).range(1..=6))
                                    .changed()
                                {
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
                                if ui
                                    .add(egui::DragValue::new(&mut used).range(0..=99))
                                    .changed()
                                {
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
                                if ui
                                    .add(egui::DragValue::new(&mut used).range(0..=1_000_000))
                                    .changed()
                                {
                                    resource["uses"] = used.into();
                                }
                            });
                        });
                    }
                }
            });
        });
        if self.desired != before {
            self.changed = Some(Instant::now());
        }
        ui.weak("Concentration, custom effects, inventory charges, initiative, and DM notes stay local.");
    }
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
fn group_label(key: &str) -> &str {
    match key {
        "hp" => "HP / temporary HP",
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
                egui::CentralPanel::default().show(ctx, |ui| sync.show(ui));
            },
        );
        assert!(!out.shapes.is_empty());
        assert_eq!(sync.desired, before);
    }
}
