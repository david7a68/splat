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

struct Snapshot {
    document: CampaignDocument,
    catalogue: Catalogue,
    /// The closure of these selections. It is too slow to calculate on every frame.
    closure: (Vec<PackageSelection>, Closure),
    subtitles: std::collections::HashMap<Uuid, String>,
}
/// An invalid note: its path, its text on disk, and its draft.
type Repair = (String, String, String);
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
    search: String,
    entry_matches: Option<(Uuid, String, Vec<Uuid>)>,
    /// The DM's choice of list. Until they choose, a package opens on the list it needs.
    pub run_open: bool,
    pub open_lookup: Option<Uuid>,
    pub found_package: Option<Module>,
    error: Option<String>,
    message: Option<String>,
    disabled_selections: std::collections::BTreeMap<Uuid, Selection>,
    refresh_pending: bool,
    removed_package: Option<RemovedPackage>,
    repairs: Vec<Repair>,
    /// What `Discard` last dropped, for one step of undo.
    discarded_selections: Option<Vec<PackageSelection>>,
    discarded_repair: Option<(String, String)>,
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
                    "line {line} opens a {thing} with {ch} that never closes"
                )),
                Some(*line),
            );
        }
        if let Some(location) = error.location() {
            let line = location.line() + 1;
            return (
                Some(format!(
                    "Check the YAML on line {line}. Use a name, a colon and its value"
                )),
                Some(line),
            );
        }
    }
    (Some(problem), None)
}
/// A folder recovery must keep the selected package's identity.
fn find_package_folder(reference: &EntryRef, folder: &std::path::Path) -> Result<Module, String> {
    let module = crate::library::local_module(folder)?;
    let matches = if let Some(id) = reference.id {
        id == module.id
    } else {
        let catalogue = Catalogue::load(std::slice::from_ref(&module), None)?;
        catalogue.resolve_ref(reference).ok() == Some(module.id)
    };
    if !matches {
        return Err(
            "Choose the folder for this package. That folder contains a different package.".into(),
        );
    }
    Ok(module)
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
    #[cfg(test)]
    pub fn busy(&self) -> bool {
        self.pending.is_some()
    }

    pub fn dirty(&self) -> bool {
        self.repairs
            .iter()
            .any(|(_, baseline, draft)| baseline != draft)
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
                .filter(|id| snapshot.catalogue.selectable(**id))
                .count();
            let mut parts = Vec::new();
            if chosen > 0 {
                parts.push(design::count(chosen, "entry chosen", "entries chosen"));
            }
            if needed > 0 {
                parts.push(design::count(
                    needed,
                    "needed by your entries",
                    "needed by your entries",
                ));
            }
            parts.join(" · ")
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
        }
    }
    fn reload(&mut self, ctx: &egui::Context, campaigns: &Campaigns, campaign: Campaign) {
        self.forget_discards_of_other_campaigns(&campaign);
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
            let repairs = splat::content::read_directory(&campaign.path)
                .map(|(_, errors)| {
                    errors
                        .into_iter()
                        .filter_map(|e| {
                            let text = fs::read_to_string(note_path(&campaign.path, &e.path).ok()?)
                                .ok()?;
                            Some((e.path, text.clone(), text))
                        })
                        .collect()
                })
                .unwrap_or_default();
            let result = CampaignDocument::load(&campaign).and_then(|document| {
                let catalogue = Catalogue::load(&modules, Some(&campaign))?;
                let closure = catalogue.closure(&document.selections, document.campaign.id);
                Ok(Snapshot {
                    closure: (document.selections.clone(), closure),
                    document,
                    subtitles: crate::library::catalogue_subtitles(&catalogue),
                    catalogue,
                })
            });
            let _ = tx.send((repairs, result));
            ctx.request_repaint();
        });
    }
    pub fn show(&mut self, ui: &mut egui::Ui, campaigns: &Campaigns) {
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
                    match result {
                        Ok(snapshot) => {
                            self.saved_selections = snapshot.document.selections.clone();
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
        self.repair_section(ui, campaigns, &active);
        if let Some(error) = &self.error {
            design::message(ui, Tone::Error, error);
        }
        if let Some(message) = &self.message {
            design::message(ui, Tone::Success, message);
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
        let mut repaired = false;
        for (path, baseline, draft) in &mut self.repairs {
            let (problem, error_line) = repair_problem(path, draft);
            let problem = problem
                .as_deref()
                .unwrap_or("The repair has not been saved");
            let problem = problem.replace('[', "`[`").replace('{', "`{`");
            design::message_with_code(
                ui,
                Tone::Error,
                &format!("{path} cannot be read. Fix it here or in World. {}.", {
                    let mut text = problem.trim_end_matches('.').to_owned();
                    if let Some(first) = text.get_mut(..1) {
                        first.make_ascii_uppercase();
                    }
                    text
                }),
            );
            let id = egui::Id::new(("repair_open", path.as_str()));
            let mut open = ui.data(|data| data.get_temp::<bool>(id)).unwrap_or(false);
            if Action::quiet(if open { "Close repair" } else { "Repair" })
                .show(ui)
                .clicked()
            {
                open = !open;
            }
            ui.data_mut(|data| data.insert_temp(id, open));
            if open {
                ui.horizontal(|ui| {
                    design::group_heading(ui, "Repair", 0);
                    ui.label(Type::Mono.rich(ui, path.as_str()));
                });
                ui.scope(|ui| {
                    ui.set_max_width(design::READER_MEASURE);
                    // What is wrong comes first, and stays true as the DM edits.

                    crate::world::repair::editor(ui, path, draft, error_line);

                    ui.horizontal(|ui| {
                        if draft != baseline
                            && Action::quiet("Discard repair draft")
                                .density(Density::Read)
                                .show(ui)
                                .clicked()
                        {
                            self.discarded_repair = Some((path.clone(), draft.clone()));
                            *draft = baseline.clone();
                        }
                        if draft != baseline
                            && self
                                .discarded_repair
                                .as_ref()
                                .is_some_and(|(p, _)| p == path)
                        {
                            self.discarded_repair = None;
                        }
                        if let Some((_, text)) =
                            self.discarded_repair.as_ref().filter(|(p, _)| p == path)
                        {
                            ui.label(
                                Type::Small
                                    .rich(ui, "Draft discarded.")
                                    .color(colors().muted),
                            );
                            if Action::secondary("Undo discard")
                                .density(Density::Read)
                                .show(ui)
                                .clicked()
                            {
                                *draft = text.clone();
                                self.discarded_repair = None;
                            }
                        }
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let pressed = Action::secondary("Save repaired note")
                                .density(Density::Read)
                                .blocked_if(draft == baseline, "No changes yet.")
                                .show(ui)
                                .clicked();
                            design::blocked_reason(
                                ui,
                                (draft == baseline).then_some("No changes yet."),
                            );
                            if pressed
                                || (draft != baseline
                                    && ui.input_mut(|i| {
                                        i.consume_key(egui::Modifiers::COMMAND, egui::Key::S)
                                    }))
                            {
                                let result = Entry::parse(path, draft).and_then(|entry| {
                                    if entry.kind == "package"
                                        || (entry.kind == "campaign" && entry.id != active.id)
                                    {
                                        return Err(
                                            "Keep campaign ownership consistent with settings."
                                                .into(),
                                        );
                                    }
                                    write_note(&active.path, path, Some(baseline), draft)
                                });
                                match result {
                                    Ok(_) => {
                                        *baseline = draft.clone();
                                        repaired = true;
                                    }
                                    Err(e) => self.error = Some(e),
                                }
                            }
                        });
                    });
                });
            }
        }
        if repaired && !self.dirty() {
            self.reload(ui.ctx(), campaigns, active.clone());
        }
    }

    fn packages(&mut self, ui: &mut egui::Ui, campaigns: &Campaigns, active: &Campaign) {
        let snapshot = self.snapshot.as_mut().unwrap();
        let catalogue = &snapshot.catalogue;
        let document = &mut snapshot.document;
        if snapshot.closure.0 != document.selections {
            snapshot.closure = (
                document.selections.clone(),
                catalogue.closure(&document.selections, document.campaign.id),
            );
        }
        let closure = &snapshot.closure.1;
        let mut reload = false;
        let c = colors();
        ui.horizontal(|ui| {
            let dirty = document.selections != self.saved_selections || document.baseline.is_none();
            let repair_text = self
                .repairs
                .first()
                .map(|(path, _, _)| format!("Fix {path} first."));
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
                if Action::quiet("Discard selection changes")
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                {
                    self.discarded_selections = Some(std::mem::replace(
                        &mut document.selections,
                        self.saved_selections.clone(),
                    ));
                }
            } else if self.discarded_selections.is_some() {
                ui.label(Type::Small.rich(ui, "Selections discarded.").color(c.muted));
                if Action::secondary("Undo discard")
                    .density(Density::Read)
                    .hint("Bring back the selection changes you discarded")
                    .show(ui)
                    .clicked()
                    && let Some(selections) = self.discarded_selections.take()
                {
                    document.selections = selections;
                }
            }
            if !dirty && !self.repairs.iter().any(|(_, b, d)| b != d) {
                reload = Action::quiet("Reload from disk")
                    .icon(icons::REFRESH)
                    .density(Density::Read)
                    .show(ui)
                    .clicked();
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
                    design::blocked_reason(ui, blocked);
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
            let undo = design::message_with_action(
                ui,
                Tone::Success,
                &format!("{} removed from this campaign.", removed.name),
                "Undo package removal",
            );
            ui.horizontal_wrapped(|ui| {
                for (at, (id, name)) in removed.users.iter().enumerate() {
                    if at > 0 {
                        ui.label("and");
                    }
                    if design::link(ui, name).clicked() {
                        self.open_lookup = Some(*id);
                    }
                }
                if !removed.users.is_empty() {
                    ui.label(
                        Type::Small
                            .rich(ui, "still use its creatures.")
                            .color(c.muted),
                    );
                }
            });
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
        let errors: Vec<_> = catalogue
            .errors
            .iter()
            .chain(closure.errors.iter())
            .collect();
        if !errors.is_empty() && self.repairs.is_empty() {
            design::section(
                ui,
                if affected.is_empty() {
                    design::count(
                        errors.len(),
                        "content issue to repair",
                        "content issues to repair",
                    )
                } else {
                    format!(
                        "{} need a creature that is gone",
                        design::count(affected.len(), "encounter", "encounters")
                    )
                },
                |ui| {
                    egui::ScrollArea::vertical()
                        .max_height(120.0)
                        .id_salt("campaign_issues")
                        .show(ui, |ui| {
                            for e in errors {
                                ui.add(
                                    egui::Label::new(
                                        Type::Small.rich(ui, e.as_str()).color(c.muted),
                                    )
                                    .wrap(),
                                );
                            }
                            for entry in catalogue.entries.values().filter(|entry| entry.private) {
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
                },
            );
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
            ui.push_id(i, |ui| {
                design::message(ui, Tone::Warning, &selection.unavailable_message(catalogue));
                for (id, name) in selection.campaign_users(catalogue) {
                    if design::link(ui, &name).clicked() {
                        self.open_lookup = Some(id);
                    }
                }
                ui.horizontal_wrapped(|ui| {
                    if Action::quiet("Remove from this campaign")
                        .density(Density::Read)
                        .show(ui)
                        .clicked()
                    {
                        remove = Some(i);
                    }
                    if Action::secondary("Find the folder…")
                        .blocked_if(
                            document.selections != self.saved_selections
                                || self
                                    .repairs
                                    .iter()
                                    .any(|(_, baseline, draft)| baseline != draft),
                            "Save or discard changes before finding the folder.",
                        )
                        .density(Density::Read)
                        .show(ui)
                        .clicked()
                        && let Some(folder) = crate::hermetic::pick_folder()
                    {
                        match find_package_folder(&selection.package, &folder) {
                            Ok(module) => self.found_package = Some(module),
                            Err(error) => self.error = Some(error),
                        }
                    }
                });
                design::quiet_section(ui, "Technical details", |ui| {
                    ui.label(Type::Mono.rich(
                        ui,
                        format!("Package reference: {}", selection.package.label()),
                    ));
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
                            let required =
                                existing.map_or(0, |i| match &document.selections[i].selection {
                                    Selection::All => 0,
                                    Selection::Selected { entries } => module
                                        .entries
                                        .iter()
                                        .filter(|id| {
                                            catalogue.selectable(**id)
                                                && closure.required.contains_key(id)
                                                && !entries.iter().any(|e| {
                                                    catalogue.resolve_ref(e).ok() == Some(**id)
                                                })
                                        })
                                        .count(),
                                });
                            let partial = count + required < total
                                && existing.is_some_and(|i| {
                                    matches!(&document.selections[i].selection,
                                    Selection::Selected { entries } if !entries.is_empty())
                                });
                            let mut enabled = count > 0;
                            ui.push_id(module.id, |ui| {
                                ui.horizontal(|ui| {
                                    let check = ui.add(
                                        egui::Checkbox::new(&mut enabled, "")
                                            .indeterminate(partial),
                                    );
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
                                        &if required > 0 {
                                            format!(
                                                "{} of {} · {} required",
                                                design::count(count + required, "", "").trim(),
                                                design::count(total, "", "").trim(),
                                                design::count(required, "", "").trim()
                                            )
                                        } else {
                                            format!(
                                                "{} · {}",
                                                if partial {
                                                    design::count(count, "selected", "selected")
                                                } else if enabled {
                                                    "All".into()
                                                } else {
                                                    "None".into()
                                                },
                                                design::count(total, "entry", "entries"),
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
                    design::group_heading(ui, "Needed by your entries", closure.required.len());
                    design::search_field(ui, "Find a needed entry", &mut self.search, "Find an entry…", ui.available_width());
                    let query = self.search.to_lowercase();
                    let mut required: Vec<_> = closure.required.iter().filter(|(id, _)| catalogue.entries[id].entry.name.to_lowercase().contains(&query)).collect();
                    required.sort_by_key(|(id, _)| catalogue.entries[id].entry.name.to_lowercase());
                    egui::ScrollArea::vertical().id_salt("required_entries").auto_shrink([false, true]).show(ui, |ui| {
                        for (id, reasons) in required {
                            let entry = &catalogue.entries[id];
                            let owner = if entry.private { "Campaign".to_owned() } else { catalogue.package_name(entry.owner) };
                            let detail = format!("{} · {owner} · needed by {}", crate::design::reader::human_label(&entry.entry.kind), reasons.iter().cloned().collect::<Vec<_>>().join(", "));
                            if design::two_line_row(ui, Density::Glance.row() + design::XS, false, &entry.entry.name, &detail).clicked() { self.open_lookup = Some(*id); }
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
                    ui.label(Type::Section.rich(ui, &module.name).color(c.text));
                    if Action::secondary("Use all")
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
                                                    "{} is required by {why}, so it stays selected.",
                                                    entry.name
                                                )
                                            },
                                        ))
                                        ;
                                    design::named(ui, &response, &format!("Use {}", entry.name));
                                    if crate::encounters::creature_name_link(ui, &entry.name).clicked() { self.open_lookup = Some(id); }
                                    if response.changed() {
                                        toggled = Some((id, selected));
                                    }
                                    ui.label(
                                        Type::Small
                                            .rich(ui, snapshot.subtitles.get(&id).cloned().unwrap_or_default())
                                            .color(c.muted),
                                    );
                                    if let Some(why) = &why {
                                        ui.label(
                                            Type::Small
                                                .rich(ui, format!("Required by {why}"))
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
        if reload {
            self.reload(ui.ctx(), campaigns, active.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn campaign(root: &std::path::Path) -> Campaign {
        Campaign {
            id: uuid::Uuid::new_v4(),
            name: "Test campaign".into(),
            path: root.into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        }
    }

    fn finished(content: &mut CampaignContent, checked: Checked) {
        let (tx, rx) = mpsc::channel();
        content.reconcile.pending = Some(rx);
        tx.send(checked).unwrap();
        content.finish_check();
    }

    #[test]
    fn an_unclosed_list_names_and_marks_its_opening_line() {
        let text = "---\nsplat_schema: 1\nsplat_kind: [\n---\n";
        let (problem, line) = repair_problem("Notes/Broken.md", text);
        assert_eq!(line, Some(3));
        assert_eq!(
            problem.as_deref(),
            Some("line 3 opens a list with [ that never closes")
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
                egui::CentralPanel::default()
                    .show(ctx, |ui| content.packages(ui, &campaigns, &active));
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
            discarded_selections: Some(Vec::new()),
            discarded_repair: Some(("note.md".into(), "words".into())),
            ..Default::default()
        };
        content.reload(&ctx, &campaigns, first);
        assert!(content.discarded_selections.is_some(), "a refresh keeps it");
        assert!(content.discarded_repair.is_some(), "a refresh keeps it");
        content.reload(&ctx, &campaigns, second);
        assert!(content.discarded_selections.is_none());
        assert!(content.discarded_repair.is_none());
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
            find_package_folder(&reference, &expected).unwrap().name,
            "Ashen Marches"
        );
        assert!(
            find_package_folder(&reference, &other)
                .unwrap_err()
                .contains("different package")
        );
    }
}
