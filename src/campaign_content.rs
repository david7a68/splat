//! What a campaign uses: package selections, and the repair of invalid notes. This is
//! a rare setup task, so it stays calm, with one scrolling list at 2,000+ rows.
use crate::{
    campaigns::Campaigns,
    content_workspace::{
        CampaignDocument, Catalogue, Closure, EntryRef, PackageSelection, Selection,
    },
    design::{self, Action, Density, Tone, Type, colors, icons},
    library::{note_path, write_note},
    persistence::{Campaign, Module},
};
use eframe::egui;
use splat::character::campaign_package::{self, Move, Proposal};
use splat::content::Entry;
use std::{
    collections::BTreeSet,
    fs,
    sync::mpsc,
    time::{Duration, Instant},
};
use uuid::Uuid;

/// Sheets refresh in the background, so the campaign package needs another look now and then.
const CHECK_EVERY: Duration = Duration::from_secs(5 * 60);

pub(crate) struct Snapshot {
    document: CampaignDocument,
    catalogue: Catalogue,
    /// The closure of these selections. It is too slow to calculate on every frame.
    closure: (Vec<PackageSelection>, Closure),
    subtitles: std::collections::HashMap<Uuid, String>,
}
/// An invalid note in repair: its path, its text on disk and its draft, with Save's check of the
/// draft. The check runs only where the draft changes (a load, an edit, Discard, Undo discard), so
/// an unchanged frame neither hashes nor parses the text.
pub(crate) struct Repair {
    path: String,
    baseline: String,
    draft: String,
    /// Why Save would refuse `draft`: Save's own check without writing, shared with World.
    refusal: Option<String>,
    /// The line of `draft` to mark.
    error_line: Option<usize>,
    /// A Save of this draft that failed on the disk: Save's reason. Only this repair owns it. It
    /// ends at this repair's next edit, its successful Save, a reload or another campaign.
    failure: Option<String>,
}
impl Repair {
    fn new(path: String, text: String, campaign: &Campaign) -> Self {
        let mut repair = Self {
            path,
            baseline: text.clone(),
            draft: text,
            refusal: None,
            error_line: None,
            failure: None,
        };
        repair.check(campaign);
        repair
    }
    /// Ask Save's question of the draft again.
    fn check(&mut self, campaign: &Campaign) {
        (self.refusal, self.error_line) =
            crate::world::repair::refusal(campaign, &self.path, &self.draft);
    }
    /// The draft changed: check it again. A failed Save no longer describes it.
    fn draft_changed(&mut self, campaign: &Campaign, checks: &mut usize) {
        *checks += 1;
        self.check(campaign);
        self.failure = None;
    }
    fn dirty(&self) -> bool {
        self.draft != self.baseline
    }
}
type Loaded = (Vec<Repair>, Result<Snapshot, String>);
struct Checked {
    applied: Option<Result<usize, String>>,
    proposals: Result<Vec<Proposal>, String>,
}
/// What the last check of the campaign package found. Nothing moves until the DM applies it.
#[derive(Default)]
struct Reconcile {
    campaign: Option<Uuid>,
    checked_at: Option<Instant>,
    pending: Option<mpsc::Receiver<Checked>>,
    applying: bool,
    proposals: Vec<Proposal>,
    /// Proposals that the DM unticked.
    declined: BTreeSet<String>,
    error: Option<String>,
    message: Option<String>,
}
#[derive(Default)]
pub struct CampaignContent {
    reconcile: Reconcile,
    active: Option<Campaign>,
    pending: Option<mpsc::Receiver<Loaded>>,
    snapshot: Option<Snapshot>,
    saved_selections: Vec<PackageSelection>,
    module: Option<Uuid>,
    dependencies: bool,
    notice_scope: Option<(Option<Uuid>, bool)>,
    search: String,
    entry_matches: Option<(Uuid, String, Vec<Uuid>)>,
    /// The DM's choice of list. Until they choose, a package opens on the list it needs.
    pub run_open: bool,
    pub open_lookup: Option<Uuid>,
    pub open_run_settings: bool,
    pub found_package: Option<Module>,
    pub forget_package: Option<Uuid>,
    recovered: Option<(Module, Option<Module>)>,
    error: Option<String>,
    message: Option<String>,
    disabled_selections: std::collections::BTreeMap<Uuid, Selection>,
    refresh_pending: bool,
    removed_package: Option<RemovedPackage>,
    manifest_write: Option<crate::content_workspace::ManifestWrite>,
    campaign_write: Option<crate::content_workspace::ManifestWrite>,
    repairs: Vec<Repair>,
    /// How often a frame checked a repair draft again. Only an edit, Discard or Undo discard does.
    repair_checks: usize,
    /// What `Discard` last dropped, for one step of undo, and the Campaign.md text it was a draft
    /// of. A missing or changed Campaign.md ends it.
    discarded_selections: Option<(Vec<PackageSelection>, Option<String>)>,
    discarded_repair: Option<(String, String)>,
    /// The repair the DM edited or focused last. It owns the primary Save while it is open and
    /// changed; other changed repairs keep their drafts with a secondary Save.
    active_repair: Option<String>,
}
struct RemovedPackage {
    campaign: Uuid,
    name: String,
    before: Vec<PackageSelection>,
    after: Vec<PackageSelection>,
    users: Vec<(Uuid, String)>,
}

/// Return a useful YAML repair message and the physical line to mark.
pub(crate) fn repair_problem(path: &str, text: &str) -> (Option<String>, Option<usize>) {
    // Repair rows and editors ask on every frame, and a parse of a long note takes about 1 ms.
    static PROBLEMS: splat::memo::Memo<(Option<String>, Option<usize>)> =
        splat::memo::Memo::new(64);
    PROBLEMS.get_or(&[path.as_bytes(), text.as_bytes()], || {
        find_repair_problem(path, text)
    })
}

fn find_repair_problem(path: &str, text: &str) -> (Option<String>, Option<usize>) {
    let Some(problem) = Entry::parse(path, text).err() else {
        return (None, None);
    };
    let yaml = text
        .strip_prefix("---\n")
        .and_then(|text| text.split_once("\n---").map(|(yaml, _)| yaml));
    if let Some(yaml) = yaml
        && let Err(error) = serde_yaml_ng::from_str::<serde_yaml_ng::Value>(yaml)
    {
        let mut opens = Vec::new();
        for (index, line) in yaml.lines().enumerate() {
            let mut quote = None;
            for ch in line.chars() {
                if matches!(ch, '\'' | '"') {
                    if quote == Some(ch) {
                        quote = None;
                    } else if quote.is_none() {
                        quote = Some(ch);
                    }
                }
                if quote.is_some() {
                    continue;
                }
                if ch == '#' {
                    break;
                }
                match ch {
                    '[' | '{' => opens.push((ch, index + 2)),
                    ']' | '}' => {
                        opens.pop();
                    }
                    _ => {}
                }
            }
        }
        if let Some((ch, line)) = opens.first() {
            let thing = if *ch == '[' { "list" } else { "mapping" };
            return (
                Some(format!(
                    "Line {line} opens a {thing} with {ch} that never closes."
                )),
                Some(*line),
            );
        }
        if let Some(location) = error.location() {
            let line = location.line() + 1;
            return (
                Some(format!(
                    "Check the YAML on line {line}. Use a name, a colon and its value."
                )),
                Some(line),
            );
        }
    }
    (Some(problem), None)
}
/// A folder recovery must keep the selected package's identity. `name` is the package the DM
/// looks for, as the campaign names it.
fn find_package_folder(
    reference: &EntryRef,
    name: Option<&str>,
    folder: &std::path::Path,
) -> Result<Module, String> {
    let module = crate::library::local_module(folder)?;
    let matches = if let Some(id) = reference.id {
        id == module.id
    } else {
        let catalogue = Catalogue::load(std::slice::from_ref(&module), None)?;
        catalogue.resolve_ref(reference).ok() == Some(module.id)
    };
    if !matches {
        // The fault first, then what to do, and both things by name.
        let picked = folder.file_name().map_or_else(
            || folder.display().to_string(),
            |name| name.to_string_lossy().into_owned(),
        );
        return Err(format!(
            "{picked}/ holds a different package. Choose the folder that holds {}.",
            name.unwrap_or("this package")
        ));
    }
    Ok(module)
}

fn relocated_package(module: &Module, registered: &[Module]) -> Option<Module> {
    let parents: BTreeSet<_> = std::iter::once(module)
        .chain(registered)
        .filter_map(|module| module.path.parent())
        .collect();
    let mut found = std::collections::BTreeMap::new();
    for parent in parents {
        for folder in fs::read_dir(parent).ok().into_iter().flatten().flatten() {
            if folder.file_name().to_string_lossy().starts_with('.') {
                continue;
            }
            if let Ok(candidate) = crate::library::local_module(&folder.path())
                && candidate.id == module.id
                && candidate.path != module.path
            {
                found.insert(candidate.path.clone(), candidate);
            }
        }
    }
    (found.len() == 1).then(|| found.into_values().next().unwrap())
}

fn folder_words(path: &std::path::Path) -> String {
    path.parent()
        .and_then(|parent| parent.file_name())
        .map_or_else(
            || path.display().to_string(),
            |parent| {
                format!(
                    "{}/{}",
                    parent.to_string_lossy(),
                    path.file_name().unwrap_or_default().to_string_lossy()
                )
            },
        )
}

impl CampaignContent {
    pub fn package_restored(&mut self, result: Result<(), String>) {
        match result {
            Ok(()) => {
                self.refresh_pending = true;
                self.error = None;
            }
            Err(error) => self.error = Some(error),
        }
    }
    /// How often a frame checked a repair draft again.
    #[cfg(test)]
    pub fn repair_checks(&self) -> usize {
        self.repair_checks
    }
    #[cfg(test)]
    pub fn busy(&self) -> bool {
        self.pending.is_some()
    }

