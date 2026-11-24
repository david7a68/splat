//! What the run's views read and do (the GPUI views are `gui::pages::run`): the words, sizes and
//! columns of the rows, the presses of the run's messages, the Arrivals words, the close
//! dialog's words, the Undo forms and the keys. The condition grid and the other-effect field
//! stay here for the Encounters page's egui group form until that page's strip.
#![allow(deprecated)] // Old message functions; Wave 2 moves this file to `design::outcome`.
use super::{Command, Panel, Participant, PartySync, Phase, Run, SHEET_SOURCE};
use crate::{
    combat_sync::SyncState,
    design::{self, Action, Density, RunScale, Status, Type, colors, icons, reader as style},
    encounter_settings::{Grouping, GroupingWords},
};
use eframe::egui::{self, vec2};
use splat::content::overrides::{BY_HAND, Row};
use splat::content::{Entry, Reference};
use std::sync::Arc;
use uuid::Uuid;

pub const READ_ALOUD: &str = "Read-aloud and notes";
/// FI10: Arrivals says what the fight becomes when a group joins.
pub const FIGHT_AFTER: &str = "Fight after: ";
/// Two lines of controls in a frame.
pub const DOCK_HEIGHT: f32 = 36.0 + 4.0 + 28.0 + 8.0;
/// The dock's width from which Damage has its key hint, and Temp HP and Roll their full forms.
pub const DOCK_ALL_WORDS_FROM: f32 = design::turn_heading::ADD_WORDS_FROM;
/// FK1 (iter-27-R): the dock's name slot, one width in every fight and for every row, so the
/// controls after it keep one x. A name that `Heading` does not hold on one line wraps in it.
pub const DOCK_NAME_WIDTH: f32 = 8.0 * design::L;
/// Room for the note under a long roster, past the scroll area's clip margin.
pub const OVERFLOW_LINE: f32 = 22.0;
/// A name-only row when the list has no other way to fit. Still a 24 px target and more.
pub const TIGHT_ROW: f32 = 25.0;
/// The name column a row needs, in units of its name size, to keep a state column too.
const NAME_ROOM: f32 = 8.0;
/// X8: the Conditions field's words while the dock has no room for `Effect`.
pub const OTHER_EFFECT_TIMED: &str = "Other effect, such as Bless 10 rounds";
/// X7: what the Conditions tab records, and what Splat does with a time the DM types.
pub const CONDITIONS_HELP: &str = "No HP, AC or roll changes by itself. A time you type counts down at the end of that creature’s turn: Bless 10 rounds goes from 10 to 9. Other effects stay local and do not sync.";

/// Run toggles use a selected surface; the turn action keeps the accent fill.
#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
)]
pub fn toggle(
    ui: &mut egui::Ui,
    label: &str,
    icon: &str,
    selected: bool,
    name: &str,
    density: Density,
) -> egui::Response {
    let button = egui::Button::new(
        Type::Body
            .rich(ui, format!("{icon} {label}"))
            .color(colors().text),
    )
    .selected(selected)
    .min_size(vec2(0.0, density.control()));
    // FH1, FI3: an off toggle keeps the frame's padding, with nothing painted, so turning it
    // on moves no word and no neighbour.
    let button = if selected {
        button.fill(colors().selected)
    } else {
        button
            .frame(true)
            .fill(egui::Color32::TRANSPARENT)
            .stroke(egui::Stroke::NONE)
    };
    let response = ui.add(button);
    design::named(ui, &response, name);
    response
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// How many times a group row scanned the run's participants for that group's joins.
    static GROUP_SCANS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// How often the Arrivals summaries and the Arrivals reasons were worked out (Astra 2): an
    /// unchanged frame works out none of them.
    pub(super) static PREPARED: std::cell::Cell<(usize, usize)> =
        const { std::cell::Cell::new((0, 0)) };
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// How many toggle widths the conditions grid measured.
    static CONDITION_MEASURES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// Where an official condition is among a participant's conditions (Exhaustion at any level).
fn found_condition(p: &Participant, name: &str) -> Option<usize> {
    p.conditions
        .iter()
        .position(|c| c.same(name) || (name == "Exhaustion" && c.name().starts_with("Exhaustion")))
}

/// An official condition's words on its toggle while it holds (`Bless 9` with a count), or
/// `None` while it does not.
pub fn condition_label(p: &Participant, name: &str) -> Option<String> {
    found_condition(p, name).map(|at| {
        let c = &p.conditions[at];
        c.count().map_or_else(|| name.to_owned(), |_| c.words())
    })
}

/// A press on an official condition's toggle: it holds now, or it ends.
pub fn condition_press(p: &Participant, index: usize, name: &str) -> Command {
    found_condition(p, name).map_or_else(
        || Command::Condition(index, name.to_owned()),
        |at| Command::RemoveCondition(index, at),
    )
}

/// The effects and unofficial conditions the Conditions tab lists under `Other effects`, by
/// their place: an effect whose count the toggles show (`counts`) stays on its toggle.
pub fn other_effects(p: &Participant, counts: bool) -> Vec<(usize, String)> {
    p.conditions
        .iter()
        .enumerate()
        .filter(|(_, c)| {
            super::recognized_condition(c.name()).is_none()
                || (c.is_effect() && !(counts && c.count().is_some()))
        })
        .map(|(at, c)| (at, c.words()))
        .collect()
}

/// `Other effect…`'s words, added: an Exhaustion level sets the level of the one it has.
pub fn other_effect_command(p: &Participant, index: usize, typed: String) -> Command {
    let exhausted = p
        .conditions
        .iter()
        .any(|c| c.name().starts_with("Exhaustion"));
    let level = typed.starts_with("Exhaustion") && !super::Condition::read(&typed).is_effect();
    if exhausted && level {
        set_exhaustion(index, exhaustion_level(&typed))
    } else {
        Command::Condition(index, typed)
    }
}

/// Blank means roll; invalid text must never silently request a roll.
fn arrival_total(text: &str) -> Result<Option<i64>, &'static str> {
    let text = text.trim();
    if text.is_empty() {
        Ok(None)
    } else {
        design::parse_signed(text)
            .map(Some)
            .ok_or("Enter a whole number, or clear the field to roll.")
    }
}

/// Every smallest `Fight after: …` an Arrivals heading can show: each difficulty word and
/// each reason. The heading reserves the widest of them (point 3 of the fourth review).
pub const WIDEST_ANSWERS: [&str; 6] = [
    "Fight after: Low",
    "Fight after: Moderate",
    "Fight after: High",
    "Fight after: Above High",
    "Fight after: XP not known",
    "Fight after: Level not known",
];

/// How one Arrivals heading shares its width (points 2 and 3 of the round 29 reviews). The
/// heading holds the title and the difficulty line only: a group's join status (`Added in
/// round 1`, `For …`, `Changed`) has ONE place in every group, its member line (FB12, AM8), so
/// no heading gives a fact of its line to a status and every heading gives way in one order.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeadingSplit {
    /// The title's room.
    pub title: f32,
    /// The difficulty line's room.
    pub room: f32,
    /// The line has its own line under the heading.
    pub below: bool,
}

/// `whole` is the heading's width, `title` the whole title's, `floor` its first word with `…`
/// (or the whole title when shorter), and `widest` the widest smallest `Fight after: …` the row
/// can show. R1: beside the title or on its own line is decided by `widest` (fourth review), so
/// a change of difficulty never moves the answer or Add; the title shortens to leave `widest`,
/// never below `floor`. Where even `floor` and the answer do not fit together (the rule for the
/// boundary), the answer takes its own line under the heading at the whole width, and the title
/// keeps the heading.
pub fn heading_split(whole: f32, title: f32, floor: f32, widest: f32) -> HeadingSplit {
    if whole < floor + design::L + widest {
        return HeadingSplit {
            title: title.min(whole),
            room: whole,
            below: true,
        };
    }
    let title = title.min(whole - design::L - widest).max(floor);
    HeadingSplit {
        title,
        room: whole - title - design::L,
        below: false,
    }
}

/// The repair a group's fault in Arrivals offers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GroupRepair {
    /// Choose the package with the creature (Campaign → Packages).
    Packages,
    /// Open the group's editor.
    EditGroup,
}

/// An Arrivals heading, fitted for a view that measures its own words (`Run::arrival_heading`).
pub struct ArrivalHeading {
    /// How the title and the line share the heading.
    pub split: HeadingSplit,
    /// The `Fight after: …` line at its room, or none (the group's totals show instead).
    pub line: Option<ArrivalLine>,
    /// The group's own facts: `Enemies · 3 members · XP 150`.
    pub totals: String,
    /// The line's description, for its accessible name and hover.
    pub described: Option<String>,
    /// The heading's hover: the group's totals, its notes, the join rule and the budget.
    pub hover: String,
}

/// A fitted difficulty line: the muted words before the form, then the form's parts (the word
/// in the text colour); `wrap` when the protected answer is wider than its room and wraps.
pub struct ArrivalLine {
    pub lead: String,
    pub form: Vec<(String, bool)>,
    pub wrap: bool,
}

/// The words of a blocked Add whose group would roll with no bonus (R35: the same value gives
/// the lock and the painted words).
const NO_BONUS: &str = "The first member has no initiative bonus. Type a total.";

/// Why a group's Add, its drop and its Enter do not run now (R35, ruling M1). This ONE value
/// gives the lock, the hover and the accessible description, and the painted words: the
/// member-fault rows under the group paint `Faults`; the footer below the list paints the
/// others, named by the group (`AddWaits::footer`), so no group row moves when they come or go.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AddWaits<'a> {
    /// The group's own faults (`RunGroup::problem`).
    Faults(&'a str),
    /// The typed total cannot be read (`arrival_total`).
    Total(&'static str),
    /// The total is blank, so the group rolls, and its first member has no bonus to roll with.
    NoBonus,
}

impl<'a> AddWaits<'a> {
    fn words(self) -> &'a str {
        match self {
            Self::Faults(words) => words,
            Self::Total(words) => words,
            Self::NoBonus => NO_BONUS,
        }
    }

    /// The footer's line for it: `<group>: <reason>`. `None` for the faults, which the
    /// group's own rows say.
    fn footer(self, group: &str) -> Option<String> {
        match self {
            Self::Faults(_) => None,
            Self::Total(_) | Self::NoBonus => Some(format!("{group}: {}", self.words())),
        }
    }
}

/// A group whose members' changes make another creature says `Changed`, with its muted pencil,
/// after its members (FJ5). The no-break space keeps the two on one line.
pub fn changed_mark(group: &super::RunGroup) -> Option<String> {
    group
        .members
        .iter()
        .any(|member| member.overrides.changes_creature())
        .then(|| format!("{}\u{a0}Changed", icons::CHANGED))
}

pub fn arrival_add_word(group: &super::RunGroup) -> String {
    if group.warnings.is_empty() {
        "Add".to_owned()
    } else {
        format!("Add {}", group.joinable_count())
    }
}

/// FL4: the line under a pending sync block's heading in the ended view. The heading above it
/// names the character, and `Stop sync for …` under it is the way to close now, so the line
/// says only what waits and that it needs no step (two lines at 960, not three).
const PENDING_WORDS: &str =
    "Its changes have not reached D&D Beyond yet. Splat tries again by itself.";

/// FL4, AL2, FL2, SL3 (iter-29 G178): the pending line names what waits, from the values that
/// Splat would send and D&D Beyond does not have yet: `Its change to 49/62 HP has not reached
/// D&D Beyond yet. Splat tries again by itself.` HP has the form of the conflict's buttons.
/// With nothing to compare, the general words.
pub fn pending_words(sync: &crate::combat_sync::CombatSync) -> String {
    let max = sync.max_hp().unwrap_or(0);
    let mut changed: Vec<String> = Vec::new();
    let mut add = |words: String| {
        if !changed.contains(&words) {
            changed.push(words);
        }
    };
    for (key, value) in sync.values() {
        let sheet = sync.confirmed().and_then(|sheet| sheet.get(key));
        if sheet == Some(value) {
            continue;
        }
        // HP holds two values; each one that waits is named, and only those (Astra, PR 371).
        if key == "hp" {
            let field = |value: Option<&serde_json::Value>, name: &str| {
                value.and_then(|value| value[name].as_i64())
            };
            let removed = field(Some(value), "removedHitPoints");
            if removed != field(sheet, "removedHitPoints") {
                add(format!(
                    "{} HP",
                    crate::combat_sync::fraction((max - removed.unwrap_or(0)).max(0), max)
                ));
            }
            let temporary = field(Some(value), "temporaryHitPoints");
            if temporary != field(sheet, "temporaryHitPoints") {
                add(format!("{} temporary HP", temporary.unwrap_or(0)));
            }
            continue;
        }
        let words = match key.as_str() {
            "death" => "death saves".to_owned(),
            "conditions" => "conditions".to_owned(),
            "slots" => "spell slots".to_owned(),
            "pact" => "pact slots".to_owned(),
            "inspiration" => "Heroic Inspiration".to_owned(),
            "hp_bonus" => "the maximum HP modifier".to_owned(),
            "currency" => "currency".to_owned(),
            key if key.starts_with("item/") => "an item".to_owned(),
            key if key.starts_with("creature/") => "companion HP".to_owned(),
            _ => "a limited-use resource".to_owned(),
        };
        add(words);
    }
    let tail = "D&D Beyond yet. Splat tries again by itself.";
    match changed.as_slice() {
        [] => PENDING_WORDS.to_owned(),
        [one] => format!("Its change to {one} has not reached {tail}"),
        [first @ .., last] => format!(
            "Its changes to {} and {last} have not reached {tail}",
            first.join(", ")
        ),
    }
}

pub fn sync_status(state: SyncState) -> Status<'static> {
    let s = colors().sync;
    match state {
        SyncState::Off => Status {
            icon: icons::SYNC_OFF,
            word: "Sync off",
            color: s.off,
        },
        SyncState::FirstRead => Status {
            icon: icons::BUSY,
            word: "Reading sheet…",
            color: s.first_read,
        },
        SyncState::Matching => Status {
            icon: icons::OK,
            word: "Synced",
            color: s.matching,
        },
        SyncState::Pending => Status {
            icon: icons::REFRESH,
            word: "Sync pending",
            color: s.pending,
        },
        SyncState::Conflict => Status {
            icon: icons::WARN,
            word: "Conflict",
            color: s.conflict,
        },
        SyncState::ReadOnly => Status {
            icon: icons::READ_ONLY,
            word: "Sync read-only",
            color: s.read_only,
        },
        SyncState::NotSent => Status {
            icon: icons::WARN,
            word: "Not sent",
            color: colors().result.warning,
        },
        SyncState::Stopped => Status {
            icon: icons::SYNC_OFF,
            word: "Sync stopped",
            color: colors().sync.pending,
        },
        SyncState::Error => Status {
            icon: icons::ERROR,
            word: "Sync error",
            color: s.error,
        },
    }
}

