//! Read-only API transport and visibility-scoped synchronization for the Party viewer.
use crate::auth::Session;
use eframe::egui;
use reqwest::{
    blocking::{Client, Response},
    redirect::Policy,
};
use serde::Deserialize;
use serde_json::Value;
use splat::character::{CampaignCopyError, EntitySnapshot, now};
use std::{
    collections::{BTreeMap, BTreeSet},
    io::Read,
    path::PathBuf,
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver},
    },
    thread,
    time::{Duration, Instant},
};
use uuid::Uuid;
const MAX_RESPONSE: u64 = 16 * 1024 * 1024;
const INTERVAL: Duration = Duration::from_secs(15);
#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    #[serde(deserialize_with = "crate::dndbeyond::campaign_id")]
    pub id: String,
    pub name: String,
    pub level: Option<u64>,
}
#[derive(Debug)]
pub struct Failure {
    pub message: String,
    pub retry_after: Duration,
}
impl From<String> for Failure {
    fn from(message: String) -> Self {
        Self {
            message,
            retry_after: Duration::from_secs(60),
        }
    }
}
impl From<&str> for Failure {
    fn from(message: &str) -> Self {
        message.to_owned().into()
    }
}
fn response(response: Response) -> Result<Vec<u8>, Failure> {
    let retry = response
        .headers()
        .get("Retry-After")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(60)
        .max(15);
    match response.status().as_u16() {
        200..=299 => {}
        401 | 403 => {
            return Err(
                "Access denied. Reconnect in Settings and check character sharing on D&D Beyond."
                    .into(),
            );
        }
        404 => return Err("The character or campaign is no longer available.".into()),
        429 => {
            return Err(Failure {
                message: "D&D Beyond is limiting requests. Updates will retry after a delay."
                    .into(),
                retry_after: Duration::from_secs(retry),
            });
        }
        _ => {
            return Err(
                "D&D Beyond could not complete the request. The local copy was kept.".into(),
            );
        }
    }
    let mut bytes = Vec::new();
    response
        .take(MAX_RESPONSE + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| Failure::from("Could not read the D&D Beyond response."))?;
    if bytes.len() as u64 > MAX_RESPONSE {
        return Err("The D&D Beyond response is too large.".into());
    }
    Ok(bytes)
}
fn client() -> Result<Client, Failure> {
    Client::builder()
        .timeout(Duration::from_secs(20))
        .redirect(Policy::none())
        .user_agent("Splat/0.1")
        .build()
        .map_err(|_| "Could not start the D&D Beyond client.".into())
}
fn get(client: &Client, session: &Session, url: &str, website: bool) -> Result<Vec<u8>, Failure> {
    let request = client.get(url).header("Accept", "application/json");
    let request = if website {
        session.website_request(request)?
    } else {
        session.authorize(request)
    };
    response(
        request
            .send()
            .map_err(|_| Failure::from("Could not reach D&D Beyond. The local copy was kept."))?,
    )
}
pub fn roster(bytes: &[u8]) -> Result<Vec<Member>, Failure> {
    #[derive(Deserialize)]
    struct Envelope {
        status: String,
        data: Vec<Member>,
    }
    let v: Envelope = serde_json::from_slice(bytes)
        .map_err(|_| Failure::from("Unexpected D&D Beyond party response."))?;
    let mut ids = std::collections::HashSet::new();
    if v.status != "success"
        || v.data.len() > 100
        || v.data
            .iter()
            .any(|m| m.name.trim().is_empty() || !ids.insert(&m.id))
    {
        return Err("Invalid D&D Beyond party list.".into());
    }
    Ok(v.data)
}
fn computed(bytes: &[u8], id: &str) -> Result<Value, Failure> {
    let v: Value = serde_json::from_slice(bytes)
        .map_err(|_| Failure::from("Unexpected computed-character response."))?;
    let found = v["foundCharacters"]
        .as_array()
        .ok_or("D&D Beyond omitted computed characters.")?;
    let matches = found
        .iter()
        .filter(|v| v["characterId"].as_u64().map(|n| n.to_string()).as_deref() == Some(id))
        .collect::<Vec<_>>();
    if matches.len() == 1 {
        return Ok(matches[0].clone());
    }
    if splat::character::list(&v["queuedIds"])
        .iter()
        .any(|v| v.as_u64().map(|n| n.to_string()).as_deref() == Some(id))
    {
        return Err(Failure {
            message:
                "D&D Beyond is preparing the computed sheet. Keeping the local copy and retrying."
                    .into(),
            retry_after: INTERVAL,
        });
    }
    Err("D&D Beyond did not return this computed character. Check access and refresh.".into())
}
pub fn fetch(
    client: &Client,
    session: &Session,
    id: &str,
    campaign: &str,
) -> Result<EntitySnapshot, Failure> {
    if id.parse::<u64>().ok().is_none_or(|n| n == 0) || !id.bytes().all(|b| b.is_ascii_digit()) {
        return Err("Invalid character ID.".into());
    }
    let raw: Value = serde_json::from_slice(&get(
        client,
        session,
        &format!("https://character-service.dndbeyond.com/character/v5/character/{id}"),
        false,
    )?)
    .map_err(|_| Failure::from("Unexpected character response."))?;
    if raw["success"] != true {
        return Err("D&D Beyond could not read this character.".into());
    }
    let raw_time = now();
    // SCDS uses POST for a read query. This endpoint cannot edit a character.
    let bytes=response(session.authorize(client.post("https://character-service-scds.dndbeyond.com/v2/characters")).json(&serde_json::json!({"characterIds":[id.parse::<u64>().map_err(|_|Failure::from("Invalid ID."))?]})).send().map_err(|_|Failure::from("Could not reach computed-character service."))?)?;
    Ok(EntitySnapshot::from_beyond(
        raw["data"].clone(),
        computed(&bytes, id)?,
        campaign,
        id,
        raw_time,
        now(),
    )?)
}
#[derive(Clone, PartialEq)]
struct Scope {
    epoch: u64,
    local: Uuid,
    remote: String,
    path: std::path::PathBuf,
    connected: bool,
}
enum Event {
    Cache(Result<Vec<EntitySnapshot>, String>),
    Roster(Result<Vec<Member>, Failure>),
    Sheet(String, Box<Result<EntitySnapshot, Failure>>),
    CampaignCopy(String, Result<Vec<PathBuf>, CampaignCopyError>),
    Stored(Stored, Result<(), String>),
}
/// What a local storage warning is about. The next success for the same thing clears it.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stored {
    SavedSheets,
    Roster,
    Sheet(String),
    CampaignCopy(String),
}
fn save_campaign_copy(
    record: &EntitySnapshot,
    path: &std::path::Path,
    force: bool,
) -> Result<Vec<PathBuf>, CampaignCopyError> {
    if force {
        record.overwrite_campaign_copy(path)
    } else {
        record.save_campaign_copy(path).map(|_| Vec::new())
    }
}
#[derive(Default)]
pub struct Reader {
    scope: Option<Scope>,
    pending: Option<Receiver<Event>>,
    gate: Arc<Mutex<bool>>,
    pub members: Vec<Member>,
    pub sheets: BTreeMap<String, EntitySnapshot>,
    pub errors: BTreeMap<String, String>,
    pub error: Option<String>,
    pub warnings: BTreeMap<Stored, String>,
    /// Characters whose campaign notes have local edits, each with the note that stopped its save.
    pub conflicts: BTreeMap<String, PathBuf>,
    /// Archives that hold edits a force read replaced. They stay listed until dismissed.
    pub archived: Vec<PathBuf>,
    /// Characters whose next read replaces those edits. The DM asked for this.
    force: BTreeSet<String>,
    /// The characters that the running read replaces edits for.
    forcing: BTreeSet<String>,
    pub selected: Option<String>,
    pub loaded: bool,
    pub online_ids: std::collections::BTreeSet<String>,
    stream: Option<crate::party_stream::Stream>,
    pub stream_open: bool,
    reconnect_at: Option<Instant>,
    stream_failures: u32,
    refresh_requested: bool,
    followup: Option<Instant>,
    last_check: Option<Instant>,
    cooldown: Option<Instant>,
    next: Option<Instant>,
    last_roster: Option<Instant>,
    visible: bool,
    backoff: u32,
    cache_loaded: bool,
}
impl Drop for Reader {
    fn drop(&mut self) {
        if let Ok(mut gate) = self.gate.lock() {
            *gate = false;
        }
    }
}
impl Reader {
    pub fn sync(
        &mut self,
        ctx: &egui::Context,
        epoch: u64,
        campaign: Option<(Uuid, String, std::path::PathBuf)>,
        connected: bool,
        visible: bool,
    ) {
        let scope = campaign.map(|(local, remote, path)| Scope {
            epoch,
            local,
            remote,
            path,
            connected,
        });
        if scope != self.scope {
            if let Ok(mut gate) = self.gate.lock() {
                *gate = false;
            }
            let mut next = Self::default();
            next.scope = scope;
            next.gate = Arc::new(Mutex::new(true));
            *self = next;
            self.load_cache(ctx);
        }
        if !visible && self.visible {
            if let Ok(mut gate) = self.gate.lock() {
                *gate = false;
            }
            self.stream = None;
            self.stream_open = false;
            self.pending = None;
            self.forcing.clear();
            self.gate = Arc::new(Mutex::new(true));
        }
        if visible && !self.visible {
            self.next = None;
            self.refresh_requested = true;
        }
        self.visible = visible;
        self.poll();
    }
    pub fn local_scope(&self) -> Option<Uuid> {
        self.scope.as_ref().map(|s| s.local)
    }
    pub fn campaign_path(&self) -> Option<&std::path::Path> {
        self.scope.as_ref().map(|s| s.path.as_path())
    }
    pub fn loading(&self) -> bool {
        self.pending.is_some()
    }
    fn load_cache(&mut self, ctx: &egui::Context) {
        let Some(scope) = self.scope.clone() else {
            return;
        };
        let (tx, rx) = mpsc::channel();
        self.pending = Some(rx);
        let ctx = ctx.clone();
        let gate = self.gate.clone();
        thread::spawn(move || {
            let result = crate::hermetic::character_cache()
                .and_then(|c| c.load_all(scope.local, &scope.remote));
            if let Ok(records) = &result {
                for record in records {
                    if let Ok(active) = gate.lock() {
                        if !*active {
                            break;
                        }
                        let _ = tx.send(Event::CampaignCopy(
                            record.source.character_id.clone(),
                            save_campaign_copy(record, &scope.path, false),
                        ));
                    }
                }
            }
            let _ = tx.send(Event::Cache(result));
            ctx.request_repaint();
        });
    }
    pub fn tick(&mut self, session: Option<Session>, ctx: &egui::Context) {
        self.poll();
        if !self.cache_loaded && self.pending.is_none() {
            self.load_cache(ctx);
            return;
        }
        if !self.visible {
            return;
        }
        if let Some(session) = session.as_ref() {
            if self.stream.is_none()
                && self.reconnect_at.is_none_or(|t| t <= Instant::now())
                && let Some(scope) = &self.scope
            {
                self.stream = Some(crate::party_stream::Stream::start(
                    session.clone(),
                    scope.remote.clone(),
                    ctx,
                ));
            }
            let mut closed = false;
            if let Some(stream) = &self.stream {
                while let Ok(event) = stream.rx.try_recv() {
                    match event {
                        crate::party_stream::Event::Open => {
                            self.stream_open = true;
                            self.stream_failures = 0;
                            self.refresh_requested = true;
                            self.next = None;
                        }
                        crate::party_stream::Event::Closed => {
                            closed = true;
                            self.stream_open = false;
                            self.stream_failures = self.stream_failures.saturating_add(1);
                            self.reconnect_at = Some(
                                Instant::now()
                                    + Duration::from_secs(2u64.pow(self.stream_failures.min(5))),
                            );
                        }
                        crate::party_stream::Event::Changed(id) => {
                            if self.selected.as_ref() == Some(&id) || id == "*" {
                                self.refresh_requested = true;
                                self.next = Some(Instant::now() + Duration::from_millis(750));
                                self.followup = Some(Instant::now() + Duration::from_secs(5));
                            } else if !self.members.iter().any(|m| m.id == id) {
                                self.last_roster = None;
                                self.refresh_requested = true;
                                self.next = None;
                            }
                        }
                    }
                }
            }
            if closed {
                self.stream = None;
            }
        }
        if self.followup.is_some_and(|t| t <= Instant::now()) {
            self.followup = None;
            self.refresh_requested = true;
            self.next = None;
        }
        if !self.wants_refresh(Instant::now()) {
            return;
        }
        if let Some(session) = session {
            let roster = self
                .last_roster
                .is_none_or(|t| t.elapsed() >= Duration::from_secs(60));
            self.refresh(session, ctx, roster);
        }
    }
    fn wants_refresh(&self, now: Instant) -> bool {
        let safety_due = self
            .last_check
            .is_none_or(|t| now.duration_since(t) >= Duration::from_secs(300));
        self.visible
            && self.pending.is_none()
            && self.cooldown.is_none_or(|t| t <= now)
            && self.next.is_none_or(|t| t <= now)
            && (self.refresh_requested || !self.stream_open || safety_due)
    }
    pub fn refresh(&mut self, session: Session, ctx: &egui::Context, full: bool) {
        // Test builds have no route to D&D Beyond. A signed-in test keeps its saved sheets.
        if cfg!(test) {
            self.refresh_requested = false;
            self.last_check = Some(Instant::now());
            self.next = Some(Instant::now() + INTERVAL);
            for id in std::mem::take(&mut self.force) {
                if let Some(record) = self.sheets.get(&id).cloned()
                    && let Some(path) = self.campaign_path()
                {
                    let saved = save_campaign_copy(&record, path, true);
                    self.campaign_copy_saved(id, saved);
                }
            }
            return;
        }
        if self.pending.is_some() || self.cooldown.is_some_and(|t| t > Instant::now()) {
            return;
        }
        let Some(scope) = self.scope.clone() else {
            return;
        };
        let selected = self.selected.clone();
        let full = full || selected.is_none();
        let force = std::mem::take(&mut self.force);
        self.forcing = force.clone();
        let (tx, rx) = mpsc::channel();
        self.pending = Some(rx);
        self.error = None;
        self.refresh_requested = false;
        self.last_check = Some(Instant::now());
        self.next = Some(Instant::now() + INTERVAL);
        let ctx = ctx.clone();
        let gate = self.gate.clone();
        thread::spawn(move || {
            let run = || -> Result<(), Failure> {
                let client = client()?;
                let members = if full {
                    let members = roster(&get(
                        &client,
                        &session,
                        &format!(
                            "https://www.dndbeyond.com/api/campaign/characters/{}",
                            scope.remote
                        ),
                        true,
                    )?)?;
                    if tx.send(Event::Roster(Ok(members.clone()))).is_err() {
                        return Ok(());
                    }
                    ctx.request_repaint();
                    // The authoritative roster prevents removed members from returning offline.
                    if let Ok(active) = gate.lock()
                        && *active
                    {
                        let saved = crate::hermetic::character_cache().and_then(|c| {
                            c.save_roster(
                                scope.local,
                                &scope.remote,
                                &members.iter().map(|m| m.id.clone()).collect::<Vec<_>>(),
                            )
                        });
                        let _ = tx.send(Event::Stored(Stored::Roster, saved));
                    }
                    members.into_iter().map(|m| m.id).collect::<Vec<_>>()
                } else {
                    force
                        .iter()
                        .cloned()
                        .chain(selected)
                        .collect::<BTreeSet<_>>()
                        .into_iter()
                        .collect()
                };
                for id in members {
                    if gate.lock().map_or(true, |active| !*active) {
                        break;
                    }
                    let result = fetch(&client, &session, &id, &scope.remote);
                    let failure = result
                        .as_ref()
                        .err()
                        .is_some_and(|e| e.message.contains("limiting requests"));
                    if let Ok(active) = gate.lock() {
                        if !*active {
                            break;
                        }
                        if let Ok(record) = &result {
                            let saved = crate::hermetic::character_cache()
                                .and_then(|c| c.save(scope.local, record))
                                .map(|_| ());
                            let _ = tx.send(Event::Stored(Stored::Sheet(id.clone()), saved));
                            let saved =
                                save_campaign_copy(record, &scope.path, force.contains(&id));
                            let _ = tx.send(Event::CampaignCopy(id.clone(), saved));
                        }
                    }
                    if tx.send(Event::Sheet(id, Box::new(result))).is_err() {
                        break;
                    }
                    ctx.request_repaint();
                    if failure {
                        break;
                    } // Do not hammer an unavailable or rate-limited service.
                }
                Ok(())
            };
            if let Err(e) = run() {
                let _ = tx.send(Event::Roster(Err(e)));
                ctx.request_repaint();
            }
        });
    }
    fn poll(&mut self) {
        while let Some(rx) = &self.pending {
            match rx.try_recv() {
                Ok(Event::Cache(result)) => {
                    self.cache_loaded = true;
                    self.refresh_requested = true;
                    self.next = None;
                    match result {
                        Ok(records) => {
                            for record in records {
                                let id = record.source.character_id.clone();
                                self.members.push(member(&record));
                                self.sheets.insert(id, record);
                            }
                            self.selected = self.members.first().map(|m| m.id.clone());
                        }
                        Err(e) => self.stored(Stored::SavedSheets, Err(e)),
                    }
                }
                Ok(Event::Roster(Ok(members))) => {
                    self.sheets
                        .retain(|id, _| members.iter().any(|m| &m.id == id));
                    self.errors
                        .retain(|id, _| members.iter().any(|m| &m.id == id));
                    self.online_ids
                        .retain(|id| members.iter().any(|m| &m.id == id));
                    self.conflicts
                        .retain(|id, _| members.iter().any(|m| &m.id == id));
                    self.warnings.retain(|what, _| match what {
                        Stored::Sheet(id) | Stored::CampaignCopy(id) => {
                            members.iter().any(|m| &m.id == id)
                        }
                        Stored::SavedSheets | Stored::Roster => true,
                    });
                    if !members
                        .iter()
                        .any(|m| Some(&m.id) == self.selected.as_ref())
                    {
                        self.selected = members.first().map(|m| m.id.clone());
                    }
                    self.members = members;
                    self.loaded = true;
                    self.last_roster = Some(Instant::now());
                }
                Ok(Event::Roster(Err(e))) => {
                    self.next = Some(
                        Instant::now()
                            + e.retry_after
                                .max(Duration::from_secs(15 * 2u64.pow(self.backoff.min(5)))),
                    );
                    self.backoff = self.backoff.saturating_add(1);
                    self.cooldown = self.next;
                    self.error = Some(e.message);
                    self.loaded = true;
                }
                Ok(Event::Sheet(id, result)) => match *result {
                    Ok(record) => {
                        self.errors.remove(&id);
                        self.online_ids.insert(id.clone());
                        if let Some(m) = self.members.iter_mut().find(|m| m.id == id) {
                            *m = member(&record);
                        }
                        self.sheets.insert(id, record);
                        self.backoff = 0;
                    }
                    Err(e) => {
                        self.next = Some(
                            Instant::now()
                                + e.retry_after
                                    .max(Duration::from_secs(15 * 2u64.pow(self.backoff.min(5)))),
                        );
                        self.backoff = self.backoff.saturating_add(1);
                        self.cooldown = self.next;
                        self.errors.insert(id, e.message);
                    }
                },
                Ok(Event::CampaignCopy(id, result)) => self.campaign_copy_saved(id, result),
                Ok(Event::Stored(what, result)) => self.stored(what, result),
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.pending = None;
                    self.forcing.clear();
                    if !self.refresh_requested && self.next.is_none_or(|t| t < Instant::now()) {
                        self.next = Some(Instant::now() + INTERVAL);
                    }
                }
            }
        }
    }
    pub fn stored(&mut self, what: Stored, result: Result<(), String>) {
        match result {
            Ok(()) => {
                self.warnings.remove(&what);
            }
            Err(message) => {
                self.warnings.insert(what, message);
            }
        }
    }
    /// Saves a sheet that came from elsewhere, such as combat sync, and records the result.
    pub fn save_campaign_copy(&mut self, id: &str, record: &EntitySnapshot) {
        if let Some(path) = self.campaign_path() {
            let saved = save_campaign_copy(record, path, false);
            self.campaign_copy_saved(id.to_owned(), saved);
        }
    }
    /// A saved note clears its character's warnings, so moving a note aside clears them too.
    fn campaign_copy_saved(&mut self, id: String, result: Result<Vec<PathBuf>, CampaignCopyError>) {
        let what = Stored::CampaignCopy(id.clone());
        match result {
            Ok(archived) => {
                self.conflicts.remove(&id);
                self.archived.extend(archived);
                self.stored(what, Ok(()));
            }
            Err(CampaignCopyError::Conflict(path)) => {
                self.conflicts.insert(id, path);
                self.stored(what, Ok(()));
            }
            Err(CampaignCopyError::Failed(message)) => self.stored(what, Err(message)),
        }
    }
    /// Reads every character with a conflict again, and replaces its edits with that read.
    /// It waits its turn behind a running refresh or a rate limit.
    pub fn force_read(&mut self) {
        self.force.extend(self.conflicts.keys().cloned());
        self.refresh_requested = true;
        self.next = None;
    }
    pub fn force_waiting(&self) -> bool {
        !self.force.is_empty() || !self.forcing.is_empty()
    }
    pub fn member_name<'a>(&'a self, id: &'a str) -> &'a str {
        self.members
            .iter()
            .find(|m| m.id == id)
            .map_or(id, |m| m.name.as_str())
    }
    pub fn select(&mut self, id: String) {
        if self.selected.as_ref() != Some(&id) {
            self.selected = Some(id);
            self.next = None;
            self.refresh_requested = true;
        }
    }
}
fn member(record: &EntitySnapshot) -> Member {
    Member {
        id: record.source.character_id.clone(),
        name: record.definition.splat_name.clone(),
        level: Some(record.character.level),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn roster_rejects_duplicate_and_invalid_ids() {
        assert!(
            roster(br#"{"status":"success","data":[]}"#)
                .unwrap()
                .is_empty()
        );
        assert!(
            roster(br#"{"status":"success","data":[{"id":1,"name":"A"},{"id":"1","name":"B"}]}"#)
                .is_err()
        );
        assert!(roster(br#"{"status":"success","data":[{"id":"../x","name":"A"}]}"#).is_err());
    }
    #[test]
    fn queued_and_missing_computed_sheets_are_errors_not_empty_characters() {
        assert!(
            computed(br#"{"foundCharacters":[],"queuedIds":[12]}"#, "12")
                .unwrap_err()
                .message
                .contains("preparing")
        );
        assert!(computed(br#"{"foundCharacters":[],"notFoundIds":[12]}"#, "12").is_err());
        assert!(computed(br#"{"foundCharacters":[{"characterId":13}]}"#, "12").is_err());
    }
    #[test]
    fn hiding_or_switching_campaign_closes_workers_and_discards_late_replies() {
        let mut reader = Reader::default();
        let ctx = egui::Context::default();
        let id = Uuid::new_v4();
        reader.scope = Some(Scope {
            epoch: 1,
            local: id,
            remote: "12".into(),
            path: "/unused".into(),
            connected: true,
        });
        reader.visible = true;
        reader.gate = Arc::new(Mutex::new(true));
        let gate = reader.gate.clone();
        let (tx, rx) = mpsc::channel();
        reader.pending = Some(rx);
        reader.sync(
            &ctx,
            1,
            Some((id, "12".into(), "/unused".into())),
            true,
            false,
        );
        assert!(!*gate.lock().unwrap());
        assert!(
            tx.send(Event::Stored(Stored::Roster, Err("late".into())))
                .is_err()
        );
        assert!(!reader.visible);
    }
    #[test]
    fn a_warning_stays_until_the_same_thing_saves_and_force_reads_at_once() {
        let mut reader = Reader::default();
        reader.visible = true;
        reader.stream_open = true;
        reader.last_check = Some(Instant::now());
        let path = PathBuf::from("/campaign/note.md");
        reader.campaign_copy_saved("12".into(), Err(CampaignCopyError::Conflict(path.clone())));
        reader.campaign_copy_saved(
            "13".into(),
            Err(CampaignCopyError::Failed("Disk full".into())),
        );
        reader.stored(Stored::Roster, Err("Roster unsaved".into()));
        assert_eq!(reader.conflicts.keys().collect::<Vec<_>>(), ["12"]);
        assert_eq!(reader.warnings.len(), 2);

        reader.next = Some(Instant::now() + Duration::from_secs(600));
        assert!(!reader.wants_refresh(Instant::now()));
        reader.force_read();
        assert!(reader.force_waiting());
        assert!(reader.wants_refresh(Instant::now()));
        assert_eq!(reader.force.iter().collect::<Vec<_>>(), ["12"]);

        reader.campaign_copy_saved("12".into(), Ok(vec![path.clone()]));
        reader.campaign_copy_saved("13".into(), Ok(Vec::new()));
        assert!(reader.conflicts.is_empty());
        assert_eq!(reader.archived, [path]);
        assert_eq!(
            reader.warnings.keys().collect::<Vec<_>>(),
            [&Stored::Roster]
        );
        reader.stored(Stored::Roster, Ok(()));
        assert!(reader.warnings.is_empty());
    }
    #[test]
    fn stream_driven_refresh_obeys_visibility_debounce_and_rate_limits() {
        let now = Instant::now();
        let mut reader = Reader::default();
        reader.visible = true;
        reader.stream_open = true;
        reader.last_check = Some(now);
        assert!(!reader.wants_refresh(now + Duration::from_secs(15)));
        assert!(reader.wants_refresh(now + Duration::from_secs(300)));
        reader.select("12".into());
        assert!(reader.wants_refresh(now));
        reader.next = Some(now + Duration::from_millis(750));
        assert!(!reader.wants_refresh(now));
        assert!(reader.wants_refresh(now + Duration::from_secs(1)));
        reader.cooldown = Some(now + Duration::from_secs(120));
        reader.next = None;
        assert!(!reader.wants_refresh(now + Duration::from_secs(60)));
        assert!(reader.wants_refresh(now + Duration::from_secs(120)));
        reader.visible = false;
        assert!(!reader.wants_refresh(now + Duration::from_secs(121)));
        reader.visible = true;
        reader.cooldown = None;
        reader.refresh_requested = false;
        reader.stream_open = false;
        assert!(reader.wants_refresh(now));
    }
    #[test]
    #[ignore = "Uses the desktop keyring and active campaign, and writes authorized local character cache"]
    fn live_parity_cache_and_stream() {
        let dirs = directories::ProjectDirs::from("", "", "splat").unwrap();
        let config: Value = serde_json::from_slice(
            &std::fs::read(dirs.config_dir().join("settings.json")).unwrap(),
        )
        .unwrap();
        let campaign = config["campaigns"]
            .as_array()
            .unwrap()
            .iter()
            .find(|v| v["id"] == config["active_campaign_id"])
            .unwrap();
        let remote = campaign["dndbeyond_campaign_id"].as_str().unwrap();
        let local = Uuid::parse_str(campaign["id"].as_str().unwrap()).unwrap();
        let session = crate::auth::connect_live(zeroize::Zeroizing::new(
            keyring::Entry::new("splat", "dndbeyond-session")
                .unwrap()
                .get_password()
                .unwrap(),
        ))
        .unwrap();
        let client = client().unwrap();
        let members = roster(
            &get(
                &client,
                &session,
                &format!("https://www.dndbeyond.com/api/campaign/characters/{remote}"),
                true,
            )
            .unwrap(),
        )
        .unwrap();
        assert!(!members.is_empty());
        let cache = splat::character::Cache::user().unwrap();
        for m in &members {
            let record = fetch(&client, &session, &m.id, remote).unwrap();
            assert_eq!(
                record.stat("armor_class"),
                record.source.computed["armorClass"].as_i64()
            );
            assert_eq!(
                record.initial.hp,
                record.source.computed["hitPointInfo"]["current"]
                    .as_i64()
                    .unwrap()
            );
            for a in &record.character.skills {
                assert_eq!(
                    record.stat(&a.stat),
                    record.source.computed["skills"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .find(|v| v["name"] == a.name)
                        .unwrap()["modifier"]
                        .as_i64()
                );
            }
            assert_eq!(
                record.character.attacks,
                record.source.computed["attacks"]
                    .as_array()
                    .unwrap()
                    .clone()
            );
            assert!(record.entry().is_ok());
            let ctx = egui::Context::default();
            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    crate::party::overview(ui, &record, None);
                    for page in 1..5 {
                        crate::party::details(ui, &record, page, "");
                    }
                });
            });
            cache.save(local, &record).unwrap();
        }
        cache
            .save_roster(
                local,
                remote,
                &members.iter().map(|m| m.id.clone()).collect::<Vec<_>>(),
            )
            .unwrap();
        assert_eq!(cache.load_all(local, remote).unwrap().len(), members.len());
        println!(
            "Verified supplied totals, attacks, schema entries and persistent cache for {} party members.",
            members.len()
        );
        let ctx = egui::Context::default();
        let stream = crate::party_stream::Stream::start(session, remote.into(), &ctx);
        match stream.rx.recv_timeout(Duration::from_secs(20)) {
            Ok(crate::party_stream::Event::Open) => println!(
                "Official campaign WebSocket handshake succeeded. No character edit was sent."
            ),
            _ => panic!("The event stream did not connect."),
        }
    }
}