    pub fn dirty(&self) -> bool {
        self.repairs.iter().any(Repair::dirty)
            || self
                .snapshot
                .as_ref()
                .is_some_and(|s| s.document.selections != self.saved_selections)
    }
    pub fn context(&self) -> String {
        self.snapshot.as_ref().map_or_else(String::new, |snapshot| {
            let chosen = snapshot
                .closure
                .1
                .direct
                .iter()
                .filter(|id| snapshot.catalogue.selectable(**id))
                .count();
            let needed = snapshot
                .closure
                .1
                .required
                .keys()
                .filter(|id| {
                    !snapshot.closure.1.direct.contains(id)
                        && snapshot.catalogue.selectable(**id)
                        && snapshot.catalogue.entries.get(id).is_some_and(|entry| {
                            snapshot
                                .catalogue
                                .modules
                                .iter()
                                .any(|module| !module.private && module.id == entry.owner)
                        })
                })
                .count();
            let chosen = design::count(chosen, "entry chosen", "entries chosen");
            if needed == 0 {
                chosen
            } else {
                format!("{chosen} · {needed} needed")
            }
        })
    }
    /// Runs on every frame, on any page, so the sidebar can say when the DM has moves to review.
    pub fn tick(&mut self, ctx: &egui::Context, campaigns: &Campaigns) {
        self.finish_check();
        if cfg!(test) {
            return;
        }
        let Some(active) = campaigns.active_campaign() else {
            return;
        };
        let due = self.reconcile.campaign != Some(active.id)
            || self
                .reconcile
                .checked_at
                .is_none_or(|at| at.elapsed() >= CHECK_EVERY);
        if due && self.reconcile.pending.is_none() {
            self.start_check(ctx, campaigns, active.clone(), None);
        }
    }
    /// Words for the sidebar, when the campaign package has something to review.
    pub fn review_detail(&self) -> Option<String> {
        let count = self.reconcile.proposals.len();
        (count > 0).then(|| design::count(count, "to review", "to review"))
    }
    fn finish_check(&mut self) {
        let Some(rx) = &self.reconcile.pending else {
            return;
        };
        let checked = match rx.try_recv() {
            Ok(checked) => checked,
            Err(mpsc::TryRecvError::Empty) => return,
            Err(mpsc::TryRecvError::Disconnected) => Checked {
                applied: None,
                proposals: Err("The campaign package check stopped.".into()),
            },
        };
        self.reconcile.pending = None;
        self.reconcile.applying = false;
        self.reconcile.checked_at = Some(Instant::now());
        self.reconcile.error = None;
        match checked.proposals {
            Ok(proposals) => self.reconcile.proposals = proposals,
            Err(error) => self.reconcile.error = Some(error),
        }
        match checked.applied {
            Some(Ok(count)) => {
                self.reconcile.message = Some(design::count(
                    count,
                    "change applied. The old note is archived beside its path.",
                    "changes applied. The old notes are archived beside their paths.",
                ));
                self.refresh_pending = !self.dirty();
            }
            Some(Err(error)) => self.reconcile.error = Some(error),
            None => {}
        }
    }
    /// With `keys`, carries out those proposals first. The check itself changes nothing.
    fn start_check(
        &mut self,
        ctx: &egui::Context,
        campaigns: &Campaigns,
        campaign: Campaign,
        keys: Option<BTreeSet<String>>,
    ) {
        let (tx, rx) = mpsc::channel();
        self.reconcile.campaign = Some(campaign.id);
        self.reconcile.pending = Some(rx);
        self.reconcile.applying = keys.is_some();
        let modules = campaigns.modules().to_vec();
        let ctx = ctx.clone();
        std::thread::spawn(move || {
            let destinations = crate::content_workspace::destinations(&campaign.path, &modules);
            let records = campaign.dndbeyond_campaign_id.as_ref().map_or_else(
                || Ok(Vec::new()),
                |remote| {
                    crate::hermetic::character_cache()
                        .and_then(|cache| cache.load_all(campaign.id, remote))
                },
            );
            let applied = keys.map(|keys| {
                records.as_ref().map_err(Clone::clone).and_then(|records| {
                    campaign_package::apply(&campaign.path, &destinations, records, &keys)
                })
            });
            let proposals = records.and_then(|records| {
                campaign_package::check(&campaign.path, &destinations, &records)
            });
            let _ = tx.send(Checked { applied, proposals });
            ctx.request_repaint();
        });
    }
    pub fn refresh_destination(&mut self, _root: &std::path::Path) {
        // A package write can also save this campaign's selection (Use in this campaign).
        if !self.dirty() && self.active.is_some() {
            self.refresh_pending = true;
        }
    }
    /// Discarded work belongs to its campaign. Undo must not save it into another.
    fn forget_discards_of_other_campaigns(&mut self, campaign: &Campaign) {
        if self.active.as_ref().is_none_or(|old| old.id != campaign.id) {
            self.discarded_selections = None;
            self.discarded_repair = None;
            self.removed_package = None;
            self.campaign_write = None;
        }
    }
    /// Undo package removal checks the package choices only. If the file's choices changed since
    /// the removal, that Undo would refuse: the result and its Undo go.
    fn forget_changed_removal(&mut self, document: &crate::content_workspace::CampaignDocument) {
        if self.removed_package.as_ref().is_some_and(|removed| {
            removed.campaign == document.campaign.id && removed.after != document.selections
        }) {
            self.removed_package = None;
        }
    }
    /// `Undo discard` of the package choices belongs to Campaign.md as it was. When the file goes
    /// missing or changes on disk, the discarded draft and its Undo go.
    fn forget_discard_of_changed_file(&mut self, loaded: Result<&CampaignDocument, &String>) {
        if self
            .discarded_selections
            .as_ref()
            .is_some_and(|(_, baseline)| {
                !loaded.is_ok_and(|document| document.baseline == *baseline)
            })
        {
            self.discarded_selections = None;
        }
    }
    fn reload(&mut self, ctx: &egui::Context, campaigns: &Campaigns, campaign: Campaign) {
        self.forget_discards_of_other_campaigns(&campaign);
        crate::content_workspace::ManifestWrite::forget_if_changed(&mut self.manifest_write);
        crate::content_workspace::ManifestWrite::forget_if_changed(&mut self.campaign_write);
        self.refresh_pending = false;
        self.reconcile.checked_at = None;
        self.disabled_selections.clear();
        self.repairs.clear();
        let modules = campaigns.modules().to_vec();
        self.active = Some(campaign.clone());
        self.snapshot = None;
        self.entry_matches = None;
        self.error = None;
        self.message = None;
        let (tx, rx) = mpsc::channel();
        self.pending = Some(rx);
        let ctx = ctx.clone();
        std::thread::spawn(move || {
            let _ = tx.send(load_snapshot(&campaign, &modules));
            ctx.request_repaint();
        });
    }
    pub fn clear_notice(&mut self) {
        self.message = None;
    }

    pub fn show(&mut self, ui: &mut egui::Ui, campaigns: &Campaigns) {
        let scope = (self.module, self.dependencies);
        if self.notice_scope.is_some_and(|previous| previous != scope) {
            self.clear_notice();
        }
        self.notice_scope = Some(scope);
        let Some(active) = campaigns.active_campaign() else {
            ui.label("Select a campaign first.");
            return;
        };
        if self.refresh_pending {
            self.reload(ui.ctx(), campaigns, active.clone());
        }
        if self
            .active
            .as_ref()
            .is_none_or(|c| c.id != active.id || c.path != active.path)
        {
            if self.dirty() {
                ui.label(
                    Type::Section
                        .rich(ui, "Unsaved campaign content")
                        .color(colors().text),
                );
                ui.label("The previous campaign still has pending changes. Select it again to save them, or discard and switch.");
                if Action::destructive("Discard and switch campaign")
                    .show(ui)
                    .clicked()
                {
                    self.reload(ui.ctx(), campaigns, active);
                }
                return;
            }
            self.reload(ui.ctx(), campaigns, active.clone());
        }
        if let Some(rx) = &self.pending {
            match rx.try_recv() {
                Ok((repairs, result)) => {
                    self.pending = None;
                    self.repairs = repairs;
                    self.forget_discard_of_changed_file(result.as_ref().map(|s| &s.document));
                    match result {
                        Ok(snapshot) => {
                            campaigns.remember_campaign_document(&snapshot.document);
                            self.saved_selections = snapshot.document.selections.clone();
                            self.forget_changed_removal(&snapshot.document);
                            if self.module.is_none() {
                                self.dependencies = true;
                            }
                            self.snapshot = Some(snapshot);
                        }
                        Err(e) => self.error = Some(e),
                    }
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.pending = None;
                    self.error = Some("Loading stopped. Reload to retry.".into());
                }
                Err(mpsc::TryRecvError::Empty) => {}
            }
        }
        if self.dirty() {
            self.message = None;
        }
        if self.snapshot.is_some() {
            self.selection_bar(ui, campaigns, &active);
        }
        self.repair_section(ui, campaigns, &active);
        if let Some(error) = &self.error {
            // One cause, one message: a missing Campaign.md says what Splat can write
            // and offers it on the same line.
            let missing = error.contains("Campaign.md is missing");
            if !missing {
                design::message(ui, Tone::Error, error);
            }
            let (preview, action) = campaigns.campaign_write_preview(&active);
            if missing && design::message_with_action(ui, Tone::Error, &preview, action) {
                match campaigns.write_campaign_again(&active) {
                    Ok(write) => {
                        self.campaign_write = Some(write);
                        self.refresh_pending = true;
                    }
                    Err(error) => self.error = Some(error),
                }
            }
        }
        if let Some(write) = &self.campaign_write
            && design::undo_message(
                ui,
                Tone::Success,
                "Campaign.md was written again.",
                "Undo write Campaign.md",
            )
        {
            match write.undo() {
                Ok(_) => {
                    self.campaign_write = None;
                    self.refresh_pending = true;
                }
                Err(error) => self.error = Some(error),
            }
        }
        if let Some(message) = &self.message {
            design::message(ui, Tone::Success, message);
        }
        if let Some((module, previous)) = self.recovered.clone() {
            let message = format!("{} found in {}/.", module.name, folder_words(&module.path));
            if design::undo_message(ui, Tone::Success, &message, "Undo folder recovery") {
                if let Some(previous) = previous {
                    self.found_package = Some(previous);
                } else {
                    self.forget_package = Some(module.id);
                }
                self.recovered = None;
            }
        }
        if !active.path.exists() {
            design::message(
                ui,
                Tone::Warning,
                "This campaign folder does not exist yet.",
            );
            if Action::primary("Create campaign folder").show(ui).clicked() {
                match fs::create_dir_all(&active.path) {
                    Ok(()) => self.reload(ui.ctx(), campaigns, active.clone()),
                    Err(e) => self.error = Some(e.to_string()),
                }
            }
        }
        if self.pending.is_some() {
            ui.horizontal(|ui| {
                crate::design::spinner(ui);
                ui.label("Loading campaign content…");
            });
            return;
        }
        self.reconcile_section(ui, campaigns, &active);
        if self.snapshot.is_some() {
            self.packages(ui, campaigns, &active);
        }
    }

