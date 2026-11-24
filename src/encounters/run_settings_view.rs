//! Campaign → Rules and runs, free of any toolkit, for the GPUI tab
//! (`src/gui/pages/campaigns/run_settings.rs`). The footer is `run_settings_footer`; the save
//! row's rule is `run_row_of`.
#![warn(deprecated)]
use super::{
    CHOOSE_MOVE, Encounters, FooterPress, KeptResult, OptionsCommit, commit_options, rule_choices,
};
use crate::campaigns::Campaigns;
use crate::design;
use crate::design::outcome::plain::PlainLine;
use crate::encounter_settings::{Grouping, GroupingWords as _};
use crate::persistence::Campaign;
use crate::state_context::StateContext;
use uuid::Uuid;

/// The form's values as the tab shows them.
pub struct RunForm {
    pub folder: String,
    /// `Roll NPC initiative`'s choices, and the chosen one.
    pub groupings: Vec<&'static str>,
    pub grouping: usize,
    pub roster_ties: bool,
    /// Each inspiration rule's line, and the chosen one (only with more than one rule).
    pub inspiration: Option<(Vec<String>, Option<usize>)>,
    /// No rule chosen yet among several.
    pub inspiration_unset: bool,
    /// The house rule notes, each with whether it is in play.
    pub house_rules: Vec<(Uuid, String, bool)>,
}

/// The save row's state, and whether the form changed.
pub struct RunRow<'a> {
    pub state: design::DraftState<'a>,
    pub changed: bool,
    /// The run files the folder change leaves (read once a frame, for the folder the fields
    /// left), and that folder: the footer's warning says them.
    pub left: (String, Result<Vec<std::path::PathBuf>, String>),
}

impl Encounters {
    /// The tab's work before it draws: the campaign's encounters and settings are read.
    pub fn run_settings_prepare(&mut self, ctx: &StateContext, campaigns: &Campaigns) {
        self.poll(ctx, campaigns);
    }

    /// The form, once the settings are read.
    pub fn run_form(&self) -> Option<RunForm> {
        let options = &self.snapshot.as_ref()?.options;
        let rules = &options.inspiration_rules;
        let inspiration = (rules.len() > 1).then(|| {
            let lines = rules
                .iter()
                .map(|rule| format!("{}: {}", rule.name, rule.effect.effect()))
                .collect();
            let chosen = options
                .inspiration_choice
                .and_then(|id| rules.iter().position(|rule| rule.id == id));
            (lines, chosen)
        });
        Some(RunForm {
            folder: options.folder.clone(),
            groupings: Grouping::ALL.iter().map(|mode| mode.label()).collect(),
            grouping: Grouping::ALL
                .iter()
                .position(|mode| *mode == options.initiative.grouping)
                .unwrap_or(0),
            roster_ties: options.initiative.roster_ties,
            inspiration_unset: rules.len() > 1 && options.inspiration().is_none(),
            inspiration,
            house_rules: options
                .house_rules
                .iter()
                .map(|(id, name)| (*id, name.clone(), options.in_play.contains(id)))
                .collect(),
        })
    }

    fn options_mut(&mut self) -> Option<&mut crate::encounter_settings::Options> {
        self.snapshot.as_mut().map(|snapshot| &mut snapshot.options)
    }

    pub fn set_run_folder(&mut self, words: &str) {
        if let Some(options) = self.options_mut() {
            options.folder = words.to_owned();
        }
    }

    pub fn set_grouping(&mut self, at: usize) {
        if let (Some(options), Some(mode)) = (self.options_mut(), Grouping::ALL.get(at)) {
            options.initiative.grouping = *mode;
        }
    }

    pub fn set_roster_ties(&mut self, on: bool) {
        if let Some(options) = self.options_mut() {
            options.initiative.roster_ties = on;
        }
    }

    pub fn set_inspiration(&mut self, at: usize) {
        if let Some(options) = self.options_mut()
            && let Some(rule) = options.inspiration_rules.get(at)
        {
            options.inspiration_choice = Some(rule.id);
        }
    }

    /// A house rule note's switch.
    pub fn set_in_play(&mut self, id: Uuid, on: bool) {
        if let Some(options) = self.options_mut() {
            if on {
                if !options.in_play.contains(&id) {
                    options.in_play.push(id);
                }
            } else {
                options.in_play.retain(|other| *other != id);
            }
        }
    }

    /// The save row: the word, the repair of an unread choice, Save's
    /// weight and why it waits. It ends the last Save's `Saved` and the discard's draft at the
    /// form's first edit.
    pub fn run_row<'a>(&mut self, active: &Campaign) -> Option<RunRow<'a>> {
        let run_open = self.has_run();
        let draft_open = self.draft_dirty();
        let snapshot = self.snapshot.as_ref()?;
        Some(run_row_of(
            snapshot,
            active,
            (run_open, draft_open),
            &mut self.options_saved,
            &mut self.discarded_options,
        ))
    }

    /// `Discard`: the saved settings come back, with an Undo in the footer.
    pub fn discard_run_settings(&mut self) {
        let Some(snapshot) = self.snapshot.as_mut() else {
            return;
        };
        self.discarded_options = Some(std::mem::replace(
            &mut snapshot.options,
            snapshot.saved_options.clone(),
        ));
        Self::record_kept(
            &mut self.kept,
            self.active.as_ref(),
            KeptResult::OptionsDiscarded,
        );
    }