/// The run's keyboard commands.
pub const KEYS: [(&str, &str); 13] = [
    ("N", "Next turn"),
    ("Enter", "Damage the selected rows by the amount"),
    ("Shift+Enter", "Heal the selected rows by the amount"),
    ("Ctrl+click", "Add a row to the targets of one amount"),
    ("Escape", "Select the current turn again"),
    ("U or Ctrl+Z", "Undo"),
    ("Up, Down", "Select another row"),
    ("A", "Focus the amount"),
    ("C", "Conditions"),
    (
        "Ctrl+1 to 9",
        "Arm an action; click a row to target it; Escape stops",
    ),
    ("Ctrl+G", "Add to fight"),
    ("Ctrl+K", "Open or close Lookup. Escape also closes it"),
    ("Ctrl+J", "Open or close Notes. Escape also closes them"),
];

impl Run {
    /// The egui Run page is gone (the GPUI page draws the run). Kept, drawing nothing, only so
    /// the Encounters page's egui `show_run` builds until that area's strip deletes it.
    pub fn show(&mut self, _ui: &mut egui::Ui) -> bool {
        false
    }

    pub fn run_command(&mut self, command: Command) {
        let follow = matches!(command, Command::Next | Command::Begin | Command::Continue);
        // A field that reports each key keeps the keyboard, or the next digit lands in the
        // note. The roll goes on to the first empty field.
        let keeps_keyboard = matches!(
            command,
            Command::Initiative(..)
                | Command::Health(..)
                | Command::Bonus(..)
                | Command::RollNpcs(_)
        );
        match self.apply(command) {
            Err(error) => self.error = Some(error),
            Ok(()) => {
                self.note.clear();
                // Beside a panel, every step gives the keyboard back, or words typed next
                // would go nowhere.
                if follow || (self.lookup_open && !keeps_keyboard) {
                    self.follow_with_keyboard();
                }
                if follow {
                    self.scroll_to_selected = true;
                    self.amount.clear();
                    self.clear_targets();
                    if self.panel == Panel::Conditions {
                        self.panel = Panel::Reference;
                    }
                }
            }
        }
    }

    pub fn clear_targets(&mut self) {
        self.also.clear();
        self.halved.clear();
    }

    /// Add or take back one row, for one amount on several targets.
    pub fn toggle_target(&mut self, index: usize) {
        if index == self.selected {
            if let Some(next) = self.also.pop_first() {
                self.selected = next;
            }
            self.halved.remove(&index);
        } else if !self.also.remove(&index) {
            self.also.insert(index);
        } else {
            self.halved.remove(&index);
        }
        // One target has no half to choose, and a chip that is gone must not still count.
        if self.also.is_empty() {
            self.halved.clear();
        }
        self.amount.clear();
        self.focus_amount = true;
    }

    /// The selected row and every other target, in turn order, as the log reads.
    pub fn targets(&self) -> Vec<usize> {
        let mut targets: Vec<usize> = self
            .also
            .iter()
            .copied()
            .filter(|index| *index != self.selected)
            .collect();
        targets.push(self.selected);
        targets.sort_by_key(|index| self.state.order.iter().position(|at| at == index));
        targets
    }

    /// The same step on the selected row and on every other target, as one step of undo. Half
    /// is for damage, and it rounds down.
    pub fn amount_command(&self, amount: i64, make: fn(usize, i64) -> Command) -> Command {
        if self.also.is_empty() {
            return make(self.selected, amount);
        }
        let halves = matches!(make(0, 0), Command::Damage(..));
        Command::Several(
            self.targets()
                .into_iter()
                .map(|index| {
                    let half = halves && self.halved.contains(&index);
                    make(index, if half { amount / 2 } else { amount })
                })
                .collect(),
        )
    }

    pub fn select(&mut self, index: usize) {
        self.selection_left_save();
        self.clear_targets();
        if self.selected != index {
            self.selected = index;
            self.note.clear();
            self.amount.clear();
            self.scroll_to_selected = true;
            if self.panel == Panel::Conditions {
                self.panel = Panel::Reference;
            }
        }
        // Beside a panel, the digits of an earlier number must not join this one.
        if self.lookup_open {
            self.amount.clear();
        }
        self.focus_amount = true;
    }

    /// After a step, the keyboard goes to Amount, for the next number. Beside a panel, the
    /// DM may be typing there, and a digit and Enter must never become damage. Only a click
    /// on a row or on Amount itself takes the keyboard from a panel, for one number.
    pub fn follow_with_keyboard(&mut self) {
        if self.lookup_open {
            self.amount.clear();
            self.return_focus = true;
        } else {
            self.focus_amount = true;
        }
    }

    /// A footer's second step waits for an answer, and Escape belongs to it.
    pub fn footer_step_open(&self) -> bool {
        self.closing.is_open() || self.ending
    }

    pub fn step_selection(&mut self, delta: isize) {
        let order = &self.state.order;
        let Some(position) = order.iter().position(|i| *i == self.selected) else {
            return;
        };
        let next = position
            .saturating_add_signed(delta)
            .min(order.len().saturating_sub(1));
        self.select(order[next]);
    }

    pub fn empty_undo_reason(&self) -> &str {
        if self.undo_stopped {
            self.undo_boundary
                .as_deref()
                .unwrap_or("Nothing to undo since the sheets changed")
        } else if let Some(reason) = self.no_saved_undo {
            reason
        } else {
            "Nothing to undo"
        }
    }

    /// The words beside an empty Undo, before any fitting. While the unread-steps warning
    /// shows, it says the cause, so the header says only `Nothing to undo` (AI3).
    pub fn empty_undo_words(&self) -> &str {
        if self.undo_unread {
            "Nothing to undo"
        } else {
            self.empty_undo_reason()
        }
    }