    /// Each change waits for the DM. A blocked one says what they must decide first.
    fn reconcile_section(&mut self, ui: &mut egui::Ui, campaigns: &Campaigns, active: &Campaign) {
        let reconcile = &mut self.reconcile;
        if let Some(error) = &reconcile.error {
            design::message(ui, Tone::Error, error);
        }
        if let Some(message) = &reconcile.message {
            design::message(ui, Tone::Success, message);
        }
        if reconcile.applying {
            ui.horizontal(|ui| {
                crate::design::spinner(ui);
                ui.label("Applying changes to the campaign package…");
            });
            return;
        }
        if reconcile.proposals.is_empty() {
            return;
        }
        let c = colors();
        ui.label(
            Type::Section
                .rich(ui, "Campaign package: changes to review")
                .color(c.text),
        );
        egui::ScrollArea::vertical()
            .max_height(220.0)
            .id_salt("campaign package changes")
            .show(ui, |ui| {
                for proposal in &reconcile.proposals {
                    let line = format!(
                        "{} ({}): {}",
                        proposal.name,
                        proposal.kind,
                        match &proposal.change {
                            Move::ToPackage(package) => format!("move to {package}"),
                            Move::ToCampaign =>
                                "move back to the campaign package; its package is no longer enabled"
                                    .to_owned(),
                            Move::Forget => "remove; no current entry uses it".to_owned(),
                        }
                    );
                    if let Some(reason) = &proposal.blocked {
                        ui.add(
                            egui::Label::new(
                                Type::Small
                                    .rich(ui, format!("{line}. Not available: {reason}"))
                                    .color(c.muted),
                            )
                            .wrap(),
                        );
                        continue;
                    }
                    let mut chosen = !reconcile.declined.contains(&proposal.key);
                    if ui.checkbox(&mut chosen, line).changed() {
                        if chosen {
                            reconcile.declined.remove(&proposal.key);
                        } else {
                            reconcile.declined.insert(proposal.key.clone());
                        }
                    }
                }
            });
        let keys: BTreeSet<String> = reconcile
            .proposals
            .iter()
            .filter(|p| p.blocked.is_none() && !reconcile.declined.contains(&p.key))
            .map(|p| p.key.clone())
            .collect();
        let unsaved = self
            .snapshot
            .as_ref()
            .is_some_and(|s| s.document.selections != self.saved_selections);
        let label = format!("Apply {}", design::count(keys.len(), "change", "changes"));
        let apply = Action::secondary(&label)
            .density(Density::Read)
            .hint("Sheets follow the change. Package notes get the D&D Beyond ID and keep their words. Old notes go to archives.")
            .blocked(if unsaved {
                Some("Save or discard your selection changes first.")
            } else if keys.is_empty() {
                Some("Tick a change to apply it.")
            } else {
                None
            })
            .show(ui)
            .clicked();
        if apply && self.reconcile.pending.is_none() {
            self.reconcile.message = None;
            self.start_check(ui.ctx(), campaigns, active.clone(), Some(keys));
        }
        ui.add_space(design::S);
    }

    fn repair_section(&mut self, ui: &mut egui::Ui, campaigns: &Campaigns, active: &Campaign) {
        ui.scope(|ui| {
            ui.set_max_width(ui.available_width().min(design::LIST_MEASURE));
            self.repair_section_content(ui, campaigns, active);
        });
    }