    /// `Save run settings` (or Ctrl+S).
    pub fn save_run_settings(
        &mut self,
        ctx: &StateContext,
        active: &Campaign,
        campaigns: &Campaigns,
    ) {
        let Some(snapshot) = &self.snapshot else {
            return;
        };
        let result = commit_options(snapshot, active, OptionsCommit::Save);
        self.options_committed(ctx, active, campaigns.modules().to_vec(), result);
    }

    /// The tab's footer lines (`run_settings_footer`).
    pub fn run_settings_lines(
        &mut self,
        ctx: &StateContext,
        active: &Campaign,
        campaigns: &Campaigns,
    ) -> Vec<PlainLine<FooterPress>> {
        let run_open = self.has_run();
        let draft_open = self.draft_dirty();
        let frame = design::outcome::PaneFrame::new();
        let mut messages = design::outcome::Messages::new();
        // The run files the folder change leaves, for the folder the fields hold now: the
        // footer's warning says them and offers its two choices (egui's `run_settings`).
        let form_left = self.snapshot.as_ref().map(|snapshot| {
            (
                snapshot.options.folder.clone(),
                snapshot
                    .options
                    .runs_left_in(&snapshot.saved_options, active),
            )
        });
        self.run_settings_footer(
            ctx,
            &frame,
            &mut messages,
            active,
            campaigns.modules(),
            None,
            form_left,
            run_open,
            draft_open,
        );
        messages.into_plain()
    }

    /// A press in the tab's footer: the discard's Undo, or the folder warning's choices.
    pub fn run_settings_press(
        &mut self,
        ctx: &StateContext,
        active: &Campaign,
        campaigns: &Campaigns,
        press: design::outcome::Press<FooterPress>,
    ) {
        let commit = match press.id {
            FooterPress::Undo => {
                if let Some(ticket) = press.ticket {
                    self.undo_by_ticket(ticket, None);
                }
                return;
            }
            FooterPress::MoveRuns => OptionsCommit::MoveRuns,
            FooterPress::SaveWithoutMoving => OptionsCommit::WithoutMoving,
            _ => return,
        };
        if !self.has_run()
            && !self.draft_dirty()
            && let Some(snapshot) = &self.snapshot
        {
            let result = commit_options(snapshot, active, commit);
            self.options_committed(ctx, active, campaigns.modules().to_vec(), result);
        }
    }
}

/// The save row's rule of Rules and runs (`run_row`): the word (`Unsaved`, `Saved` while the last Save's settings
/// hold, `No changes`), the repair of an unread choice, Save's weight and why it waits, Discard
/// and the campaign lock. It ends the last Save's `Saved` at the form's first edit, at another
/// campaign and when the file's settings changed outside Splat, and the discard's result at
/// the next edit. `open`: a run is open, and an encounter draft is open.
pub(super) fn run_row_of<'a>(
    snapshot: &super::Snapshot,
    active: &Campaign,
    (run_open, draft_open): (bool, bool),
    options_saved: &mut Option<(Uuid, crate::encounter_settings::Options)>,
    discarded_options: &mut Option<crate::encounter_settings::Options>,
) -> RunRow<'a> {
    // The unread rule choices' lines, for the save row's state. Their warnings are the
    // footer's.
    let rule_lines: Vec<String> = rule_choices(snapshot, active.id)
        .into_iter()
        .map(|(line, _)| line)
        .collect();
    let changed = snapshot.options != snapshot.saved_options;
    // SD7: the last Save's `Saved` ends at the form's first edit (as the discard's result
    // does), at another campaign, and when the file's settings changed outside Splat.
    if changed
        || options_saved.as_ref().is_some_and(|(campaign, saved)| {
            *campaign != active.id || *saved != snapshot.saved_options
        })
    {
        *options_saved = None;
    }
    // The run files the folder change leaves, read once a frame for the folder the fields
    // left; the footer's warning says them.
    let left = snapshot
        .options
        .runs_left_in(&snapshot.saved_options, active);
    let run_files = left.as_ref().map_or(0, Vec::len);
    let folder_changed =
        snapshot.options.directory(active).ok() != snapshot.saved_options.directory(active).ok();
    // M4: `Save run settings` is the commit in every state. While the folder change leaves
    // saved runs, it waits in its place, and the folder warning's two choices (the footer's)
    // commit, so a second press where Save was never moves runs.
    let choose_move = run_files > 0 && !run_open;
    let repair = (!rule_lines.is_empty()).then(|| {
        // Save is the repair here: it writes the shown settings over the unread choice.
        let lines = match rule_lines.split_last() {
            Some((last, [])) => format!("line {last}"),
            Some((last, before)) => format!("lines {} and {last}", before.join(", ")),
            None => String::new(),
        };
        format!("Save replaces the unread choice on {lines} with these settings")
    });
    let mut state = design::DraftState::new(
        design::save_row::DraftOrigin::Saved,
        if changed {
            design::save_row::Changed::Yes
        } else {
            design::save_row::Changed::No
        },
        design::save_row::Primary::Draft,
    )
    .locks_campaign();
    if options_saved.is_some() {
        state = state.just_saved();
    }
    if let Some(write) = repair {
        state = state.repairs(write);
    }
    if run_open {
        state = state.waits("Close the open run to save run settings.");
    } else if folder_changed && left.is_err() {
        state = state.waits("Could not read saved runs. Check the old run folder before saving.");
    } else if choose_move {
        state = state.commits_elsewhere(CHOOSE_MOVE);
    }
    if draft_open {
        state = state.waits("Save or discard the open encounter first.");
    }
    if changed {
        *discarded_options = None;
    }
    RunRow {
        state,
        changed,
        left: (snapshot.options.folder.clone(), left),
    }
}