    /// FI2: the run's one message footer, typed (round 28, package M2). The notices that come by
    /// themselves (a changed file, an error, Undo steps that could not be read) and, in setup,
    /// the rest and carry-over offers and their result. The pane (`Messages::pane`) places it
    /// under the body, so a row the DM aims at never moves when one comes or goes. A stat-block
    /// error is one of them: its whole fault wraps there, with its repair place as a choice.
    /// Owners: the step's result is its step's (`step_results`); the reasons and notices are
    /// worked out again each frame from the run's own state, which owns them.
    pub fn run_messages<'f>(
        &self,
        frame: &'f design::outcome::PaneFrame,
        setup: bool,
        width: f32,
    ) -> design::outcome::Messages<'f, RunPress> {
        use design::outcome::{
            Choice, FaultMessage, Lost, MessageChoice, Messages, Notice, Said, UndoStep,
        };
        let mut messages = Messages::new(frame, ("run_messages", self.id), width);
        let disk = self.has_disk_conflict();
        let rest = setup && self.rest_filled_saved_hp();
        let used = setup && !rest && self.saved_hp_used();
        // The result of the DM's own press, with its Undo: the newest step's kept result.
        if used && let Some(shown) = self.newest_step_result() {
            messages.kept(shown.outcome(frame, |_| {
                Said::new(crate::sentence!("Party HP set to the saved values."))
                    .undo(UndoStep::act(super::SAVED_HP_STEP).pressed_as(RunPress::UndoStep))
                    .into()
            }));
        }
        // X4: a recharge roll's result, until the next act. Its Undo is the header's.
        if !setup
            && let Some(recharged) = self.recharge_result()
            && let Some(words) = outside_words(recharged.words.clone())
            && let Some(shown) = self.newest_step_result()
        {
            messages.kept(shown.outcome(frame, |_| Said::new(words).into()));
        }
        if disk {
            // The run cannot save until the DM chooses, so this cannot be dismissed. The safe
            // choice first; the one that loses the file's changes after it.
            messages.outcome(
                FaultMessage::new(&RunFault::DiskConflict)
                    .choice(Choice::new(
                        MessageChoice::Choice("Save as a new run"),
                        RunPress::SaveAsNew,
                    ))
                    .choice(Choice::new(
                        MessageChoice::Lose("Replace the file with this run"),
                        RunPress::Replace,
                    )),
            );
        }
        if let Some(error) = self.error.as_deref().filter(|_| !disk) {
            // Astra 3 (FI11, AI1): a stat-block error says its whole fault, wrapped, with its
            // repair place as a choice under its words. Nothing is cut.
            let repair = error.contains(STAT_BLOCK_UNREAD);
            let words = if repair {
                let first = error
                    .split_once(" Its row")
                    .map_or(error, |(first, _)| first);
                format!("{first} Its row still works.")
            } else {
                error.to_owned()
            };
            if let Some(words) = outside_words(words) {
                let mut message = FaultMessage::new(&RunFault::Error(words));
                if repair {
                    message = message.choice(Choice::new(
                        MessageChoice::Choice("Campaign → Packages"),
                        RunPress::ToPackages,
                    ));
                }
                messages.outcome(message.choice(Choice::new(
                    MessageChoice::Dismiss(Lost::Nothing),
                    RunPress::DismissError,
                )));
            }
        }
        // Issue #507: a refused Continue fight or Undo of the ending says what did not happen,
        // why, in its diagnosis's tone (R41), and the next step, until its attempt ends.
        if let Some((_, _, refusal)) = self.unend_refused.as_ref().filter(|_| !disk) {
            messages.outcome(
                FaultMessage::new(&RunFault::Refused(refusal.clone())).choice(Choice::new(
                    MessageChoice::Dismiss(Lost::Nothing),
                    RunPress::DismissRefusal,
                )),
            );
        }
        // Said once: the Log keeps the line, and the next save leaves the steps out.
        if !disk && self.undo_unread {
            messages.outcome(FaultMessage::new(&RunFault::UndoUnread).choice(Choice::new(
                MessageChoice::Dismiss(Lost::Nothing),
                RunPress::DismissUnread,
            )));
        }
        if setup
            && let Some(words) = self.carry_missing()
            && let Some(words) = outside_words(words)
        {
            messages.outcome(FaultMessage::new(&RunFault::Carry(words)));
        }
        if rest {
            messages.outcome(
                Notice::new(crate::sentence!("Party HP is full after the long rest.")).choice(
                    Choice::new(
                        MessageChoice::Choice("Use the saved HP"),
                        RunPress::UseSaved,
                    ),
                ),
            );
        }
        if setup
            && !rest
            && !used
            && let Some((from, _)) = self.back_to_sheets()
        {
            messages.outcome(
                Notice::new(crate::sentence!("Party HP carries over from {}.", from)).choice(
                    Choice::new(
                        MessageChoice::Choice("Use the sheets’ HP"),
                        RunPress::UseSheets,
                    ),
                ),
            );
        }
        messages
    }

    /// What a press in the run's footer does, in its frame. The step's Undo goes by its ticket:
    /// a step that is gone (undone, or no longer the newest) does nothing.
    pub fn run_press(
        &mut self,
        press: Option<design::outcome::Press<RunPress>>,
        command: &mut Option<Command>,
    ) {
        let Some(press) = press else {
            return;
        };
        match press.id {
            RunPress::UndoStep => {
                let Some(ticket) = press.ticket else {
                    return;
                };
                // Review of PR 338: the stack's Undo takes back the newest step only, so the
                // press of an older step's line (the body added a step in the same frame) is
                // refused, and its line stays. Only the newest step's line is undone.
                let newest = self.history_steps.last().copied();
                if self
                    .step_results
                    .undo_by(ticket, |result| {
                        if Some(result.owner.step) == newest {
                            Ok(())
                        } else {
                            Err(())
                        }
                    })
                    .is_ok()
                {
                    self.undo();
                }
            }
            other => {
                if let Some(back) = self.press_message(other) {
                    *command = Some(back);
                }
            }
        }
    }

    // ---- Live -------------------------------------------------------------------

    /// The sync state a row shows, if this participant has one.
    pub fn sync_word(&self, index: usize) -> Option<SyncState> {
        self.syncs
            .get(&index)
            .map(crate::combat_sync::CombatSync::state)
    }

    pub fn party_sync_off(&self) -> bool {
        let mut linked = self
            .state
            .participants
            .iter()
            .enumerate()
            .filter(|(_, p)| super::remote_identity(p).is_some())
            .peekable();
        linked.peek().is_some()
            && linked.all(|(index, participant)| {
                self.sync_word(index) == Some(SyncState::Off)
                    || (!self.syncs.contains_key(&index)
                        && !self.keep_party_synced
                        && !self.sync_to_restore.contains(&participant.key))
            })
    }

    pub fn row_sync_word(&self, index: usize) -> Option<SyncState> {
        // A lost connection stops sync, and it starts again by itself: not the DM's Sync off.
        self.sync_word(index).filter(|state| {
            *state != SyncState::Stopped && !(*state == SyncState::Off && self.party_sync_off())
        })
    }

    /// A row marked Conc that the newest step damaged: the DC of its save, 10 or half the
    /// damage dealt if that is more, and at most 30. The step keeps its own damage in memory,
    /// so the DC lasts until the next step; the run file gains no field, and after a restart
    /// the row has no DC. Splat does not roll it.
    pub fn conc_save_dc(&self, index: usize) -> Option<u32> {
        let p = self.state.participants.get(index)?;
        // The newest step's own damage to this row, as dealt: nothing is read from words.
        let damage = *self.state.damage.get(&index)?;
        (p.concentration && damage > 0 && self.history_from_sheet.last() != Some(&true))
            .then(|| (damage / 2).clamp(10, 30) as u32)
    }

    /// What a long roster's line under it says (both views): how many rows are out of view
    /// above and below, and what needs attention there. None when every row is in view.
    pub fn overflow_words(&self, above: &[usize], below: &[usize]) -> Option<String> {
        if above.is_empty() && below.is_empty() {
            return None;
        }
        let side = |word: &str, rows: &[usize]| -> Option<String> {
            if rows.is_empty() {
                return None;
            }
            let count = |test: &dyn Fn(usize) -> bool| rows.iter().filter(|i| test(**i)).count();
            let state = |i: usize| {
                design::hp_state(
                    self.state.participants[i].hp,
                    self.state.participants[i].max_hp,
                )
            };
            let syncs_in = |wanted: SyncState| {
                count(&|i| {
                    self.syncs
                        .get(&i)
                        .is_some_and(|sync| sync.state() == wanted)
                })
            };
            let mut text = format!("{} {word}", rows.len());
            for (n, label) in [
                (syncs_in(SyncState::Conflict), "Conflict"),
                (syncs_in(SyncState::NotSent), "Not sent"),
                (syncs_in(SyncState::Error), "Sync error"),
                (count(&|i| state(i) == design::HpState::Zero), "0 HP"),
                (
                    count(&|i| state(i) == design::HpState::Bloodied),
                    "Bloodied",
                ),
            ] {
                if n > 0 {
                    text.push_str(&format!(", {n} {label}"));
                }
            }
            if self.state.phase == Phase::Setup {
                text.push_str(&self.setup_hidden_attention(rows));
            }
            Some(text)
        };
        Some(
            [side("above", above), side("below", below)]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>()
                .join(" · "),
        )
    }

    pub fn open_conditions_tab(&mut self) {
        self.panel = Panel::Conditions;
        self.close_lookup = true;
    }

    /// `E`: the dock's effect field for the selected row; when `Effect` has no room there, or
    /// several rows are selected, the Conditions tab with focus in its `Other effect…` field.
    pub fn open_effect_field(&mut self) {
        if self.state.phase != Phase::Running {
            return;
        }
        if self.effect_in_dock && self.also.is_empty() {
            let key = self.state.participants[self.selected].key.clone();
            self.effect_fields.entry(key.clone()).or_default().focus = true;
            self.effect_open = Some(key);
        } else {
            // The Conditions tab's field shows the same draft, the participant's.
            self.open_conditions_tab();
            self.focus_other_effect = true;
        }
    }

    pub fn sync_needs_sign_in(&self) -> bool {
        !self.connected && !self.simulate_connection && !self.linked_participants().is_empty()
    }

    pub fn save_status_value(&self) -> Status<'static> {
        if self.strip().not_saved {
            Status {
                icon: icons::ERROR,
                word: "Not saved",
                color: colors().document.invalid,
            }
        } else if !self.dirty {
            Status {
                icon: icons::OK,
                word: "Saved",
                color: colors().document.saved,
            }
        } else {
            Status {
                icon: icons::DRAFT,
                word: "Not saved",
                color: colors().document.unsaved,
            }
        }
    }

    /// The second step of closing. Returns true when the run has closed. The words follow
    /// the save state: a run that cannot save must not say that it stays saved.
    /// Why a close waits, as one exact sentence: `Settle 2 conflicts and 1 pending change
    /// first.` The ended view and the close dialog both say it, so one block has one form.
    pub fn close_reason(&self) -> Option<String> {
        self.close_block().map(|general| {
            let conflicts = self.conflict_count();
            let pending = self
                .syncs
                .values()
                .filter(|sync| {
                    sync.unconfirmed() && sync.state() != SyncState::Conflict && !sync.stranded()
                })
                .count();
            let mut parts = Vec::new();
            if conflicts > 0 {
                parts.push(design::count(conflicts, "conflict", "conflicts"));
            }
            if pending > 0 {
                parts.push(design::count(pending, "pending change", "pending changes"));
            }
            if parts.is_empty() {
                general.to_owned()
            } else {
                format!("Settle {} first.", parts.join(" and "))
            }
        })
    }

    pub fn unsaved_summary(&self) -> Vec<String> {
        let saved_lines = self
            .storage
            .as_ref()
            .and_then(|s| s.baseline.as_ref())
            .and_then(|bytes| serde_json::from_slice::<super::SavedRun>(bytes).ok())
            .map_or(0, |saved| saved.state.log.len());
        let changes: Vec<_> = self
            .history
            .iter()
            .zip(&self.history_labels)
            .enumerate()
            .filter(|(_, (state, _))| state.log.len() >= saved_lines)
            .map(|(at, (_, label))| {
                let target = self.history_targets.get(at).and_then(Option::as_deref);
                Self::change_with_result(&self.state, label, target)
            })
            .collect();
        if changes.is_empty() {
            vec!["Everything since the last save.".to_owned()]
        } else {
            changes
        }
    }

    pub fn request_close(&mut self) -> bool {
        if self.strip().not_saved
            || !self.stranded_names().is_empty()
            || !self.unsent_only_here_names().is_empty()
        {
            self.closing.open();
            false
        } else {
            self.close_run()
        }
    }

    pub fn close_run(&mut self) -> bool {
        if let Some(reason) = self.close_block() {
            self.error = Some(reason.to_owned());
            return false;
        }
        self.mark_closed();
        match self.save() {
            Ok(()) => {
                self.closing.shut();
                self.confirm_close = true;
                true
            }
            Err(error) => {
                self.error = Some(error);
                false
            }
        }
    }

    /// One action for the whole party (T3.3). The run works fully without it.
    pub fn party_sync_action(&self) -> Option<Action<'static>> {
        if self.linked_participants().is_empty() {
            return None;
        }
        let stopping = matches!(self.party_sync_plan(), PartySync::Stop(_));
        Some(Action::quiet(if stopping { "Stop sync" } else { "Sync party" })
            .icon(icons::SYNC_ON)
            .density(Density::Read)
            .name(if stopping {
                "Stop party sync"
            } else {
                "Sync party"
            })
            .hint("Read every linked character’s sheet. A value you changed stays, and you choose.")
            .blocked_if(
                !self.connected && !self.simulate_connection,
                "Connect to D&D Beyond in Settings to sync.",
            ))
    }

    pub fn toggle_party_sync(&mut self) {
        let plan = self.party_sync_plan();
        self.keep_party_synced = matches!(plan, PartySync::Start(_));
        self.dirty = true;
        match plan {
            PartySync::Stop(indexes) => {
                for index in indexes {
                    self.syncs.entry(index).or_default().stop();
                }
            }
            PartySync::Start(indexes) => {
                let simulated = self.simulate_connection;
                for index in indexes {
                    let sync = self.syncs.entry(index).or_default();
                    sync.start();
                    if simulated {
                        sync.keep_without_connection();
                    }
                }
            }
        }
    }

    // ---- Side panel -------------------------------------------------------------

    /// Add to fight opens Arrivals. It is beside the fight's own controls, not among the
    /// panel's tabs, so that the DM finds it when something arrives. It takes the height of
    /// the controls in its row. Narrow layouts keep the icon and the word Add.
    pub fn add_to_fight(&self, words: bool, density: Density) -> Action<'static> {
        design::turn_heading::add_to_fight(
            words,
            density,
            self.panel == Panel::Groups && !self.hide_panel,
        )
    }

    /// The Arrivals footer's lines (both views): the newest arrival's result with a tie's Move,
    /// then each group's reason its Add waits for.
    pub fn arrival_lines<'f>(
        &mut self,
        messages: &mut design::outcome::Messages<'f, ArrivalPress>,
        frame: &'f design::outcome::PaneFrame,
    ) {
        let said = self
            .arrival_result_shown()
            .then(|| self.arrival_said())
            .flatten();
        if let Some(said) = said
            && let Some(shown) = self.newest_step_result()
        {
            messages.kept(shown.outcome(frame, |_| said.into()));
        }
        for reason in self.arrival_reasons().iter() {
            // The group faults' words, named by their group, in the warning tone.
            if let Some(words) = outside_words(reason.clone()) {
                messages.outcome(design::outcome::FaultMessage::new(&RunFault::Total(words)));
            }
        }
    }

    /// A press in the Arrivals footer. The tie's Move acts only while its line is the newest
    /// step's result: a step made in the same frame (the body added another arrival) ends it,
    /// and the press does nothing.
    pub fn arrival_press(&mut self, press: design::outcome::Press<ArrivalPress>) {
        let newest = self.history_steps.last().copied();
        let live = press.ticket.as_ref().is_some_and(|ticket| {
            self.step_results
                .get_by(ticket)
                .is_some_and(|shown| Some(shown.value().owner.step) == newest)
        });
        match press.id {
            ArrivalPress::Move(first, before) if live => {
                self.run_command(Command::MoveArrivalBefore(first, before));
            }
            ArrivalPress::Move(..) => {}
        }
    }

    /// The revision of the groups' read (`set_reads`): a new one with each read, whatever page
    /// was open when it came.
    pub fn groups_revision(&self) -> design::memo::Revision {
        self.groups_read.revision()
    }

    /// The revision of the scale's read, as `groups_revision` is for the groups.
    pub fn scale_revision(&self) -> design::memo::Revision {
        self.scale_read.revision()
    }

    /// Each offered group whose Add waits for the DM: `<group>: <reason>`, in list order, from
    /// the value that locks that Add (`add_waits`; R35, M1). Worked out again only after an edit
    /// of the typed totals, a new read of the groups, another encounter, a step or its Undo
    /// (which can give a side its initiative) or another grouping rule (`totals_revision`,
    /// `groups_revision`, `source`, `revision`, `initiative.grouping`).
    pub fn arrival_reasons(&mut self) -> Arc<[String]> {
        let groups_revision = self.groups_revision();
        let Some(groups) = self.groups.as_ref() else {
            return Arc::from([]);
        };
        let key = (
            self.totals_revision,
            groups_revision,
            self.source,
            *self.revision,
            self.initiative.grouping,
        );
        if let Some((_, reasons)) = self.arrival_reasons.as_ref().filter(|(at, _)| *at == key) {
            return reasons.clone();
        }
        #[cfg(any(test, feature = "test-support"))]
        {
            let (a, b) = PREPARED.with(std::cell::Cell::get);
            PREPARED.with(|count| count.set((a, b + 1)));
        }
        let offered = super::OfferedGroups::of(groups, self.source);
        let reasons: Arc<[String]> = offered
            .own
            .iter()
            .chain(&offered.any)
            .chain(&offered.other)
            .filter_map(|group| self.add_waits(group)?.footer(&group.name))
            .collect();
        self.arrival_reasons = Some((key, reasons.clone()));
        reasons
    }

    /// The words of `add_waits`, for a view in any toolkit.
    pub fn add_waits_words(&self, group: &super::RunGroup) -> Option<String> {
        self.add_waits(group).map(|waits| waits.words().to_owned())
    }

    /// Why this group's Add waits now, or `None` when it runs: its faults, then its typed total,
    /// then a roll with no bonus. The ONE value for the button, its drop, its Enter and the
    /// painted words (`AddWaits`).
    fn add_waits<'g>(&self, group: &'g super::RunGroup) -> Option<AddWaits<'g>> {
        if let Some(faults) = group.problem.as_deref() {
            return Some(AddWaits::Faults(faults));
        }
        let typed = self.group_totals.get(&group.id).map_or("", String::as_str);
        match arrival_total(typed) {
            Err(reason) => return Some(AddWaits::Total(reason)),
            Ok(Some(_)) => return None,
            Ok(None) => {}
        }
        // Blank: the group rolls, unless its side already has an initiative to join.
        let leader = group.leader()?;
        (!self.joins_side(&leader.side) && leader.initiative_bonus.is_none())
            .then_some(AddWaits::NoBonus)
    }

    /// What each group would make the fight when it joins, for the groups whose XP is known. The
    /// party is the characters who are present; the fight is every present creature against them.
    /// Worked out again only after a step, a new read of the groups or of the scale.
    pub fn arrival_words(&mut self) -> Option<&super::ArrivalWords> {
        let key = (
            *self.revision,
            self.groups_revision(),
            self.scale_revision(),
        );
        let (groups, scale) = (self.groups.as_ref()?, self.scale.as_ref()?);
        #[cfg(any(test, feature = "test-support"))]
        {
            self.arrival_counts.0 += 1;
        }
        if self.arrival_words.as_ref().map(|(at, _)| *at) != Some(key) {
            #[cfg(any(test, feature = "test-support"))]
            {
                self.arrival_counts.1 += 1;
            }
            // Each input with the name that a missing one is called by (`Difficulty::named`).
            let (mut party, mut foes) = (Vec::new(), Vec::new());
            let mut characters = false;
            for p in &self.state.participants {
                if p.side.trim() == "party" {
                    let level = scale.level(&p.definition);
                    characters |= level.is_some();
                    if p.present {
                        party.extend(level.map(|level| (level, p.name.clone())));
                    }
                } else if p.present {
                    foes.push((
                        crate::difficulty::creature_xp(&p.definition),
                        p.name.clone(),
                    ));
                }
            }
            // The campaign's party stands in only for a fight with no character at all. When
            // every character is absent, there is no party to weigh against.
            let nobody = characters && party.is_empty();
            let words = groups
                .iter()
                .filter_map(|group| {
                    if nobody {
                        return None;
                    }
                    // Each member joins on its own side; one on the party's side is an ally,
                    // and an ally that is a character brings its level (R1, FJ3: a group of
                    // allies says the fight after it joins too).
                    let (mut allies, mut added) = (Vec::new(), Vec::new());
                    for member in &group.members {
                        let side = member.overrides.side.as_deref().unwrap_or(&group.side);
                        let base = splat::content::overrides::Base::of(&member.entry, side);
                        let name = member
                            .overrides
                            .resolve(&base)
                            .map_or(base.name, |resolved| resolved.name);
                        let count = member.count as usize;
                        if side.trim() == "party" {
                            if let Some(level) = scale.level(&member.entry) {
                                allies.extend(std::iter::repeat_n((level, name), count));
                            }
                        } else {
                            let xp = crate::difficulty::creature_xp(&member.entry);
                            added.extend(std::iter::repeat_n((xp, name), count));
                        }
                    }
                    let adds = added.iter().map(|(xp, _)| xp.unwrap_or(0)).sum();
                    let party: Vec<_> = party.iter().chain(&allies).collect();
                    let foes: Vec<_> = foes.iter().chain(&added).collect();
                    // R1 (FK3, SK3): a fight with no word still has its line: `XP not known`
                    // in the word's place, with whose XP or level it is.
                    let levels: Vec<_> = party.iter().map(|(level, _)| *level).collect();
                    let fight =
                        scale
                            .weigh(&levels, foes.iter().map(|(xp, _)| *xp))
                            .named(|what, at| match what {
                                crate::difficulty::Unknown::Xp => {
                                    foes.get(at).map(|(_, n)| n.clone())
                                }
                                crate::difficulty::Unknown::Level => {
                                    party.get(at).map(|(_, n)| n.clone())
                                }
                            });
                    Some((group.id, (fight, adds)))
                })
                .collect();
            self.arrival_words = Some((key, words));
        }
        self.arrival_words.as_ref().map(|(_, words)| words)
    }

    /// Whether `arrival_result` has a result to show: the newest step added an arrival, or it
    /// settled a tie.
    pub fn arrival_result_shown(&self) -> bool {
        let last = self.state.log.last();
        (!self.history.is_empty()
            && last.is_some_and(|line| {
                line.starts_with(super::GROUP_ADDED) || line.starts_with(super::ONE_ADDED)
            }))
            || last.is_some_and(|line| line.contains(" moved before "))
    }

    /// The newest arrival's result, as data: who joined and at what total, with
    /// `Move … before …` when it tied, or the move that settled a tie. Every result is a success
    /// message; the Move choice shows that a choice is open, and gives up the whole parenthesis
    /// before anything else when its line is short (`move_choice`, FA6).
    /// `arrival_result_shown` says when it shows.
    fn arrival_said(&mut self) -> Option<design::outcome::Said<ArrivalPress>> {
        use design::outcome::{Choice, Said};
        if let Some(previous) = self.history.last().filter(|_| {
            self.state.log.last().is_some_and(|line| {
                line.starts_with(super::GROUP_ADDED) || line.starts_with(super::ONE_ADDED)
            })
        }) {
            let start = previous.participants.len();
            let tied = self
                .state
                .participants
                .iter()
                .enumerate()
                .skip(start)
                .find_map(|(first, arrival)| {
                    let total = arrival.initiative?;
                    self.state
                        .order
                        .iter()
                        .copied()
                        .rev()
                        .find(|i| {
                            *i < start
                                && self.state.participants[*i].present
                                && self.state.participants[*i].initiative == Some(total)
                        })
                        .map(|before| (first, Self::roll_block_first(&self.state, before)))
                });
            let group_line = self.group_arrival_notice(start);
            let line = group_line
                .as_deref()
                .unwrap_or_else(|| self.state.log.last().map_or("", String::as_str));
            let Some((first, before)) = tied else {
                return outside_words(line.to_owned()).map(Said::new);
            };
            let name = Self::roll_block_name(&self.state, &self.state.participants[before]);
            let waits = self.state.phase == Phase::Running
                && self
                    .state
                    .order
                    .iter()
                    .position(|i| *i == before)
                    .is_some_and(|at| at <= self.state.cursor);
            let total = self.state.participants[first].initiative?;
            let tie = if group_line.is_some() {
                format!(" Tied with {name} at\u{a0}{total}, placed after.")
            } else {
                String::new()
            };
            let line = format!(
                "{line}{tie}{}",
                if waits {
                    " ·\u{a0}acts next round if moved before"
                } else {
                    ""
                }
            );
            // The name the `joined at` message uses, so one arrival has one name.
            let parts = Self::arrival_parts(&self.state, first, &name);
            let arrival = parts.whole();
            // Its forms are made once for these two names (review of PR 490): an unchanged
            // frame clones the kept words (their forms are shared) and builds none.
            let words = match &self.tie_choice {
                Some((kept_arrival, kept_name, words))
                    if *kept_arrival == arrival && *kept_name == name =>
                {
                    words.clone()
                }
                _ => {
                    let words = move_choice(&parts, &name);
                    self.tie_choice = Some((arrival, name, words.clone()));
                    words
                }
            };
            // A finished change, tie or not: the Move choice shows the open choice.
            return outside_words(line).map(|line| {
                Said::new(line).choice(Choice::named(words, ArrivalPress::Move(first, before)))
            });
        }
        self.state
            .log
            .last()
            .filter(|line| line.contains(" moved before "))
            .and_then(|line| outside_words(line.clone()))
            .map(Said::new)
    }

    /// The roster shows members; this notice keeps just the group and its totals.
    /// Roll details and member names remain in the unmodified Log.
    pub fn group_arrival_notice(&self, start: usize) -> Option<String> {
        let added = self.state.participants.get(start..)?;
        let origin = added.first()?.added_from.as_ref()?;
        // A creature added by hand has no group: the notice names the rows it added, in the
        // group's words. `entered by the DM` stays in the Log, which is not spoken to the DM.
        let who = if origin.group.is_empty() {
            let names: Vec<_> = added.iter().map(|p| p.name.as_str()).collect();
            match names.as_slice() {
                [] => return None,
                [one] => (*one).to_owned(),
                [first @ .., last] => format!("{} and {last}", first.join(", ")),
            }
        } else {
            origin.group.clone()
        };
        let totals: std::collections::BTreeSet<_> =
            added.iter().filter_map(|p| p.initiative).collect();
        let totals = totals
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        Some(format!("{who} joined at\u{a0}{totals}."))
    }

    /// Point 1 of the fourth review: a group that is no longer offered here
    /// (`OfferedGroups::of`, as this list shows them) drops its kept Arrivals line. It runs where
    /// the list is built, also when the list is empty, once for each read of the groups.
    pub fn keep_offered_lines(&mut self) {
        let revision = self.groups_revision();
        if self.arrival_lines_seen == Some(revision) {
            return;
        }
        self.arrival_lines_seen = Some(revision);
        let offered: std::collections::BTreeSet<Uuid> = self
            .groups
            .as_ref()
            .map(|groups| {
                let shown = super::OfferedGroups::of(groups, self.source);
                shown
                    .own
                    .iter()
                    .chain(&shown.any)
                    .chain(&shown.other)
                    .map(|group| group.id)
                    .collect()
            })
            .unwrap_or_default();
        self.arrival_lines.retain(|id| offered.contains(id));
    }

    /// By side, a creature joins its side's initiative with no roll, when the side has one.
    pub fn joins_side(&self, side: &str) -> bool {
        self.initiative.grouping == Grouping::BySide
            && self
                .state
                .participants
                .iter()
                .any(|p| p.npc && p.side == side && p.initiative.is_some())
    }

    /// The newest round this group joined in (0 for setup), from one scan of the participants
    /// and no list. Its row's status and its layout both take it from here.
    pub fn group_joined(&self, group: &super::RunGroup) -> Option<u64> {
        #[cfg(any(test, feature = "test-support"))]
        GROUP_SCANS.with(|count| count.set(count.get() + 1));
        self.state
            .participants
            .iter()
            .filter_map(|p| p.added_from.as_ref())
            .filter(|from| from.group_id == Some(group.id))
            .map(|from| from.round)
            .max()
    }

    /// What a group's row says about it before its members: when it joined (`joined`, from
    /// `group_joined`), or whose encounter it is for. `Changed` follows the members
    /// (`changed_mark`).
    pub fn group_status(&self, group: &super::RunGroup, joined: Option<u64>) -> String {
        if let Some(round) = joined {
            if round == 0 {
                // Short enough for the member slot at 960, in the run's own phase word, so a
                // first join in setup needs no line of its own (review 2).
                "Added in setup".to_owned()
            } else {
                format!("Added in round {round}")
            }
        } else if let Some(name) = group
            .encounter_name
            .as_ref()
            .filter(|_| group.encounter != self.source)
        {
            format!("For {name}")
        } else if group.encounter.is_some() && group.encounter != self.source {
            // FB10: the Groups list's words, short enough for the member slot.
            crate::encounters::ENCOUNTER_DELETED.to_owned()
        } else {
            String::new()
        }
    }

    /// An Arrivals heading for a view that measures its own words (`group_row`'s rule): how the
    /// title and the `Fight after: …` line share `block` (`heading_split`), the line's words at
    /// its room (`Line::fit_or_wrap`; `None` with no form, when the group's totals show
    /// instead), the line's description and the heading's hover. `small` and `strong` measure
    /// words in those types. Reads the kept `arrival_words`; call `arrival_words` first.
    pub fn arrival_heading(
        &self,
        group: &super::RunGroup,
        block: f32,
        small: impl Fn(&str) -> f32,
        strong: impl Fn(&str) -> f32,
    ) -> ArrivalHeading {
        let name = &group.name;
        let rating = self
            .arrival_words
            .as_ref()
            .and_then(|(_, words)| words.get(&group.id))
            .cloned();
        let (rating, adds) = rating.map_or((None, 0), |(rating, adds)| (Some(rating), adds));
        let no_form = crate::difficulty::Difficulty::default();
        let (side, members, worth) = (
            style::human_label(&group.side),
            group.count_label(),
            group.xp_label(),
        );
        let line = crate::difficulty::Line::fight_after(
            rating.as_ref().unwrap_or(&no_form),
            crate::difficulty::FightAfter {
                side: &side,
                members: &members,
                worth: &worth,
            },
        );
        let heading_width = strong(&format!("{} {name}", icons::DRAG));
        let first_word = name.split_whitespace().next().unwrap_or(name);
        let floor = heading_width.min(strong(&format!("{} {first_word}…", icons::DRAG)));
        let shortest = line
            .shortest()
            .map_or(0.0, |shortest| small(&shortest.text()));
        let widest = WIDEST_ANSWERS
            .iter()
            .map(|answer| small(answer))
            .fold(shortest, f32::max);
        let split = heading_split(block, heading_width, floor, widest);
        let fitted = line
            .fit_or_wrap(|text| small(text) <= split.room)
            .map(|fitted| ArrivalLine {
                lead: fitted.lead,
                form: fitted.form,
                wrap: fitted.wrap,
            });
        let totals_text = group.facts(true);
        let shown = rating.as_ref().and_then(|fight| fight.rating.as_ref());
        let described = match (shown, rating.as_ref().and_then(|f| f.missing.as_ref())) {
            (Some(rating), _) => Some(format!(
                "{totals_text}. Adds {} XP. {FIGHT_AFTER}{}.",
                crate::encounter_run::grouped_number(adds),
                rating.form(true)
            )),
            (None, Some(missing)) => Some(format!(
                "{totals_text}. {FIGHT_AFTER}{}.",
                missing
                    .words()
                    .unwrap_or_else(|| missing.short().to_owned())
            )),
            (None, None) => None,
        };
        let changed = changed_mark(group).is_some();
        let context = group
            .encounter_name
            .as_ref()
            .filter(|_| group.encounter != self.source)
            .map_or_else(String::new, |name| format!("For {name}. "));
        let hover = format!(
            "{name}. {totals_text}.{} {context}{} {}{}",
            if changed { " Changed." } else { "" },
            group.notes.trim(),
            self.initiative.grouping.arrival_rule(),
            shown.map_or_else(String::new, |rating| format!("\n{}", rating.budget_line()))
        );
        ArrivalHeading {
            split,
            line: fitted,
            totals: totals_text,
            described,
            hover,
        }
    }

    /// A group's `Cannot join` faults in Arrivals, each with its repair, as plain lines for a
    /// view that is not egui (`group_row`'s `member_fault_row` rows): the members' faults, then
    /// the group's own reasons. When every fault points to packages, the last also offers the
    /// group editor.
    pub fn arrival_faults(
        &self,
        group: &super::RunGroup,
    ) -> Vec<design::outcome::plain::PlainLine<GroupRepair>> {
        use design::outcome::{Choice, FaultMessage, MessageChoice};
        let own: Vec<super::MemberFault> = group
            .own_problems()
            .into_iter()
            .map(|line| super::MemberFault::new(super::FaultKind::CannotJoin, line.into()))
            .collect();
        let faults: Vec<&super::MemberFault> = group.warnings.iter().chain(&own).collect();
        let packages_only = faults
            .iter()
            .all(|fault| fault.kind == super::FaultKind::NotInPackages);
        faults
            .iter()
            .enumerate()
            .map(|(at, fault)| {
                let (label, repair) = if fault.kind == super::FaultKind::NotInPackages {
                    (crate::encounters::PACKAGES_LINK, GroupRepair::Packages)
                } else {
                    (crate::encounters::EDIT_GROUP, GroupRepair::EditGroup)
                };
                let mut message = FaultMessage::new(*fault)
                    .choice(Choice::new(MessageChoice::Choice(label), repair));
                if packages_only && at + 1 == faults.len() && repair == GroupRepair::Packages {
                    message = message.choice(Choice::new(
                        MessageChoice::Choice(crate::encounters::EDIT_GROUP),
                        GroupRepair::EditGroup,
                    ));
                }
                message.into_plain()
            })
            .collect()
    }

    /// A repair pressed under a group's fault in Arrivals.
    pub fn group_repair(&mut self, group: uuid::Uuid, repair: GroupRepair) {
        match repair {
            GroupRepair::Packages => self.wants_packages = true,
            GroupRepair::EditGroup => {
                if let Some(path) = self
                    .groups
                    .iter()
                    .flat_map(|groups| groups.iter())
                    .find(|found| found.id == group)
                    .map(|found| found.path.clone())
                {
                    self.wants_group_editor = Some(path);
                }
            }
        }
    }

    /// The same step for the button and for a drop: the total typed beside it, or a roll.
    pub fn add_group_from_panel(&mut self, id: Uuid) {
        let Some(group) = self
            .groups
            .iter()
            .flat_map(|groups| groups.iter())
            .find(|group| group.id == id)
            .cloned()
        else {
            return;
        };
        let total = match arrival_total(self.group_totals.get(&id).map_or("", String::as_str)) {
            Ok(total) => total,
            Err(reason) => {
                self.error = Some(reason.into());
                return;
            }
        };
        self.run_command(Command::AddGroup(group, total));
        if self.error.is_none() {
            self.group_totals.remove(&id);
            self.totals_revision += 1;
            self.return_focus = self.lookup_open;
        }
    }

    /// The stat-block pane's footer (both views): a snapshot whose file changed, went or cannot
    /// be read now, with `Read the new version` while the campaign has the entry. Also whether it
    /// has that notice (a sheet's values line says so).
    pub fn reference_messages<'f>(
        &self,
        frame: &'f design::outcome::PaneFrame,
        index: usize,
        width: f32,
    ) -> (design::outcome::Messages<'f, ReferencePress>, bool) {
        let p = &self.state.participants[index];
        let definition = &p.definition;
        let mut notice = None;
        if let Some(content) = &self.content {
            let at = content.position_of(&Reference::Id(definition.id));
            let current = at.map(|at| &content.items[at].entry);
            let unreadable = content
                .errors
                .iter()
                .any(|error| error.contains(&definition.path));
            notice = snapshot_notice(definition, current, unreadable).map(|words| {
                // FD3: a file that cannot be read now is an error, as in every other view; a
                // changed file is one to look at (warning). The kind decides the tone.
                (
                    SnapshotFault {
                        words,
                        severity: if unreadable {
                            design::outcome::Severity::CannotRead
                        } else {
                            design::outcome::Severity::NeedsFixing
                        },
                    },
                    at,
                )
            });
        }
        let mut messages = design::outcome::Messages::<ReferencePress>::new(
            frame,
            ("run_reference_footer", self.id, &p.key),
            width,
        );
        let shown = notice.is_some();
        if let Some((fault, at)) = &notice {
            let message = design::outcome::FaultMessage::new(fault);
            messages.outcome(match at {
                Some(at) => message.choice(design::outcome::Choice::new(
                    design::outcome::MessageChoice::Choice("Read the new version"),
                    ReferencePress::Read(*at),
                )),
                None => message,
            });
        }
        (messages, shown)
    }

    /// A press in the stat-block pane's footer.
    pub fn reference_press(&mut self, press: design::outcome::Press<ReferencePress>) {
        match press.id {
            ReferencePress::Read(at) => self.lookup_request = Some(at),
        }
    }

    // ---- Setup ------------------------------------------------------------------

    /// How many the DM must order, when Notes hide the panel that says so.
    pub fn tied_count(&self) -> usize {
        (0..self.state.participants.len())
            .filter(|&index| self.setup_tied(index))
            .count()
    }

    pub fn setup_tied(&self, index: usize) -> bool {
        let p = &self.state.participants[index];
        !self.initiative.roster_ties
            && p.initiative.is_some()
            && self
                .state
                .participants
                .iter()
                .enumerate()
                .any(|(other, q)| {
                    index != other
                        && p.initiative == q.initiative
                        && !self.shared_initiative(index, other)
                })
    }

    pub fn setup_hidden_attention(&self, rows: &[usize]) -> String {
        let missing = rows
            .iter()
            .filter(|&&i| {
                self.state.participants[i].present
                    && self.state.participants[i].initiative.is_none()
            })
            .count();
        let tied = rows.iter().filter(|&&i| self.setup_tied(i)).count();
        let mut words = String::new();
        if missing > 0 {
            words.push_str(&format!(
                ", {missing} {} initiative",
                if missing == 1 { "needs" } else { "need" }
            ));
        }
        if tied > 0 {
            words.push_str(&format!(", {}", design::count(tied, "tied", "tied")));
        }
        words
    }

    pub fn shared_initiative(&self, a: usize, b: usize) -> bool {
        let p = &self.state.participants[a];
        let q = &self.state.participants[b];
        p.shared_roll.is_some() && p.shared_roll == q.shared_roll
    }

    pub fn setup_tie_directions(&self, index: usize) -> Option<(usize, bool, bool)> {
        let position = self.state.order.iter().position(|i| *i == index)?;
        let p = &self.state.participants[index];
        let tied = |other: Option<&usize>| {
            !self.initiative.roster_ties
                && p.initiative.is_some()
                && other.is_some_and(|i| self.state.participants[*i].initiative == p.initiative)
        };
        let up = position > 0 && tied(self.state.order.get(position - 1));
        let down = tied(self.state.order.get(position + 1));
        (up || down).then_some((position, up, down))
    }

    // ---- Ended ------------------------------------------------------------------

    /// The shorter forms of Undo's words, longest first, for a view that measures them itself:
    /// the one rule's target forms (`undo_forms`), then its forms without the target. After
    /// them a view shows `Undo` alone.
    pub fn undo_word_forms(&self, full: &str) -> Vec<String> {
        let forms = undo_forms(
            full,
            self.undo_target(),
            self.undo_from_sheet(),
            &self.state.participants,
        );
        forms.target.into_iter().chain(forms.bare).collect()
    }
}

