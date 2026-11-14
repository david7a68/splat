//! One catalogue: built-in, package and campaign-owned entries, with their authoring.
//! Browsing it never changes the active campaign.
#![allow(deprecated)] // Old message functions; Wave 2 moves this file to `design::outcome`.
mod class;
mod creature;
pub(crate) mod presentation;
mod reader;
pub(crate) mod view;
use crate::{
    campaigns::Campaigns,
    content_workspace::{CampaignDocument, Catalogue},
    design::{self, Action, Density, Status, Tone, Type, colors, icons, reader as reader_style},
    history::{self, Step, Trail},
    matching::{CLOSE_NAMES, Rank, Typed, with_other},
    persistence::{Campaign, Module},
    world::file_fault::FileFault,
};
use eframe::egui;
use presentation::Section;
use splat::content::{
    self, Entry,
    creature_filter::{Facets, Filter, Query},
};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    sync::mpsc,
};
use uuid::Uuid;

const BUILTIN_NAME: &str = "D&D 2024 · SRD 5.2.1";

/// The choice after a package copy's registration names what it changes, in its result's words.
pub(crate) const USE_COPY: &str = "Use the copy in encounters";
pub(crate) const USE_COPY_RESULT: &str = "Encounters now use the copy.";

/// Why source editing is blocked for a file that the system will not read.
const UNREAD_EDIT: &str =
    "Splat cannot read this file. Fix it outside Splat, then choose Try again.";

/// A file that cannot be read: what happened, how to recover, and Try again, which starts at the
/// message words. The system's words go to Technical details. Nothing here changes the file.
/// Returns whether Try again was chosen.
/// A read fault's first sentence in the reader, below the file line that names the file: it
/// says `this file`, so the path shows once. Messages away from the reader keep the path.
fn fault_here(fault: &FileFault) -> String {
    use crate::world::file_fault::Kind;
    match fault.kind {
        Kind::NotText => "Splat cannot read this file as text.".to_owned(),
        Kind::ReadDenied => "Splat has no permission to read this file.".to_owned(),
        Kind::Gone => "Splat cannot find this file.".to_owned(),
        Kind::CannotRead => "Splat cannot open this file.".to_owned(),
        Kind::WriteDenied | Kind::Exists | Kind::DiskFull | Kind::CannotWrite => fault.headline(),
    }
}

/// The recovery steps of a read fault. They are prose: they keep the reader measure inside the
/// message's list measure, as World's do (round 25 FB3, SB1).
fn recovery_paragraph(ui: &mut egui::Ui, fault: &FileFault) {
    ui.scope(|ui| {
        ui.set_max_width(ui.available_width().min(design::READER_MEASURE));
        ui.add(
            egui::Label::new(
                Type::Body
                    .rich(ui, fault.recovery_with_try_again())
                    .color(colors().text),
            )
            .wrap(),
        );
    });
}

fn fault_notice(ui: &mut egui::Ui, fault: &FileFault) -> bool {
    let again = design::message_with_actions(ui, Tone::Error, &fault_here(fault), |ui| {
        recovery_paragraph(ui, fault);
        // A message action without an icon is a bordered secondary button (round 25).
        Action::secondary("Try again")
            .density(Density::Read)
            .show(ui)
            .clicked()
    });
    design::gap(ui, design::Gap::S);
    design::quiet_section(ui, "Technical details", |ui| {
        ui.add(egui::Label::new(Type::Small.rich(ui, &fault.system).color(colors().muted)).wrap());
    });
    again
}

/// What is wrong with a repair draft now, and the line to mark. Any reason Save would refuse the
/// draft (`refusal`, from `Loaded::draft_refusal`) is the notice, a YAML fault in its own words. A
/// saved file whose own text does not parse has a parse fault: a draft Save would write has no
/// notice. Another fault (a read fault, a deleted manifest, a manifest with another identity) is
/// not in the draft's text, so its stored words stay until the file changes.
fn draft_repair_problem(
    note: &Note,
    draft: &str,
    refusal: Option<String>,
    recovery: Option<&(Module, String, bool)>,
) -> (Option<String>, Option<usize>) {
    let (problem, line) = crate::campaign_content::repair_problem(&note.path, draft);
    // A file the system will not read has no text to repair: its reader names the read fault.
    if note.fault.is_some() {
        return (None, line);
    }
    let Some(error) = note.error.as_ref() else {
        // A readable entry's draft: Save's own refusal, with a YAML fault in its own words.
        return (refusal.map(|refusal| problem.unwrap_or(refusal)), line);
    };
    if recovery.is_some_and(|(_, _, exists)| !exists) {
        return (Some(error.clone()), line);
    }
    let parse_fault = || {
        crate::campaign_content::repair_problem(&note.path, &note.text)
            .0
            .is_some()
    };
    match problem.or(refusal) {
        Some(problem) => (Some(problem), line),
        None if parse_fault() => (None, line),
        None => (Some(error.clone()), line),
    }
}

/// A manifest's recovery in Edit Markdown: the bordered action at the message text edge, then
/// its consequence. It follows the error in the pane's message footer: the action depends on the
/// saved file only, so it stays while the DM edits and moves nothing.
#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
)]
fn recovery_action(
    ui: &mut egui::Ui,
    (_, path, exists): &(Module, String, bool),
    blocked: Option<&str>,
) -> bool {
    let inset = design::S
        + design::HAIRLINE_WIDTH
        + design::number_width(ui, icons::ERROR, &Type::Body.font(ui))
        + ui.spacing().item_spacing.x * 2.0
        - design::XS;
    let label = if *exists {
        format!("Replace with a basic {path}")
    } else {
        "Write the manifest again".into()
    };
    ui.scope(|ui| {
        ui.set_max_width(ui.available_width().min(EDITOR_MAX_WIDTH));
        let pressed = ui
            .horizontal(|ui| {
                ui.add_space((inset - ui.spacing().item_spacing.x).max(0.0));
                design::locked(ui, blocked, |ui| {
                    Action::secondary(&label)
                        .density(Density::Read)
                        .show(ui)
                        .clicked()
                })
            })
            .inner;
        if *exists {
            ui.horizontal(|ui| {
                ui.add_space((inset - ui.spacing().item_spacing.x).max(0.0));
                // Help keeps the reader measure (round 26).
                design::help(
                    ui,
                    &crate::content_workspace::manifest_replace_consequence(path),
                );
            });
        }
        pressed
    })
    .inner
}

/// Keep the error with its repair and consequence in both reader and source views.
#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
)]
fn repair_notice(
    ui: &mut egui::Ui,
    problem: &str,
    recovery: Option<&(Module, String, bool)>,
    blocked: Option<&str>,
    source: bool,
) -> bool {
    let inset = design::S
        + design::HAIRLINE_WIDTH
        + design::number_width(ui, icons::ERROR, &Type::Body.font(ui))
        + ui.spacing().item_spacing.x * 2.0
        - design::XS;
    if let Some((_, path, exists)) = recovery {
        let label = if *exists {
            format!("Replace with a basic {path}")
        } else {
            "Write the manifest again".into()
        };
        let tone = crate::content_workspace::manifest_fault_tone(*exists);
        let pressed = ui
            .scope(|ui| {
                // Keep the action below the error at the text edge, even in a wide pane.
                let width = (design::number_width(ui, problem, &Type::Body.font(ui)) + inset).max(
                    Action::secondary(&label)
                        .density(Density::Read)
                        .natural_width(ui)
                        + inset,
                );
                ui.set_max_width(ui.available_width().min(width).min(EDITOR_MAX_WIDTH));
                design::locked(ui, blocked, |ui| {
                    design::message_with_action(ui, tone, problem, &label)
                })
            })
            .inner;
        if *exists {
            // The consequence wraps at the pane's notice measure, not at the action's width.
            ui.scope(|ui| {
                ui.set_max_width(ui.available_width().min(EDITOR_MAX_WIDTH));
                ui.horizontal(|ui| {
                    ui.add_space((inset - ui.spacing().item_spacing.x).max(0.0));
                    design::help(
                        ui,
                        &crate::content_workspace::manifest_replace_consequence(path),
                    );
                });
            });
        }
        pressed
    } else if source {
        design::message(ui, Tone::Error, problem);
        false
    } else {
        ui.scope(|ui| {
            let label = "Fix in Edit Markdown";
            let width = (design::number_width(ui, problem, &Type::Body.font(ui)) + inset).max(
                Action::secondary(label)
                    .density(Density::Read)
                    .natural_width(ui)
                    + inset,
            );
            ui.set_max_width(ui.available_width().min(width));
            design::message_with_action(ui, Tone::Error, problem, label)
        })
        .inner
    }
}

fn copy_destination_reason(campaigns: &Campaigns) -> Option<&'static str> {
    (campaigns.modules().is_empty() && campaigns.all().is_empty())
        .then_some("Add a campaign or a package to create entries.")
}

/// Who owns an entry, and so whether and where it can be saved.
#[derive(Clone, Default, PartialEq, Debug)]
enum Owner {
    #[default]
    Builtin,
    Package(Module),
    Campaign(Campaign),
}

impl Owner {
    fn folder(&self) -> String {
        self.root()
            .and_then(Path::file_name)
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned()
    }
    fn id(&self) -> Option<Uuid> {
        match self {
            Self::Builtin => None,
            Self::Package(module) => Some(module.id),
            Self::Campaign(campaign) => Some(campaign.id),
        }
    }
    fn root(&self) -> Option<&Path> {
        match self {
            Self::Builtin => None,
            Self::Package(module) => Some(&module.path),
            Self::Campaign(campaign) => Some(&campaign.path),
        }
    }
    fn editable(&self) -> bool {
        !matches!(self, Self::Builtin)
    }
    fn label(&self) -> String {
        match self {
            Self::Builtin => "Built-in package · Read only".into(),
            Self::Package(module) => format!("Package · {}", module.name),
            Self::Campaign(_) => "Campaign".into(),
        }
    }
    /// Where an entry came from.
    fn source(&self) -> &str {
        match self {
            Self::Builtin => BUILTIN_NAME,
            Self::Package(module) => &module.name,
            Self::Campaign(_) => "Campaign",
        }
    }
}

fn name_list(names: &[String]) -> String {
    let shown = &names[..names.len().min(3)];
    if names.len() > 3 {
        return format!("{} and {} more", shown.join(", "), names.len() - 3);
    }
    match shown {
        [] => String::new(),
        [name] => name.clone(),
        [rest @ .., last] => format!("{} and {last}", rest.join(", ")),
    }
}

/// NPC roster rows keep their stat block and display name, rather than the NPC identity.
pub(crate) fn encounter_uses_entry(encounter: &Entry, entry: &Entry) -> bool {
    if content::required_references(encounter)
        .is_ok_and(|refs| refs.contains(&content::Reference::Id(entry.id)))
    {
        return true;
    }
    let Some(world) = entry
        .fields
        .get("splat_world")
        .filter(|_| entry.kind == "world" && crate::world::fights(entry))
    else {
        return false;
    };
    let Some(creature) = world
        .get("creature")
        .and_then(|creature| creature.get("id"))
        .and_then(serde_yaml_ng::Value::as_str)
    else {
        return false;
    };
    encounter
        .fields
        .get("splat_encounter")
        .and_then(|setup| setup.get("roster"))
        .and_then(serde_yaml_ng::Value::as_sequence)
        .is_some_and(|roster| {
            roster.iter().any(|slot| {
                slot.get("creature")
                    .and_then(|reference| reference.get("id"))
                    .and_then(serde_yaml_ng::Value::as_str)
                    == Some(creature)
                    && slot
                        .get("overrides")
                        .and_then(|changes| changes.get("name"))
                        .and_then(serde_yaml_ng::Value::as_str)
                        == Some(entry.name.as_str())
            })
        })
}

/// The warning for a first heading that differs from the entry's name (its words, its action
/// and the fixed text), and the result message whose `Undo heading change` shows for this draft.
struct HeadingChoice {
    fix: Option<(String, String, String)>,
    undo: Option<String>,
}

impl HeadingChoice {
    /// `entry` is `None` when the file cannot take the fix (read only, unparsed or in a conflict).
    fn new(
        entry: Option<&Entry>,
        draft: &str,
        undo: Option<&(String, String, String)>,
        path: &str,
    ) -> Self {
        let Some(entry) = entry else {
            return Self {
                fix: None,
                undo: None,
            };
        };
        Self {
            fix: presentation::heading_fix(draft, entry).map(|(old, after)| {
                (
                    format!(
                        "The first heading says {old}, but the name is {}.",
                        entry.name
                    ),
                    format!("Use {}", entry.name),
                    after,
                )
            }),
            undo: undo
                .is_some_and(|(at, _, after)| at == path && after == draft)
                .then(|| format!("The first heading now says {}.", entry.name)),
        }
    }

    /// Applies a press on the fix or its Undo to the draft. Returns true when the draft changed
    /// and must be saved.
    fn apply(
        &self,
        (fix, undo): (bool, bool),
        path: &str,
        draft: &mut String,
        heading_undo: &mut Option<(String, String, String)>,
    ) -> bool {
        let mut changed = false;
        if fix && let Some((_, _, after)) = &self.fix {
            *heading_undo = Some((path.to_owned(), draft.clone(), after.clone()));
            draft.clone_from(after);
            changed = true;
        }
        if undo && let Some((_, before, _)) = heading_undo.take() {
            *draft = before;
            changed = true;
        }
        changed
    }
}

/// The duplicate-identity warning and its repair choices. The reader draws them inside its
/// scroll area, so the title and body scroll into view; source mode draws them above the editor.
/// The page's last error, as a reason in the pane's message footer. Its words can hold code in
/// backticks.
fn page_error<'a>(footer: &mut design::Footer<'a>, error: Option<&'a str>) {
    if let Some(error) = error {
        let words = format!("Error: {error}");
        footer.reason(words.clone(), move |ui| {
            design::message_with_code(ui, Tone::Error, &words);
        });
    }
}

/// Each kept result as its own message in the pane's message footer; the footer lists them
/// newest first by when each action was done (`orders`), as Edit Markdown lists its results.
/// `out` gathers what was pressed.
fn add_results<'a>(
    footer: &mut design::Footer<'a>,
    results: &'a ReaderResults,
    orders: &ResultOrders,
    out: &'a std::cell::Cell<ResultPress>,
) {
    for item in results.items() {
        let kind = item.0;
        add_reader_result(footer, results, orders, item, move |ui| {
            let mut pressed = out.get();
            show_result(ui, results, kind, &mut pressed);
            out.set(pressed);
        });
    }
}

/// The pane's message footer when it has no reader: below a form, or below the empty state of a
/// scope with no entries. It holds the page's results and its error. Returns what was pressed.
#[allow(clippy::disallowed_methods)]
fn form_footer(
    ui: &mut egui::Ui,
    id: &str,
    results: &ReaderResults,
    orders: &ResultOrders,
    (form_error, error): (Option<&str>, Option<&str>),
) -> (ResultPress, f32) {
    let pressed = std::cell::Cell::new(ResultPress::default());
    let mut footer = design::Footer::new(id, ui.available_width().min(design::READER_MEASURE));
    add_results(&mut footer, results, orders, &pressed);
    page_error(&mut footer, form_error);
    page_error(&mut footer, error);
    let bottom = footer.show_by_hand(ui);
    (pressed.get(), bottom)
}

/// The form above a form footer, in a scroll area that ends at the footer's top, so a long form
/// never runs under the footer.
#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw egui calls that a design part replaces (design::results::STAGED_ALLOWS)"
)]
fn above_footer<R>(ui: &mut egui::Ui, bottom: f32, add: impl FnOnce(&mut egui::Ui) -> R) -> R {
    let height = (bottom - ui.cursor().top()).max(0.0);
    egui::ScrollArea::vertical()
        .id_salt("library_form_body")
        .max_height(height)
        .auto_shrink([false, true])
        .show(ui, add)
        .inner
}

/// What the DM pressed in the reader's messages in the pane's message footer.
#[derive(Default)]
struct ReaderPress {
    /// The heading fix, then its Undo.
    heading: (bool, bool),
    results: std::cell::Cell<ResultPress>,
    new_identity: Option<usize>,
    new_identities: Option<(PathBuf, Vec<String>)>,
    /// The campaign entry to copy into the package.
    copy_dependency: Option<Uuid>,
    /// The copy result whose Undo was pressed: its place in `dependency_undos`.
    undo_dependency: std::cell::Cell<Option<usize>>,
}

/// The reader's messages that come and go, in the pane's message footer below the body, so the
/// body never moves (round 26 FD2, FC4): the page's results, the heading warning and its result,
/// the duplicate warning with its choices, and a package entry's campaign links with their copy
/// result. Results with a live Undo lead, then the warnings. `heading` is `None` when Edit
/// Markdown's messages already hold it.
#[allow(clippy::too_many_arguments)]
#[allow(
    clippy::disallowed_methods,
    reason = "staged: calls `ResultOrder::issue`, `Footer::result` or `save_row_state` (design::results::STAGED_ALLOWS)"
)]
fn reader_messages<'a>(
    footer: &mut design::Footer<'a>,
    heading: Option<&'a HeadingChoice>,
    results: &'a ReaderResults,
    duplicates: Option<(&'a Loaded, &'a Note, &'a [usize])>,
    draft_reason: Option<&'static str>,
    dependencies: &'a [(Uuid, String, String)],
    dependency_results: &'a [(usize, String, design::ResultOrder)],
    orders: &ResultOrders,
    out: &'a mut ReaderPress,
) {
    let ReaderPress {
        heading: heading_out,
        results: results_out,
        new_identity,
        new_identities,
        copy_dependency,
        undo_dependency,
    } = out;
    let (heading_fix, heading_undo) = (&mut heading_out.0, &mut heading_out.1);
    add_results(footer, results, orders, results_out);
    if let Some(result) = heading.and_then(|heading| heading.undo.as_deref()) {
        // The fix's result is a message, so its Undo keeps its icon on the message line.
        footer.result(order_of(orders, OrderKey::Heading), result, true, |ui| {
            *heading_undo = design::undo_message(ui, Tone::Success, result, "Undo heading change");
        });
    }
    // Round 25 FF8: each result names the entry and the package, in the button's words.
    for (at, result, order) in dependency_results {
        let undo = &*undo_dependency;
        footer.result(*order, result.as_str(), true, move |ui| {
            if design::undo_message(ui, Tone::Success, result, "Undo copy") {
                undo.set(Some(*at));
            }
        });
    }
    if let Some((words, action, _)) = heading.and_then(|heading| heading.fix.as_ref()) {
        footer.notice(words.as_str(), |ui| {
            *heading_fix = design::message_with_action(ui, Tone::Warning, words, action);
        });
    }
    if let Some((loaded, note, indices)) = duplicates.filter(|(_, _, at)| !at.is_empty()) {
        footer.notice(crate::world::DUPLICATE_ENTRY, move |ui| {
            duplicate_choices(
                ui,
                loaded,
                note,
                indices,
                draft_reason,
                new_identity,
                new_identities,
            );
        });
    }
    if !dependencies.is_empty() {
        let words = dependencies
            .iter()
            .map(|(_, words, _)| words.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        footer.notice(words, move |ui| {
            for (id, words, action) in dependencies {
                if design::locked(ui, draft_reason, |ui| {
                    design::message_with_action(ui, Tone::Warning, words, action)
                }) {
                    *copy_dependency = Some(*id);
                }
            }
        });
    }
}

#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
)]
fn duplicate_choices(
    ui: &mut egui::Ui,
    loaded: &Loaded,
    note: &Note,
    duplicate_indices: &[usize],
    draft_reason: Option<&'static str>,
    new_identity: &mut Option<usize>,
    new_identities: &mut Option<(PathBuf, Vec<String>)>,
) {
    // The repair choices start at the message text edge, as in World and Lookup.
    design::message_with_actions(ui, Tone::Warning, crate::world::DUPLICATE_ENTRY, |ui| {
        let users: Vec<_> = loaded
            .notes
            .iter()
            .filter_map(|other| {
                let entry = other.entry.as_ref()?;
                (entry.kind == "encounter"
                    && encounter_uses_entry(entry, note.entry.as_ref().unwrap()))
                .then(|| entry.name.clone())
            })
            .collect();
        for (position, &at) in duplicate_indices.iter().enumerate() {
            let target = &loaded.notes[at];
            let filename = Path::new(&target.path)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy();
            let same_filename = duplicate_indices.iter().any(|&other| {
                other != at
                    && Path::new(&loaded.notes[other].path).file_name()
                        == Path::new(&target.path).file_name()
            });
            let filename = if same_filename {
                format!("{filename} in {}/", target.owner.folder())
            } else {
                filename.into_owned()
            };
            if position > 0 {
                ui.add_space((design::L - ui.spacing().item_spacing.y).max(0.0));
            }
            ui.scope(|ui| {
                ui.spacing_mut().item_spacing.y = design::XS;
                let action = crate::world::separate_entry_action(&filename);
                if Action::secondary(&action)
                    .hint(&action)
                    .blocked(draft_reason)
                    .show(ui)
                    .clicked()
                {
                    *new_identity = Some(at);
                }
                if !users.is_empty() {
                    let remaining = duplicate_indices
                        .iter()
                        .copied()
                        .find(|&other| other != at)
                        .unwrap();
                    let note = &loaded.notes[remaining];
                    let remaining = Path::new(&note.path)
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy();
                    let remaining = if same_filename {
                        format!("{remaining} in {}/", note.owner.folder())
                    } else {
                        remaining.into_owned()
                    };
                    duplicate_repair_consequence(ui, &users, &remaining, position == 0);
                }
            });
        }
        let mut shown = std::collections::HashSet::new();
        for (position, &at) in duplicate_indices.iter().enumerate() {
            let owner = &loaded.notes[at].owner;
            if shown.insert(owner.id())
                && let Some(paths) = loaded.duplicate_folder_paths(owner)
            {
                ui.add_space((design::L - ui.spacing().item_spacing.y).max(0.0));
                // Encounters outside the batch whose source changes; worked out once per load.
                let slot = egui::Id::new((
                    "duplicate_folder_users",
                    owner.id(),
                    std::ptr::from_ref(loaded) as usize,
                    loaded.notes.len(),
                    // A reload in place changes some text; this cheap sum notices it.
                    loaded
                        .notes
                        .iter()
                        .map(|note| note.text.len())
                        .sum::<usize>(),
                ));
                let users = ui
                    .data(|data| data.get_temp::<Vec<String>>(slot))
                    .unwrap_or_else(|| {
                        let batch: std::collections::HashSet<&String> = paths.iter().collect();
                        let targets: Vec<&Entry> = loaded
                            .notes
                            .iter()
                            .filter(|target| {
                                target.owner.id() == owner.id() && batch.contains(&target.path)
                            })
                            .filter_map(|target| target.entry.as_ref())
                            .collect();
                        let mut users: Vec<String> = Vec::new();
                        for other in &loaded.notes {
                            // An encounter inside the batch gets the new identities with it.
                            if other.owner.id() == owner.id() && batch.contains(&other.path) {
                                continue;
                            }
                            let Some(encounter) = other.entry.as_ref() else {
                                continue;
                            };
                            if encounter.kind == "encounter"
                                && !users.contains(&encounter.name)
                                && targets
                                    .iter()
                                    .any(|entry| encounter_uses_entry(encounter, entry))
                            {
                                users.push(encounter.name.clone());
                            }
                        }
                        ui.data_mut(|data| data.insert_temp(slot, users.clone()));
                        users
                    });
                ui.scope(|ui| {
                    ui.spacing_mut().item_spacing.y = design::XS;
                    if Action::secondary(&format!(
                        "Make all {} files in {}/ separate entries",
                        paths.len(),
                        owner.folder()
                    ))
                    .blocked(draft_reason)
                    .show(ui)
                    .clicked()
                    {
                        *new_identities = Some((owner.root().unwrap().to_path_buf(), paths));
                    }
                    let remaining = duplicate_indices
                        .iter()
                        .map(|&other| &loaded.notes[other].owner)
                        .find(|other| other.id() != owner.id())
                        .expect("a folder repair has another owner");
                    duplicate_repair_consequence(
                        ui,
                        &users,
                        &format!("the files in {}/", remaining.folder()),
                        position == 0,
                    );
                });
            }
        }
    });
}

/// Shared consequence wording for the two identity-repair choices.
pub(crate) fn duplicate_repair_consequence(
    ui: &mut egui::Ui,
    users: &[String],
    remaining: &str,
    keeps: bool,
) {
    if !users.is_empty() {
        let verb = match (keeps, users.len()) {
            (true, 1) => "keeps",
            (true, _) => "keep",
            (false, _) => "will then use",
        };
        ui.label(
            Type::Small
                .rich(ui, format!("{} {verb} {remaining}.", name_list(users)))
                .color(colors().muted),
        );
    }
}

fn draft_words(campaign: bool) -> &'static str {
    if campaign {
        "Save or discard to copy, add an entry, change packages or switch campaign."
    } else {
        "Save or discard to copy, add an entry or change packages."
    }
}

/// The reader's kind line keeps its kind, class, level and source.
fn reader_subtitle(owner: &Owner, listed: &str, class_level: Option<String>) -> String {
    match class_level {
        Some(level) => format!("Class feature · {level} · {}", owner.source()),
        None if *owner == Owner::Builtin => format!("{listed} · {BUILTIN_NAME}"),
        None => listed.to_owned(),
    }
}

/// The reader's kind line from its list line. The owner line names the package, so the kind line
/// does not. Where two packages share that name, the list adds the folder, and only the folder
/// stays: `Creature · CR 1 · ashen-marches/`.
fn reader_kind_line(owner: &Owner, listed: &str) -> String {
    let Owner::Package(module) = owner else {
        return listed.to_owned();
    };
    let name = format!(" · {}", module.name);
    let folder = format!(" · {}/", owner.folder());
    match listed.strip_suffix(&format!("{name}{folder}")) {
        Some(head) => format!("{head}{folder}"),
        None => listed.strip_suffix(&name).unwrap_or(listed).to_owned(),
    }
}

/// The scope's name, as its menu says it.
fn scope_name(campaigns: &Campaigns, scope: Scope) -> String {
    match scope {
        Scope::Uses => "Can use".to_owned(),
        Scope::All => "Everything".to_owned(),
        Scope::Builtin => BUILTIN_NAME.to_owned(),
        Scope::Package(id) => campaigns
            .modules()
            .iter()
            .find(|m| m.id == id)
            .map_or_else(|| "A package".to_owned(), |m| m.name.clone()),
        Scope::Campaign => "Campaign entries".to_owned(),
    }
}

/// The action after an empty search or filter that keeps the words in the widest scope.
pub(crate) const SEARCH_EVERYTHING: &str = "Search Everything";

fn scope_label(ui: &egui::Ui, selected: &str, budget: f32) -> String {
    let font = Type::Strong.font(ui);
    let fitted = design::words_that_fit(ui, selected, &font, budget);
    if design::number_width(ui, &fitted, &font) <= budget {
        return fitted;
    }
    let mut cut = fitted.trim_end_matches('…').to_owned();
    while !cut.is_empty() && design::number_width(ui, &format!("{cut}…"), &font) > budget {
        cut.pop();
    }
    format!("{cut}…")
}

fn section_row(ui: &mut egui::Ui, label: &str, count: usize, selected: bool) -> egui::Response {
    design::row(
        ui,
        Density::Read.row(),
        selected,
        &format!("{label}   {}", design::count(count, "", "").trim()),
        |ui, rect| {
            let c = colors();
            let font = if selected { Type::Strong } else { Type::Body };
            let galley = design::one_line(
                ui,
                label,
                font.font(ui),
                if selected { c.text } else { c.muted },
                rect.width() - 44.0,
            );
            ui.painter().galley(
                egui::pos2(
                    rect.left() + design::S,
                    rect.center().y - galley.size().y / 2.0,
                ),
                galley,
                c.text,
            );
            design::number(
                ui.painter(),
                rect.right() - design::S,
                rect.center().y,
                design::count(count, "", "").trim(),
                &Type::Small.font(ui),
                c.muted,
            );
        },
    )
}

fn about_row(ui: &mut egui::Ui, label: &str, selected: bool) -> egui::Response {
    let color = if selected {
        colors().text
    } else {
        colors().muted
    };
    let font = if selected { Type::Strong } else { Type::Body };
    // One line, so a warning icon never moves the rows below. The package name shortens by whole
    // words; hover and the accessible name keep the full label.
    let room = (ui.available_width() - design::S * 2.0).max(design::XXL);
    let (head, name) = label
        .split_once("About ")
        .map_or(("", label), |(icon, name)| (icon, name));
    // Ruling R13 (row 151): the row keeps `About`, the word that says what it opens; the name
    // gives way, by whole words.
    let head = format!("{head}About ");
    let name_room = room - design::number_width(ui, &head, &font.font(ui));
    let shown = format!(
        "{head}{}",
        design::words_that_fit(ui, name, &font.font(ui), name_room)
    );
    // The warning mark has its own tone, as on an entry row.
    let galley = marked_line(ui, &shown, font.font(ui), color, room);
    let shortened = shown != label || galley.elided;
    let height = (galley.size().y + design::XS * 2.0).max(Density::Read.row());
    let response = design::row(ui, height, selected, label, |ui, rect| {
        ui.painter().galley(
            egui::pos2(
                rect.left() + design::S,
                rect.center().y - galley.size().y / 2.0,
            ),
            galley,
            color,
        );
    });
    if shortened {
        response.on_hover_text(label)
    } else {
        response
    }
}

/// What the catalogue shows. The default is what the active campaign can use.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq)]
enum Scope {
    #[default]
    Uses,
    All,
    Builtin,
    Package(Uuid),
    Campaign,
}

/// The place of the open entry, for Back and Forward. A free function, so a list closure that
/// holds the notes can record the place it leaves (round 26 SB1).
fn place(
    loaded: &Loaded,
    selected: Option<usize>,
    scope: Scope,
    section: Section,
    search: &str,
) -> Option<Place> {
    let current = loaded.notes.get(selected?)?;
    Some(Place {
        owner: current.owner.id(),
        path: current.path.clone(),
        scope,
        section,
        search: search.to_owned(),
    })
}

/// A package's About line count, in Campaign's form: `All entries chosen`, `5 of 6 entries
/// chosen`, `No entries chosen`. Round 27 FA6: a ticked box counts as chosen, whoever ticked
/// it, so the entries the DM's choices need (ticked and locked in Campaign) count too.
fn chosen_count(chosen: usize, needed: usize, total: usize) -> String {
    match (chosen + needed).min(total) {
        0 => "No entries chosen".into(),
        ticked if ticked == total => "All entries chosen".into(),
        ticked => format!("{ticked} of {total} entries chosen"),
    }
}

/// How many entries leave Splat's lists when a package is unregistered. The manifest is not one.
fn unregister_count(notes: &[Note], package: Uuid) -> usize {
    notes
        .iter()
        .filter(|n| {
            n.owner.id() == Some(package)
                && !n.is_manifest()
                && n.entry.as_ref().is_none_or(|e| e.kind != "reference")
        })
        .count()
}

pub(crate) struct Note {
    path: String,
    text: String,
    entry: Option<Entry>,
    error: Option<String>,
    /// Why the file could not be read at all, from the system error's kind. Such a note has no
    /// text, so it offers no source editing or repair until a read succeeds.
    fault: Option<FileFault>,
    owner: Owner,
}
impl Note {
    fn is_manifest(&self) -> bool {
        // A readable note is a manifest only by its kind; the file name and the kind line are
        // guesses for a note that cannot be read.
        matches!(self.owner, Owner::Package(_))
            && match &self.entry {
                Some(entry) => entry.kind == "package",
                None => {
                    matches!(self.path.as_str(), "Package.md" | "Module.md")
                        || self.text.lines().any(|line| line == "splat_kind: package")
                }
            }
    }

    /// The About row's words. `flagged` is true when the header counts a problem in this file,
    /// such as a first heading that differs from the name. The mark has the fault's tone: a
    /// manifest that cannot be read is an error, and a deleted one or another problem is a warning.
    fn about_label(&self, flagged: bool) -> String {
        let name = match &self.owner {
            Owner::Package(module) => &module.name,
            _ => self.name(),
        };
        let deleted = self.error.as_deref() == Some(crate::content_workspace::DELETED_MANIFEST);
        let mark = if self.error.is_some() && !deleted {
            Some(icons::ERROR)
        } else if deleted || flagged {
            Some(icons::WARN)
        } else {
            None
        };
        format!(
            "{}About {name}",
            mark.map_or_else(String::new, |mark| format!("{mark} "))
        )
    }

    fn unreadable_kind(&self) -> Section {
        if self.is_manifest() {
            return Section::PackageDetails;
        }
        let declared = self
            .text
            .lines()
            .find_map(|line| line.strip_prefix("splat_kind: "))
            .map(|kind| Section::for_kind(kind.trim().trim_matches(['\'', '"'])))
            .unwrap_or(Section::Unreadable);
        if ![Section::Other, Section::Unreadable].contains(&declared) {
            return declared;
        }
        self.text
            .lines()
            .find_map(|line| {
                let kind = line.strip_prefix("splat_")?.split_once(':')?.0;
                let section = Section::for_kind(kind);
                (![Section::Other, Section::Unreadable].contains(&section)).then_some(section)
            })
            .unwrap_or(Section::Unreadable)
    }
    fn name(&self) -> &str {
        if self.error.as_deref() == Some(crate::content_workspace::DELETED_MANIFEST)
            && let Owner::Package(module) = &self.owner
        {
            return &module.name;
        }
        self.entry.as_ref().map_or(&self.path, |e| &e.name)
    }
    fn section(&self) -> Section {
        self.entry
            .as_ref()
            .map_or(Section::Unreadable, Section::for_entry)
    }
    /// The key of this note's text fields (its Markdown, the read-only copy of the DM's draft),
    /// with its owner's id. A file has no identity but its path, so the path is the key: a counted
    /// escape (`Ban::RowKey`), the one place that builds it.
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: a text key from data (design::results::STAGED_ALLOWS)"
    )]
    fn field_key(&self) -> design::TextKey {
        design::TextKey::from_data(self.path.clone())
    }
    /// This note's Markdown editor.
    fn markdown_field(&self) -> design::FieldId {
        LIBRARY_MARKDOWN.of(self.owner.id()).with(self.field_key())
    }
    /// How the row matches, as `matching::with_other` ranks it, or None if it does not. Plain
    /// words match the name (in any order, or with a typo), path or section, or a creature's
    /// facts. `habitat:forest` and the other fact terms match creatures only.
    fn search_rank(
        &self,
        section: Section,
        query: &Query,
        typed: &Typed,
        facets: Option<&Facets>,
    ) -> Option<Rank> {
        let words = query.words.as_str();
        if words.is_empty() && !query.names_facts() {
            return (section == Section::All || self.section() == section)
                .then_some(Rank::typed(0));
        }
        if query.names_facts() && !facets.is_some_and(|facets| query.matches_facts(facets)) {
            return None;
        }
        if words.is_empty() {
            return Some(Rank::typed(0));
        }
        let other = self.path.to_lowercase().contains(words)
            || self.section().label().to_lowercase().contains(words)
            || facets.is_some_and(|facets| query.words_in(self.name(), facets));
        with_other(typed.name(&self.name().to_lowercase()), other)
    }
}
#[derive(Default)]
struct DuplicateIndex {
    ids: std::collections::HashMap<Uuid, Vec<usize>>,
    keys: std::collections::HashMap<String, Vec<usize>>,
}

#[cfg(test)]
thread_local! {
    /// How many times the notes were read to count a rename's links.
    static RENAME_LINK_SCANS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// The link index a scan used, and each listed entry's position and broken link count.
type BrokenLinks = (
    std::sync::Arc<crate::lookup::ContentIndex>,
    Vec<(usize, usize)>,
);

/// A file's identity in the Library: its owner and its path in the owner's folder. Two packages
/// can each have `Rules/Watch.md`; they are two files. A load error has its package or campaign
/// id and an empty path.
type FileKey = (Option<Uuid>, String);

/// A problem the Library header counts, and the file it belongs to.
pub(crate) struct Diagnostic {
    file: FileKey,
    /// The menu line: the file (with its owner when another owner has the same path) and the problem.
    text: String,
}

/// Marks a checked entry whose path another owner shares, so its problems go back to its own note.
const SHARED_PATH_TAG: &str = "\u{0}note:";

#[derive(Default)]
pub(crate) struct Loaded {
    pub(crate) notes: Vec<Note>,
    owners: Vec<Owner>,
    /// Problems that kept content from loading. The notes cannot show these again.
    /// Each keyed by its package or campaign id, so two packages with one name count twice.
    load_errors: Vec<(FileKey, String)>,
    diagnostics: Vec<Diagnostic>,
    /// Files the header counts that have no message of their own: each file of a duplicate
    /// identity or key, whose row says `Same entry in two files`. The check names one side only.
    duplicate_files: Vec<FileKey>,
    /// Files whose every counted problem is a link to an entry that cannot be found. Their row
    /// says `N broken links` once, with no second `Needs fixing` mark.
    link_faults_only: std::collections::HashSet<FileKey>,
    /// A counted file cannot be read, so the header count has the error tone.
    unreadable_counted: bool,
    /// Paths that more than one note has, such as `Rules/Watch.md` in two packages, with the
    /// owner name of each note there. Rows that name one of these files name its owner too.
    /// `invalidate` clears it.
    shared_paths: std::sync::OnceLock<std::collections::HashMap<String, Vec<String>>>,
    /// What the active campaign can use: its selections and what they require.
    uses: Option<std::collections::HashSet<Uuid>>,
    /// The entries the DM chose for the active campaign, without the entries those choices
    /// need. Counts that say `chosen` count only these (round 26 FC2, FD4).
    chosen: Option<std::collections::HashSet<Uuid>>,
    subtitles: std::sync::OnceLock<Vec<String>>,
    counts: std::sync::OnceLock<Vec<(Section, usize)>>,
    duplicate_ids: std::sync::OnceLock<std::collections::HashSet<Uuid>>,
    duplicates: std::sync::OnceLock<DuplicateIndex>,
    /// Each note's creature facts, for search and the creature filter.
    facets: std::sync::OnceLock<Vec<Option<Facets>>>,
    /// Listed entries with a broken link, found against this link index. A new index, an edited
    /// note (`invalidate`) or a reload (a new `Loaded`) finds them again.
    broken_links: std::sync::Mutex<Option<BrokenLinks>>,
    /// How many links a rename's offer would change, for its old and new names. A reload (a new
    /// `Loaded`) or an edited note (`invalidate`) counts them again.
    rename_links: std::sync::Mutex<Option<((String, String), usize)>>,
}
impl Loaded {
    fn facets(&self) -> &[Option<Facets>] {
        self.facets.get_or_init(|| {
            self.notes
                .iter()
                .map(|note| note.entry.as_ref().and_then(Facets::of))
                .collect()
        })
    }
    /// The listed entries whose rows say `broken link`, as (note position, broken link count):
    /// campaign and package entries, not manifests. Scanning every body is the work the cache
    /// keeps out of each frame, the header menu included.
    fn broken_link_notes(
        &self,
        index: &std::sync::Arc<crate::lookup::ContentIndex>,
    ) -> Vec<(usize, usize)> {
        let mut cache = self
            .broken_links
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some((known, notes)) = cache.as_ref()
            && std::sync::Arc::ptr_eq(known, index)
        {
            return notes.clone();
        }
        let notes: Vec<(usize, usize)> = self
            .notes
            .iter()
            .enumerate()
            .filter(|(_, note)| note.owner != Owner::Builtin && !note.is_manifest())
            .filter_map(|(at, note)| {
                let entry = note.entry.as_ref()?;
                let count = index.broken_link_count(&entry.body, entry.id);
                (count > 0).then_some((at, count))
            })
            .collect();
        *cache = Some((index.clone(), notes.clone()));
        notes
    }

    /// The offer checked against the disk just before its write: the entry's own file still
    /// holds it with the new name (an outside rename since the last load ends the offer).
    /// The linking files are checked as they are written: each must still hold what was read.
    fn offer_holds_on_disk(&self, offer: &LinkOffer) -> bool {
        let source = &offer.source;
        let Ok(path) = note_path(&source.root, &source.path) else {
            return false;
        };
        // The exact bytes the rename saved: an outside edit since then, even to the body, ends
        // the offer.
        fs::read_to_string(path).is_ok_and(|text| text == source.text)
            && Entry::parse(&source.path, &source.text)
                .is_ok_and(|entry| entry.id == offer.owner && entry.name == offer.new)
            && !self.name_taken_on_disk(offer)
    }

    /// Whether another file on disk, in any editable root of the notes, has the offer's identity,
    /// or its old or new name or file stem. Only the offer's own source file (its root and path)
    /// is left out: a new file with the same identity is a conflict too. The loaded notes may be
    /// older than the disk, so the write reads the folders again. Built-in entries do not
    /// change, so the loaded check covers them.
    fn name_taken_on_disk(&self, offer: &LinkOffer) -> bool {
        let mut roots: Vec<&Path> = self.notes.iter().filter_map(|n| n.owner.root()).collect();
        roots.sort_unstable();
        roots.dedup();
        let taken = |name: &str| {
            [offer.old.as_str(), offer.new.as_str()]
                .iter()
                .any(|wanted| name.eq_ignore_ascii_case(wanted))
        };
        roots.into_iter().any(|root| {
            let Ok((entries, _)) = content::read_directory(root) else {
                // A folder that cannot be read cannot be checked: the write waits.
                return true;
            };
            let source = root == offer.source.root.as_path();
            entries.iter().any(|entry| {
                !(source && entry.path == offer.source.path)
                    && (entry.id == offer.owner
                        || taken(&entry.name)
                        || Path::new(&entry.path)
                            .file_stem()
                            .and_then(|stem| stem.to_str())
                            .is_some_and(taken))
            })
        })
    }

    /// True while a rename's offer still holds: its entry still has the new name, and the old and
    /// new names name no other entry or file.
    fn offer_holds(&self, offer: &LinkOffer) -> bool {
        self.notes
            .iter()
            .position(|note| {
                note.entry.as_ref().is_some_and(|e| e.id == offer.owner)
                    && note.owner.root() == Some(offer.source.root.as_path())
            })
            .is_some_and(|at| {
                let note = &self.notes[at];
                note.path == offer.source.path
                    && note.text == offer.source.text
                    && note
                        .entry
                        .as_ref()
                        .is_some_and(|entry| entry.name == offer.new)
                    && self.can_rename_links(at, &offer.old, &offer.new)
            })
    }

    /// How many links in the notes name `old`, for the offer to change them to `new`. Counted
    /// once per pair of names until `invalidate` or a reload.
    fn rename_link_count(&self, old: &str, new: &str) -> usize {
        let mut cache = self
            .rename_links
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(((known_old, known_new), count)) = cache.as_ref()
            && known_old == old
            && known_new == new
        {
            return *count;
        }
        #[cfg(test)]
        RENAME_LINK_SCANS.with(|scans| scans.set(scans.get() + 1));
        let count = self
            .notes
            .iter()
            .filter(|note| note.owner.root().is_some())
            .map(|note| crate::world::rename_note_links(&note.text, old, new).1)
            .sum();
        *cache = Some(((old.to_owned(), new.to_owned()), count));
        count
    }

    fn can_rename_links(&self, selected: usize, old: &str, new: &str) -> bool {
        !self.notes.iter().enumerate().any(|(at, note)| {
            at != selected
                && note.entry.is_some()
                && [old, new].iter().any(|name| {
                    note.name().eq_ignore_ascii_case(name)
                        || Path::new(&note.path)
                            .file_stem()
                            .and_then(|stem| stem.to_str())
                            .is_some_and(|stem| stem.eq_ignore_ascii_case(name))
                })
        })
    }
    fn duplicate_folder_paths(&self, owner: &Owner) -> Option<Vec<String>> {
        let root = owner.root()?;
        let mut other_root = None;
        let mut paths = Vec::new();
        for (at, note) in self.notes.iter().enumerate() {
            if note
                .entry
                .as_ref()
                .is_none_or(|entry| entry.kind == "package")
            {
                continue;
            }
            let duplicates = self.duplicates(at);
            if duplicates.is_empty() {
                continue;
            }
            if duplicates.len() != 1 {
                return None;
            }
            let pair = &self.notes[duplicates[0]];
            let (here, there) = (note.owner.root()?, pair.owner.root()?);
            let counterpart = if here == root {
                there
            } else if there == root {
                here
            } else {
                return None;
            };
            if counterpart == root || other_root.is_some_and(|known| known != counterpart) {
                return None;
            }
            other_root = Some(counterpart);
            if here == root {
                paths.push(note.path.clone());
            }
        }
        (paths.len() > 1).then_some(paths)
    }
    fn duplicate_ids(&self) -> &std::collections::HashSet<Uuid> {
        self.duplicate_ids.get_or_init(|| {
            self.notes
                .iter()
                .enumerate()
                .filter(|(at, _)| !self.duplicates(*at).is_empty())
                .filter_map(|(_, note)| note.entry.as_ref().map(|entry| entry.id))
                .collect()
        })
    }
    fn dependents(&self, note: &Note) -> Vec<String> {
        let Some(id) = crate::content_workspace::note_id(&note.text) else {
            return Vec::new();
        };
        self.notes
            .iter()
            .filter_map(|other| {
                let entry = other.entry.as_ref()?;
                (entry.id != id
                    && content::required_references(entry).is_ok_and(|refs| {
                        refs.contains(&content::Reference::Id(id))
                            || note
                                .entry
                                .as_ref()
                                .and_then(|entry| entry.key.as_ref())
                                .is_some_and(|key| {
                                    refs.contains(&content::Reference::Key(key.clone()))
                                })
                    }))
                .then(|| entry.name.clone())
            })
            .collect()
    }
    fn duplicate_index(&self) -> &DuplicateIndex {
        self.duplicates.get_or_init(|| {
            let mut index = DuplicateIndex::default();
            for (at, note) in self.notes.iter().enumerate() {
                if let Some(entry) = &note.entry {
                    index.ids.entry(entry.id).or_default().push(at);
                    if let Some(key) = &entry.key {
                        index.keys.entry(key.clone()).or_default().push(at);
                    }
                }
            }
            index
        })
    }
    /// Why Save would refuse this parse of note `index`'s draft, without writing anything. Save
    /// asks this before it writes, and a repair draft shows the same reason as its notice. It
    /// reads the identity index, which `invalidate` clears.
    fn draft_refusal(&self, index: usize, draft: &Result<Entry, String>) -> Option<String> {
        let entry = match draft {
            Ok(entry) => entry,
            Err(error) => return Some(error.clone()),
        };
        let note = &self.notes[index];
        if let Some(old) = &note.entry {
            if entry.id != old.id || entry.kind != old.kind {
                return Some("Keep this entry’s UUID and kind unchanged.".into());
            }
            if matches!(note.owner, Owner::Campaign(_)) && entry.key != old.key {
                return Some("Keep the entry’s UUID, kind and key unchanged.".into());
            }
            if old.kind == "package"
                && entry.fields.get("splat_namespace") != old.fields.get("splat_namespace")
            {
                return Some("Keep the package namespace unchanged.".into());
            }
        }
        let lookup = self.duplicate_index();
        let other = |at: &Vec<usize>| at.iter().any(|&at| at != index);
        let id_taken = note.entry.as_ref().is_none_or(|old| old.id != entry.id)
            && lookup.ids.get(&entry.id).is_some_and(other);
        let key_taken = entry.key.as_ref().is_some_and(|key| {
            note.entry
                .as_ref()
                .is_none_or(|old| old.key.as_ref() != Some(key))
                && lookup.keys.get(key).is_some_and(other)
        });
        (id_taken || key_taken).then(|| "Another entry already uses this UUID or key.".into())
    }
    fn duplicates(&self, index: usize) -> Vec<usize> {
        let Some(target) = self.notes[index].entry.as_ref() else {
            return Vec::new();
        };
        let lookup = self.duplicate_index();
        let mut matches: Vec<_> = lookup
            .ids
            .get(&target.id)
            .into_iter()
            .flatten()
            .chain(
                target
                    .key
                    .as_ref()
                    .and_then(|key| lookup.keys.get(key))
                    .into_iter()
                    .flatten(),
            )
            .copied()
            .filter(|&at| at != index)
            .collect();
        // The original scan returns each matching note once, in Library order.
        matches.sort_unstable();
        matches.dedup();
        matches
    }
    /// Checks all the loaded content as one, so that a reference can resolve to built-in
    /// content or to another package. The check of one package alone cannot see those.
    fn refresh_diagnostics(&mut self) {
        let shared = self.shared_paths();
        // The check names a problem by entry path only. An entry whose path another owner shares
        // is checked under a tag that names its note, so its problems stay with its own file.
        let tagged: std::collections::HashMap<usize, Entry> = self
            .notes
            .iter()
            .enumerate()
            .filter(|(_, note)| shared.contains_key(&note.path))
            .filter_map(|(at, note)| {
                let mut entry = note.entry.clone()?;
                entry.path = format!("{SHARED_PATH_TAG}{at}");
                Some((at, entry))
            })
            .collect();
        let entries: Vec<_> = self
            .notes
            .iter()
            .enumerate()
            .filter_map(|(at, n)| tagged.get(&at).or(n.entry.as_ref()))
            // The SRD pages have no notes in the Library, but entries can refer to them.
            .chain(content::builtin::entries())
            .collect();
        let report = content::validate_all(&entries);
        let checks = report.errors.into_iter().chain(report.warnings);
        let mut diagnostics: Vec<Diagnostic> = self
            .load_errors
            .iter()
            .map(|(file, text)| Diagnostic {
                file: file.clone(),
                text: text.clone(),
            })
            .collect();
        for check in checks {
            let note = match check.path.strip_prefix(SHARED_PATH_TAG) {
                Some(at) => at.parse::<usize>().ok().and_then(|at| self.notes.get(at)),
                None => self.notes.iter().find(|note| note.path == check.path),
            };
            diagnostics.push(match note {
                Some(note) => self.note_problem(note, &check.message),
                None => Diagnostic {
                    file: (None, check.path.clone()),
                    text: format!("{}: {}", check.path, check.message),
                },
            });
        }
        for note in &self.notes {
            if let Some(error) = &note.error {
                diagnostics.push(self.note_problem(note, error));
            }
        }
        for note in self
            .notes
            .iter()
            .filter(|note| note.owner != Owner::Builtin)
        {
            if let Some(entry) = &note.entry {
                for flag in &entry.flags {
                    diagnostics.push(self.note_problem(note, &flag.message));
                }
                let faults = crate::roll_table::fault_count(&entry.body);
                if faults > 0 {
                    diagnostics.push(
                        self.note_problem(
                            note,
                            &design::count(faults, "table fault", "table faults"),
                        ),
                    );
                }
            }
            if let Some(entry) = &note.entry
                && let Some((old, _)) = presentation::heading_fix(&note.text, entry)
            {
                diagnostics.push(self.note_problem(
                    note,
                    &format!(
                        "The first heading says {old}, but the name is {}.",
                        entry.name
                    ),
                ));
            }
        }
        // Every file of a duplicate identity or key counts, as both rows are marked.
        self.duplicate_files = (0..self.notes.len())
            .filter(|&at| !self.duplicates(at).is_empty())
            .map(|at| Self::file_key(&self.notes[at]))
            .collect();
        // A file whose problems are all links to missing entries: each problem is a reference
        // that resolves to nothing, the way the check resolves them, and each such reference is
        // a link in the file's own text, which its row counts as a broken link.
        let ids: std::collections::HashSet<Uuid> = entries.iter().map(|entry| entry.id).collect();
        let keys: std::collections::HashSet<&str> = entries
            .iter()
            .filter_map(|entry| entry.key.as_deref())
            .collect();
        self.link_faults_only = self
            .notes
            .iter()
            .filter(|note| note.owner != Owner::Builtin)
            .filter_map(|note| {
                let entry = note.entry.as_ref()?;
                let unresolved: Vec<String> = content::required_references(entry)
                    .ok()?
                    .into_iter()
                    .filter_map(|reference| match reference {
                        content::Reference::Id(id) => (!ids.contains(&id)).then(|| id.to_string()),
                        content::Reference::Key(key) => {
                            (!keys.contains(key.as_str())).then(|| key.to_lowercase())
                        }
                    })
                    .collect();
                if unresolved.is_empty() {
                    return None;
                }
                let links = crate::world::links_in(&entry.body);
                let file = Self::file_key(note);
                let problems = diagnostics.iter().filter(|d| d.file == file).count();
                (problems == unresolved.len()
                    && unresolved.iter().all(|target| {
                        links
                            .iter()
                            .any(|link| link.split('|').next().unwrap_or("").trim() == target)
                    }))
                .then_some(file)
            })
            .collect();
        self.unreadable_counted = self.notes.iter().any(|note| {
            note.entry.is_none()
                && note.error.is_some()
                && note.error.as_deref() != Some(crate::content_workspace::DELETED_MANIFEST)
        });
        self.diagnostics = diagnostics;
        // A row marks each file the header counts, so the rows follow the new problems.
        self.subtitles.take();
    }

    /// Paths that more than one note has, each with the owner names there. Built once per change
    /// of the notes.
    fn shared_paths(&self) -> &std::collections::HashMap<String, Vec<String>> {
        self.shared_paths.get_or_init(|| {
            let mut owners: std::collections::HashMap<String, Vec<String>> =
                std::collections::HashMap::new();
            for note in &self.notes {
                owners
                    .entry(note.path.clone())
                    .or_default()
                    .push(note.owner.source().to_owned());
            }
            owners.retain(|_, names| names.len() > 1);
            owners
        })
    }

    /// The note's file as a menu row names it: its path, after its owner when another owner has
    /// the same path (`Ashen Marches · Rules/Watch.md`). When that other owner has the same name
    /// too, the row names the folder (`Ashen Marches · ashen-copy/Rules/Watch.md`).
    fn file_label(&self, note: &Note) -> String {
        let source = note.owner.source();
        match self.shared_paths().get(&note.path) {
            None => note.path.clone(),
            Some(names) if names.iter().filter(|name| *name == source).count() > 1 => {
                format!("{source} · {}/{}", note.owner.folder(), note.path)
            }
            Some(_) => format!("{source} · {}", note.path),
        }
    }

    fn file_key(note: &Note) -> FileKey {
        (note.owner.id(), note.path.clone())
    }

    fn note_problem(&self, note: &Note, message: &str) -> Diagnostic {
        Diagnostic {
            file: Self::file_key(note),
            text: format!("{}: {message}", self.file_label(note)),
        }
    }

    #[cfg(test)]
    fn diagnostic_texts(&self) -> Vec<&str> {
        self.diagnostics.iter().map(|d| d.text.as_str()).collect()
    }

    /// True when the header counts a problem in this note's file.
    fn has_problem(&self, note: &Note) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.file.0 == note.owner.id() && d.file.1 == note.path)
    }

    /// How many files the header counts: each file once, by owner and path, for a problem, a
    /// broken link, a duplicate identity or more than one.
    /// `extra` is a file the loaded notes do not know is faulty yet: an open draft's file that
    /// stopped reading.
    fn files_to_fix(&self, broken: &[(usize, usize)], extra: Option<&FileKey>) -> usize {
        self.diagnostics
            .iter()
            .map(|d| (d.file.0, d.file.1.as_str()))
            .chain(broken.iter().map(|&(at, _)| {
                let note = &self.notes[at];
                (note.owner.id(), note.path.as_str())
            }))
            .chain(
                self.duplicate_files
                    .iter()
                    .map(|(owner, path)| (*owner, path.as_str())),
            )
            .chain(extra.map(|(owner, path)| (*owner, path.as_str())))
            .collect::<std::collections::HashSet<_>>()
            .len()
    }

    /// Whether the DM chose this entry for the active campaign, not only needed it. With no
    /// campaign open, nothing is chosen.
    fn chose(&self, note: &Note) -> bool {
        self.chosen
            .as_ref()
            .zip(note.entry.as_ref())
            .is_some_and(|(chosen, entry)| chosen.contains(&entry.id))
    }

    fn in_scope(&self, note: &Note, scope: Scope) -> bool {
        if note.is_manifest() {
            return false;
        }
        match scope {
            Scope::All => true,
            Scope::Uses => match &self.uses {
                None => true,
                Some(uses) => {
                    matches!(note.owner, Owner::Campaign(_))
                        || note
                            .entry
                            .as_ref()
                            .is_some_and(|entry| self.duplicate_ids().contains(&entry.id))
                        || note.entry.as_ref().is_some_and(|e| uses.contains(&e.id))
                }
            },
            Scope::Builtin => note.owner == Owner::Builtin,
            Scope::Package(id) => {
                matches!(note.owner, Owner::Package(_)) && note.owner.id() == Some(id)
            }
            Scope::Campaign => matches!(note.owner, Owner::Campaign(_)),
        }
    }

    fn resolve_reference(&self, reference: &content::Reference) -> Option<usize> {
        let mut matches = self.notes.iter().enumerate().filter_map(|(i, note)| {
            let entry = note.entry.as_ref()?;
            let matches = match reference {
                content::Reference::Id(id) => entry.id == *id,
                content::Reference::Key(key) => entry.key.as_ref() == Some(key),
            };
            matches.then_some(i)
        });
        let first = matches.next()?;
        matches.next().is_none().then_some(first)
    }

    fn resolve_spell(&self, reference: &content::Reference) -> Option<usize> {
        self.resolve_reference(reference).filter(|&index| {
            self.notes[index]
                .entry
                .as_ref()
                .is_some_and(|entry| entry.kind == "spell")
        })
    }

    fn subtitles(&self) -> &Vec<String> {
        self.subtitles.get_or_init(|| self.build_subtitles())
    }
    #[cfg(test)]
    fn count(&self, section: Section) -> usize {
        self.counts
            .get_or_init(|| {
                Section::GROUPS
                    .iter()
                    .flat_map(|(_, sections)| sections.iter())
                    .map(|&section| {
                        (
                            section,
                            self.notes
                                .iter()
                                .filter(|n| section == Section::All || n.section() == section)
                                .count(),
                        )
                    })
                    .collect()
            })
            .iter()
            .find(|(s, _)| *s == section)
            .map_or(0, |(_, n)| *n)
    }
    fn invalidate(&mut self) {
        *self
            .broken_links
            .get_mut()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = None;
        *self
            .rename_links
            .get_mut()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = None;
        self.subtitles.take();
        self.counts.take();
        self.duplicate_ids.take();
        self.duplicates.take();
        self.facets.take();
        self.shared_paths.take();
    }
    fn build_subtitles(&self) -> Vec<String> {
        let mut subtitles: Vec<_> = self
            .notes
            .iter()
            .map(|note| {
                let source = match &note.owner {
                    Owner::Builtin => None,
                    Owner::Package(module) => Some(module.name.as_str()),
                    Owner::Campaign(_) => Some("Campaign"),
                };
                let line = note.entry.as_ref().map_or_else(
                    || {
                        // A file the system will not read names its short cause, as World's
                        // row does. Its owner line in the reader keeps `Cannot be read`. A file
                        // that cannot be read is an error, so its mark has the error tone, as in
                        // the reader and the header count.
                        if let Some(fault) = &note.fault {
                            return format!("{} {}", icons::ERROR, fault.cause());
                        }
                        let (_, line) =
                            crate::campaign_content::repair_problem(&note.path, &note.text);
                        format!(
                            "{} {}",
                            icons::ERROR,
                            line.map_or_else(
                                || "Cannot be read".to_owned(),
                                |line| format!("Line {line}"),
                            )
                        )
                    },
                    presentation::subtitle,
                );
                // Round 25 AC2: each known repair cause is named on its own, after one mark,
                // so a duplicate that also has a heading fault says both.
                let mut causes = Vec::new();
                if note
                    .entry
                    .as_ref()
                    .is_some_and(|entry| self.duplicate_ids().contains(&entry.id))
                {
                    causes.push(SAME_ENTRY);
                }
                if note
                    .entry
                    .as_ref()
                    .is_some_and(|entry| presentation::heading_fix(&note.text, entry).is_some())
                {
                    causes.push(HEADING_DIFFERS);
                }
                let line = if !causes.is_empty() {
                    format!("{} {} · {line}", icons::WARN, causes.join(" · "))
                } else if note.entry.as_ref().is_some_and(|entry| {
                    // A table fault marks its row where the row is drawn.
                    crate::roll_table::fault_count(&entry.body) == 0
                }) && !note.is_manifest()
                    && !line.contains(icons::WARN)
                    && self.has_problem(note)
                {
                    // Every other fault the header counts in this file marks its row too.
                    format!("{} {NEEDS_FIXING} · {line}", icons::WARN)
                } else {
                    line
                };
                let mut subtitle = presentation::list_subtitle(&line, source);
                if let Owner::Package(module) = &note.owner
                    && self.owners.iter().any(|owner| matches!(owner, Owner::Package(other) if other.id != module.id && other.name == module.name))
                { subtitle.push_str(&format!(" · {}/", note.owner.folder())); }
                subtitle
            })
            .collect();
        // Imported sections share a broad parent. Use the closest named subsection
        // before a repeated heading, from the source spans, rather than a page number.
        let parent_of = |note: &Note| {
            note.entry
                .as_ref()
                .and_then(|entry| entry.fields.get("splat_conversion"))
                .and_then(|meta| meta.get("parent_name"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_owned()
        };
        let mut name_counts = std::collections::HashMap::new();
        for note in &self.notes {
            *name_counts
                .entry((parent_of(note), note.name()))
                .or_insert(0_usize) += 1;
        }
        for (i, note) in self.notes.iter().enumerate() {
            let duplicates = name_counts[&(parent_of(note), note.name())];
            if duplicates < 2 {
                continue;
            }
            let Some(entry) = &note.entry else {
                continue;
            };
            let Some(meta) = entry.fields.get("splat_conversion") else {
                continue;
            };
            let parent = meta.get("parent_name").and_then(|v| v.as_str());
            let first = meta
                .get("line_span")
                .and_then(|span| span.get("first"))
                .and_then(|v| v.as_u64());
            let Some(first) = first else {
                continue;
            };
            let context = self
                .notes
                .iter()
                .filter(|other| other.owner == note.owner)
                .filter_map(|other| {
                    let meta = other.entry.as_ref()?.fields.get("splat_conversion")?;
                    if meta.get("parent_name").and_then(|v| v.as_str()) != parent
                        || name_counts[&(parent_of(other), other.name())] > 1
                    {
                        return None;
                    }
                    let start = meta.get("line_span")?.get("first")?.as_u64()?;
                    (if parent.is_some_and(|parent| parent.eq_ignore_ascii_case(note.name())) {
                        start > first
                    } else {
                        start < first
                    })
                    .then_some((start, other.name()))
                })
                .min_by_key(|(start, _)| {
                    if parent.is_some_and(|parent| parent.eq_ignore_ascii_case(note.name())) {
                        *start
                    } else {
                        u64::MAX - *start
                    }
                });
            if let Some((_, context)) = context
                && let Some(parent) = parent
            {
                if parent.eq_ignore_ascii_case(note.name()) {
                    subtitles[i].push_str(&format!(" · {context}"));
                } else {
                    subtitles[i] = subtitles[i]
                        .replace(&format!(" · {parent}"), &format!(" · {parent} › {context}"));
                }
            }
        }
        subtitles
    }
}

impl view::Links for Loaded {
    fn find(&self, reference: &content::Reference) -> Option<(usize, String)> {
        let index = self.resolve_reference(reference)?;
        Some((index, self.notes[index].name().to_owned()))
    }
    fn find_spell(&self, reference: &content::Reference) -> Option<(usize, String)> {
        let index = self.resolve_spell(reference)?;
        Some((index, self.notes[index].name().to_owned()))
    }
    fn spells_named_in(&self, text: &str) -> Vec<(usize, String)> {
        self.notes
            .iter()
            .enumerate()
            .filter(|(_, note)| {
                note.entry.as_ref().is_some_and(|e| e.kind == "spell")
                    && crate::lookup::names_word(text, note.name())
            })
            .map(|(index, note)| (index, note.name().to_owned()))
            .collect()
    }
}

/// The result after a copied package is registered. It says what the registration did, not what
/// the campaign uses now: a later source change can make encounters use the copy.
fn package_registered(name: &str, chosen: bool) -> String {
    if chosen {
        format!("{name} is registered, with every entry chosen for this campaign.")
    } else {
        format!("{name} is registered, with no entries chosen for this campaign.")
    }
}

/// A Save that failed on the disk (a read-only file or folder, a write error): an action-time
/// failure. It belongs to the draft Save tried to write, by owner, path and draft revision. The draft's
/// state slot says `Not saved` with Save's reason until the draft changes, another entry opens,
/// or a Save succeeds. Nothing reads the disk to show it.
struct SaveFailure {
    owner: Option<Uuid>,
    path: String,
    revision: u64,
    reason: String,
    /// FL1: Save found the file or its folder read only, so the owner line says `Read only`
    /// for as long as this failure lasts.
    read_only: Option<ReadOnly>,
}

impl SaveFailure {
    /// True while the open draft is still the one this Save tried to write: the same entry and no
    /// change event since. The check reaches the draft's words only through `draft`, which counts
    /// each read in tests; it needs none of them, so no frame compares or hashes the draft.
    fn owns(&self, note: &Note, revision: u64, draft: DraftView<'_>) -> bool {
        let _ = draft;
        self.owner == note.owner.id() && self.path == note.path && self.revision == revision
    }
}

use draft_view::DraftView;

/// The open draft's words, as the failed-Save check may see them. The field is private to this
/// module, so the only way to the words is `Deref`, and in tests every read adds one to the
/// check's read count.
mod draft_view {
    pub(super) struct DraftView<'a> {
        text: &'a str,
        #[cfg(test)]
        checks: &'a std::cell::Cell<(usize, usize)>,
    }

    impl<'a> DraftView<'a> {
        pub(super) fn new(
            text: &'a str,
            #[cfg(test)] checks: &'a std::cell::Cell<(usize, usize)>,
        ) -> Self {
            Self {
                text,
                #[cfg(test)]
                checks,
            }
        }
    }

    impl std::ops::Deref for DraftView<'_> {
        type Target = str;
        fn deref(&self) -> &str {
            #[cfg(test)]
            {
                let (checks, reads) = self.checks.get();
                self.checks.set((checks, reads + 1));
            }
            self.text
        }
    }
}

/// A draft that `Remove entry and lose draft` dropped after its file left the disk, for one step of undo, and
/// whether this campaign used its entry then.
struct LetGo {
    note: Note,
    used: bool,
    /// Whether the DM chose the entry itself, not only needed it.
    chosen: bool,
    /// The campaign that was open when the entry was removed, once the page has seen it. Its
    /// `used` flag and the catalogue it goes back into belong to that campaign.
    campaign: Option<Option<Uuid>>,
}

/// `Show folder`: opens a folder in the system's file manager.
fn show_folder(ui: &mut egui::Ui, folder: &Path) {
    if design::link(ui, "Show folder").clicked() {
        open_folder(ui, folder);
    }
}

fn open_folder(ui: &egui::Ui, folder: &Path) {
    {
        ui.ctx().open_url(egui::OpenUrl::new_tab(format!(
            "file://{}",
            folder
                .to_string_lossy()
                .bytes()
                .map(|byte| {
                    if byte.is_ascii_alphanumeric() || b"-._~/".contains(&byte) {
                        (byte as char).to_string()
                    } else {
                        format!("%{byte:02X}")
                    }
                })
                .collect::<String>()
        )));
    }
}

/// The file line of a draft whose file has gone from disk: the path it had, then `Show folder`
/// for the nearest folder that is still there. No link offers the missing file.
fn gone_file_parts(ui: &mut egui::Ui, root: &Path, relative: &str) {
    let path = root.join(relative);
    let room = (ui.available_width()
        - design::number_width(ui, "Show folder", &Type::Body.font(ui))
        - design::L)
        .max(0.0);
    let font = Type::Small.font(ui);
    let filename = relative.rsplit('/').next().unwrap_or(relative);
    let shown = if design::number_width(ui, relative, &font) <= room || filename == relative {
        relative.to_owned()
    } else {
        format!("…/{filename}")
    };
    let response = ui.label(Type::Small.rich(ui, shown).color(colors().muted));
    design::named(ui, &response, &path.display().to_string());
    response.on_hover_text(path.display().to_string());
    // The folder is looked for on the press, not every frame.
    if design::link(ui, "Show folder").clicked() {
        let folder = path
            .ancestors()
            .skip(1)
            .find(|folder| folder.is_dir())
            .unwrap_or(root);
        open_folder(ui, folder);
    }
}

/// The offer to change links after a rename. It belongs to the renamed entry (by its id, in
/// its root) and shows only in that entry's reader. It carries the version of the entry's file
/// it was made from: the root, the path and the exact text the rename saved. The offer holds
/// only while that file still has those bytes (round 26 review).
/// Its footer order is the store's (`Library::renamed_links`), issued when the rename (or the
/// Undo that gave the offer back) made it.
#[derive(Clone)]
struct LinkOffer {
    owner: Uuid,
    source: OfferSource,
    old: String,
    new: String,
}

/// The entry a link offer or a link result belongs to: its identity in its root. The root is
/// its scope, so another campaign's or package's entry with the same identity never shows it.
#[derive(Clone, PartialEq, Eq, Debug)]
struct LinkOwner {
    id: Uuid,
    root: PathBuf,
}

impl design::results::Owner for LinkOwner {
    type Scope = Path;
    fn scope(&self) -> &Path {
        &self.root
    }
}

impl design::results::Kept for LinkOffer {
    type Owner = LinkOwner;
    /// A saved edit of its entry ends it (a rename that passes the check gives a new one); a
    /// reload keeps it while it still holds; its press uses it up, written or not.
    const ENDS: design::results::Ends = design::results::Ends {
        owner_changed: design::results::OwnerChanged::Ends,
        disk_changed: design::results::DiskChanged::Checked,
        press: design::results::Press::Use,
    };
    fn owner(&self) -> LinkOwner {
        LinkOwner {
            id: self.owner,
            root: self.source.root.clone(),
        }
    }
}

/// The renamed entry's file as the rename saved it.
#[derive(Clone, PartialEq)]
struct OfferSource {
    root: PathBuf,
    path: String,
    text: String,
}

/// A link update's result and its Undo. It belongs to the renamed entry and shows only in its
/// reader: while another entry is open it is hidden, not ended. It ends on its owner's next link
/// update or saved edit, its Undo, or a change on disk to a file it wrote. Its Undo gives back
/// the rename's offer, so the links that say the old name again can be changed again (round 26
/// FD6).
struct LinkResult {
    owner: Uuid,
    /// The offer's source version, which its Undo gives back to the offer.
    source: OfferSource,
    edits: crate::content_workspace::ContentEdits,
    old: String,
    new: String,
}

impl design::results::Kept for LinkResult {
    type Owner = LinkOwner;
    /// A saved edit of its entry ends it; a reload keeps it while its files hold what it wrote;
    /// it leaves by its Undo only when the Undo works.
    const ENDS: design::results::Ends = design::results::Ends {
        owner_changed: design::results::OwnerChanged::Ends,
        disk_changed: design::results::DiskChanged::Checked,
        press: design::results::Press::Undo,
    };
    fn owner(&self) -> LinkOwner {
        LinkOwner {
            id: self.owner,
            root: self.source.root.clone(),
        }
    }
}

/// The page's results that are not about the open entry: the offer to change links after a rename
/// and its result, and a copied package's registration. They sit in the pane's message footer, so
/// a result never moves the columns, the tabs, the file line, the title or the body.
#[derive(Default)]
struct PageResults {
    /// The offer's footer line (its words with its order from the store) and its action's words.
    link_offer: Option<(design::results::FooterResult, String)>,
    /// The link update's footer line, with its order from the store.
    links: Option<design::results::FooterResult>,
    /// The registration's result, and why its Undo waits.
    registration: Option<(String, Option<&'static str>)>,
    /// `Use the copy in encounters` shows, with why it waits.
    use_copy: Option<Option<&'static str>>,
    /// The source change's Undo shows, with why it waits.
    rosters: Option<Option<&'static str>>,
    /// A package unregister's result.
    unregistered: Option<String>,
    /// Why the scope changed to show an entry.
    scope_notice: Option<String>,
}

impl PageResults {
    fn any(&self) -> bool {
        self.link_offer.is_some()
            || self.links.is_some()
            || self.registration.is_some()
            || self.rosters.is_some()
            || self.unregistered.is_some()
            || self.scope_notice.is_some()
    }
}

/// Which page result's action was pressed.
#[derive(Clone, Copy, Default)]
struct PagePress {
    link_offer: bool,
    links: bool,
    registration: bool,
    use_copy: bool,
    rosters: bool,
    unregistered: bool,
}

/// The kept results of the open entry's pane, in the order they show. They sit in the pane's
/// message footer, below the reader's body or the editor, and below the empty reader or a form
/// (round 26 FD2). Without a save row in Edit Markdown (a file changed on disk) they stay above
/// the comparison.
#[derive(Default)]
struct ReaderResults {
    /// The page's results, first.
    page: PageResults,
    /// Every entry in this package is now chosen for this campaign.
    package_use: bool,
    /// A manifest write's result and its Undo's words.
    manifest: Option<(String, String)>,
    /// A folder's separate-entries result.
    identities: Option<String>,
    /// One file's separate-entry result.
    identity: Option<String>,
    /// The name of the entry whose draft Read just discarded.
    discarded: Option<String>,
    /// Why Undo of a file change waits: an open draft. Its reason shows in the save row.
    locked: Option<&'static str>,
}

/// Which result's Undo was pressed this frame.
#[derive(Clone, Copy, Default)]
struct ResultPress {
    page: PagePress,
    package_use: bool,
    manifest: bool,
    identities: bool,
    identity: bool,
    discarded: bool,
}

impl ReaderResults {
    fn any(&self) -> bool {
        self.page.any()
            || self.package_use
            || self.manifest.is_some()
            || self.identities.is_some()
            || self.identity.is_some()
            || self.discarded.is_some()
    }
}

/// One kept result of a pane, in the order `show_results` draws them in Read.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum ResultKind {
    ScopeNotice,
    LinkOffer,
    Links,
    Registration,
    Rosters,
    Unregistered,
    PackageUse,
    Manifest,
    Identities,
    Identity,
    Discarded,
}

impl ReaderResults {
    /// The results shown, in Read's order, each with its words and whether its Undo is live.
    fn items(&self) -> Vec<(ResultKind, String, bool)> {
        let page = &self.page;
        let mut items = Vec::new();
        let mut add = |kind, words: Option<String>, undo| {
            if let Some(words) = words {
                items.push((kind, words, undo));
            }
        };
        add(ResultKind::ScopeNotice, page.scope_notice.clone(), false);
        add(
            ResultKind::LinkOffer,
            page.link_offer
                .as_ref()
                .map(|(line, _)| line.words().to_owned()),
            false,
        );
        add(
            ResultKind::Links,
            page.links.as_ref().map(|line| line.words().to_owned()),
            true,
        );
        add(
            ResultKind::Registration,
            page.registration.as_ref().map(|(words, _)| words.clone()),
            true,
        );
        add(
            ResultKind::Rosters,
            page.rosters.map(|_| USE_COPY_RESULT.to_owned()),
            true,
        );
        add(ResultKind::Unregistered, page.unregistered.clone(), true);
        add(
            ResultKind::PackageUse,
            self.package_use.then(|| PACKAGE_USE_RESULT.to_owned()),
            true,
        );
        add(
            ResultKind::Manifest,
            self.manifest.as_ref().map(|(words, _)| words.clone()),
            true,
        );
        add(ResultKind::Identities, self.identities.clone(), true);
        add(ResultKind::Identity, self.identity.clone(), true);
        add(
            ResultKind::Discarded,
            self.discarded
                .as_ref()
                .map(|name| format!("{name} draft discarded.")),
            true,
        );
        items
    }
}

fn show_results(ui: &mut egui::Ui, results: &ReaderResults) -> ResultPress {
    let mut pressed = ResultPress::default();
    for (kind, ..) in results.items() {
        show_result(ui, results, kind, &mut pressed);
    }
    pressed
}

/// Draw one kept result, and note its press in `pressed`.
fn show_result(
    ui: &mut egui::Ui,
    results: &ReaderResults,
    kind: ResultKind,
    pressed: &mut ResultPress,
) {
    let page = &results.page;
    match kind {
        ResultKind::ScopeNotice => {
            if let Some(words) = &page.scope_notice {
                // It wrote nothing: the scope only widened to show the entry.
                design::message(ui, Tone::Info, words);
            }
        }
        ResultKind::LinkOffer => {
            if let Some((line, action)) = &page.link_offer {
                let words = line.words();
                // The offer names both names, so the DM can check the words before the write.
                pressed.page.link_offer =
                    design::message_with_action(ui, Tone::Info, words, action);
            }
        }
        ResultKind::Links => {
            if let Some(line) = &page.links {
                let words = line.words();
                // The result is a message, so its Undo keeps its icon on the message line.
                pressed.page.links =
                    design::undo_message(ui, Tone::Success, words, "Undo link update");
            }
        }
        ResultKind::Registration => {
            if let Some((words, blocked)) = &page.registration {
                // The Undo sits on the message line, as every Undo after a message does. After
                // an outside change it stays there, blocked, and the reason follows at the
                // message words. The result still happened, so its words keep the text colour.
                pressed.page.registration = kept_undo_message(
                    ui,
                    Tone::Success,
                    words,
                    "Undo package registration",
                    *blocked,
                    true,
                );
                // The other choice starts below, at the message words. Only Undo is quiet.
                if let Some(blocked) = page.use_copy {
                    pressed.page.use_copy = design::message_choice_row(ui, Tone::Success, |ui| {
                        Action::secondary(USE_COPY)
                            .density(Density::Read)
                            .blocked(blocked)
                            .show(ui)
                            .clicked()
                    });
                }
            }
        }
        ResultKind::Rosters => {
            if let Some(blocked) = page.rosters {
                // After an outside change the Undo stays, blocked, and the reason follows.
                pressed.page.rosters = kept_undo_message(
                    ui,
                    Tone::Success,
                    USE_COPY_RESULT,
                    "Undo source change",
                    blocked,
                    true,
                );
            }
        }
        ResultKind::Unregistered => {
            if let Some(words) = &page.unregistered {
                // A finished change: the package left this campaign's lists.
                pressed.page.unregistered =
                    design::undo_message(ui, Tone::Success, words, "Undo unregister");
            }
        }
        ResultKind::PackageUse => {
            if results.package_use {
                pressed.package_use = design::undo_message(
                    ui,
                    Tone::Success,
                    PACKAGE_USE_RESULT,
                    "Undo package choice",
                );
            }
        }
        ResultKind::Manifest => {
            if let Some((words, action)) = &results.manifest {
                pressed.manifest =
                    kept_undo_message(ui, Tone::Success, words, action, results.locked, false);
            }
        }
        ResultKind::Identities => {
            if let Some(words) = &results.identities {
                pressed.identities = kept_undo_message(
                    ui,
                    Tone::Success,
                    words,
                    "Undo separate entries",
                    results.locked,
                    false,
                );
            }
        }
        ResultKind::Identity => {
            if let Some(words) = &results.identity {
                pressed.identity = kept_undo_message(
                    ui,
                    Tone::Success,
                    words,
                    crate::world::SEPARATE_ENTRY_UNDO,
                    results.locked,
                    false,
                );
            }
        }
        ResultKind::Discarded => {
            if let Some(name) = &results.discarded {
                pressed.discarded = design::draft_discarded(ui, name);
            }
        }
    }
}

/// The result of `Choose all` on a package About page.
pub(crate) const PACKAGE_USE_RESULT: &str =
    "Every entry in this package is now chosen for this campaign.";

/// A kept result and its Undo, as `design::undo_message` draws them. When Undo is blocked, only
/// Undo greys: the result's words keep the text colour, because the change still happened. With
/// `say_why`, the reason follows in `Small` muted words at the message words. Without it, the
/// reason is already on screen (a draft's reason in its save row) and on Undo's hover.
#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
)]
pub(crate) fn kept_undo_message(
    ui: &mut egui::Ui,
    tone: Tone,
    text: &str,
    action: &str,
    blocked: Option<&str>,
    say_why: bool,
) -> bool {
    let Some(reason) = blocked else {
        return design::undo_message(ui, tone, text, action);
    };
    let button = Action::quiet(action)
        .icon(icons::UNDO)
        .density(Density::Read)
        .blocked(Some(reason));
    let button_width = button.natural_width(ui);
    // `design::message`'s icon for this tone.
    let icon = match tone {
        Tone::Info => icons::INFO,
        Tone::Success => icons::SUCCESS,
        Tone::Warning => icons::WARN,
        Tone::Error => icons::ERROR,
    };
    let icon_width = design::number_width(ui, icon, &Type::Body.font(ui));
    // The same geometry as `undo_message`, so blocking Undo moves nothing on its line.
    let inset = design::S + 1.0 + icon_width + design::XS + ui.spacing().item_spacing.x;
    let text_width = design::number_width(ui, text, &Type::Body.font(ui)) + inset;
    let gap = ui.spacing().item_spacing.x;
    if ui.available_width() < text_width + gap + button_width {
        design::message(ui, tone, text);
        ui.horizontal(|ui| {
            ui.add_space((inset - gap).max(0.0));
            button.line_start().show(ui);
        });
    } else {
        ui.horizontal_top(|ui| {
            let width = (ui.available_width() - button_width - gap).max(0.0);
            ui.allocate_ui_with_layout(
                egui::vec2(width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    ui.add_space((Density::Read.control() - Type::Body.line()) / 2.0);
                    design::message(ui, tone, text)
                },
            );
            button.show(ui);
        });
    }
    // The reason starts at the message words, on its own line, in both layouts.
    if say_why {
        design::message_choice_row(ui, tone, |ui| design::blocked_reason(ui, Some(reason)));
    }
    false
}

/// Why a link update wrote nothing: the renamed entry changed on disk since the offer was made.
pub(crate) const LINKS_NOT_CHANGED: &str = "No links were changed. This entry changed on disk since it was renamed. Reload from disk, then rename it again.";

/// The warning for a draft whose file moved on disk. Its two choices name what each one does.
pub(crate) const MOVED_ON_DISK: &str =
    "This entry moved on disk. Your draft is safe until you choose.";

const ENTRY_ROW_HEIGHT: f32 = 44.0;
const SEARCH_NAME: &str = "Search the Library";
const LIBRARY_SEARCH: design::FieldId =
    crate::field_id!("library_search").once(design::Once::PageSearch);
const NEW_ENTRY_FOLDER: design::FieldId =
    crate::field_id!("new_entry_folder").once(design::Once::CreateForm);
const NEW_ENTRY_KIND: design::FieldId =
    crate::field_id!("new_entry_kind").once(design::Once::CreateForm);
const NEW_ENTRY_PLACE: design::FieldId =
    crate::field_id!("new_entry_place").once(design::Once::CreateForm);
const DISK_DRAFT: design::FieldName = crate::field_id!("library_disk_draft");
const LIBRARY_MARKDOWN: design::FieldName = crate::field_id!("library_markdown");
/// A row's short cause for an entry whose identity or key another file claims too.
pub(crate) const SAME_ENTRY: &str = "Same entry in two files";
/// A row's short cause for a first heading that does not say the entry's name.
const HEADING_DIFFERS: &str = "First heading differs";
/// A row's short cause for any other problem the header counts in its file.
const NEEDS_FIXING: &str = "Needs fixing";

/// A list row's line without the fault it leads with (`⚠ First heading differs · `). The reader
/// states that fault in its own warning, so its kind line does not. The rest of the line stays,
/// with the folder that tells two packages of one name apart.
fn row_line_without_fault(line: &str) -> String {
    let Some(mut rest) = line.strip_prefix(&format!("{} ", icons::WARN)) else {
        return line.to_owned();
    };
    // One mark, then one or more causes, each followed by ` · `.
    let mut stripped = false;
    while let Some(after) = [SAME_ENTRY, HEADING_DIFFERS, NEEDS_FIXING]
        .into_iter()
        .find_map(|cause| rest.strip_prefix(&format!("{cause} · ")))
    {
        rest = after;
        stripped = true;
    }
    if stripped {
        rest.to_owned()
    } else {
        line.to_owned()
    }
}

/// One line of row words in which each warning or error mark has its own tone, as a status
/// is an icon, a word and a colour. The line ends with `…` where it does not fit.
pub(crate) fn marked_line(
    ui: &egui::Ui,
    text: &str,
    font: egui::FontId,
    words: egui::Color32,
    max_width: f32,
) -> std::sync::Arc<egui::Galley> {
    let c = colors();
    let mut job = egui::text::LayoutJob::default();
    let format = |color| egui::TextFormat {
        font_id: font.clone(),
        color,
        ..Default::default()
    };
    let text = text.replace(['\n', '\r'], " ");
    let mut rest = text.as_str();
    while !rest.is_empty() {
        let next = [
            (icons::WARN, c.result.warning),
            (icons::ERROR, c.result.error),
        ]
        .into_iter()
        .filter_map(|(mark, color)| rest.find(mark).map(|at| (at, mark, color)))
        .min_by_key(|(at, _, _)| *at);
        match next {
            Some((at, mark, color)) => {
                if at > 0 {
                    job.append(&rest[..at], 0.0, format(words));
                }
                job.append(mark, 0.0, format(color));
                rest = &rest[at + mark.len()..];
            }
            None => {
                job.append(rest, 0.0, format(words));
                rest = "";
            }
        }
    }
    job.wrap.max_width = max_width.max(1.0);
    job.wrap.max_rows = 1;
    job.wrap.break_anywhere = true;
    ui.fonts_mut(|fonts| fonts.layout_job(job))
}

// Fixed height keeps the virtual scroll position independent of label length.
fn entry_row(ui: &mut egui::Ui, name: &str, subtitle: &str, selected: bool) -> egui::Response {
    let width = ui.available_width() - design::S * 2.0;
    let font = if selected { Type::Strong } else { Type::Body };
    let title = design::words_that_fit(ui, name, &font.font(ui), width);
    // Round 25 FF5: the folder that tells two same-named packages apart comes last, as on the
    // reader's kind line. When the line does not fit, the words before it shorten (the package
    // name, which both rows share), never the folder.
    let small = Type::Small.font(ui);
    let detail = match subtitle
        .rsplit_once(" · ")
        .filter(|(_, folder)| folder.ends_with('/'))
    {
        Some((head, folder)) if design::number_width(ui, subtitle, &small) > width => {
            let tail = format!(" · {folder}");
            let room = width - design::number_width(ui, &tail, &small);
            format!("{}{tail}", design::words_that_fit(ui, head, &small, room))
        }
        _ => design::words_that_fit(ui, subtitle, &small, width),
    };
    // `design::two_line_row`, with the row's warning mark in its own tone.
    let response = design::row(
        ui,
        ENTRY_ROW_HEIGHT,
        selected,
        &format!("{title} · {detail}"),
        |ui, rect| {
            let c = colors();
            let width = rect.width() - 2.0 * design::S;
            let galley = design::one_line(ui, &title, font.font(ui), c.text, width);
            let total = galley.size().y + Type::Small.line();
            let top = rect.center().y - total / 2.0;
            ui.painter()
                .galley(egui::pos2(rect.left() + design::S, top), galley, c.text);
            let detail = marked_line(ui, &detail, Type::Small.font(ui), c.muted, width);
            ui.painter().galley(
                egui::pos2(rect.left() + design::S, top + total - Type::Small.line()),
                detail,
                c.muted,
            );
        },
    );
    design::named(ui, &response, &format!("{name} · {subtitle}"));
    response
}

pub(crate) fn local_module(path: &Path) -> Result<Module, String> {
    let path = path.canonicalize().map_err(|e| e.to_string())?;
    let (entries, _) = content::read_directory(&path)?;
    module_from_entries(path, &entries)
}

fn module_from_entries(path: PathBuf, entries: &[Entry]) -> Result<Module, String> {
    let manifests: Vec<_> = entries.iter().filter(|e| e.kind == "package").collect();
    if manifests.len() != 1 || entries.iter().any(|e| e.kind == "campaign") {
        return Err(
            "Choose a package folder with exactly one package manifest and no campaign manifest."
                .into(),
        );
    }
    let manifest = manifests[0];
    let ns = manifest
        .fields
        .get("splat_namespace")
        .and_then(|v| v.as_str())
        .ok_or("The package needs a namespace.")?;
    if !content::resource_key(&format!("{ns}:library")) || ns == "dnd2024" {
        return Err(
            "Use a valid package namespace other than the built-in dnd2024 namespace.".into(),
        );
    }
    Ok(Module {
        id: manifest.id,
        name: manifest.name.clone(),
        path,
    })
}

pub(crate) fn note_text(name: &str, kind: &str, namespace: Option<&str>) -> String {
    let mut fields = serde_yaml_ng::Mapping::new();
    fields.insert("splat_schema".into(), 2.into());
    fields.insert(
        "splat_id".into(),
        crate::hermetic::new_uuid().to_string().into(),
    );
    fields.insert("splat_kind".into(), kind.into());
    fields.insert("splat_name".into(), name.into());
    if namespace.is_none() {
        fields.insert(
            format!("splat_{kind}").into(),
            serde_yaml_ng::Value::Mapping(serde_yaml_ng::Mapping::new()),
        );
    }
    if let Some(ns) = namespace {
        fields.insert("splat_namespace".into(), ns.into());
    }
    if kind == "world" {
        fields.remove("splat_world");
        fields.insert("splat_type".into(), "other".into());
    }
    if kind == "creature" {
        fields.insert(
            "splat_creature".into(),
            serde_yaml_ng::from_str::<serde_yaml_ng::Value>(
                "size: medium\ntype: humanoid\nalignment: Neutral\nmovement_text: 30 ft.\ncr_text: '1 (XP 200; PB +2)'\n",
            )
            .expect("creature identity"),
        );
        fields.insert(
            "splat_stats".into(),
            serde_yaml_ng::from_str::<serde_yaml_ng::Value>(
                "initiative: {value: 0}\nspeed: {value: 30, unit: ft}\nstrength: {value: 10}\ndexterity: {value: 10}\nconstitution: {value: 10}\nintelligence: {value: 10}\nwisdom: {value: 10}\ncharisma: {value: 10}\n",
            )
            .expect("creature fields"),
        );
    }
    if kind == "spell" {
        fields.insert("splat_spell".into(), serde_yaml_ng::from_str::<serde_yaml_ng::Value>("level: 1\nschool: evocation\ncasting_time: Action\nrange_text: 60 feet\ncomponents: [verbal, somatic]\nduration: Instantaneous\n").expect("spell fields"));
    }
    if kind == "item" {
        fields.insert(
            "splat_item".into(),
            serde_yaml_ng::from_str::<serde_yaml_ng::Value>(
                "category: wondrous_item\nrarity: common\nequipment: {attunement: none}\n",
            )
            .expect("item fields"),
        );
    }
    if ["creature", "spell", "item"].contains(&kind) {
        fields.insert("splat_schema".into(), 2.into());
        let table = content::stat_blocks::render(&fields, kind);
        for key in content::stat_blocks::data_roots(kind) {
            fields.remove(*key);
        }
        let yaml = serde_yaml_ng::to_string(&fields).expect("string fields");
        return format!(
            "---\n{yaml}---\n\n# {name}\n\n{table}{}",
            if kind == "creature" {
                "## Traits\n\n**Trait.** Describe the trait.\n\n## Actions\n\n**Attack.** Describe the attack.\n\n"
            } else if kind == "spell" {
                "Describe the spell.\n"
            } else {
                "Describe the item and its use.\n"
            }
        );
    }
    if kind == "campaign" {
        fields.remove("splat_campaign");
    }
    let yaml = serde_yaml_ng::to_string(&fields).expect("string fields");
    format!(
        "---\n{yaml}---\n\n# {name}\n\n{}",
        if kind == "campaign" {
            "## Packages\n\n## Rule choices\n\n| Rule | Choice |\n| --- | --- |\n"
        } else if kind == "effect" {
            "Describe what changes and when it ends.\n"
        } else if kind == "rule" {
            "Describe when and how this rule applies.\n"
        } else if kind == "rule_group" {
            "## Rules\n\nDescribe the rules this group enables.\n"
        } else {
            ""
        }
    )
}

/// Consistent type and inset for entry form controls, including the editable Folder.
pub(crate) fn form_control<R>(ui: &mut egui::Ui, add: impl FnOnce(&mut egui::Ui) -> R) -> R {
    ui.scope(|ui| {
        ui.style_mut().override_font_id = Some(Type::Strong.font(ui));
        ui.spacing_mut().button_padding.x = design::S;
        add(ui)
    })
    .inner
}

pub(crate) fn creation_path(ui: &mut egui::Ui, root: &Path, relative: &str, destination: &str) {
    let full = format!("Will create {}", root.join(relative).display());
    let short = format!("Will create {destination}/{relative}");
    let text = design::words_that_fit(ui, &short, &Type::Small.font(ui), ui.available_width());
    let response = ui.label(Type::Small.rich(ui, text).color(colors().muted));
    design::named(ui, &response, &full);
    response.on_hover_text(full);
}

/// A file line for a file that moved on disk under a draft: the new path, `moved from <folder>/`,
/// then the file's links, so no link splits the statement about the path. The path gives way
/// first: it ends in `…` when the line is short, and its hover and accessible name keep it whole.
fn moved_file_parts(ui: &mut egui::Ui, root: &Path, relative: &str, from: &str, obsidian: bool) {
    let path = root.join(relative);
    let font = Type::Small.font(ui);
    let gap = ui.spacing().item_spacing.x;
    let budget = (ui.available_width()
        - crate::world::entry_file_links_width(ui, obsidian)
        - design::number_width(ui, from, &font)
        - 2.0 * gap
        - design::L)
        .max(design::XXL);
    let shown = design::words_that_fit(ui, relative, &font, budget);
    let response = ui.label(Type::Small.rich(ui, shown).color(colors().muted));
    design::named(ui, &response, &path.display().to_string());
    response.on_hover_text(path.display().to_string());
    ui.label(Type::Small.rich(ui, from).color(colors().muted));
    crate::world::entry_file_links(ui, &path, obsidian);
}

/// `Will create <path>` on one line of a creation form's kept line. When the line is short, the
/// path's start gives way (`Will create …/modules/copper_coast`), so its end, the new folder,
/// stays in view; the hover and the accessible name keep the whole path.
fn will_create(ui: &mut egui::Ui, path: &Path) {
    let full = format!("Will create {}", path.display());
    let font = Type::Small.font(ui);
    let width = ui.available_width();
    let mut shown = full.clone();
    let path = path.display().to_string();
    let starts: Vec<usize> = path.char_indices().map(|(at, _)| at).collect();
    if starts.len() > 1 && design::number_width(ui, &full, &font) > width {
        let fitted = |start: usize| format!("Will create …{}", &path[starts[start]..]);
        // The fewest characters cut that fit, by halves: a few measures, not one per character.
        let (mut low, mut high) = (1, starts.len() - 1);
        while low < high {
            let middle = (low + high) / 2;
            if design::number_width(ui, &fitted(middle), &font) <= width {
                high = middle;
            } else {
                low = middle + 1;
            }
        }
        shown = fitted(low);
    }
    let response = ui.label(Type::Small.rich(ui, shown).color(colors().muted));
    design::named(ui, &response, &full);
    response.on_hover_text(full);
}

/// Existing folders, with an optional kind folder as the first suggestion.
fn kind_folder(kind: &str) -> &'static str {
    match kind {
        "creature" => "Creatures",
        "spell" => "Spells",
        "item" => "Items",
        "effect" => "Effects",
        _ => "Rules",
    }
}

fn visit_folders(root: &Path, path: &Path, folders: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(path) else {
        return;
    };
    for entry in entries.flatten() {
        if entry.file_type().is_ok_and(|kind| kind.is_dir())
            && !entry.file_name().to_string_lossy().starts_with('.')
        {
            if let Ok(path) = entry.path().strip_prefix(root) {
                folders.push(path.to_string_lossy().to_string());
            }
            visit_folders(root, &entry.path(), folders);
        }
    }
}

pub(crate) fn folder_field(
    ui: &mut egui::Ui,
    id: design::FieldId,
    label: &str,
    value: &mut String,
    root: &Path,
    kind: Option<&str>,
) {
    let mut folders = Vec::new();
    visit_folders(root, root, &mut folders);
    if let Some(kind) = kind {
        folders.push(
            match kind {
                "creature" => "Creatures",
                "spell" => "Spells",
                "item" => "Items",
                "effect" => "Effects",
                _ => "Rules",
            }
            .into(),
        );
    }
    folders.sort();
    folders.dedup();
    let query = value.to_lowercase();
    design::field(
        ui,
        label,
        Some("Type a new folder, or choose one. A new folder is made with the file."),
        None,
        |ui| {
            form_control(ui, |ui| {
                let frame = egui::Frame::new()
                    .fill(colors().well)
                    .stroke(egui::Stroke::new(
                        design::HAIRLINE_WIDTH,
                        colors().field_border,
                    ))
                    .corner_radius(design::RADIUS_CONTROL as u8)
                    .inner_margin(egui::Margin::symmetric(design::S as i8, 0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            let width =
                                (ui.available_width() - design::XL - ui.spacing().item_spacing.x)
                                    .max(design::XXL);
                            let font = Type::Strong.font(ui);
                            let response = id.single_line(ui, value, |edit| {
                                edit.frame(false)
                                    .font(font)
                                    .margin(egui::Vec2::ZERO)
                                    // Round 27 FH10: centred, as `Main folder` is painted, so
                                    // typed words and the chosen value share one baseline.
                                    .vertical_align(egui::Align::Center)
                                    .desired_width(width)
                                    .min_size(egui::vec2(width, design::XL))
                            });
                            design::named(ui, &response, label);
                            let typing = response.has_focus();
                            if value.is_empty() {
                                ui.painter().text(
                                    response.rect.left_center(),
                                    egui::Align2::LEFT_CENTER,
                                    "Main folder",
                                    Type::Strong.font(ui),
                                    colors().text,
                                );
                            }
                            let (rect, response) = ui.allocate_exact_size(
                                egui::vec2(design::XL, design::XL),
                                egui::Sense::click(),
                            );
                            let icon = egui::Align2::RIGHT_CENTER.align_size_within_rect(
                                egui::Vec2::splat(ui.spacing().icon_width),
                                rect,
                            );
                            design::combo_icon(
                                ui,
                                icon,
                                ui.style().interact(&response),
                                egui::Popup::is_id_open(
                                    ui.ctx(),
                                    egui::Popup::default_response_id(&response),
                                ),
                            );
                            design::named(ui, &response, &format!("Suggestions for {label}"));
                            design::menu_popup(&*ui, &response).show(|ui| {
                                if design::extras::choice_wrapped(
                                    ui,
                                    value.is_empty(),
                                    "Main folder",
                                    None,
                                )
                                .clicked()
                                {
                                    value.clear();
                                    ui.close();
                                }
                                for folder in folders
                                    .iter()
                                    .filter(|folder| folder.to_lowercase().contains(&query))
                                {
                                    if design::extras::choice_wrapped(
                                        ui,
                                        value == folder,
                                        folder,
                                        None,
                                    )
                                    .clicked()
                                    {
                                        *value = folder.clone();
                                        ui.close();
                                    }
                                }
                            });
                            typing
                        })
                        .inner
                    });
                // Round 25 FE1: the caret in the field shows the focus ring round the whole
                // field, as `text_field` does. The suggestions button keeps its own ring.
                if frame.inner {
                    design::focus_ring(ui, frame.response.rect);
                }
            });
        },
    );
}

/// `Choose for this campaign` and its help, which `help` gives for the box's state after this
/// frame's click.
#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
)]
pub(crate) fn use_checkbox(
    ui: &mut egui::Ui,
    not_used: &mut bool,
    help: impl FnOnce(bool) -> &'static str,
) {
    let mut enabled = !*not_used;
    ui.checkbox(&mut enabled, "Choose for this campaign");
    *not_used = !enabled;
    // Help keeps the reader measure (round 25 family).
    design::help(ui, help(enabled));
    // Round 27 FB9, FD8: `L` after the help, as after a field, so the help stays with its
    // checkbox and does not read as help for the next field.
    ui.add_space(design::L - ui.spacing().item_spacing.y);
}

/// The commit of `Register a copy`: one word, so the button keeps its width while the DM types
/// the name (round 27 FD2).
const REGISTER_COPY: &str = "Register";

fn create_module(path: &Path, name: &str, namespace: &str) -> Result<Module, String> {
    if !path.is_absolute()
        || name.trim().is_empty()
        || namespace == "dnd2024"
        || !content::resource_key(&format!("{namespace}:library"))
    {
        return Err(
            "Enter a name, a new absolute folder path, and a namespace such as my_homebrew.".into(),
        );
    }
    fs::create_dir(path).map_err(|e| format!("Could not create a new package folder: {e}"))?;
    let text = note_text(name.trim(), "package", Some(namespace));
    if let Err(error) = write_note(path, "Package.md", None, &text) {
        let _ = fs::remove_dir(path);
        return Err(error);
    }
    local_module(path)
}

// Refuse traversal and symlinks, and preserve external edits before atomic replacement.
/// The rule a note's path must keep, checked without the disk: relative, inside its folder.
pub(crate) fn note_path_rule(relative: &str) -> Option<&'static str> {
    let relative = Path::new(relative);
    (relative.as_os_str().is_empty()
        || relative
            .components()
            .any(|c| !matches!(c, std::path::Component::Normal(_))))
    .then_some("Use a relative note path within the package.")
}

pub(crate) fn note_path(root: &Path, relative: &str) -> Result<PathBuf, String> {
    if fs::symlink_metadata(root)
        .map_err(|error| {
            crate::world::file_fault::FileFault::read(&root.display().to_string(), &error).message()
        })?
        .file_type()
        .is_symlink()
    {
        return Err(
            "The package folder was replaced with a symbolic link. Register the actual folder."
                .into(),
        );
    }
    if let Some(rule) = note_path_rule(relative) {
        return Err(rule.into());
    }
    let relative = Path::new(relative);
    let mut path = root.to_path_buf();
    for component in relative.components() {
        path.push(component);
        if fs::symlink_metadata(&path).is_ok_and(|m| m.file_type().is_symlink()) {
            return Err("Editing through symbolic links is not supported.".into());
        }
    }
    Ok(path)
}

/// What makes `write_note` refuse a note before it writes: the file is read only, or the
/// campaign folder is. Save's reason names it and says what to do (AL1, FL2, SL1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ReadOnly {
    File,
    Folder,
}

impl ReadOnly {
    pub(crate) fn of(root: &Path, path: &Path) -> Option<Self> {
        if fs::metadata(path).is_ok_and(|m| m.permissions().readonly()) {
            Some(Self::File)
        } else if fs::metadata(root).is_ok_and(|m| m.permissions().readonly()) {
            Some(Self::Folder)
        } else {
            None
        }
    }

    pub(crate) fn reason(self) -> &'static str {
        match self {
            Self::File => "This file is read only. Make it writable, then Save again.",
            Self::Folder => "The campaign folder is read only. Make it writable, then Save again.",
        }
    }
}

pub(crate) fn write_note(
    root: &Path,
    relative: &str,
    baseline: Option<&str>,
    text: &str,
) -> Result<Entry, String> {
    let entry = Entry::parse(relative, text)?;
    let path = note_path(root, relative)?;
    if let Some(read_only) = ReadOnly::of(root, &path) {
        return Err(read_only.reason().into());
    }
    let current = match fs::read_to_string(&path) {
        Ok(text) => Some(text),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
        Err(e) => return Err(crate::world::file_fault::FileFault::read(relative, &e).message()),
    };
    if baseline.is_none() && current.is_some() {
        return Err("A note with this name already exists. Choose another name.".into());
    }
    if current.as_deref() != baseline {
        return Err(NOTE_CHANGED.into());
    }
    // The system's words name no file; the DM reads which note and how to recover.
    let failed = crate::world::file_fault::write_error(relative);
    if baseline.is_none() {
        fs::create_dir_all(path.parent().ok_or("Missing parent folder")?).map_err(&failed)?;
    }
    let mut temp = tempfile::NamedTempFile::new_in(path.parent().ok_or("Missing parent folder")?)
        .map_err(&failed)?;
    temp.write_all(text.as_bytes())
        .and_then(|_| temp.as_file().sync_all())
        .map_err(&failed)?;
    if baseline.is_some() {
        temp.persist(&path).map_err(|error| failed(error.error))?;
    } else {
        temp.persist_noclobber(&path)
            .map_err(|error| failed(error.error))?;
    }
    Ok(entry)
}

/// A note that the folder reading reported a problem with. A file the system will not read keeps
/// its fault, from the error's kind, so the reader names the real cause and never offers a
/// repair of text it does not have. Nothing here changes the file.
fn problem_note(
    root: &Path,
    folder: &content::Folder,
    error: content::Diagnostic,
    owner: Owner,
) -> Result<Note, String> {
    let full = note_path(root, &error.path)?;
    let read = match folder.text(&error.path) {
        Some(text) => Ok(text.to_owned()),
        // A folder that could not be listed: its own error kind says why.
        None if full.is_dir() => fs::read_dir(&full).map(|_| String::new()),
        None => fs::read_to_string(&full),
    };
    let (text, fault) = match read {
        Ok(text) => (text, None),
        Err(e) => (String::new(), Some(FileFault::read(&error.path, &e))),
    };
    Ok(Note {
        error: Some(
            fault
                .as_ref()
                .map_or(error.message, |fault| fault.cause().to_owned()),
        ),
        path: error.path,
        text,
        entry: None,
        fault,
        owner,
    })
}

fn load(module: Option<&Module>) -> Result<Loaded, String> {
    let mut notes = Vec::new();
    if let Some(module) = module {
        let path = module.path.canonicalize().map_err(|e| e.to_string())?;
        let folder = content::read_folder(&path)?;
        let (entries, mut errors) = (folder.entries.clone(), folder.errors.clone());
        let manifest_path = crate::content_workspace::manifest_path_in(&folder, module);
        let has_manifest = entries.iter().any(|entry| entry.kind == "package");
        let missing_manifest = !has_manifest
            && !crate::content_workspace::manifest_file_exists(module, &manifest_path);
        // The folder was just read; its texts are the ones these entries came from.
        let text_of = |path: &str| -> Result<String, String> {
            let full = note_path(&module.path, path)?;
            match folder.text(path) {
                Some(text) => Ok(text.to_owned()),
                None => fs::read_to_string(full).map_err(|e| FileFault::read(path, &e).message()),
            }
        };
        if !has_manifest
            && !missing_manifest
            && !errors.iter().any(|error| error.path == manifest_path)
        {
            let message = text_of(&manifest_path)
                .and_then(|text| Entry::parse(&manifest_path, &text))
                .err();
            if let Some(message) = message {
                errors.push(content::Diagnostic {
                    path: manifest_path.clone(),
                    message,
                });
            }
        }
        for entry in entries {
            let text = text_of(&entry.path)?;
            let mismatched = entry.kind == "package" && entry.id != module.id;
            notes.push(Note {
                path: entry.path.clone(),
                text,
                entry: (!mismatched).then_some(entry),
                error: mismatched.then(|| "Package details have another identity. Replace with the registered details, or restore the original id.".into()),
                fault: None,
                owner: Owner::Package(module.clone()),
            });
        }
        if missing_manifest {
            notes.push(Note {
                path: manifest_path.clone(),
                text: String::new(),
                entry: None,
                error: Some(crate::content_workspace::DELETED_MANIFEST.into()),
                fault: None,
                owner: Owner::Package(module.clone()),
            });
        }
        for error in errors {
            notes.push(problem_note(
                &module.path,
                &folder,
                error,
                Owner::Package(module.clone()),
            )?);
        }
    } else {
        for entry in content::definitions::entries()?.iter().cloned() {
            let text = content::definitions::source_file(&entry.path)
                .ok_or("Missing built-in note")?
                .to_owned();
            notes.push(Note {
                path: entry.path.clone(),
                text,
                entry: Some(entry),
                error: None,
                fault: None,
                owner: Owner::Builtin,
            });
        }
    }
    notes.sort_by_cached_key(|n| natural_key(n.name()));
    Ok(Loaded {
        notes,
        ..Default::default()
    })
}

/// The whole catalogue: built-in, every package, and the active campaign's own entries.
pub(crate) fn load_all(modules: &[Module], campaign: Option<&Campaign>) -> Result<Loaded, String> {
    let mut all = load(None)?;
    all.owners = modules
        .iter()
        .cloned()
        .map(Owner::Package)
        .chain(campaign.cloned().map(Owner::Campaign))
        .collect();
    for module in modules {
        // A load error names its package, and its folder when another package has the same name.
        let owner = Owner::Package(module.clone());
        let name = if modules.iter().filter(|m| m.name == module.name).count() > 1 {
            format!("{} · {}/", module.name, owner.folder())
        } else {
            module.name.clone()
        };
        let key = (Some(module.id), String::new());
        if let Err(error) = splat::content::art::ArtIndex::default().add_package(&module.path) {
            all.load_errors
                .push((key.clone(), format!("{name}: {error}")));
        }
        match load(Some(module)) {
            Ok(package) => {
                all.notes.extend(package.notes);
            }
            Err(error) => all.load_errors.push((key, format!("{name}: {error}"))),
        }
    }
    if let Some(campaign) = campaign {
        let owner = Owner::Campaign(campaign.clone());
        match content::read_folder(&campaign.path) {
            Ok(folder) => {
                let (entries, errors) = (folder.entries.clone(), folder.errors.clone());
                let text_of = |path: &str| -> Result<String, String> {
                    let full = note_path(&campaign.path, path)?;
                    match folder.text(path) {
                        Some(text) => Ok(text.to_owned()),
                        None => fs::read_to_string(full)
                            .map_err(|e| FileFault::read(path, &e).message()),
                    }
                };
                for entry in entries.into_iter().filter(|e| {
                    e.kind != "campaign" && !splat::character::campaign_package::is_manifest(e)
                }) {
                    let text = text_of(&entry.path)?;
                    all.notes.push(Note {
                        path: entry.path.clone(),
                        text,
                        entry: Some(entry),
                        error: None,
                        fault: None,
                        owner: owner.clone(),
                    });
                }
                for error in errors {
                    all.notes
                        .push(problem_note(&campaign.path, &folder, error, owner.clone())?);
                }
            }
            Err(error) => all.load_errors.push((
                (Some(campaign.id), String::new()),
                format!("{}: {error}", campaign.name),
            )),
        }
        // If the selections cannot be read, say so and show everything rather than nothing.
        match CampaignDocument::load(campaign)
            .and_then(|document| Ok((document, Catalogue::load(modules, Some(campaign))?)))
        {
            Ok((document, catalogue)) => {
                let closure = catalogue.closure(&document.selections, campaign.id);
                all.chosen = Some(closure.direct.iter().copied().collect());
                all.uses = Some(
                    closure
                        .direct
                        .iter()
                        .copied()
                        .chain(closure.required.keys().copied())
                        .collect(),
                );
            }
            Err(error) => all.load_errors.push((
                (Some(campaign.id), String::new()),
                format!("{}: {error}", campaign.name),
            )),
        }
    }
    all.notes.sort_by_cached_key(|n| natural_key(n.name()));
    all.refresh_diagnostics();
    Ok(all)
}

/// Undo discard offers words only to the note they came from. Two owners can each have a
/// note at the same path.
fn discard_is_for(discarded: Option<&(Option<Uuid>, String, String)>, note: &Note) -> bool {
    discarded.is_some_and(|(owner, path, _)| *owner == note.owner.id() && *path == note.path)
}

/// The catalogue needs reloading when either of these changes.
type LoadKey = (Option<Uuid>, Vec<Uuid>);

fn load_key(campaigns: &Campaigns) -> LoadKey {
    (
        campaigns.active_campaign().map(|c| c.id),
        campaigns.modules().iter().map(|m| m.id).collect(),
    )
}

pub(crate) fn catalogue_subtitles(
    catalogue: &crate::content_workspace::Catalogue,
) -> std::collections::HashMap<Uuid, String> {
    let loaded = Loaded {
        notes: catalogue
            .entries
            .values()
            .map(|e| Note {
                path: e.entry.path.clone(),
                text: String::new(),
                entry: Some(e.entry.clone()),
                error: None,
                fault: None,
                owner: Owner::Builtin,
            })
            .collect(),
        ..Default::default()
    };
    loaded
        .notes
        .iter()
        .zip(loaded.subtitles())
        .filter_map(|(note, suffix)| Some((note.entry.as_ref()?.id, suffix.clone())))
        .collect()
}

struct SelectionUndo {
    campaign: Campaign,
    before: Vec<crate::content_workspace::PackageSelection>,
    after: Vec<crate::content_workspace::PackageSelection>,
}

struct DependencyUndo {
    root: PathBuf,
    path: String,
    before: String,
    after: String,
    copied_path: String,
    copied_text: String,
    /// The result's words: `<entry> copied to <package> and linked.`
    result: String,
    /// When the copy was made, for the footer's order.
    order: design::ResultOrder,
}

struct IdentityUndo {
    root: PathBuf,
    path: String,
    before: String,
    after: String,
}

/// The draft as parsed, with the revision of that parse: a new parse (new draft text) is a new
/// revision, and caches of what the parse holds are keyed by it.
type ParsedDraft = design::memo::Revised<std::sync::Arc<Result<Entry, String>>>;

/// A missing link Campaign's repair sent to a file: the file, the typed reference and the
/// fault's words. `found` is the row that links it in one parse of the draft, keyed by that
/// parse's revision; the draft's parse is rebuilt only when its text changes, so the row is
/// looked up again only then.
struct RepairLink {
    path: PathBuf,
    target: splat::content::Reference,
    words: String,
    found: design::memo::Memo<design::memo::Revision, Option<usize>>,
}

#[cfg(test)]
thread_local! {
    static REPAIR_LOOKUPS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// The row that links a repair's reference. Every repair lookup goes through here, and the
/// test count is kept here.
fn repair_row(entry: &Entry, target: &splat::content::Reference) -> Option<usize> {
    #[cfg(test)]
    REPAIR_LOOKUPS.with(|count| count.set(count.get() + 1));
    splat::content::list_formats::row_linking(entry, target)
}

#[cfg(test)]
thread_local! {
    static BROKEN_LINE_SCANS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static BROKEN_WORD_COMPOSITIONS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// The message space's accessible text for the broken-link warnings. Every composition goes
/// through here, and the test count is kept here.
fn broken_words(warnings: &[String]) -> String {
    #[cfg(test)]
    BROKEN_WORD_COMPOSITIONS.with(|count| count.set(count.get() + 1));
    warnings.join(" ")
}

/// Edit Markdown's warning for each link to no entry in a draft, as the row's `N broken links`
/// counts them (`Line 17: this link names no entry.`), outside the front matter and code fences.
/// `skip` is the line the link Campaign sent the DM to fix, which has its own words. The whole
/// preparation (the scan and the words) goes through here, and the test count is kept here.
fn broken_link_warnings(
    draft: &str,
    entry: &Entry,
    index: &crate::lookup::ContentIndex,
    skip: Option<usize>,
) -> (Vec<String>, String) {
    #[cfg(test)]
    BROKEN_LINE_SCANS.with(|count| count.set(count.get() + 1));
    let mut lines = Vec::new();
    let mut front = false;
    let mut fence = false;
    for (at, line) in draft.lines().enumerate() {
        let trimmed = line.trim();
        if at == 0 && trimmed == "---" {
            front = true;
            continue;
        }
        if front {
            front = trimmed != "---";
            continue;
        }
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            fence = !fence;
            continue;
        }
        if fence || presentation::hidden_provenance_line(line) {
            continue;
        }
        for target in crate::world::links_in(line) {
            if index.link_is_missing(&target, entry.id) && skip != Some(at + 1) {
                lines.push(format!("Line {}: this link names no entry.", at + 1));
            }
        }
    }
    // The message space's accessible text for these warnings, composed once with them.
    let words = broken_words(&lines);
    (lines, words)
}

/// The broken-link warnings of one parsed draft against one link index, with the line Campaign's
/// link has. A new draft (a new parse), a new index or another such line prepares them again; an
/// unchanged frame prepares nothing and borrows them. The parse is known by its revision.
type BrokenLines = (
    design::memo::Revision,
    std::sync::Arc<crate::lookup::ContentIndex>,
    Option<usize>,
    (Vec<String>, String),
);

#[derive(Default)]
pub struct Library {
    /// The draft as last parsed, with its path. The editor needs the parse on every frame,
    /// and parsing a long note each time took most of the frame.
    draft_parse: Option<(String, String, ParsedDraft)>,
    /// The list should scroll to the selected entry, which arrived by a link.
    reveal_row: bool,
    list_hidden: bool,
    /// The draft `Discard changes` last dropped, with its note's path, for one step of undo.
    /// The note's owner and path, and the discarded words.
    discarded: Option<(Option<Uuid>, String, String)>,
    discarded_note: Option<LetGo>,
    /// A Save that failed on the disk, for the draft it tried to write.
    save_failure: Option<SaveFailure>,
    /// Raised by every event that changes the open draft (an edit, Discard and its Undo, a
    /// reload, a repair action). A failed Save keeps the revision it wrote, so no frame compares
    /// the draft's words to know whether it is still that draft.
    draft_revision: u64,
    /// Test only: how often the failed-Save check ran, and how often it read the draft's words
    /// (counted by `DraftView` itself).
    #[cfg(test)]
    failure_checks: std::cell::Cell<(usize, usize)>,
    identity_undo: Option<IdentityUndo>,
    identities_undo: Option<(crate::content_workspace::ContentEdits, String)>,
    manifest_write: Option<crate::content_workspace::ManifestWrite>,
    package_copy: Option<crate::content_workspace::PackageCopy>,
    copy_not_used: bool,
    copy_campaign_before: Option<(Campaign, String, String, String)>,
    unregistered: Option<Module>,
    copied_rosters: Option<crate::content_workspace::ContentEdits>,
    /// Offers to change links after a rename, each owned by the renamed entry.
    renamed_links: design::Results<LinkOwner, LinkOffer>,
    /// Link updates and the names their links now say, each owned by the renamed entry.
    link_edits: design::Results<LinkOwner, LinkResult>,
    /// This frame's page results (link update, registration), drawn in the pane's message footer.
    page_results: PageResults,
    /// Which page result's action the reader pressed, applied after the reader is drawn.
    page_pressed: PagePress,
    copied_package: Option<Module>,
    copy_name: String,
    copy_name_open: bool,
    /// A save was refused because the file changed. The draft waits for the DM's choice.
    disk_change: Option<DiskChange>,
    /// The note on screen was saved, and has not changed since.
    just_saved: bool,
    overwrite_disk: bool,
    scope: Scope,
    load_key: Option<LoadKey>,
    stale: bool,
    matches_scope: Option<Scope>,
    scope_counts: Option<(Scope, Vec<(Section, usize)>)>,
    confirm_unregister: Option<Module>,
    new_place: Option<usize>,
    /// A saved note's owner folder, so other pages can read it again.
    pub content_changed: Option<PathBuf>,
    /// A package's new name, for the registration.
    pub renamed: Option<(Uuid, String)>,
    /// The DM asked to go and add a campaign.
    pub wants_settings: bool,
    loaded: Option<Loaded>,
    pending: Option<mpsc::Receiver<Result<Loaded, String>>>,
    selected: Option<usize>,
    draft: String,
    search: String,
    matches: Option<(Section, String, Vec<usize>)>,
    /// Chosen creature facts. They apply in Creatures and to search results.
    filter: Filter,
    /// The filter that `matches` used.
    matched_filter: Filter,
    /// Where results found only with a typo start in `matches`.
    typo_from: usize,
    section: Section,
    unreadable_kind: Option<Section>,
    error: Option<String>,
    form: bool,
    path: String,
    name: String,
    pub run_open: bool,
    pub open_world_npc: Option<Uuid>,
    pending_edit: Option<Entry>,
    pending_repair: Option<PathBuf>,
    /// A missing link that Campaign's repair opened this file for. Edit Markdown names it at its
    /// line while the draft still holds it; it ends when another note opens or a Save writes.
    repair_link: Option<RepairLink>,
    /// The open draft's broken-link lines, for Edit Markdown's warnings (round 25 FH6).
    broken_lines: Option<BrokenLines>,
    scope_notice: Option<String>,
    new_form: bool,
    focus_new_name: bool,
    focus_hp: bool,
    focus_repair: bool,
    focus_markdown: bool,
    new_name: String,
    new_kind: String,
    new_folder: String,
    new_not_used: bool,
    /// The New entry form's last create error. It shows in the pane's message footer and ends
    /// when the form closes, the name changes, or an entry is created.
    new_error: Option<String>,
    package_not_used: bool,
    focus_package_name: bool,
    package_path: Option<PathBuf>,
    package_folder_changed: bool,
    pub use_entry: Option<(Uuid, Uuid)>,
    source: bool,
    reselect: Option<(Option<Uuid>, String)>,
    pub copy: Option<crate::content_workspace::CopySource>,
    pub copy_to_package: Option<Uuid>,
    /// An entry to read beside the open draft, in Lookup.
    pub peek: Option<Uuid>,
    trail: Trail<Place>,
    /// Each `Copy <entry> to <package>` result with its Undo, newest last. A result belongs to
    /// the file it linked (its package root and path), so a file at the same path in another
    /// package never shows it. Each ends on its own Undo, or on a reload that finds one of its
    /// files changed; other copies keep theirs (round 26 review).
    dependency_undos: Vec<DependencyUndo>,
    selection_undo: Option<SelectionUndo>,
    /// The last reload found a registration file changed, so `Undo package registration` would
    /// refuse. The message stays for its other action; its Undo is blocked with the reason.
    registration_changed: bool,
    /// The same for `Undo source change`.
    rosters_changed: bool,
    link_choices: Vec<Uuid>,
    /// The file, the draft before and after a stat fix, and its result message.
    vital_undo: Option<(String, String, String, String)>,
    heading_undo: Option<(String, String, String)>,
    /// When each kept result's action was done (`stamp_result`), so the message footer lists
    /// results newest first. The next action of the same kind replaces its entry.
    result_orders: ResultOrders,
}

/// A kept result of the Library's panes, for its footer order. A copy result has its own order
/// in its `DependencyUndo`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum OrderKey {
    Reader(ResultKind),
    Heading,
    Vital,
    /// An entry removed on disk, whose draft was let go.
    Removed,
}

type ResultOrders = std::collections::HashMap<OrderKey, design::ResultOrder>;

/// One of the reader's results in a footer, in Read and in Edit Markdown alike. A link offer and
/// a link result each belong to one renamed entry: they go through `Footer::kept`, with the order
/// their store issued bound to their words. The other kinds hold one result at a time, so their
/// kind's slot is that result's.
#[allow(
    clippy::disallowed_methods,
    reason = "staged: calls `ResultOrder::issue`, `Footer::result` or `save_row_state` (design::results::STAGED_ALLOWS)"
)]
fn add_reader_result<'a>(
    footer: &mut design::Footer<'a>,
    results: &'a ReaderResults,
    orders: &ResultOrders,
    (kind, words, undo): (ResultKind, String, bool),
    draw: impl FnOnce(&mut egui::Ui) + 'a,
) {
    let page = &results.page;
    let line = match kind {
        ResultKind::LinkOffer => page.link_offer.as_ref().map(|(line, _)| line),
        ResultKind::Links => page.links.as_ref(),
        _ => None,
    };
    match line {
        Some(line) => {
            footer.kept_line(line, undo, move |_, ui| draw(ui));
        }
        None => {
            footer.result(order_of(orders, OrderKey::Reader(kind)), words, undo, draw);
        }
    }
}

/// The order of the result `key`, issued when its action was done.
fn order_of(orders: &ResultOrders, key: OrderKey) -> design::ResultOrder {
    // Every action that makes a kept result stamps it (`Library::stamp_result`). A result made
    // elsewhere without one is older than every stamped one: a draw never issues an order.
    orders.get(&key).copied().unwrap_or_default()
}
/// Sort so that "Level 2" comes before "Level 18".
fn natural_key(name: &str) -> Vec<(String, u64)> {
    let lower = name.to_lowercase();
    let mut parts = Vec::new();
    let mut text = String::new();
    let mut chars = lower.chars().peekable();
    while let Some(c) = chars.next() {
        if c.is_ascii_digit() {
            let mut number = u64::from(c.to_digit(10).unwrap_or(0));
            while let Some(digit) = chars.peek().and_then(|c| c.to_digit(10)) {
                number = number.saturating_mul(10).saturating_add(u64::from(digit));
                chars.next();
            }
            parts.push((std::mem::take(&mut text), number));
        } else {
            text.push(c);
        }
    }
    parts.push((text, 0));
    parts
}

pub(crate) const EDITOR_MAX_WIDTH: f32 = 720.0;

#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
)]
fn form_heading(ui: &mut egui::Ui, title: &str) {
    ui.add_space(design::L);
    ui.label(
        Type::Label
            .rich(ui, title.to_uppercase())
            .color(colors().text),
    );
}

fn obsidian_for(root: &Path, owner: &Owner) -> bool {
    matches!(owner, Owner::Campaign(_))
        || root
            .ancestors()
            .any(|parent| parent.join(".obsidian").is_dir())
}

/// The reader's owner and file line is one Read row for every owner, whatever it holds
/// (status, `Show file`, a quiet `Choose all`), so the title below it does not move
/// between a built-in entry, a campaign entry and a package About.
fn file_line_row(ui: &mut egui::Ui, wrap: bool, add: impl FnOnce(&mut egui::Ui)) {
    let height = Density::Read.control();
    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), height),
        egui::Layout::left_to_right(egui::Align::Center).with_main_wrap(wrap),
        |ui| {
            ui.set_min_height(height);
            add(ui);
        },
    );
}
const NOTE_CHANGED: &str = "This entry changed on disk.";
/// What is on disk now, in place of what the draft started from.
struct DiskChange {
    /// None when the file has gone, or cannot be read.
    text: Option<String>,
    moved_to: Option<String>,
    /// Why the file is there but cannot be read. The draft cannot be saved over it.
    fault: Option<FileFault>,
}

/// What a note's file holds now: its text, None when it has gone, or why it cannot be read.
fn read_disk(root: &Path, relative: &str) -> Result<Option<String>, FileFault> {
    let Ok(path) = note_path(root, relative) else {
        return Ok(None);
    };
    match fs::read_to_string(path) {
        Ok(text) => Ok(Some(text)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(FileFault::read(relative, &error)),
    }
}

/// Check readable and broken notes, so a move cannot create a second identity.
fn moved_note(root: &Path, id: Uuid, old: &str) -> Result<Option<(String, String)>, String> {
    let (entries, errors) = content::read_directory(root)?;
    let mut found = None;
    for path in entries
        .into_iter()
        .map(|e| e.path)
        .chain(errors.into_iter().map(|e| e.path))
    {
        if path == old {
            continue;
        }
        // A file that cannot be read might hold this ID: say which, rather than guess.
        let text = fs::read_to_string(note_path(root, &path)?)
            .map_err(|e| FileFault::read(&path, &e).message())?;
        if crate::content_workspace::note_id(&text) == Some(id) {
            if found.is_some() {
                return Err(
                    "Several files have this entry’s ID. Keep one file, then try again.".into(),
                );
            }
            found = Some((path, text));
        }
    }
    Ok(found)
}
/// An entry Back or Forward returns to, with the list as it was there.
struct Place {
    owner: Option<Uuid>,
    path: String,
    scope: Scope,
    section: Section,
    search: String,
}
impl Place {
    fn find(&self, notes: &[Note]) -> Option<usize> {
        notes
            .iter()
            .position(|n| n.owner.id() == self.owner && n.path == self.path)
    }
}
impl Library {
    /// The footer order of the open entry's link result: the order its own update issued
    /// (review of PR 298), for the check that proves it.
    #[cfg(test)]
    pub(crate) fn open_link_result_order(&self) -> Option<design::ResultOrder> {
        let owner = self.open_link_owner()?;
        self.link_edits.get(&owner).map(|shown| shown.order())
    }

    /// The open entry as the owner of its link offer and link result.
    fn open_link_owner(&self) -> Option<LinkOwner> {
        let (id, root) = self.open_entry_key()?;
        Some(LinkOwner { id, root })
    }

    pub fn refocus_draft(&mut self) {
        self.focus_markdown = true;
    }
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: calls `ResultOrder::issue`, `Footer::result` or `save_row_state` (design::results::STAGED_ALLOWS)"
    )]
    fn register_copy(&mut self, campaigns: &mut Campaigns) -> Result<(), String> {
        let source = self
            .copied_package
            .clone()
            .ok_or("Choose the copied folder first.")?;
        if campaigns
            .modules()
            .iter()
            .any(|m| m.path.starts_with(&source.path) || source.path.starts_with(&m.path))
        {
            return Err("Choose a separate copied folder first.".into());
        }
        let campaign_document = if !self.copy_not_used {
            campaigns
                .active_campaign()
                .map(|campaign| CampaignDocument::load(&campaign))
                .transpose()?
        } else {
            None
        };
        let copy = crate::content_workspace::PackageCopy::write(&source.path, &self.copy_name)?;
        if let Err(error) = campaigns.register_module(copy.module.clone()) {
            copy.undo()?;
            return Err(error);
        }
        if let Some(mut document) = campaign_document {
            let before = document.text.clone();
            document
                .selections
                .push(crate::content_workspace::PackageSelection {
                    name: Some(copy.module.name.clone()),
                    package: crate::content_workspace::EntryRef::id(copy.module.id),
                    selection: crate::content_workspace::Selection::All,
                });
            if let Err(error) = document.save() {
                let unregister = campaigns.unregister_module(copy.module.id);
                let restore = copy.undo();
                return Err(format!(
                    "Could not use the copy: {error}. {} {}",
                    unregister.err().unwrap_or_default(),
                    restore.err().unwrap_or_default()
                ));
            }
            self.copy_campaign_before =
                Some((document.campaign, document.path, before, document.text));
        }
        self.result_orders.insert(
            OrderKey::Reader(ResultKind::Registration),
            design::ResultOrder::issue(),
        );
        self.package_copy = Some(copy);
        self.registration_changed = false;
        self.rosters_changed = false;
        self.copied_package = None;
        self.copy_name_open = false;
        self.form = false;
        self.stale = true;
        self.scope = Scope::All;
        self.content_changed = Some(source.path.clone());
        Ok(())
    }

    fn undo_package_registration(&mut self, campaigns: &mut Campaigns) -> Result<(), String> {
        let copy = self
            .package_copy
            .as_ref()
            .ok_or("No package registration to undo.")?;
        let mut extra: Vec<_> = self
            .copy_campaign_before
            .as_ref()
            .map(|(campaign, path, before, after)| {
                (
                    campaign.path.clone(),
                    path.clone(),
                    after.clone(),
                    before.clone(),
                )
            })
            .into_iter()
            .collect();
        if let Some(edits) = &self.copied_rosters {
            extra.extend(edits.inverse_files());
        }
        let edits = copy.undo_with(extra)?;
        if let Err(error) = campaigns.unregister_module(copy.module.id) {
            return Err(match edits.undo() {
                Ok(()) => error,
                Err(restore) => format!("{error}. Could not restore the copy: {restore}"),
            });
        }
        self.content_changed = Some(copy.module.path.clone());
        self.package_copy = None;
        self.copy_campaign_before = None;
        self.copied_rosters = None;
        self.stale = true;
        Ok(())
    }

    /// On reload, each result with an Undo checks that its files still hold what its action
    /// wrote. If any file differs, Undo would only refuse: the result and its Undo go. A message
    /// with another action stays and blocks its Undo instead. Call this once per reload, not every
    /// frame: it reads the files.
    fn forget_changed_undos(&mut self) {
        use crate::content_workspace::{CampaignDocument, ManifestWrite, holds};
        ManifestWrite::forget_if_changed(&mut self.manifest_write);
        if self
            .identities_undo
            .as_ref()
            .is_some_and(|(edits, _)| !edits.on_disk())
        {
            self.identities_undo = None;
        }
        if self
            .identity_undo
            .as_ref()
            .is_some_and(|undo| !holds(&undo.root, &undo.path, &undo.after))
        {
            self.identity_undo = None;
        }
        self.dependency_undos.retain(|undo| {
            holds(&undo.root, &undo.path, &undo.after)
                && holds(&undo.root, &undo.copied_path, &undo.copied_text)
        });
        // Its Undo checks the package choices only, so other campaign edits keep it.
        if self.selection_undo.as_ref().is_some_and(|undo| {
            !CampaignDocument::load(&undo.campaign)
                .is_ok_and(|document| document.selections == undo.after)
        }) {
            self.selection_undo = None;
        }
        self.link_edits
            .disk_changed(|result| result.edits.on_disk());
        self.rosters_changed = self
            .copied_rosters
            .as_ref()
            .is_some_and(|edits| !edits.on_disk());
        self.registration_changed = self.rosters_changed
            || self
                .package_copy
                .as_ref()
                .is_some_and(|copy| !copy.on_disk())
            || self
                .copy_campaign_before
                .as_ref()
                .is_some_and(|(campaign, path, _, after)| !holds(&campaign.path, path, after));
    }

    #[allow(
        clippy::disallowed_methods,
        reason = "staged: calls `ResultOrder::issue`, `Footer::result` or `save_row_state` (design::results::STAGED_ALLOWS)"
    )]
    fn give_new_identity(&mut self) -> Result<(), String> {
        let index = self.selected.ok_or("Open the copy first.")?;
        let note = &self.loaded.as_ref().unwrap().notes[index];
        note.entry.as_ref().ok_or("Repair this entry first.")?;
        let root = note
            .owner
            .root()
            .ok_or("Built-in entries are read only.")?
            .to_path_buf();
        let before = note.text.clone();
        let id = crate::hermetic::new_uuid();
        let after = crate::content_workspace::new_note_identity(&before, &note.path, id)?;
        let updated = write_note(&root, &note.path, Some(&before), &after)?;
        self.result_orders.insert(
            OrderKey::Reader(ResultKind::Identity),
            design::ResultOrder::issue(),
        );
        self.identity_undo = Some(IdentityUndo {
            root: root.clone(),
            path: note.path.clone(),
            before,
            after: after.clone(),
        });
        let loaded = self.loaded.as_mut().unwrap();
        loaded.notes[index].entry = Some(updated);
        loaded.notes[index].text = after.clone();
        loaded.invalidate();
        loaded.refresh_diagnostics();
        self.draft = after;
        // The draft changed: a failed Save of the old words no longer owns it.
        self.draft_revision += 1;
        self.stale = true;
        self.content_changed = Some(root);
        Ok(())
    }

    fn undo_new_identity(&mut self) -> Result<(), String> {
        let undo = self.identity_undo.as_ref().ok_or("Nothing to undo.")?;
        write_note(&undo.root, &undo.path, Some(&undo.after), &undo.before)?;
        self.stale = true;
        self.content_changed = Some(undo.root.clone());
        if let Some(index) = self.selected {
            let note = &mut self.loaded.as_mut().unwrap().notes[index];
            if note.path == undo.path && note.owner.root() == Some(undo.root.as_path()) {
                note.text = undo.before.clone();
                note.entry = Entry::parse(&note.path, &note.text).ok();
                self.draft = note.text.clone();
                // The draft changed: a failed Save of the old words no longer owns it.
                self.draft_revision += 1;
            }
        }
        self.identity_undo = None;
        self.loaded.as_mut().unwrap().invalidate();
        self.loaded.as_mut().unwrap().refresh_diagnostics();
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn repairing_package_about(&self) -> bool {
        self.selected
            .and_then(|at| self.loaded.as_ref()?.notes.get(at))
            .is_some_and(|note| {
                note.is_manifest()
                    && note.error.is_some()
                    && self.source
                    && self.scope == Scope::Package(note.owner.id().unwrap())
                    && self.section == Section::All
            })
    }

    #[cfg(test)]
    pub(crate) fn selected_name(&self) -> Option<&str> {
        self.selected
            .and_then(|i| self.loaded.as_ref()?.notes.get(i))
            .map(Note::name)
    }

    pub fn report_error(&mut self, error: String) {
        self.error = Some(error);
    }
    #[cfg(test)]
    pub fn busy(&self) -> bool {
        self.pending.is_some()
    }

    /// The DM chose the file over the draft. The draft stays one step of undo away.
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: calls `ResultOrder::issue`, `Footer::result` or `save_row_state` (design::results::STAGED_ALLOWS)"
    )]
    fn reload_note_from_disk(&mut self, index: usize) {
        let Some(change) = self.disk_change.take() else {
            return;
        };
        // Whether this campaign uses the entry now, and whether the DM chose it, for Undo
        // remove to give back.
        let used = self.loaded.as_ref().is_some_and(|loaded| {
            loaded
                .notes
                .get(index)
                .is_some_and(|note| loaded.in_scope(note, Scope::Uses))
        });
        let chosen = self.loaded.as_ref().is_some_and(|loaded| {
            loaded
                .notes
                .get(index)
                .is_some_and(|note| loaded.chose(note))
        });
        let Some(note) = self.loaded.as_mut().and_then(|l| l.notes.get_mut(index)) else {
            return;
        };
        if let Some(path) = change.moved_to {
            note.path = path;
        }
        let draft = std::mem::take(&mut self.draft);
        self.draft_revision += 1;
        self.result_orders.insert(
            OrderKey::Reader(ResultKind::Discarded),
            design::ResultOrder::issue(),
        );
        self.discarded = Some((note.owner.id(), note.path.clone(), draft));
        if let Some(fault) = change.fault {
            // The note becomes the file as it is: unreadable, with its fault and no text.
            note.entry = None;
            note.error = Some(fault.cause().to_owned());
            note.fault = Some(fault);
            note.text = String::new();
            self.stale = true;
            if let Some(loaded) = &mut self.loaded {
                loaded.invalidate();
            }
            self.matches = None;
            self.scope_counts = None;
            return;
        }
        match change.text {
            Some(text) => {
                self.stale = true;
                match Entry::parse(&note.path, &text) {
                    Ok(entry) => {
                        note.entry = Some(entry);
                        note.error = None;
                    }
                    Err(error) => {
                        note.entry = None;
                        note.error = Some(error);
                    }
                }
                note.text = text.clone();
                self.draft = text;
            }
            None => {
                self.result_orders
                    .insert(OrderKey::Removed, design::ResultOrder::issue());
                self.discarded_note = Some(LetGo {
                    note: Note {
                        path: note.path.clone(),
                        text: note.text.clone(),
                        entry: note.entry.clone(),
                        error: note.error.clone(),
                        fault: note.fault.clone(),
                        owner: note.owner.clone(),
                    },
                    used,
                    chosen,
                    campaign: None,
                });
                self.draft = note.text.clone();
                self.stale = true;
                self.selected = None;
            }
        }
        if let Some(loaded) = &mut self.loaded {
            loaded.invalidate();
        }
        self.matches = None;
        self.scope_counts = None;
    }

    #[allow(
        clippy::disallowed_methods,
        reason = "staged: calls `ResultOrder::issue`, `Footer::result` or `save_row_state` (design::results::STAGED_ALLOWS)"
    )]
    fn open_entry(&mut self, index: usize) {
        if self.dirty() {
            return;
        }
        self.discarded_note = None;
        let Some(note) = self.loaded.as_ref().and_then(|l| l.notes.get(index)) else {
            return;
        };
        if self.selected != Some(index) {
            self.trail.leave(self.here());
        }
        if !note.is_manifest() && !self.loaded.as_ref().unwrap().in_scope(note, self.scope) {
            self.scope = Scope::All;
            self.result_orders.insert(
                OrderKey::Reader(ResultKind::ScopeNotice),
                design::ResultOrder::issue(),
            );
            self.scope_notice = Some(format!("Scope is now Everything, to show {}.", note.name()));
        }
        self.selected = Some(index);
        self.draft = note.text.clone();
        self.disk_change = None;
        self.section = if note.is_manifest() {
            Section::All
        } else {
            note.section()
        };
        self.search.clear();
        self.matches = None;
        self.source = false;
        self.error = None;
    }

    /// The open draft's entry name and owner, if it has changes.
    pub fn draft_place(&self) -> Option<(String, String)> {
        let note = self
            .loaded
            .as_ref()
            .and_then(|l| self.selected.and_then(|i| l.notes.get(i)))
            .filter(|n| n.text != self.draft)?;
        Some((
            note.name().to_owned(),
            match &note.owner {
                Owner::Package(module) => format!("the {} package", module.name),
                Owner::Campaign(campaign) => format!("the {} campaign", campaign.name),
                Owner::Builtin => "the built-in package".to_owned(),
            },
        ))
    }
    pub fn dirty(&self) -> bool {
        self.loaded
            .as_ref()
            .and_then(|l| self.selected.and_then(|i| l.notes.get(i)))
            .is_some_and(|n| n.text != self.draft)
    }
    pub fn form_open(&self) -> bool {
        self.form || self.new_form || self.confirm_unregister.is_some()
    }
    /// A draft in the campaign's own folder must block a campaign switch.
    pub fn campaign_draft(&self) -> bool {
        self.dirty()
            && self
                .loaded
                .as_ref()
                .and_then(|l| self.selected.and_then(|i| l.notes.get(i)))
                .is_some_and(|n| matches!(n.owner, Owner::Campaign(_)))
    }
    /// Remove's Undo (and the draft it keeps) belongs to the removed entry in the campaign that was
    /// open. The app tells the Library the open campaign on every frame, whichever page shows, so
    /// the Undo ends at the switch itself, even while the Library is hidden: a switch there and
    /// back never brings it back.
    pub fn note_campaign(&mut self, campaign: Option<Uuid>) {
        let Some(gone) = &mut self.discarded_note else {
            return;
        };
        if *gone.campaign.get_or_insert(campaign) != campaign {
            self.discarded_note = None;
            self.discarded = None;
        }
    }

    /// Remove's Undo also ends when any entry is selected, however the selection was made: the
    /// list, the About row, `reveal` (Lookup and other pages), Back or Forward, or `open_entry`.
    /// Its own Undo selects the entry it gives back after taking it.
    fn end_remove_on_selection(&mut self) {
        if self.discarded_note.is_some() && self.selected.is_some() {
            self.discarded_note = None;
            self.discarded = None;
        }
    }

    /// Show an entry where it lives. False if a draft is open: it stays in Lookup.
    pub fn reveal(&mut self, entry: &Entry) -> bool {
        if self.dirty() {
            return false;
        }
        self.reveal_row = true;
        let Some((index, in_scope)) = self.loaded.as_ref().and_then(|l| {
            let index = l
                .notes
                .iter()
                .position(|n| n.entry.as_ref().is_some_and(|e| e.id == entry.id))?;
            Some((index, l.in_scope(&l.notes[index], self.scope)))
        }) else {
            return false;
        };
        // Round 26 review: every way to open another entry leaves the open one on the trail,
        // before the scope, the search or the selection changes.
        if self.selected != Some(index) {
            self.trail.leave(self.here());
        }
        if !in_scope {
            self.scope = Scope::All;
        }
        self.search.clear();
        self.section = Section::for_entry(entry);
        self.source = false;
        self.matches = None;
        self.selected = Some(index);
        self.draft = self.loaded.as_ref().unwrap().notes[index].text.clone();
        true
    }
    pub fn edit_copy(&mut self, entry: Entry) {
        self.pending_edit = Some(entry);
    }
    /// Open a file for repair; with a missing link's reference and words, at that link.
    /// The repair guidance has one owner, its entry. When no load and no pending repair is in
    /// progress and the selected entry is not the owner, the guidance ends, however the
    /// selection was made: the list, the keyboard, search, `reveal`, Back or Forward, a link, or
    /// a repair whose file could not be opened.
    fn end_repair_on_owner_change(&mut self) {
        if self.pending.is_some() || self.stale || self.pending_repair.is_some() {
            return;
        }
        let here = self
            .selected
            .and_then(|i| self.loaded.as_ref()?.notes.get(i))
            .and_then(|note| Some(note.owner.root()?.join(&note.path)));
        if self
            .repair_link
            .as_ref()
            .is_some_and(|link| here.as_ref() != Some(&link.path))
        {
            self.repair_link = None;
        }
    }

    #[cfg(test)]
    pub(crate) fn repair_lookups(&self) -> usize {
        REPAIR_LOOKUPS.with(std::cell::Cell::get)
    }

    /// The broken-link scans and the compositions of their words, together: an unchanged frame
    /// does neither.
    #[cfg(test)]
    pub(crate) fn broken_line_scans(&self) -> usize {
        BROKEN_LINE_SCANS.with(std::cell::Cell::get)
            + BROKEN_WORD_COMPOSITIONS.with(std::cell::Cell::get)
    }
    pub fn repair_file_link(
        &mut self,
        path: PathBuf,
        link: Option<(splat::content::Reference, String)>,
    ) {
        self.repair_link = link.map(|(target, words)| RepairLink {
            path: path.clone(),
            target,
            words,
            found: design::memo::Memo::new(),
        });
        self.pending_repair = Some(path);
        self.stale = true;
    }
    /// The close dialog's Save is offered only when Save would accept the draft: its own check
    /// before the write, not only a parse.
    pub fn can_save_and_close(&self) -> bool {
        self.dirty()
            && self.disk_change.is_none()
            && self
                .selected
                .zip(self.loaded.as_ref())
                .is_some_and(|(i, loaded)| {
                    let note = &loaded.notes[i];
                    loaded
                        .draft_refusal(i, &Entry::parse(&note.path, &self.draft))
                        .is_none()
                })
    }
    pub fn save_for_close(&mut self) -> bool {
        if !self.can_save_and_close() {
            return false;
        }
        self.save_selected(false);
        !self.dirty()
    }

    pub fn refresh_destination(&mut self, root: &Path, _ctx: &egui::Context) {
        if self.dirty()
            && let Some(note) = self
                .selected
                .and_then(|i| self.loaded.as_ref()?.notes.get(i))
            && note.owner.root() == Some(root)
        {
            match read_disk(root, &note.path) {
                Err(fault) => {
                    self.disk_change = Some(DiskChange {
                        text: None,
                        moved_to: None,
                        fault: Some(fault),
                    });
                }
                Ok(text) if text.as_deref() != Some(&note.text) => {
                    let moved = (text.is_none()
                        && self
                            .loaded
                            .as_ref()
                            .unwrap()
                            .duplicates(self.selected.unwrap())
                            .is_empty())
                    .then(|| {
                        crate::content_workspace::note_id(&note.text)
                            .and_then(|id| moved_note(root, id, &note.path).ok().flatten())
                    })
                    .flatten();
                    self.disk_change = Some(DiskChange {
                        text: moved.as_ref().map(|(_, text)| text.clone()).or(text),
                        moved_to: moved.map(|(path, _)| path),
                        fault: None,
                    });
                }
                // The file reads as the draft's start again: a fault it had is over.
                Ok(_) => {
                    if self
                        .disk_change
                        .as_ref()
                        .is_some_and(|change| change.fault.is_some())
                    {
                        self.disk_change = None;
                    }
                }
            }
        }
        // Discard's and Remove's Undo belong to one file. A refresh ends them only when it is
        // about that file: the refreshed folder is its owner's, and the file on disk is no longer
        // what the Undo was made against (another write, or the deleted file is back). A write to
        // an unrelated entry, such as choosing a creature for the campaign, keeps them.
        let changed_here = |owner: &Owner, path: &str, expected: Option<&str>| {
            owner.root() == Some(root)
                && read_disk(root, path).map_or(true, |text| text.as_deref() != expected)
        };
        let gone_changed = self
            .discarded_note
            .as_ref()
            .is_some_and(|LetGo { note, .. }| changed_here(&note.owner, &note.path, None));
        if gone_changed {
            self.discarded_note = None;
        }
        let discard_changed = self.discarded.as_ref().is_some_and(|(owner, path, _)| {
            let loaded = self.loaded.as_ref().and_then(|loaded| {
                loaded
                    .notes
                    .iter()
                    .find(|note| note.owner.id() == *owner && &note.path == path)
            });
            match (loaded, &self.discarded_note) {
                (Some(note), _) => changed_here(&note.owner, path, Some(&note.text)),
                (None, Some(LetGo { note, .. })) if &note.path == path => false,
                // Its file is neither loaded nor removed by this Library: nothing owns it.
                (None, _) => true,
            }
        });
        if discard_changed || gone_changed {
            self.discarded = None;
        }
        self.stale = true;
    }
    fn invalidate(&mut self) {
        self.matches = None;
        self.scope_counts = None;
    }
    fn start_load(&mut self, ctx: &egui::Context, campaigns: &Campaigns) {
        self.reselect = self
            .loaded
            .as_ref()
            .and_then(|l| self.selected.and_then(|i| l.notes.get(i)))
            .map(|n| (n.owner.id(), n.path.clone()));
        self.load_key = Some(load_key(campaigns));
        let modules = campaigns.modules().to_vec();
        let campaign = campaigns.active_campaign();
        let ctx = ctx.clone();
        let (tx, rx) = mpsc::channel();
        self.pending = Some(rx);
        self.loaded = None;
        self.invalidate();
        self.selected = None;
        self.error = None;
        std::thread::spawn(move || {
            let _ = tx.send(load_all(&modules, campaign.as_ref()));
            ctx.request_repaint();
        });
    }
    /// The draft of note `i`, parsed. The same text keeps the same parse.
    fn parsed_draft(&mut self, i: usize) -> ParsedDraft {
        let path = &self.loaded.as_ref().unwrap().notes[i].path;
        if let Some((parsed_path, draft, parsed)) = &self.draft_parse
            && parsed_path == path
            && *draft == self.draft
        {
            return parsed.clone();
        }
        let parsed =
            design::memo::Revised::new(std::sync::Arc::new(Entry::parse(path, &self.draft)));
        self.draft_parse = Some((path.clone(), self.draft.clone(), parsed.clone()));
        parsed
    }

    fn draft_reason(&self) -> Option<&'static str> {
        if self.disk_change.is_some() {
            Some("Choose what to do with the changed file first.")
        } else if self.dirty() {
            Some(draft_words(
                self.selected
                    .and_then(|i| self.loaded.as_ref().map(|l| &l.notes[i].owner))
                    .is_some_and(|owner| matches!(owner, Owner::Campaign(_))),
            ))
        } else {
            None
        }
    }
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw egui calls that a design part replaces (design::results::STAGED_ALLOWS)"
    )]
    pub fn show(&mut self, ui: &mut egui::Ui, campaigns: &mut Campaigns) {
        // The New entry form's error ends with the form, however it closed.
        if !self.new_form {
            self.new_error = None;
        }
        self.end_repair_on_owner_change();
        self.end_remove_on_selection();
        ui.ctx().data_mut(|data| {
            data.insert_temp(egui::Id::new("reader_surface"), egui::Id::new("library"))
        });
        let key = load_key(campaigns);
        if self.load_key.is_none() && self.loaded.is_some() {
            self.load_key = Some(key.clone());
        }
        let changed = self.load_key.as_ref().is_some_and(|known| *known != key);
        if self.loaded.is_none() && self.pending.is_none() && self.error.is_none() {
            self.start_load(ui.ctx(), campaigns);
        } else if (changed || self.stale) && !self.dirty() && self.pending.is_none() {
            self.stale = false;
            self.start_load(ui.ctx(), campaigns);
        }
        if let Some(rx) = &self.pending {
            match rx.try_recv() {
                Ok(result) => {
                    self.pending = None;
                    match result {
                        Ok(l) => {
                            for module in campaigns.modules().to_vec() {
                                if let Some(name) = l
                                    .notes
                                    .iter()
                                    .filter(|n| {
                                        n.owner.id() == Some(module.id) && n.error.is_none()
                                    })
                                    .find_map(|n| {
                                        n.entry
                                            .as_ref()
                                            .filter(|entry| entry.kind == "package")
                                            .map(|entry| entry.name.clone())
                                    })
                                    && name != module.name
                                {
                                    let _ = campaigns.rename_module(module.id, name);
                                }
                            }
                            self.selected = self.reselect.as_ref().and_then(|(owner, path)| {
                                l.notes
                                    .iter()
                                    .position(|n| n.owner.id() == *owner && &n.path == path)
                            });
                            if let Some(i) = self.selected {
                                self.draft = l.notes[i].text.clone();
                                // The draft changed: a failed Save of the old words no longer owns it.
                                self.draft_revision += 1;
                            }
                            self.reveal_row |= self.selected.is_some();
                            self.focus_repair =
                                self.selected.is_some_and(|i| l.notes[i].error.is_some());
                            self.forget_changed_undos();
                            // An offer whose entry changed outside Splat, or whose names now name
                            // another entry, goes; it is checked again before any write too.
                            self.renamed_links
                                .disk_changed(|offer| l.offer_holds(offer));
                            self.loaded = Some(l);
                            self.invalidate();
                        }
                        Err(e) => self.error = Some(e),
                    }
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.pending = None;
                    self.error = Some("Library loading stopped. Reload to retry.".into());
                }
                Err(mpsc::TryRecvError::Empty) => {}
            }
        }
        if self.pending.is_none()
            && !self.stale
            && let Some(entry) = self.pending_edit.take()
        {
            if self.reveal(&entry) {
                self.source = true;
            } else {
                self.pending_edit = Some(entry);
            }
        }
        if self.pending.is_none()
            && !self.stale
            && !self.dirty()
            && let Some(path) = self.pending_repair.take()
            && let Some(index) = self.loaded.as_ref().and_then(|loaded| {
                loaded.notes.iter().position(|note| {
                    note.owner
                        .root()
                        .is_some_and(|root| root.join(&note.path) == path)
                })
            })
        {
            let note = &self.loaded.as_ref().unwrap().notes[index];
            self.search.clear();
            if note.is_manifest() {
                self.scope = Scope::Package(note.owner.id().unwrap());
                self.section = Section::All;
                self.unreadable_kind = None;
            } else {
                self.scope = Scope::All;
                self.section = Section::Unreadable;
                self.unreadable_kind = Some(note.unreadable_kind());
            }
            self.open_entry(index);
            self.source = true;
            self.focus_repair = true;
        }
        let dirty = self.dirty();
        let locked = self.draft_reason().or(self
            .pending
            .is_some()
            .then_some("Wait for the Library to load."));
        // Beside Lookup at the minimum window, everything here gets smaller.
        let compact = ui.available_width() < 900.0 || self.disk_change.is_some();
        let mut open_form = false;
        let mut reload = false;
        let total = self.loaded.as_ref().map_or(0, |loaded| {
            loaded
                .notes
                .iter()
                .filter(|n| loaded.in_scope(n, self.scope))
                .count()
        });
        let mut new_entry = false;
        let link_index = ui.ctx().data(|data| {
            data.get_temp::<std::sync::Arc<crate::lookup::ContentIndex>>(egui::Id::new(
                "reader_link_index",
            ))
        });
        let draft_reason = self.draft_reason();
        let mut open_broken = None;
        // An open draft's file that stopped reading has no reload yet: the header counts it now,
        // as a reload would, and names its short cause.
        let stopped_reading = self
            .disk_change
            .as_ref()
            .and_then(|change| change.fault.as_ref())
            .zip(
                self.selected
                    .and_then(|at| self.loaded.as_ref()?.notes.get(at)),
            )
            .map(|(fault, note)| {
                (
                    Loaded::file_key(note),
                    format!(
                        "{}: {}",
                        self.loaded.as_ref().unwrap().file_label(note),
                        fault.cause()
                    ),
                )
            });
        design::page_header(
            ui,
            "Library",
            (total > 0 && ui.available_width() >= design::READER_MEASURE)
                .then(|| design::count(total, "entry", "entries"))
                .as_deref(),
            |ui| {
                let no_place =
                    campaigns.active_campaign().is_none() && campaigns.modules().is_empty();
                if !(self.scope == Scope::Campaign && total == 0) {
                    new_entry = Action::secondary("New entry")
                        .icon(icons::ADD)
                        .blocked(
                            locked.or(no_place.then_some("Add a campaign or a package first.")),
                        )
                        .show(ui)
                        .clicked();
                }
                design::locked(ui, locked, |ui| {
                    let response = design::menu_button(ui, "Packages", false);
                    design::menu_popup(&*ui, &response).show(|ui| {
                        if Action::quiet("Reload from disk")
                            .icon(icons::REFRESH)
                            .show(ui)
                            .clicked()
                        {
                            reload = true;
                            ui.close();
                        }
                        if Action::quiet("Manage packages…")
                            .icon(icons::SETTINGS)
                            .show(ui)
                            .clicked()
                        {
                            open_form = true;
                            ui.close();
                        }
                    });
                });
                let broken = self
                    .loaded
                    .as_ref()
                    .zip(link_index.as_ref())
                    .map(|(loaded, index)| loaded.broken_link_notes(index))
                    .unwrap_or_default();
                if let Some(loaded) = &self.loaded
                    && (!loaded.diagnostics.is_empty()
                        || !broken.is_empty()
                        || stopped_reading.is_some())
                {
                    let word = design::count(
                        loaded
                            .files_to_fix(&broken, stopped_reading.as_ref().map(|(file, _)| file)),
                        "file needs fixing",
                        "files need fixing",
                    );
                    // The count has the tone of the most severe fault it counts: a file that
                    // cannot be read is an error.
                    let unreadable = loaded.unreadable_counted || stopped_reading.is_some();
                    let response = design::status_link(
                        ui,
                        Status {
                            icon: if unreadable {
                                icons::ERROR
                            } else {
                                icons::WARN
                            },
                            word: &word,
                            color: if unreadable {
                                colors().result.error
                            } else {
                                colors().result.warning
                            },
                        },
                    );
                    // Round 26 FB1: the menu is as wide as its content, up to the reader
                    // measure, and ends at the count's right edge, so it stays clear of the
                    // splitter between the list and the reader.
                    design::menu_popup(&*ui, &response)
                        .align(egui::RectAlign::BOTTOM_END)
                        .show(|ui| {
                            let link_label = |at: usize, count: usize| {
                                format!(
                                    "{}: {}",
                                    loaded.file_label(&loaded.notes[at]),
                                    design::count(count, "broken link", "broken links")
                                )
                            };
                            let font = Type::Small.font(ui);
                            let natural = |text: &str| {
                                ui.painter()
                                    .layout_no_wrap(text.to_owned(), font.clone(), colors().text)
                                    .size()
                                    .x
                            };
                            let width = loaded
                                .diagnostics
                                .iter()
                                .map(|message| natural(&message.text))
                                .chain(stopped_reading.as_ref().map(|(_, text)| natural(text)))
                                .chain(
                                    broken
                                        .iter()
                                        .map(|&(at, count)| natural(&link_label(at, count))),
                                )
                                .fold(design::XL, f32::max)
                                .min(design::READER_MEASURE)
                                .ceil();
                            ui.set_width(width);
                            egui::ScrollArea::vertical()
                                .max_height(design::XXL * 6.0)
                                .show(ui, |ui| {
                                    for message in loaded
                                        .diagnostics
                                        .iter()
                                        .map(|message| message.text.as_str())
                                        .chain(
                                            stopped_reading.as_ref().map(|(_, text)| text.as_str()),
                                        )
                                    {
                                        ui.add(
                                            egui::Label::new(
                                                Type::Small.rich(ui, message).color(colors().muted),
                                            )
                                            .wrap(),
                                        );
                                    }
                                    // As in World, each entry with a broken link opens from here.
                                    design::locked(ui, draft_reason, |ui| {
                                        for &(at, count) in &broken {
                                            if design::small_link(ui, &link_label(at, count))
                                                .clicked()
                                            {
                                                open_broken = Some(at);
                                                ui.close();
                                            }
                                        }
                                    });
                                });
                        });
                }
            },
        );
        if let Some(at) = open_broken {
            self.open_entry(at);
            self.reveal_row = true;
        }
        if new_entry {
            self.new_form = true;
            self.new_folder = kind_folder(&self.new_kind).into();
            self.focus_new_name = true;
            self.form = false;
        }
        if reload {
            self.start_load(ui.ctx(), campaigns);
        }
        if open_form {
            self.new_form = false;
            self.form = !self.form;
            self.focus_package_name = self.form;
        }
        if campaigns.active_campaign().is_none() {
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    Type::Small
                        .rich(
                            ui,
                            if campaigns.all().is_empty() {
                                "No campaign yet. Add a campaign or a package to create entries."
                            } else {
                                "No campaign open. The Library works without one."
                            },
                        )
                        .color(colors().muted),
                );
                if Action::primary(if campaigns.all().is_empty() {
                    "Add campaign"
                } else {
                    "Choose a campaign"
                })
                .density(Density::Read)
                .show(ui)
                .clicked()
                {
                    self.wants_settings = true;
                }
            });
        }
        design::gap(ui, design::Gap::XS);
        self.unregister_dialog(ui.ctx(), campaigns);
        // Round 26: the page's error is a reason in the pane's message footer, so it never moves
        // the list or the reader. It ends on the events that clear `error`.
        // The page's results sit in the pane's message footer (below the reader, the editor, the
        // empty reader or a form), so they never move the columns, the tabs, the file line, the
        // title or the body. Their presses are applied after the reader is drawn.
        // A link offer and a link result show only in their own entry's reader.
        // Each comes with the order its store issued at its action; this draw issues none.
        let open = self.open_link_owner();
        self.page_results = PageResults {
            link_offer: open
                .as_ref()
                .and_then(|owner| self.renamed_links.get(owner))
                .and_then(|shown| {
                    let offer = shown.value();
                    let count = self.link_rename_count(&offer.old, &offer.new);
                    (count > 0).then(|| {
                        let (words, action) = crate::content_workspace::link_offer_words(
                            &[&offer.old],
                            &offer.new,
                            count,
                        );
                        (shown.footer(|_| words), action)
                    })
                }),
            links: open
                .as_ref()
                .and_then(|owner| self.link_edits.get(owner))
                .map(|shown| shown.footer(|result| format!("Links now say “{}”.", result.new))),
            registration: self.package_copy.as_ref().map(|copy| {
                (
                    package_registered(&copy.module.name, self.copy_campaign_before.is_some()),
                    self.registration_changed
                        .then_some(crate::content_workspace::UNDO_ENDED),
                )
            }),
            use_copy: (self.package_copy.is_some()
                && campaigns.active_campaign().is_some()
                && self.copied_rosters.is_none())
            .then(|| self.draft_reason()),
            rosters: self.copied_rosters.as_ref().map(|_| {
                self.rosters_changed
                    .then_some(crate::content_workspace::UNDO_ENDED)
            }),
            // Round 25 FG4: the unregister result sits in the pane's message footer, so the scope,
            // the search and the list never move when it comes or goes.
            scope_notice: self.scope_notice.clone(),
            unregistered: self.unregistered.as_ref().map(|module| {
                format!(
                    "{} is unregistered. Its files stay in {}.",
                    module.name,
                    module.path.display()
                )
            }),
        };
        if self.pending.is_some() {
            ui.horizontal(|ui| {
                crate::design::spinner(ui);
                ui.label("Loading the Library…");
            });
            return;
        }

        let Some(loaded) = &self.loaded else {
            // Nothing loaded: there is no pane, so a failed load says so here.
            if let Some(error) = &self.error {
                design::message_with_code(ui, Tone::Error, &format!("Error: {error}"));
            }
            return;
        };
        // Each scope has its own counts. Search and sections stay within the scope.
        let scope = self.scope;
        let about_open = self.selected.is_some_and(|at| {
            loaded.notes[at].is_manifest()
                && matches!(scope, Scope::Package(id) if loaded.notes[at].owner.id() == Some(id))
        });
        if self
            .scope_counts
            .as_ref()
            .is_none_or(|(known, _)| *known != scope)
        {
            let notes = &loaded.notes;
            let counts = Section::GROUPS
                .iter()
                .flat_map(|(_, sections)| sections.iter())
                .map(|&section| {
                    (
                        section,
                        notes
                            .iter()
                            .filter(|n| {
                                loaded.in_scope(n, scope)
                                    && (section == Section::All || n.section() == section)
                            })
                            .count(),
                    )
                })
                .collect();
            self.scope_counts = Some((scope, counts));
        }
        let count_of = |section: Section, counts: &Vec<(Section, usize)>| {
            counts
                .iter()
                .find(|(s, _)| *s == section)
                .map_or(0, |(_, n)| *n)
        };
        let counts = self
            .scope_counts
            .as_ref()
            .map(|(_, counts)| counts.clone())
            .unwrap_or_default();
        let searching = !self.search.trim().is_empty();
        let empty_scope = scope == Scope::Campaign && count_of(Section::All, &counts) == 0;
        if empty_scope {
            ui.scope(|ui| {
                ui.set_max_width(166.0);
                Self::scope_menu(
                    ui,
                    compact,
                    campaigns,
                    &mut self.scope,
                    &mut self.section,
                    &mut self.search,
                );
            });
            // Round 26 review: the page's results and error sit in this pane's footer too, below
            // the form or the empty state.
            let results = ReaderResults {
                page: std::mem::take(&mut self.page_results),
                ..Default::default()
            };
            let (pressed, bottom) = form_footer(
                ui,
                "library_form_footer",
                &results,
                &self.result_orders,
                (
                    self.new_form.then_some(self.new_error.as_deref()).flatten(),
                    self.error.as_deref(),
                ),
            );
            self.page_pressed = pressed.page;
            if self.new_form {
                above_footer(ui, bottom, |ui| self.new_entry(ui, dirty, campaigns));
            } else {
                // The empty state is short; the pane already ends at the footer's top.
                let mut can_use = false;
                design::empty_state_with_body(
                    ui,
                    icons::LIBRARY,
                    "No entries of your own yet",
                    |ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.spacing_mut().item_spacing.x = 0.0;
                            ui.label(Type::Body.rich(ui, "Copy an ").color(colors().muted));
                            can_use = design::link(ui, "entry you can use").clicked();
                            ui.label(
                                Type::Body
                                    .rich(ui, ", or make a new one here.")
                                    .color(colors().muted),
                            );
                        });
                    },
                    |ui| {
                        if Action::primary("New entry")
                            .icon(icons::ADD)
                            .show(ui)
                            .clicked()
                        {
                            self.new_form = true;
                            self.new_folder = kind_folder(&self.new_kind).into();
                        }
                    },
                );
                if can_use {
                    self.scope = Scope::Uses;
                }
            }
            let pressed = std::mem::take(&mut self.page_pressed);
            self.apply_page_press(pressed, campaigns);
            return;
        }
        if !compact && self.disk_change.is_none() {
            design::Column::left("library_sections", 166.0, 150.0..=280.0)
                // The drag handle is on this edge, so the rows stop short of it.
                .frame(egui::Frame::new().inner_margin(egui::Margin {
                    right: design::S as i8,
                    ..Default::default()
                }))
                .show_inside(ui, |ui| {
                    Self::scope_menu(
                        ui,
                        compact,
                        campaigns,
                        &mut self.scope,
                        &mut self.section,
                        &mut self.search,
                    );
                    if let Scope::Package(package) = scope
                        && let Some((index, note)) = loaded
                            .notes
                            .iter()
                            .enumerate()
                            .find(|(_, n)| n.owner.id() == Some(package) && n.is_manifest())
                        && about_row(ui, &note.about_label(loaded.has_problem(note)), about_open)
                            .clicked()
                        && !dirty
                    {
                        // Round 26 SB1: a list click records the trail, as a link does.
                        if self.selected != Some(index) {
                            self.trail.leave(place(
                                loaded,
                                self.selected,
                                self.scope,
                                self.section,
                                &self.search,
                            ));
                        }
                        self.selected = Some(index);
                        self.draft = note.text.clone();
                        self.section = Section::All;
                        self.unreadable_kind = None;
                        self.source = false;
                    }
                    egui::ScrollArea::vertical()
                        .id_salt("section_list")
                        .show(ui, |ui| {
                            ui.spacing_mut().item_spacing.y = 0.0;
                            for &(group, sections) in Section::GROUPS {
                                // Keep the empty encounter section discoverable, but hide an unused fallback.
                                if sections
                                    .iter()
                                    .all(|section| count_of(*section, &counts) == 0)
                                {
                                    continue;
                                }
                                if !group.is_empty() {
                                    design::heading(
                                        ui,
                                        design::Heading::Group {
                                            title: group,
                                            count: sections
                                                .iter()
                                                .map(|section| count_of(*section, &counts))
                                                .sum(),
                                        },
                                    );
                                }
                                for &section in sections {
                                    let count = count_of(section, &counts);
                                    if count == 0 {
                                        continue;
                                    }
                                    if section == Section::Unreadable {
                                        for kind in Section::GROUPS
                                            .iter()
                                            .flat_map(|(_, sections)| sections.iter())
                                            .copied()
                                        {
                                            let count = loaded
                                                .notes
                                                .iter()
                                                .filter(|note| {
                                                    note.error.is_some()
                                                        && loaded.in_scope(note, scope)
                                                        && note.unreadable_kind() == kind
                                                })
                                                .count();
                                            if count == 0 {
                                                continue;
                                            }
                                            let label = if kind == Section::Unreadable {
                                                "Entries"
                                            } else {
                                                kind.label()
                                            };
                                            if section_row(
                                                ui,
                                                label,
                                                count,
                                                !about_open
                                                    && self.section == section
                                                    && self.unreadable_kind == Some(kind)
                                                    && !searching,
                                            )
                                            .clicked()
                                            {
                                                self.section = section;
                                                self.unreadable_kind = Some(kind);
                                                self.matches = None;
                                                self.search.clear();
                                            }
                                        }
                                        continue;
                                    }
                                    let selected =
                                        self.section == section && !searching && !about_open;
                                    let label = section.label();
                                    let response = section_row(ui, label, count, selected);
                                    if response.clicked() {
                                        if about_open && !dirty {
                                            self.selected = None;
                                            self.draft.clear();
                                        }
                                        self.section = section;
                                        self.unreadable_kind = None;
                                        self.search.clear();
                                    }
                                }
                            }
                        });
                });
        }
        // A query always searches the whole scope. Clearing it returns to the selected section.
        let query = self.search.trim().to_lowercase();
        let searching = !query.is_empty();
        let filtering =
            (self.section == Section::Creatures || searching) && !self.filter.is_empty();
        if self.matches_scope != Some(scope)
            || self
                .matches
                .as_ref()
                .is_some_and(|(_, prior, _)| *prior != query)
        {
            self.scope_notice = None;
        }
        if self.matches_scope != Some(scope)
            || self
                .matches
                .as_ref()
                .is_none_or(|(section, cached_query, _)| {
                    *section != self.section || *cached_query != query
                })
            || self.matched_filter != self.filter
        {
            self.matched_filter = self.filter.clone();
            self.reveal_row |= self.matches_scope != Some(scope);
            self.matches_scope = Some(scope);
            let parsed = Query::parse(&query);
            let typed = Typed::new(&parsed.words);
            let facets = loaded.facets();
            let mut found: Vec<(Rank, usize)> = loaded
                .notes
                .iter()
                .enumerate()
                .filter(|(i, n)| {
                    loaded.in_scope(n, scope)
                        && (!filtering
                            || facets[*i].as_ref().is_some_and(|f| self.filter.matches(f)))
                        && (!query.is_empty()
                            || self.section != Section::Unreadable
                            || self
                                .unreadable_kind
                                .is_none_or(|kind| n.unreadable_kind() == kind))
                })
                .filter_map(|(i, n)| {
                    n.search_rank(self.section, &parsed, &typed, facets[i].as_ref())
                        .map(|rank| (rank, i))
                })
                .collect();
            if !parsed.words.is_empty() {
                // Best first: the exact name, then a prefix, then a whole word, then each word in
                // any order. A path, section or fact match comes next, and a typo comes last. The
                // sort is stable, so the list's order stays inside each rank.
                found.sort_by_key(|&(rank, i)| {
                    (
                        rank,
                        crate::lookup::entry_origin(
                            loaded.notes[i].entry.as_ref(),
                            matches!(loaded.notes[i].owner, Owner::Campaign(_)),
                        ),
                    )
                });
            }
            self.typo_from = found
                .iter()
                .position(|(rank, _)| rank.typo)
                .unwrap_or(found.len());
            let found = found.into_iter().map(|(_, i)| i).collect();
            self.matches = Some((self.section, query.clone(), found));
        }
        let matches = &self.matches.as_ref().unwrap().2;
        // Keep the reader in sync with the visible list without dropping an unsaved draft.
        let mut select = None;
        // The whole catalogue has no first entry worth opening for the DM.
        let whole_catalogue = self.section == Section::All && query.is_empty();
        if !dirty
            && self
                .selected
                .is_none_or(|i| !matches.contains(&i) && !loaded.notes[i].is_manifest())
        {
            // A name found only with a typo is a guess. It does not open by itself.
            select = matches
                .first()
                .copied()
                .filter(|_| !whole_catalogue && self.typo_from > 0);
            if select.is_none() {
                self.selected = None;
                self.draft.clear();
            }
        }
        let selected = select.or(self.selected);
        let subtitles = loaded.subtitles();
        let link_index = ui.ctx().data(|data| {
            data.get_temp::<std::sync::Arc<crate::lookup::ContentIndex>>(egui::Id::new(
                "reader_link_index",
            ))
        });
        // A wide window's list gets room too, so that long names keep their second line.
        let min_list_width =
            (design::number_width(ui, "Can use", &Type::Strong.font(ui)) + design::XXL + design::L)
                * 2.0
                + design::S * 2.0
                + ui.spacing().item_spacing.x;
        let list_width = (ui.available_width() * 0.38).clamp(min_list_width, 420.0);
        let mut peek = None;
        let mut search_everything = false;
        // Beside Lookup at the minimum window, a draft needs the room more than the list.
        let hide_list =
            self.disk_change.is_some() || (ui.available_width() < 480.0 && (dirty || self.source));
        self.list_hidden = hide_list;
        if !hide_list {
            design::Column::left("library_entries", list_width, min_list_width..=560.0)
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                left: design::S as i8,
                right: design::S as i8,
                top: 0,
                bottom: 0,
            }))
            .show_inside(ui, |ui| {
                if compact {
                    let half = (ui.available_width() - ui.spacing().item_spacing.x) / 2.0;
                    ui.horizontal_top(|ui| {
                        ui.allocate_ui_with_layout(egui::vec2(half, 0.0), egui::Layout::top_down(egui::Align::Min), |ui| {
                            Self::scope_menu(ui, compact, campaigns, &mut self.scope, &mut self.section, &mut self.search);
                        });
                        let combo = egui::ComboBox::from_id_salt("library_section")
                            .icon(design::combo_icon).width(half).truncate()
                            .selected_text(match self.section { _ if about_open => "About", Section::All => "All", Section::Characters => "Characters", _ => self.section.label() })
                            .show_ui(ui, |ui| {
                                for &(_, sections) in Section::GROUPS {
                                    for &section in sections {
                                        if count_of(section, &counts) == 0 { continue; }
                                        let response = design::choice(ui, self.section == section && !about_open, section.label());
                                        design::named(ui, &response, &format!("Library section: {}", section.label()));
                                        if response.clicked() { if about_open && !dirty { self.selected = None; self.draft.clear(); } self.section = section; self.search.clear(); }
                                    }
                                }
                            });
                        design::named(ui, &combo.response, "Section");
                    });
                }
                let width = ui.available_width();
                // Clear empties the text; the search chooses nothing else to reset.
                let _ = LIBRARY_SEARCH.search(
                    ui,
                    SEARCH_NAME,
                    &mut self.search,
                    "Search by name or detail…",
                    width,
                );
                if compact
                    && let Scope::Package(package) = scope
                    && let Some((index, note)) = loaded.notes.iter().enumerate().find(|(_, n)| n.owner.id() == Some(package) && n.is_manifest())
                    && about_row(ui, &note.about_label(loaded.has_problem(note)), about_open).clicked() && !dirty
                {
                    if self.selected != Some(index) {
                        self.trail.leave(place(
                            loaded,
                            self.selected,
                            self.scope,
                            self.section,
                            &self.search,
                        ));
                    }
                    self.selected = Some(index);
                    self.draft = note.text.clone();
                    self.section = Section::All;
                    self.unreadable_kind = None;
                    self.source = false;
                }
                if self.section == Section::Creatures || searching {
                    let notes = &loaded.notes;
                    let facets = loaded.facets();
                    let changed = crate::creature_filter::filter_bar(ui, &mut self.filter, || {
                        notes
                            .iter()
                            .zip(facets)
                            .filter(|(note, _)| loaded.in_scope(note, scope))
                            .filter_map(|(_, facets)| facets.as_ref())
                            .collect()
                    });
                    if changed {
                        ui.ctx().request_repaint();
                    }
                }
                let headed = searching || filtering || (compact && self.section != Section::All);
                if headed {
                    design::heading(ui, design::Heading::Group { title: if searching { "Search results" } else if filtering { "Matches" } else { self.section.label() }, count: matches.len() });
                } else {
                    design::gap(ui, design::Gap::XS);
                }
                if searching && !matches.is_empty() && self.typo_from == 0 {
                    ui.add(
                        egui::Label::new(
                            Type::Body.rich(ui, CLOSE_NAMES)
                                .color(colors().muted),
                        )
                        .wrap(),
                    );
                }
                if matches.is_empty() {
                    // A heading owns the space after it.
                    if !headed {
                        design::gap(ui, design::Gap::S);
                    }
                    if let Scope::Package(id) = scope && !searching
                        && !loaded.notes.iter().any(|note| note.owner.id() == Some(id) && note.entry.as_ref().is_some_and(|entry| entry.kind != "package")) {
                        let name = campaigns.modules().iter().find(|m| m.id == id).map_or("this package", |m| m.name.as_str());
                        let mut create = false;
                        if self.new_form {
                            ui.label(Type::Title.rich(ui, icons::LIBRARY).color(colors().muted));
                            ui.label(Type::Title.rich(ui, format!("No entries in {name} yet")).color(colors().text));
                        } else {
                            design::empty_state(ui, icons::LIBRARY, &format!("No entries in {name} yet"), "Create the first entry.", |ui| {
                                create = Action::secondary("New entry").icon(icons::ADD).show(ui).clicked();
                            });
                        }
                        if create { self.new_form = true; self.focus_new_name = true; self.new_kind = "creature".into(); self.new_folder = "Creatures".into(); }
                    } else {
                    let filtered = !self.filter.is_empty() && (searching || self.section == Section::Creatures);
                    // Where a wider scope can find more, the sentence names this scope, and quiet
                    // `Search Everything` below it keeps the words and the filters.
                    let wider = (filtered || searching) && scope != Scope::All;
                    let text = if filtered && wider {
                        format!("No creatures match these filters in {}. Remove a filter, or search Everything.", scope_name(campaigns, scope))
                    } else if filtered {
                        "No creatures match these filters. Remove a filter.".to_owned()
                    } else if scope == Scope::Campaign && !searching {
                        "No campaign entries yet. Copy an entry here with Copy to…, or create one with New entry.".to_owned()
                    } else if searching && wider {
                        format!("No entries match in {}. Try fewer words, or search Everything.", scope_name(campaigns, scope))
                    } else if searching {
                        "No entries match. Try fewer words.".to_owned()
                    } else {
                        "No entries in this section.".to_owned()
                    };
                    // At the rows' text edge, so the action's hanging padding stays clear of the
                    // splitter at the column's left edge.
                    egui::Frame::new()
                        .inner_margin(egui::Margin {
                            left: design::S as i8,
                            ..Default::default()
                        })
                        .show(ui, |ui| {
                            ui.add(
                                egui::Label::new(Type::Body.rich(ui, &text).color(colors().muted))
                                    .wrap(),
                            );
                            if wider {
                                search_everything = Action::quiet(SEARCH_EVERYTHING)
                                    .density(Density::Read)
                                    .line_start()
                                    .show(ui)
                                    .clicked();
                            }
                        });
                    }
                }
                ui.spacing_mut().item_spacing.y = 0.0;
                let mut list = egui::ScrollArea::vertical()
                    .id_salt(("entry_list", self.section.label(), &query, scope))
                    .auto_shrink([false, false]);
                if std::mem::take(&mut self.reveal_row)
                    && let Some(row) =
                        selected.and_then(|selected| matches.iter().position(|i| *i == selected))
                {
                    list = list.vertical_scroll_offset(
                        (row as f32 - 2.0).max(0.0) * ENTRY_ROW_HEIGHT,
                    );
                }
                list.show_rows(ui, ENTRY_ROW_HEIGHT, matches.len(), |ui, range| {
                        for row in range {
                            let i = matches[row];
                            let note = &loaded.notes[i];
                            let line = &subtitles[i];
                            let subtitle = match (&note.owner, scope) {
                                (Owner::Campaign(_), Scope::Campaign) => line.strip_suffix(" · Campaign").unwrap_or(line),
                                (Owner::Package(module), Scope::Package(id)) if module.id == id => line.strip_suffix(&format!(" · {}", module.name)).unwrap_or(line),
                                _ => line,
                            };
                            let broken = note.entry.as_ref().zip(link_index.as_ref()).filter(|_| note.owner != Owner::Builtin).map_or(0, |(entry, index)| index.broken_link_count(&entry.body, entry.id));
                            let faults = note.entry.as_ref().map_or(0, |entry| crate::roll_table::fault_count(&entry.body));
                            let subtitle = if faults > 0 {
                                format!("{} {} · {subtitle}", icons::WARN, design::count(faults, "table fault", "table faults"))
                            } else if broken > 0 {
                                // One fault gets one mark: when the file's every counted problem
                                // is a missing link, the links' count is its only cause.
                                let needs_fixing = format!("{} {NEEDS_FIXING} · ", icons::WARN);
                                let rest = subtitle
                                    .strip_prefix(&needs_fixing)
                                    .filter(|_| loaded.link_faults_only.contains(&Loaded::file_key(note)))
                                    .unwrap_or(subtitle);
                                format!("{} {} · {rest}", icons::WARN, design::count(broken, "broken link", "broken links"))
                            } else { subtitle.to_owned() };
                            let response = ui
                                .push_id(i, |ui| {
                                    entry_row(
                                        ui,
                                        note.name(),
                                        &subtitle,
                                        selected == Some(i),
                                    )
                                })
                                .inner;
                            if response
                                .on_hover_text(format!(
                                    "{}\n{}\n{}",
                                    note.name(),
                                    subtitle,
                                    note.path
                                ))
                                .clicked()
                            {
                                if dirty {
                                    peek = note.entry.as_ref().map(|e| e.id);
                                } else {
                                    select = Some(i);
                                }
                            }
                        }
                    });
            });
        }
        if peek.is_some() {
            self.peek = peek;
        }
        if search_everything {
            // The same words and filters, in the widest scope.
            self.scope = Scope::All;
            self.section = Section::All;
        }
        if let Some(i) = select {
            // Round 26 SF1, SB1: a list click leaves the open entry as a link does, so Back
            // returns to it and Forward ends.
            if self.selected != Some(i) {
                self.trail.leave(self.here());
            }
            self.selected = Some(i);
            self.draft = self.loaded.as_ref().unwrap().notes[i].text.clone();
            self.error = None;
            self.source = false;
        }
        let subtitle = self
            .selected
            .map(|i| subtitles[i].clone())
            .unwrap_or_default();
        egui::CentralPanel::default()
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                left: if self.disk_change.is_some() {
                    0
                } else {
                    design::L as i8
                },
                right: 0,
                top: 0,
                bottom: 0,
            }))
            .show_inside(ui, |ui| {
                let mut form_bottom = f32::INFINITY;
                if self.form || self.new_form {
                    // A form has no reader: the page's results sit in the pane's message footer,
                    // below the form, so they never move it (round 26).
                    let results = ReaderResults {
                        page: std::mem::take(&mut self.page_results),
                        ..Default::default()
                    };
                    let (pressed, bottom) = form_footer(
                        ui,
                        "library_form_footer",
                        &results,
                        &self.result_orders,
                        (
                            self.new_form.then_some(self.new_error.as_deref()).flatten(),
                            self.error.as_deref(),
                        ),
                    );
                    self.page_pressed = pressed.page;
                    form_bottom = bottom;
                }
                if self.form {
                    above_footer(ui, form_bottom, |ui| self.package_form(ui, campaigns));
                } else if self.new_form {
                    above_footer(ui, form_bottom, |ui| self.new_entry(ui, dirty, campaigns));
                } else {
                    self.editor(ui, &subtitle, campaigns);
                }
            });
        let pressed = std::mem::take(&mut self.page_pressed);
        self.apply_page_press(pressed, campaigns);
    }

    /// The open entry's identity and root: what owns a link offer and its result.
    fn open_entry_key(&self) -> Option<(Uuid, PathBuf)> {
        let note = self.loaded.as_ref()?.notes.get(self.selected?)?;
        Some((note.entry.as_ref()?.id, note.owner.root()?.to_path_buf()))
    }

    /// How many links the offer after a rename would change. The loaded notes keep the count
    /// until they change, so an unchanged frame reads no note.
    fn link_rename_count(&self, old: &str, new: &str) -> usize {
        self.loaded
            .as_ref()
            .map_or(0, |loaded| loaded.rename_link_count(old, new))
    }

    /// The page results' presses from the reader: the link update and its Undo, and the
    /// registration's Undo, its source change and that change's Undo.
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: calls `ResultOrder::issue`, `Footer::result` or `save_row_state` (design::results::STAGED_ALLOWS)"
    )]
    fn apply_page_press(&mut self, pressed: PagePress, campaigns: &mut Campaigns) {
        if pressed.unregistered
            && let Some(module) = self.unregistered.clone()
        {
            let result = local_module(&module.path).and_then(|found| {
                if found.id != module.id {
                    return Err("That folder now holds a different package.".into());
                }
                campaigns.restore_module(module)
            });
            match result {
                Ok(()) => {
                    self.unregistered = None;
                    self.stale = true;
                }
                Err(error) => self.error = Some(error),
            }
        }
        let open = self.open_link_owner();
        if pressed.link_offer
            && let Some(loaded) = &self.loaded
            && let Some(offer) = open
                .as_ref()
                .and_then(|owner| self.renamed_links.use_press(owner))
        {
            // The offer is checked again before the write: its entry still has the new name, and
            // neither name names another entry. A stale offer goes and writes nothing.
            // The offer is checked against the loaded notes first, then, directly before the
            // write (after the scan and after each linking file is checked against the disk),
            // against the entry's own file on disk. A stale offer goes and writes nothing.
            if !loaded.offer_holds(&offer) {
                self.error = Some(LINKS_NOT_CHANGED.into());
            } else {
                let files: Vec<_> = loaded
                    .notes
                    .iter()
                    .filter_map(|note| {
                        let root = note.owner.root()?;
                        let (after, changed) =
                            crate::world::rename_note_links(&note.text, &offer.old, &offer.new);
                        (changed > 0).then(|| {
                            (
                                root.to_path_buf(),
                                note.path.clone(),
                                note.text.clone(),
                                after,
                            )
                        })
                    })
                    .collect();
                let written = crate::content_workspace::ContentEdits::write_checked(files, || {
                    if loaded.offer_holds_on_disk(&offer) {
                        Ok(())
                    } else {
                        Err(LINKS_NOT_CHANGED.into())
                    }
                });
                let LinkOffer {
                    owner,
                    source,
                    old,
                    new,
                    ..
                } = offer;
                match written {
                    Ok(edits) => {
                        // The owner's new update ends its earlier result.
                        self.link_edits.record(LinkResult {
                            owner,
                            source,
                            edits,
                            old,
                            new,
                        });
                        self.stale = true;
                        self.content_changed =
                            campaigns.active_campaign().map(|campaign| campaign.path);
                    }
                    Err(error) => self.error = Some(error),
                }
            }
        }
        if pressed.links
            && let Some(owner) = &open
        {
            // Undo restores only the files this update wrote, and only while they hold what it
            // wrote, so another entry's link update stays. A failed Undo keeps the result and
            // its Undo.
            match self.link_edits.undo(owner, |result| result.edits.undo()) {
                None => {}
                Some(Ok((result, ()))) => {
                    // The links say the old name again: the rename's offer comes back in the
                    // result's place, with a new order. The reload checks it again, as every
                    // offer. The offer keeps the source version it was made from: its write
                    // checks it again.
                    let LinkResult {
                        owner,
                        source,
                        old,
                        new,
                        ..
                    } = result;
                    self.renamed_links.record(LinkOffer {
                        owner,
                        source,
                        old,
                        new,
                    });
                    self.stale = true;
                    self.content_changed =
                        campaigns.active_campaign().map(|campaign| campaign.path);
                }
                Some(Err(error)) => self.error = Some(error),
            }
        }
        if pressed.registration
            && let Err(error) = self.undo_package_registration(campaigns)
        {
            self.error = Some(error);
        }
        if pressed.use_copy
            && let Some(copy) = &self.package_copy
            && let Some(campaign) = campaigns.active_campaign()
            && self.copied_rosters.is_none()
        {
            match copy.use_in_rosters(&campaign) {
                Ok(edits) => {
                    self.result_orders.insert(
                        OrderKey::Reader(ResultKind::Rosters),
                        design::ResultOrder::issue(),
                    );
                    self.copied_rosters = Some(edits);
                    self.rosters_changed = false;
                    self.content_changed = Some(campaign.path);
                    self.stale = true;
                }
                Err(error) => self.error = Some(error),
            }
        }
        if pressed.rosters
            && let Some(edits) = &self.copied_rosters
        {
            match edits.undo() {
                Ok(()) => {
                    self.copied_rosters = None;
                    self.content_changed =
                        campaigns.active_campaign().map(|campaign| campaign.path);
                    self.stale = true;
                }
                Err(error) => self.error = Some(error),
            }
        }
    }

    /// What the catalogue shows. Choosing here never changes the active campaign.
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw egui calls that a design part replaces (design::results::STAGED_ALLOWS)"
    )]
    fn scope_menu(
        ui: &mut egui::Ui,
        compact: bool,
        campaigns: &Campaigns,
        current: &mut Scope,
        section: &mut Section,
        _search: &mut String,
    ) {
        let active = campaigns.active_campaign();
        let label = |scope: Scope| scope_name(campaigns, scope);
        let mut scope = if active.is_none() && matches!(*current, Scope::Uses | Scope::Campaign) {
            Scope::All
        } else {
            *current
        };
        let before = scope;
        // This control moves between columns as the reader narrows. Keep an open
        // menu attached to it when the window changes size.
        ui.scope_builder(egui::UiBuilder::new().id("library_scope_host"), |ui| {
            let width =
                ui.available_width()
                    .min(if compact { ui.available_width() } else { 166.0 });
            let selected = match scope {
                Scope::Uses => "Can use".to_owned(),
                Scope::Builtin => "D&D 2024".to_owned(),
                Scope::Campaign => "Campaign entries".to_owned(),
                _ => label(scope),
            };
            let budget = width - design::XXL - design::L;
            let fitted = if matches!(scope, Scope::Package(_)) {
                scope_label(ui, &selected, budget)
            } else {
                selected
            };
            // Wrapped names make the menu taller, so it may use the window's height below the
            // control before it scrolls.
            let height = (ui.ctx().content_rect().bottom()
                - ui.next_widget_position().y
                - design::XXL * 2.0)
                .max(design::XXL * 4.0);
            let combo = egui::ComboBox::from_id_salt("library_scope")
                .icon(design::combo_icon)
                .width(width)
                .height(height)
                .wrap()
                .selected_text(Type::Strong.rich(ui, fitted))
                .show_ui(ui, |ui| {
                    // The popup is as wide as the control, so it covers the counts beside it and
                    // stays in the scope's column, clear of the splitter. Its content is the
                    // control's width less the popup's frame. A long package name wraps.
                    let frame = ui.style().spacing.menu_margin.sum().x
                        + 2.0 * ui.style().visuals.window_stroke.width;
                    let inner = (width - frame).max(0.0);
                    ui.set_min_width(inner);
                    ui.set_max_width(inner);
                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
                    let mut choose = |ui: &mut egui::Ui, choice: Scope| {
                        if design::extras::choice_wrapped(ui, scope == choice, &label(choice), None)
                            .clicked()
                        {
                            scope = choice;
                        }
                    };
                    if active.is_some() {
                        choose(ui, Scope::Uses);
                    }
                    choose(ui, Scope::All);
                    ui.separator();
                    choose(ui, Scope::Builtin);
                    for m in campaigns.modules() {
                        choose(ui, Scope::Package(m.id));
                    }
                    if active.is_some() {
                        choose(ui, Scope::Campaign);
                    }
                });
            combo.response.clone().on_hover_text(label(scope));
            design::named(
                ui,
                &combo.response,
                &format!("What the Library shows: {}", label(scope)),
            );
        });
        *current = scope;
        if scope != before {
            // The selected entry can stay if it is in this scope. A draft always stays.
            *section = Section::All;
        }
    }

    /// Show the catalogue limited to this campaign's own entries.
    pub fn show_campaign_entries(&mut self) {
        self.scope = Scope::Campaign;
        self.search.clear();
        self.section = Section::All;
    }

    #[allow(
        clippy::disallowed_methods,
        reason = "staged: calls `ResultOrder::issue`, `Footer::result` or `save_row_state` (design::results::STAGED_ALLOWS)"
    )]
    fn unregister_dialog(&mut self, ctx: &egui::Context, campaigns: &mut Campaigns) {
        let Some(module) = self.confirm_unregister.clone() else {
            return;
        };
        let count = self
            .loaded
            .as_ref()
            .map_or(0, |l| unregister_count(&l.notes, module.id));
        let mut keep = ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape));
        let mut remove = false;
        let title = format!("Unregister {}?", module.name);
        design::dialog(ctx, "unregister_package", &title, |ui| {
            ui.add(
                egui::Label::new(format!(
                    "{} leave Splat’s lists, and any campaign that selects them can no longer use them.", design::count(count, "entry", "entries")
                ))
                .wrap(),
            );
            if let Some(campaign) = campaigns.active_campaign()
                && let Ok(catalogue) =
                    crate::content_workspace::Catalogue::load(campaigns.modules(), Some(&campaign))
            {
                let entry_ids: Vec<_> = catalogue
                    .entries
                    .values()
                    .filter(|entry| {
                        entry.owner == module.id && catalogue.selectable(entry.entry.id)
                    })
                    .map(|entry| crate::content_workspace::EntryRef::id(entry.entry.id))
                    .collect();
                let selection = crate::content_workspace::PackageSelection {
                    name: Some(module.name.clone()),
                    package: crate::content_workspace::EntryRef::id(module.id),
                    selection: crate::content_workspace::Selection::Selected { entries: entry_ids },
                };
                let users = selection.campaign_users(&catalogue);
                if !users.is_empty() {
                    let mut names = Vec::new();
                    for (id, _) in &users {
                        if let Some(user) = catalogue.entries.get(id) {
                            for reference in
                                content::required_references(&user.entry).unwrap_or_default()
                            {
                                if let Ok(id) = catalogue.resolve(&reference)
                                    && let Some(entry) = catalogue.entries.get(&id)
                                    && entry.owner == module.id
                                {
                                    names.push(entry.entry.name.clone());
                                }
                            }
                        }
                    }
                    names.sort();
                    names.dedup();
                    let users: Vec<_> = users.into_iter().map(|(_, name)| name).collect();
                    ui.label(format!(
                        "{} uses {}, in {}.",
                        campaign.name,
                        name_list(&names),
                        name_list(&users)
                    ));
                }
            }
            design::help(ui, &format!("The files stay in {}.", module.path.display()));
            design::gap(ui, design::Gap::S);
            design::dialog_actions(
                ui,
                |ui| {
                    remove = Action::destructive("Unregister").show(ui).clicked();
                },
                |ui| {
                    keep |= Action::secondary("Keep the package").show(ui).clicked();
                },
            );
        });
        if keep {
            self.confirm_unregister = None;
        }
        if remove {
            self.confirm_unregister = None;
            match campaigns.unregister_module(module.id) {
                Ok(()) => {
                    self.result_orders.insert(
                        OrderKey::Reader(ResultKind::Unregistered),
                        design::ResultOrder::issue(),
                    );
                    self.unregistered = Some(module);
                    self.scope = Scope::default();
                    self.stale = true;
                }
                Err(e) => self.error = Some(e),
            }
        }
    }

    /// Registering a package is a form in the page, not a window over it.
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw egui calls that a design part replaces (design::results::STAGED_ALLOWS)"
    )]
    fn package_form(&mut self, ui: &mut egui::Ui, campaigns: &mut Campaigns) {
        let c = colors();
        let locked = self.draft_reason().or(self
            .pending
            .is_some()
            .then_some("Wait for the Library to load."));
        if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
            self.form = false;
        }
        egui::ScrollArea::vertical()
            .id_salt("package_form")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.set_max_width(ui.available_width().min(design::FORM_WIDTH));
                design::gap(ui, design::Gap::L);
                ui.horizontal(|ui| {
                    ui.label(
                        Type::Heading
                            .rich(ui, "Manage packages")
                            .color(colors().text),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if Action::quiet("Close")
                            .name("Close the package form")
                            .show(ui)
                            .clicked()
                        {
                            self.form = false;
                        }
                    });
                });
                form_heading(ui, "Add a package folder");
                if self.copied_package.as_ref().is_some_and(|copy| {
                    Path::new(self.path.trim()).canonicalize().ok().as_ref() != Some(&copy.path)
                }) {
                    self.copied_package = None;
                    self.copy_name_open = false;
                }
                let copy_reason = self
                    .copied_package
                    .is_some()
                    .then_some("This folder is a copy.");
                let add_reason = locked
                    .or(copy_reason)
                    .or(self
                        .path
                        .trim()
                        .is_empty()
                        .then_some("Choose a folder first."));
                // Round 26 FB6: Add folder follows the field on its line; the reason is below.
                let add = ui
                    .horizontal(|ui| {
                    // As tall as the buttons, so the field and its actions share one centre.
                    ui.set_min_height(Density::Scan.control());
                    design::text_input(
                        ui,
                        "Package folder",
                        &mut self.path,
                        "Choose a folder",
                        (ui.available_width()
                            - Action::secondary("Browse…").natural_width(ui)
                            - Action::secondary("Add folder").natural_width(ui)
                            - 2.0 * ui.spacing().item_spacing.x)
                            .max(design::XXL),
                    );
                    if Action::secondary("Browse…")
                        .blocked(locked)
                        .show(ui)
                        .clicked()
                        && let Some(path) = crate::hermetic::pick_folder()
                    {
                        self.path = path.display().to_string();
                    }
                    Action::secondary("Add folder")
                        .blocked(add_reason)
                        .show(ui)
                        .clicked()
                    })
                    .inner;
                {
                    if add {
                        match local_module(Path::new(self.path.trim())).and_then(|module| {
                            if let Some(original) = campaigns
                                .modules()
                                .iter()
                                .find(|m| m.id == module.id && m.path != module.path)
                            {
                                self.copy_name = format!("{} copy", original.name);
                                self.copy_name_open = false;
                                self.copy_not_used = campaigns
                                    .active_campaign()
                                    .and_then(|campaign| CampaignDocument::load(&campaign).ok())
                                    .is_some_and(|document| {
                                        document.selections.iter().any(|selection| {
                                            selection.package.id == Some(module.id)
                                        })
                                    });
                                self.copied_package = Some(module);
                                self.error = None;
                                return Ok(None);
                            }
                            campaigns.register_module(module.clone())?;
                            Ok(Some(module))
                        }) {
                            Ok(Some(module)) => {
                                self.scope = Scope::Package(module.id);
                                self.form = false;
                                self.stale = true;
                            }
                            Ok(None) => {}
                            Err(error) => self.error = Some(error),
                        }
                    }
                }
                // Round 27 FA7, FD6: while the copy warning shows below, it is the reason; the
                // kept line stays empty, and Add folder's hover and accessible description keep
                // `This folder is a copy.`
                crate::campaigns::creation_reason(
                    ui,
                    add_reason.filter(|_| locked.is_some() || copy_reason.is_none()),
                );

                if let Some(copy) = self.copied_package.clone() {
                    if Path::new(self.path.trim()).canonicalize().ok().as_ref() != Some(&copy.path)
                    {
                        self.copied_package = None;
                    } else {
                        let name = campaigns
                            .modules()
                            .iter()
                            .find(|m| m.id == copy.id)
                            .map_or(copy.name.as_str(), |m| m.name.as_str());
                        let warning = format!("This folder is a copy of {name}.");
                        let reason = locked
                            .or((self.copy_name_open && self.copy_name.trim().is_empty())
                                .then_some("Type a name first."));
                        let register = if self.copy_name_open {
                            design::message(ui, Tone::Warning, &warning);
                            form_heading(ui, "Register a copy");
                            // Round 27 FD2: the inline creation form's one order. The choice
                            // first, then the name with `Register` at the end of its line, then
                            // one kept line: quiet Cancel at its start, the reason after it. The
                            // commit keeps one word, so typing moves nothing.
                            if campaigns.active_campaign().is_some() {
                                use_checkbox(ui, &mut self.copy_not_used, |chosen| {
                                    if chosen {
                                        "Uses every entry in this package, including future entries."
                                    } else {
                                        "This campaign keeps the original. The copy is registered but not chosen."
                                    }
                                });
                            }
                            // The commit of an inline creation form has one weight: primary, as
                            // `Create package` is, unless a run owns the primary.
                            let register_action = || {
                                if self.run_open {
                                    Action::secondary(REGISTER_COPY)
                                } else {
                                    Action::primary(REGISTER_COPY)
                                }
                            };
                            let register_width = register_action().natural_width(ui);
                            let mut clicked = false;
                            ui.horizontal_top(|ui| {
                                let width = ui.available_width()
                                    - register_width
                                    - ui.spacing().item_spacing.x;
                                ui.allocate_ui_with_layout(
                                    egui::vec2(width, 0.0),
                                    egui::Layout::top_down(egui::Align::Min),
                                    |ui| {
                                        design::text_field(
                                            ui,
                                            "New package name",
                                            &mut self.copy_name,
                                            None,
                                            None,
                                        );
                                    },
                                );
                                ui.vertical(|ui| {
                                    ui.add_space(design::L + design::XS);
                                    clicked =
                                        register_action().blocked(reason).show(ui).clicked();
                                });
                            });
                            ui.horizontal(|ui| {
                                // As tall as Cancel, kept when the reason is empty.
                                ui.set_min_height(Density::Read.control());
                                if Action::quiet("Cancel")
                                    .density(Density::Read)
                                    .line_start()
                                    .show(ui)
                                    .clicked()
                                {
                                    self.copy_name_open = false;
                                }
                                design::gap(ui, design::Gap::S); // With the line's item gap (`S`): `L`.
                                design::blocked_reason(ui, reason);
                            });
                            clicked
                        } else {
                            design::locked(ui, reason, |ui| {
                                design::message_with_action(
                                    ui,
                                    Tone::Warning,
                                    &warning,
                                    "Register as a new package",
                                )
                            })
                        };
                        if register {
                            if self.copy_name_open {
                                if let Err(error) = self.register_copy(campaigns) {
                                    self.error = Some(error);
                                }
                            } else {
                                self.copy_name_open = true;
                            }
                        }
                        design::quiet_section(ui, "Technical details", |ui| {
                            ui.label(copy.id.to_string());
                        });
                    }
                }

                if !self.copy_name_open {
                    form_heading(ui, "New package");
                    let response = design::text_field(ui, "Name", &mut self.name, None, None);
                    if std::mem::take(&mut self.focus_package_name) {
                        response.request_focus();
                    }
                    // Round 26 FB6: the choice comes before the last field, so Create package can
                    // follow that field on its line.
                    if campaigns.active_campaign().is_some() {
                        use_checkbox(ui, &mut self.package_not_used, |_| {
                            "Uses every entry in this package, including future entries."
                        });
                    }
                    let reason =
                        locked.or(self.name.trim().is_empty().then_some("Type a name first."));
                    let create_action = || {
                        if self.run_open {
                            Action::secondary("Create package")
                        } else {
                            Action::primary("Create package")
                        }
                    };
                    let create_width = create_action().natural_width(ui);
                    let mut create = false;
                    let parent = self.package_path.get_or_insert_with(|| {
                        campaigns
                            .modules()
                            .last()
                            .and_then(|module| module.path.parent())
                            .map(Path::to_path_buf)
                            .or_else(|| {
                                campaigns
                                    .package_folder("new_package")
                                    .ok()
                                    .and_then(|p| p.parent().map(Path::to_path_buf))
                            })
                            .unwrap_or_default()
                    });
                    ui.horizontal_top(|ui| {
                        let width = ui.available_width()
                            - Action::secondary("Browse…").natural_width(ui)
                            - create_width
                            - 2.0 * ui.spacing().item_spacing.x;
                        ui.allocate_ui_with_layout(
                            egui::vec2(width, 0.0),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                let mut path = parent.display().to_string();
                                if design::text_field(ui, "Folder", &mut path, None, None).changed()
                                {
                                    *parent = PathBuf::from(path);
                                    self.package_folder_changed = true;
                                }
                            },
                        );
                        ui.vertical(|ui| {
                            ui.add_space(design::L + design::XS);
                            if Action::secondary("Browse…")
                                .name("Choose new package folder")
                                .show(ui)
                                .clicked()
                                && let Some(chosen) = crate::hermetic::pick_folder()
                            {
                                *parent = chosen;
                                self.package_folder_changed = true;
                            }
                        });
                        ui.vertical(|ui| {
                            ui.add_space(design::L + design::XS);
                            create = create_action().blocked(reason).show(ui).clicked();
                        });
                    });
                    let namespace: String = self
                        .name
                        .trim()
                        .to_lowercase()
                        .chars()
                        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
                        .collect();
                    let root = self
                        .package_path
                        .as_ref()
                        .map(|parent| Ok(parent.join(&namespace)))
                        .unwrap_or_else(|| {
                            campaigns
                                .modules()
                                .last()
                                .and_then(|module| module.path.parent())
                                .map(|parent| Ok(parent.join(&namespace)))
                                .unwrap_or_else(|| campaigns.package_folder(&namespace))
                        });
                    // One kept line below, as tall as a control: quiet Clear at its start when
                    // there is something to clear, then the reason, or the folder Create package
                    // will make. Round 27: the preview lives in this line, so typing moves
                    // nothing below the form.
                    ui.horizontal(|ui| {
                        // As tall as Clear, so Clear coming moves nothing.
                        ui.set_min_height(Density::Read.control());
                        if (!self.name.is_empty() || self.package_folder_changed)
                            && Action::quiet("Clear")
                                .name("Clear the new package fields")
                                .density(Density::Read)
                                .line_start()
                                .show(ui)
                                .clicked()
                        {
                            self.name.clear();
                            self.package_path = None;
                            self.package_folder_changed = false;
                        }
                        if !self.name.is_empty() || self.package_folder_changed {
                            design::gap(ui, design::Gap::S); // With the line's item gap (`S`): `L`.
                        }
                        match (&root, reason) {
                            (_, Some(reason)) => design::blocked_reason(ui, Some(reason)),
                            (Ok(root), None) if !self.name.trim().is_empty() => {
                                will_create(ui, root);
                            }
                            _ => {}
                        }
                    });
                    {
                        if create {
                            let namespace: String = self
                                .name
                                .trim()
                                .to_lowercase()
                                .chars()
                                .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
                                .collect();
                            match root
                                .and_then(|root| {
                                    fs::create_dir_all(
                                        root.parent().ok_or("No package folder is available.")?,
                                    )
                                    .map_err(|e| e.to_string())?;
                                    create_module(&root, &self.name, &namespace)
                                })
                                .and_then(|module| {
                                    campaigns.register_module(module.clone())?;
                                    if !self.package_not_used
                                        && let Some(campaign) = campaigns.active_campaign()
                                    {
                                        crate::content_workspace::use_in_campaign(
                                            &campaign,
                                            module.id,
                                            &module.name,
                                            None,
                                        )?;
                                    }
                                    Ok(module)
                                }) {
                                Ok(module) => {
                                    self.content_changed =
                                        campaigns.active_campaign().map(|c| c.path);
                                    self.scope = Scope::Package(module.id);
                                    self.form = false;
                                    self.new_form = true;
                                    self.new_place = None;
                                    self.new_name.clear();
                                    self.new_folder = kind_folder(&self.new_kind).into();
                                    self.new_not_used = false;
                                    self.focus_new_name = true;
                                    self.stale = true;
                                }
                                Err(error) => self.error = Some(error),
                            }
                        }
                    }
                }
                form_heading(ui, "Registered packages");
                design::rule(ui);
                if campaigns.modules().is_empty() {
                    ui.label(Type::Body.rich(ui, "None yet.").color(c.muted));
                }
                for module in campaigns.modules().to_vec() {
                    ui.push_id(module.id, |ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.spacing_mut().item_spacing.y = 0.0;
                                ui.set_max_width((ui.available_width() - design::XXL).max(120.0));
                                ui.label(Type::Strong.rich(ui, &module.name).color(c.text));
                                ui.add(
                                    egui::Label::new(
                                        Type::Small
                                            .rich(
                                                ui,
                                                module
                                                    .path
                                                    .display()
                                                    .to_string()
                                                    .replace('/', "/\u{200b}"),
                                            )
                                            .color(c.muted),
                                    )
                                    .wrap(),
                                );
                            });
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    design::row_menu(ui, &module.name, |ui| {
                                        if Action::destructive("Unregister…")
                                            .density(Density::Read)
                                            .name(&format!("Unregister {}", module.name))
                                            .blocked(locked)
                                            .show(ui)
                                            .clicked()
                                        {
                                            self.confirm_unregister = Some(module.clone());
                                            ui.close();
                                        }
                                    });
                                },
                            );
                        });
                    });
                    design::rule(ui);
                }
            });
    }

    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw egui calls that a design part replaces (design::results::STAGED_ALLOWS)"
    )]
    fn new_entry(&mut self, ui: &mut egui::Ui, dirty: bool, campaigns: &Campaigns) {
        if self.new_kind.is_empty() {
            self.new_kind = "creature".into();
            self.new_folder = kind_folder(&self.new_kind).into();
        }
        if self.loaded.is_none() {
            return;
        }
        if ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
            self.new_form = false;
            self.new_error = None;
            return;
        }
        // Where a new entry can go: the campaign, or a package. Never the built-in rules.
        let mut places: Vec<Owner> = campaigns
            .modules()
            .iter()
            .cloned()
            .map(Owner::Package)
            .collect();
        if let Some(campaign) = campaigns.active_campaign() {
            places.insert(0, Owner::Campaign(campaign));
        }
        if places.is_empty() {
            return;
        }
        let in_scope = places.iter().position(|owner| match self.scope {
            Scope::Package(id) => owner.id() == Some(id),
            Scope::Campaign => matches!(owner, Owner::Campaign(_)),
            _ => false,
        });
        let default_place = in_scope.unwrap_or(0);
        let mut place = self
            .new_place
            .filter(|place| *place < places.len())
            .unwrap_or(default_place);
        ui.vertical(|ui| {
            ui.set_max_width(ui.available_width().min(design::FORM_WIDTH));
            form_heading(ui, "New entry");
            ui.label(
                Type::Small
                    .rich(ui, "Creates the entry, then opens it in the editor.")
                    .color(colors().muted),
            );
            ui.vertical(|ui| {
                let response = design::text_field(ui, "Entry name", &mut self.new_name, None, None);
                // The create attempt's error ends when the DM changes the name.
                if response.changed() {
                    self.new_error = None;
                }
                if std::mem::take(&mut self.focus_new_name) {
                    response.request_focus();
                }
                if self.new_kind.is_empty() {
                    self.new_kind = "creature".into();
                }
                let old_kind = self.new_kind.clone();
                design::field(ui, "Kind", None, None, |ui| {
                    let word = |kind: &str| {
                        let word = kind.replace('_', " ");
                        format!("{}{}", word[..1].to_uppercase(), &word[1..])
                    };
                    let mut kinds = ["spell", "item", "effect", "rule", "rule_group"]
                        .into_iter()
                        .map(|kind| (kind.to_owned(), word(kind)));
                    design::Dropdown::new(NEW_ENTRY_KIND, "Kind of entry")
                        .choice("creature".to_owned(), word("creature"))
                        .choices(kinds.by_ref())
                        .width(ui.available_width())
                        .show(ui, &mut self.new_kind);
                });
                if self.new_kind != old_kind {
                    self.new_folder = kind_folder(&self.new_kind).into();
                }
                design::field(ui, "Destination", None, None, |ui| {
                    let mut rows = places
                        .iter()
                        .enumerate()
                        .map(|(i, owner)| (i, owner.label()));
                    if let Some((first, label)) = rows.next() {
                        design::Dropdown::new(NEW_ENTRY_PLACE, "Destination")
                            .choice(first, label)
                            .choices(rows)
                            .width(ui.available_width())
                            .show(ui, &mut place);
                    }
                });
                // Round 26 FB6: the choice comes before the last field, so Create entry follows
                // that field on its line; Cancel and the reason are on the line below.
                if matches!(places[place], Owner::Package(_))
                    && campaigns.active_campaign().is_some()
                {
                    use_checkbox(
                        ui,
                        &mut self.new_not_used,
                        |_| "Adds this entry to the campaign’s package choices.",
                    );
                }
                let blocked = if dirty {
                    Some("Save or discard this entry first.")
                } else if self.new_name.trim().is_empty() {
                    Some("Type a name first.")
                } else {
                    None
                };
                let create_width = Action::primary("Create entry").natural_width(ui);
                let mut create = false;
                ui.horizontal_top(|ui| {
                    let width = ui.available_width() - create_width - ui.spacing().item_spacing.x;
                    ui.allocate_ui_with_layout(
                        egui::vec2(width, 0.0),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            folder_field(
                                ui,
                                NEW_ENTRY_FOLDER,
                                "Folder",
                                &mut self.new_folder,
                                places[place].root().unwrap(),
                                Some(&self.new_kind),
                            );
                        },
                    );
                    ui.vertical(|ui| {
                        ui.add_space(design::L + design::XS);
                        create = Action::primary("Create entry")
                            .blocked(blocked)
                            .show(ui)
                            .clicked();
                    });
                });
                let relative = if self.new_folder.trim().is_empty() {
                    format!("{}.md", self.new_name.trim())
                } else {
                    format!(
                        "{}/{}.md",
                        self.new_folder.trim().trim_end_matches('/'),
                        self.new_name.trim()
                    )
                };
                // One kept line: quiet Cancel at its start, then the reason, or the file Create
                // entry will write. Round 27: the preview lives in this line, so typing moves
                // nothing.
                ui.horizontal(|ui| {
                    ui.set_min_height(Density::Read.control());
                    if Action::quiet("Cancel")
                        .density(Density::Read)
                        .line_start()
                        .show(ui)
                        .clicked()
                    {
                        self.new_form = false;
                        self.new_error = None;
                    }
                    design::gap(ui, design::Gap::S); // With the line's item gap (`S`): `L`.
                    if blocked.is_some() {
                        design::blocked_reason(ui, blocked);
                    } else if !self.new_name.trim().is_empty() {
                        let destination = match &places[place] {
                            Owner::Package(package) => &package.name,
                            Owner::Campaign(campaign) => &campaign.name,
                            Owner::Builtin => BUILTIN_NAME,
                        };
                        creation_path(ui, places[place].root().unwrap(), &relative, destination);
                    }
                });
                // A closure, so an early `return` ends only the creation.
                (|| {
                    if create {
                        let name = self.new_name.trim();
                        if name.contains(['/', '\\', '\n', '\r']) || name == "." || name == ".." {
                            self.new_error =
                                Some("Use a name without path separators or line breaks.".into());
                            return;
                        }
                        let path = relative.clone();
                        let text = note_text(name, &self.new_kind, None);
                        let owner = places[place].clone();
                        let root = owner.root().expect("a writable owner").to_path_buf();
                        // The entry the new one replaces in the reader, for Back.
                        let left = self.here();
                        match write_note(&root, &path, None, &text) {
                            Ok(entry) => {
                                let used =
                                    if !self.new_not_used && matches!(owner, Owner::Package(_)) {
                                        campaigns
                                            .active_campaign()
                                            .map(|campaign| {
                                                crate::content_workspace::use_in_campaign(
                                                    &campaign,
                                                    owner.id().unwrap(),
                                                    owner.source(),
                                                    Some(entry.id),
                                                )
                                            })
                                            .transpose()
                                    } else {
                                        Ok(None)
                                    };
                                let l = self.loaded.as_mut().unwrap();
                                if !self.new_not_used
                                    && used.is_ok()
                                    && let (Some(uses), Some(chosen)) = (&mut l.uses, &mut l.chosen)
                                {
                                    uses.insert(entry.id);
                                    chosen.insert(entry.id);
                                }
                                let created_path = path.clone();
                                let created_owner = owner.id();
                                l.notes.push(Note {
                                    path,
                                    text: text.clone(),
                                    entry: Some(entry),
                                    error: None,
                                    fault: None,
                                    owner,
                                });
                                l.notes.sort_by_cached_key(|n| natural_key(n.name()));
                                l.invalidate();
                                self.matches = None;
                                self.scope_counts = None;
                                self.selected = l.notes.iter().position(|n| {
                                    n.path == created_path && n.owner.id() == created_owner
                                });
                                if self.selected.is_some() {
                                    self.trail.leave(left);
                                }
                                self.reveal_row = true;
                                self.search.clear();
                                self.section = Section::All;
                                self.source = true;
                                self.draft = text;
                                self.focus_hp = self.new_kind == "creature";
                                self.new_name.clear();
                                self.new_error = None;
                                self.new_folder.clear();
                                self.new_form = false;
                                self.error = used.err().map(|error| {
                                    format!(
                                        "Entry created. Could not use it in this campaign: {error}"
                                    )
                                });
                                // Show the new entry, wherever the DM chose to put it.
                                if self
                                    .selected
                                    .is_some_and(|i| !l.in_scope(&l.notes[i], self.scope))
                                {
                                    self.scope = Scope::All;
                                }
                                self.content_changed = Some(root);
                            }
                            Err(e) => self.new_error = Some(e),
                        }
                    }
                })();
            });
        });
        self.new_place = (place != default_place).then_some(place);
    }

    /// The open entry, as Back or Forward returns to it.
    fn here(&self) -> Option<Place> {
        place(
            self.loaded.as_ref()?,
            self.selected,
            self.scope,
            self.section,
            &self.search,
        )
    }

    fn go(&mut self, step: Step) {
        if self.dirty() {
            return;
        }
        let here = self.here();
        let Some((target, index)) = self.trail.go(step, here, |target| {
            target.find(&self.loaded.as_ref()?.notes)
        }) else {
            return;
        };
        self.selected = Some(index);
        self.reveal_row = true;
        self.draft = self.loaded.as_ref().unwrap().notes[index].text.clone();
        self.disk_change = None;
        self.scope_notice = None;
        self.scope = target.scope;
        self.section = target.section;
        self.search = target.search;
        self.matches = None;
        self.source = false;
        self.error = None;
    }

    /// Keep the reader at the pane's left edge, with its scroll bar beside its measure.
    fn editor(&mut self, ui: &mut egui::Ui, subtitle: &str, campaigns: &Campaigns) {
        let spare = ui.available_width() - view::WIDE_READER;
        if spare <= 0.0 || self.disk_change.is_some() {
            self.editor_pane(ui, subtitle, campaigns, None);
            return;
        }
        // The reader's lines keep their widest measure. Its body may still reach the pane's
        // edge, for a wide table.
        let pane_right = ui.max_rect().right();
        ui.horizontal_top(|ui| {
            let size = egui::vec2(view::WIDE_READER, ui.available_height());
            ui.allocate_ui_with_layout(size, egui::Layout::top_down(egui::Align::Min), |ui| {
                ui.set_max_width(view::WIDE_READER);
                self.editor_pane(ui, subtitle, campaigns, Some(pane_right));
            });
        });
    }

    fn save_selected(&mut self, file_gone: bool) {
        let Some(i) = self.selected else {
            return;
        };
        let note = &self.loaded.as_ref().unwrap().notes[i];
        let mut file_gone = file_gone;
        if self.loaded.as_ref().unwrap().duplicates(i).is_empty()
            && (file_gone
                || note_path(note.owner.root().unwrap(), &note.path)
                    .is_ok_and(|path| !path.exists()))
        {
            let moved = crate::content_workspace::note_id(&note.text)
                .map(|id| {
                    let mut owners = vec![note.owner.clone()];
                    for owner in &self.loaded.as_ref().unwrap().owners {
                        if !owners.contains(owner) {
                            owners.push(owner.clone());
                        }
                    }
                    for other in &self.loaded.as_ref().unwrap().notes {
                        if other.owner.root().is_some() && !owners.contains(&other.owner) {
                            owners.push(other.owner.clone());
                        }
                    }
                    let mut found = None;
                    for owner in owners {
                        let old = if owner == note.owner {
                            note.path.as_str()
                        } else {
                            ""
                        };
                        if let Some((path, text)) = moved_note(owner.root().unwrap(), id, old)? {
                            if found.is_some() {
                                return Err(
                                    "Several files have this entry’s ID. Choose one in Library."
                                        .to_owned(),
                                );
                            }
                            found = Some((owner, path, text));
                        }
                    }
                    Ok(found)
                })
                .transpose();
            match moved {
                Ok(Some(Some((owner, path, text)))) => {
                    let note = &mut self.loaded.as_mut().unwrap().notes[i];
                    note.path = path;
                    note.owner = owner;
                    if note.text != text {
                        self.disk_change = Some(DiskChange {
                            text: Some(text),
                            moved_to: Some(note.path.clone()),
                            fault: None,
                        });
                        self.error = Some(NOTE_CHANGED.into());
                        return;
                    }
                    file_gone = false;
                    self.stale = true;
                }
                Err(error) => {
                    self.error = Some(error);
                    return;
                }
                _ => {}
            }
        }
        // The write check reads the identity index from the notes as they are now.
        self.loaded.as_mut().unwrap().duplicates.take();
        let loaded = self.loaded.as_ref().unwrap();
        let note = &loaded.notes[i];
        let root = note.owner.root().expect("an editable note").to_path_buf();
        let refused = loaded.draft_refusal(i, &Entry::parse(&note.path, &self.draft));
        let result = match &refused {
            Some(error) => Err(error.clone()),
            None => {
                let baseline = (!file_gone).then_some(note.text.as_str());
                write_note(&root, &note.path, baseline, &self.draft)
            }
        };
        match result {
            Ok(entry) => {
                let l = self.loaded.as_mut().unwrap();
                // A saved edit of the entry ends its link update's result and its earlier offer,
                // whatever this save offers. A rename that passes the check gives a new offer.
                // Other entries keep theirs.
                let owner = LinkOwner {
                    id: entry.id,
                    root: root.clone(),
                };
                self.link_edits.owner_changed(&owner);
                self.renamed_links.owner_changed(&owner);
                if let Some(old) = &l.notes[i].entry
                    && old.name != entry.name
                    && entry.kind != "package"
                    && l.can_rename_links(i, &old.name, &entry.name)
                {
                    self.renamed_links.record(LinkOffer {
                        owner: entry.id,
                        source: OfferSource {
                            root: root.clone(),
                            path: l.notes[i].path.clone(),
                            text: self.draft.clone(),
                        },
                        old: old.name.clone(),
                        new: entry.name.clone(),
                    });
                }
                if entry.kind == "package"
                    && let Owner::Package(module) = &l.notes[i].owner
                {
                    self.renamed = Some((module.id, entry.name.clone()));
                    let id = module.id;
                    for note in &mut l.notes {
                        if let Owner::Package(owner) = &mut note.owner
                            && owner.id == id
                        {
                            owner.name = entry.name.clone();
                        }
                    }
                }
                self.stale |= l.notes[i]
                    .entry
                    .as_ref()
                    .is_none_or(|old| old.id != entry.id);
                l.notes[i].entry = Some(entry);
                l.notes[i].text = self.draft.clone();
                l.notes[i].error = None;
                l.invalidate();
                self.matches = None;
                self.scope_counts = None;
                self.just_saved = true;
                self.save_failure = None;
                self.content_changed = Some(root);
                l.refresh_diagnostics();
                self.error = None;
                // Written: the repair it was opened for is done. A failed Save keeps it.
                self.repair_link = None;
            }
            Err(e) => match read_disk(&root, &note.path) {
                // The file is there but cannot be read: the draft stays, the file keeps its bytes.
                Err(fault) => {
                    self.disk_change = Some(DiskChange {
                        text: None,
                        moved_to: None,
                        fault: Some(fault),
                    });
                }
                Ok(text) if e == NOTE_CHANGED => {
                    self.disk_change = Some(DiskChange {
                        moved_to: None,
                        text,
                        fault: None,
                    });
                }
                // The disk refused the write: the draft's state slot says so with Save's reason.
                Ok(_) if refused.is_none() => {
                    self.save_failure = Some(SaveFailure {
                        owner: note.owner.id(),
                        path: note.path.clone(),
                        revision: self.draft_revision,
                        reason: e,
                        read_only: note_path(&root, &note.path)
                            .ok()
                            .and_then(|path| ReadOnly::of(&root, &path)),
                    });
                }
                // Save refused the draft before the write: the refusal is the draft's own reason,
                // shown with its state while that draft lives. A page error would outlive the
                // draft (round 26: it now shows in the same footer).
                Ok(_) => {}
            },
        }
    }

    /// Undo package choice and Undo write: the results that this pane keeps for its package. The
    /// page results' presses wait until the reader is drawn.
    fn apply_package_results(&mut self, pressed: &ResultPress) {
        self.page_pressed = pressed.page;
        if pressed.package_use
            && let Some(undo) = self.selection_undo.take()
        {
            let result = crate::content_workspace::CampaignDocument::load(&undo.campaign).and_then(|mut document| {
                if document.selections != undo.after { return Err("Campaign choices changed. Keep them and edit the package choices in Campaign.".into()); }
                document.selections = undo.before;
                document.save()
            });
            match result {
                Ok(()) => {
                    self.stale = true;
                    self.content_changed = Some(undo.campaign.path);
                }
                Err(error) => self.error = Some(error),
            }
        }
        if pressed.manifest
            && let Some(write) = &self.manifest_write
        {
            match write.undo() {
                Ok(root) => {
                    self.content_changed = Some(root);
                    self.manifest_write = None;
                    self.stale = true;
                }
                Err(error) => self.error = Some(error),
            }
        }
    }

    /// `pane_right` is the pane's right edge, where the reader's body may end.
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: calls `ResultOrder::issue`, `Footer::result` or `save_row_state`, and raw egui calls (design::results::STAGED_ALLOWS)"
    )]
    fn editor_pane(
        &mut self,
        ui: &mut egui::Ui,
        subtitle: &str,
        campaigns: &Campaigns,
        pane_right: Option<f32>,
    ) {
        // The pane's kept results. They sit in the pane's message footer, so a result never moves
        // the tabs, the file line, the title or the body.
        let mut results = ReaderResults {
            package_use: self.selection_undo.as_ref().is_some_and(|undo| {
                campaigns
                    .active_campaign()
                    .is_some_and(|c| c.id == undo.campaign.id)
            }),
            manifest: self
                .manifest_write
                .as_ref()
                .map(|write| (write.message(), format!("Undo write {}", write.path()))),
            locked: self.draft_reason(),
            page: std::mem::take(&mut self.page_results),
            ..Default::default()
        };
        // With no entry open, the results and Remove's notice sit in the pane's message footer,
        // below the empty reader (round 26). Its id is the reader's, so one pane has one footer.
        let gone_notice = self.discarded_note.as_ref().map(|LetGo { note, .. }| {
            let names = self
                .loaded
                .as_ref()
                .map(|loaded| loaded.dependents(note))
                .unwrap_or_default();
            if names.is_empty() {
                format!("{} removed. Its draft was discarded.", note.name())
            } else {
                format!(
                    "{} removed. Its draft was discarded. {} now needs a missing entry.",
                    note.name(),
                    name_list(&names)
                )
            }
        });
        let mut undo_remove = false;
        if self.selected.is_none() {
            let pressed = std::cell::Cell::new(ResultPress::default());
            let mut footer = design::Footer::new(
                "library_footer",
                ui.available_width().min(design::READER_MEASURE),
            );
            if let Some(words) = gone_notice.as_deref().filter(|_| !self.dirty()) {
                let undo = &mut undo_remove;
                let order = order_of(&self.result_orders, OrderKey::Removed);
                footer.result(order, words, true, move |ui| {
                    *undo = design::undo_message(ui, Tone::Info, words, "Undo remove");
                });
            }
            add_results(&mut footer, &results, &self.result_orders, &pressed);
            page_error(&mut footer, self.error.as_deref());
            // The empty reader below holds nothing past the cursor, so the pane ends at the
            // footer's top.
            let _ = footer.show_by_hand(ui);
            self.apply_package_results(&pressed.get());
        }
        let recovery = self
            .selected
            .and_then(|at| self.loaded.as_ref()?.notes.get(at))
            .filter(|note| note.is_manifest() && note.error.is_some())
            .and_then(|note| match &note.owner {
                Owner::Package(module) => {
                    let path = note.path.clone();
                    let exists = note.error.as_deref()
                        != Some(crate::content_workspace::DELETED_MANIFEST)
                        && module.path.join(&path).exists();
                    Some((module.clone(), path, exists))
                }
                _ => None,
            });
        if undo_remove
            && let Some((_, _, draft)) = self.discarded.take()
            && let Some(LetGo {
                note, used, chosen, ..
            }) = self.discarded_note.take()
            && let Some(loaded) = self.loaded.as_mut()
        {
            // The reload after Remove entry and lose draft left the entry out of what this campaign uses and
            // counted the entries that need it. Undo gives back both with the draft.
            if used && let (Some(uses), Some(entry)) = (loaded.uses.as_mut(), note.entry.as_ref()) {
                uses.insert(entry.id);
            }
            if chosen
                && let (Some(set), Some(entry)) = (loaded.chosen.as_mut(), note.entry.as_ref())
            {
                set.insert(entry.id);
            }
            self.selected = Some(loaded.notes.len());
            loaded.notes.push(note);
            loaded.invalidate();
            loaded.refresh_diagnostics();
            self.draft = draft;
            // The draft changed: a failed Save of the old words no longer owns it.
            self.draft_revision += 1;
            self.source = true;
            self.disk_change = Some(DiskChange {
                text: None,
                moved_to: None,
                fault: None,
            });
            self.invalidate();
        }
        let Some(i) = self.selected else {
            design::empty_state(
                ui,
                icons::LIBRARY,
                "No entry open",
                if campaigns.active_campaign().is_some() || !campaigns.modules().is_empty() {
                    "Choose one from the list, or make a new one."
                } else {
                    "Choose one from the list."
                },
                |_| {},
            );
            return;
        };
        // Overwrite was chosen: what is on disk now becomes what this save replaces.
        let overwrite = std::mem::take(&mut self.overwrite_disk);
        let mut file_gone = false;
        if overwrite && let Some(change) = self.disk_change.take() {
            if let Some(path) = change.moved_to {
                self.loaded.as_mut().unwrap().notes[i].path = path;
                self.stale = true;
            }
            match change.text {
                Some(text) => self.loaded.as_mut().unwrap().notes[i].text = text,
                None => file_gone = true,
            }
        }
        let draft_reason = self.draft_reason();
        let parsed_draft = self.parsed_draft(i);
        let parsed: &Result<Entry, String> = &parsed_draft;
        let note = &self.loaded.as_ref().unwrap().notes[i];
        let reading = parsed
            .as_ref()
            .ok()
            .and_then(|entry| entry.reading().ok().flatten());
        // A file with no entry to read is titled by its file name; the file line names its path.
        let file_name = Path::new(&note.path)
            .file_name()
            .map(|name| name.to_string_lossy().into_owned());
        let title = match &reading {
            Some(reading) => reading.title.as_str(),
            None if note.entry.is_none()
                && note.error.as_deref() != Some(crate::content_workspace::DELETED_MANIFEST) =>
            {
                file_name.as_deref().unwrap_or(note.name())
            }
            None => note.name(),
        };
        // A duplicate keeps its folder too: the two readers differ only there.
        let reading_subtitle = reader_subtitle(
            &note.owner,
            &row_line_without_fault(subtitle),
            reading
                .as_ref()
                .and_then(|_| presentation::class_level(parsed.as_ref().ok()?)),
        );
        let dirty = self.draft != note.text;
        let editable = note.owner.editable();
        // A file the system will not read has no text to edit: the reader names its fault.
        let fault = note.fault.clone();
        if fault.is_some() {
            self.source = false;
        }
        let conflict = self.disk_change.is_some();
        // Read: the reader's messages sit in the pane's message footer, below the body.
        let choices_in_reader = !self.source && !conflict;
        let mut duplicate_indices = self.loaded.as_ref().unwrap().duplicates(i);
        if !duplicate_indices.is_empty() {
            duplicate_indices.push(i);
            let notes = &self.loaded.as_ref().unwrap().notes;
            let first = crate::content_workspace::duplicate_repair_path(
                duplicate_indices.iter().filter_map(|&at| {
                    notes[at]
                        .owner
                        .root()
                        .map(|root| root.join(&notes[at].path))
                }),
            );
            duplicate_indices.sort_by_key(|&at| {
                notes[at]
                    .owner
                    .root()
                    .map(|root| root.join(&notes[at].path))
                    != first
            });
        }
        let mut new_identity = None;
        let mut new_identities = None;
        // Only a side-by-side comparison of draft and disk text takes the full width. A deleted
        // file's lone draft keeps the source editor's measure.
        let editor_width = if self
            .disk_change
            .as_ref()
            .is_some_and(|change| change.text.is_some())
        {
            ui.available_width()
        } else {
            ui.available_width().min(EDITOR_MAX_WIDTH)
        };
        self.just_saved &= !dirty && self.source;
        // Ctrl+S is taken only where a draft could be saved; it saves exactly when Save can
        // (`DraftState::can_save`, below).
        let save_key = dirty
            && editable
            && !conflict
            && ui.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::S));
        let d = colors().document;
        // The file stopped reading under the draft: its owner line says so, as for a file that
        // never read.
        let stopped_reading = self
            .disk_change
            .as_ref()
            .is_some_and(|change| change.fault.is_some());
        let about = note.entry.as_ref().is_some_and(|e| e.kind == "package");
        let mut use_every = None;
        let mut every_chosen = false;
        let package_word = match &note.owner {
            Owner::Package(module) if stopped_reading => {
                format!("{} · Cannot be read", module.name)
            }
            Owner::Package(_) if about && campaigns.active_campaign().is_none() => {
                "No campaign open".into()
            }
            Owner::Package(module) if about => {
                let loaded = self.loaded.as_ref().unwrap();
                let entries: Vec<_> = loaded
                    .notes
                    .iter()
                    .filter(|n| {
                        n.owner.id() == Some(module.id)
                            && n.entry.as_ref().is_some_and(|e| {
                                !["package", "reference"].contains(&e.kind.as_str())
                            })
                    })
                    .collect();
                // Round 27 FA6: the DM's choices and the entries they need, as Campaign's
                // package row counts its ticked boxes.
                let chosen = entries.iter().filter(|n| loaded.chose(n)).count();
                let needed = entries
                    .iter()
                    .filter(|n| !loaded.chose(n) && loaded.in_scope(n, Scope::Uses))
                    .count();
                every_chosen = chosen == entries.len();
                chosen_count(chosen, needed, entries.len())
            }
            Owner::Package(module)
                if note.error.as_deref() == Some(crate::content_workspace::DELETED_MANIFEST) =>
            {
                format!("{} · Manifest deleted", module.name)
            }
            Owner::Package(module) if note.error.is_some() => {
                format!("{} · Cannot be read", module.name)
            }
            Owner::Package(module) => format!(
                "{} · {}",
                module.name,
                if self.loaded.as_ref().unwrap().in_scope(note, Scope::Uses) {
                    "Chosen for this campaign"
                } else {
                    "Not chosen"
                }
            ),
            _ => String::new(),
        };
        // A ready state means Save would accept the draft. Every draft with changes, and every
        // repair draft, asks Save's own check before the write (parse, identity, ownership):
        // the parse is cached, and the identity index is cached until the notes change.
        let refusal = (note.error.is_some() || dirty)
            .then(|| self.loaded.as_ref().unwrap().draft_refusal(i, parsed))
            .flatten();
        // Why Save refuses this draft now. It blocks Save and Ctrl+S, and the state is Invalid.
        let save_refusal = refusal.clone().filter(|_| dirty);
        let (repair_problem, repair_line) =
            draft_repair_problem(note, &self.draft, refusal, recovery.as_ref());
        // A missing link Campaign sent here: the row that holds it in the parsed draft.
        let here = note.owner.root().map(|root| root.join(&note.path));
        let missing_link = self
            .repair_link
            .as_mut()
            .filter(|link| here.as_ref() == Some(&link.path))
            .and_then(|link| {
                // Looked up again only for a new parse, that is, new draft text.
                let line = (*link.found.get_or(parsed_draft.revision(), || {
                    parsed
                        .as_ref()
                        .ok()
                        .and_then(|entry| repair_row(entry, &link.target))
                }))?;
                let words = &link.words;
                Some((
                    line,
                    format!(
                        "Line {line}: {words} Replace the link on this line with another creature, or remove its row."
                    ),
                ))
            });
        let repair_line = repair_line.or(missing_link.as_ref().map(|(line, _)| *line));
        // A Save that failed on the disk, while this is still the draft it tried to write.
        if let Some(failure) = &self.save_failure {
            #[cfg(test)]
            {
                let (checks, reads) = self.failure_checks.get();
                self.failure_checks.set((checks + 1, reads));
            }
            let draft = DraftView::new(
                &self.draft,
                #[cfg(test)]
                &self.failure_checks,
            );
            if !failure.owns(note, self.draft_revision, draft) {
                self.save_failure = None;
            }
        }
        let failed = self
            .save_failure
            .as_ref()
            .map(|failure| failure.reason.clone());
        // The save row's state. Its word, Save's weight and why Save waits all come from it
        // (`design::save_row`). A parse fault ends with the draft that parses: the saved file
        // stays unreadable on the owner line until Save, but the draft is not Invalid.
        let mut draft_state = design::DraftState::new(
            if dirty {
                design::save_row::Changed::Yes
            } else {
                design::save_row::Changed::No
            },
            if self.run_open {
                design::save_row::Primary::Elsewhere
            } else {
                design::save_row::Primary::Draft
            },
        );
        if let Some(refusal) = &save_refusal {
            draft_state = draft_state.refused(refusal.as_str());
        }
        if fault.is_some() || repair_problem.is_some() {
            draft_state = draft_state.invalid();
        }
        if conflict {
            draft_state = draft_state.changed_on_disk();
        }
        if failed.is_some() {
            // An action-time failure: the disk refused the last Save of this draft.
            draft_state = draft_state.not_saved();
        }
        if self.just_saved {
            draft_state = draft_state.just_saved();
        }
        // Overwrite was chosen in the comparison: it saves whatever the draft holds.
        let mut save = overwrite || (save_key && draft_state.can_save());
        let save_row = editable && (self.source || dirty);
        let mut step = history::take_if(ui.ctx(), |_| true);
        // Edit Markdown's save row, with the pane's message footer below the editor.
        let slot_row = save_row && !conflict;
        let can_copy = note
            .entry
            .as_ref()
            .is_some_and(|e| !["package", "campaign", "reference"].contains(&e.kind.as_str()));
        let copy_reason = draft_reason.or_else(|| copy_destination_reason(campaigns));
        let can_make = matches!(note.owner, Owner::Campaign(_))
            && note.entry.as_ref().is_some_and(|e| e.kind == "creature");
        // Round 25 FF3: the campaign choice is a toolbar action, so it never sends the file line's
        // path to a second row: the title keeps one place for chosen and not-chosen entries.
        let choose = match &note.owner {
            Owner::Package(_)
                if about && campaigns.active_campaign().is_some() && !every_chosen =>
            {
                Some("Choose all")
            }
            Owner::Package(_)
                if !about
                    && note.entry.is_some()
                    && !self.loaded.as_ref().unwrap().in_scope(note, Scope::Uses) =>
            {
                Some("Choose for this campaign")
            }
            _ => None,
        };
        let mut copy_clicked = false;
        let mut make_clicked = false;
        let mut choose_clicked = false;
        let back = Action::quiet("Back")
            .icon(icons::BACK)
            .density(Density::Read);
        let edit_label = if editable {
            "Edit Markdown"
        } else {
            "Markdown source"
        };
        let base_width = Action::quiet("Read")
            .density(Density::Read)
            .natural_width(ui)
            + Action::quiet(edit_label)
                .density(Density::Read)
                .natural_width(ui)
            + if self.trail.peek(Step::Back).is_none() {
                0.0
            } else {
                back.natural_width(ui) + design::S
            };
        let extras = if can_copy {
            Action::quiet("Copy to…")
                .icon(icons::COPY)
                .density(Density::Read)
                .natural_width(ui)
                + design::S
        } else {
            0.0
        } + if can_make {
            Action::quiet("Make NPC note")
                .density(Density::Read)
                .natural_width(ui)
                + design::S
        } else {
            0.0
        } + choose.map_or(0.0, |label| {
            Action::quiet(label)
                .density(Density::Read)
                .natural_width(ui)
                + design::S
        });
        let with_key = base_width
            + extras
            + design::number_width(ui, "Alt+Left", &Type::Small.font(ui))
            + design::L;
        let show_key = with_key <= ui.available_width();
        let overflow = base_width + extras > ui.available_width();
        ui.horizontal(|ui| {
            let edit_label = if editable {
                "Edit Markdown"
            } else {
                "Markdown source"
            };
            let was_source = self.source;
            design::locked(ui, fault.as_ref().map(|_| UNREAD_EDIT), |ui| {
                design::segmented(ui, &mut self.source, &[(false, "Read"), (true, edit_label)])
            });
            if self.source != was_source {
                self.scope_notice = None;
                self.focus_repair = self.source && note.error.is_some();
            }
            if !overflow {
                if can_copy {
                    copy_clicked = Action::quiet("Copy to…")
                        .icon(icons::COPY)
                        .density(Density::Read)
                        .blocked(copy_reason)
                        .show(ui)
                        .clicked();
                }
                if can_make {
                    make_clicked = Action::quiet("Make NPC note")
                        .density(Density::Read)
                        .blocked(draft_reason)
                        .show(ui)
                        .clicked();
                }
                if let Some(label) = choose {
                    choose_clicked = Action::quiet(label)
                        .density(Density::Read)
                        .blocked(draft_reason)
                        .show(ui)
                        .clicked();
                }
            }
            if self.trail.peek(Step::Back).is_some()
                && (if show_key { back.key("Alt+Left") } else { back })
                    .hint(
                        "Return to the entry before this link. During a draft, read it in Lookup.",
                    )
                    .show(ui)
                    .clicked()
            {
                step = Some(Step::Back);
            }
            if overflow {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    egui::containers::menu::MenuButton::from_button(
                        egui::Button::new(Type::Strong.rich(ui, "More"))
                            .frame(false)
                            .min_size(egui::vec2(0.0, Density::Read.control())),
                    )
                    .ui(ui, |ui| {
                        if can_copy {
                            copy_clicked = Action::quiet("Copy to…")
                                .icon(icons::COPY)
                                .density(Density::Read)
                                .blocked(copy_reason)
                                .show(ui)
                                .clicked();
                        }
                        if can_make {
                            make_clicked = Action::quiet("Make NPC note")
                                .density(Density::Read)
                                .blocked(draft_reason)
                                .show(ui)
                                .clicked();
                        }
                        if let Some(label) = choose {
                            choose_clicked = Action::quiet(label)
                                .density(Density::Read)
                                .blocked(draft_reason)
                                .show(ui)
                                .clicked();
                        }
                        if copy_clicked || make_clicked || choose_clicked {
                            ui.close();
                        }
                    });
                });
            }
        });
        if choose_clicked && let Owner::Package(module) = &note.owner {
            if about {
                use_every = Some(module.clone());
            } else if let Some(entry) = &note.entry {
                self.use_entry = Some((module.id, entry.id));
            }
        }
        if copy_clicked {
            self.copy = Some(crate::content_workspace::CopySource {
                entry: note.entry.as_ref().unwrap().clone(),
                text: note.text.clone(),
                root: note.owner.root().map(Path::to_path_buf),
                campaign: match &note.owner {
                    Owner::Campaign(campaign) => Some(campaign.clone()),
                    _ => None,
                },
            });
        }
        if make_clicked && let (Owner::Campaign(campaign), Some(entry)) = (&note.owner, &note.entry)
        {
            match crate::world::make_npc_note(campaign, entry) {
                Ok(id) => {
                    self.stale = true;
                    self.content_changed = Some(campaign.path.clone());
                    self.open_world_npc = Some(id);
                }
                Err(error) => self.error = Some(error),
            }
        }
        if let Some(root) = note.owner.root() {
            let moved_path = self
                .disk_change
                .as_ref()
                .and_then(|change| change.moved_to.as_deref());
            let shown_path = moved_path.unwrap_or(&note.path);
            let moved_from = moved_path.map(|_| {
                format!(
                    "moved from {}/",
                    Path::new(&note.path)
                        .parent()
                        .unwrap_or(Path::new(""))
                        .display()
                )
            });
            // No editor link for a file that cannot be read, whether it never read or stopped
            // reading under a draft: an editor cannot open it either.
            let unread = fault.is_some()
                || self
                    .disk_change
                    .as_ref()
                    .is_some_and(|change| change.fault.is_some());
            let obsidian = obsidian_for(root, &note.owner) && !unread;
            // The draft's file has gone from disk: no link can show it, so the line offers its
            // folder, as a deleted manifest's does.
            let file_gone_now = self.disk_change.as_ref().is_some_and(|change| {
                change.text.is_none() && change.moved_to.is_none() && change.fault.is_none()
            });
            let deleted_manifest = recovery.as_ref().is_some_and(|(_, _, exists)| !exists);
            let file_line = |ui: &mut egui::Ui| {
                if deleted_manifest {
                    show_folder(ui, root);
                } else if file_gone_now {
                    gone_file_parts(ui, root, shown_path);
                } else if let Some(from) = &moved_from {
                    // Round 27 FD7: one statement about the path, then its links: the new
                    // path, the muted old folder, then `Show file`.
                    moved_file_parts(ui, root, shown_path, from, obsidian);
                } else {
                    crate::world::entry_file_parts(ui, root, shown_path, obsidian);
                }
            };
            // FL1: once Save has found the file or its folder read only, the owner line says so,
            // as long as that failure lasts (the next edit, another entry or a Save ends it).
            let read_only = self
                .save_failure
                .as_ref()
                .is_some_and(|failure| failure.read_only.is_some())
                && note.error.is_none()
                && !stopped_reading;
            let read_only_word;
            let owner_word = match &note.owner {
                Owner::Package(module) if read_only => {
                    read_only_word = format!("{} · Read only", module.name);
                    read_only_word.as_str()
                }
                Owner::Package(_) => package_word.as_str(),
                Owner::Campaign(_) if note.error.is_some() || stopped_reading => {
                    "Campaign · Cannot be read"
                }
                _ if read_only => "Campaign · Read only",
                _ => "Campaign · Editable",
            };
            let file_below = ui.available_width()
                < design::number_width(ui, owner_word, &Type::Small.font(ui))
                    + design::number_width(ui, "Show file", &Type::Body.font(ui))
                    + design::XXL * 2.0
                    + if obsidian {
                        design::number_width(ui, "Edit in Obsidian", &Type::Body.font(ui))
                    } else {
                        0.0
                    };
            // The path shortens from the front and keeps its file name whole
            // (`…/Cinder Wolf copy.md`). When even that does not fit after the owner words and
            // their action, the path takes its own Read row: it is measured where it would start.
            let filename = Path::new(shown_path)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy();
            let shortest_path = if filename == *shown_path {
                filename.to_string()
            } else {
                format!("…/{filename}")
            };
            let path_room = if deleted_manifest {
                design::number_width(ui, "Show folder", &Type::Body.font(ui))
            } else {
                design::number_width(ui, &shortest_path, &Type::Small.font(ui))
                    + design::L
                    + moved_from.as_ref().map_or(0.0, |from| {
                        design::number_width(ui, from, &Type::Small.font(ui)) + design::S
                    })
                    + if file_gone_now {
                        design::number_width(ui, "Show folder", &Type::Body.font(ui))
                    } else {
                        crate::world::entry_file_links_width(ui, obsidian)
                    }
            };
            let mut path_below = false;
            let owner_line = |ui: &mut egui::Ui| {
                // In a narrow reader (Lookup open beside it at 960), the campaign choice keeps
                // its first word rather than a cut: `Ashen Marches · Chosen`.
                let short_word;
                let owner_word = match owner_word.strip_suffix(" for this campaign") {
                    Some(short)
                        if design::number_width(ui, owner_word, &Type::Small.font(ui))
                            + design::XL
                            > ui.available_width() =>
                    {
                        short_word = short.to_owned();
                        short_word.as_str()
                    }
                    _ => owner_word,
                };
                design::status(
                    ui,
                    if read_only {
                        Status {
                            icon: icons::READ_ONLY,
                            word: owner_word,
                            color: d.read_only,
                        }
                    } else {
                        Status {
                            icon: icons::EDIT,
                            word: owner_word,
                            color: d.clean,
                        }
                    },
                );
                if !file_below {
                    if ui.available_width() >= path_room {
                        file_line(ui);
                    } else {
                        path_below = true;
                    }
                }
            };
            file_line_row(ui, file_below, owner_line);
            if file_below || path_below {
                file_line_row(ui, false, file_line);
            }
        } else {
            file_line_row(ui, false, |ui| {
                design::status(
                    ui,
                    Status {
                        icon: icons::READ_ONLY,
                        word: "Built-in · Read only",
                        color: d.read_only,
                    },
                );
            });
        }
        // Round 26 FC4: with a save row, the duplicate warning and its choices sit in the pane's
        // message footer, so Save keeps one place and the editor its top. Without one (a file
        // changed on disk) they stay above, with the other warnings.
        if !duplicate_indices.is_empty() && !choices_in_reader && !slot_row {
            duplicate_choices(
                ui,
                self.loaded.as_ref().unwrap(),
                note,
                &duplicate_indices,
                draft_reason,
                &mut new_identity,
                &mut new_identities,
            );
        }
        results.identities = self
            .identities_undo
            .as_ref()
            .map(|(_, words)| words.clone());
        results.identity = self.identity_undo.as_ref().map(|undo| {
            crate::world::separate_entry_result(
                &Path::new(&undo.path)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy(),
            )
        });
        // Edit Markdown puts the results in the pane's message footer, below the editor, so a
        // result never moves the editor. Without that row (a built-in's source, a file changed on disk)
        // they stay above the editor, with the warnings.
        let mut pressed = ResultPress::default();
        if !choices_in_reader && !slot_row {
            // A file changed on disk: the comparison owns the pane, and its messages stay above it.
            if let Some(error) = &self.error {
                design::message_with_code(ui, Tone::Error, &format!("Error: {error}"));
            }
            pressed = show_results(ui, &results);
        }
        let mut repair_pressed = false;
        let mut read_again = false;
        // The heading warning and its Undo. Read and Edit Markdown both draw them in the pane's
        // message footer, with the duplicate warning, so one file's warnings share one place and
        // the body never moves.
        let heading = HeadingChoice::new(
            (editable && !conflict)
                .then(|| parsed.as_ref().ok())
                .flatten(),
            &self.draft,
            self.heading_undo.as_ref(),
            &note.path,
        );
        let mut heading_pressed = (false, false);
        // Read's messages for the pane's message footer: a package entry's links to entries in
        // this campaign only, each with its copy action, and the copy's result.
        let dependencies: Vec<(Uuid, String, String)> = match (&note.owner, &note.entry) {
            (Owner::Package(package), Some(entry)) if choices_in_reader => ui
                .ctx()
                .data(|data| {
                    data.get_temp::<std::sync::Arc<crate::lookup::ContentIndex>>(egui::Id::new(
                        "reader_link_index",
                    ))
                })
                .map(|index| {
                    index
                        .campaign_dependencies(entry)
                        .into_iter()
                        .map(|at| {
                            let dependency = &index.items[at].entry;
                            (
                                dependency.id,
                                // Row 145 (FC6): say what the link is and why it matters.
                                format!(
                                    "{} links to {}, which is in this campaign only. Another \
                                     campaign that uses {} will not find it.",
                                    entry.name, dependency.name, package.name
                                ),
                                format!("Copy {} to {}", dependency.name, package.name),
                            )
                        })
                        .collect()
                })
                .unwrap_or_default(),
            _ => Vec::new(),
        };
        // The copies this file made, by package root and path, newest first, with their place in
        // `dependency_undos` for their Undo.
        let dependency_results: Vec<(usize, String, design::ResultOrder)> = self
            .dependency_undos
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, change)| {
                choices_in_reader
                    && change.path == note.path
                    && note.owner.root() == Some(change.root.as_path())
            })
            .map(|(at, change)| (at, change.result.clone(), change.order))
            .collect();
        let mut reader_press = ReaderPress::default();
        let no_results = ReaderResults::default();
        // Where the body ends: above the message footer, when the save row has one.
        let mut body_bottom = f32::INFINITY;
        // Save and Discard are one row, as far apart as the pane allows.
        if slot_row {
            let row = ui
                .scope(|ui| {
                    ui.set_max_width(editor_width);
                    design::save_row(ui, &draft_state, "Save")
                })
                .inner;
            save |= row.save;
            if row.discard {
                self.result_orders.insert(
                    OrderKey::Reader(ResultKind::Discarded),
                    design::ResultOrder::issue(),
                );
                self.discarded = Some((note.owner.id(), note.path.clone(), self.draft.clone()));
                self.draft = note.text.clone();
                // The draft changed: a failed Save of the old words no longer owns it.
                self.draft_revision += 1;
                self.error = None;
            }
            // The pane's message footer, pinned at the bottom of the pane below the editor, so
            // no message moves the editor: its top stays and only its visible height changes.
            // Results that carry a live Undo come first (Discard's, the heading and stat fixes',
            // the reader's results with Undo): a broken file's reason never clears after Discard
            // or Reload, so a reason first would hide the only way back. Then Save's reason (a
            // repair draft's parse error with its recovery, a refusal before the write, the
            // disk's failure). Then the notices: results with no Undo, the missing link Campaign
            // sent here, the other broken links, the first-heading and stat warnings. With none
            // of these, the footer says why other actions wait.
            let discard_shown =
                discard_is_for(self.discarded.as_ref(), note) && self.draft == note.text;
            let link_words = missing_link.as_ref().map(|(_, words)| words.as_str());
            let vital_result = self
                .vital_undo
                .as_ref()
                .filter(|(path, _, after, _)| path == &note.path && after == &self.draft)
                .map(|(_, _, _, result)| result.as_str());
            let vitals = match (editable, &parsed) {
                (true, Ok(entry)) => creature::vital_checks(entry),
                _ => Vec::new(),
            };
            // Round 25 FH6: each broken link the row counts has its own line here, so the DM
            // knows where every one is. The link Campaign sent here has its own words.
            let index = ui.ctx().data(|data| {
                data.get_temp::<std::sync::Arc<crate::lookup::ContentIndex>>(egui::Id::new(
                    "reader_link_index",
                ))
            });
            // Borrowed out of the cache for this frame and put back after the footer is drawn.
            let mut broken_taken = false;
            let broken_cache: (Vec<String>, String) = match (&index, parsed) {
                (Some(index), Ok(entry)) if note.owner.editable() && !note.is_manifest() => {
                    let skip = missing_link.as_ref().map(|(at, _)| *at);
                    let fresh = self
                        .broken_lines
                        .as_ref()
                        .is_some_and(|(seen, known, at, _)| {
                            *seen == parsed_draft.revision()
                                && std::sync::Arc::ptr_eq(known, index)
                                && *at == skip
                        });
                    if !fresh {
                        let warnings = broken_link_warnings(&self.draft, entry, index, skip);
                        self.broken_lines =
                            Some((parsed_draft.revision(), index.clone(), skip, warnings));
                    }
                    broken_taken = true;
                    self.broken_lines
                        .as_mut()
                        .map_or_else(Default::default, |(_, _, _, warnings)| {
                            std::mem::take(warnings)
                        })
                }
                _ => Default::default(),
            };
            let broken = &broken_cache.0;
            // In Read with a changed draft too: one list of kept results per footer.
            let results_shown = results.any();
            let repair = recovery.as_ref().filter(|_| note.error.is_some());
            let waits = repair_problem.is_none()
                && failed.is_none()
                && link_words.is_none()
                && broken.is_empty()
                && heading.fix.is_none()
                && vitals.is_empty()
                && !discard_shown
                && heading.undo.is_none()
                && vital_result.is_none()
                && !results_shown;
            let reason_tone = recovery
                .as_ref()
                .filter(|_| note.error.is_some())
                .map_or(Tone::Error, |(_, _, exists)| {
                    crate::content_workspace::manifest_fault_tone(*exists)
                });
            let (mut heading_fix, mut vital_use) = (false, None);
            // Each kept result is its own footer message, with its own Undo: Discard's, the
            // heading and stat fixes', and each of the reader's results. The footer lists results
            // with a live Undo newest first, by when each action was done (`result_orders`).
            #[derive(Clone, Copy)]
            enum Kept {
                Discard,
                Heading,
                Vital,
            }
            let orders = &self.result_orders;
            let mut kept: Vec<(Kept, String, bool, design::ResultOrder)> = Vec::new();
            if discard_shown {
                kept.push((
                    Kept::Discard,
                    format!("{} draft discarded.", note.name()),
                    true,
                    order_of(orders, OrderKey::Reader(ResultKind::Discarded)),
                ));
            }
            if let Some(result) = heading.undo.as_deref() {
                kept.push((
                    Kept::Heading,
                    result.to_owned(),
                    true,
                    order_of(orders, OrderKey::Heading),
                ));
            }
            if let Some(result) = vital_result {
                kept.push((
                    Kept::Vital,
                    result.to_owned(),
                    true,
                    order_of(orders, OrderKey::Vital),
                ));
            }
            let (discard_undo, heading_undo, vital_undo) = (
                std::cell::Cell::new(false),
                std::cell::Cell::new(false),
                std::cell::Cell::new(false),
            );
            let shown = std::cell::RefCell::new(None::<ResultPress>);
            let mut footer = design::Footer::new("library_footer", editor_width);
            for (which, words, undo, order) in &kept {
                let which = *which;
                let (discard_undo, heading_undo, vital_undo) =
                    (&discard_undo, &heading_undo, &vital_undo);
                let heading_result = heading.undo.as_deref();
                // The fixes' results are messages, so each Undo keeps its icon on the line.
                footer.result(*order, words.as_str(), *undo, move |ui| match which {
                    Kept::Discard => discard_undo.set(design::draft_discarded(ui, note.name())),
                    Kept::Heading => heading_undo.set(heading_result.is_some_and(|result| {
                        design::undo_message(ui, Tone::Success, result, "Undo heading change")
                    })),
                    Kept::Vital => vital_undo.set(vital_result.is_some_and(|result| {
                        design::undo_message(ui, Tone::Success, result, "Undo stat change")
                    })),
                });
            }
            // The reader's results, each with its own order (a link offer's and a link result's
            // from their stores).
            if results_shown {
                for item in results.items() {
                    let (kind, results, shown) = (item.0, &results, &shown);
                    add_reader_result(&mut footer, results, orders, item, move |ui| {
                        let mut shown = shown.borrow_mut();
                        show_result(
                            ui,
                            results,
                            kind,
                            shown.get_or_insert_with(Default::default),
                        );
                    });
                }
            }
            // A repair draft's fault in its tone (a deleted manifest is a warning), or Save's
            // refusal before the write, a YAML fault in its own words.
            if let Some(problem) = &repair_problem {
                footer.reason(problem.as_str(), move |ui| {
                    design::message(ui, reason_tone, problem);
                });
            }
            // A broken manifest's recovery depends on the saved file only, so it stays while
            // the DM edits.
            if let Some(repair) = repair {
                let pressed = &mut repair_pressed;
                footer.reason("", |ui| {
                    *pressed = recovery_action(ui, repair, draft_reason);
                });
            }
            if let Some(failure) = &failed {
                // The disk's failure of the last Save of this draft.
                footer.reason(failure.as_str(), |ui| {
                    design::message(ui, Tone::Error, failure);
                });
            }
            // The repair this note was opened for.
            if let Some(words) = link_words {
                footer.notice(words, |ui| {
                    design::message(ui, Tone::Warning, words);
                });
            }
            for words in broken {
                footer.notice(words.as_str(), |ui| {
                    design::message(ui, Tone::Warning, words);
                });
            }
            if let Some((words, action, _)) = heading.fix.as_ref() {
                let fix = &mut heading_fix;
                footer.notice(words.as_str(), |ui| {
                    *fix = design::message_with_action(ui, Tone::Warning, words, action);
                });
            }
            if !vitals.is_empty() {
                let vital_use = &mut vital_use;
                let words = vitals
                    .iter()
                    .map(|(_, _, _, words)| words.as_str())
                    .collect::<Vec<_>>()
                    .join(" ");
                let vitals = &vitals;
                footer.notice(words, move |ui| {
                    for (key, label, value, message) in vitals {
                        if design::message_with_action(
                            ui,
                            Tone::Warning,
                            message,
                            &format!("Use {value}"),
                        ) {
                            *vital_use = Some((*key, *label, *value));
                        }
                    }
                });
            }
            if waits && let Some(reason) = draft_reason {
                // Round 27 FA3: the footer's message form (bar and icon), as its other
                // messages have.
                footer.notice(reason, move |ui| {
                    design::message(ui, Tone::Info, reason);
                });
            }
            // Round 26 FC4: the duplicate warning and its choices join Edit Markdown's messages,
            // with the campaign links in Read. The kept results and the heading's warning and
            // result are already in the list above.
            reader_messages(
                &mut footer,
                None,
                &no_results,
                Some((self.loaded.as_ref().unwrap(), note, &duplicate_indices)),
                draft_reason,
                &dependencies,
                &dependency_results,
                &self.result_orders,
                &mut reader_press,
            );
            page_error(&mut footer, self.error.as_deref());
            body_bottom = footer.show_by_hand(ui);
            let (discard_undo, heading_undo, vital_undo, shown) = (
                discard_undo.get(),
                heading_undo.get(),
                vital_undo.get(),
                shown.into_inner(),
            );
            if broken_taken && let Some((_, _, _, warnings)) = &mut self.broken_lines {
                *warnings = broken_cache;
            }
            if let Some(shown) = shown {
                pressed = shown;
            }
            heading_pressed = (heading_fix, heading_undo);
            if discard_undo && let Some((_, _, text)) = self.discarded.take() {
                self.draft = text;
                // The draft changed: a failed Save of the old words no longer owns it.
                self.draft_revision += 1;
            }
            // Applied now, before the editor shows the draft.
            if heading_pressed.0 || heading_pressed.1 {
                self.draft_revision += 1;
            }
            let fixed = heading_pressed.0;
            save |= heading.apply(
                std::mem::take(&mut heading_pressed),
                &note.path,
                &mut self.draft,
                &mut self.heading_undo,
            );
            if fixed && self.heading_undo.is_some() {
                self.result_orders
                    .insert(OrderKey::Heading, design::ResultOrder::issue());
            }
            if let (Some((key, label, value)), Ok(entry)) = (vital_use, &parsed) {
                match creature::use_vital(&self.draft, entry, key, value) {
                    Ok(after) => {
                        self.result_orders
                            .insert(OrderKey::Vital, design::ResultOrder::issue());
                        self.vital_undo = Some((
                            note.path.clone(),
                            self.draft.clone(),
                            after.clone(),
                            format!("The stat block now says {label} {value}."),
                        ));
                        self.draft = after;
                        // The draft changed: a failed Save of the old words no longer owns it.
                        self.draft_revision += 1;
                    }
                    Err(error) => self.error = Some(error),
                }
            }
            if vital_undo && let Some((_, before, _, _)) = self.vital_undo.take() {
                self.draft = before;
                // The draft changed: a failed Save of the old words no longer owns it.
                self.draft_revision += 1;
            }
        }
        // Live, because Discard in this same frame changed the draft.
        if self.draft != note.text {
            self.discarded = None;
        }
        // In Read, the discard result sits with the reader's other results, in the footer.
        if !save_row && discard_is_for(self.discarded.as_ref(), note) {
            if choices_in_reader {
                results.discarded = Some(note.name().to_owned());
            } else if design::draft_discarded(ui, note.name())
                && let Some((_, _, text)) = self.discarded.take()
            {
                self.draft = text;
                // The draft changed: a failed Save of the old words no longer owns it.
                self.draft_revision += 1;
            }
        }
        // In Read the pane draws the body and then this footer (`Footer::pane`), so the footer
        // follows a short body; its presses are read after the pane (round 28, row G6).
        let mut read_footer = None;
        if choices_in_reader && !slot_row {
            let mut footer =
                design::Footer::new("library_footer", editor_width.min(design::READER_MEASURE));
            reader_messages(
                &mut footer,
                Some(&heading),
                &results,
                Some((self.loaded.as_ref().unwrap(), note, &duplicate_indices)),
                draft_reason,
                &dependencies,
                &dependency_results,
                &self.result_orders,
                &mut reader_press,
            );
            page_error(&mut footer, self.error.as_deref());
            read_footer = Some(footer);
        }
        let mut reload_from_disk = false;
        let mut read_disk_again = false;
        if let Some(fault) = self
            .disk_change
            .as_ref()
            .and_then(|change| change.fault.clone())
        {
            // The file is there but cannot be read. Saving the draft over it would replace bytes
            // Splat never read, so the choices are to read it again or to let the draft go.
            let (again, reload) = design::message_with_actions(
                ui,
                Tone::Error,
                &format!(
                    "{} Your draft is safe here until you choose.",
                    fault_here(&fault)
                ),
                |ui| {
                    recovery_paragraph(ui, &fault);
                    ui.horizontal_wrapped(|ui| {
                        let again = Action::secondary("Try again")
                            .density(Density::Read)
                            .show(ui)
                            .clicked();
                        design::gap(ui, design::Gap::S); // With the line's item gap (`S`): `L`.
                        let reload = Action::destructive("Reload and lose my draft")
                            .density(Density::Read)
                            .show(ui)
                            .clicked();
                        (again, reload)
                    })
                    .inner
                },
            );
            design::gap(ui, design::Gap::S);
            design::quiet_section(ui, "Technical details", |ui| {
                ui.add(
                    egui::Label::new(Type::Small.rich(ui, &fault.system).color(colors().muted))
                        .wrap(),
                );
            });
            read_disk_again = again;
            reload_from_disk = reload;
        } else if let Some(change) = &mut self.disk_change {
            let text = if change.moved_to.is_some() {
                MOVED_ON_DISK
            } else if change.text.is_some() {
                "This entry changed on disk. Your draft is safe until you choose."
            } else {
                "This entry’s file was deleted on disk. Your draft is safe until you choose."
            };
            let names = if change.text.is_none() {
                self.loaded.as_ref().unwrap().dependents(note)
            } else {
                Vec::new()
            };
            // The note and the choices start at the message text, as in World and the run.
            let (overwrite, reload) = design::message_with_actions(ui, Tone::Warning, text, |ui| {
                if !names.is_empty() {
                    ui.label(
                        Type::Small
                            .rich(ui, format!("{} needs this entry.", name_list(&names)))
                            .color(colors().muted),
                    );
                }
                ui.horizontal_wrapped(|ui| {
                    let save_word = change.moved_to.as_ref().map_or_else(
                        || {
                            if change.text.is_none() {
                                "Save my draft again".to_owned()
                            } else {
                                "Overwrite with my draft".to_owned()
                            }
                        },
                        |path| format!("Save my draft to {path}"),
                    );
                    let overwrite = Action::secondary(&save_word)
                        .density(Density::Read)
                        .show(ui)
                        .clicked();
                    // With the line's item gap (`S`), `L` before the destructive choice.
                    design::gap(ui, design::Gap::S);
                    let reload = Action::destructive(if change.text.is_none() {
                        "Remove entry and lose draft"
                    } else {
                        "Reload and lose my draft"
                    })
                    .density(Density::Read)
                    .show(ui)
                    .clicked();
                    (overwrite, reload)
                })
                .inner
            });
            self.overwrite_disk |= overwrite;
            reload_from_disk = reload;
        }
        let mut open_link = None;
        let mut pressed_link = None;
        if self.source && self.disk_change.is_none() {
            egui::Frame::new()
                .fill(colors().well)
                .stroke(egui::Stroke::new(
                    design::HAIRLINE_WIDTH,
                    colors().field_border,
                ))
                .corner_radius(design::RADIUS_CONTROL)
                .inner_margin(design::S)
                .show(ui, |ui| {
                    ui.set_max_width(editor_width - (design::S + design::HAIRLINE_WIDTH) * 2.0);
                    // The editor ends above the message footer, inside this frame's margin.
                    let visible_height = ui
                        .available_height()
                        .min(body_bottom - design::S - design::HAIRLINE_WIDTH - ui.cursor().top())
                        .max(design::XL);
                    egui::ScrollArea::vertical()
                        .id_salt(("library_source", note.owner.id(), &note.path))
                        .max_height(visible_height)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            let width = ui.available_width();
                            let output =
                                note.markdown_field().editor(ui, &mut self.draft, |edit| {
                                    edit.font(egui::TextStyle::Monospace)
                                        .frame(false)
                                        .margin(egui::Vec2::ZERO)
                                        .desired_width(width)
                                        .min_size(egui::vec2(0.0, visible_height))
                                        .interactive(editable)
                                });
                            // Each edit is a new draft: a failed Save of the old words ends.
                            if output.response.changed() {
                                self.draft_revision += 1;
                            }
                            pressed_link = crate::markdown_source::pressed_note_link(
                                ui,
                                &output,
                                note.entry.as_ref().map(|entry| entry.id),
                            );
                            let response = output.response;
                            design::named(ui, &response, "Markdown editor");
                            if std::mem::take(&mut self.focus_markdown) {
                                response.request_focus();
                            }
                            if std::mem::take(&mut self.focus_repair)
                                && let Some(line) = repair_line
                            {
                                let at = self
                                    .draft
                                    .lines()
                                    .take(line.saturating_sub(1))
                                    .map(|line| line.chars().count() + 1)
                                    .sum();
                                response.request_focus();
                                let row =
                                    output.galley.pos_from_cursor(egui::text::CCursor::new(at));
                                ui.scroll_to_rect(
                                    row.translate(output.galley_pos.to_vec2()),
                                    Some(egui::Align::Center),
                                );
                                let mut state = egui::TextEdit::load_state(ui.ctx(), response.id)
                                    .unwrap_or_default();
                                state
                                    .cursor
                                    .set_char_range(Some(egui::text::CCursorRange::one(
                                        egui::text::CCursor::new(at),
                                    )));
                                state.store(ui.ctx(), response.id);
                            }
                            if std::mem::take(&mut self.focus_hp)
                                && let Some(at) = self
                                    .draft
                                    .find("initiative: ")
                                    .map(|at| at + "initiative: ".len())
                            {
                                response.request_focus();
                                let at = self.draft[..at].chars().count();
                                let mut state = egui::TextEdit::load_state(ui.ctx(), response.id)
                                    .unwrap_or_default();
                                state
                                    .cursor
                                    .set_char_range(Some(egui::text::CCursorRange::two(
                                        egui::text::CCursor::new(at),
                                        egui::text::CCursor::new(at + 1),
                                    )));
                                state.store(ui.ctx(), response.id);
                            }
                        });
                });
        } else {
            // The body's words, in Read (in the pane) and beside an open conflict (in its own
            // scroll area, as Edit Markdown's form keeps until the pane part P2).
            let mut content = |ui: &mut egui::Ui| {
                if let Some(change) = &mut self.disk_change {
                    if let Some(text) = &change.text {
                        // The side-by-side comparison is the one state that takes the full width.
                        ui.set_max_width(editor_width);
                        design::grouped_draft_comparison(
                            ui,
                            &self.draft,
                            text,
                            (&note.path, text),
                            true,
                        );
                    } else {
                        design::heading(
                            ui,
                            design::Heading::Group {
                                title: "Your draft",
                                count: 0,
                            },
                        );
                        let mut draft = self.draft.as_str();
                        // The field belongs to this note: its owner and path key it.
                        let field = DISK_DRAFT.of(note.owner.id()).with(note.field_key());
                        let response = field.multi_line(ui, &mut draft, |edit| {
                            edit.font(egui::TextStyle::Monospace)
                                .margin(egui::vec2(design::S, design::S))
                                .desired_width(editor_width - 2.0 * design::S)
                                .interactive(false)
                        });
                        design::named(ui, &response, "Your draft");
                    }
                    return;
                }
                // A failed parse must never render YAML properties as book prose.
                let body = parsed.as_ref().map_or("", |e| e.body.as_str());
                ui.push_id(&note.path, |ui| {
                    reader_style::measure(ui, |ui| {
                        design::gap(ui, design::Gap::XS);
                        let reading_subtitle = reader_kind_line(&note.owner, &reading_subtitle);
                        let subtitle = reading_subtitle.as_str();
                        let subtitle = subtitle.strip_suffix(" · Campaign").unwrap_or(subtitle);
                        let subtitle = if note.error.is_some() {
                            match note.unreadable_kind() {
                                Section::Unreadable | Section::Other => "",
                                Section::PackageDetails => "Package",
                                kind => kind.label().trim_end_matches('s'),
                            }
                        } else if subtitle == "Campaign" {
                            note.section().label().trim_end_matches('s')
                        } else {
                            subtitle
                        };
                        view::title_with_art(ui, title, subtitle, parsed.as_ref().ok());
                        // Round 27 FH9: `L` between the kind line and its
                        // first warning, as between a kind line and prose,
                        // so the kind word does not read as the warning's.
                        if fault.is_some() || repair_problem.is_some() {
                            ui.add_space(design::L - ui.spacing().item_spacing.y);
                        }
                        if let Some(fault) = &fault {
                            read_again = fault_notice(ui, fault);
                        }
                        // A read fault has no repair problem: there is no text to repair.
                        if let Some(problem) = &repair_problem {
                            repair_pressed =
                                repair_notice(ui, problem, recovery.as_ref(), draft_reason, false);
                        }
                    });
                    open_link = view::show_with_related(
                        ui,
                        note.name(),
                        parsed.as_ref().ok(),
                        body,
                        &view::Origin {
                            path: &note.path,
                            owner: &note.owner.label(),
                            builtin: note.owner == Owner::Builtin,
                        },
                        self.loaded.as_ref().unwrap(),
                        // The Library draws no value block above the entry.
                        false,
                        // A package entry's campaign links sit in the
                        // pane's message footer (round 26).
                        |_| None,
                    );
                });
            };
            // The body takes the whole pane, so a wide table can use it; prose keeps its
            // measure. A draft's comparison keeps the pane it had.
            let right = pane_right
                .filter(|_| self.disk_change.is_none())
                .unwrap_or(ui.max_rect().right())
                .max(ui.max_rect().right());
            if !slot_row {
                // Read: the pane draws the body, then its message footer `S` below a short body,
                // or pinned when the body scrolls (round 28 rows 17, 50, 53, 54; G6).
                let footer = read_footer.take().unwrap_or_else(|| {
                    design::Footer::new("library_footer", editor_width.min(design::READER_MEASURE))
                });
                let body = egui::Rect::from_min_max(
                    ui.cursor().min,
                    egui::pos2(right, ui.max_rect().bottom()),
                );
                ui.scope_builder(egui::UiBuilder::new().max_rect(body), |ui| {
                    crate::markdown_view::with_wide_tables(ui, |ui| {
                        footer.pane(ui, design::Body::Content, |ui| content(ui));
                    });
                });
            } else {
                // It starts `S` left of the text, with an `S` margin, so a quiet `line_start`
                // action that hangs its icon left of the message words is not clipped at the
                // pane's edge. The text stays where it was.
                let body = egui::Rect::from_min_max(
                    egui::pos2(ui.cursor().min.x - design::S, ui.cursor().min.y),
                    egui::pos2(right, ui.max_rect().bottom().min(body_bottom)),
                );
                ui.scope_builder(egui::UiBuilder::new().max_rect(body), |ui| {
                    crate::markdown_view::with_wide_tables(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .id_salt(("entry_body", note.owner.id(), &note.path))
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                egui::Frame::new()
                                    .inner_margin(egui::Margin {
                                        left: design::S as i8,
                                        ..Default::default()
                                    })
                                    .show(ui, |ui| content(ui));
                            });
                    });
                });
            }
        }
        // The Read footer's presses, after the pane drew it.
        if choices_in_reader && !slot_row {
            heading_pressed = reader_press.heading;
            // Presses come only from the footer that drew the results: this one in Read. With a
            // save row, Edit Markdown's list drew them and `pressed` already holds them.
            pressed = reader_press.results.get();
        }
        new_identity = new_identity.or(reader_press.new_identity.take());
        new_identities = new_identities.or(reader_press.new_identities.take());
        let copy_dependency = reader_press.copy_dependency;
        let undo_dependency = reader_press.undo_dependency.get();
        if heading_pressed.0 || heading_pressed.1 {
            self.draft_revision += 1;
        }
        let fixed = heading_pressed.0;
        save |= heading.apply(
            heading_pressed,
            &note.path,
            &mut self.draft,
            &mut self.heading_undo,
        );
        if fixed && self.heading_undo.is_some() {
            self.result_orders
                .insert(OrderKey::Heading, design::ResultOrder::issue());
        }
        let undo_identities = pressed.identities;
        let undo_identity = pressed.identity;
        if pressed.discarded
            && let Some((_, _, text)) = self.discarded.take()
        {
            self.draft = text;
            // The draft changed: a failed Save of the old words no longer owns it.
            self.draft_revision += 1;
        }
        if repair_pressed {
            if let Some((module, path, exists)) = recovery {
                match crate::content_workspace::ManifestWrite::write_as(&module, &path, exists) {
                    Ok(write) => {
                        self.result_orders.insert(
                            OrderKey::Reader(ResultKind::Manifest),
                            design::ResultOrder::issue(),
                        );
                        self.manifest_write = Some(write);
                        self.stale = true;
                        self.content_changed = Some(module.path);
                    }
                    Err(error) => self.error = Some(error),
                }
            } else {
                self.source = true;
                self.focus_repair = true;
            }
            return;
        }
        let reconnect = (|| -> Result<(), String> {
            if let Some(at) = undo_dependency
                && let Some(DependencyUndo {
                    root,
                    path,
                    before,
                    after,
                    copied_path,
                    copied_text,
                    ..
                }) = self.dependency_undos.get(at)
            {
                let copied = note_path(root, copied_path)?;
                if fs::read_to_string(&copied)
                    .map_err(|e| FileFault::read(copied_path, &e).message())?
                    != *copied_text
                {
                    return Err(
                        "The copy changed on disk. Keep it, and change the link in Markdown."
                            .into(),
                    );
                }
                write_note(root, path, Some(after), before)?;
                if let Err(error) = fs::remove_file(copied) {
                    let _ = write_note(root, path, Some(before), after);
                    return Err(error.to_string());
                }
                self.content_changed = Some(root.clone());
                self.dependency_undos.remove(at);
                self.stale = true;
            }
            if let Some(id) = copy_dependency
                && let Owner::Package(package) = &note.owner
                && let Some(source) = self
                    .loaded
                    .as_ref()
                    .unwrap()
                    .notes
                    .iter()
                    .find(|n| n.entry.as_ref().is_some_and(|e| e.id == id))
            {
                let source = crate::content_workspace::CopySource {
                    entry: source.entry.clone().unwrap(),
                    text: source.text.clone(),
                    root: source.owner.root().map(Path::to_path_buf),
                    campaign: None,
                };
                let catalogue = crate::content_workspace::Catalogue::load(
                    campaigns.modules(),
                    campaigns.active_campaign().as_ref(),
                )?;
                let plan = crate::content_workspace::CopyPlan::prepare(
                    &source,
                    &package.path,
                    None,
                    &source.entry.name,
                    "",
                    &catalogue,
                )?;
                let copied = plan.save()?;
                let after = crate::content_workspace::redirect_reference(
                    &note.text,
                    &source.entry,
                    copied.id,
                )?;
                if let Err(error) = write_note(&package.path, &note.path, Some(&note.text), &after)
                {
                    let _ = fs::remove_file(package.path.join(&plan.relative));
                    return Err(error);
                }
                self.dependency_undos.push(DependencyUndo {
                    root: package.path.clone(),
                    path: note.path.clone(),
                    before: note.text.clone(),
                    after,
                    copied_path: plan.relative.clone(),
                    copied_text: plan.text.clone(),
                    result: format!(
                        "{} copied to {} and linked.",
                        source.entry.name, package.name
                    ),
                    order: design::ResultOrder::issue(),
                });
                self.content_changed = Some(package.path.clone());
                self.stale = true;
            }
            Ok(())
        })();
        if let Err(error) = reconnect {
            self.error = Some(error);
        }
        if reload_from_disk {
            self.reload_note_from_disk(i);
            return;
        }
        // Try again reads the file now. With no draft, every file is read again and the note opens
        // with its text or its new fault. With a draft (after Undo discard), the file is compared
        // with it as any outside change is: a load waits while a draft is open.
        if (read_again || read_disk_again)
            && let Some(root) = note.owner.root().map(Path::to_path_buf)
        {
            self.refresh_destination(&root, ui.ctx());
            return;
        }
        if let Some(target) =
            pressed_link.or_else(|| crate::markdown_view::take_note_link(ui.ctx()))
            && let Some(index) = ui.ctx().data(|data| {
                data.get_temp::<std::sync::Arc<crate::lookup::ContentIndex>>(egui::Id::new(
                    "reader_link_index",
                ))
            })
        {
            let from = note.entry.as_ref().map(|entry| entry.id);
            let choices = match index.resolve_link(&target, from) {
                crate::world::Go::Item(at, heading) => {
                    crate::markdown_view::want_heading(ui.ctx(), heading);
                    vec![index.items[at].entry.id]
                }
                crate::world::Go::Many(positions) => positions
                    .iter()
                    .map(|&at| index.items[at].entry.id)
                    .collect(),
                _ => Vec::new(),
            };
            self.link_choices = choices;
        }
        if self.link_choices.len() == 1 {
            let id = self.link_choices.remove(0);
            open_link = self
                .loaded
                .as_ref()
                .unwrap()
                .notes
                .iter()
                .position(|n| n.entry.as_ref().is_some_and(|e| e.id == id));
            if open_link.is_none() {
                self.peek = Some(id);
            }
        } else if !self.link_choices.is_empty() {
            ui.horizontal_wrapped(|ui| {
                for id in &self.link_choices {
                    if let Some((at, note)) = self
                        .loaded
                        .as_ref()
                        .unwrap()
                        .notes
                        .iter()
                        .enumerate()
                        .find(|(_, n)| n.entry.as_ref().is_some_and(|e| e.id == *id))
                        && design::link(ui, &format!("{} · {}", note.name(), note.owner.label()))
                            .clicked()
                    {
                        open_link = Some(at);
                    }
                }
            });
            if open_link.is_some() {
                self.link_choices.clear();
            }
        }
        if let Some(module) = use_every
            && let Some(campaign) = campaigns.active_campaign()
        {
            let result =
                crate::content_workspace::CampaignDocument::load(&campaign).and_then(|before| {
                    crate::content_workspace::use_in_campaign(
                        &campaign,
                        module.id,
                        &module.name,
                        None,
                    )?;
                    let after = crate::content_workspace::CampaignDocument::load(&campaign)?;
                    Ok(SelectionUndo {
                        campaign: campaign.clone(),
                        before: before.selections,
                        after: after.selections,
                    })
                });
            match result {
                Ok(undo) => {
                    self.result_orders.insert(
                        OrderKey::Reader(ResultKind::PackageUse),
                        design::ResultOrder::issue(),
                    );
                    self.selection_undo = Some(undo);
                    self.stale = true;
                    self.content_changed = Some(campaign.path);
                }
                Err(error) => self.error = Some(error),
            }
        }

        if save {
            self.save_selected(file_gone);
        }
        let identity_result = if let Some((root, paths)) = new_identities {
            crate::content_workspace::ContentEdits::new_identities(&root, &paths).map(|edits| {
                let folder = root.file_name().unwrap_or_default().to_string_lossy();
                self.result_orders.insert(
                    OrderKey::Reader(ResultKind::Identities),
                    design::ResultOrder::issue(),
                );
                self.identities_undo = Some((
                    edits,
                    format!(
                        "{} in {folder}/ are now separate entries.",
                        design::count(paths.len(), "file", "files")
                    ),
                ));
                self.stale = true;
                self.content_changed = Some(root);
            })
        } else if undo_identities {
            self.identities_undo.as_ref().unwrap().0.undo().map(|()| {
                self.identities_undo = None;
                self.stale = true;
                self.content_changed = campaigns.active_campaign().map(|campaign| campaign.path);
            })
        } else if let Some(at) = new_identity {
            self.open_entry(at);
            self.give_new_identity()
        } else if undo_identity {
            self.undo_new_identity()
        } else {
            Ok(())
        };
        if let Err(error) = identity_result {
            self.error = Some(error);
        }
        if let Some(index) = open_link {
            if dirty {
                // Keep the draft. The linked entry opens beside it, in Lookup.
                self.peek = self
                    .loaded
                    .as_ref()
                    .and_then(|l| l.notes.get(index))
                    .and_then(|n| n.entry.as_ref())
                    .map(|e| e.id);
            } else {
                self.open_entry(index);
                // Followed from the text, so the list may not show it yet.
                self.reveal_row = true;
            }
        }
        // One press a frame, so no early return above skips one of these.
        self.apply_package_results(&pressed);
        if let Some(step) = step {
            if dirty {
                self.peek = self.trail.peek(step).and_then(|target| {
                    let notes = &self.loaded.as_ref()?.notes;
                    notes[target.find(notes)?].entry.as_ref().map(|e| e.id)
                });
            } else {
                self.go(step);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicate_checks_keep_id_key_matches_and_refresh_after_edits() {
        // Row 115 (FH10): a new rule or effect has no heading that repeats its kind line.
        for (kind, word) in [("rule", "Rule"), ("effect", "Effect")] {
            let text = note_text("Watch Orders", kind, None);
            assert!(!text.contains(&format!("## {word}\n")), "{text}");
            assert!(text.contains("\n# Watch Orders\n\nDescribe "), "{text}");
        }
        let base = Entry::parse("Rule.md", &note_text("Rule", "rule", None)).unwrap();
        let mut loaded = Loaded::default();
        // Same ID, same key, both, no key, and unreadable notes all occur in a
        // loaded Library. A key collision must mark both distinct IDs.
        for (id, key) in [
            (1, Some("test:a")),
            (2, Some("test:a")),
            (1, Some("test:b")),
            (3, Some("test:b")),
            (1, Some("test:a")),
            (4, None),
            (5, None),
        ] {
            let mut entry = base.clone();
            entry.id = Uuid::from_u128(id);
            entry.key = key.map(str::to_owned);
            loaded.notes.push(Note {
                path: format!("{}.md", loaded.notes.len()),
                text: String::new(),
                entry: Some(entry),
                error: None,
                fault: None,
                owner: Owner::Builtin,
            });
        }
        loaded.notes.push(Note {
            path: "Broken.md".into(),
            text: String::new(),
            entry: None,
            error: Some("Invalid YAML".into()),
            fault: None,
            owner: Owner::Builtin,
        });
        let check = |loaded: &Loaded| {
            let mut ids = std::collections::HashSet::new();
            for (at, note) in loaded.notes.iter().enumerate() {
                let expected: Vec<_> = loaded
                    .notes
                    .iter()
                    .enumerate()
                    .filter_map(|(other, candidate)| {
                        let target = note.entry.as_ref()?;
                        let candidate = candidate.entry.as_ref()?;
                        (at != other
                            && (target.id == candidate.id
                                || (target.key.is_some() && target.key == candidate.key)))
                            .then_some(other)
                    })
                    .collect();
                assert_eq!(loaded.duplicates(at), expected);
                if !expected.is_empty() {
                    ids.insert(note.entry.as_ref().unwrap().id);
                }
            }
            assert_eq!(loaded.duplicate_ids(), &ids);
        };
        check(&loaded);
        for (at, note) in loaded.notes.iter_mut().enumerate() {
            if let Some(entry) = &mut note.entry {
                entry.id = Uuid::from_u128(at as u128 + 10);
                entry.key = None;
            }
        }
        loaded.invalidate();
        check(&loaded);
        assert!(loaded.duplicate_ids().is_empty());
    }

    #[test]
    fn i20_c1_stat_flags_reach_list_subtitles_and_header_count() {
        let dir = tempfile::tempdir().unwrap();
        let module = create_module(&dir.path().join("book"), "Book", "book").unwrap();
        for (name, kind, table) in [
            (
                "Wolf",
                "creature",
                "## Stat block\n\n| Stat | Value |\n| --- | --- |\n| Hit Points | about twenty |\n",
            ),
            (
                "Spark",
                "spell",
                "## Spell\n\n| Field | Value |\n| --- | --- |\n| Level | many |\n",
            ),
            (
                "Bear",
                "creature",
                "## Stat block\n\n| Stat | Value |\n| --- | --- |\n| Armor Class | thick fur |\n| Hit Points | about forty |\n",
            ),
        ] {
            fs::write(module.path.join(format!("{name}.md")), format!(
                "---\nsplat_schema: 2\nsplat_id: {}\nsplat_kind: {kind}\nsplat_name: {name}\n---\n\n{table}", Uuid::new_v4()
            )).unwrap();
        }
        let mut loaded = load(Some(&module)).unwrap();
        loaded.refresh_diagnostics();
        for name in ["Wolf", "Spark"] {
            let at = loaded
                .notes
                .iter()
                .position(|note| note.name() == name)
                .unwrap();
            assert!(loaded.subtitles()[at].contains("1 value not read"));
        }
        // FD5: the Library row uses World's words, in the plural too.
        let bear = loaded
            .notes
            .iter()
            .position(|note| note.name() == "Bear")
            .unwrap();
        assert!(
            loaded.subtitles()[bear].contains("2 values not read"),
            "{}",
            loaded.subtitles()[bear]
        );
        let mut library = Library {
            loaded: Some(loaded),
            ..Default::default()
        };
        let ctx = egui::Context::default();
        let output = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1200.0, 800.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .show(ctx, |ui| library.show(ui, &mut Campaigns::default()));
            },
        );
        assert!(output.shapes.iter().any(|shape| matches!(&shape.shape, egui::Shape::Text(text) if text.galley.job.text.contains("3 files need fixing"))));
        let diagnostics = &library.loaded.as_ref().unwrap().diagnostic_texts();
        assert!(diagnostics.iter().any(|s| s.starts_with("Wolf.md: Line ")
            && s.contains(": Hit Points “about twenty” was not read")));
        assert!(
            diagnostics
                .iter()
                .any(|s| s.starts_with("Spark.md: Line ")
                    && s.contains(": Level “many” was not read"))
        );
    }

    /// iter-12-E-FD2: an entry with a broken link counts in the header once, beside unreadable and
    /// flagged files. The scan is cached against the link index until a note changes.
    #[test]
    fn broken_links_count_in_the_header_once() {
        let dir = tempfile::tempdir().unwrap();
        let module = create_module(&dir.path().join("book"), "Book", "book").unwrap();
        let note = |name: &str, kind: &str, body: &str| {
            fs::write(
                module.path.join(format!("{name}.md")),
                format!(
                    "---\nsplat_schema: 2\nsplat_id: {}\nsplat_kind: {kind}\nsplat_name: {name}\n---\n\n{body}",
                    Uuid::new_v4()
                ),
            )
            .unwrap();
        };
        note("Linker", "rule", "See [[Gone Page]].\n");
        note("Fine", "rule", "See [[Linker]].\n");
        // A flag and a broken link in one file: one file to fix.
        note(
            "Both",
            "creature",
            "## Stat block\n\n| Stat | Value |\n| --- | --- |\n| Hit Points | about twenty |\n\n[[Gone Too]]\n",
        );
        fs::write(
            module.path.join("Unreadable.md"),
            "---\nsplat_kind: [\n---\n",
        )
        .unwrap();
        let mut loaded = load(Some(&module)).unwrap();
        loaded.refresh_diagnostics();
        let index = std::sync::Arc::new(
            crate::lookup::ContentIndex::load(std::slice::from_ref(&module), None).unwrap(),
        );
        let names = |loaded: &Loaded, notes: Vec<(usize, usize)>| -> Vec<String> {
            notes
                .into_iter()
                .map(|(at, _)| loaded.notes[at].name().to_owned())
                .collect()
        };
        assert_eq!(
            names(&loaded, loaded.broken_link_notes(&index)),
            ["Both", "Linker"]
        );
        // Cached: a changed body is not scanned again until the note is invalidated.
        let fine = loaded
            .notes
            .iter()
            .position(|n| n.name() == "Fine")
            .unwrap();
        loaded.notes[fine].entry.as_mut().unwrap().body =
            "See [[Nowhere]] and [[Nor Here]].\n".into();
        assert_eq!(loaded.broken_link_notes(&index).len(), 2);
        loaded.invalidate();
        assert_eq!(
            names(&loaded, loaded.broken_link_notes(&index)),
            ["Both", "Fine", "Linker"]
        );
        // The menu reads each count from the cache: two links in Fine, one in each other file.
        assert_eq!(
            loaded
                .broken_link_notes(&index)
                .iter()
                .map(|&(_, count)| count)
                .collect::<Vec<_>>(),
            [1, 2, 1]
        );
        // A new index finds them again, with the same notes.
        loaded.notes[fine].entry.as_mut().unwrap().body = "See [[Linker]].\n".into();
        let other = std::sync::Arc::new(
            crate::lookup::ContentIndex::load(std::slice::from_ref(&module), None).unwrap(),
        );
        assert_eq!(loaded.broken_link_notes(&other).len(), 2);

        let mut library = Library {
            loaded: Some(loaded),
            ..Default::default()
        };
        let ctx = egui::Context::default();
        ctx.data_mut(|data| data.insert_temp(egui::Id::new("reader_link_index"), other));
        let output = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1200.0, 800.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .show(ctx, |ui| library.show(ui, &mut Campaigns::default()));
            },
        );
        let painted = |words: &str| {
            output.shapes.iter().any(|shape| {
                matches!(&shape.shape, egui::Shape::Text(text) if text.galley.job.text.contains(words))
            })
        };
        // Linker, Both (flag and link) and the unreadable file: three, not four.
        assert!(painted("3 files need fixing"));
    }

    /// iter-13-E-SD1: the header counts files by owner and path. Two packages each with
    /// `Rules/Watch.md` and a broken link are two files with two rows that name their owners. A
    /// file with a problem and a broken link still counts once, and a check's problem stays with
    /// the package whose file has it.
    #[test]
    fn iter13_e_files_to_fix_count_by_owner_and_path() {
        let dir = tempfile::tempdir().unwrap();
        let ashen = create_module(&dir.path().join("ashen"), "Ashen Marches", "ashen").unwrap();
        let ember = create_module(&dir.path().join("ember"), "Ember Coast", "ember").unwrap();
        let write = |module: &Module, path: &str, kind: &str, extra: &str, body: &str| {
            let full = module.path.join(path);
            fs::create_dir_all(full.parent().unwrap()).unwrap();
            fs::write(
                full,
                format!(
                    "---\nsplat_schema: 2\nsplat_id: {}\nsplat_kind: {kind}\nsplat_name: Watch\n{extra}---\n\n{body}",
                    Uuid::new_v4()
                ),
            )
            .unwrap();
        };
        write(&ashen, "Rules/Watch.md", "rule", "", "See [[Gone Page]].\n");
        // The same path in another package: an item that needs a key no package has.
        let item = splat::content::formats::fixture("Rules/Watch.md", &format!("splat_id: {}\nsplat_kind: item\nsplat_key: ember:items/watch\n\
                 splat_name: Watch\nsplat_item: {{category: gear}}\nsplat_requires:\n  - key: nowhere:items/gone\n", Uuid::new_v4()), "\nSee [[Gone Page]].\n")
        .expect("schema-2 fixture");
        fs::create_dir_all(ember.path.join("Rules")).unwrap();
        fs::write(ember.path.join("Rules/Watch.md"), item).unwrap();
        // A flag and a broken link in one file: one file to fix.
        write(
            &ashen,
            "Both.md",
            "creature",
            "",
            "## Stat block\n\n| Stat | Value |\n| --- | --- |\n| Hit Points | about twenty |\n\n[[Gone Too]]\n",
        );
        let modules = [ashen.clone(), ember.clone()];
        let loaded = load_all(&modules, None).unwrap();
        let index = std::sync::Arc::new(crate::lookup::ContentIndex::load(&modules, None).unwrap());
        let broken = loaded.broken_link_notes(&index);
        let watch = |module: &Module| {
            loaded
                .notes
                .iter()
                .find(|n| n.owner.id() == Some(module.id) && n.path == "Rules/Watch.md")
                .unwrap()
        };
        let both = loaded.notes.iter().find(|n| n.path == "Both.md").unwrap();
        // Both Watch files and Both.md have a broken link.
        let mut broken_files: Vec<_> = broken
            .iter()
            .map(|&(at, _)| loaded.file_label(&loaded.notes[at]))
            .collect();
        broken_files.sort();
        assert_eq!(
            broken_files,
            [
                "Ashen Marches · Rules/Watch.md",
                "Both.md",
                "Ember Coast · Rules/Watch.md"
            ]
        );
        // The check's problem goes to Ember Coast's file only, and its row names the owner.
        let unresolved: Vec<_> = loaded
            .diagnostics
            .iter()
            .filter(|d| d.text.contains("nowhere:items/gone"))
            .collect();
        assert_eq!(unresolved.len(), 1, "{:?}", loaded.diagnostic_texts());
        assert_eq!(
            unresolved[0].file,
            (Some(ember.id), "Rules/Watch.md".to_owned())
        );
        assert!(
            unresolved[0]
                .text
                .starts_with("Ember Coast · Rules/Watch.md: "),
            "{}",
            unresolved[0].text
        );
        assert!(loaded.has_problem(watch(&ember)));
        assert!(!loaded.has_problem(watch(&ashen)));
        // A path only one note has keeps its plain row.
        assert!(loaded.has_problem(both));
        assert!(
            loaded
                .diagnostic_texts()
                .iter()
                .any(|text| text.starts_with("Both.md: Line") && text.contains(": Hit Points"))
        );
        // Two Watch files and Both.md: three, not two (by path) and not five (by problem).
        assert_eq!(loaded.files_to_fix(&broken, None), 3);
        // Round 24 FH5 (must stay true): a file with a flag beside its broken link, and a file
        // whose requirement needs a catalogue match, keep `Needs fixing` for the fault the link
        // count does not name.
        assert!(!loaded.link_faults_only.contains(&Loaded::file_key(both)));
        assert!(
            !loaded
                .link_faults_only
                .contains(&Loaded::file_key(watch(&ember)))
        );
        assert!(
            !loaded.unreadable_counted,
            "warnings only: the count stays amber"
        );

        let mut library = Library {
            loaded: Some(loaded),
            ..Default::default()
        };
        let ctx = egui::Context::default();
        ctx.data_mut(|data| data.insert_temp(egui::Id::new("reader_link_index"), index));
        let output = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1200.0, 800.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .show(ctx, |ui| library.show(ui, &mut Campaigns::default()));
            },
        );
        assert!(output.shapes.iter().any(|shape| matches!(&shape.shape, egui::Shape::Text(text) if text.galley.job.text.contains("3 files need fixing"))));

        // iter-13-E2 review 1: a second package with the same name and path. Its rows name the
        // folder, so the two Ashen Marches rows differ; Ember Coast keeps its plain owner row.
        let copy =
            create_module(&dir.path().join("ashen-copy"), "Ashen Marches", "ashcopy").unwrap();
        write(&copy, "Rules/Watch.md", "rule", "", "See [[Gone Page]].\n");
        let modules = [ashen.clone(), ember.clone(), copy.clone()];
        let loaded = load_all(&modules, None).unwrap();
        let index = std::sync::Arc::new(crate::lookup::ContentIndex::load(&modules, None).unwrap());
        let broken = loaded.broken_link_notes(&index);
        let mut broken_files: Vec<_> = broken
            .iter()
            .map(|&(at, _)| loaded.file_label(&loaded.notes[at]))
            .collect();
        broken_files.sort();
        assert_eq!(
            broken_files,
            [
                "Ashen Marches · ashen-copy/Rules/Watch.md",
                "Ashen Marches · ashen/Rules/Watch.md",
                "Both.md",
                "Ember Coast · Rules/Watch.md"
            ]
        );
        assert_eq!(loaded.files_to_fix(&broken, None), 4);

        // Two packages with the same name that both fail to load are two problems, and their
        // messages name the folders.
        let gone = [
            create_module(&dir.path().join("lost-one"), "Lost Book", "lostone").unwrap(),
            create_module(&dir.path().join("lost-two"), "Lost Book", "losttwo").unwrap(),
        ];
        for module in &gone {
            fs::remove_dir_all(&module.path).unwrap();
        }
        let loaded = load_all(&gone, None).unwrap();
        assert_eq!(
            loaded.files_to_fix(&[], None),
            2,
            "{:?}",
            loaded.diagnostic_texts()
        );
        let texts = loaded.diagnostic_texts();
        assert!(
            texts
                .iter()
                .any(|text| text.starts_with("Lost Book · lost-one/: ")),
            "{texts:?}"
        );
        assert!(
            texts
                .iter()
                .any(|text| text.starts_with("Lost Book · lost-two/: ")),
            "{texts:?}"
        );
    }

    /// iter-13-E-FD1: the About row warns for any problem the header counts in the manifest, such
    /// as a first heading that differs from the name. A clean manifest has no warning.
    #[test]
    fn iter13_e_about_row_warns_for_a_counted_manifest_problem() {
        let dir = tempfile::tempdir().unwrap();
        let module = create_module(&dir.path().join("ember"), "Emberfall", "ember").unwrap();
        let manifest_path = module.path.join("Package.md");
        let clean = fs::read_to_string(&manifest_path).unwrap();
        let about = |loaded: &Loaded| {
            let manifest = loaded.notes.iter().find(|n| n.is_manifest()).unwrap();
            manifest.about_label(loaded.has_problem(manifest))
        };
        let loaded = load_all(std::slice::from_ref(&module), None).unwrap();
        assert_eq!(loaded.diagnostic_texts(), Vec::<&str>::new());
        assert_eq!(about(&loaded), "About Emberfall");
        let heading = clean
            .lines()
            .find(|line| line.starts_with("# "))
            .expect("the manifest has a first heading");
        fs::write(&manifest_path, clean.replacen(heading, "# Ember Notes", 1)).unwrap();
        let loaded = load_all(std::slice::from_ref(&module), None).unwrap();
        let manifest = loaded.notes.iter().find(|n| n.is_manifest()).unwrap();
        // The header counts it, and the manifest still reads: only the counted problem warns.
        assert!(manifest.error.is_none());
        assert_eq!(loaded.files_to_fix(&[], None), 1);
        assert_eq!(about(&loaded), format!("{} About Emberfall", icons::WARN));
    }

    #[test]
    fn copy_registration_and_undo_keep_campaign_choices_and_later_edits() {
        let temp = tempfile::tempdir().unwrap();
        let world = crate::visual_tests::fixture::World::build(
            temp.path(),
            &temp.path().join("cache"),
            "copy-use",
            crate::visual_tests::fixture::Seed::Campaign,
        )
        .unwrap();
        let mut app = world.app();
        let original = app.campaigns.modules()[0].clone();
        let campaign = app.campaigns.active_campaign().unwrap();
        let campaign_before = CampaignDocument::load(&campaign).unwrap().text;
        let copied = temp.path().join("copied");
        fs::create_dir(&copied).unwrap();
        for entry in content::read_directory(&original.path).unwrap().0 {
            let path = copied.join(&entry.path);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::copy(original.path.join(&entry.path), path).unwrap();
        }
        for used in [false, true] {
            let mut library = Library {
                copied_package: Some(local_module(&copied).unwrap()),
                copy_name: "Reaches".into(),
                copy_not_used: !used,
                ..Default::default()
            };
            library.register_copy(&mut app.campaigns).unwrap();
            let id = library.package_copy.as_ref().unwrap().module.id;
            let document = CampaignDocument::load(&campaign).unwrap();
            assert_eq!(
                document
                    .selections
                    .iter()
                    .any(|selection| selection.package.id == Some(id)),
                used
            );
            assert!(
                document
                    .selections
                    .iter()
                    .any(|selection| selection.package.id == Some(original.id))
            );
            let path = copied.join("Module.md");
            let manifest = fs::read_to_string(&path).unwrap();
            fs::write(&path, format!("{manifest}\nLater edit.\n")).unwrap();
            assert!(
                library
                    .undo_package_registration(&mut app.campaigns)
                    .is_err()
            );
            assert_eq!(
                CampaignDocument::load(&campaign).unwrap().text,
                document.text
            );
            fs::write(path, manifest).unwrap();
            library
                .undo_package_registration(&mut app.campaigns)
                .unwrap();
            assert_eq!(
                CampaignDocument::load(&campaign).unwrap().text,
                campaign_before
            );
            assert_eq!(app.campaigns.modules(), std::slice::from_ref(&original));
        }
    }

    fn navigation_library() -> Library {
        let notes = ["First", "Second", "Destination"]
            .into_iter()
            .map(|name| {
                let text = note_text(name, "rule", None);
                Note {
                    path: if name == "Destination" {
                        "Destination.md".into()
                    } else {
                        "Rule.md".into()
                    },
                    entry: Some(Entry::parse("Rule.md", &text).unwrap()),
                    text,
                    error: None,
                    fault: None,
                    owner: Owner::Package(Module {
                        id: Uuid::new_v4(),
                        name: name.into(),
                        path: name.into(),
                    }),
                }
            })
            .collect();
        Library {
            loaded: Some(Loaded {
                notes,
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    #[test]
    fn iter2_d_copied_folder_blocks_add_until_the_path_changes() {
        use egui_kittest::kittest::{NodeT, Queryable};
        let folder = tempfile::tempdir().unwrap();
        let other = tempfile::tempdir().unwrap();
        let library = Library {
            path: folder.path().display().to_string(),
            copied_package: Some(Module {
                id: Uuid::new_v4(),
                name: "Copied package".into(),
                path: folder.path().canonicalize().unwrap(),
            }),
            ..Default::default()
        };
        let mut campaigns = Campaigns::default();
        let mut harness = egui_kittest::Harness::new_ui_state(
            move |ui, library| library.package_form(ui, &mut campaigns),
            library,
        );
        design::theme::install(&harness.ctx);
        harness.run_steps(2);
        assert!(
            harness
                .get_by_label("Add folder")
                .accesskit_node()
                .is_disabled()
        );
        harness.get_by_label("Package folder").focus();
        harness.key_press_modifiers(egui::Modifiers::CTRL, egui::Key::A);
        harness
            .get_by_label("Package folder")
            .type_text(&other.path().display().to_string());
        harness.run_steps(2);
        assert!(
            !harness
                .get_by_label("Add folder")
                .accesskit_node()
                .is_disabled()
        );
        assert!(harness.state().copied_package.is_none());
    }

    #[test]
    fn iter2_d_about_selects_its_own_navigation_row() {
        use egui_kittest::kittest::{NodeT, Queryable};
        let mut library = navigation_library();
        let owner = library.loaded.as_ref().unwrap().notes[0].owner.clone();
        let package = owner.id().unwrap();
        let text = note_text("First", "package", Some("first"));
        library.loaded.as_mut().unwrap().notes[0] = Note {
            path: "Package.md".into(),
            entry: Some(Entry::parse("Package.md", &text).unwrap()),
            text,
            error: None,
            fault: None,
            owner,
        };
        library.loaded.as_mut().unwrap().notes[1].owner =
            library.loaded.as_ref().unwrap().notes[0].owner.clone();
        library.scope = Scope::Package(package);
        let mut campaigns = Campaigns::default();
        let mut harness = egui_kittest::Harness::new_ui_state(
            move |ui, library| library.show(ui, &mut campaigns),
            library,
        );
        harness.set_size(egui::vec2(1920.0, 1080.0));
        design::theme::install(&harness.ctx);
        harness.run_steps(2);
        harness.get_by_label("About First").click();
        harness.run_steps(2);
        assert_eq!(
            harness
                .get_by_label("About First")
                .accesskit_node()
                .toggled(),
            Some(egui::accesskit::Toggled::True)
        );
        assert_eq!(
            harness
                .get_by_label("All entries   1")
                .accesskit_node()
                .toggled(),
            Some(egui::accesskit::Toggled::False)
        );
        harness.get_by_label("All entries   1").click();
        harness.run_steps(2);
        assert_eq!(
            harness
                .get_by_label("About First")
                .accesskit_node()
                .toggled(),
            Some(egui::accesskit::Toggled::False)
        );
        assert_eq!(
            harness
                .get_by_label("All entries   1")
                .accesskit_node()
                .toggled(),
            Some(egui::accesskit::Toggled::True)
        );
    }

    #[test]
    fn link_updates_stop_when_a_name_or_filename_is_ambiguous() {
        let mut library = navigation_library();
        let loaded = library.loaded.as_mut().unwrap();
        assert!(loaded.can_rename_links(0, "First", "New name"));
        assert!(!loaded.can_rename_links(0, "First", "Second"));
        loaded.notes[1].path = "Elsewhere/FIRST.md".into();
        assert!(!loaded.can_rename_links(0, "First", "New name"));
        loaded.notes[1].path = "Second.md".into();
        loaded.notes[1].entry.as_mut().unwrap().name = "First".into();
        assert!(!loaded.can_rename_links(0, "First", "New name"));

        // Round 24 FC4: the offer's count reads the notes once; an unchanged frame reads none,
        // and an edited note (`invalidate`) is counted again.
        let scans = || RENAME_LINK_SCANS.with(std::cell::Cell::get);
        let start = scans();
        let before = loaded.rename_link_count("Watch", "Ward");
        assert_eq!(scans(), start + 1);
        assert_eq!(loaded.rename_link_count("Watch", "Ward"), before);
        assert_eq!(scans(), start + 1, "an unchanged frame reads no note");
        let linker = loaded
            .notes
            .iter()
            .position(|note| note.owner.root().is_some())
            .expect("fixture: a note in a folder");
        loaded.notes[linker].text.push_str("\nSee [[Watch]].\n");
        loaded.invalidate();
        assert_eq!(loaded.rename_link_count("Watch", "Ward"), before + 1);
        assert_eq!(scans(), start + 2, "an edited note is counted again");
    }

    #[test]
    fn saving_a_note_moved_to_another_owner_does_not_create_a_second_file() {
        let temp = tempfile::tempdir().unwrap();
        let first = create_module(&temp.path().join("first"), "First", "first").unwrap();
        let second = create_module(&temp.path().join("second"), "Second", "second").unwrap();
        let text = note_text("Wolf", "creature", None);
        fs::write(first.path.join("Wolf.md"), &text).unwrap();
        let loaded = load_all(&[first.clone(), second.clone()], None).unwrap();
        let selected = loaded
            .notes
            .iter()
            .position(|note| note.path == "Wolf.md")
            .unwrap();
        let mut library = Library {
            loaded: Some(loaded),
            selected: Some(selected),
            draft: format!("{text}\nMy draft.\n"),
            ..Default::default()
        };
        fs::rename(first.path.join("Wolf.md"), second.path.join("Moved.md")).unwrap();
        library.save_selected(false);
        assert!(library.error.is_none(), "{:?}", library.error);
        assert!(!first.path.join("Wolf.md").exists());
        assert_eq!(
            fs::read_to_string(second.path.join("Moved.md")).unwrap(),
            library.draft
        );
    }

    #[test]
    fn saving_a_moved_and_edited_note_opens_comparison_before_writing() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        let text = note_text("Wolf", "creature", None);
        fs::write(module.path.join("Wolf.md"), &text).unwrap();
        let loaded = load(Some(&module)).unwrap();
        let selected = loaded
            .notes
            .iter()
            .position(|note| note.path == "Wolf.md")
            .unwrap();
        let mut library = Library {
            loaded: Some(loaded),
            selected: Some(selected),
            draft: format!("{text}\nMy draft.\n"),
            ..Default::default()
        };
        fs::rename(module.path.join("Wolf.md"), module.path.join("Moved.md")).unwrap();
        let outside = format!("{text}\nOutside edit.\n");
        fs::write(module.path.join("Moved.md"), &outside).unwrap();
        library.save_selected(false);
        assert!(library.disk_change.is_some());
        assert_eq!(
            fs::read_to_string(module.path.join("Moved.md")).unwrap(),
            outside
        );
        assert!(library.draft.contains("My draft."));
    }

    #[test]
    fn repairing_a_created_package_replaces_its_actual_manifest() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        let (entries, _) = content::read_directory(&module.path).unwrap();
        let path = module.path.join(&entries[0].path);
        let valid = fs::read_to_string(&path).unwrap();
        fs::write(
            &path,
            valid.replace("splat_namespace: test", "splat_namespace: ["),
        )
        .unwrap();
        let repair = crate::content_workspace::ManifestWrite::write(&module).unwrap();
        assert!(repair.replaced());
        assert_eq!(content::read_directory(&module.path).unwrap().0.len(), 1);
        repair.undo().unwrap();
        fs::write(&path, valid).unwrap();
        assert_eq!(local_module(&module.path).unwrap().id, module.id);
    }

    #[test]
    fn copying_without_a_destination_is_blocked() {
        let campaigns = Campaigns::default();
        assert_eq!(
            copy_destination_reason(&campaigns),
            Some("Add a campaign or a package to create entries.")
        );
    }

    #[test]
    fn duplicate_package_id_save_keeps_both_files() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        let text = note_text("Wolf", "creature", None);
        fs::write(module.path.join("Wolf.md"), &text).unwrap();
        let copy = format!("{text}\nVariant on disk.\n");
        fs::write(module.path.join("Wolf copy.md"), &copy).unwrap();
        let loaded = load(Some(&module)).unwrap();
        let selected = loaded
            .notes
            .iter()
            .position(|n| n.path == "Wolf.md")
            .unwrap();
        let mut library = Library {
            loaded: Some(loaded),
            selected: Some(selected),
            draft: format!("{text}\nEdited original.\n"),
            ..Default::default()
        };
        library.save_selected(false);
        assert!(library.error.is_none());
        assert_eq!(
            fs::read_to_string(module.path.join("Wolf.md")).unwrap(),
            library.draft
        );
        assert_eq!(
            fs::read_to_string(module.path.join("Wolf copy.md")).unwrap(),
            copy
        );
        let id = crate::content_workspace::note_id(&text).unwrap();
        let catalogue = Catalogue::load(std::slice::from_ref(&module), None).unwrap();
        assert!(!catalogue.entries.contains_key(&id));
        let closure = catalogue.closure(
            &[crate::content_workspace::PackageSelection {
                name: Some(module.name.clone()),
                package: crate::content_workspace::EntryRef::id(module.id),
                selection: crate::content_workspace::Selection::All,
            }],
            Uuid::new_v4(),
        );
        assert!(!closure.direct.contains(&id));
        assert!(closure.errors.iter().any(|error| {
            error.contains("These two files claim to be the same entry: Wolf copy.md and Wolf.md.")
        }));
        assert!(
            catalogue.unreadable[&id]
                .word
                .contains("These two files claim to be the same entry: Wolf copy.md and Wolf.md.")
        );
        let copy_index = library
            .loaded
            .as_ref()
            .unwrap()
            .notes
            .iter()
            .position(|n| n.path == "Wolf copy.md")
            .unwrap();
        library.selected = Some(copy_index);
        library.draft = format!("{copy}\nEdited copy.\n");
        library.save_selected(false);
        let original = fs::read_to_string(module.path.join("Wolf.md")).unwrap();
        let saved_copy = library.draft.clone();
        library.give_new_identity().unwrap();
        assert_ne!(crate::content_workspace::note_id(&library.draft), Some(id));
        let repaired = Entry::parse("Wolf copy.md", &library.draft).unwrap();
        assert_eq!(repaired.name, "Wolf copy");
        assert!(repaired.body.contains("# Wolf copy\n"));
        assert_eq!(
            fs::read_to_string(module.path.join("Wolf.md")).unwrap(),
            original
        );
        assert!(
            Catalogue::load(std::slice::from_ref(&module), None)
                .unwrap()
                .entries
                .contains_key(&id)
        );
        library.undo_new_identity().unwrap();
        assert_eq!(
            fs::read_to_string(module.path.join("Wolf copy.md")).unwrap(),
            saved_copy
        );
        assert_eq!(
            library
                .loaded
                .as_ref()
                .unwrap()
                .duplicates(copy_index)
                .len(),
            1
        );
        library.give_new_identity().unwrap();
        fs::write(module.path.join("Wolf copy.md"), "A later disk edit.").unwrap();
        assert!(library.undo_new_identity().is_err());
        assert_eq!(
            fs::read_to_string(module.path.join("Wolf copy.md")).unwrap(),
            "A later disk edit."
        );
        assert_eq!(
            fs::read_to_string(module.path.join("Wolf.md")).unwrap(),
            original
        );
    }

    #[test]
    fn manifest_only_hand_repair_keeps_original_and_both_library_files() {
        let temp = tempfile::tempdir().unwrap();
        let original =
            create_module(&temp.path().join("original"), "Original", "original").unwrap();
        let copy = create_module(&temp.path().join("copy"), "Copy", "copy").unwrap();
        let manifest = fs::read_to_string(original.path.join("Package.md")).unwrap();
        let hand_edit = crate::content_workspace::patch_fields(
            &manifest,
            &[("splat_id", Some(copy.id.to_string().into()))],
        )
        .unwrap();
        fs::write(copy.path.join("Package.md"), hand_edit).unwrap();
        let text = note_text("Wolf", "creature", None);
        fs::write(original.path.join("Wolf.md"), &text).unwrap();
        fs::write(copy.path.join("Wolf.md"), &text).unwrap();
        let id = crate::content_workspace::note_id(&text).unwrap();
        let modules = [original.clone(), copy];
        let catalogue = Catalogue::load(&modules, None).unwrap();
        assert_eq!(catalogue.entries[&id].owner, original.id);
        assert!(
            catalogue
                .errors
                .iter()
                .any(|e| e.contains("These two files claim to be the same entry:"))
        );
        let loaded = load_all(&modules, None).unwrap();
        let wolves: Vec<_> = loaded
            .notes
            .iter()
            .enumerate()
            .filter(|(_, n)| n.name() == "Wolf" && n.owner.root().is_some())
            .collect();
        assert_eq!(wolves.len(), 2);
        // Round 24 AC1: both files of the duplicate count in the header, as both rows are
        // marked, though the check names one side.
        let wolf_files: Vec<FileKey> = wolves.iter().map(|(_, n)| Loaded::file_key(n)).collect();
        let named = loaded
            .diagnostics
            .iter()
            .filter(|d| wolf_files.contains(&d.file))
            .map(|d| &d.file)
            .collect::<std::collections::HashSet<_>>()
            .len();
        assert!(named < 2, "fixture: the check names one Wolf file only");
        let counted: std::collections::HashSet<&FileKey> = loaded
            .diagnostics
            .iter()
            .map(|d| &d.file)
            .chain(&wolf_files)
            .collect();
        assert_eq!(
            loaded.files_to_fix(&[], None),
            counted.len(),
            "iter-24-L-AC1"
        );
        for (at, _) in wolves {
            assert_eq!(loaded.duplicates(at).len(), 1);
            assert!(loaded.build_subtitles()[at].contains("Same entry in two files"));
        }
    }

    #[test]
    fn duplicate_repair_keeps_key_named_original_after_atomic_save() {
        let temp = tempfile::tempdir().unwrap();
        let original = temp.path().join("Wolf.md");
        let copy = temp.path().join("Wolf copy.md");
        let text = note_text("Wolf", "creature", None).replace(
            "splat_kind: creature",
            "splat_key: test:creatures/wolf\nsplat_kind: creature",
        );
        fs::write(&original, &text).unwrap();
        fs::write(&copy, &text).unwrap();
        write_note(
            temp.path(),
            "Wolf.md",
            Some(&text),
            &format!("{text}\nEdit original.\n"),
        )
        .unwrap();
        assert_eq!(
            crate::content_workspace::duplicate_repair_path([original, copy.clone()]),
            Some(copy)
        );
    }

    #[test]
    fn deleted_duplicate_is_not_treated_as_a_move_to_the_other_file() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        let text = note_text("Wolf", "creature", None);
        fs::write(module.path.join("Wolf.md"), &text).unwrap();
        fs::write(module.path.join("Wolf copy.md"), &text).unwrap();
        let loaded = load(Some(&module)).unwrap();
        let selected = loaded
            .notes
            .iter()
            .position(|note| note.path == "Wolf.md")
            .unwrap();
        let mut library = Library {
            loaded: Some(loaded),
            selected: Some(selected),
            draft: format!("{text}\nMy draft.\n"),
            ..Default::default()
        };
        fs::remove_file(module.path.join("Wolf.md")).unwrap();
        library.refresh_destination(&module.path, &egui::Context::default());
        assert!(library.disk_change.as_ref().unwrap().moved_to.is_none());
        library.save_selected(false);
        assert!(!module.path.join("Wolf.md").exists());
        assert_eq!(
            fs::read_to_string(module.path.join("Wolf copy.md")).unwrap(),
            text
        );
        library.save_selected(true);
        assert_eq!(
            fs::read_to_string(module.path.join("Wolf.md")).unwrap(),
            library.draft
        );
        assert_eq!(
            fs::read_to_string(module.path.join("Wolf copy.md")).unwrap(),
            text
        );
    }

    #[test]
    fn moved_draft_save_does_not_duplicate_its_id() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        let text = note_text("Moved rule", "rule", None);
        fs::write(module.path.join("Rule.md"), &text).unwrap();
        let loaded = load(Some(&module)).unwrap();
        let selected = loaded
            .notes
            .iter()
            .position(|n| n.path == "Rule.md")
            .unwrap();
        let mut library = Library {
            loaded: Some(loaded),
            selected: Some(selected),
            draft: format!("{text}\nMy draft.\n"),
            ..Default::default()
        };
        fs::create_dir(module.path.join("Moved")).unwrap();
        fs::rename(
            module.path.join("Rule.md"),
            module.path.join("Moved/Rule.md"),
        )
        .unwrap();
        library.refresh_destination(&module.path, &egui::Context::default());
        library.save_selected(true);
        assert!(
            !module.path.join("Rule.md").exists(),
            "must not recreate the old path"
        );
        assert!(
            fs::read_to_string(module.path.join("Moved/Rule.md"))
                .unwrap()
                .contains("My draft.")
        );
    }

    #[test]
    fn repaired_package_note_reloads_campaign_uses() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test package", "test").unwrap();
        let campaign = Campaign {
            id: Uuid::new_v4(),
            name: "Test campaign".into(),
            path: temp.path().join("campaign"),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        fs::create_dir_all(&campaign.path).unwrap();
        CampaignDocument::new(&campaign).unwrap().save().unwrap();
        let text = note_text("Test rule", "rule", None);
        let id = Entry::parse("Rule.md", &text).unwrap().id;
        fs::write(module.path.join("Rule.md"), &text).unwrap();
        crate::content_workspace::use_in_campaign(&campaign, module.id, &module.name, Some(id))
            .unwrap();
        assert!(
            load_all(std::slice::from_ref(&module), Some(&campaign))
                .unwrap()
                .uses
                .unwrap()
                .contains(&id)
        );
        fs::write(
            module.path.join("Rule.md"),
            text.replace("splat_kind: rule", "splat_kind: ["),
        )
        .unwrap();
        let loaded = load_all(std::slice::from_ref(&module), Some(&campaign)).unwrap();
        assert!(!loaded.uses.as_ref().unwrap().contains(&id));
        let selected = loaded
            .notes
            .iter()
            .position(|n| n.path == "Rule.md" && n.owner.id() == Some(module.id))
            .unwrap();
        let mut library = Library {
            loaded: Some(loaded),
            selected: Some(selected),
            draft: text.clone(),
            ..Default::default()
        };
        library.save_selected(false);
        assert!(library.error.is_none(), "{:?}", library.error);
        assert!(library.stale, "the next frame must rebuild campaign uses");
        let loaded = load_all(&[module], Some(&campaign)).unwrap();
        assert!(
            loaded.uses.unwrap().contains(&id),
            "the repaired note is Chosen for this campaign"
        );
    }

    #[test]
    fn scope_picker_cuts_two_words_with_one_ellipsis() {
        let ctx = egui::Context::default();
        let _ = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let budget = design::number_width(ui, "Ashen…", &Type::Strong.font(ui));
                assert_eq!(scope_label(ui, "Ashen Marches", budget), "Ashen…");
                assert_eq!(
                    scope_label(ui, "Ashen Marches", budget / 2.0)
                        .matches('…')
                        .count(),
                    1
                );
            });
        });
    }

    #[test]
    fn creature_template_uses_fields_on_every_read_surface() {
        let text = note_text("Copper Drake", "creature", None);
        assert!(
            !text
                .lines()
                .any(|line| line.trim_start().starts_with("value:"))
        );
        assert!(text.contains("| Speed | 30 ft. |"));
        assert!(text.contains("splat_schema: 2"));
        assert!(!text.contains("splat_stats:"));
        let entry = Entry::parse("Copper Drake.md", &text).unwrap();
        assert!(entry.fields["splat_stats"]["max_hp"].get("value").is_none());
        assert!(
            entry.fields["splat_stats"]["armor_class"]
                .get("value")
                .is_none()
        );
        assert_eq!(
            entry.fields["splat_creature"]["type"].as_str(),
            Some("humanoid")
        );
        for key in [
            "strength",
            "dexterity",
            "constitution",
            "intelligence",
            "wisdom",
            "charisma",
        ] {
            assert_eq!(entry.fields["splat_stats"][key]["value"].as_i64(), Some(10));
        }
        assert_eq!(
            presentation::challenge_text(&entry),
            Some("1 (XP 200; PB +2)")
        );
        assert_eq!(presentation::subtitle(&entry), "Creature · CR 1");
        assert!(!entry.body.contains("Speed 30"));
        assert!(!entry.body.contains("Str 10"));
        assert!(!entry.body.contains("CR 1"));
        let mut conflicting = entry.clone();
        conflicting.body.push_str("\nCR 9 (XP 5,000; PB +4)\n");
        assert_eq!(
            presentation::challenge_text(&conflicting),
            Some("1 (XP 200; PB +2)")
        );
        assert_eq!(presentation::subtitle(&conflicting), "Creature · CR 1");
    }

    #[test]
    fn heading_repair_changes_only_the_first_h1() {
        let text = note_text("Watch Ashfall", "spell", None)
            .replace("# Watch Ashfall", "# Ashfall")
            + "\n# Another heading\n";
        let entry = Entry::parse("Ashfall.md", &text).unwrap();
        let (old, after) = presentation::heading_fix(&text, &entry).unwrap();
        assert_eq!(old, "Ashfall");
        assert_eq!(after, text.replacen("# Ashfall", "# Watch Ashfall", 1));
    }

    #[test]
    fn back_keeps_an_unsaved_homebrew_draft() {
        let mut library = navigation_library();
        library.open_entry(0);
        library.open_entry(1);
        library.draft.push_str("Unsaved ruling.");
        let draft = library.draft.clone();
        library.go(Step::Back);
        assert_eq!(library.selected, Some(1));
        assert_eq!(library.draft, draft);
        assert!(library.trail.peek(Step::Back).is_some());
    }

    #[test]
    fn forward_returns_to_what_back_left_until_another_link() {
        let mut library = navigation_library();
        library.open_entry(0);
        library.open_entry(1);
        library.go(Step::Back);
        assert_eq!(library.selected, Some(0));
        library.go(Step::Forward);
        assert_eq!(library.selected, Some(1));
        library.go(Step::Back);
        library.open_entry(2);
        assert!(
            library.trail.peek(Step::Forward).is_none(),
            "a new link starts a new trail"
        );
    }

    #[test]
    fn back_returns_to_the_right_owner_and_scope() {
        let mut library = navigation_library();
        let owner = library.loaded.as_ref().unwrap().notes[1]
            .owner
            .id()
            .unwrap();
        library.scope = Scope::Package(owner);
        library.open_entry(1);
        library.search = "Second".into();
        library.open_entry(2);
        library.scope = Scope::All;
        library.go(Step::Back);
        assert_eq!(library.selected, Some(1));
        assert_eq!(library.scope, Scope::Package(owner));
        assert_eq!(library.search, "Second");
    }

    #[test]
    fn following_a_link_reveals_an_entry_outside_the_package_scope() {
        let mut library = navigation_library();
        let owner = library.loaded.as_ref().unwrap().notes[1]
            .owner
            .id()
            .unwrap();
        library.scope = Scope::Package(owner);
        library.open_entry(1);
        library.open_entry(2);
        let loaded = library.loaded.as_ref().unwrap();
        assert!(loaded.in_scope(&loaded.notes[2], library.scope));
        library.go(Step::Back);
        assert_eq!(library.scope, Scope::Package(owner));
    }
    /// Run alone so the first load starts without a warmed process cache.
    #[test]
    #[ignore]
    fn srd_library_load_time() {
        for run in 0..3 {
            let start = std::time::Instant::now();
            let loaded = load(None).unwrap();
            assert_eq!(loaded.notes.len(), 2065);
            let load_time = start.elapsed();
            let _ = loaded.subtitles();
            let _ = loaded.count(Section::All);
            let elapsed = start.elapsed();
            eprintln!("SRD run {run}: load={load_time:?}, ready={elapsed:?}");
            assert!(elapsed.as_secs_f64() < 1.0, "SRD library must load in <1s");
        }
    }

    /// Exercise the editable package path with the same notes, read from disk.
    #[test]
    #[ignore]
    fn user_library_load_time() {
        fn copy_notes(source: &Path, destination: &Path) {
            fs::create_dir_all(destination).unwrap();
            for item in fs::read_dir(source).unwrap() {
                let path = item.unwrap().path();
                let target = destination.join(path.file_name().unwrap());
                if path.is_dir() {
                    copy_notes(&path, &target);
                } else if path.extension().is_some_and(|ext| ext == "md") {
                    fs::copy(path, target).unwrap();
                }
            }
        }
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "User package", "user").unwrap();
        copy_notes(
            &Path::new(env!("CARGO_MANIFEST_DIR")).join("builtin/dnd2024-definitions"),
            &module.path,
        );
        for run in 0..3 {
            let start = std::time::Instant::now();
            let loaded = load(Some(&module)).unwrap();
            assert_eq!(loaded.notes.len(), 2066);
            assert!(loaded.notes.iter().all(|note| note.entry.is_some()));
            let _ = loaded.subtitles();
            let _ = loaded.count(Section::All);
            let elapsed = start.elapsed();
            eprintln!("User package run {run}: ready={elapsed:?}");
            assert!(elapsed.as_secs_f64() < 1.0, "User library must load in <1s");
        }
    }

    #[test]
    fn duplicate_package_rows_keep_the_folder_last_and_whole() {
        let ctx = egui::Context::default();
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(256.0, 100.0),
            )),
            ..Default::default()
        };
        let output = ctx.run(input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.set_width(240.0);
                let response = entry_row(
                    ui,
                    "Cinder Wolf",
                    "Same entry in two files · Creature · CR 1 · Ashen Marches · ashen-marches-copy/",
                    false,
                );
                assert_eq!(response.rect.height(), ENTRY_ROW_HEIGHT);
            });
        });
        // Round 25 FF5: the folder ends the row, as it ends the reader's kind line. The row is
        // too narrow for the whole line, so the words before the folder shorten, never the folder.
        let lines: Vec<String> = output
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Text(text) if text.galley.job.text.starts_with("Same entry") => {
                    Some(text.galley.job.text.clone())
                }
                _ => None,
            })
            .collect();
        assert!(
            lines.len() == 1
                && lines[0].starts_with("Same entry")
                && lines[0].contains('…')
                && lines[0].ends_with(" · ashen-marches-copy/"),
            "{lines:?}"
        );
    }

    #[test]
    fn entry_rows_keep_both_lines_at_a_fixed_height() {
        let ctx = egui::Context::default();
        let output = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(186.0, 100.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.set_width(170.0);
                    let response = entry_row(
                        ui,
                        &"Long entry name ".repeat(30),
                        &"Long subtitle ".repeat(30),
                        false,
                    );
                    assert_eq!(response.rect.height(), 44.0);
                });
            },
        );
        let rows: Vec<_> = output
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Text(text) if !text.galley.job.text.is_empty() => Some(&text.galley),
                _ => None,
            })
            .collect();
        assert_eq!(rows.len(), 2);
        assert!(
            rows.iter()
                .all(|galley| galley.rows.len() == 1 && galley.rect.width() <= 162.0)
        );
    }

    #[test]
    fn list_metadata_is_refreshed_after_content_changes() {
        let mut loaded = Loaded {
            notes: vec![Note {
                path: "first.md".into(),
                text: String::new(),
                entry: None,
                error: None,
                fault: None,
                owner: Owner::Builtin,
            }],
            ..Default::default()
        };
        assert_eq!(loaded.count(Section::All), 1);
        assert_eq!(loaded.subtitles().len(), 1);
        loaded.notes.push(Note {
            path: "second.md".into(),
            text: String::new(),
            entry: None,
            error: None,
            fault: None,
            owner: Owner::Builtin,
        });
        loaded.invalidate();
        assert_eq!(loaded.count(Section::All), 2);
        assert_eq!(loaded.subtitles().len(), 2);
    }

    #[test]
    fn reader_names_the_package_once() {
        let package = Owner::Package(Module {
            id: Uuid::new_v4(),
            name: "Ashen Marches".into(),
            path: "ashen".into(),
        });
        // The list leaves the built-in package unsaid, and already names any other.
        assert_eq!(
            reader_subtitle(&Owner::Builtin, "Spell · Level 3", None),
            format!("Spell · Level 3 · {BUILTIN_NAME}")
        );
        assert_eq!(
            reader_subtitle(&package, "Spell · Level 3 · Ashen Marches", None),
            "Spell · Level 3 · Ashen Marches"
        );
        // A class page's own line replaces the list's, so it needs the name too.
        assert_eq!(
            reader_subtitle(&package, "Rule", Some("Bard · Level 3".into())),
            "Class feature · Bard · Level 3 · Ashen Marches"
        );
        // Round 23 FE8: the kind line under the owner line leaves the package name out. Where
        // two packages share the name, the folder alone tells them apart.
        assert_eq!(
            reader_kind_line(&package, "Creature · CR 1 · Ashen Marches"),
            "Creature · CR 1"
        );
        assert_eq!(
            reader_kind_line(&package, "Creature · CR 1 · Ashen Marches · ashen/"),
            "Creature · CR 1 · ashen/"
        );
        // Round 24 AC2, SC1: a duplicate's reader leaves out its row's fault, and keeps the
        // folder that tells the two files apart.
        assert_eq!(
            reader_kind_line(
                &package,
                &row_line_without_fault(&format!(
                    "{} {SAME_ENTRY} · Creature · CR 1 · Ashen Marches · ashen/",
                    icons::WARN
                ))
            ),
            "Creature · CR 1 · ashen/"
        );
        // Round 25 AC2: a row with two causes names both, and the reader leaves both out.
        assert_eq!(
            reader_kind_line(
                &package,
                &row_line_without_fault(&format!(
                    "{} {SAME_ENTRY} · {HEADING_DIFFERS} · Creature · CR 1 · Ashen Marches · ashen/",
                    icons::WARN
                ))
            ),
            "Creature · CR 1 · ashen/"
        );
        // Lookalike: another owner's line keeps every part.
        let campaign = Owner::Campaign(Campaign {
            id: Uuid::new_v4(),
            name: "Ashen Marches".into(),
            path: "ashen".into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        });
        assert_eq!(
            reader_kind_line(&campaign, "Creature · CR 1 · Ashen Marches · ashen/"),
            "Creature · CR 1 · Ashen Marches · ashen/"
        );
    }

    /// Run explicitly to measure the complete library UI while scrolling the built-in package.
    #[test]
    #[ignore]
    fn library_scroll_frame_times() {
        let mut library = Library {
            loaded: Some(load(None).unwrap()),
            ..Default::default()
        };
        let mut campaigns = Campaigns::default();
        let ctx = egui::Context::default();
        let mut times = Vec::new();
        for frame in 0..150 {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1200.0, 800.0),
                )),
                time: Some(frame as f64 / 60.0),
                events: vec![
                    egui::Event::PointerMoved(egui::pos2(300.0, 450.0)),
                    egui::Event::MouseWheel {
                        unit: egui::MouseWheelUnit::Point,
                        delta: egui::vec2(0.0, -30.0),
                        modifiers: Default::default(),
                    },
                ],
                ..Default::default()
            };
            let start = std::time::Instant::now();
            let output = ctx.run(input, |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| library.show(ui, &mut campaigns));
            });
            let _ = ctx.tessellate(output.shapes, output.pixels_per_point);
            if frame >= 30 {
                times.push(start.elapsed().as_secs_f64() * 1000.0);
            }
        }
        times.sort_by(f64::total_cmp);
        println!(
            "Library entries: {}; frame CPU ms: median={:.3}, p95={:.3}, max={:.3}",
            library.loaded.as_ref().unwrap().notes.len(),
            times[times.len() / 2],
            times[times.len() * 95 / 100],
            times.last().unwrap()
        );
    }

    #[test]
    fn repeated_rule_headings_name_their_subsection() {
        let loaded = load(None).unwrap();
        let subtitles = loaded.subtitles();
        for (suffix, context) in [
            ("ability_modifier", "Ability Checks"),
            ("ability_modifier_2", "Saving Throws"),
            ("ability_modifier_3", "Attack Rolls"),
        ] {
            let row = loaded
                .notes
                .iter()
                .position(|note| {
                    note.entry.as_ref().and_then(|entry| entry.key.as_deref())
                        == Some(format!("dnd2024:rules/d20_tests_{suffix}").as_str())
                })
                .unwrap();
            assert_eq!(subtitles[row], format!("Rule · D20 Tests › {context}"));
        }
    }

    #[test]
    fn same_name_entries_have_distinct_visible_labels() {
        let loaded = load(None).unwrap();
        let subtitles = loaded.subtitles();
        let labels: std::collections::HashSet<_> = loaded
            .notes
            .iter()
            .zip(subtitles)
            .map(|(note, subtitle)| (note.name(), subtitle))
            .collect();
        assert_eq!(labels.len(), loaded.notes.len());
    }

    #[test]
    fn imported_overview_renders_as_prose_without_source_markup() {
        for (path, expected) in [
            (
                "definitions/glossary/ability_check.md",
                "represents using one of the six abilities",
            ),
            (
                "definitions/spells/bless.md",
                "Whenever a target makes an attack roll",
            ),
            (
                "definitions/spells/command.md",
                "Approach. The target moves toward you by the shortest",
            ),
            (
                "definitions/rules/d20_tests_ability_modifier_2.md",
                "Physically resist direct force",
            ),
            (
                "definitions/rules/d20_tests_ability_modifier.md",
                "Notice things in the environment or in creatures’ behavior",
            ),
            (
                "definitions/spells/divine_smite.md",
                "if the target is a Fiend or an Undead.",
            ),
            (
                "definitions/spells/augury.md",
                "circumstances, such as other spells, that might change the results.",
            ),
        ] {
            let text = content::definitions::source_file(path).unwrap().to_owned();
            let entry = Entry::parse(path, &text).unwrap();
            let overview = presentation::reading_sections(&entry.body, &entry.name)
                .into_iter()
                .find(|section| section.title == "Source text")
                .unwrap();
            let readable = presentation::overview_markdown(overview.body);
            if path.ends_with("command.md") {
                for option in ["Approach.", "Drop.", "Flee.", "Grovel.", "Halt."] {
                    assert!(readable.contains(&format!("\n\n{option}")), "{readable}");
                }
                assert!(
                    readable.contains("holding and then ends its turn."),
                    "{readable}"
                );
            }
            if path.ends_with("augury.md") {
                assert!(
                    readable.contains(
                        "| Weal | Good |\n| Woe | Bad |\n| Weal and woe | Good and bad |"
                    ),
                    "{readable}"
                );
            }
            let mut library = Library {
                loaded: Some(Loaded {
                    notes: vec![Note {
                        path: entry.path.clone(),
                        text: text.clone(),
                        entry: Some(entry),
                        error: None,
                        fault: None,
                        owner: Owner::Builtin,
                    }],
                    diagnostics: vec![],
                    ..Default::default()
                }),
                selected: Some(0),
                draft: text,
                ..Default::default()
            };
            let ctx = egui::Context::default();
            let subtitles = library.loaded.as_ref().unwrap().subtitles().clone();
            let output = ctx.run(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1000.0, 700.0),
                    )),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        library.editor(ui, &subtitles[0], &Campaigns::default())
                    });
                },
            );
            let text = output
                .shapes
                .iter()
                .filter_map(|s| match &s.shape {
                    egui::Shape::Text(t) => Some(t.galley.job.text.as_str()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join("\n");
            assert!(text.contains(expected), "{text}");
            assert!(!text.contains('\\'), "{text}");
            assert!(!text.contains("[["), "{text}");
            assert!(!text.contains("Original layout:"), "{text}");
        }
    }
    #[test]
    fn creature_spell_links_open_named_spells_and_protect_drafts() {
        let notes = [
            "creatures/adult_bronze_dragon",
            "spells/detect_magic",
            "spells/guiding_bolt",
            "spells/shapechange",
            "spells/speak_with_animals",
            "spells/thaumaturgy",
            "spells/detect_thoughts",
            "spells/water_breathing",
        ]
        .map(|name| {
            let path = format!("definitions/{name}.md");
            let source = content::definitions::source_file(&path).unwrap();
            Note {
                entry: Some(Entry::parse(&path, source).unwrap()),
                path,
                text: source.into(),
                error: None,
                fault: None,
                owner: Owner::Builtin,
            }
        })
        .into_iter()
        .collect();
        let loaded = Loaded {
            notes,
            ..Default::default()
        };
        let references = creature::spell_references(loaded.notes[0].entry.as_ref().unwrap());
        assert_eq!(references.len(), 7);
        for (reference, index) in references.iter().zip(1..8) {
            assert_eq!(loaded.resolve_spell(reference), Some(index));
        }
        let by_id = content::Reference::Id(loaded.notes[2].entry.as_ref().unwrap().id);
        assert_eq!(loaded.resolve_spell(&by_id), Some(2));
        assert_eq!(
            loaded.resolve_spell(&content::Reference::Key("missing:spell".into())),
            None
        );

        let mut library = Library {
            draft: loaded.notes[0].text.clone(),
            loaded: Some(loaded),
            selected: Some(0),
            section: Section::Creatures,
            search: "bronze".into(),
            ..Default::default()
        };
        for width in [390.0, 900.0] {
            for dirty in [true, false] {
                library.open_entry(0);
                library.search = "bronze".into();
                library.matches = Some((Section::Creatures, "bronze".into(), vec![0]));
                if dirty {
                    library.draft.push_str("\nUnsaved note.\n");
                }
                let ctx = egui::Context::default();
                let mut frame = |events| {
                    ctx.run(
                        egui::RawInput {
                            screen_rect: Some(egui::Rect::from_min_size(
                                egui::Pos2::ZERO,
                                egui::vec2(width, 6000.0),
                            )),
                            events,
                            ..Default::default()
                        },
                        |ctx| {
                            egui::CentralPanel::default().show(ctx, |ui| {
                                library.editor(ui, "Definition · creature", &Campaigns::default())
                            });
                        },
                    )
                };
                frame(vec![]);
                let output = frame(vec![]);
                let texts: Vec<_> = output
                    .shapes
                    .iter()
                    .filter_map(|shape| match &shape.shape {
                        egui::Shape::Text(text) => Some(text),
                        _ => None,
                    })
                    .collect();
                let link = texts
                    .iter()
                    .find(|t| t.galley.job.text == "Guiding Bolt")
                    .unwrap();
                let spellcasting = texts
                    .iter()
                    .find(|t| t.galley.job.text.starts_with("Spellcasting."))
                    .unwrap();
                let legendary = texts
                    .iter()
                    .find(|t| t.galley.job.text == "Legendary Actions")
                    .unwrap();
                assert!(link.pos.y > spellcasting.pos.y && link.pos.y < legendary.pos.y);
                assert!(!texts.iter().any(|t| t.galley.job.text.contains("^splat-")));
                let position = link.pos
                    + link.galley.rows[0]
                        .rect_without_leading_space()
                        .center()
                        .to_vec2();
                frame(vec![egui::Event::PointerMoved(position)]);
                for pressed in [true, false] {
                    frame(vec![egui::Event::PointerButton {
                        pos: position,
                        button: egui::PointerButton::Primary,
                        pressed,
                        modifiers: egui::Modifiers::NONE,
                    }]);
                }
                if dirty {
                    assert_eq!(library.selected, Some(0));
                    assert!(library.draft.ends_with("Unsaved note.\n"));
                    library.draft = library.loaded.as_ref().unwrap().notes[0].text.clone();
                } else {
                    assert_eq!(library.selected, Some(2));
                    assert_eq!(library.section, Section::Spells);
                    assert!(library.search.is_empty());
                    assert!(library.matches.is_none());
                    assert!(!library.source);
                    assert!(library.draft.contains("# Guiding Bolt"));
                }
            }
        }
        // Conflicting identities must not choose an arbitrary destination.
        let loaded = library.loaded.as_mut().unwrap();
        let duplicate = &loaded.notes[2];
        loaded.notes.push(Note {
            path: "duplicate.md".into(),
            text: duplicate.text.clone(),
            entry: duplicate.entry.clone(),
            error: None,
            fault: None,
            owner: Owner::Builtin,
        });
        assert_eq!(loaded.resolve_spell(&by_id), None);
    }

    #[test]
    fn creature_read_view_hides_rule_anchors() {
        let path = "definitions/creatures/adult_bronze_dragon.md";
        let source = content::definitions::source_file(path).unwrap();
        let mut library = Library {
            loaded: Some(Loaded {
                notes: vec![Note {
                    path: path.into(),
                    text: source.into(),
                    entry: Some(Entry::parse(path, source).unwrap()),
                    error: None,
                    fault: None,
                    owner: Owner::Builtin,
                }],
                ..Default::default()
            }),
            selected: Some(0),
            draft: source.into(),
            ..Default::default()
        };
        let ctx = egui::Context::default();
        let output = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(900.0, 6000.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    library.editor(ui, "Definition · creature", &Campaigns::default())
                });
            },
        );
        let text = output
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Text(text) => Some(text.galley.job.text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert!(text.iter().any(|text| text.contains("Spellcasting.")));
        assert!(
            !text.iter().any(|text| text.contains("^splat-")),
            "{text:?}"
        );
        assert!(
            !text.contains(&"Rule"),
            "Empty mechanics section should be hidden"
        );
    }

    #[test]
    fn reviewed_summary_keeps_rules_and_source_and_opens_related_entry() {
        let loaded = load(None).unwrap();
        let index = loaded
            .notes
            .iter()
            .position(|note| note.name() == "Level 3: Bard Subclass")
            .unwrap();
        let entry = loaded.notes[index].entry.as_ref().unwrap();
        let mut original = entry.clone();
        original.fields.remove("splat_reading");
        assert_eq!(
            content::contracts::specification(entry).unwrap(),
            content::contracts::specification(&original).unwrap()
        );
        assert_eq!(
            serde_json::to_value(content::required_references(entry).unwrap()).unwrap(),
            serde_json::to_value(content::required_references(&original).unwrap()).unwrap()
        );
        assert!(
            entry
                .body
                .contains("of Lore subclass is detailed after this class’s description")
        );
        for width in [390.0, 900.0] {
            let mut library = Library {
                loaded: Some(load(None).unwrap()),
                ..Default::default()
            };
            library.open_entry(index);
            let ctx = egui::Context::default();
            let mut frame = |events| {
                ctx.run(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(width, 1400.0),
                        )),
                        events,
                        ..Default::default()
                    },
                    |ctx| {
                        egui::CentralPanel::default().show(ctx, |ui| {
                            library.editor(
                                ui,
                                "Definition · class feature · Bard",
                                &Campaigns::default(),
                            )
                        });
                    },
                )
            };
            frame(vec![]);
            let output = frame(vec![]);
            let texts: Vec<_> = output
                .shapes
                .iter()
                .filter_map(|shape| match &shape.shape {
                    egui::Shape::Text(text) => Some(text),
                    _ => None,
                })
                .collect();
            assert!(
                texts
                    .iter()
                    .all(|text| text.pos.x + text.galley.size().x <= width)
            );
            let rendered = texts
                .iter()
                .map(|text| text.galley.job.text.as_str())
                .collect::<Vec<_>>()
                .join("\n");
            for expected in [
                "Bard Subclass",
                "Bard · Level 3 · D&D 2024 · SRD 5.2.1",
                "Choose a Bard subclass.",
                "Subclasses",
                "Source text",
            ] {
                assert!(rendered.contains(expected), "{rendered}");
            }
            assert!(!rendered.contains("You gain a Bard subclass of your choice."));
            // The published wording is still readable, and the related link navigates.
            for label in ["Source text", "College of Lore"] {
                let current = frame(vec![]);
                let text = current
                    .shapes
                    .iter()
                    .filter_map(|shape| match &shape.shape {
                        egui::Shape::Text(text) => Some(text),
                        _ => None,
                    })
                    .find(|text| text.galley.job.text == label)
                    .unwrap();
                let pos = text.pos + text.galley.size() * 0.5;
                for pressed in [true, false] {
                    frame(vec![
                        egui::Event::PointerMoved(pos),
                        egui::Event::PointerButton {
                            pos,
                            button: egui::PointerButton::Primary,
                            pressed,
                            modifiers: Default::default(),
                        },
                    ]);
                }
                if label == "Source text" {
                    let output = frame(vec![]);
                    assert!(output.shapes.iter().any(|shape| matches!(&shape.shape, egui::Shape::Text(text) if text.galley.job.text.contains("You gain a Bard subclass of your choice."))));
                }
            }
            assert!(library.draft.contains("# College of Lore"));
        }
    }

    #[test]
    fn library_toolbar_keeps_back_visible_with_more_menu() {
        let loaded = load(None).unwrap();
        let mut library = Library {
            loaded: Some(loaded),
            ..Default::default()
        };
        library.open_entry(0);
        library.open_entry(1);
        let ctx = egui::Context::default();
        crate::design::theme::install(&ctx);
        let mut rendered = Vec::new();
        for _ in 0..3 {
            let output = ctx.run(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(360.0, 640.0),
                    )),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        library.editor(ui, "Entry", &Campaigns::default());
                    });
                },
            );
            rendered = output
                .shapes
                .into_iter()
                .filter_map(|shape| match shape.shape {
                    egui::Shape::Text(text)
                        if shape.clip_rect.contains_rect(text.visual_bounding_rect()) =>
                    {
                        Some(text.galley.job.text.clone())
                    }
                    _ => None,
                })
                .collect();
        }
        for word in ["Read", "Back", "More"] {
            assert!(
                rendered.iter().any(|text| text == word),
                "Missing {word}: {rendered:?}"
            );
        }
    }

    #[test]
    fn source_editor_keeps_its_frame_inside_the_remaining_height() {
        for size in [egui::vec2(480.0, 480.0), egui::vec2(960.0, 920.0)] {
            let source = format!(
                "{}\n{}",
                note_text("Long rule", "rule", None),
                "Long source line.\n".repeat(200)
            );
            let mut library = Library {
                loaded: Some(Loaded {
                    notes: vec![Note {
                        path: "Long rule.md".into(),
                        text: source.clone(),
                        entry: Some(Entry::parse("Long rule.md", &source).unwrap()),
                        error: None,
                        fault: None,
                        owner: Owner::Builtin,
                    }],
                    ..Default::default()
                }),
                selected: Some(0),
                source: true,
                draft: source,
                ..Default::default()
            };
            let ctx = egui::Context::default();
            crate::design::theme::install(&ctx);
            for _ in 0..3 {
                let output = ctx.run(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                        ..Default::default()
                    },
                    |ctx| {
                        egui::CentralPanel::default().show(ctx, |ui| {
                            library.editor(ui, "Rule", &Campaigns::default());
                        });
                    },
                );
                let frames: Vec<_> = output
                    .shapes
                    .iter()
                    .filter_map(|shape| {
                        if let egui::Shape::Rect(rect) = &shape.shape
                            && rect.stroke.color == colors().field_border
                            && rect.rect.height() > design::XXL
                        {
                            Some((rect.rect, shape.clip_rect))
                        } else {
                            None
                        }
                    })
                    .collect();
                assert!(!frames.is_empty(), "The source needs a visible frame");
                for (frame, clip) in frames {
                    assert!(
                        clip.expand(design::HAIRLINE_WIDTH).contains_rect(frame),
                        "Frame {frame:?} leaves viewport {clip:?}"
                    );
                    assert!(
                        frame.bottom() <= size.y,
                        "Frame leaves the window: {frame:?}"
                    );
                }
            }
        }
    }

    /// The words a built-in entry paints in the reader, in order.
    fn reader_text(name: &str) -> String {
        let loaded = load(None).unwrap();
        let index = loaded
            .notes
            .iter()
            .position(|note| note.name() == name)
            .unwrap();
        let mut library = Library {
            loaded: Some(loaded),
            ..Default::default()
        };
        library.open_entry(index);
        let ctx = egui::Context::default();
        let mut frame = || {
            ctx.run(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(390.0, 2400.0),
                    )),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        library.editor(ui, "Definition", &Campaigns::default())
                    });
                },
            )
        };
        frame();
        frame()
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Text(text) => Some(text.galley.job.text.clone()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn an_item_has_one_identity_line_with_its_attunement_before_the_rules() {
        let text = reader_text("Dragon Orb");
        assert!(!text.contains("Source classification"), "{text}");
        assert!(!text.contains("wondrous item · artifact"), "{text}");
        let identity = text.find("(Requires Attunement)").expect("the book's line");
        assert!(
            identity < text.find("An orb is an etched crystal").unwrap(),
            "{text}"
        );
    }

    #[test]
    fn a_class_that_grants_its_features_links_to_them_and_its_spell_list() {
        let text = reader_text("Wizard");
        assert!(text.contains("Class features"), "{text}");
        assert!(text.find("Arcane Recovery").unwrap() < text.find("Signature Spells").unwrap());
        assert!(text.contains("Wizard Spell List"), "{text}");
        assert!(text.contains("Wizard Progression"), "{text}");
        // Row 107 (AG3): the table's own title is its one heading.
        assert_eq!(text.matches("Wizard Features").count(), 1, "{text}");
        assert!(!text.contains("Class table"), "{text}");
        // Tight rows in one case, as a spell's are, and each option on its own row.
        assert!(text.contains("Primary ability\nIntelligence"), "{text}");
        assert!(text.contains("Option A\n"), "{text}");
        assert!(text.contains("Option B\n55 GP"), "{text}");
    }

    #[test]
    fn a_subclass_lists_its_features() {
        let text = reader_text("College of Lore");
        assert!(text.contains("Subclass features"), "{text}");
        assert!(text.contains("Cutting Words"), "{text}");
    }

    #[test]
    fn class_reader_is_readable_and_opens_features() {
        let loaded = load(None).unwrap();
        // Row 144 (FG6): no built-in name ends in a colon, which reads as a cut title.
        let colons: Vec<_> = loaded
            .notes
            .iter()
            .map(Note::name)
            .filter(|name| name.ends_with(':'))
            .collect();
        assert!(loaded.notes.len() > 100 && colons.is_empty(), "{colons:?}");
        // The old names stay aliases: a saved link `[[Bard Subclass:]]` opens the renamed entry,
        // which shows its new name, and Lookup's search finds it by the old name too.
        let index = crate::lookup::ContentIndex::load(&[], None).unwrap();
        for (old, new) in [
            ("Barbarian Subclass:", "Barbarian Subclass"),
            ("Bard Subclass:", "Bard Subclass"),
            ("Druid Subclass:", "Druid Subclass"),
        ] {
            let crate::world::Go::Item(at, _) = index.resolve_link(old, None) else {
                panic!("row-144-link: [[{old}]] does not resolve");
            };
            assert_eq!(index.items[at].entry.name, new, "row-144-link");
            let found = index.search(old);
            assert!(
                found
                    .first()
                    .is_some_and(|&at| index.items[at].entry.name == new),
                "row-144-search: {old} finds {:?}",
                found
                    .iter()
                    .take(3)
                    .map(|&at| &index.items[at].entry.name)
                    .collect::<Vec<_>>()
            );
        }
        let index = loaded
            .notes
            .iter()
            .position(|note| note.name() == "Barbarian")
            .unwrap();
        let mut library = Library {
            loaded: Some(loaded),
            ..Default::default()
        };
        library.open_entry(index);
        let ctx = egui::Context::default();
        let mut frame = |events| {
            ctx.run(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(390.0, 2400.0),
                    )),
                    events,
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        library.editor(ui, "Definition · class", &Campaigns::default())
                    });
                },
            )
        };
        frame(vec![]);
        let output = frame(vec![]);
        let texts: Vec<_> = output
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Text(text) => Some(text),
                _ => None,
            })
            .collect();
        // Stat-sized table cells can extend sideways inside their scroll area.
        // Prose and feature links must still fit without horizontal scrolling.
        assert!(output.shapes.iter().all(|shape| {
            let egui::Shape::Text(text) = &shape.shape else {
                return true;
            };
            text.pos.x + text.galley.size().x <= 390.0
                || (shape.clip_rect.right() <= 390.0
                    && text
                        .galley
                        .job
                        .sections
                        .iter()
                        .all(|section| section.format.font_id.size == Type::Stat.size()))
        }));
        let rendered = texts
            .iter()
            .map(|text| text.galley.job.text.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        // A label and its value, as a spell's facts are: on one row, in one case.
        assert!(rendered.contains("Primary ability\nStrength"), "{rendered}");
        assert!(rendered.contains("Starting equipment"));
        assert!(!rendered.contains("dnd2024:class_features/"));
        assert!(rendered.find("Rage").unwrap() < rendered.find("Relentless Rage").unwrap());
        // The class table names the feature too. The link is in the list below it. Its own
        // title is its heading, once (row 107).
        assert_eq!(
            rendered.matches("Barbarian Features").count(),
            1,
            "{rendered}"
        );
        assert!(!rendered.contains("Class table"), "{rendered}");
        let link = texts
            .iter()
            .rfind(|text| text.galley.job.text == "Relentless Rage")
            .unwrap();
        let pos = link.pos + link.galley.size() * 0.5;
        for pressed in [true, false] {
            frame(vec![
                egui::Event::PointerMoved(pos),
                egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed,
                    modifiers: Default::default(),
                },
            ]);
        }
        assert!(library.draft.contains("# Level 11: Relentless Rage"));
    }

    #[test]
    fn creature_read_view_shows_one_styled_stat_block() {
        let path = "definitions/creatures/vampire_familiar.md";
        let source = content::definitions::source_file(path).unwrap();
        let entry = Entry::parse(path, source).unwrap();
        for width in [390.0, 800.0] {
            let mut library = Library {
                loaded: Some(Loaded {
                    notes: vec![Note {
                        path: path.into(),
                        text: source.into(),
                        entry: Some(entry.clone()),
                        error: None,
                        fault: None,
                        owner: Owner::Builtin,
                    }],
                    diagnostics: vec![],
                    ..Default::default()
                }),
                selected: Some(0),
                draft: source.into(),
                ..Default::default()
            };
            let ctx = egui::Context::default();
            let mut rendered = String::new();
            for _ in 0..2 {
                let output = ctx.run(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(width, 2400.0),
                        )),
                        ..Default::default()
                    },
                    |ctx| {
                        egui::CentralPanel::default().show(ctx, |ui| {
                            library.editor(ui, "Definition · creature", &Campaigns::default())
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
            assert_eq!(rendered.matches("65 (10d8 + 20)").count(), 1, "{rendered}");
            for expected in [
                "ARMOR CLASS",
                "STR",
                "Save +5",
                "Vampiric Connection.",
                // An action is a tile with its name; its rules text is in hover.
                "Umbral Dagger",
                "Deathless Agility.",
                "except from its vampire master",
            ] {
                assert!(
                    rendered.contains(expected),
                    "Missing {expected}: {rendered}"
                );
            }
            for duplicate in [
                "Creature data",
                "Abilities data",
                "Stats data",
                "MOD SAVE",
                "AC 15",
            ] {
                assert!(!rendered.contains(duplicate), "{rendered}");
            }
            assert_eq!(library.draft, source);
        }
    }

    #[test]
    fn iter8_e_package_plain_readme_is_not_a_second_manifest() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        fs::rename(
            module.path.join("Package.md"),
            module.path.join("Module.md"),
        )
        .unwrap();
        fs::write(module.path.join("Package.md"), "Notes on this adventure.").unwrap();
        let folder = content::read_folder(&module.path).unwrap();
        assert!(folder.errors.is_empty(), "{:?}", folder.errors);
        let loaded = load(Some(&module)).unwrap();
        assert!(loaded.notes.iter().all(|note| note.error.is_none()));
        assert_eq!(
            loaded
                .notes
                .iter()
                .filter(|note| note.is_manifest())
                .count(),
            1
        );
        assert!(loaded.notes.iter().all(|note| note.path != "Package.md"));
        assert_eq!(
            crate::content_workspace::package_manifest_path(&module),
            "Module.md"
        );
        assert!(
            Catalogue::load(std::slice::from_ref(&module), None)
                .unwrap()
                .manifest_problems
                .is_empty()
        );
    }

    #[test]
    fn iter8_e_deleted_manifest_notice_and_action_do_not_guess_a_filename() {
        for filename in ["Package.md", "Module.md"] {
            let temp = tempfile::tempdir().unwrap();
            let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
            if filename == "Module.md" {
                fs::rename(module.path.join("Package.md"), module.path.join(filename)).unwrap();
            }
            fs::remove_file(module.path.join(filename)).unwrap();
            let loaded = load(Some(&module)).unwrap();
            assert_eq!(loaded.notes.len(), 1);
            assert_eq!(
                loaded.notes[0].error.as_deref(),
                Some(crate::content_workspace::DELETED_MANIFEST)
            );
            // Round 24 FC2: a deleted manifest is a warning, not the error of an unreadable one.
            assert!(
                loaded.notes[0].about_label(false).starts_with(icons::WARN),
                "iter-24-L-FC2-deleted: {}",
                loaded.notes[0].about_label(false)
            );
            assert!(!loaded.unreadable_counted, "iter-24-L-FC2-deleted-header");
            let catalogue = Catalogue::load(std::slice::from_ref(&module), None).unwrap();
            assert_eq!(
                catalogue.manifest_problems[&module.id],
                "The package manifest was deleted. Entries are still available. Write it again, or restore it from a backup."
            );
            let mut library = Library {
                loaded: Some(loaded),
                selected: Some(0),
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
                            library.editor_pane(ui, "Cannot be read", &Campaigns::default(), None);
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
            assert!(
                rendered.contains("The package manifest was deleted."),
                "{rendered}"
            );
            assert!(rendered.contains("Write the manifest again"), "{rendered}");
            assert!(!rendered.contains("Missing YAML frontmatter"), "{rendered}");
            assert!(rendered.contains("Test · Manifest deleted"), "{rendered}");
            assert!(rendered.contains("Show folder"), "{rendered}");
            assert!(!rendered.contains("Show file"), "{rendered}");
            assert!(!rendered.contains("Package.md"), "{rendered}");
            assert!(!rendered.contains("Module.md"), "{rendered}");
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
    fn iter9_e_empty_or_malformed_manifest_still_opens_for_repair() {
        for filename in ["Package.md", "Module.md"] {
            let temp = tempfile::tempdir().unwrap();
            let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
            if filename == "Module.md" {
                fs::rename(module.path.join("Package.md"), module.path.join(filename)).unwrap();
            }
            let path = module.path.join(filename);
            for bad in ["", "---\nsplat_id: [\n---\n"] {
                fs::write(&path, bad).unwrap();
                let loaded = load(Some(&module)).unwrap();
                let note = loaded.notes.iter().find(|note| note.is_manifest()).unwrap();
                assert_eq!(note.path, filename);
                assert_eq!(note.text, bad);
                assert!(note.error.is_some());
                assert!(
                    crate::campaign_content::repair_problem(filename, bad)
                        .0
                        .is_some()
                );
                let catalogue = Catalogue::load(std::slice::from_ref(&module), None).unwrap();
                assert!(catalogue.manifest_problems[&module.id].starts_with(filename));
                assert!(catalogue.manifest_problems[&module.id].contains("cannot be read"));
            }
            fs::write(&path, [0xff]).unwrap();
            let loaded = load(Some(&module)).unwrap();
            assert!(
                loaded
                    .notes
                    .iter()
                    .any(|note| note.path == filename && note.error.is_some())
            );
            // Moving a broken file out and back must not leave a stale parse fault.
            let moved = module.path.join("Moved.md");
            fs::rename(&path, &moved).unwrap();
            assert!(
                load(Some(&module))
                    .unwrap()
                    .notes
                    .iter()
                    .any(|note| note.is_manifest() && note.error.is_some())
            );
            fs::remove_file(&moved).unwrap();
            fs::write(
                &path,
                content::patch_fields(
                    &note_text("Test", "package", Some("test")),
                    &[("splat_id", Some(module.id.to_string().into()))],
                )
                .unwrap(),
            )
            .unwrap();
            assert!(
                Catalogue::load(std::slice::from_ref(&module), None)
                    .unwrap()
                    .manifest_problems
                    .is_empty()
            );
            assert!(
                load(Some(&module))
                    .unwrap()
                    .notes
                    .iter()
                    .all(|note| note.error.is_none())
            );
        }
    }

    #[test]
    fn iter9_e_manifest_write_rechecks_the_offered_action() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        let path = module.path.join("Package.md");
        fs::remove_file(&path).unwrap();
        // A missing-file Write must not become a replacement after an outside edit.
        let bad = "---\nsplat_id: [\n---\n";
        fs::write(&path, bad).unwrap();
        assert!(
            crate::content_workspace::ManifestWrite::write_as(&module, "Package.md", false)
                .is_err()
        );
        assert_eq!(fs::read(&path).unwrap(), bad.as_bytes());
        // Nor may a Replace become a creation after the file moves.
        let moved = module.path.join("Moved.md");
        fs::rename(&path, &moved).unwrap();
        assert!(
            crate::content_workspace::ManifestWrite::write_as(&module, "Package.md", true).is_err()
        );
        assert!(!path.exists());
        assert_eq!(fs::read(&moved).unwrap(), bad.as_bytes());
        // A README and a stale Write together still preserve the README.
        fs::write(&path, "My README.\n").unwrap();
        assert!(
            crate::content_workspace::ManifestWrite::write_as(&module, "Package.md", false)
                .is_err()
        );
        assert_eq!(fs::read(&path).unwrap(), b"My README.\n");
    }

    #[test]
    fn iter9_e_plain_manifest_named_readme_is_never_replaced() {
        // A divider and a heading underline are prose, not front matter.
        for (filename, readme) in [
            (
                "Package.md",
                "A package description without front matter.\n",
            ),
            ("Module.md", "A package description without front matter.\n"),
            ("Package.md", "Overview\n---\n\nNotes.\n\n---\n\nMore.\n"),
            ("Module.md", "Notes.\r\n\r\n---\r\n\r\nMore.\r\n"),
        ] {
            let temp = tempfile::tempdir().unwrap();
            let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
            fs::remove_file(module.path.join("Package.md")).unwrap();
            let path = module.path.join(filename);
            fs::write(&path, readme).unwrap();
            let loaded = load(Some(&module)).unwrap();
            let note = loaded.notes.iter().find(|note| note.is_manifest()).unwrap();
            assert_eq!(
                note.error.as_deref(),
                Some(crate::content_workspace::DELETED_MANIFEST)
            );
            assert_eq!(note.name(), "Test");
            assert_ne!(note.path, filename);
            let write = crate::content_workspace::ManifestWrite::write(&module).unwrap();
            assert!(!write.replaced());
            assert_ne!(write.path(), filename);
            assert_eq!(
                write.message(),
                format!(
                    "{} was written again. It now has the package’s name and ID in the current \
                     format.",
                    write.path()
                )
            );
            assert_eq!(fs::read(&path).unwrap(), readme.as_bytes());
            write.undo().unwrap();
            assert_eq!(fs::read(&path).unwrap(), readme.as_bytes());
            // Two README names leave no safe manifest destination.
            let other = if filename == "Package.md" {
                "Module.md"
            } else {
                "Package.md"
            };
            fs::write(module.path.join(other), readme).unwrap();
            assert!(crate::content_workspace::ManifestWrite::write(&module).is_err());
            assert_eq!(fs::read(&path).unwrap(), readme.as_bytes());
            assert_eq!(
                fs::read(module.path.join(other)).unwrap(),
                readme.as_bytes()
            );
        }
    }

    /// AD1: on reload, every Undo result checks that its files still hold what its action wrote.
    /// Each caller is given one outside change; the check never writes a file.
    #[test]
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: calls `ResultOrder::issue`, `Footer::result` or `save_row_state` (design::results::STAGED_ALLOWS)"
    )]
    fn iter14_e_changed_files_end_every_undo_result() {
        use crate::content_workspace::ContentEdits;
        let temp = tempfile::tempdir().unwrap();
        let world = crate::visual_tests::fixture::World::build(
            temp.path(),
            &temp.path().join("cache"),
            "undo-ends",
            crate::visual_tests::fixture::Seed::Campaign,
        )
        .unwrap();
        let mut app = world.app();
        let original = app.campaigns.modules()[0].clone();
        let campaign = app.campaigns.active_campaign().unwrap();
        let root = original.path.clone();
        let paths: Vec<String> = content::read_directory(&root)
            .unwrap()
            .0
            .into_iter()
            .filter(|entry| entry.kind != "package")
            .map(|entry| entry.path)
            .take(2)
            .collect();
        assert_eq!(paths.len(), 2, "fixture: two package entries");
        let outside = b"---\nsplat_kind: [\n---\nAn outside edit.\n".to_vec();
        // Writes `after` over each file and returns the edits, as an action would.
        let edit = |tag: &str| {
            let files = paths
                .iter()
                .map(|path| {
                    let before = fs::read_to_string(root.join(path)).unwrap();
                    let after = format!("{before}\n{tag}\n");
                    (root.clone(), path.clone(), before, after)
                })
                .collect();
            ContentEdits::write(files).unwrap()
        };
        // Changes one file outside Splat, reloads, and returns the raw bytes left on disk.
        let change_outside = |library: &mut Library| {
            let target = root.join(&paths[1]);
            let kept = fs::read(&target).unwrap();
            fs::write(&target, &outside).unwrap();
            let first = fs::read(root.join(&paths[0])).unwrap();
            library.forget_changed_undos();
            assert_eq!(
                fs::read(&target).unwrap(),
                outside,
                "the reload wrote nothing"
            );
            assert_eq!(fs::read(root.join(&paths[0])).unwrap(), first);
            fs::write(&target, kept).unwrap();
        };

        // `Undo separate entries`: the batch.
        let mut library = Library {
            identities_undo: Some((edit("batch"), "2 files are now separate entries.".into())),
            ..Default::default()
        };
        library.forget_changed_undos();
        assert!(
            library.identities_undo.is_some(),
            "unchanged files keep Undo"
        );
        change_outside(&mut library);
        assert!(library.identities_undo.is_none());
        // A deleted file of the batch also ends the result.
        library.identities_undo =
            Some((edit("deleted"), "2 files are now separate entries.".into()));
        library.forget_changed_undos();
        assert!(
            library.identities_undo.is_some(),
            "fixture: Undo before the delete"
        );
        let target = root.join(&paths[1]);
        let kept = fs::read(&target).unwrap();
        let first = fs::read(root.join(&paths[0])).unwrap();
        fs::remove_file(&target).unwrap();
        library.forget_changed_undos();
        assert!(library.identities_undo.is_none());
        assert!(!target.exists(), "the reload wrote nothing");
        assert_eq!(fs::read(root.join(&paths[0])).unwrap(), first);
        fs::write(&target, kept).unwrap();

        // `Undo separate entry`: one file.
        let text = fs::read_to_string(root.join(&paths[1])).unwrap();
        library.identity_undo = Some(IdentityUndo {
            root: root.clone(),
            path: paths[1].clone(),
            before: "before".into(),
            after: text,
        });
        library.forget_changed_undos();
        assert!(library.identity_undo.is_some());
        change_outside(&mut library);
        assert!(library.identity_undo.is_none());

        // `Undo copy`: the changed file is the copy.
        library.dependency_undos = vec![DependencyUndo {
            root: root.clone(),
            path: paths[0].clone(),
            before: "before".into(),
            after: fs::read_to_string(root.join(&paths[0])).unwrap(),
            copied_path: paths[1].clone(),
            copied_text: fs::read_to_string(root.join(&paths[1])).unwrap(),
            result: String::new(),
            order: design::ResultOrder::issue(),
        }];
        library.forget_changed_undos();
        assert!(!library.dependency_undos.is_empty());
        change_outside(&mut library);
        assert!(library.dependency_undos.is_empty());

        // `Undo link update`.
        library.link_edits.record(LinkResult {
            owner: Uuid::nil(),
            source: OfferSource {
                root: root.clone(),
                path: paths[0].clone(),
                text: String::new(),
            },
            edits: edit("links"),
            old: "Cinder Wolf".into(),
            new: "Ash Wolf".into(),
        });
        library.forget_changed_undos();
        assert!(!library.link_edits.is_empty());
        change_outside(&mut library);
        assert!(library.link_edits.is_empty());

        // `Undo source change` keeps its data for registration, and blocks both Undos.
        library.copied_rosters = Some(edit("rosters"));
        library.forget_changed_undos();
        assert!(!library.rosters_changed && !library.registration_changed);
        change_outside(&mut library);
        assert!(library.copied_rosters.is_some());
        assert!(library.rosters_changed && library.registration_changed);
        // The file holds the written text again: Undo comes back.
        library.forget_changed_undos();
        assert!(!library.rosters_changed && !library.registration_changed);
        library.copied_rosters = None;

        // `Undo package choice`: only the package choices count.
        let mut document = CampaignDocument::load(&campaign).unwrap();
        library.selection_undo = Some(SelectionUndo {
            campaign: campaign.clone(),
            before: Vec::new(),
            after: document.selections.clone(),
        });
        let campaign_file = campaign.path.join(&document.path);
        let edited = format!("{}\nA later note.\n", document.text);
        fs::write(&campaign_file, &edited).unwrap();
        library.forget_changed_undos();
        assert!(
            library.selection_undo.is_some(),
            "other campaign edits keep Undo"
        );
        let saved = fs::read(&campaign_file).unwrap();
        document = CampaignDocument::load(&campaign).unwrap();
        document.selections.clear();
        document.save().unwrap();
        let cleared = fs::read(&campaign_file).unwrap();
        library.forget_changed_undos();
        assert!(library.selection_undo.is_none());
        assert_eq!(fs::read(&campaign_file).unwrap(), cleared);
        fs::write(&campaign_file, saved).unwrap();

        // `Undo package registration`: a copied file or the campaign file changed.
        let copied = temp.path().join("copied");
        fs::create_dir(&copied).unwrap();
        for entry in content::read_directory(&original.path).unwrap().0 {
            let path = copied.join(&entry.path);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::copy(original.path.join(&entry.path), path).unwrap();
        }
        let mut library = Library {
            copied_package: Some(local_module(&copied).unwrap()),
            copy_name: "Reaches".into(),
            ..Default::default()
        };
        library.register_copy(&mut app.campaigns).unwrap();
        // Round 23 AD1: the result says what the registration chose, not the campaign's use now.
        assert_eq!(
            package_registered(
                &library.package_copy.as_ref().unwrap().module.name,
                library.copy_campaign_before.is_some()
            ),
            "Reaches is registered, with every entry chosen for this campaign.",
            "iter-23-L-AD1-chosen"
        );
        library.forget_changed_undos();
        assert!(!library.registration_changed);
        let manifest = copied.join("Module.md");
        let written = fs::read(&manifest).unwrap();
        fs::write(&manifest, &outside).unwrap();
        library.forget_changed_undos();
        assert!(library.registration_changed && library.package_copy.is_some());
        assert_eq!(fs::read(&manifest).unwrap(), outside);
        fs::write(&manifest, &written).unwrap();
        library.forget_changed_undos();
        assert!(!library.registration_changed);
        let (_, path, _, _) = library.copy_campaign_before.clone().unwrap();
        let campaign_text = fs::read(campaign.path.join(&path)).unwrap();
        fs::write(campaign.path.join(&path), &outside).unwrap();
        library.forget_changed_undos();
        assert!(library.registration_changed);
        fs::write(campaign.path.join(&path), campaign_text).unwrap();
        library.forget_changed_undos();
        assert!(!library.registration_changed);
        library
            .undo_package_registration(&mut app.campaigns)
            .unwrap();
    }

    #[test]
    fn iter10_e_stale_manifest_write_result_clears_on_reload() {
        use crate::content_workspace::ManifestWrite;
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        let path = module.path.join("Package.md");
        fs::remove_file(&path).unwrap();
        let mut write = Some(ManifestWrite::write(&module).unwrap());
        // The reload just after the write keeps the result and its Undo.
        ManifestWrite::forget_if_changed(&mut write);
        assert!(write.as_ref().is_some_and(ManifestWrite::on_disk));
        // The written file becomes a README: the result and its Undo go.
        let readme = "Notes on this adventure.\n";
        fs::write(&path, readme).unwrap();
        ManifestWrite::forget_if_changed(&mut write);
        assert!(write.is_none());
        assert_eq!(fs::read(&path).unwrap(), readme.as_bytes());
        // A deleted file also ends the result.
        fs::remove_file(&path).unwrap();
        let mut write = Some(ManifestWrite::write(&module).unwrap());
        fs::remove_file(&path).unwrap();
        ManifestWrite::forget_if_changed(&mut write);
        assert!(write.is_none());
        // A replacement whose file changed back to the broken text also ends.
        let broken = "---\nsplat_kind: [\n---\n";
        fs::write(&path, broken).unwrap();
        let mut write = Some(ManifestWrite::write(&module).unwrap());
        assert!(write.as_ref().unwrap().replaced());
        fs::write(&path, broken).unwrap();
        ManifestWrite::forget_if_changed(&mut write);
        assert!(write.is_none());
        assert_eq!(fs::read(&path).unwrap(), broken.as_bytes());
    }

    #[test]
    fn iter8_e_manifest_repair_message_names_actual_file() {
        for filename in ["Package.md", "Module.md"] {
            let temp = tempfile::tempdir().unwrap();
            let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
            if filename == "Module.md" {
                fs::rename(module.path.join("Package.md"), module.path.join(filename)).unwrap();
            }
            fs::write(module.path.join(filename), "---\nsplat_kind: [\n---\n").unwrap();
            let write = crate::content_workspace::ManifestWrite::write(&module).unwrap();
            assert_eq!(write.path(), filename);
            assert!(write.replaced());
            // Round 24 FF5: the result and its Undo (`Undo write <file>`) share one verb.
            // Round 27 FC5: a replaced file's result says that Undo puts the old text back.
            assert_eq!(
                write.message(),
                format!(
                    "{filename} was written again. It now has the package’s name and ID in the \
                     current format. Undo puts your old text back."
                )
            );
            write.undo().unwrap();
        }
    }

    #[test]
    fn iter8_e_new_package_manifest_is_package_md() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("new"), "New", "new").unwrap();
        assert!(module.path.join("Package.md").is_file());
        assert!(!module.path.join("Module.md").exists());
        let loaded = load(Some(&module)).unwrap();
        let manifest = loaded.notes.iter().find(|note| note.is_manifest()).unwrap();
        assert_eq!(manifest.path, "Package.md");
        assert_eq!(manifest.entry.as_ref().unwrap().id, module.id);
    }

    #[test]
    fn create_register_and_edit_preserves_identity_and_external_changes() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("module");
        let m = create_module(&root, "My Homebrew", "my_homebrew").unwrap();
        assert_eq!(local_module(&root).unwrap().id, m.id);
        assert!(create_module(&root, "Overwrite", "other").is_err());
        let text = note_text("Goblin", "creature", None);
        let entry = write_note(&root, "Goblin.md", None, &text).unwrap();
        assert!(write_note(&root, "Goblin.md", None, &text).is_err());
        let edited = format!("{text}New description\n");
        assert_eq!(
            write_note(&root, "Goblin.md", Some(&text), &edited)
                .unwrap()
                .id,
            entry.id
        );
        fs::write(root.join("Goblin.md"), "external edit").unwrap();
        assert!(write_note(&root, "Goblin.md", Some(&edited), &text).is_err());
        assert_eq!(
            fs::read_to_string(root.join("Goblin.md")).unwrap(),
            "external edit"
        );
        assert!(write_note(&root, "../escape.md", None, &text).is_err());
    }
    #[test]
    fn undo_discard_is_offered_only_to_the_note_it_came_from() {
        let temp = tempfile::tempdir().unwrap();
        let first = create_module(&temp.path().join("first"), "First", "first").unwrap();
        let second = create_module(&temp.path().join("second"), "Second", "second").unwrap();
        let note = |module: &Module, path: &str| Note {
            path: path.into(),
            text: String::new(),
            entry: None,
            error: None,
            fault: None,
            owner: Owner::Package(module.clone()),
        };
        let discarded = (Some(first.id), "a.md".to_string(), "words".to_string());
        assert!(discard_is_for(Some(&discarded), &note(&first, "a.md")));
        assert!(!discard_is_for(Some(&discarded), &note(&first, "b.md")));
        assert!(!discard_is_for(Some(&discarded), &note(&second, "a.md")));
    }

    #[test]
    fn library_frame_keeps_campaign_selection_and_draft_independent() {
        let temp = tempfile::tempdir().unwrap();
        let m = create_module(&temp.path().join("module"), "Module", "test").unwrap();
        let mut campaigns = Campaigns::at(temp.path().join("settings.json"));
        campaigns.register_module(m.clone()).unwrap();
        let mut library = Library {
            loaded: Some(load(Some(&m)).unwrap()),
            selected: Some(0),
            ..Default::default()
        };
        library.draft = library.loaded.as_ref().unwrap().notes[0].text.clone();
        library.draft.push_str("Unsaved description");
        let expected = library.draft.clone();
        library.search = "no matching entry".into();
        let ctx = egui::Context::default();
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| library.show(ui, &mut campaigns));
        });
        assert!(campaigns.active_campaign().is_none());
        assert_eq!(library.draft, expected);
        campaigns.unregister_module(m.id).unwrap();
        assert!(m.path.join("Package.md").exists());
    }

    #[test]
    fn loading_keeps_invalid_notes_available_for_repair() {
        let temp = tempfile::tempdir().unwrap();
        let m = create_module(&temp.path().join("module"), "Module", "test").unwrap();
        fs::write(
            m.path.join("Broken.md"),
            "---\nsplat_schema: 999\n---\nBroken",
        )
        .unwrap();
        let loaded = load(Some(&m)).unwrap();
        assert_eq!(loaded.notes.len(), 2);
        assert!(loaded.notes.iter().any(|n| n.error.is_some()));
        let loaded = load_all(std::slice::from_ref(&m), None).unwrap();
        assert!(
            loaded
                .diagnostic_texts()
                .iter()
                .any(|d| d.starts_with("Broken.md"))
        );
    }

    #[test]
    fn package_manifest_failure_keeps_library_entries_and_a_named_warning() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        fs::write(
            module.path.join("Wolf.md"),
            note_text("Wolf", "creature", None),
        )
        .unwrap();
        fs::write(module.path.join("Package.md"), "---\nsplat_id: [\n---\n").unwrap();
        for deleted in [false, true] {
            if deleted {
                fs::remove_file(module.path.join("Package.md")).unwrap();
            }
            let loaded = load_all(std::slice::from_ref(&module), None).unwrap();
            assert!(loaded.notes.iter().any(|note| note.name() == "Wolf"));
            assert!(
                loaded
                    .diagnostic_texts()
                    .iter()
                    .any(|message| message.contains("Package.md"))
            );
        }
    }

    #[test]
    fn iter3_d_a_readable_entry_named_like_a_manifest_stays_an_entry() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        fs::rename(
            module.path.join("Package.md"),
            module.path.join("Module.md"),
        )
        .unwrap();
        fs::write(
            module.path.join("Package.md"),
            note_text("Package Wolf", "creature", None),
        )
        .unwrap();
        fs::write(module.path.join("Module.md"), "---\nsplat_id: [\n---\n").unwrap();
        let loaded = load_all(std::slice::from_ref(&module), None).unwrap();
        let wolf = loaded
            .notes
            .iter()
            .find(|note| note.path == "Package.md")
            .unwrap();
        assert!(!wolf.is_manifest());
        assert!(loaded.in_scope(wolf, Scope::Package(module.id)));
        assert_eq!(unregister_count(&loaded.notes, module.id), 1);
    }

    #[test]
    fn iter3_d_manifests_never_enter_content_counts_or_results() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        fs::write(
            module.path.join("Wolf.md"),
            note_text("Wolf", "creature", None),
        )
        .unwrap();
        for text in [
            "---\nsplat_id: [\n---\n",
            "---\nsplat_kind: package\nsplat_id: [\n---\n",
        ] {
            fs::write(module.path.join("Package.md"), text).unwrap();
            let loaded = load_all(std::slice::from_ref(&module), None).unwrap();
            let manifest = loaded.notes.iter().find(|note| note.is_manifest()).unwrap();
            assert!(manifest.error.is_some());
            // Round 24 FC2: a manifest that cannot be read is an error on the About row.
            assert!(
                manifest.about_label(false).starts_with(icons::ERROR)
                    && !manifest.about_label(false).contains(icons::WARN),
                "iter-24-L-FC2: {}",
                manifest.about_label(false)
            );
            assert!(loaded.unreadable_counted, "iter-24-L-FC2-header");
            assert!(
                loaded
                    .diagnostic_texts()
                    .iter()
                    .any(|line| line.contains("Package.md"))
            );
            for scope in [Scope::All, Scope::Uses, Scope::Package(module.id)] {
                assert!(
                    !loaded.in_scope(manifest, scope),
                    "manifest entered {scope:?}"
                );
            }
            assert_eq!(
                loaded
                    .notes
                    .iter()
                    .filter(|note| loaded.in_scope(note, Scope::Package(module.id)))
                    .count(),
                1
            );
            assert_eq!(unregister_count(&loaded.notes, module.id), 1);
            assert_eq!(
                loaded
                    .notes
                    .iter()
                    .filter(|note| loaded.in_scope(note, Scope::Package(module.id))
                        && note.section() == Section::Unreadable)
                    .count(),
                0
            );
        }
    }

    /// Writes a package whose one item needs `needs`.
    fn package_needing(root: &Path, name: &str, namespace: &str, needs: &str) -> Module {
        let module = create_module(&root.join(name), name, namespace).unwrap();
        let note = splat::content::formats::fixture(
            "Fixture.md",
            &format!(
                "splat_id: {}\nsplat_kind: item\nsplat_key: {namespace}:items/own\n\
             splat_name: Own\nsplat_item: {{category: gear}}\nsplat_requires:\n  - key: {needs}\n",
                Uuid::new_v4()
            ),
            "",
        )
        .expect("schema-2 fixture");
        fs::write(module.path.join("Own.md"), note).unwrap();
        module
    }

    #[test]
    fn references_resolve_across_the_whole_library() {
        let temp = tempfile::tempdir().unwrap();
        let modules = [
            // One reference to built-in content, and one to the other package.
            package_needing(temp.path(), "First", "first", "dnd2024:equipment/battleaxe"),
            package_needing(temp.path(), "Second", "second", "first:items/own"),
        ];
        let mut loaded = load_all(&modules, None).unwrap();
        assert_eq!(loaded.diagnostic_texts(), Vec::<&str>::new());
        // A save checks again, and that check must agree with the first.
        loaded.refresh_diagnostics();
        assert_eq!(loaded.diagnostic_texts(), Vec::<&str>::new());
    }

    #[test]
    fn a_reference_to_nothing_is_still_reported() {
        let temp = tempfile::tempdir().unwrap();
        let module = package_needing(temp.path(), "First", "first", "missing:items/gone");
        let loaded = load_all(&[module], None).unwrap();
        assert_eq!(
            loaded.diagnostic_texts().len(),
            1,
            "{:?}",
            loaded.diagnostic_texts()
        );
        assert!(loaded.diagnostic_texts()[0].contains("missing:items/gone"));
    }

    #[test]
    fn a_save_keeps_the_problems_that_kept_content_from_loading() {
        let temp = tempfile::tempdir().unwrap();
        let mut module = package_needing(temp.path(), "First", "first", "first:items/own");
        module.path = temp.path().join("deleted");
        let mut loaded = load_all(&[module], None).unwrap();
        assert_eq!(
            loaded.diagnostic_texts().len(),
            1,
            "{:?}",
            loaded.diagnostic_texts()
        );
        loaded.refresh_diagnostics();
        assert_eq!(
            loaded.diagnostic_texts().len(),
            1,
            "{:?}",
            loaded.diagnostic_texts()
        );
    }
    /// Bytes that are not UTF-8 text, as a file saved in another encoding holds.
    const NOT_TEXT: [u8; 3] = [0xff, 0xfe, b'\n'];

    fn sd1_campaign(root: &Path) -> Campaign {
        let campaign = Campaign {
            id: Uuid::new_v4(),
            name: "Test campaign".into(),
            path: root.join("campaign"),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        fs::create_dir_all(campaign.path.join("Notes")).unwrap();
        CampaignDocument::new(&campaign).unwrap().save().unwrap();
        campaign
    }

    /// The text the reader paints for the selected note, after two frames.
    fn sd1_painted(library: &mut Library) -> String {
        sd1_painted_at(library, (900.0, 1400.0))
    }

    /// The text the reader paints for the selected note, after two frames at `size`.
    fn sd1_painted_at(library: &mut Library, size: (f32, f32)) -> String {
        let ctx = egui::Context::default();
        crate::design::theme::install(&ctx);
        let mut texts = Vec::new();
        for _ in 0..2 {
            let output = ctx.run(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(size.0, size.1),
                    )),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        library.editor(ui, "Entry", &Campaigns::default());
                    });
                },
            );
            texts = output
                .shapes
                .into_iter()
                .filter_map(|shape| match shape.shape {
                    egui::Shape::Text(text) => Some(text.galley.job.text.clone()),
                    _ => None,
                })
                .collect();
        }
        texts.join("\n")
    }

    /// Types `words` into the open Markdown editor, as the DM does: focus, then text input.
    fn sd1_type(library: &mut Library, words: &str) {
        sd1_input(library, vec![vec![egui::Event::Text(words.to_owned())]]);
    }

    /// Sends each batch of input to the open Markdown editor, one frame each, in one context
    /// (so the editor's own Undo remembers the earlier frames). The editor has focus.
    fn sd1_input(library: &mut Library, batches: Vec<Vec<egui::Event>>) {
        let ctx = egui::Context::default();
        crate::design::theme::install(&ctx);
        let note = &library.loaded.as_ref().unwrap().notes[library.selected.unwrap()];
        let id = note.markdown_field().id();
        let input = |events, time| egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(900.0, 1400.0),
            )),
            events,
            time: Some(time),
            ..Default::default()
        };
        let frames = [vec![], vec![]]
            .into_iter()
            .chain(batches.into_iter().flat_map(|batch| {
                // A quiet frame after each batch, a second later, so the editor settles its Undo.
                [batch, vec![]]
            }));
        for (at, events) in frames.enumerate() {
            let _ = ctx.run(input(events, at as f64 * 2.0), |ctx| {
                ctx.memory_mut(|memory| memory.request_focus(id));
                egui::CentralPanel::default().show(ctx, |ui| {
                    library.editor(ui, "Entry", &Campaigns::default());
                });
            });
        }
    }

    fn sd1_select(loaded: Loaded, owner: Option<Uuid>, path: &str) -> Library {
        let selected = loaded
            .notes
            .iter()
            .position(|note| note.path == path && note.owner.id() == owner)
            .unwrap_or_else(|| panic!("{path} is listed"));
        let draft = loaded.notes[selected].text.clone();
        Library {
            loaded: Some(loaded),
            selected: Some(selected),
            draft,
            ..Default::default()
        }
    }

    /// SD1: both load paths (package and campaign) keep why a file could not be read. The plain
    /// cases of the same callers stay as they were: a readable entry has no fault, and a file with
    /// a YAML fault keeps its parse message and its text for repair.
    #[test]
    fn iter20_e_sd1_both_load_paths_keep_the_read_fault() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        let campaign = sd1_campaign(temp.path());
        let wolf = note_text("Wolf", "creature", None);
        fs::write(module.path.join("Wolf.md"), &wolf).unwrap();
        fs::write(module.path.join("Bad.md"), NOT_TEXT).unwrap();
        let yaml = wolf.replace("splat_kind: creature", "splat_kind: [");
        fs::write(module.path.join("Yaml.md"), &yaml).unwrap();
        fs::write(campaign.path.join("Notes/Bad.md"), NOT_TEXT).unwrap();
        fs::write(campaign.path.join("Notes/Yaml.md"), &yaml).unwrap();
        let rule = note_text("Rule", "rule", None);
        fs::write(campaign.path.join("Notes/Rule.md"), &rule).unwrap();

        let loaded = load_all(std::slice::from_ref(&module), Some(&campaign)).unwrap();
        let note = |owner: Uuid, path: &str| {
            loaded
                .notes
                .iter()
                .find(|note| note.owner.id() == Some(owner) && note.path == path)
                .unwrap_or_else(|| panic!("{path} is listed"))
        };
        for (owner, path) in [(module.id, "Bad.md"), (campaign.id, "Notes/Bad.md")] {
            let bad = note(owner, path);
            let fault = bad.fault.as_ref().expect("SD1-kind-kept");
            assert_eq!(
                fault.kind,
                crate::world::file_fault::Kind::NotText,
                "{path}"
            );
            assert_eq!(fault.path, path, "SD1-fault-names-the-file");
            assert_eq!(
                bad.error.as_deref(),
                Some("Not a text file"),
                "SD1-plain-cause: {path}"
            );
            assert!(bad.text.is_empty() && bad.entry.is_none());
            // FD2 (round 21): the row's second line names the short cause, as World's row does.
            let at = loaded
                .notes
                .iter()
                .position(|n| std::ptr::eq(n, bad))
                .unwrap();
            // Round 24 AE1, FE2: the cause follows the error mark, as the reader marks it.
            assert!(
                loaded.subtitles()[at].starts_with(&format!("{} Not a text file", icons::ERROR)),
                "iter21-E-FD2-row, iter-24-L-AE1: {path}: {}",
                loaded.subtitles()[at]
            );
            // The list's problem text is the plain cause, never the system's words.
            assert!(
                !loaded
                    .diagnostic_texts()
                    .iter()
                    .any(|text| text.contains("UTF-8") || text.contains("stream")),
                "{:?}",
                loaded.diagnostic_texts()
            );
        }
        for (owner, path) in [(module.id, "Yaml.md"), (campaign.id, "Notes/Yaml.md")] {
            let broken = note(owner, path);
            assert!(broken.fault.is_none(), "a YAML fault is not a read fault");
            // Must stay true: a YAML fault's row still names its line.
            let at = loaded
                .notes
                .iter()
                .position(|n| std::ptr::eq(n, broken))
                .unwrap();
            assert!(
                loaded.subtitles()[at].starts_with(&format!("{} Line ", icons::ERROR)),
                "iter21-E-FD2-yaml-line, iter-24-L-AE1: {path}: {}",
                loaded.subtitles()[at]
            );
            assert_eq!(broken.text, yaml, "a YAML fault keeps its text for repair");
            assert!(
                broken
                    .error
                    .as_deref()
                    .is_some_and(|e| e != "Not a text file")
            );
        }
        for (owner, path) in [(module.id, "Wolf.md"), (campaign.id, "Notes/Rule.md")] {
            let plain = note(owner, path);
            assert!(plain.fault.is_none() && plain.error.is_none() && plain.entry.is_some());
            // Must stay true: a readable row has no mark.
            let at = loaded
                .notes
                .iter()
                .position(|n| std::ptr::eq(n, plain))
                .unwrap();
            assert!(
                !loaded.subtitles()[at].contains(icons::ERROR)
                    && !loaded.subtitles()[at].contains(icons::WARN),
                "{path}: {}",
                loaded.subtitles()[at]
            );
        }
        // Round 24: a counted file that cannot be read gives the header count the error tone.
        assert!(loaded.unreadable_counted, "iter-24-L-AE1-header");
        assert_eq!(fs::read(module.path.join("Bad.md")).unwrap(), NOT_TEXT);
        assert_eq!(
            fs::read(campaign.path.join("Notes/Bad.md")).unwrap(),
            NOT_TEXT
        );
    }

    /// SD1: a file the system refuses to read names that cause, from the error's kind.
    #[cfg(unix)]
    #[test]
    fn iter20_e_sd1_a_refused_read_names_its_own_cause() {
        use std::os::unix::fs::PermissionsExt;
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        let file = module.path.join("Locked.md");
        fs::write(&file, note_text("Locked", "creature", None)).unwrap();
        fs::set_permissions(&file, fs::Permissions::from_mode(0o000)).unwrap();
        if fs::read(&file).is_ok() {
            // Root reads every file; this case cannot be made here.
            fs::set_permissions(&file, fs::Permissions::from_mode(0o644)).unwrap();
            return;
        }
        let loaded = load(Some(&module));
        fs::set_permissions(&file, fs::Permissions::from_mode(0o644)).unwrap();
        let loaded = loaded.unwrap();
        let note = loaded.notes.iter().find(|n| n.path == "Locked.md").unwrap();
        assert_eq!(
            note.fault.as_ref().map(|fault| fault.kind),
            Some(crate::world::file_fault::Kind::ReadDenied)
        );
        assert_eq!(note.error.as_deref(), Some("No permission to read"));
    }

    /// SD1: the reader names the real cause with Show file, Try again and Technical details, and
    /// offers no editor link, no repair of empty text and no source editing. A campaign note with
    /// a YAML fault, in the same reader, still offers both: the check can fail.
    #[test]
    fn iter20_e_sd1_reader_names_the_fault_and_blocks_source_repair() {
        let temp = tempfile::tempdir().unwrap();
        let campaign = sd1_campaign(temp.path());
        fs::write(campaign.path.join("Notes/Bad.md"), NOT_TEXT).unwrap();
        let yaml = note_text("Yaml", "rule", None).replace("splat_kind: rule", "splat_kind: [");
        fs::write(campaign.path.join("Notes/Yaml.md"), &yaml).unwrap();
        // Another loaded note, whose UUID a repair draft can take.
        let other = note_text("Other", "rule", None);
        fs::write(campaign.path.join("Notes/Other.md"), &other).unwrap();

        let loaded = load_all(&[], Some(&campaign)).unwrap();
        let mut library = sd1_select(loaded, Some(campaign.id), "Notes/Yaml.md");
        let yaml_painted = sd1_painted(&mut library);
        for words in ["Edit in Obsidian", "Fix in Edit Markdown", "Show file"] {
            assert!(
                yaml_painted.contains(words),
                "control: {words}\n{yaml_painted}"
            );
        }

        // iter-22-E-SD1: a repair draft's notice and state come from its current parse. The saved
        // file stays unreadable on the owner line until Save.
        library.source = true;
        let painted = sd1_painted(&mut library);
        for words in [
            "Line 4 opens a list",
            "Invalid",
            "Campaign · Cannot be read",
        ] {
            assert!(
                painted.contains(words),
                "SD1-saved-fault: {words}\n{painted}"
            );
        }
        // The stored error has its own words: the old code fell back to them.
        let stored = library.loaded.as_ref().unwrap().notes[library.selected.unwrap()]
            .error
            .clone()
            .unwrap();
        let stored = stored.split(" at line").next().unwrap().to_owned();
        assert!(!stored.contains("opens a list"), "{stored}");
        let valid = yaml.replace("splat_kind: [", "splat_kind: rule");
        library.draft = valid.clone();
        let painted = sd1_painted(&mut library);
        for words in ["opens a list", stored.as_str()] {
            assert!(
                !painted.contains(words),
                "SD1-valid-notice: {words}\n{painted}"
            );
        }
        assert!(!painted.contains("Invalid"), "SD1-valid-state\n{painted}");
        assert!(painted.contains("Unsaved"), "SD1-valid-unsaved\n{painted}");
        assert!(
            painted.contains("Campaign · Cannot be read"),
            "SD1-owner-line-stays\n{painted}"
        );
        // The reader shows the same draft: it has no notice either.
        library.source = false;
        let painted = sd1_painted(&mut library);
        for words in ["opens a list", stored.as_str()] {
            assert!(
                !painted.contains(words),
                "SD1-reader-notice: {words}\n{painted}"
            );
        }
        assert!(
            !painted.contains("Fix in Edit Markdown"),
            "SD1-reader-fix\n{painted}"
        );
        // Edited back to a new fault: the notice names the draft's line, not the saved one.
        library.source = true;
        library.draft = valid.replace("splat_name: Yaml", "splat_name: [");
        let painted = sd1_painted(&mut library);
        assert!(
            painted.contains("Line 5 opens a list"),
            "SD1-new-fault\n{painted}"
        );
        assert!(!painted.contains("Line 4"), "SD1-old-fault-gone\n{painted}");
        assert!(painted.contains("Invalid"), "SD1-invalid-again\n{painted}");
        // Discard goes back to the saved text and its fault.
        library.draft = yaml.clone();
        let painted = sd1_painted(&mut library);
        assert!(
            painted.contains("Line 4 opens a list"),
            "SD1-discard\n{painted}"
        );
        assert!(painted.contains("Invalid"), "SD1-discard-state\n{painted}");
        // A draft that parses but takes another note's UUID: Save would refuse it, so the notice
        // names that cause and the state is Invalid. Save leaves the file as it was.
        let id_line = |text: &str| {
            text.lines()
                .find(|line| line.starts_with("splat_id: "))
                .unwrap()
                .to_owned()
        };
        let taken = valid.replace(&id_line(&valid), &id_line(&other));
        assert_ne!(taken, valid);
        assert!(Entry::parse("Notes/Yaml.md", &taken).is_ok());
        library.draft = taken.clone();
        let painted = sd1_painted(&mut library);
        let cause = "Another entry already uses this UUID or key.";
        assert!(painted.contains(cause), "SD1-taken-notice\n{painted}");
        assert!(painted.contains("Invalid"), "SD1-taken-state\n{painted}");
        assert!(
            !painted.contains("Unsaved"),
            "SD1-taken-not-unsaved\n{painted}"
        );
        library.save_selected(false);
        assert_eq!(
            fs::read(campaign.path.join("Notes/Yaml.md")).unwrap(),
            yaml.as_bytes(),
            "SD1-taken-not-written"
        );
        assert_eq!(library.draft, taken, "SD1-taken-draft-kept");
        let painted = sd1_painted(&mut library);
        assert!(painted.contains(cause), "SD1-taken-after-save\n{painted}");
        // Must stay true: a draft Save accepts has no notice, and Save writes it.
        library.draft = valid.clone();
        let painted = sd1_painted(&mut library);
        assert!(!painted.contains(cause), "SD1-valid-again\n{painted}");
        library.save_selected(false);
        assert_eq!(
            fs::read_to_string(campaign.path.join("Notes/Yaml.md")).unwrap(),
            valid,
            "SD1-valid-written"
        );

        // Round 23 lead note: a ready state means Save would accept the draft. The saved note
        // reads now. A draft of it that parses but takes another UUID is refused by Save's own
        // check, so its state is Invalid with Save's reason, Save is not offered (here or in the
        // close dialog), and a Save attempt leaves the file's bytes as they are.
        let path = campaign.path.join("Notes/Yaml.md");
        assert!(
            library.loaded.as_ref().unwrap().notes[library.selected.unwrap()]
                .error
                .is_none()
        );
        let saved = fs::read(&path).unwrap();
        let moved_id =
            format!("{valid}\nA new line.\n").replace(&id_line(&valid), &id_line(&other));
        assert!(Entry::parse("Notes/Yaml.md", &moved_id).is_ok());
        library.draft = moved_id.clone();
        let painted = sd1_painted(&mut library);
        let reason = "Keep this entry’s UUID and kind unchanged.";
        assert!(painted.contains(reason), "ready-refused-reason\n{painted}");
        assert!(
            painted.contains("Invalid"),
            "ready-refused-state\n{painted}"
        );
        assert!(
            !painted.contains("Unsaved"),
            "ready-refused-not-unsaved\n{painted}"
        );
        assert!(!library.can_save_and_close(), "ready-refused-no-close-save");
        library.save_selected(false);
        assert_eq!(fs::read(&path).unwrap(), saved, "ready-refused-not-written");
        assert_eq!(library.draft, moved_id, "ready-refused-draft-kept");
        // Must stay true: the same note's draft that Save accepts is Unsaved, with no reason.
        library.draft = format!("{valid}\nA new line.\n");
        let painted = sd1_painted(&mut library);
        assert!(
            painted.contains("Unsaved"),
            "ready-accepted-unsaved\n{painted}"
        );
        assert!(
            !painted.contains(reason),
            "ready-accepted-no-reason\n{painted}"
        );
        assert!(library.can_save_and_close(), "ready-accepted-close-save");

        // An action-time failure: the folder is read-only, so Save fails on the disk. The state
        // slot says Not saved with Save's own reason, not Unsaved, and the file keeps its bytes.
        use std::os::unix::fs::PermissionsExt;
        let notes = campaign.path.join("Notes");
        let writable = fs::metadata(&notes).unwrap().permissions();
        fs::set_permissions(&notes, fs::Permissions::from_mode(0o555)).unwrap();
        let accepted = library.draft.clone();
        // The failed-Save check counts each run, and `DraftView` counts each read of the words.
        let failure_work =
            crate::test_support::CachedWork::start("Failed-Save check", &mut library, |library| {
                let (reached, work) = library.failure_checks.get();
                crate::test_support::Counts { reached, work }
            });
        library.save_selected(false);
        let reason = library
            .save_failure
            .as_ref()
            .map(|failure| failure.reason.clone())
            .expect("disk-failure-recorded");
        let painted = sd1_painted(&mut library);
        assert!(
            painted.contains("Not saved"),
            "disk-failure-state\n{painted}"
        );
        // The check reads none of the words, even while it is new.
        failure_work
            .warm(&mut library, painted.contains("Not saved"), 0)
            .expect("disk-failure-check-reached");
        assert!(
            painted.contains(&reason),
            "disk-failure-reason {reason}\n{painted}"
        );
        assert!(
            !painted.contains("Unsaved"),
            "disk-failure-not-unsaved\n{painted}"
        );
        assert_eq!(fs::read(&path).unwrap(), saved, "disk-failure-not-written");
        assert_eq!(library.draft, accepted, "disk-failure-draft-kept");
        // iter-24-B FL1, lookalike outside the rule: a folder inside the campaign refused the
        // write, not the file or the campaign folder, so the owner line stays Editable.
        assert_eq!(
            library
                .save_failure
                .as_ref()
                .and_then(|failure| failure.read_only),
            None,
            "FL1-folder-inside-not-read-only"
        );
        assert!(
            painted.contains("Campaign · Editable") && !painted.contains("Read only"),
            "FL1-folder-inside-owner-line\n{painted}"
        );
        // Unchanged frames and a resize keep the failure and never read the draft's words to do
        // so. Each paint is two frames, and each frame checks.
        let steady = failure_work
            .steady(&mut library, |library| {
                [
                    (900.0, 1400.0),
                    (900.0, 1400.0),
                    (960.0, 640.0),
                    (1920.0, 1080.0),
                ]
                .into_iter()
                .all(|size| sd1_painted_at(library, size).contains("Not saved"))
            })
            .expect("disk-failure-no-draft-scan");
        assert!(
            steady.reached >= 8,
            "disk-failure-checked-each-frame: {steady:?}"
        );
        // Only that draft owns it: the next edit in the editor ends it, and going back to the
        // same words does not bring it back.
        assert!(library.source, "fixture: Edit Markdown is open");
        failure_work
            .refresh(&mut library, 0, |library| {
                sd1_type(library, "More.");
                assert_ne!(
                    library.draft, accepted,
                    "fixture: the editor took the words"
                );
                let painted = sd1_painted(library);
                painted.contains("Unsaved") && !painted.contains("Not saved")
            })
            .expect("disk-failure-edit-ends");
        // Edit, then the editor's Undo back to the very words that failed: still ended.
        let failed_again = |library: &mut Library| {
            library.save_selected(false);
            assert!(
                sd1_painted(library).contains("Not saved"),
                "fixture: the Save failed again"
            );
        };
        library.draft = accepted.clone();
        library.draft_revision += 1;
        failed_again(&mut library);
        sd1_input(
            &mut library,
            vec![
                vec![egui::Event::Text("More.".into())],
                vec![egui::Event::Key {
                    key: egui::Key::Z,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::COMMAND,
                }],
            ],
        );
        assert_eq!(
            library.draft, accepted,
            "fixture: Undo gave back the failed words"
        );
        let painted = sd1_painted(&mut library);
        assert!(
            !painted.contains("Not saved") && painted.contains("Unsaved"),
            "disk-failure-stays-ended\n{painted}"
        );
        // Never on another file, even with the same words: the failure is Yaml.md's.
        failed_again(&mut library);
        let yaml_at = library.selected.unwrap();
        let other_at = library
            .loaded
            .as_ref()
            .unwrap()
            .notes
            .iter()
            .position(|note| note.path == "Notes/Other.md")
            .unwrap();
        library.selected = Some(other_at);
        library.draft = accepted.clone();
        let painted = sd1_painted(&mut library);
        assert!(
            !painted.contains("Not saved"),
            "disk-failure-not-on-other\n{painted}"
        );
        library.selected = Some(yaml_at);
        let painted = sd1_painted(&mut library);
        assert!(
            !painted.contains("Not saved"),
            "disk-failure-owner-ended\n{painted}"
        );
        // It comes back on another failed Save, and a Save that succeeds ends it.
        library.save_selected(false);
        assert!(
            sd1_painted(&mut library).contains("Not saved"),
            "disk-failure-again"
        );
        fs::set_permissions(&notes, writable).unwrap();
        library.save_selected(false);
        let painted = sd1_painted(&mut library);
        assert!(
            !painted.contains("Not saved"),
            "disk-failure-save-ends\n{painted}"
        );
        assert_eq!(
            fs::read_to_string(&path).unwrap(),
            accepted,
            "disk-failure-then-written"
        );
        // iter-24-B FL1, AL1, FL2, SL1: the file itself is read only. Save's reason says what to
        // do next, and the owner line says Read only while that failure lasts. The next edit
        // ends both.
        let file_permissions = fs::metadata(&path).unwrap().permissions();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o444)).unwrap();
        library.draft = format!("{accepted}\nA locked file.\n");
        library.draft_revision += 1;
        library.save_selected(false);
        let painted = sd1_painted(&mut library);
        fs::set_permissions(&path, file_permissions).unwrap();
        assert_eq!(
            library
                .save_failure
                .as_ref()
                .and_then(|failure| failure.read_only),
            Some(ReadOnly::File),
            "FL1-kind"
        );
        assert!(
            painted.contains("Campaign · Read only")
                && !painted.contains("Campaign · Editable")
                && painted.contains("This file is read only. Make it writable, then Save again."),
            "FL1-read-only-owner-line\n{painted}"
        );
        assert_eq!(
            fs::read_to_string(&path).unwrap(),
            accepted,
            "FL1-not-written"
        );
        sd1_type(&mut library, "More.");
        let painted = sd1_painted(&mut library);
        assert!(
            painted.contains("Campaign · Editable") && !painted.contains("Read only"),
            "FL1-edit-ends\n{painted}"
        );
        // The repair-link flow: an entry that cannot be read opens as a repair draft in Edit
        // Markdown (as `repair_file` opens it). Its fixed draft parses, so Save would accept it,
        // and the disk refuses the write: the same Not saved state, for this draft.
        let broken = note_text("Broken", "rule", None).replace("splat_kind: rule", "splat_kind: [");
        fs::write(notes.join("Broken.md"), &broken).unwrap();
        let loaded = load_all(&[], Some(&campaign)).unwrap();
        let mut repair = sd1_select(loaded, Some(campaign.id), "Notes/Broken.md");
        repair.source = true;
        assert!(
            sd1_painted(&mut repair).contains("Invalid"),
            "fixture: a repair draft"
        );
        let fixed = broken.replace("splat_kind: [", "splat_kind: rule");
        sd1_input(
            &mut repair,
            vec![
                vec![egui::Event::Key {
                    key: egui::Key::A,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::COMMAND,
                }],
                vec![egui::Event::Text(fixed.clone())],
            ],
        );
        assert_eq!(
            repair.draft, fixed,
            "fixture: the editor holds the fixed draft"
        );
        let writable = fs::metadata(&notes).unwrap().permissions();
        fs::set_permissions(&notes, fs::Permissions::from_mode(0o555)).unwrap();
        repair.save_selected(false);
        let painted = sd1_painted(&mut repair);
        let reason = repair
            .save_failure
            .as_ref()
            .map(|failure| failure.reason.clone())
            .expect("repair-disk-failure-recorded");
        fs::set_permissions(&notes, writable).unwrap();
        assert!(
            painted.contains("Not saved")
                && painted.contains(&reason)
                && !painted.contains("Unsaved"),
            "repair-disk-failure-state\n{painted}"
        );
        assert_eq!(
            fs::read_to_string(notes.join("Broken.md")).unwrap(),
            broken,
            "repair-disk-failure-not-written"
        );

        // Lookalike outside the rule: another identity is not in the draft's parse, so a draft
        // that parses keeps the stored fault and Invalid until the file changes.
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        let manifest = module.path.join("Package.md");
        let text = fs::read_to_string(&manifest).unwrap();
        let other = text.replace(&module.id.to_string(), &Uuid::new_v4().to_string());
        assert_ne!(other, text);
        fs::write(&manifest, &other).unwrap();
        let mut library = sd1_select(load(Some(&module)).unwrap(), Some(module.id), "Package.md");
        library.source = true;
        library.draft = format!("{other}\nA description.\n");
        assert!(
            crate::campaign_content::repair_problem("Package.md", &library.draft)
                .0
                .is_none()
        );
        let painted = sd1_painted(&mut library);
        assert!(
            painted.contains("another identity"),
            "SD1-lookalike-notice\n{painted}"
        );
        assert!(
            painted.contains("Invalid"),
            "SD1-lookalike-state\n{painted}"
        );

        let loaded = load_all(&[], Some(&campaign)).unwrap();
        let mut library = sd1_select(loaded, Some(campaign.id), "Notes/Bad.md");
        // Edit Markdown was chosen before (for example by a repair link): it does not open.
        library.source = true;
        let painted = sd1_painted(&mut library);
        assert!(!library.source, "SD1-no-source-editing");
        for words in [
            // Round 24 FE5: the file line names the file; the error says `this file`.
            "Splat cannot read this file as text.",
            "Splat reads notes as UTF-8 text.",
            "Then choose Try again.",
            "Try again",
            "Show file",
            "Technical details",
            "Campaign · Cannot be read",
        ] {
            assert!(painted.contains(words), "SD1-shows: {words}\n{painted}");
        }
        for words in [
            "Edit in Obsidian",
            "Fix in Edit Markdown",
            "Missing YAML frontmatter",
            "Missing frontmatter",
            "Save",
            "cannot read Notes/Bad.md",
        ] {
            assert!(!painted.contains(words), "SD1-hides: {words}\n{painted}");
        }
        assert_eq!(
            fs::read(campaign.path.join("Notes/Bad.md")).unwrap(),
            NOT_TEXT
        );
    }

    /// SD1: a package manifest that is not text is never replaced by the manifest repair.
    #[test]
    fn iter20_e_sd1_an_unreadable_manifest_offers_no_replacement() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        fs::write(module.path.join("Package.md"), NOT_TEXT).unwrap();
        let loaded = load(Some(&module)).unwrap();
        let mut library = sd1_select(loaded, Some(module.id), "Package.md");
        assert!(
            library.loaded.as_ref().unwrap().notes[library.selected.unwrap()].is_manifest(),
            "the manifest is found by its file name"
        );
        let painted = sd1_painted(&mut library);
        assert!(
            painted.contains("Splat cannot read this file as text."),
            "{painted}"
        );
        assert!(painted.contains("Try again"), "{painted}");
        assert!(
            !painted.contains("Replace with a basic"),
            "SD1-no-replace\n{painted}"
        );
        assert!(!painted.contains("Write the manifest again"), "{painted}");
        assert_eq!(fs::read(module.path.join("Package.md")).unwrap(), NOT_TEXT);
    }

    /// SD1, action-time failure: a note that read as text when opened stops reading. Save, a
    /// later disk check and Reload each keep the draft or its Undo, name the cause, and never
    /// change the file's bytes. When the file reads again, the fault ends.
    #[test]
    fn iter20_e_sd1_a_file_that_stops_reading_keeps_the_draft_and_its_bytes() {
        let temp = tempfile::tempdir().unwrap();
        // A package in an Obsidian vault, so its file line offers Edit in Obsidian while it reads.
        fs::create_dir(temp.path().join(".obsidian")).unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        let file = module.path.join("Wolf.md");
        let text = note_text("Wolf", "creature", None);
        fs::write(&file, &text).unwrap();
        let ctx = egui::Context::default();
        let mut library = sd1_select(load(Some(&module)).unwrap(), Some(module.id), "Wolf.md");
        let draft = format!("{text}\nMy draft.\n");
        library.draft = draft.clone();
        let painted = sd1_painted(&mut library);
        assert!(
            painted.contains("Edit in Obsidian"),
            "control: the vault link shows\n{painted}"
        );
        fs::write(&file, NOT_TEXT).unwrap();

        // Save: the write reads the file first and refuses it.
        library.save_selected(false);
        let fault = |library: &Library| {
            library
                .disk_change
                .as_ref()
                .and_then(|change| change.fault.as_ref())
                .map(|fault| fault.kind)
        };
        assert_eq!(
            fault(&library),
            Some(crate::world::file_fault::Kind::NotText),
            "SD1-save"
        );
        assert!(library.error.is_none(), "{:?}", library.error);
        assert_eq!(library.draft, draft, "SD1-save-keeps-draft");
        assert_eq!(fs::read(&file).unwrap(), NOT_TEXT, "SD1-save-keeps-bytes");
        let painted = sd1_painted(&mut library);
        for words in [
            "Splat cannot read this file as text. Your draft is safe here until you choose.",
            "Try again",
            "Reload and lose my draft",
            "Technical details",
        ] {
            assert!(
                painted.contains(words),
                "SD1-conflict-shows: {words}\n{painted}"
            );
        }
        for words in [
            "Save my draft again",
            "Overwrite with my draft",
            "was deleted on disk",
            "Edit in Obsidian",
        ] {
            assert!(
                !painted.contains(words),
                "SD1-conflict-hides: {words}\n{painted}"
            );
        }

        // A later disk check (focus, or Try again) finds the same fault, not a deleted file.
        library.disk_change = None;
        library.refresh_destination(&module.path, &ctx);
        assert_eq!(
            fault(&library),
            Some(crate::world::file_fault::Kind::NotText),
            "SD1-check"
        );
        assert!(library.disk_change.as_ref().unwrap().text.is_none());

        // The file reads as before again: the fault ends and the draft stays.
        fs::write(&file, &text).unwrap();
        library.refresh_destination(&module.path, &ctx);
        assert!(library.disk_change.is_none(), "SD1-fault-ends");
        assert_eq!(library.draft, draft);

        // Reload and lose my draft: the note becomes the unreadable file; Undo keeps the draft.
        fs::write(&file, NOT_TEXT).unwrap();
        library.refresh_destination(&module.path, &ctx);
        let selected = library.selected.unwrap();
        library.reload_note_from_disk(selected);
        let note = &library.loaded.as_ref().unwrap().notes[selected];
        assert_eq!(
            note.fault.as_ref().map(|fault| fault.kind),
            Some(crate::world::file_fault::Kind::NotText),
            "SD1-reload"
        );
        assert!(note.text.is_empty() && note.entry.is_none());
        assert_eq!(
            library.discarded.as_ref().map(|(_, _, text)| text.as_str()),
            Some(draft.as_str()),
            "SD1-reload-undo"
        );
        assert!(library.stale, "the next frame reads every file again");
        assert_eq!(fs::read(&file).unwrap(), NOT_TEXT, "SD1-reload-keeps-bytes");

        // With no draft, a disk check reloads, and the new reading keeps the fault.
        let mut library = sd1_select(
            {
                fs::write(&file, &text).unwrap();
                load(Some(&module)).unwrap()
            },
            Some(module.id),
            "Wolf.md",
        );
        fs::write(&file, NOT_TEXT).unwrap();
        library.refresh_destination(&module.path, &ctx);
        assert!(library.disk_change.is_none() && library.stale);
        let loaded = load(Some(&module)).unwrap();
        let note = loaded.notes.iter().find(|n| n.path == "Wolf.md").unwrap();
        assert!(note.fault.is_some(), "SD1-reread");
        assert_eq!(fs::read(&file).unwrap(), NOT_TEXT);
    }

    /// FD2: the choice after a package copy's registration names what it changes (encounters),
    /// in the words of its result.
    #[test]
    fn iter20_e_fd2_the_copy_choice_names_what_it_changes() {
        assert!(
            USE_COPY.contains("encounters"),
            "FD2-names-encounters: {USE_COPY}"
        );
        let choice = USE_COPY.trim_end_matches(" in encounters").to_lowercase();
        assert!(
            USE_COPY_RESULT
                .to_lowercase()
                .contains(&format!("encounters now {choice}")),
            "FD2-same-words: {USE_COPY} / {USE_COPY_RESULT}"
        );
    }

    /// SD1: a save that looks for its moved file names an unreadable file in plain words.
    #[test]
    fn iter20_e_sd1_a_moved_file_search_names_an_unreadable_file() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        let text = note_text("Wolf", "creature", None);
        fs::write(module.path.join("Wolf.md"), &text).unwrap();
        let mut library = sd1_select(load(Some(&module)).unwrap(), Some(module.id), "Wolf.md");
        library.draft = format!("{text}\nMy draft.\n");
        fs::remove_file(module.path.join("Wolf.md")).unwrap();
        fs::write(module.path.join("Other.md"), NOT_TEXT).unwrap();
        library.save_selected(false);
        let error = library
            .error
            .clone()
            .expect("the search stops at the unreadable file");
        assert!(
            error.starts_with("Splat cannot read Other.md as text."),
            "SD1-moved: {error}"
        );
        assert!(!error.contains("stream"), "{error}");
        assert!(!module.path.join("Wolf.md").exists(), "nothing is written");
        assert_eq!(fs::read(module.path.join("Other.md")).unwrap(), NOT_TEXT);
    }
}