    fn repair_section_content(
        &mut self,
        ui: &mut egui::Ui,
        campaigns: &Campaigns,
        active: &Campaign,
    ) {
        let mut repaired = false;
        // One primary Save on the screen: the active repair's, else the last open changed one.
        let changed: Vec<&str> = self
            .repairs
            .iter()
            .filter(|repair| {
                repair.dirty()
                    && repair.refusal.is_none()
                    && ui.data(|data| {
                        data.get_temp::<bool>(egui::Id::new(("repair_open", repair.path.as_str())))
                            .unwrap_or(false)
                    })
            })
            .map(|repair| repair.path.as_str())
            .collect();
        // Only a draft that Save would write can hold the primary.
        let owner = (!self.run_open)
            .then(|| {
                self.active_repair
                    .as_deref()
                    .filter(|path| changed.contains(path))
                    .or(changed.last().copied())
                    .map(str::to_owned)
            })
            .flatten();
        let checks = &mut self.repair_checks;
        for repair in &mut self.repairs {
            // The message, the state word and Save all use Save's own check of the draft, which
            // the repair keeps until the draft changes.
            let path = repair.path.clone();
            let path = &path;
            let problem = repair
                .refusal
                .clone()
                .unwrap_or_else(|| "The repair has not been saved".into());
            let id = egui::Id::new(("repair_open", path.as_str()));
            let mut open = ui.data(|data| data.get_temp::<bool>(id)).unwrap_or(false);
            if design::message_with_action(
                ui,
                Tone::Error,
                &format!("{path} cannot be read. Fix it here or in World. {}.", {
                    let mut text = problem.trim_end_matches('.').to_owned();
                    if let Some(first) = text.get_mut(..1) {
                        first.make_ascii_uppercase();
                    }
                    text
                }),
                if open { "Close repair" } else { "Repair" },
            ) {
                open = !open;
            }
            ui.data_mut(|data| data.insert_temp(id, open));
            if open {
                // The message above names the file; the editor's accessible name names it too.
                design::group_heading(ui, "Repair", 0);
                ui.scope(|ui| {
                    // What is wrong comes first, and stays true as the DM edits.

                    let (edited, focused) = crate::world::repair::editor_events(
                        ui,
                        path,
                        &mut repair.draft,
                        repair.error_line,
                    );
                    if edited || focused {
                        self.active_repair = Some(path.clone());
                    }
                    if edited {
                        repair.draft_changed(active, checks);
                    }

                    ui.horizontal(|ui| {
                        let dirty = repair.dirty();
                        // The Library rule: the draft is checked as Save checks it. While Save
                        // would refuse it, the state says Invalid and Save names the cause.
                        let c = colors();
                        // An action-time failure on the disk holds this repair's slot until its
                        // draft changes or a Save of it works.
                        let failed = repair
                            .failure
                            .as_ref()
                            .map(|reason| format!("Not saved. {reason}"));
                        design::save_row_state(
                            ui,
                            if let Some(failed) = &failed {
                                design::Status {
                                    icon: icons::ERROR,
                                    word: failed,
                                    color: c.document.invalid,
                                }
                            } else if repair.refusal.is_some() {
                                design::Status {
                                    icon: icons::ERROR,
                                    word: "Invalid",
                                    color: c.document.invalid,
                                }
                            } else if dirty {
                                design::Status {
                                    icon: icons::DRAFT,
                                    word: "Unsaved",
                                    color: c.document.unsaved,
                                }
                            } else {
                                design::Status {
                                    icon: icons::OK,
                                    word: "No changes",
                                    color: c.muted,
                                }
                            },
                            false,
                            None,
                        );
                        if repair.dirty()
                            && Action::quiet("Discard")
                                .density(Density::Read)
                                .show(ui)
                                .clicked()
                        {
                            self.discarded_repair = Some((path.clone(), repair.draft.clone()));
                            repair.draft = repair.baseline.clone();
                            repair.draft_changed(active, checks);
                        }
                        if repair.dirty()
                            && self
                                .discarded_repair
                                .as_ref()
                                .is_some_and(|(p, _)| p == path)
                        {
                            self.discarded_repair = None;
                        }
                        if let Some((_, text)) =
                            self.discarded_repair.as_ref().filter(|(p, _)| p == path)
                            && design::draft_discarded(ui, path.rsplit('/').next().unwrap_or(path))
                        {
                            repair.draft = text.clone();
                            self.discarded_repair = None;
                            repair.draft_changed(active, checks);
                        }
                        if repair.dirty() {
                            design::blocked_reason(
                                ui,
                                Some(crate::encounters::draft_switch_reason()),
                            );
                        }
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            // One weight for a save row: primary while the draft has
                            // changes, unless an open run owns `Next turn` or another
                            // repair is the active one.
                            let writes = repair.dirty() && repair.refusal.is_none();
                            let pressed = (if writes && owner.as_deref() == Some(path.as_str()) {
                                Action::primary("Save repaired note")
                            } else {
                                Action::secondary("Save repaired note")
                            })
                            .density(Density::Read)
                            .blocked(
                                repair
                                    .refusal
                                    .as_deref()
                                    .or((!repair.dirty()).then_some("No changes to save.")),
                            )
                            .show(ui)
                            .clicked();

                            if pressed
                                || (writes
                                    && ui.input_mut(|i| {
                                        i.consume_key(egui::Modifiers::COMMAND, egui::Key::S)
                                    }))
                            {
                                // Save asks its own question again before it writes.
                                let result = match crate::world::repair::refusal(
                                    active,
                                    path,
                                    &repair.draft,
                                )
                                .0
                                {
                                    Some(refusal) => Err(refusal),
                                    None => write_note(
                                        &active.path,
                                        path,
                                        Some(&repair.baseline),
                                        &repair.draft,
                                    ),
                                };
                                match result {
                                    Ok(_) => {
                                        repair.baseline = repair.draft.clone();
                                        repaired = true;
                                        repair.failure = None;
                                    }
                                    // The cause stays with this repair, in its state slot.
                                    Err(reason) => repair.failure = Some(reason),
                                }
                            }
                        });
                    });
                });
            }
        }
        if self.repairs.iter().any(|repair| {
            ui.data(|data| {
                data.get_temp::<bool>(egui::Id::new(("repair_open", repair.path.as_str())))
                    .unwrap_or(false)
            })
        }) {
            // The package list below draws the one rule; a second rule here would separate nothing.
            ui.add_space(design::L);
        }
        if repaired && !self.dirty() {
            self.reload(ui.ctx(), campaigns, active.clone());
        }
    }

    fn selection_bar(&mut self, ui: &mut egui::Ui, campaigns: &Campaigns, active: &Campaign) {
        ui.scope(|ui| {
            ui.set_max_width(ui.available_width().min(design::LIST_MEASURE));
            self.selection_bar_content(ui, campaigns, active);
        });
    }

    fn selection_bar_content(
        &mut self,
        ui: &mut egui::Ui,
        campaigns: &Campaigns,
        active: &Campaign,
    ) {
        let snapshot = self.snapshot.as_mut().unwrap();
        let catalogue = &snapshot.catalogue;
        let document = &mut snapshot.document;
        if snapshot.closure.0 != document.selections {
            snapshot.closure = (
                document.selections.clone(),
                catalogue.closure(&document.selections, document.campaign.id),
            );
        }
        let mut reload = false;
        let c = colors();
        let mut reason_below = None;
        ui.horizontal(|ui| {
            let dirty = document.selections != self.saved_selections || document.baseline.is_none();
            let repair_text = self
                .repairs
                .first()
                .map(|repair| format!("Fix {} first.", repair.path));
            let repair = repair_text.as_deref();
            let save_key = dirty
                && repair.is_none()
                && ui.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::S));
            let changed = document.selections != self.saved_selections;
            if changed {
                design::status(
                    ui,
                    design::Status {
                        icon: icons::DRAFT,
                        word: "Unsaved",
                        color: c.document.unsaved,
                    },
                );
                self.discarded_selections = None;
                if Action::quiet("Discard")
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                {
                    self.discarded_selections = Some((
                        std::mem::replace(&mut document.selections, self.saved_selections.clone()),
                        document.baseline.clone(),
                    ));
                }
            } else {
                design::save_row_state(
                    ui,
                    design::Status {
                        icon: icons::OK,
                        word: "No changes",
                        color: c.muted,
                    },
                    false,
                    None,
                );
            }
            if !changed
                && self.discarded_selections.is_some()
                && design::draft_discarded(ui, "Campaign packages")
                && let Some((selections, _)) = self.discarded_selections.take()
            {
                document.selections = selections;
            }
            if !dirty && !self.repairs.iter().any(Repair::dirty) {
                reload = Action::quiet("Reload from disk")
                    .icon(icons::REFRESH)
                    .density(Density::Read)
                    .show(ui)
                    .clicked();
            }
            if document.selections != self.saved_selections {
                design::blocked_reason(ui, Some(crate::encounters::draft_switch_reason()));
            }
            let pressed = ui
                .with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let blocked = repair.or((!dirty).then_some("No changes to save."));
                    let pressed = (if self.run_open {
                        Action::secondary("Save selections")
                    } else {
                        Action::primary("Save selections")
                    })
                    .density(Density::Read)
                    .key("Ctrl+S")
                    .blocked(blocked)
                    .show(ui)
                    .clicked();
                    // One place at every width: beside Save it would fit only once `Reload from
                    // disk` hides, so the first repair edit would move the editor.
                    reason_below = repair.map(str::to_owned);
                    pressed
                })
                .inner;
            if pressed || save_key {
                for selection in &mut document.selections {
                    if let Some(module) = catalogue
                        .resolve_ref(&selection.package)
                        .ok()
                        .and_then(|id| catalogue.modules.iter().find(|module| module.id == id))
                    {
                        selection.name = Some(module.name.clone());
                    }
                }
                match document.save() {
                    Ok(()) => {
                        self.saved_selections = document.selections.clone();
                        self.reconcile.checked_at = None;
                        self.message = Some("Campaign selections saved.".into());
                        self.error = None;
                    }
                    Err(e) => self.error = Some(e),
                }
            }
        });
        if let Some(reason) = reason_below {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    design::blocked_reason(ui, Some(&reason));
                });
            });
        }

        if reload {
            self.reload(ui.ctx(), campaigns, active.clone());
        }
    }

    fn packages(&mut self, ui: &mut egui::Ui, campaigns: &Campaigns, active: &Campaign) {
        ui.scope(|ui| {
            ui.set_max_width(ui.available_width().min(design::LIST_MEASURE));
            self.packages_content(ui, campaigns, active);
        });
    }

    fn packages_content(&mut self, ui: &mut egui::Ui, campaigns: &Campaigns, active: &Campaign) {
        let manifest_locked = self
            .dirty()
            .then_some("Save or discard campaign changes first.");
        let snapshot = self.snapshot.as_mut().unwrap();
        let catalogue = &snapshot.catalogue;
        let document = &mut snapshot.document;
        let closure = &snapshot.closure.1;
        let c = colors();

        for module in campaigns.modules() {
            if let Some(problem) = catalogue.manifest_problems.get(&module.id) {
                let path = &catalogue.entries[&module.id].entry.path;
                let exists = problem != crate::content_workspace::DELETED_MANIFEST
                    && module.path.join(path).exists();
                let label = if exists {
                    format!("Replace with a basic {path}")
                } else {
                    "Write the manifest again".into()
                };
                let message = format!(
                    "{}: {problem}{}",
                    module.name,
                    if problem == crate::content_workspace::DELETED_MANIFEST {
                        ""
                    } else {
                        ". Entries are still available."
                    }
                );
                let mut open = false;
                let tone = crate::content_workspace::manifest_fault_tone(exists);
                let pressed = if exists {
                    design::message_with_actions(ui, tone, &message, |ui| {
                        let mut pressed = false;
                        ui.horizontal_wrapped(|ui| {
                            open = Action::secondary("Open in Library").show(ui).clicked();
                            pressed = design::locked(ui, manifest_locked, |ui| {
                                Action::secondary(&label).show(ui).clicked()
                            });
                        });
                        ui.label(
                            Type::Small
                                .rich(
                                    ui,
                                    crate::content_workspace::manifest_replace_consequence(path),
                                )
                                .color(colors().muted),
                        );
                        pressed
                    })
                } else {
                    design::locked(ui, manifest_locked, |ui| {
                        design::message_with_action(ui, tone, &message, &label)
                    })
                };
                if pressed {
                    match crate::content_workspace::ManifestWrite::write_as(module, path, exists) {
                        Ok(write) => {
                            self.manifest_write = Some(write);
                            self.refresh_pending = true;
                        }
                        Err(error) => self.error = Some(error),
                    }
                }
                if open {
                    ui.ctx().data_mut(|data| {
                        data.insert_temp(
                            egui::Id::new("open_library_repair"),
                            module.path.join(path),
                        )
                    });
                }
            }
        }
        // While a draft waits, only Undo greys: the write happened, so its words keep the text
        // colour, and the draft's own save row says why.
        if let Some(write) = &self.manifest_write
            && crate::library::kept_undo_message(
                ui,
                Tone::Success,
                &write.message(),
                &format!("Undo write {}", write.path()),
                manifest_locked,
                false,
            )
        {
            match write.undo() {
                Ok(_) => {
                    self.manifest_write = None;
                    self.refresh_pending = true;
                }
                Err(error) => self.error = Some(error),
            }
        }
        let affected: Vec<_> = catalogue
            .entries
            .values()
            .filter(|entry| entry.private && entry.entry.kind == "encounter")
            .filter(|entry| {
                splat::content::required_references(&entry.entry).is_ok_and(|references| {
                    references.iter().any(|r| catalogue.resolve(r).is_err())
                })
            })
            .map(|entry| (entry.entry.id, entry.entry.name.clone()))
            .collect();
        if let Some(removed) = &self.removed_package
            && removed.campaign == active.id
        {
            let undo = if removed.users.is_empty() {
                design::undo_message(
                    ui,
                    Tone::Success,
                    &format!("{} removed from this campaign.", removed.name),
                    "Undo package removal",
                )
            } else {
                let names: Vec<_> = removed
                    .users
                    .iter()
                    .map(|(_, name)| name.as_str())
                    .collect();
                let (undo, link) = design::undo_message_with_links(
                    ui,
                    Tone::Warning,
                    &names,
                    if names.len() == 1 {
                        "now has creatures Splat cannot find."
                    } else {
                        "now have creatures Splat cannot find."
                    },
                    "Undo package removal",
                );
                if let Some(at) = link {
                    self.open_lookup = Some(removed.users[at].0);
                }
                undo
            };
            if undo {
                if document.selections != removed.after {
                    self.error = Some(
                        "Campaign choices changed. Keep them and edit the package choices here."
                            .into(),
                    );
                } else {
                    let after = document.selections.clone();
                    document.selections = removed.before.clone();
                    match document.save() {
                        Ok(()) => {
                            self.saved_selections = document.selections.clone();
                            self.removed_package = None;
                            self.reconcile.checked_at = None;
                        }
                        Err(error) => {
                            document.selections = after;
                            self.error = Some(error);
                        }
                    }
                }
            }
        }
        let campaign_flags = catalogue
            .entries
            .get(&active.id)
            .map(|entry| entry.entry.flags.as_slice())
            .unwrap_or_default();
        let errors: Vec<_> = catalogue
            .errors
            .iter()
            .filter(|error| {
                !campaign_flags
                    .iter()
                    .any(|flag| error.contains(&flag.message))
                    && !catalogue.manifest_problems.iter().any(|(id, problem)| {
                        error.contains(problem)
                            || catalogue
                                .entries
                                .get(id)
                                .is_some_and(|entry| error.contains(&entry.entry.path))
                    })
            })
            .chain(&closure.errors)
            .collect();
        let missing_package = document.selections.iter().any(|selection| {
            catalogue
                .resolve_ref(&selection.package)
                .ok()
                .is_none_or(|id| !catalogue.modules.iter().any(|module| module.id == id))
        });
        let issues_shown = self.repairs.is_empty()
            && !missing_package
            && !self
                .removed_package
                .as_ref()
                .is_some_and(|removed| removed.campaign == active.id && !removed.users.is_empty());
        let flag_lines = |ui: &mut egui::Ui, open_run_settings: &mut bool| {
            for (at, flag) in campaign_flags.iter().enumerate() {
                // Every flag is about Campaign.md, so one file line follows the last of them.
                let last = at + 1 == campaign_flags.len();
                ui.scope(|ui| {
                    ui.set_max_width(ui.available_width().min(design::LIST_MEASURE));
                    let words = format!("Line {}: {}", flag.line, flag.message);
                    let choice = flag.kind == splat::content::layout::FlagKind::RuleChoice;
                    if !choice && !last {
                        design::message(ui, Tone::Warning, &words);
                        return;
                    }
                    // The repair choice and the file line start at the warning's words.
                    design::message_with_actions(ui, Tone::Warning, &words, |ui| {
                        if choice {
                            *open_run_settings |= Action::secondary("Choose in Rules and runs")
                                .density(Density::Read)
                                .show(ui)
                                .clicked();
                        }
                        if last {
                            crate::encounters::file_line(ui, &active.path, &document.path);
                        }
                    });
                });
            }
        };
        // Campaign.md flags stay in view while another repair hides the issue list.
        if !issues_shown && !campaign_flags.is_empty() {
            flag_lines(ui, &mut self.open_run_settings);
        }
        if (!errors.is_empty() || !campaign_flags.is_empty()) && issues_shown {
            let heading = if affected.is_empty() {
                design::count(
                    errors.len() + campaign_flags.len(),
                    "content issue to repair",
                    "content issues to repair",
                )
            } else {
                let gone = format!(
                    "{} need a creature that is gone",
                    design::count(affected.len(), "encounter", "encounters")
                );
                if campaign_flags.is_empty() {
                    gone
                } else {
                    format!(
                        "{gone} · {}",
                        design::count(
                            campaign_flags.len(),
                            "Campaign.md issue",
                            "Campaign.md issues"
                        )
                    )
                }
            };
            // A fault count is a status: the warning icon and colour, then the count.
            let format = egui::TextFormat {
                font_id: egui::TextStyle::Button.resolve(ui.style()),
                color: c.result.warning,
                ..Default::default()
            };
            let mut job = egui::text::LayoutJob::default();
            job.append(icons::WARN, 0.0, format.clone());
            job.append(" ", 0.0, format.clone());
            job.append(&heading, 0.0, format);
            let issues = egui::CollapsingHeader::new(job)
                .icon(design::section_icon)
                .default_open(!campaign_flags.is_empty())
                .show(ui, |ui| {
                    egui::ScrollArea::vertical()
                        .max_height(design::FORM_WIDTH / 2.0)
                        .id_salt("campaign_issues")
                        .show(ui, |ui| {
                            flag_lines(ui, &mut self.open_run_settings);
                            for e in errors.iter().filter(|error| {
                                !closure
                                    .missing_dependencies
                                    .iter()
                                    .any(|issue| issue.message.as_str() == error.as_str())
                            }) {
                                ui.add(
                                    egui::Label::new(
                                        Type::Small.rich(ui, e.as_str()).color(c.muted),
                                    )
                                    .wrap(),
                                );
                            }
                            for issue in &closure.missing_dependencies {
                                ui.push_id(&issue.reference, |ui| {
                                    // FA6: a dependency issue is a warning like the flags above
                                    // it, and its repair links start at its words.
                                    let repair = design::message_with_actions(
                                        ui,
                                        Tone::Warning,
                                        &issue.message,
                                        |ui| {
                                            let mut repair = None;
                                            for id in &issue.owners {
                                                let owner = &catalogue.entries[id];
                                                let action = format!("Repair {}", owner.entry.name);
                                                if let Some(path) = catalogue.file_path_of(*id)
                                                    && design::link(ui, &action).clicked()
                                                {
                                                    repair = Some(path.to_path_buf());
                                                }
                                            }
                                            repair
                                        },
                                    );
                                    if let Some(path) = repair {
                                        ui.ctx().data_mut(|data| {
                                            data.insert_temp(
                                                egui::Id::new("open_library_repair"),
                                                path,
                                            )
                                        });
                                    }
                                    design::quiet_section(ui, "Technical details", |ui| {
                                        ui.label(
                                            Type::Small.rich(ui, &issue.reference).color(c.muted),
                                        );
                                    });
                                });
                            }
                            for entry in catalogue.entries.values().filter(|entry| {
                                entry.private
                                    && !closure
                                        .missing_dependencies
                                        .iter()
                                        .any(|issue| issue.owners.contains(&entry.entry.id))
                            }) {
                                if splat::content::required_references(&entry.entry).is_ok_and(
                                    |references| {
                                        references
                                            .iter()
                                            .any(|reference| catalogue.resolve(reference).is_err())
                                    },
                                ) && design::link(ui, &format!("Open {}", entry.entry.name))
                                    .clicked()
                                {
                                    self.open_lookup = Some(entry.entry.id);
                                }
                            }
                        });
                });
            design::named(ui, &issues.header_response, &heading);
        }
        // Unavailable registrations and references remain explicit and editable.
        let unavailable: Vec<_> = document
            .selections
            .iter()
            .enumerate()
            .filter(|(_, s)| {
                catalogue
                    .resolve_ref(&s.package)
                    .ok()
                    .is_none_or(|id| !catalogue.modules.iter().any(|m| m.id == id))
            })
            .map(|(i, selection)| {
                let mut selection = selection.clone();
                if let Some(module) = campaigns
                    .modules()
                    .iter()
                    .find(|module| selection.package.id == Some(module.id))
                {
                    selection.name = Some(module.name.clone());
                }
                (i, selection)
            })
            .collect();
        let mut remove = None;
        for (i, selection) in unavailable {
            let registered = campaigns
                .modules()
                .iter()
                .find(|module| selection.package.id == Some(module.id))
                .cloned();
            let known = registered.clone().or_else(|| {
                selection
                    .package
                    .id
                    .and_then(|id| campaigns.unregistered_module(id).cloned())
            });
            let unregistered = registered.is_none() && known.is_some();
            let relocated = known
                .as_ref()
                .filter(|known| !known.path.is_dir())
                .and_then(|known| relocated_package(known, campaigns.modules()));
            ui.push_id(i, |ui| {
                let message = if let (Some(old), Some(moved)) = (&known, &relocated) {
                    format!(
                        "{} moved from {}/ to {}/.",
                        moved.name,
                        folder_words(&old.path),
                        folder_words(&moved.path)
                    )
                } else if unregistered {
                    format!("{} is not registered.", selection.display_name())
                } else {
                    selection.unavailable_message()
                };
                let text = design::message(ui, Tone::Warning, &message);
                let remaining = egui::Rect::from_min_max(
                    egui::pos2(text.rect.left(), ui.cursor().top()),
                    ui.max_rect().max,
                );
                ui.scope_builder(egui::UiBuilder::new().max_rect(remaining), |ui| {
                    let users = selection.campaign_users(catalogue);
                    if !users.is_empty() {
                        let names: Vec<_> = users.iter().map(|(_, name)| name.as_str()).collect();
                        if let Some(at) = design::linked_sentence(
                            ui,
                            &names,
                            if names.len() == 1 {
                                "uses its creatures."
                            } else {
                                "use its creatures."
                            },
                        ) {
                            self.open_lookup = Some(users[at].0);
                        }
                    }
                    if relocated.is_none()
                        && let Some(known) = &known
                    {
                        ui.label(
                            Type::Small
                                .rich(ui, format!("It was in {}/.", folder_words(&known.path)))
                                .color(c.muted),
                        );
                    }
                    ui.horizontal_wrapped(|ui| {
                        if let Some(module) = &relocated
                            && Action::secondary("Use this folder")
                                .blocked(manifest_locked)
                                .density(Density::Read)
                                .show(ui)
                                .clicked()
                        {
                            self.recovered = Some((module.clone(), registered.clone()));
                            self.found_package = Some(module.clone());
                        }
                        if unregistered
                            && relocated.is_none()
                            && Action::secondary("Register it again")
                                .blocked(manifest_locked)
                                .density(Density::Read)
                                .show(ui)
                                .clicked()
                            && let Some(known) = &known
                        {
                            match find_package_folder(
                                &selection.package,
                                selection.name.as_deref(),
                                &known.path,
                            ) {
                                Ok(module) => {
                                    self.recovered = Some((module.clone(), registered.clone()));
                                    self.found_package = Some(module);
                                }
                                Err(error) => self.error = Some(error),
                            }
                        }
                        if (if relocated.is_some() {
                            Action::quiet("Find the folder…")
                        } else {
                            Action::secondary("Find the folder…")
                        })
                        .blocked_if(
                            document.selections != self.saved_selections
                                || self.repairs.iter().any(Repair::dirty),
                            "Save or discard changes before finding the folder.",
                        )
                        .density(Density::Read)
                        .show(ui)
                        .clicked()
                            && let Some(folder) = crate::hermetic::pick_folder_from(
                                known.as_ref().and_then(|module| module.path.parent()),
                            )
                        {
                            match find_package_folder(
                                &selection.package,
                                selection.name.as_deref(),
                                &folder,
                            ) {
                                Ok(module) => {
                                    self.recovered = Some((module.clone(), registered.clone()));
                                    self.found_package = Some(module);
                                }
                                Err(error) => self.error = Some(error),
                            }
                        }
                        if Action::quiet("Remove from this campaign")
                            .density(Density::Read)
                            .show(ui)
                            .clicked()
                        {
                            remove = Some(i);
                        }
                    });
                    design::quiet_section(ui, "Technical details", |ui| {
                        ui.label(Type::Mono.rich(
                            ui,
                            format!("Package reference: {}", selection.package.label()),
                        ));
                    });
                });
            });
        }
        if let Some(i) = remove {
            let selection = document.selections[i].clone();
            let before = document.selections.clone();
            let users = selection.campaign_users(catalogue);
            document.selections.remove(i);
            match document.save() {
                Ok(()) => {
                    self.saved_selections = document.selections.clone();
                    self.removed_package = Some(RemovedPackage {
                        campaign: active.id,
                        name: selection.display_name().to_owned(),
                        before,
                        after: document.selections.clone(),
                        users,
                    });
                    self.reconcile.checked_at = None;
                }
                Err(error) => {
                    document.selections = before;
                    self.error = Some(error);
                }
            }
        }
        design::rule(ui);
        design::Column::left("campaign_modules", 236.0, 180.0..=420.0)
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                left: 0,
                right: design::M as i8,
                top: design::XS as i8,
                bottom: 0,
            }))
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("package_list")
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for module in catalogue.modules.iter().filter(|m| !m.private) {
                            let existing = document.selections.iter().position(|s| {
                                catalogue.resolve_ref(&s.package).ok() == Some(module.id)
                            });
                            let total = module
                                .entries
                                .iter()
                                .filter(|id| catalogue.selectable(**id))
                                .count();
                            let count =
                                existing.map_or(0, |i| match &document.selections[i].selection {
                                    Selection::All => total,
                                    Selection::Selected { entries } => entries.len(),
                                });
                            // What the campaign's other choices pull in is ticked too.
                            let required = module
                                .entries
                                .iter()
                                .filter(|id| {
                                    catalogue.selectable(**id)
                                        && closure.required.contains_key(id)
                                        && !closure.direct.contains(id)
                                })
                                .count();
                            let partial = count + required < total
                                && existing.is_some_and(|i| {
                                    matches!(&document.selections[i].selection,
                                    Selection::Selected { entries } if !entries.is_empty())
                                });
                            let mut enabled = count > 0;
                            ui.push_id(module.id, |ui| {
                                let rect = egui::Rect::from_min_size(
                                    ui.cursor().min,
                                    egui::vec2(
                                        ui.available_width(),
                                        Density::Glance.row() + design::XS,
                                    ),
                                );
                                let selected = !self.dependencies && self.module == Some(module.id);
                                let hovered = ui.rect_contains_pointer(rect);
                                if selected || hovered {
                                    ui.painter().rect_filled(
                                        rect,
                                        design::RADIUS_CONTROL,
                                        if selected {
                                            colors().selected
                                        } else {
                                            colors().hover
                                        },
                                    );
                                }
                                ui.horizontal(|ui| {
                                    let check = ui
                                        .add(egui::Checkbox::new(&mut enabled, "").indeterminate(
                                            partial || (count == 0 && required > 0),
                                        ));
                                    design::named(ui, &check, &format!("Use {}", module.name));
                                    if check.changed() {
                                        self.message = None;
                                        if enabled {
                                            if let Some(i) = existing {
                                                document.selections.remove(i);
                                            }
                                            document.selections.push(PackageSelection {
                                                name: Some(module.name.clone()),
                                                package: EntryRef::id(module.id),
                                                selection: self
                                                    .disabled_selections
                                                    .remove(&module.id)
                                                    .unwrap_or(Selection::All),
                                            });
                                        } else if let Some(i) = existing {
                                            let removed = document.selections.remove(i);
                                            self.disabled_selections
                                                .insert(module.id, removed.selection);
                                        }
                                    }
                                    if design::two_line_row(
                                        ui,
                                        Density::Glance.row() + design::XS,
                                        !self.dependencies && self.module == Some(module.id),
                                        &module.name,
                                        &if count == 0 && required > 0 {
                                            format!(
                                                "None chosen · {}",
                                                design::count(required, "needed", "needed")
                                            )
                                        } else if required > 0 {
                                            format!(
                                                "{} of {} · {} needed",
                                                design::count(count + required, "", "").trim(),
                                                design::count(total, "", "").trim(),
                                                design::count(required, "", "").trim()
                                            )
                                        } else {
                                            format!(
                                                "{}{}",
                                                if partial {
                                                    design::count(count, "selected", "selected")
                                                } else if enabled {
                                                    "All".into()
                                                } else {
                                                    "None".into()
                                                },
                                                if total == 0 {
                                                    String::new()
                                                } else {
                                                    format!(
                                                        " · {}",
                                                        design::count(total, "entry", "entries")
                                                    )
                                                },
                                            )
                                        },
                                    )
                                    .clicked()
                                    {
                                        self.module = Some(module.id);
                                        self.dependencies = false;
                                        self.search.clear();
                                    }
                                });
                                if count == 0 && required > 0 {
                                    let selection = PackageSelection {
                                        name: Some(module.name.clone()),
                                        package: EntryRef::id(module.id),
                                        selection: Selection::All,
                                    };
                                    let users = selection.campaign_users(catalogue);
                                    let mut users = users;
                                    users.sort_by(|a, b| a.1.cmp(&b.1));
                                    let names = users
                                        .iter()
                                        .map(|(_, name)| name.as_str())
                                        .collect::<Vec<_>>();
                                    let names = match names.as_slice() {
                                        [] => String::new(),
                                        [name] => (*name).to_owned(),
                                        [rest @ .., last] => {
                                            format!("{} and {last}", rest.join(", "))
                                        }
                                    };
                                    if !names.is_empty() {
                                        ui.label(
                                            Type::Small
                                                .rich(ui, format!("{names} use its creatures."))
                                                .color(c.muted),
                                        );
                                    }
                                }
                            });
                        }
                        design::rule(ui);
                        let needed_count = if closure.required.is_empty() {
                            String::new()
                        } else {
                            design::count(closure.required.len(), "entry", "entries")
                        };
                        if design::two_line_row(
                            ui,
                            Density::Glance.row() + design::XS,
                            self.dependencies,
                            "Needed by your entries",
                            &needed_count,
                        )
                        .clicked()
                        {
                            self.dependencies = true;
                            self.search.clear();
                        }
                    });
            });
        egui::CentralPanel::default()
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                left: design::M as i8,
                right: 0,
                top: design::XS as i8,
                bottom: 0,
            }))
            .show_inside(ui, |ui| {
                if self.dependencies {
                    ui.horizontal(|ui| {
                        ui.set_min_height(Density::Read.control());
                        design::group_heading(ui, "Needed by your entries", closure.required.len());
                    });
                    design::search_field(
                        ui,
                        "Find a needed entry",
                        &mut self.search,
                        "Find an entry…",
                        ui.available_width(),
                    );
                    let query = self.search.to_lowercase();
                    let mut required: Vec<_> = closure
                        .required
                        .iter()
                        .filter(|(id, _)| {
                            catalogue.entries[id]
                                .entry
                                .name
                                .to_lowercase()
                                .contains(&query)
                        })
                        .collect();
                    required.sort_by_key(|(id, _)| catalogue.entries[id].entry.name.to_lowercase());
                    egui::ScrollArea::vertical()
                        .id_salt("required_entries")
                        .auto_shrink([false, true])
                        .show(ui, |ui| {
                            for (id, reasons) in required {
                                let entry = &catalogue.entries[id];
                                let owner = if entry.private {
                                    "Campaign".to_owned()
                                } else {
                                    catalogue.package_name(entry.owner)
                                };
                                let detail = format!(
                                    "{} · {owner} · needed by {}",
                                    crate::design::reader::human_label(&entry.entry.kind),
                                    reasons.iter().cloned().collect::<Vec<_>>().join(", ")
                                );
                                if design::two_line_row(
                                    ui,
                                    Density::Glance.row() + design::XS,
                                    false,
                                    &entry.entry.name,
                                    &detail,
                                )
                                .clicked()
                                {
                                    self.open_lookup = Some(*id);
                                }
                            }
                        });
                    return;
                }
                let Some(module) = catalogue
                    .modules
                    .iter()
                    .find(|m| Some(m.id) == self.module)
                    .or_else(|| catalogue.modules.first())
                else {
                    return;
                };
                self.module = Some(module.id);
                let selection_index = document
                    .selections
                    .iter()
                    .position(|s| catalogue.resolve_ref(&s.package).ok() == Some(module.id));
                let entry_ids: Vec<_> = module
                    .entries
                    .iter()
                    .copied()
                    .filter(|id| catalogue.selectable(*id))
                    .collect();
                let all = selection_index
                    .is_some_and(|i| document.selections[i].selection == Selection::All);
                ui.horizontal_wrapped(|ui| {
                    ui.set_min_height(Density::Read.control());
                    design::group_heading(ui, &module.name, entry_ids.len());
                    ui.label(
                        Type::Small
                            .rich(
                                ui,
                                if all {
                                    "Whole package, with future additions"
                                } else {
                                    "Chosen entries only"
                                },
                            )
                            .color(c.muted),
                    );
                    if !all
                        && Action::secondary(
                            if entry_ids.iter().all(|id| {
                                closure.direct.contains(id) || closure.required.contains_key(id)
                            }) {
                                "Include future additions"
                            } else {
                                "Use all"
                            },
                        )
                        .density(Density::Read)
                        .blocked_if(all, "This whole package is already in use.")
                        .hint("Every entry, including future additions")
                        .show(ui)
                        .clicked()
                    {
                        if let Some(i) = selection_index {
                            document.selections[i].selection = Selection::All;
                        } else {
                            document.selections.push(PackageSelection {
                                name: Some(module.name.clone()),
                                package: EntryRef::id(module.id),
                                selection: Selection::All,
                            });
                        }
                    }
                    if Action::quiet("Use none")
                        .density(Density::Read)
                        .blocked_if(selection_index.is_none(), "Nothing here is chosen.")
                        .show(ui)
                        .clicked()
                        && let Some(i) = selection_index
                    {
                        document.selections.remove(i);
                    }
                });
                design::search_field(
                    ui,
                    "Find an entry in this package",
                    &mut self.search,
                    "Find an entry…",
                    ui.available_width(),
                );
                let query = self.search.to_lowercase();
                if self
                    .entry_matches
                    .as_ref()
                    .is_none_or(|(module_id, cached_query, _)| {
                        *module_id != module.id || *cached_query != query
                    })
                {
                    let mut ids: Vec<_> = entry_ids
                        .iter()
                        .copied()
                        .filter(|id| {
                            catalogue.entries[id]
                                .entry
                                .name
                                .to_lowercase()
                                .contains(&query)
                        })
                        .collect();
                    ids.sort_by_cached_key(|id| catalogue.entries[id].entry.name.to_lowercase());
                    self.entry_matches = Some((module.id, query.clone(), ids));
                }
                let ids = &self.entry_matches.as_ref().unwrap().2;
                let mut toggled = None;
                egui::ScrollArea::vertical()
                    .id_salt(("package_entries", module.id, &query))
                    .auto_shrink([false, true])
                    .show_rows(ui, 30.0, ids.len(), |ui, range| {
                        for index in range {
                            let id = ids[index];
                            let entry = &catalogue.entries[&id].entry;
                            let required = closure.required.contains_key(&id);
                            let mut selected = closure.direct.contains(&id) || required;
                            ui.push_id(id, |ui| {
                                ui.horizontal(|ui| {
                                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);
                                    ui.set_min_height(30.0);
                                    let why = closure.required.get(&id).map(|reasons| {
                                        reasons.iter().cloned().collect::<Vec<_>>().join(", ")
                                    });
                                    let response = ui
                                        .add_enabled(
                                            !required,
                                            egui::Checkbox::new(&mut selected, ""),
                                        )
                                        .on_hover_text(&entry.name)
                                        .on_disabled_hover_text(why.as_ref().map_or_else(
                                            || entry.name.clone(),
                                            |why| {
                                                format!(
                                                    "{} is needed by {why}, so it stays selected.",
                                                    entry.name
                                                )
                                            },
                                        ));
                                    design::named(ui, &response, &format!("Use {}", entry.name));
                                    if crate::encounters::creature_name_link(ui, &entry.name)
                                        .clicked()
                                    {
                                        self.open_lookup = Some(id);
                                    }
                                    if response.changed() {
                                        toggled = Some((id, selected));
                                    }
                                    ui.label(
                                        Type::Small
                                            .rich(
                                                ui,
                                                snapshot
                                                    .subtitles
                                                    .get(&id)
                                                    .cloned()
                                                    .unwrap_or_default(),
                                            )
                                            .color(c.muted),
                                    );
                                    if let Some(why) = &why {
                                        ui.label(
                                            Type::Small
                                                .rich(ui, format!("Needed by {why}"))
                                                .color(c.document.read_only),
                                        );
                                    }
                                });
                            });
                        }
                    });
                if let Some((id, checked)) = toggled {
                    let selection_index = document
                        .selections
                        .iter()
                        .position(|s| catalogue.resolve_ref(&s.package).ok() == Some(module.id));
                    let mut selected =
                        match selection_index.map(|i| &document.selections[i].selection) {
                            Some(Selection::All) => entry_ids
                                .iter()
                                .map(|id| EntryRef::id(*id))
                                .collect::<Vec<_>>(),
                            Some(Selection::Selected { entries }) => entries.clone(),
                            None => Vec::new(),
                        };
                    selected.retain(|r| catalogue.resolve_ref(r).ok() != Some(id));
                    if checked {
                        selected.push(EntryRef::id(id));
                    }
                    let empty = selected.is_empty();
                    let selection = Selection::Selected { entries: selected };
                    if empty {
                        if let Some(i) = selection_index {
                            document.selections.remove(i);
                        }
                    } else if let Some(i) = selection_index {
                        document.selections[i].selection = selection;
                    } else {
                        document.selections.push(PackageSelection {
                            name: Some(module.name.clone()),
                            package: EntryRef::id(module.id),
                            selection,
                        });
                    }
                }
            });
    }
}