/// The level in `Exhaustion (2)`. Plain `Exhaustion` is the first.
pub fn exhaustion_level(condition: &str) -> u8 {
    condition
        .trim()
        .strip_prefix("Exhaustion (")
        .and_then(|rest| rest.strip_suffix(')'))
        .and_then(|level| level.trim().parse().ok())
        .unwrap_or(1)
}

/// The highest exhaustion level. The run's stepper stops here and says why.
pub const EXHAUSTION_MAX: i64 = 6;

/// One step of undo: the old level goes, and the new one comes.
/// The one Exhaustion entry at another level. Its time stays.
pub fn set_exhaustion(index: usize, level: u8) -> Command {
    Command::Condition(index, format!("Exhaustion ({level})"))
}

/// The line under a character's card. The notice above it, when it shows, already says the
/// card is the copy from the start of the fight, so the line does not say it again.
pub fn snapshot_values_line(notice_shown: bool) -> &'static str {
    if notice_shown {
        "The roster has the current values."
    } else {
        "Saved when this run began. The roster has the current values."
    }
}

/// Words from outside the app's own: a write's or the system's error, a stat block's file
/// name, the DM's names in a recharge roll or a carry-over. `Sentence::outside` gives them a
/// capital and an end mark; blank words give `None`.
#[allow(
    clippy::disallowed_methods,
    reason = "the run’s errors carry the system’s words, and its results the DM’s names"
)]
pub(crate) fn outside_words(words: String) -> Option<design::outcome::Sentence> {
    design::outcome::Sentence::outside(words)
}

