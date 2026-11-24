//! Campaign → Rules and runs, free of any toolkit, for the GPUI tab
//! (`src/gui/pages/campaigns/run_settings.rs`). The footer is `run_settings_footer`, which the
//! egui tab (`Encounters::run_settings`) calls too; the save row's state follows the egui tab's
//! rule, step for step.
use super::{
    CHOOSE_MOVE, Encounters, FooterPress, KeptResult, OptionsCommit, commit_options, rule_choices,
};
use crate::campaigns::Campaigns;
use crate::design;
use crate::design::outcome::plain::PlainLine;
use crate::encounter_settings::{Grouping, GroupingWords as _};
use crate::persistence::Campaign;
use eframe::egui;
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
}

impl Encounters {
    /// The tab's work before it draws: the campaign's encounters and settings are read.
    pub fn run_settings_prepare(&mut self, ctx: &egui::Context, campaigns: &Campaigns) {
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

    /// The save row (the egui tab's rule): the word, the repair of an unread choice, Save's
    /// weight and why it waits. It ends the last Save's `Saved` and the discard's draft at the
    /// form's first edit, as the egui tab does.
    pub fn run_row<'a>(&mut self, active: &Campaign) -> Option<RunRow<'a>> {
        let run_open = self.has_run();
        let snapshot = self.snapshot.as_ref()?;
        let rule_lines: Vec<String> = rule_choices(snapshot, active.id)
            .into_iter()
            .map(|(line, _)| line)
            .collect();
        let changed = snapshot.options != snapshot.saved_options;
        if changed
            || self.options_saved.as_ref().is_some_and(|(campaign, saved)| {
                *campaign != active.id || *saved != snapshot.saved_options
            })
        {
            self.options_saved = None;
        }
        let snapshot = self.snapshot.as_ref()?;
        let left = snapshot.options.runs_left_in(&snapshot.saved_options, active);
        let run_files = left.as_ref().map_or(0, Vec::len);
        let folder_changed = snapshot.options.directory(active).ok()
            != snapshot.saved_options.directory(active).ok();
        let choose_move = run_files > 0 && !run_open;
        let repair = (!rule_lines.is_empty()).then(|| {
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
        if self.options_saved.is_some() {
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
        if self.draft_dirty() {
            state = state.waits("Save or discard the open encounter first.");
        }
        if changed {
            self.discarded_options = None;
        }
        Some(RunRow { state, changed })
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
    pub fn save_run_settings(&mut self, ctx: &egui::Context, active: &Campaign, campaigns: &Campaigns) {
        let Some(snapshot) = &self.snapshot else {
            return;
        };
        let result = commit_options(snapshot, active, OptionsCommit::Save);
        self.options_committed(ctx, active, campaigns.modules().to_vec(), result);
    }

    /// The tab's footer lines (`run_settings_footer`).
    pub fn run_settings_lines(
        &mut self,
        ctx: &egui::Context,
        active: &Campaign,
        campaigns: &Campaigns,
    ) -> Vec<PlainLine<FooterPress>> {
        let run_open = self.has_run();
        let draft_open = self.draft_dirty();
        let frame = design::outcome::PaneFrame::headless();
        let mut messages = design::outcome::Messages::new(&frame, "run_settings_footer", 0.0);
        self.run_settings_footer(
            ctx,
            &frame,
            &mut messages,
            active,
            campaigns.modules(),
            None,
            None,
            run_open,
            draft_open,
        );
        messages.into_plain()
    }

    /// A press in the tab's footer: the discard's Undo, or the folder warning's choices.
    pub fn run_settings_press(
        &mut self,
        ctx: &egui::Context,
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