/// Read what the Packages page shows: invalid notes to repair, and the campaign's packages.
pub(crate) fn load_snapshot(campaign: &Campaign, modules: &[Module]) -> Loaded {
    let repairs: Vec<Repair> = splat::content::read_folder(&campaign.path)
        .map(|folder| {
            folder
                .errors
                .iter()
                .filter_map(|e| {
                    let path = note_path(&campaign.path, &e.path).ok()?;
                    let text = match folder.text(&e.path) {
                        Some(text) => text.to_owned(),
                        None => fs::read_to_string(path).ok()?,
                    };
                    Some(Repair::new(e.path.clone(), text, campaign))
                })
                .collect()
        })
        .unwrap_or_default();
    let result = CampaignDocument::load_with_modules(campaign, modules).and_then(|document| {
        let catalogue = Catalogue::load(modules, Some(campaign))?;
        let closure = catalogue.closure(&document.selections, document.campaign.id);
        Ok(Snapshot {
            closure: (document.selections.clone(), closure),
            document,
            subtitles: crate::library::catalogue_subtitles(&catalogue),
            catalogue,
        })
    });
    (repairs, result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn folder_recovery_requires_exactly_one_matching_identity() {
        let temp = tempfile::tempdir().unwrap();
        let old = temp.path().join("original");
        fs::create_dir(&old).unwrap();
        let text = crate::library::note_text("Test", "package", Some("test"));
        fs::write(old.join("Package.md"), &text).unwrap();
        let module = crate::library::local_module(&old).unwrap();
        assert!(relocated_package(&module, &[]).is_none());
        let moved = temp.path().join("moved");
        fs::rename(&old, &moved).unwrap();
        assert_eq!(relocated_package(&module, &[]).unwrap().path, moved);
        let copy = temp.path().join("copy");
        fs::create_dir(&copy).unwrap();
        fs::write(copy.join("Package.md"), text).unwrap();
        assert!(relocated_package(&module, &[]).is_none());
        crate::hermetic::closed::next_folder(moved.clone());
        assert_eq!(
            crate::hermetic::pick_folder_from(Some(temp.path())),
            Some(moved)
        );
        assert!(crate::hermetic::pick_folder().is_none());
    }

    fn campaign(root: &std::path::Path) -> Campaign {
        let campaign = Campaign {
            id: uuid::Uuid::new_v4(),
            name: "Test campaign".into(),
            path: root.into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        if campaign.path.is_dir() && !campaign.path.join("Campaign.md").exists() {
            crate::content_workspace::CampaignDocument::new(&campaign)
                .unwrap()
                .save()
                .unwrap();
        }
        campaign
    }

    fn finished(content: &mut CampaignContent, checked: Checked) {
        let (tx, rx) = mpsc::channel();
        content.reconcile.pending = Some(rx);
        tx.send(checked).unwrap();
        content.finish_check();
    }

    #[test]
    fn iter8_e_campaign_deleted_manifest_action_does_not_guess_a_filename() {
        for filename in ["Package.md", "Module.md"] {
            let temp = tempfile::tempdir().unwrap();
            let root = temp.path().join("campaign");
            fs::create_dir(&root).unwrap();
            let active = campaign(&root);
            let package = temp.path().join("package");
            fs::create_dir(&package).unwrap();
            fs::write(
                package.join("Package.md"),
                crate::library::note_text("Test", "package", Some("test")),
            )
            .unwrap();
            let module = crate::library::local_module(&package).unwrap();
            if filename == "Module.md" {
                fs::rename(module.path.join("Package.md"), module.path.join(filename)).unwrap();
            }
            let mut campaigns = Campaigns::at(temp.path().join("settings.json"));
            campaigns.register_module(module.clone()).unwrap();
            fs::remove_file(module.path.join(filename)).unwrap();
            let mut content = CampaignContent {
                snapshot: Some(load_snapshot(&active, &[module]).1.unwrap()),
                ..Default::default()
            };
            let ctx = egui::Context::default();
            let mut rendered = String::new();
            for _ in 0..2 {
                let output = ctx.run(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(1200.0, 2400.0),
                        )),
                        ..Default::default()
                    },
                    |ctx| {
                        egui::CentralPanel::default().show(ctx, |ui| {
                            content.packages_content(ui, &campaigns, &active);
                        });
                    },
                );
                rendered = output
                    .shapes
                    .iter()
                    .filter_map(|shape| match &shape.shape {
                        egui::Shape::Text(text) => Some(text.galley.job.text.as_str()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
            }
            assert!(rendered.contains("Write the manifest again"), "{rendered}");
            assert!(
                rendered.contains("The package manifest was deleted."),
                "{rendered}"
            );
            assert!(!rendered.contains("Missing YAML frontmatter"), "{rendered}");
            for guess in [
                "Package.md was deleted",
                "Module.md was deleted",
                "Write Package.md again",
                "Write Module.md again",
            ] {
                assert!(!rendered.contains(guess), "{rendered}");
            }
        }
    }

    #[test]
    fn an_unclosed_list_names_and_marks_its_opening_line() {
        let text = "---\nsplat_schema: 2\nsplat_kind: [\n---\n";
        let (problem, line) = repair_problem("Notes/Broken.md", text);
        assert_eq!(line, Some(3));
        assert_eq!(
            problem.as_deref(),
            Some("Line 3 opens a list with [ that never closes.")
        );
        // Round 23 FA3: the save row asks Save's own question, the one World's repair asks. A
        // draft Save refuses names its cause; one Save writes has none.
        let active = campaign(std::path::Path::new("/nonexistent/splat-campaign"));
        let refusal = |draft: &str| Repair::new("Notes/Broken.md".into(), draft.into(), &active);
        assert_eq!(
            refusal(text).refusal.as_deref(),
            Some("Line 3 opens a list with [ that never closes.")
        );
        assert_eq!(refusal(text).error_line, Some(3));
        let rule = crate::library::note_text("Broken", "rule", None);
        assert_eq!(refusal(&rule).refusal, None);
        // A draft that parses but takes an identity settings own is still refused.
        let ownership = Some("Keep campaign ownership consistent with settings.".to_owned());
        let package = crate::library::note_text("Broken", "package", Some("broken"));
        assert_eq!(refusal(&package).refusal, ownership);
        let other = crate::library::note_text("Broken", "campaign", Some("broken"));
        assert_eq!(refusal(&other).refusal, ownership);
        let own = other.replace(
            &splat::content::Entry::parse("Campaign.md", &other)
                .unwrap()
                .id
                .to_string(),
            &active.id.to_string(),
        );
        assert_eq!(refusal(&own).refusal, None);
        // Round 23 review: the check runs when the draft changes, and the change ends this
        // repair's failed Save.
        let mut repair = refusal(&rule);
        repair.failure = Some("This file is read only.".into());
        let mut checks = 0;
        repair.draft = package.clone();
        repair.draft_changed(&active, &mut checks);
        assert_eq!(
            (repair.refusal, repair.failure, checks),
            (ownership, None, 1)
        );
    }

    #[test]
    fn a_check_reaches_the_sidebar_and_the_review_list_and_changes_nothing() {
        let temp = tempfile::tempdir().unwrap();
        let active = campaign(temp.path());
        let campaigns = Campaigns::at(temp.path().join("settings.json"));
        let mut content = CampaignContent::default();
        assert_eq!(content.review_detail(), None);
        let proposal = |key: &str, blocked: Option<&str>| Proposal {
            key: key.into(),
            name: "Witch Bolt".into(),
            kind: "spell".into(),
            change: Move::ToPackage("Player's Handbook".into()),
            blocked: blocked.map(str::to_owned),
        };
        finished(
            &mut content,
            Checked {
                applied: None,
                proposals: Ok(vec![
                    proposal("a", None),
                    proposal("b", Some("It has personal notes.")),
                ]),
            },
        );
        assert_eq!(content.review_detail().as_deref(), Some("2 to review"));
        assert!(content.reconcile.pending.is_none());
        assert!(!content.refresh_pending);
        // Showing the list starts nothing. Only the DM's click applies a change.
        let _ = egui::Context::default().run(Default::default(), |ctx| {
            egui::CentralPanel::default()
                .show(ctx, |ui| content.reconcile_section(ui, &campaigns, &active));
        });
        assert!(content.reconcile.pending.is_none());
    }

    #[test]
    fn applied_changes_reload_the_catalogue_and_failures_are_shown() {
        let mut content = CampaignContent::default();
        finished(
            &mut content,
            Checked {
                applied: Some(Ok(2)),
                proposals: Ok(Vec::new()),
            },
        );
        assert!(content.refresh_pending);
        assert_eq!(content.review_detail(), None);
        assert!(
            content
                .reconcile
                .message
                .as_deref()
                .unwrap()
                .starts_with("2 changes")
        );
        finished(
            &mut content,
            Checked {
                applied: Some(Err("A sheet has local edits.".into())),
                proposals: Ok(Vec::new()),
            },
        );
        assert_eq!(
            content.reconcile.error.as_deref(),
            Some("A sheet has local edits.")
        );
    }

    #[test]
    fn closure_follows_a_selection_change() {
        let temp = tempfile::tempdir().unwrap();
        let active = campaign(temp.path());
        let campaigns = Campaigns::at(temp.path().join("settings.json"));
        let catalogue = Catalogue::load(&[], None).unwrap();
        let document = CampaignDocument::load(&active).unwrap();
        let closure = catalogue.closure(&document.selections, active.id);
        let mut content = CampaignContent {
            active: Some(active.clone()),
            snapshot: Some(Snapshot {
                closure: (document.selections.clone(), closure),
                document,
                subtitles: crate::library::catalogue_subtitles(&catalogue),
                catalogue,
            }),
            ..Default::default()
        };
        let ctx = egui::Context::default();
        let frame = |content: &mut CampaignContent| {
            let _ = ctx.run(Default::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    content.selection_bar(ui, &campaigns, &active);
                    content.packages(ui, &campaigns, &active);
                });
            });
        };
        let kept =
            |content: &CampaignContent| content.snapshot.as_ref().unwrap().closure.0.as_ptr();
        let before = kept(&content);
        frame(&mut content);
        assert_eq!(
            kept(&content),
            before,
            "the same selections keep the closure"
        );
        let direct =
            |content: &CampaignContent| content.snapshot.as_ref().unwrap().closure.1.direct.len();
        assert!(
            direct(&content) > 0,
            "a new campaign uses the built-in package"
        );
        let snapshot = content.snapshot.as_mut().unwrap();
        snapshot.document.selections.clear();
        frame(&mut content);
        assert_eq!(direct(&content), 0);
    }

    #[test]
    fn undo_discard_stays_in_its_campaign() {
        let temp = tempfile::tempdir().unwrap();
        let (first, second) = (
            campaign(&temp.path().join("a")),
            campaign(&temp.path().join("b")),
        );
        let campaigns = Campaigns::at(temp.path().join("settings.json"));
        let ctx = egui::Context::default();
        let mut content = CampaignContent {
            active: Some(first.clone()),
            discarded_selections: Some((Vec::new(), None)),
            discarded_repair: Some(("note.md".into(), "words".into())),
            ..Default::default()
        };
        content.reload(&ctx, &campaigns, first);
        assert!(content.discarded_selections.is_some(), "a refresh keeps it");
        assert!(content.discarded_repair.is_some(), "a refresh keeps it");
        content.reload(&ctx, &campaigns, second);
        assert!(content.discarded_selections.is_none());
        assert!(content.discarded_repair.is_none());

        // Round 23 FA8: the discard belongs to Campaign.md as it was read. The same file keeps
        // it; a changed or missing file ends it.
        let folder = temp.path().join("c");
        fs::create_dir(&folder).unwrap();
        let active = campaign(&folder);
        let document = CampaignDocument::load(&active).unwrap();
        let discarded = || Some((Vec::new(), document.baseline.clone()));
        content.discarded_selections = discarded();
        content.forget_discard_of_changed_file(Ok(&CampaignDocument::load(&active).unwrap()));
        assert!(
            content.discarded_selections.is_some(),
            "the same file keeps it"
        );
        let file = active.path.join(&document.path);
        fs::write(
            &file,
            format!("{}\nA later note.\n", fs::read_to_string(&file).unwrap()),
        )
        .unwrap();
        content.forget_discard_of_changed_file(Ok(&CampaignDocument::load(&active).unwrap()));
        assert!(
            content.discarded_selections.is_none(),
            "a changed file ends it"
        );
        content.discarded_selections = discarded();
        fs::remove_file(&file).unwrap();
        let missing = CampaignDocument::load(&active).map(|_| ()).unwrap_err();
        content.forget_discard_of_changed_file(Err(&missing));
        assert!(
            content.discarded_selections.is_none(),
            "a missing file ends it"
        );
    }
    /// AD1: `Undo package removal` goes when the file's package choices changed after it.
    #[test]
    fn iter14_e_changed_choices_end_undo_package_removal() {
        use crate::content_workspace::{CampaignDocument, EntryRef, PackageSelection, Selection};
        let temp = tempfile::tempdir().unwrap();
        for folder in ["a", "b"] {
            fs::create_dir(temp.path().join(folder)).unwrap();
        }
        let active = campaign(&temp.path().join("a"));
        let other = campaign(&temp.path().join("b"));
        let selection = |name: &str| PackageSelection {
            name: Some(name.into()),
            package: EntryRef::id(uuid::Uuid::new_v4()),
            selection: Selection::All,
        };
        let mut document = CampaignDocument::load(&active).unwrap();
        let before = vec![selection("Gone")];
        document.selections = Vec::new();
        document.save().unwrap();
        let removed = |campaign: &Campaign| RemovedPackage {
            campaign: campaign.id,
            name: "Gone".into(),
            before: before.clone(),
            after: Vec::new(),
            users: Vec::new(),
        };
        let mut content = CampaignContent {
            removed_package: Some(removed(&active)),
            ..Default::default()
        };
        // A reload of the file as the removal left it keeps Undo, and other edits keep it too.
        let file = active.path.join(&document.path);
        let text = format!("{}\nA later note.\n", fs::read_to_string(&file).unwrap());
        fs::write(&file, &text).unwrap();
        content.forget_changed_removal(&CampaignDocument::load(&active).unwrap());
        assert!(content.removed_package.is_some());
        // The file's package choices changed outside Splat: the result and its Undo go.
        let mut changed = CampaignDocument::load(&active).unwrap();
        changed.selections.push(selection("Added in Obsidian"));
        changed.save().unwrap();
        let bytes = fs::read(&file).unwrap();
        content.forget_changed_removal(&CampaignDocument::load(&active).unwrap());
        assert!(content.removed_package.is_none());
        assert_eq!(fs::read(&file).unwrap(), bytes, "the check writes nothing");
        // A removal from another campaign is not judged by this campaign's file.
        content.removed_package = Some(removed(&other));
        content.forget_changed_removal(&CampaignDocument::load(&active).unwrap());
        assert!(content.removed_package.is_some());
    }

    #[test]
    fn finding_a_package_rejects_another_packages_folder() {
        let dir = tempfile::tempdir().unwrap();
        let expected = dir.path().join("expected");
        let other = dir.path().join("other");
        fs::create_dir(&expected).unwrap();
        fs::create_dir(&other).unwrap();
        let text = crate::library::note_text("Ashen Marches", "package", Some("ashen"));
        let entry = write_note(&expected, "Package.md", None, &text).unwrap();
        let text = crate::library::note_text("Other package", "package", Some("other"));
        write_note(&other, "Package.md", None, &text).unwrap();
        let reference = EntryRef::id(entry.id);
        assert_eq!(
            find_package_folder(&reference, Some("Ashen Marches"), &expected)
                .unwrap()
                .name,
            "Ashen Marches"
        );
        // Round 23 FE3: the fault comes first and names the picked folder and the package.
        let folder = other.file_name().unwrap().to_string_lossy().into_owned();
        assert_eq!(
            find_package_folder(&reference, Some("Ashen Marches"), &other).unwrap_err(),
            format!(
                "{folder}/ holds a different package. Choose the folder that holds Ashen Marches."
            )
        );
    }
}