/// What the Arrivals footer can press: the tie's Move, by the arrival's first row and the row it
/// moves before.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArrivalPress {
    Move(usize, usize),
}

/// What the run's message footer can press.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunPress {
    /// The Undo of the newest step's result (the saved HP's use).
    UndoStep,
    SaveAsNew,
    Replace,
    DismissError,
    /// Dismiss of a refused Continue fight or Undo of the ending.
    DismissRefusal,
    ToPackages,
    DismissUnread,
    UseSaved,
    UseSheets,
}

/// The run's faults in its footer. The kind decides the tone.
enum RunFault {
    /// The run's file changed on disk: saving has stopped until the DM chooses.
    DiskConflict,
    /// The run's own error (a write, a stat block that cannot be read), in its own words.
    Error(design::outcome::Sentence),
    /// The saved Undo steps could not be read.
    UndoUnread,
    /// Party HP that a carry-over could not bring.
    Carry(design::outcome::Sentence),
    /// FI5: a typed initiative total that cannot be read, named by its group.
    Total(design::outcome::Sentence),
    /// A refused Continue fight or Undo of the ending (#507): its tone is its diagnosis's.
    Refused(design::results::Refusal),
}

impl design::outcome::Fault for RunFault {
    fn severity(&self) -> design::outcome::Severity {
        use design::outcome::Severity;
        match self {
            Self::DiskConflict => Severity::NotSaved,
            Self::Error(_) => Severity::CannotRead,
            Self::UndoUnread | Self::Carry(_) | Self::Total(_) => Severity::NeedsFixing,
            Self::Refused(refusal) => match refusal.tone() {
                design::Tone::Error => Severity::CannotRead,
                _ => Severity::NeedsFixing,
            },
        }
    }
    fn cause(&self) -> design::outcome::Sentence {
        match self {
            Self::DiskConflict => crate::sentence!(
                "This run changed on disk, perhaps from another copy of Splat or a sync tool. Your run is safe until you choose. Saving has stopped."
            ),
            Self::Error(words) | Self::Carry(words) | Self::Total(words) => words.clone(),
            Self::Refused(refusal) => refusal.words(),
            Self::UndoUnread => crate::sentence!(
                "Splat could not read this run’s saved Undo steps, so Undo starts again from here. No turn, HP or condition was lost."
            ),
        }
    }
}

/// What the stat-block pane's footer can press.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReferencePress {
    /// `Read the new version`: the entry at this place in the campaign's content.
    Read(usize),
}

/// A row's stat block whose file is no longer the run's copy: changed, gone, or unreadable.
/// Its kind decides its tone (FD3).
struct SnapshotFault {
    words: design::outcome::Sentence,
    severity: design::outcome::Severity,
}

impl design::outcome::Fault for SnapshotFault {
    fn severity(&self) -> design::outcome::Severity {
        self.severity
    }
    fn cause(&self) -> design::outcome::Sentence {
        self.words.clone()
    }
}

pub fn snapshot_notice(
    snapshot: &Entry,
    current: Option<&Entry>,
    unreadable: bool,
) -> Option<design::outcome::Sentence> {
    let name = std::path::Path::new(&snapshot.path)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy();
    // The card says first what it shows: the copy the run took at its start.
    let kind = if snapshot.fields.contains_key("splat_external") {
        "sheet"
    } else {
        "stat block"
    };
    let Some(current) = current else {
        if !unreadable
            && let Some(path) = snapshot
                .fields
                .get("splat_run_source_path")
                .and_then(|value| value.as_str())
            && std::path::Path::new(path).is_file()
        {
            let owner = snapshot
                .fields
                .get("splat_run_source_owner")
                .and_then(|value| value.as_str())
                .unwrap_or("This package");
            return Some(crate::sentence!(
                "This {} uses the copy from the start of the fight. {} is not used in this campaign now.",
                kind,
                owner
            ));
        }
        return Some(crate::sentence!(
            "This {} uses the copy from the start of the fight. {} {}.",
            kind,
            name,
            if unreadable {
                "cannot be read now"
            } else if snapshot.fields.contains_key("splat_run_source_path") {
                "was deleted"
            } else {
                "is not available in this campaign now"
            }
        ));
    };
    let fields = |entry: &Entry| {
        entry
            .fields
            .iter()
            .filter(|(key, _)| {
                !key.as_str()
                    .is_some_and(|key| key == "splat_schema" || key.starts_with("splat_run_"))
            })
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect::<serde_yaml_ng::Mapping>()
    };
    // Normalization ignores schema/run metadata. With the same model and kind,
    // identical source bodies therefore have identical normalized bodies too.
    (fields(snapshot) != fields(current)
        || ((snapshot.body != current.body || snapshot.kind != current.kind)
            && splat::content::stat_blocks::comparison_body(snapshot)
                != splat::content::stat_blocks::comparison_body(current)))
    .then(|| {
        crate::sentence!(
            "This {} uses the copy from the start of the fight. {} has changed.",
            kind,
            name
        )
    })
}

pub fn change_origin(p: &Participant) -> Option<String> {
    let mut parts = Vec::new();
    if let Some(added) = &p.added_from
        && (!p.stat_changes() || added.group.is_empty())
    {
        parts.push(if added.group.is_empty() {
            "Added by hand".into()
        } else {
            format!("Arrived with {}", added.group)
        });
    }
    let has_name_row = p.changes.as_ref().is_some_and(|changes| {
        changes
            .overrides
            .rows(&changes.base)
            .iter()
            .any(|row| row.label == "Name")
    });
    if p.named_stat_block() && (p.npc_identity() || !has_name_row) {
        parts.push(format!("as {}", p.definition.name));
    }
    (!parts.is_empty()).then(|| parts.join(" · "))
}

/// What `changes_block` shows, as data for either view: what to add by hand (above a stat
/// block), the origin line, the `Changed` status, the change rows and the by-hand note.
pub struct ChangeLines {
    /// `Add by hand: …`, in the changed colour, first (above a stat block only).
    pub add_by_hand: Option<String>,
    /// `Arrived with …`, `as …`, muted.
    pub origin: Option<String>,
    /// `Changed`, or `Changed · arrived with <group>`.
    pub status: Option<String>,
    /// The change rows to list (`change_rows`).
    pub rows: Vec<Row>,
    /// The note under the rows when some are done by hand.
    pub note: Option<&'static str>,
    /// The block ends with `S` (a creature the encounter or group changed).
    pub gap: bool,
}

/// What the encounter or group changed on this creature, for `changes_block` and the GPUI
/// panel. `above_stat_block` leaves out the vitals the stat block shows itself.
pub fn change_lines(p: &Participant, above_stat_block: bool) -> ChangeLines {
    let origin = change_origin(p);
    let status = p.stat_changes().then(|| {
        p.added_from
            .as_ref()
            .filter(|added| !added.group.is_empty())
            .map_or_else(
                || "Changed".to_owned(),
                |added| format!("Changed · arrived with {}", added.group),
            )
    });
    let Some(changes) = p.changes.as_ref().filter(|_| p.stat_changes()) else {
        return ChangeLines {
            add_by_hand: None,
            origin,
            status,
            rows: Vec::new(),
            note: None,
            gap: false,
        };
    };
    let rows: Vec<_> = changes
        .overrides
        .rows(&changes.base)
        .into_iter()
        .filter(|row| {
            !above_stat_block || !matches!(row.label, "Maximum HP" | "AC" | "Initiative bonus")
        })
        .filter(|row| !p.npc_identity() || row.label != "Name")
        .collect();
    let note = (!above_stat_block && rows.iter().any(|row| row.by_hand)).then_some(BY_HAND);
    if above_stat_block {
        let by_hand = rows
            .iter()
            .filter(|row| row.by_hand)
            .map(|row| {
                let value = row.changed.split(" on each").next().unwrap_or(&row.changed);
                let value = value.split(" (").next().unwrap_or(value);
                format!(
                    "{} {value}",
                    row.label.trim_end_matches(" bonus").to_lowercase()
                )
            })
            .collect::<Vec<_>>();
        ChangeLines {
            add_by_hand: (!by_hand.is_empty())
                .then(|| format!("Add by hand: {}", by_hand.join(", "))),
            origin,
            status,
            rows: rows.into_iter().filter(|row| !row.by_hand).collect(),
            note,
            gap: true,
        }
    } else {
        ChangeLines {
            add_by_hand: None,
            origin,
            status,
            rows,
            note,
            gap: true,
        }
    }
}

pub use splat_content_ui::roster::change_preview;

pub fn ordinal(place: usize) -> String {
    let suffix = if (11..=13).contains(&(place % 100)) {
        "th"
    } else {
        match place % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        }
    };
    format!("{place}{suffix}")
}

/// The words of an error for a stat block that cannot be read, whose repair is in Campaign →
/// Packages.
pub(crate) const STAT_BLOCK_UNREAD: &str = "stat block could not be read";

/// The close dialog's words for a sync conflict. They offer a close only when a close can happen.
/// Round 27 FN4: a blocked close has no words here. The title names the conflict, and the reason
/// beside the disabled `Close now` says what to do, once.
pub fn conflict_close_words(blocked: bool) -> Option<&'static str> {
    (!blocked).then_some("Choose which HP to keep, or close and choose next time.")
}

/// What a close keeps. A close keeps the run's Undo steps too (they are saved with the run), so
/// nothing here may say that closing ends Undo.
pub fn close_saved_words(sync: bool) -> String {
    if sync {
        "The run stays saved. Sync stops."
    } else {
        "The run stays saved. Resume it from Paused run on the left, or from Encounters."
    }
    .to_owned()
}

