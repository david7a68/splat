//! Read-only API transport and the background reads that keep every party sheet current,
//! whichever page is showing, from the time Splat connects to D&D Beyond.
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
/// The longest any sheet goes without a read, even if the stream reports nothing.
const SAFETY_READ: Duration = Duration::from_secs(60);
/// D&D Beyond can announce a change before its computed sheet has it, so a changed sheet is
/// read this many times.
const READS_OF_A_CHANGE: u8 = 2;
const PORTRAIT_HOSTS: &[&str] = &["www.dndbeyond.com", "media.dndbeyond.com"];
const PORTRAIT_RETRY: Duration = Duration::from_secs(60 * 60);
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
/// The address to download a portrait from. Sheet data names the address, so only D&D
/// Beyond's own hosts are accepted. The request carries no session.
fn portrait_request(url: &str) -> Option<String> {
    let mut url = reqwest::Url::parse(url).ok()?;
    let trusted = url.scheme() == "https"
        && url
            .host_str()
            .is_some_and(|host| PORTRAIT_HOSTS.contains(&host));
    trusted.then(|| {
        url.set_query(Some("width=512&height=512&fit=crop&quality=90"));
        url.into()
    })
}
/// Download a character's portrait if the cache does not have it yet. After a failure, the
/// same address waits before another attempt. Returns false while it waits, as nothing changed.
fn store_portrait(
    client: &Client,
    gate: &Mutex<bool>,
    failures: &Mutex<BTreeMap<String, Instant>>,
    local: Uuid,
    record: &EntitySnapshot,
) -> Result<bool, String> {
    let Some(url) = record.portrait_url() else {
        return Ok(true);
    };
    let (remote, id) = (&record.source.campaign_id, &record.source.character_id);
    let cache = crate::hermetic::character_cache()?;
    if cache.portrait_path(local, remote, id, url)?.exists() {
        return Ok(true);
    }
    let waiting = failures
        .lock()
        .ok()
        .and_then(|failures| failures.get(url).copied())
        .is_some_and(|failed| failed.elapsed() < PORTRAIT_RETRY);
    if waiting {
        return Ok(false);
    }
    let failed = || {
        if let Ok(mut failures) = failures.lock() {
            failures.insert(url.to_owned(), Instant::now());
        }
        format!(
            "No portrait saved for {}. The download failed.",
            record.definition.splat_name
        )
    };
    let request = portrait_request(url).ok_or_else(failed)?;
    let bytes = client
        .get(request)
        .send()
        .map_err(|_| failed())
        .and_then(|reply| response(reply).map_err(|_| failed()))?;
    // A slow download must not write into a campaign the DM has left.
    if let Ok(active) = gate.lock()
        && *active
    {
        cache
            .save_portrait(local, remote, id, url, &bytes)
            .map_err(|_| failed())?;
    }
    Ok(true)
}
/// The cached portrait of a character, if there is one.
pub fn portrait_path(local: Uuid, record: &EntitySnapshot) -> Option<std::path::PathBuf> {
    let source = &record.source;
    crate::hermetic::character_cache()
        .and_then(|cache| {
            cache.portrait_path(
                local,
                &source.campaign_id,
                &source.character_id,
                record.portrait_url().ok_or("No portrait.")?,
            )
        })
        .ok()
        .filter(|path| path.exists())
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
    Writer(crate::sheet_writer::Turn),
    /// Cached sheets waited for this computer's new claim to settle. Save them after it.
    CacheSavesLater,
}
/// What a local storage warning is about. The next success for the same thing clears it.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stored {
    SavedSheets,
    Roster,
    Sheet(String),
    CampaignCopy(String),
    Portrait(String),
    /// The note that says which computer saves the sheets. See `sheet_writer`.
    Writer,
}
fn save_campaign_copy(
    record: &EntitySnapshot,
    path: &std::path::Path,
    force: bool,
) -> Result<Vec<PathBuf>, CampaignCopyError> {
    // Settings that cannot be read list no packages, so new definitions go to the campaign package.
    let modules = crate::persistence::Store::user()
        .and_then(|mut store| store.load())
        .map(|data| data.modules)
        .unwrap_or_default();
    let destinations = crate::content_workspace::destinations(path, &modules);
    if force {
        record.overwrite_campaign_copy_to(path, &destinations)
    } else {
        record
            .save_campaign_copy_to(path, &destinations)
            .map(|_| Vec::new())
    }
}
/// Whether this computer saves the sheets from automatic reads. See `sheet_writer`.
fn writer_turn(path: &std::path::Path) -> crate::sheet_writer::Turn {
    let (device, name) = crate::hermetic::device();
    crate::sheet_writer::take(path, device, &name, crate::hermetic::now())
}
/// A save that failed gives up this computer's turn, so that a computer that can save takes
/// over. A note with local edits is not a failure: the DM decides about it.
fn release_after(path: &std::path::Path, saved: &Result<Vec<PathBuf>, CampaignCopyError>) {
    if matches!(saved, Err(CampaignCopyError::Failed(_))) {
        let (device, _) = crate::hermetic::device();
        crate::sheet_writer::release(path, device, crate::hermetic::now());
    }
}
/// Whether an automatic save may write now. It asks the note right before each save, because
/// a read can take long enough for another computer to take the turn, and a failed save gives
/// the turn up. The answer also goes to the page.
fn may_save(path: &std::path::Path, tx: &mpsc::Sender<Event>) -> crate::sheet_writer::Turn {
    let turn = writer_turn(path);
    let _ = tx.send(Event::Writer(turn.clone()));
    turn
}
/// Saves the campaign copies of cached sheets, each only while this computer has the turn.
/// A claim that has not settled asks for the saves again later.
fn save_cached(
    records: &[EntitySnapshot],
    path: &std::path::Path,
    gate: &Arc<Mutex<bool>>,
    tx: &mpsc::Sender<Event>,
) {
    for record in records {
        // Hold the gate through the save, so a campaign switch cannot meet a save halfway.
        let Ok(active) = gate.lock() else {
            break;
        };
        if !*active {
            break;
        }
        match may_save(path, tx) {
            crate::sheet_writer::Turn::Mine => {}
            crate::sheet_writer::Turn::Settling => {
                let _ = tx.send(Event::CacheSavesLater);
                break;
            }
            _ => break,
        }
        let saved = save_campaign_copy(record, path, false);
        release_after(path, &saved);
        let _ = tx.send(Event::CampaignCopy(
            record.source.character_id.clone(),
            saved,
        ));
        drop(active);
    }
}
#[derive(Default)]
pub struct Reader {
    portrait_failures: Arc<Mutex<BTreeMap<String, Instant>>>,
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
    pub archived_owners: BTreeMap<PathBuf, String>,
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
    /// The Party page shows. With the stream down, only then is a 15-second read worth its cost.
    pub on_page: bool,
    /// A read found a character with no copy before. Whoever lists the party should look again.
    pub new_copy: bool,
    /// Members the stream says changed, each with the reads still owed.
    changed: BTreeMap<String, u8>,
    followup: Option<Instant>,
    last_check: Option<Instant>,
    cooldown: Option<Instant>,
    next: Option<Instant>,
    last_roster: Option<Instant>,
    backoff: u32,
    cache_loaded: bool,
    /// The computer that saves the sheets from automatic reads, when it is not this one.
    pub writer_elsewhere: Option<String>,
    /// The DM asked for the next read, so it saves on this computer too.
    by_hand: bool,
    /// When to save the cached sheets again, after a new claim settles.
    cache_saves_at: Option<Instant>,
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
                save_cached(records, &scope.path, &gate, &tx);
            }
            let _ = tx.send(Event::Cache(result));
            ctx.request_repaint();
        });
    }
    /// Saves the cached sheets again, after this computer's claim has settled. It works without
    /// a session, so a DM who starts offline still gets the copies.
    fn save_cached_later(&mut self, ctx: &egui::Context) {
        let Some(scope) = self.scope.clone() else {
            return;
        };
        let (tx, rx) = mpsc::channel();
        self.pending = Some(rx);
        let ctx = ctx.clone();
        let gate = self.gate.clone();
        thread::spawn(move || {
            if let Ok(records) = crate::hermetic::character_cache()
                .and_then(|c| c.load_all(scope.local, &scope.remote))
            {
                save_cached(&records, &scope.path, &gate, &tx);
            }
            ctx.request_repaint();
        });
    }
    pub fn tick(&mut self, session: Option<Session>, ctx: &egui::Context) {
        self.poll();
        // An idle app draws no frames, so ask for one when the cached saves are due.
        if let Some(due) = self.cache_saves_at {
            ctx.request_repaint_after(
                due.saturating_duration_since(Instant::now())
                    .max(Duration::from_millis(250)),
            );
        }
        if self.pending.is_none() && self.cache_saves_at.is_some_and(|t| t <= Instant::now()) {
            self.cache_saves_at = None;
            self.save_cached_later(ctx);
            return;
        }
        if !self.cache_loaded && self.pending.is_none() {
            self.load_cache(ctx);
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
            let events: Vec<_> = self
                .stream
                .as_ref()
                .map_or_else(Vec::new, |stream| stream.rx.try_iter().collect());
            let mut closed = false;
            for event in events {
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
                    crate::party_stream::Event::Changed(id) => self.note_change(&id),
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
            // Each check can come an interval late, and every sheet is still read each minute.
            let roster = self
                .last_roster
                .is_none_or(|t| t.elapsed() + INTERVAL >= SAFETY_READ);
            self.refresh(session, ctx, roster);
        }
    }
    /// The stream says a sheet changed. `*` means it could be any of them. Any member's change
    /// is read, because a page other than Party may be showing that member.
    fn note_change(&mut self, id: &str) {
        let everyone = id == "*";
        if everyone || self.members.iter().any(|m| m.id == id) {
            for member in &self.members {
                if everyone || member.id == id {
                    self.changed.insert(member.id.clone(), READS_OF_A_CHANGE);
                }
            }
            self.refresh_requested = true;
            self.next = Some(Instant::now() + Duration::from_millis(750));
            self.followup = Some(Instant::now() + Duration::from_secs(5));
        } else {
            self.last_roster = None;
            self.refresh_requested = true;
            self.next = None;
        }
    }
    /// The members to read for a change, with one read fewer owed for each.
    fn take_changed(&mut self) -> Vec<String> {
        let changed = self.changed.keys().cloned().collect();
        self.changed.retain(|_, owed| {
            *owed -= 1;
            *owed > 0
        });
        changed
    }
    fn wants_refresh(&self, now: Instant) -> bool {
        let safety_due = self
            .last_check
            .is_none_or(|t| now.duration_since(t) >= SAFETY_READ);
        self.pending.is_none()
            && self.cooldown.is_none_or(|t| t <= now)
            && self.next.is_none_or(|t| t <= now)
            && (self.refresh_requested || (!self.stream_open && self.on_page) || safety_due)
    }
    pub fn refresh(&mut self, session: Session, ctx: &egui::Context, full: bool) {
        // A request by hand counts for this call only. If the call cannot read now, a later
        // automatic read must not inherit it.
        let by_hand = std::mem::take(&mut self.by_hand);
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
        let changed = self.take_changed();
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
        let failures = self.portrait_failures.clone();
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
                        .chain(changed)
                        .chain(selected)
                        .collect::<BTreeSet<_>>()
                        .into_iter()
                        .collect()
                };
                let mut portraits = Vec::new();
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
                            let forced = force.contains(&id);
                            let allowed = by_hand
                                || forced
                                || match may_save(&scope.path, &tx) {
                                    crate::sheet_writer::Turn::Mine => true,
                                    crate::sheet_writer::Turn::Settling => {
                                        // The read is cached. Save it once the claim settles,
                                        // even if the next reads fail.
                                        let _ = tx.send(Event::CacheSavesLater);
                                        false
                                    }
                                    _ => false,
                                };
                            if allowed {
                                let saved = save_campaign_copy(record, &scope.path, forced);
                                if !by_hand && !forced {
                                    release_after(&scope.path, &saved);
                                }
                                let _ = tx.send(Event::CampaignCopy(id.clone(), saved));
                            }
                        }
                    }
                    if let Ok(record) = &result
                        && record.portrait_url().is_some()
                    {
                        portraits.push(record.clone());
                    }
                    if tx.send(Event::Sheet(id, Box::new(result))).is_err() {
                        break;
                    }
                    ctx.request_repaint();
                    if failure {
                        break;
                    } // Do not hammer an unavailable or rate-limited service.
                }
                // After the sheets, so that a slow image never holds a sheet back.
                for record in portraits {
                    if gate.lock().map_or(true, |active| !*active) {
                        break;
                    }
                    let stored = store_portrait(&client, &gate, &failures, scope.local, &record);
                    if let Some(result) =
                        stored.map_or_else(|e| Some(Err(e)), |done| done.then_some(Ok(())))
                    {
                        let what = Stored::Portrait(record.source.character_id.clone());
                        let _ = tx.send(Event::Stored(what, result));
                    }
                    ctx.request_repaint();
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
                        Stored::Sheet(id) | Stored::CampaignCopy(id) | Stored::Portrait(id) => {
                            members.iter().any(|m| &m.id == id)
                        }
                        Stored::SavedSheets | Stored::Roster | Stored::Writer => true,
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
                        self.new_copy |= self.sheets.insert(id, record).is_none();
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
                Ok(Event::Writer(turn)) => self.note_turn(turn),
                Ok(Event::CacheSavesLater) => {
                    self.cache_saves_at =
                        Some(Instant::now() + Duration::from_secs(crate::sheet_writer::SETTLE + 1));
                }
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
    /// The DM asked to read the party now. This read saves the sheets on this computer, even when
    /// another computer saves the automatic reads.
    pub fn refresh_by_hand(&mut self, session: Session, ctx: &egui::Context) {
        self.by_hand = true;
        self.refresh(session, ctx, true);
    }
    /// Saves a sheet that came from elsewhere, such as combat sync, and records the result.
    /// Combat sync reads on its own, so only the computer that saves the sheets saves it.
    pub fn save_campaign_copy(&mut self, id: &str, record: &EntitySnapshot) {
        let Some(path) = self.campaign_path().map(std::path::Path::to_path_buf) else {
            return;
        };
        // Ask the note now: an answer from an earlier read can be out of date.
        let turn = writer_turn(&path);
        let mine = turn == crate::sheet_writer::Turn::Mine;
        self.note_turn(turn);
        if mine {
            let saved = save_campaign_copy(record, &path, false);
            release_after(&path, &saved);
            self.campaign_copy_saved(id.to_owned(), saved);
        }
    }
    /// Shows who saves the sheets, and a warning when the note stops every automatic save.
    fn note_turn(&mut self, turn: crate::sheet_writer::Turn) {
        use crate::sheet_writer::Turn;
        self.writer_elsewhere = match &turn {
            Turn::Elsewhere(name) => Some(name.clone()),
            Turn::Mine | Turn::Settling | Turn::Unavailable(_) => None,
        };
        match turn {
            Turn::Unavailable(message) => self.stored(Stored::Writer, Err(message)),
            _ => self.stored(Stored::Writer, Ok(())),
        }
    }
    /// A saved note clears its character's warnings, so moving a note aside clears them too.
    fn campaign_copy_saved(&mut self, id: String, result: Result<Vec<PathBuf>, CampaignCopyError>) {
        let what = Stored::CampaignCopy(id.clone());
        match result {
            Ok(archived) => {
                self.conflicts.remove(&id);
                for path in &archived {
                    self.archived_owners.insert(path.clone(), id.clone());
                }
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
    /// Replace only the named character's edited notes, keeping the other drafts intact.
    pub fn force_read_character(&mut self, id: &str) {
        if self.conflicts.contains_key(id) {
            self.force.insert(id.to_owned());
            self.refresh_requested = true;
            self.next = None;
        }
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
    fn portraits_come_only_from_dnd_beyond_over_https() {
        assert_eq!(
            portrait_request("https://www.dndbeyond.com/avatars/1/2/a.jpeg?width=150&height=150")
                .as_deref(),
            Some(
                "https://www.dndbeyond.com/avatars/1/2/a.jpeg?width=512&height=512&fit=crop&quality=90"
            )
        );
        for url in [
            "http://www.dndbeyond.com/avatars/a.jpeg",
            "https://www.dndbeyond.com.evil.example/a.jpeg",
            "https://example.com/a.jpeg",
            "file:///etc/passwd",
            "not a url",
        ] {
            assert_eq!(portrait_request(url), None, "{url}");
        }
    }
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
    fn switching_campaign_closes_workers_and_discards_late_replies() {
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
        reader.gate = Arc::new(Mutex::new(true));
        let gate = reader.gate.clone();
        let (tx, rx) = mpsc::channel();
        reader.pending = Some(rx);
        reader.sync(&ctx, 1, Some((id, "13".into(), "/unused".into())), true);
        assert!(!*gate.lock().unwrap());
        assert!(
            tx.send(Event::Stored(Stored::Roster, Err("late".into())))
                .is_err()
        );
    }
    #[test]
    fn only_the_computer_with_the_turn_saves_automatic_reads() {
        let campaign = tempfile::tempdir().unwrap();
        let mut reader = Reader::default();
        reader.scope = Some(Scope {
            epoch: 1,
            local: Uuid::new_v4(),
            remote: "12".into(),
            path: campaign.path().into(),
            connected: true,
        });
        let saved = |root: &std::path::Path| {
            std::fs::read_dir(root.join("Characters/D&D Beyond")).map_or(0, |files| {
                files
                    .flatten()
                    .filter(|e| e.path().extension().is_some_and(|x| x == "md"))
                    .count()
            })
        };
        let record = crate::visual_tests::fixture::sample_sheet();
        let id = record.source.character_id.clone();
        let now = crate::hermetic::now();
        let turn = |device: u128, name: &str| crate::sheet_writer::Holder {
            device: Uuid::from_u128(device),
            name: name.into(),
            since: now - 120,
            until: now + 300,
        };
        // Another computer holds the turn. An out-of-date answer that says this computer
        // saves does not count: the save asks the note.
        crate::sheet_writer::hold(campaign.path(), &turn(7, "Desk"));
        reader.writer_elsewhere = None;
        reader.save_campaign_copy(&id, &record);
        assert_eq!(saved(campaign.path()), 0);
        assert_eq!(reader.writer_elsewhere.as_deref(), Some("Desk"));
        // With the turn, the same save writes the sheet.
        let (device, _) = crate::hermetic::device();
        crate::sheet_writer::hold(campaign.path(), &turn(device.as_u128(), "This computer"));
        reader.save_campaign_copy(&id, &record);
        assert_eq!(saved(campaign.path()), 1);
        assert_eq!(reader.writer_elsewhere, None);
    }
    #[test]
    fn cached_sheets_wait_for_a_new_claim_and_ask_to_be_saved_after_it() {
        let campaign = tempfile::tempdir().unwrap();
        let record = crate::visual_tests::fixture::sample_sheet();
        let gate = Arc::new(Mutex::new(true));
        let (tx, rx) = mpsc::channel();
        // No note yet: this computer claims the turn, and the claim has not settled.
        save_cached(std::slice::from_ref(&record), campaign.path(), &gate, &tx);
        drop(tx);
        let mut reader = Reader::default();
        reader.pending = Some(rx);
        reader.poll();
        assert!(reader.cache_saves_at.is_some());
        assert!(!campaign.path().join("Characters/D&D Beyond").exists());
        assert!(campaign.path().join(crate::sheet_writer::FILE).exists());
    }
    #[test]
    fn a_closed_gate_stops_cached_saves_before_they_claim_or_write() {
        let campaign = tempfile::tempdir().unwrap();
        let record = crate::visual_tests::fixture::sample_sheet();
        let gate = Arc::new(Mutex::new(false));
        let (tx, rx) = mpsc::channel();
        save_cached(std::slice::from_ref(&record), campaign.path(), &gate, &tx);
        drop(tx);
        assert_eq!(rx.try_iter().count(), 0);
        assert!(!campaign.path().join(crate::sheet_writer::FILE).exists());
    }
    #[test]
    fn a_request_by_hand_counts_only_for_the_read_it_asked_for() {
        let mut reader = Reader::default();
        reader.by_hand = true;
        // No campaign, so this read cannot start. The request must not wait for a later
        // automatic read.
        let hour = Duration::from_secs(60 * 60);
        let session = crate::auth::Session::for_test(hour, hour);
        reader.refresh(session, &egui::Context::default(), true);
        assert!(!reader.by_hand);
    }
    #[test]
    fn a_change_to_any_member_is_read_twice_whichever_is_selected() {
        let mut reader = Reader::default();
        let member = |id: &str| Member {
            id: id.into(),
            name: id.into(),
            level: None,
        };
        reader.members = vec![member("12"), member("13"), member("14")];
        reader.selected = Some("12".into());
        reader.note_change("13");
        assert!(reader.refresh_requested);
        assert_eq!(reader.take_changed(), ["13"]);
        assert_eq!(reader.take_changed(), ["13"]);
        assert!(reader.take_changed().is_empty());
        reader.note_change("*");
        assert_eq!(reader.take_changed(), ["12", "13", "14"]);
        // Someone new: the roster is read, and no sheet is owed yet.
        reader.take_changed();
        reader.note_change("99");
        assert!(reader.take_changed().is_empty());
        assert!(reader.last_roster.is_none());
    }
    #[test]
    fn replacing_one_characters_sheet_keeps_the_other_characters_edits() {
        let mut reader = Reader::default();
        reader
            .conflicts
            .insert("12".into(), "/campaign/12.md".into());
        reader
            .conflicts
            .insert("13".into(), "/campaign/13.md".into());
        reader.force_read_character("12");
        reader.force_read_character("missing");
        assert_eq!(
            reader.force.iter().map(String::as_str).collect::<Vec<_>>(),
            ["12"]
        );
        assert!(reader.conflicts.contains_key("13"));
    }

    #[test]
    fn a_warning_stays_until_the_same_thing_saves_and_force_reads_at_once() {
        let mut reader = Reader::default();
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
    fn stream_driven_refresh_obeys_debounce_and_rate_limits() {
        let now = Instant::now();
        let mut reader = Reader::default();
        reader.stream_open = true;
        reader.last_check = Some(now);
        assert!(!reader.wants_refresh(now + Duration::from_secs(15)));
        assert!(reader.wants_refresh(now + SAFETY_READ));
        reader.select("12".into());
        assert!(reader.wants_refresh(now));
        reader.next = Some(now + Duration::from_millis(750));
        assert!(!reader.wants_refresh(now));
        assert!(reader.wants_refresh(now + Duration::from_secs(1)));
        reader.cooldown = Some(now + Duration::from_secs(120));
        reader.next = None;
        assert!(!reader.wants_refresh(now + Duration::from_secs(60)));
        assert!(reader.wants_refresh(now + Duration::from_secs(120)));
        reader.cooldown = None;
        reader.refresh_requested = false;
        reader.stream_open = false;
        // The stream is down. Away from the page, a sheet waits for the safety read.
        reader.next = Some(now);
        reader.last_check = Some(now);
        assert!(!reader.wants_refresh(now + INTERVAL));
        assert!(reader.wants_refresh(now + SAFETY_READ));
        reader.on_page = true;
        assert!(reader.wants_refresh(now + INTERVAL));
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