/// The tie choice's words, `Move <arrival> before <name>`. FA6: the choice sits under the
/// message that names its arrival whole, and it only ever moves that arrival, so when the whole
/// words do not fit, an arrival named by its first member, `Shrine Reserve (Goblin Warrior 3)`,
/// gives up the parenthesis, never just the member's name: `Move Shrine Reserve before Brannoch
/// Vale`. A bare `(… 3)` tells the DM nothing. Then (at once, for an arrival with no
/// parenthesis) the two names give up their last words in turn (`shortened_names`): the label
/// shortens by whole words only (R3f X43, X44). The typed choice shows the first form that fits
/// (`ChoiceWords::or_shorter`); hover and the accessible name keep the whole words.
fn move_choice(arrival: &super::ArrivalName, before: &str) -> design::outcome::ChoiceWords {
    let whole = crate::choice!("Move {} before {}", arrival.whole(), before);
    // The member goes by data (#512): a member's own parentheses are never read as the end of
    // the group's name.
    let group = arrival.group.as_str();
    // With no member, the first pair is the whole words again.
    let forms = shortened_names(group, before)
        .skip(usize::from(arrival.member.is_none()))
        .inspect(|_| {
            #[cfg(any(test, feature = "test-support"))]
            MOVE_FORMS.with(|forms| forms.set(forms.get() + 1));
        });
    whole
        .or_shorter(forms.map(|(group, before)| crate::choice!("Move {} before {}", group, before)))
}

// Move forms built on this thread, for a test that an unchanged frame builds none.
#[cfg(any(test, feature = "test-support"))]
thread_local! {
    static MOVE_FORMS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// Two names with their last words given up in turn, the other name's last: each pair from
/// both whole to both at their first word and `…`.
fn shortened_names<'a>(group: &'a str, before: &'a str) -> impl Iterator<Item = (String, String)> {
    let groups = name_words(group);
    name_words(before).into_iter().flat_map(move |before| {
        groups
            .clone()
            .into_iter()
            .map(move |group| (group, before.clone()))
    })
}

/// The forms of one Undo label, longest first: see `undo_forms`.
pub struct UndoForms {
    /// Forms that name the act's target, from the whole label to its shortest telling form.
    pub(super) target: Vec<String>,
    /// Forms without the target, for an Undo too narrow for any of `target`:
    /// `Undo: 8 damage × 3`. After them, the caller shows `Undo` alone.
    pub(super) bare: Vec<String>,
}

/// The one rule of the run's Undo labels. A label names its act and its target, which
/// `describe_named` gives as `target`: never read back out of the label's words. A long
/// effect keeps four of its words, then two (`Undo: add effect Wis save… on Cinder Wolf 2`). Then
/// the target shortens by whole words and keeps the part that tells it from another row: its
/// copy number or letter (`Cinder… 2`), else its last word. A sheet's step gives up its source
/// before its target's short forms. When no telling form fits, the target goes (`bare`). No
/// form cuts a name to only `…` and its number. Hover and the accessible name keep the whole
/// label.
pub(crate) fn undo_forms(
    label: &str,
    target: Option<&str>,
    from_sheet: bool,
    participants: &[Participant],
) -> UndoForms {
    let (prefix, action) = label
        .strip_prefix("Undo: ")
        .map_or(("", label), |action| ("Undo: ", action));
    let mut actions = vec![action];
    if from_sheet && let Some(rest) = action.strip_prefix(SHEET_SOURCE) {
        actions.push(rest);
    }
    let mut seen = std::collections::HashSet::new();
    // SP3 (`docs/design-system.md:1538`): a sheet's step gives up `D&D Beyond` before any
    // short form of its target. With the source, only the whole label; then every form without.
    let sourced = actions.len() > 1;
    let forms = actions
        .iter()
        .enumerate()
        .flat_map(|(at, action)| {
            let forms = action_forms(action, target, participants);
            let keep = if sourced && at == 0 { 1 } else { forms.len() };
            forms.into_iter().take(keep)
        })
        .filter(|words| keeps_names(words) && seen.insert(words.clone()))
        .map(|words| format!("{prefix}{words}"))
        .collect();
    let bare = if let Some((verb, ..)) = move_parts(action) {
        vec![verb.to_owned()]
    } else if let Some((before, after)) = target.and_then(|target| split_target(action, target)) {
        // SP2 (`undo-action-words-fit`): a step whose words go on after its target (`mark
        // Brannoch Vale present`) keeps its verb and its state without the name: `mark
        // present`, as a short footer Undo keeps its verb (`UndoStep::short`). `U` and the
        // name give way before these words; `Undo` alone comes only after them.
        let mut words = act_words(before);
        let rest = after.trim();
        // A name-first step (`Brannoch Vale absent`) has no verb to keep: none.
        if words.is_empty() && !before.trim().is_empty() && !rest.is_empty() {
            words.push(format!("{} {rest}", before.trim()).trim().to_owned());
        }
        words
    } else {
        undo_compact(label).into_iter().collect()
    };
    UndoForms {
        target: forms,
        bare: bare
            .into_iter()
            .map(|words| format!("{prefix}{}", words.trim_start_matches(prefix)))
            .collect(),
    }
}

/// An action's words before and after its target, which `describe_named` puts at the label's
/// end, or at its start (`Brannoch Vale absent`).
fn split_target<'a>(action: &'a str, target: &str) -> Option<(&'a str, &'a str)> {
    if target.is_empty() {
        return None;
    }
    action
        .strip_suffix(target)
        .map(|before| (before, ""))
        .or_else(|| action.strip_prefix(target).map(|after| ("", after)))
        // A name between the act's words, as a whole word: `mark ` · name · ` absent`.
        .or_else(|| {
            let at = action.find(&format!(" {target} "))? + 1;
            Some((&action[..at], &action[at + target.len()..]))
        })
}

/// The act's own words without the target and its joining word: an effect's four, then two
/// (`add effect Wis save…`), or `7 damage`, `initiative`, `2 loot rolls`. None for a name-first
/// step.
fn act_words(before: &str) -> Vec<String> {
    if let Some((words, _)) = effect_words(before) {
        return words;
    }
    [" on ", " off ", " for ", " to ", " from "]
        .into_iter()
        .find_map(|joining| before.strip_suffix(joining))
        .map(str::trim)
        .filter(|words| !words.is_empty())
        .map(str::to_owned)
        .into_iter()
        .collect()
}

/// One action's forms with its target, longest first.
fn action_forms(action: &str, target: Option<&str>, participants: &[Participant]) -> Vec<String> {
    if let Some(forms) = move_forms(action) {
        return forms;
    }
    let Some((name, (before, after))) =
        target.and_then(|name| Some((name, split_target(action, name)?)))
    else {
        return vec![action.to_owned()];
    };
    let words: Vec<String> = match effect_words(before).filter(|_| after.is_empty()) {
        Some((words, joining)) => words
            .into_iter()
            .map(|words| format!("{words}{joining}"))
            .collect(),
        None => vec![before.to_owned()],
    };
    let shortest = words.last().expect("one form of the words");
    let mut forms: Vec<String> = words
        .iter()
        .map(|words| format!("{words}{name}{after}"))
        .collect();
    forms.extend(
        target_forms(name, participants)
            .into_iter()
            .skip(1)
            .map(|name| format!("{shortest}{name}{after}")),
    );
    forms
}

/// The words of an effect's step before its target (`add effect Wis save DC 15 (Hold Person)
/// on `) when they are more than four: four of them, then two, each with `…`, and the joining
/// word. The verb (`add effect`, `add`, `remove`) stays and is not counted. None for any other step's words,
/// which stay whole.
fn effect_words(before: &str) -> Option<(Vec<String>, &'static str)> {
    let (words, joining) = [" on ", " off ", " from "]
        .into_iter()
        .find_map(|joining| Some((before.strip_suffix(joining)?, joining)))?;
    // FK4 (iter-27-R): every effect step starts with its verb, `add effect`, `add` or `remove`.
    let (verb, words) = ["add effect ", "add ", "effect ", "remove "]
        .into_iter()
        .find_map(|verb| Some((verb.to_owned(), words.strip_prefix(verb)?)))
        .unwrap_or((String::new(), words));
    let words: Vec<&str> = words.split_whitespace().collect();
    (words.len() > 4).then(|| {
        let forms = [4, 2]
            .into_iter()
            .map(|n| format!("{verb}{}…", words[..n].join(" ")))
            .collect();
        (forms, joining)
    })
}

/// A row's name by whole words, longest first, each form telling it from every other row:
/// `Cinder Wolf 2` then `Cinder… 2`; `Brannoch Vale` then `Brannoch…`. A form that another
/// row's name also gives (`Rival…` beside `Rival Captain 2`) keeps the last word instead, or is
/// left out.
fn target_forms(name: &str, participants: &[Participant]) -> Vec<String> {
    let words: Vec<&str> = name.split_whitespace().collect();
    let mut forms = vec![name.to_owned()];
    let Some(last) = words.last().copied() else {
        return forms;
    };
    let copy =
        words.len() > 1 && (last.chars().all(|c| c.is_ascii_digit()) || last.chars().count() == 1);
    let tells = |kept: &str, end: Option<&str>| {
        !participants.iter().any(|p| {
            p.name != name
                && p.name.starts_with(&format!("{kept} "))
                && end.is_none_or(|end| p.name.ends_with(&format!(" {end}")))
        })
    };
    let longest = words.len().saturating_sub(if copy { 2 } else { 1 });
    for count in (1..=longest).rev() {
        let kept = words[..count].join(" ");
        if !copy && tells(&kept, None) {
            forms.push(format!("{kept}…"));
        } else if count + 1 < words.len() && tells(&kept, Some(last)) {
            forms.push(format!("{kept}… {last}"));
        }
    }
    forms
}

/// A move's verb, its arrival and the name it moved before: `move <arrival> before <name>`.
fn move_parts(action: &str) -> Option<(&'static str, &str, &str)> {
    let (verb, words) = action
        .strip_prefix("move ")
        .map(|words| ("move", words))
        .or_else(|| action.strip_prefix("Move ").map(|words| ("Move", words)))?;
    let (moved, before) = words.split_once(" before ")?;
    Some((verb, moved, before))
}

/// A move's forms with its target, longest first. SJ1: Undo is not beside the message that
/// names the arrival, so an arrival with a copy number keeps it. The moved arrival is what the
/// DM must know (iter-27-R FI8), so the place goes first, `move Rival Team (Rival Captain 2)`,
/// then the group, `move Rival Captain 2`; last the member's name gives way to `…`,
/// `move Rival Team (… 2)`, then `move Rival… (… 2)`.
/// An arrival with no copy number shortens with the other name by whole words. None for any
/// other step.
fn move_forms(action: &str) -> Option<Vec<String>> {
    let (verb, moved, before) = move_parts(action)?;
    let parts = moved
        .strip_suffix(')')
        .and_then(|rest| rest.rsplit_once(" ("));
    let copy = parts.and_then(|(group, member)| {
        let (_, last) = member.rsplit_once(' ')?;
        (!last.is_empty() && last.chars().all(|c| c.is_ascii_digit()))
            .then_some((group, member, last))
    });
    let mut forms = vec![action.to_owned()];
    match copy {
        Some((group, member, copy)) => {
            forms.push(format!("{verb} {moved}"));
            forms.push(format!("{verb} {member}"));
            forms.extend(
                name_words(group)
                    .into_iter()
                    .map(|group| format!("{verb} {group} (…\u{a0}{copy})")),
            );
        }
        None => {
            let moved = parts.map_or(moved, |(group, _)| group);
            forms.extend(
                shortened_names(moved, before)
                    .map(|(moved, before)| format!("{verb} {moved} before {before}")),
            );
        }
    }
    Some(forms)
}

/// A step on several rows without them, for an Undo too narrow for its words:
/// `Undo: 8 damage × 3` (a step on one row gives up its row by `act_words`). FI4: the row count follows `×`, never `to`, so it cannot be read as a copy number
/// (`to 2` beside `Cinder Wolf 2`). `None` when the words name no row to leave out.
pub(crate) fn undo_compact(label: &str) -> Option<String> {
    let action = label.strip_prefix("Undo: ")?;
    let several = action.split(super::HALF_FOR).next().unwrap_or(action);
    let counted = several.strip_suffix(super::SEVERAL_TARGETS)?;
    let (words, count) = counted.rsplit_once(" to ")?;
    Some(format!("Undo: {words} × {count}"))
}

/// False when shortening left a name as only `…` and its copy number (`move … 2 before`,
/// `effect Wis… on … 2`): such a form tells the DM nothing, so `undo_forms` never gives it.
pub(crate) fn keeps_names(words: &str) -> bool {
    !words.split(' ').any(|word| word == "…")
}

/// A name with its last words given up in turn, by whole words: from the whole name to its
/// first word and `…`.
fn name_words(name: &str) -> Vec<String> {
    let all: Vec<&str> = name.split_whitespace().collect();
    (1..=all.len())
        .rev()
        .map(|n| {
            let kept = all[..n].join(" ");
            if n == all.len() {
                kept
            } else {
                format!("{kept}…")
            }
        })
        .collect()
}

pub fn presence_action(present: bool) -> (&'static str, &'static str) {
    if present {
        (
            "Absent",
            "Skip this participant’s turns until marked present",
        )
    } else {
        ("Absent", "Give this participant turns again")
    }
}

/// The death-save counts in words: `1 success · 0 failures`.
pub fn death_save_words(successes: u8, failures: u8) -> String {
    format!(
        "{successes} {} · {failures} {}",
        if successes == 1 {
            "success"
        } else {
            "successes"
        },
        if failures == 1 { "failure" } else { "failures" }
    )
}

/// Concentration, as a run row marks it and History's Final state repeats it: its icon and
/// `Conc`, in its own colour.
pub fn concentration_mark() -> (String, egui::Color32) {
    (
        format!("{} Conc", icons::CONCENTRATING),
        colors().participant.concentrating,
    )
}

/// A run row's fixed columns (both views): from the left, the turn bar and the initiative, the
/// side's mark, the picture, the name, AC, temporary HP, HP and the state words.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RowColumns {
    /// The initiative column's right edge, from the row's left.
    pub initiative_right: f32,
    /// The gap after the initiative.
    pub gap: f32,
    /// The picture's column.
    pub picture: f32,
    /// Where the name, and the second line under it, start.
    pub name_left: f32,
    /// The name's column: near its HP in a wide window, unless a name in the list needs more.
    pub name_width: f32,
    pub ac_width: f32,
    pub temp_width: f32,
    pub hp_width: f32,
    /// The state words' column (none when `inline_state`).
    pub state_width: f32,
    /// No room for a state column: the state words open the second line.
    pub inline_state: bool,
    pub ac_right: f32,
    pub temp_right: f32,
    pub hp_right: f32,
    /// How far the HP bar starts after the temporary HP column's edge, so the second line
    /// keeps the width the turn word took.
    pub bar_inset: f32,
}

/// The columns of every row of a roster `width` wide at `scale`. `narrow` rows (a list that
/// compresses, or the compact scale) put `TURN` on the number's line and give the picture up
/// first. `longest_name` is the list's widest name at the name's size; `state_width` the widest
/// state word with its gap.
pub fn row_columns(
    scale: RunScale,
    narrow: bool,
    width: f32,
    longest_name: f32,
    state_width: f32,
) -> RowColumns {
    // The current-row bar, the gap after it, and room for the word `TURN` at this size. In
    // narrow columns the word can sit on the number's line, so that a row's height never
    // depends on whose turn it is.
    let word = scale.mark * 3.2;
    let initiative_right = 6.0 + design::XS + if narrow { word + scale.stat } else { word };
    let gap = if narrow { 6.0 } else { 12.0 };
    // Its bar starts later, still under the digits, so its second line has the width that
    // the turn word took and a condition does not wrap again.
    let bar_inset = if narrow { 8.0 } else { 0.0 };
    // Compressed rows have a roster to fit, so the picture gives up what it can.
    let picture = if narrow {
        scale.name * 1.25
    } else {
        (scale.row - 10.0).min(scale.name * 1.75)
    };
    let name_left = initiative_right + gap + scale.name * 0.7 + picture + PICTURE_GAP;
    let hp_width = scale.hp * 1.9 + scale.hp_max * 2.0 + 4.0;
    let temp_width = scale.stat * 2.6 + 12.0;
    let ac_width = scale.stat * 1.5 + 22.0;
    let numbers = name_left + ac_width + temp_width + hp_width + design::S;
    let inline_state = width - numbers - state_width < scale.name * NAME_ROOM;
    let name_room = (width - numbers - if inline_state { 0.0 } else { state_width }).max(90.0);
    // Near its HP in a wide window, unless a name in this list needs more.
    let name_width = name_room.min((scale.name * 12.5).max(longest_name));
    let ac_right = name_left + name_width + ac_width;
    let temp_right = ac_right + temp_width;
    let hp_right = temp_right + hp_width;
    RowColumns {
        initiative_right,
        gap,
        picture,
        name_left,
        name_width,
        ac_width,
        temp_width,
        hp_width,
        state_width,
        inline_state,
        ac_right,
        temp_right,
        hp_right,
        bar_inset,
    }
}

/// The height that makes these rows, none shorter than it is now, fill `room`, up to `cap`.
/// Rows taller than that keep their own height. Zero if the rows already fill `room`.
pub fn even_height(heights: &[f32], room: f32, cap: f32) -> f32 {
    let mut sorted = heights.to_vec();
    sorted.sort_by(f32::total_cmp);
    let mut rest = room;
    let mut left = sorted.len();
    for &height in sorted.iter().rev() {
        if height <= rest / left as f32 {
            break;
        }
        rest -= height;
        left -= 1;
    }
    if left == 0 || rest <= 0.0 {
        return 0.0;
    }
    (rest / left as f32).min(cap)
}

/// The fixed columns of a run row, and its second line, which grows rather than cuts.
pub const PICTURE_GAP: f32 = 6.0;

#[cfg(test)]
mod tests {

    use super::*;
    use serde_yaml_ng::Mapping;
    use splat::content::{Entry, overrides::Overrides};

    #[test]
    fn run_snapshot_notice_ignores_metadata_but_keeps_model_and_prose_changes() {
        let source =
            splat::content::definitions::source_file("definitions/creatures/wolf.md").unwrap();
        let current = Entry::parse("Wolf.md", source).unwrap();
        let mut snapshot = current.clone();
        snapshot.fields.insert("splat_schema".into(), 2.into());
        snapshot
            .fields
            .insert("splat_run_source_owner".into(), "Ashen Marches".into());
        assert!(snapshot_notice(&snapshot, Some(&current), false).is_none());

        let mut changed = current.clone();
        changed.fields["splat_stats"]["max_hp"]["value"] = 12.into();
        assert!(snapshot_notice(&snapshot, Some(&changed), false).is_some());

        let mut changed = current.clone();
        changed.body.push_str("\n\nA new attack.\n");
        assert!(snapshot_notice(&snapshot, Some(&changed), false).is_some());

        let mut whitespace = current.clone();
        whitespace.body.push_str("\n\n");
        assert!(snapshot_notice(&snapshot, Some(&whitespace), false).is_none());
    }

    #[test]
    fn run_copy_ignores_an_unchanged_source_but_reports_hp_and_source_removal() {
        let source =
            splat::content::definitions::source_file("definitions/creatures/wolf.md").unwrap();
        let mut snapshot = Entry::parse("Wolf.md", source).unwrap();
        let current = Entry::parse("Wolf.md", source).unwrap();
        assert!(snapshot_notice(&snapshot, Some(&current), false).is_none());
        let changed = Entry::parse(
            "Wolf.md",
            &source.replace("| Hit Points | 11", "| Hit Points | 12"),
        )
        .unwrap();
        assert!(
            snapshot_notice(&snapshot, Some(&changed), false)
                .unwrap()
                .as_str()
                .contains("has changed.")
        );
        let folder = tempfile::tempdir().unwrap();
        let path = folder.path().join("Wolf.md");
        std::fs::write(&path, source).unwrap();
        snapshot.fields.insert(
            "splat_run_source_path".into(),
            path.to_string_lossy().to_string().into(),
        );
        snapshot
            .fields
            .insert("splat_run_source_owner".into(), "Ashen Marches".into());
        assert!(
            snapshot_notice(&snapshot, None, false)
                .unwrap()
                .as_str()
                .contains("Ashen Marches is not used in this campaign now")
        );
        std::fs::remove_file(path).unwrap();
        assert!(
            snapshot_notice(&snapshot, None, false)
                .unwrap()
                .as_str()
                .contains("was deleted")
        );
    }

    #[test]
    fn card_origin_does_not_repeat_a_name_change_and_joins_group_and_npc() {
        let text = crate::library::note_text("Guard", "creature", None);
        let mut p = Participant::with_overrides(
            "guard".into(),
            "Enemies".into(),
            Entry::parse("Guard.md", &text).unwrap(),
            &Mapping::new(),
            &Overrides {
                name: Some("Captain".into()),
                ..Default::default()
            },
            true,
        )
        .unwrap();
        assert_eq!(change_origin(&p), None);
        p.added_from = Some(super::super::AddedFrom {
            group: "Rival Team".into(),
            instance: "arrival".into(),
            round: 1,
            group_id: None,
        });
        assert_eq!(change_origin(&p).as_deref(), None);
        let changed = p.changes.take();
        assert_eq!(
            change_origin(&p).as_deref(),
            Some("Arrived with Rival Team")
        );
        p.changes = changed;
        Arc::make_mut(&mut p.definition)
            .fields
            .insert("splat_run_npc_name".into(), "Captain".into());
        assert_eq!(
            change_origin(&p).as_deref(),
            Some("Arrived with Rival Team · as Guard")
        );
        p.added_from = None;
        assert_eq!(change_origin(&p).as_deref(), Some("as Guard"));
    }

    /// iter-20-R-FT6, ST1: the notice says first what the card shows, with the card's own
    /// word, and the line under a character's card does not say it again.
    #[test]
    fn the_snapshot_notice_names_the_copy_once() {
        let text = crate::library::note_text("Cinder Wolf", "creature", None);
        let snapshot = Entry::parse("Cinder Wolf.md", &text).unwrap();
        let mut current = snapshot.clone();
        current.body.push_str("The pack howls.");
        assert_eq!(
            snapshot_notice(&snapshot, Some(&current), false)
                .as_ref()
                .map(|words| words.as_str()),
            Some(
                "This stat block uses the copy from the start of the fight. Cinder Wolf.md has changed."
            )
        );
        assert_eq!(
            snapshot_notice(&snapshot, None, true)
                .as_ref()
                .map(|words| words.as_str()),
            Some(
                "This stat block uses the copy from the start of the fight. Cinder Wolf.md cannot be read now."
            )
        );
        let mut sheet = snapshot.clone();
        sheet
            .fields
            .insert("splat_external".into(), serde_yaml_ng::Value::Bool(true));
        assert!(
            snapshot_notice(&sheet, Some(&current), false)
                .unwrap()
                .as_str()
                .starts_with("This sheet uses the copy from the start of the fight.")
        );
        assert_eq!(
            snapshot_values_line(true),
            "The roster has the current values."
        );
        // Must stay true: with no notice above it, the line still says when it was saved.
        assert_eq!(
            snapshot_values_line(false),
            "Saved when this run began. The roster has the current values."
        );
    }

    #[test]
    fn run_snapshot_notice_distinguishes_unchanged_changed_unreadable_and_deleted() {
        let text = crate::library::note_text("Wolf", "creature", None);
        let snapshot = Entry::parse("Wolf.md", &text).unwrap();
        assert!(snapshot_notice(&snapshot, Some(&snapshot), false).is_none());
        let mut current = snapshot.clone();
        current.body.push_str("A new attack.");
        assert!(
            snapshot_notice(&snapshot, Some(&current), false)
                .unwrap()
                .as_str()
                .contains("has changed.")
        );
        assert!(
            snapshot_notice(&snapshot, None, true)
                .unwrap()
                .as_str()
                .contains("cannot be read now")
        );
        assert!(
            snapshot_notice(&snapshot, None, false)
                .unwrap()
                .as_str()
                .contains("is not available in this campaign now")
        );
    }

    #[test]
    fn a_replaced_creature_keeps_its_stat_block_name_in_run_rows() {
        let text = crate::library::note_text("Guard", "creature", None)
            .replace("max_hp: {}", "max_hp: {value: 11}");
        let entry = Entry::parse("Guard.md", &text).unwrap();
        let mut p = Participant::with_overrides(
            "wolf".into(),
            "Enemies".into(),
            entry,
            &Mapping::new(),
            &Overrides {
                name: Some("Cinder Wolf".into()),
                ..Default::default()
            },
            true,
        )
        .unwrap();
        p.name = "Cinder Wolf 1".into();
        assert!(p.named_stat_block());
        // Its row says `as Guard` (`roster::parts`).
        assert_eq!(p.definition.name, "Guard");
    }

    #[test]
    fn initiative_bonus_formats_and_parses_signed_values() {
        // The Correct tab's local copies went with M0169: the bonus is `design::signed_stepper`,
        // which shows the shared form; its reading of `−1` and `-1` is checked through the real
        // field in `iter9_b_correct_does_not_change_sync_and_explicit_stop_stays`.
        for (value, shown) in [(1, "+1"), (0, "+0"), (-1, "−1")] {
            assert_eq!(design::signed(value), shown);
        }
    }

    #[test]
    fn hidden_shared_roll_of_three_has_no_tie_warning() {
        let mut run = super::super::tests::run();
        for p in &mut run.state.participants[..3] {
            p.initiative = Some(17);
            p.shared_roll = Some("goblins".into());
        }
        assert_eq!(run.tied_count(), 0);
        assert_eq!(run.setup_hidden_attention(&[0, 1, 2]), "");
        assert!(run.setup_tie_directions(1).is_some());
    }

    #[test]
    fn hidden_independent_equal_rolls_say_two_tied() {
        let mut run = super::super::tests::run();
        run.state.participants[0].initiative = Some(17);
        run.state.participants[1].initiative = Some(17);
        run.state.participants[2].initiative = Some(12);
        assert_eq!(run.tied_count(), 2);
        assert_eq!(run.setup_hidden_attention(&[0, 1]), ", 2 tied");
        run.initiative.roster_ties = true;
        assert_eq!(run.setup_hidden_attention(&[0, 1]), "");
    }

    #[test]
    fn saving_wait_and_save_failure_have_distinct_statuses() {
        let mut run = super::super::tests::run();
        run.dirty = true;
        let status = run.save_status_value();
        assert_eq!(status.icon, icons::DRAFT);
        assert_eq!(status.color, colors().document.unsaved);
        run.disk_conflict = true;
        let status = run.save_status_value();
        assert_eq!(status.word, "Not saved");
        assert_eq!(status.icon, icons::ERROR);
        assert_eq!(status.color, colors().document.invalid);
    }

    #[test]
    fn closing_lists_changes_with_their_targets() {
        let mut run = super::super::tests::run();
        run.state.participants[0].hp = Some(11);
        run.state.participants[0].temporary_hp = 0;
        let name = run.state.participants[0].name.clone();
        run.apply(Command::Damage(0, 3)).unwrap();
        assert_eq!(
            run.unsaved_summary(),
            vec![format!("3 damage to {name} (8 HP left)")]
        );
        run.apply(Command::Condition(0, "Prone".into())).unwrap();
        assert_eq!(run.unsaved_summary().len(), 2);
        assert_eq!(run.unsaved_summary()[1], format!("add Prone on {name}"));
        run.undo();
        assert_eq!(run.unsaved_summary().len(), 1);
    }

    #[test]
    fn iter_2_b_undo_keeps_actions_directions_amounts_and_copy_numbers() {
        let entry = Entry::parse("wolf.md", &splat::content::formats::fixture("Fixture.md", &format!("splat_id: {}\nsplat_kind: creature\nsplat_name: Cinder Wolf 3\nsplat_creature: {{type: beast}}\nsplat_stats: {{max_hp: {{value: 33}}}}\n", Uuid::new_v4()), "").expect("schema-2 fixture")).unwrap();
        let p = Participant::new(
            "wolf".into(),
            "enemies".into(),
            entry,
            &Mapping::new(),
            true,
        )
        .unwrap();
        let rows = std::slice::from_ref(&p);
        for (before, after) in [
            ("7 damage to ", ""),
            ("add Frightened on ", ""),
            ("remove Frightened from ", ""),
            ("", " present"),
            ("", " absent"),
            // Review of PR 338: the name between the act's words keeps its place, cut to its
            // first word and copy number.
            ("mark ", " absent"),
            ("mark ", " present"),
            ("move ", " up"),
            ("move ", " down"),
            ("initiative for ", ""),
            ("start concentration for ", ""),
            ("stop concentration for ", ""),
            ("exhaustion 2 → 3 on ", ""),
            ("exhaustion 3 → 1 on ", ""),
        ] {
            let full = format!("Undo: {before}{}{after}", p.name);
            let forms = undo_forms(&full, Some(&p.name), false, rows).target;
            // iter10-B-FT2: the word Undo stays; the target is shortened to its first word and
            // its copy number.
            assert!(
                forms
                    .iter()
                    .all(|form| form.starts_with(&format!("Undo: {before}"))
                        && form.ends_with(after)),
                "{forms:?}"
            );
            assert!(
                forms.contains(&format!("Undo: {before}Cinder… 3{after}")),
                "{forms:?}"
            );
        }
        for direction in ["on", "off"] {
            let full = format!(
                "Undo: Wreathed in the cinders of the old shrine {direction} {}",
                p.name
            );
            // X6: one form, `Undo: <words>`, never `Undo add:` or `Undo remove:`.
            let forms = undo_forms(&full, Some(&p.name), false, rows).target;
            let expected = format!("Undo: Wreathed in… {direction} Cinder… 3");
            assert!(forms.contains(&expected), "{forms:?}");
        }
        // X6: an added effect's Undo has one form, `Undo: add effect <words>`: four words, then
        // two, then the target by whole words with its copy number, then no target. Never a
        // name cut to `…` and its number.
        let effect = format!(
            "Undo: add effect Wis save DC 15 (Hold Person) on {}",
            p.name
        );
        let forms = undo_forms(&effect, Some(&p.name), false, rows);
        assert_eq!(
            forms.target,
            [
                "Undo: add effect Wis save DC 15… on Cinder Wolf 3",
                "Undo: add effect Wis save… on Cinder Wolf 3",
                "Undo: add effect Wis save… on Cinder… 3",
            ]
        );
        assert_eq!(
            forms.bare,
            [
                "Undo: add effect Wis save DC 15…",
                "Undo: add effect Wis save…"
            ]
        );
        // A form that another row's name also gives does not tell the row apart.
        let mut plain = p.clone();
        plain.name = "Cinder Wolf".into();
        let both = [p.clone(), plain];
        assert_eq!(target_forms("Cinder Wolf", &both), ["Cinder Wolf"]);
        assert_eq!(target_forms(&p.name, &both), ["Cinder Wolf 3", "Cinder… 3"]);
        // SJ1, SH1: when no target fits, Undo keeps the step's words without the row. A step
        // that names no row in that place has no short form, so it shows `Undo` alone.
        // The row is the step's own target (`describe_named`), never found in the words.
        let compact = |label: &str| {
            undo_forms(label, Some(&p.name), false, std::slice::from_ref(&p))
                .bare
                .first()
                .cloned()
        };
        for (label, short) in [
            // FI4: the count follows `×`, so `2` is not read as Cinder Wolf 2.
            (
                "Undo: 3 damage to 2 participants, half for 1",
                Some("Undo: 3 damage × 2"),
            ),
            (
                "Undo: 8 damage to 3 participants",
                Some("Undo: 8 damage × 3"),
            ),
            (
                "Undo: a change to 3 participants",
                Some("Undo: a change × 3"),
            ),
            (
                "Undo: exhaustion 2 → 3 on Cinder Wolf 3",
                Some("Undo: exhaustion 2 → 3"),
            ),
            (
                "Undo: initiative for Cinder Wolf 3",
                Some("Undo: initiative"),
            ),
            ("Undo: 7 damage to Cinder Wolf 3", Some("Undo: 7 damage")),
            (
                "Undo: D&D Beyond HP for Cinder Wolf 3",
                Some("Undo: D&D Beyond HP"),
            ),
            // Lookalikes: the name first, a row not in the run, or no row at all.
            ("Undo: Cinder Wolf 3 absent", None),
            ("Undo: initiative for Cinder Wolf 4", None),
            ("Undo: next turn", None),
            ("Undo: add 2 × Wolf", None),
        ] {
            assert_eq!(compact(label).as_deref(), short, "{label}");
        }
        // Review round 1 of PR 286: the row is the step's own target, never found in its
        // words. A row named `2` is not the `2` of `DC 12`, so the effect keeps two words and
        // the row.
        let mut run = super::super::tests::run();
        run.state.participants[0].name = "2".into();
        let (label, target) = run.describe_named(&Command::Condition(
            0,
            "Wis save DC 12 Against the Ancient Archmage on its turn".into(),
        ));
        assert_eq!(
            (label.as_str(), target.as_deref()),
            (
                "add effect Wis save DC 12 Against the Ancient Archmage on 2",
                Some("2")
            )
        );
        let forms = undo_forms(
            &format!("Undo: {label}"),
            target.as_deref(),
            false,
            &run.state.participants,
        );
        assert!(
            forms
                .target
                .contains(&"Undo: add effect Wis save… on 2".to_owned()),
            "{:?}",
            forms.target
        );
    }

    #[test]
    fn iter4_b_party_sync_off_is_once_in_header_and_mixed_rows_keep_their_state() {
        let mut run = super::super::tests::run();
        while run.state.participants.len() < 6 {
            run.state
                .participants
                .push(run.state.participants[0].clone());
        }
        for index in 0..5 {
            super::super::tests::linked(&mut run, index, &index.to_string());
        }
        assert!(
            run.party_sync_off(),
            "An encounter with sync disabled keeps its shared header notice"
        );
        run.keep_party_synced = true;
        assert!(
            !run.party_sync_off(),
            "Waiting for the connection is not a disabled party"
        );
        run.keep_party_synced = false;
        for index in 0..5 {
            assert_eq!(
                run.sync_word(index),
                None,
                "Untouched linked rows have no sync word"
            );
        }
        // Only workers the DM stopped supply Off. When all are stopped, the header
        // carries their shared state once, as before.
        for index in 0..5 {
            run.syncs
                .insert(index, crate::combat_sync::CombatSync::default());
        }
        assert!(run.party_sync_off());
        for index in 0..5 {
            assert_eq!(run.sync_word(index), Some(SyncState::Off));
            assert_eq!(run.row_sync_word(index), None);
        }
        assert_eq!(run.row_sync_word(5), None);
        run.syncs.insert(
            0,
            crate::combat_sync::CombatSync::simulated_conflict(1, 10, 1),
        );
        assert_eq!(run.row_sync_word(0), Some(SyncState::Conflict));
        assert_eq!(run.row_sync_word(1), Some(SyncState::Off));
        let mut stopped = crate::combat_sync::CombatSync::default();
        stopped.start();
        stopped.session_ended();
        run.syncs.insert(1, stopped);
        // A lost connection stopped it; sync starts again by itself, so the row is not Sync off.
        assert_eq!(run.row_sync_word(1), None);
        // Rows that wait for the connection, or a party kept in sync, are not Sync off either.
        run.syncs.clear();
        run.sync_to_restore = vec![run.state.participants[2].key.clone()];
        assert_eq!(run.row_sync_word(2), None);
        assert_eq!(run.row_sync_word(3), None);
        run.keep_party_synced = true;
        assert_eq!(run.row_sync_word(3), None);
    }

    #[test]
    fn iter10_b_undo_falls_back_to_its_word_when_a_name_is_lost() {
        for (words, keeps) in [
            ("Undo: move Rival… 2 before Rival Team", true),
            ("Undo: D&D Beyond HP for Brannoch…", true),
            ("Undo: 7 damage to Cinder… 3", true),
            ("Undo: next turn", true),
            ("Undo: move … 2 before Rival Team", false),
            ("Undo: … 3", false),
            ("Undo add: effect Wis… on … 2", false),
            ("… 2", false),
        ] {
            assert_eq!(keeps_names(words), keeps, "iter10-B-FT2-lost-name: {words}");
        }
    }

    /// Points 2 and 3 of the round 29 reviews (R1): an Arrivals heading reserves the smallest
    /// complete `Fight after: …` before its title; the title shortens, never below its first
    /// word; where even that and the answer do not fit, the answer takes its own line under the
    /// heading at the whole width. At no width is the answer's room less than that answer.
    /// FB12 and AM8 (M-R0): the split has no status and no current answer among its inputs or
    /// its result, so no heading can give its room to a status, and the current answer (fourth
    /// review) cannot move the answer's place or the title: only the widest answer does.
    #[test]
    fn iter_29_r1_the_heading_reserves_fight_after_before_the_title() {
        let (title, floor, shortest) = (250.0, 60.0, 100.0);
        for whole in 1..=600 {
            let whole = whole as f32;
            let split = heading_split(whole, title, floor, shortest);
            assert!(
                split.room >= shortest - 0.01 || (split.below && split.room == whole),
                "{whole}: {split:?}"
            );
            if split.below {
                assert!(whole < floor + design::L + shortest, "{whole}: {split:?}");
            } else {
                assert!(split.title >= floor, "{whole}: {split:?}");
                assert!(split.title <= title);
                // The line has all the room the title leaves.
                assert!(
                    (split.title + design::L + split.room - whole).abs() < 0.01,
                    "{whole}: {split:?}"
                );
            }
        }
        // The boundary: exactly the first word, its gap and the answer.
        let edge = heading_split(floor + design::L + shortest, title, floor, shortest);
        assert_eq!(
            (edge.below, edge.title, edge.room),
            (false, floor, shortest)
        );
        let under = heading_split(floor + design::L + shortest - 1.0, title, floor, shortest);
        assert!(under.below);
    }

    /// FL4, AL2, FL2, SL3 (iter-29 G178): the pending line names the change that waits, in the
    /// HP form of the conflict's buttons; with nothing to compare, the general words.
    #[test]
    fn iter_29_r1_g178_the_pending_line_names_what_waits() {
        use crate::combat_sync::CombatSync;
        let pending = CombatSync::simulated_state(SyncState::Pending, 7, 62, 8);
        assert_eq!(
            pending_words(&pending),
            "Its change to 53/62 HP has not reached D&D Beyond yet. Splat tries again by itself."
        );
        let same = CombatSync::simulated_state(SyncState::Error, 7, 62, 8);
        assert_eq!(pending_words(&same), PENDING_WORDS);
        // Astra, PR 371: HP holds two values; only those that wait are named. Temporary HP
        // alone, then with HP.
        let changed = |removed: i64, temporary: i64| {
            let mut sync = CombatSync::simulated_state(SyncState::Pending, 7, 62, 8);
            let mut hp = sync.values()["hp"].clone();
            hp["removedHitPoints"] = removed.into();
            hp["temporaryHitPoints"] = temporary.into();
            sync.set_group("hp", hp);
            sync
        };
        assert_eq!(
            pending_words(&changed(8, 5)),
            "Its change to 5 temporary HP has not reached D&D Beyond yet. Splat tries again by itself."
        );
        assert_eq!(
            pending_words(&changed(9, 5)),
            "Its changes to 53/62 HP and 5 temporary HP have not reached D&D Beyond yet. Splat tries again by itself."
        );
    }

    #[test]
    fn iter_1_b_arrival_total_accepts_blank_and_signed_whole_numbers() {
        for text in ["", "  "] {
            assert_eq!(arrival_total(text), Ok(None));
        }
        for number in [i64::MIN, -1, 0, 13, i64::MAX] {
            assert_eq!(arrival_total(&number.to_string()), Ok(Some(number)));
        }
        // M-P0.21 (review of #709): a total typed as the app writes it, with a true minus.
        assert_eq!(arrival_total(&design::signed(-2)), Ok(Some(-2)));
        assert!(arrival_total("1.5").is_err());
        assert!(arrival_total("3\u{2212}5").is_err());
    }

    #[test]
    fn iter23_s_am1_conflict_words_offer_a_close_only_when_it_can_happen() {
        // Round 27 FN4: a blocked close has no body words; its reason sits beside Close now.
        assert_eq!(conflict_close_words(true), None);
        // Lookalike outside the rule: a conflict with nothing blocking keeps the close offer.
        assert_eq!(
            conflict_close_words(false),
            Some("Choose which HP to keep, or close and choose next time.")
        );
    }

    #[test]
    fn close_names_only_the_live_losses() {
        assert_eq!(
            close_saved_words(false),
            "The run stays saved. Resume it from Paused run on the left, or from Encounters."
        );
        assert_eq!(close_saved_words(true), "The run stays saved. Sync stops.");
        // AL1: a close keeps Undo, so no close words say that it ends.
        for sync in [false, true] {
            assert!(!close_saved_words(sync).contains("Undo"));
        }
    }

    /// iter7-B-FT1, iter-29-R1-G130: death saves (the panel's block and the compact line's
    /// `Death saves`) show for a party row at 0 HP, for any row with a save recorded, and for a
    /// linked sheet on any side; not for an enemy at 0 HP with none.
    #[test]
    fn death_saves_follow_party_and_recorded_counts() {
        for (side, linked, successes, failures, shown) in [
            ("party", false, 0, 0, true),
            ("enemies", false, 0, 0, false),
            ("enemies", false, 1, 0, true),
            ("other", false, 0, 1, true),
            // A linked player sheet on a side the DM named makes death saves too.
            ("heroes", true, 0, 0, true),
        ] {
            let mut run = super::super::tests::run();
            let p = &mut run.state.participants[0];
            if linked {
                let mut definition = (*p.definition).clone();
                definition
                    .fields
                    .insert("splat_external".into(), "dndbeyond:1".into());
                p.definition = std::sync::Arc::new(definition);
            }
            p.side = side.into();
            p.hp = Some(0);
            p.successes = successes;
            p.failures = failures;
            assert_eq!(
                p.shows_death_saves(),
                shown,
                "{side}, {successes} successes, {failures} failures"
            );
        }
    }

    #[test]
    fn change_previews_keep_units_and_ordinals_their_suffix() {
        assert_eq!(
            change_preview(&Row {
                label: "Speed",
                base: "30 ft.".into(),
                changed: "40 ft".into(),
                by_hand: true,
                in_rolls: false,
            }),
            "30 ft → 40 ft · add by hand"
        );
        assert_eq!(
            change_preview(&Row {
                label: "Attack bonus",
                base: "As printed".into(),
                changed: "+1 on each attack".into(),
                by_hand: false,
                in_rolls: true,
            }),
            "As printed → +1 on each attack · in Splat’s rolls"
        );
        assert_eq!(
            change_preview(&Row {
                label: "Maximum HP",
                base: "1000000".into(),
                changed: "-28".into(),
                by_hand: false,
                in_rolls: false,
            }),
            "1,000,000 → −28"
        );
        assert_eq!(ordinal(12), "12th");
        assert_eq!(ordinal(21), "21st");
    }
}
