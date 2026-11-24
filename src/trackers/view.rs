//! The Trackers view's words and acts: the Due rows' parts, each tracker line's words, the footer's
//! messages and the acts a press asks for. The GPUI page (`gui::pages::trackers`) lays them out
//! and draws them; every press is one act, which the shell applies.
use super::{
    Rules, Says, State, TrackerFault, Trackers, Unrecorded, ledger::Entry, ledger::NoticeRef,
};
use crate::design::outcome::PaneFrame;
use crate::design::outcome::{
    Choice, Fault, FaultMessage, KeptOutcome, MessageChoice, Messages, Said, Sentence, UndoStep,
    Verb,
};
use std::collections::HashSet;

/// What the view keeps between frames: a typed amount and the DM's unticked proposals.
#[derive(Default)]
pub(crate) struct ViewState {
    /// A typed amount: the tracker, its instance, the action's label, and the words so far.
    asking: Option<(String, Option<String>, String, String)>,
    focus_asking: bool,
    /// Proposed changes the DM unticked, by notice and the change's own key. Ticked is the
    /// default.
    unticked: HashSet<Tick>,
    /// The act pressed in this frame. The shell applies it after Lookup drew, once it has read
    /// the day count again, so an act uses the day at the click.
    pressed: Option<Act>,
}

/// The fewest digits a value slot holds, so a value that gains a digit moves no action.
const SLOT_DIGITS: usize = 3;

/// The text a value's slot is measured on: its digits and its goal's, at least
/// `SLOT_DIGITS`, as zeros. The digits are tabular, so every value up to that many digits
/// fits the slot exactly.
pub fn value_slot(value: i64, goal: Option<i64>) -> String {
    let digits = |n: i64| n.unsigned_abs().to_string().len();
    let wide = digits(value).max(goal.map_or(0, digits)).max(SLOT_DIGITS);
    let sign = if value < 0 { "-" } else { "" };
    let zeros = "0".repeat(wide);
    match goal {
        Some(goal) => format!("{sign}{zeros} / {goal}"),
        None => format!("{sign}{zeros}"),
    }
}

impl ViewState {
    /// The amount field again, with the number typed for an act that recorded nothing.
    pub(super) fn ask_again(
        &mut self,
        key: &str,
        instance: Option<&str>,
        label: &str,
        amount: i64,
    ) {
        self.asking = Some((
            key.to_owned(),
            instance.map(str::to_owned),
            label.to_owned(),
            amount.to_string(),
        ));
        self.focus_asking = true;
    }

    /// The words in the amount field, while it is open.
    #[cfg(feature = "test-support")]
    pub(super) fn typed(&self) -> Option<&str> {
        self.asking.as_ref().map(|(_, _, _, typed)| typed.as_str())
    }
}

/// A proposed change of a Due row, by its notice and its own key (`ChangeKey`).
pub type Tick = (NoticeRef, ChangeKey);

/// A mark's change by its own identity, not its place in the mark (Astra's P2 on PR 897): the
/// tracker it changes, its line, what it does there (its `add` and `set`), and which of the
/// mark's changes that are the same in all of these it is (Astra's P2 on PR 904). A change that
/// leaves the mark before it moves no other change's key, also on the same line: its check box
/// keeps its id (and its focus), and the DM's untick stays with its change.
pub type ChangeKey = (String, Option<String>, String, usize);

/// Each change's key, in the mark's order (`ChangeKey`).
fn change_keys(notice: &super::Notice, mark: Says<'_>) -> Vec<ChangeKey> {
    let mut keys: Vec<ChangeKey> = Vec::with_capacity(mark.changes.len());
    for change in mark.changes {
        let tracker = change.tracker.clone().unwrap_or_else(|| notice.key.clone());
        let instance = change.instance.clone();
        let does = format!("{:?} {:?}", change.add, change.set);
        let nth = keys
            .iter()
            .filter(|(t, i, d, _)| *t == tracker && *i == instance && *d == does)
            .count();
        keys.push((tracker, instance, does, nth));
    }
    keys
}

/// The empty state's route to the switches of the house rules in play.
pub const RULES_ROUTE: &str = "Campaign → Rules and runs";

/// The Due list's words after the Timeline's own, while the day count is not known.
pub(crate) const DAY_MARKS_WAIT: Sentence =
    crate::sentence!("No day mark shows until the Timeline is read.");

/// One act the DM asked for in this frame.
#[derive(Clone)]
pub enum Act {
    Press {
        key: String,
        instance: Option<String>,
        label: String,
        /// The action's place in `quick`: two actions can share a label.
        at: usize,
        asked: Option<i64>,
    },
    Roll(super::Notice, String),
    Done(super::Notice, Vec<bool>),
    Dismiss(super::Notice),
    DismissAll(super::Notice),
    Undo(Vec<Entry>),
    AddToTimeline(String, String),
    UndoAdd(String),
    ReadAgain,
}

impl Trackers {
    /// Does a typed amount hold Escape? Then Escape ends the amount, not Lookup.
    pub fn takes_escape(&self) -> bool {
        self.view.asking.is_some()
    }

    /// Was an act pressed in the view this frame?
    pub fn pressed(&self) -> bool {
        self.view.pressed.is_some()
    }

    /// Apply the act pressed in the view. The shell calls it after `follow_clock`, so an act
    /// on a day notice reads the day count's one answer, also when it is not known.
    pub fn apply_pressed(&mut self) {
        if let Some(act) = self.view.pressed.take() {
            apply(self, act);
        }
    }

    /// An act pressed in the view: the shell applies it (`apply_pressed`) after it has read the
    /// day count again.
    pub fn press_in_view(&mut self, act: Act) {
        self.view.pressed = Some(act);
    }

    /// The typed amount while its field is open: the tracker, its line, the action's label and
    /// the words so far.
    pub fn asking(&self) -> Option<(&str, Option<&str>, &str, &str)> {
        self.view
            .asking
            .as_ref()
            .map(|(key, instance, label, typed)| {
                (
                    key.as_str(),
                    instance.as_deref(),
                    label.as_str(),
                    typed.as_str(),
                )
            })
    }

    /// An action that asks for an amount was pressed: its field opens in its place, with focus.
    pub fn ask(&mut self, key: &str, instance: Option<&str>, label: &str) {
        self.view.asking = Some((
            key.to_owned(),
            instance.map(str::to_owned),
            label.to_owned(),
            String::new(),
        ));
        self.view.focus_asking = true;
    }

    /// The words typed in the open amount field.
    pub fn set_typed(&mut self, words: &str) {
        if let Some((_, _, _, typed)) = self.view.asking.as_mut() {
            words.clone_into(typed);
        }
    }

    /// Whether the amount field takes focus in this frame (once).
    pub fn take_focus_asking(&mut self) -> bool {
        std::mem::take(&mut self.view.focus_asking)
    }

    /// The amount field closes: Enter presses its action (`at`, its place in `quick`) with the
    /// typed number (a number that does not read presses nothing), Escape and focus elsewhere
    /// press nothing.
    pub fn end_asking(&mut self, enter: bool, at: usize) {
        let Some((key, instance, label, typed)) = self.view.asking.take() else {
            return;
        };
        if !enter {
            return;
        }
        let Some(amount) = crate::design::parse_signed(&typed) else {
            return;
        };
        self.view.pressed = Some(Act::Press {
            key,
            instance,
            label,
            at,
            asked: Some(amount),
        });
    }

    /// Whether a proposed change of a Due row is ticked (the default) or the DM unticked it.
    pub fn ticked(&self, tick: &Tick) -> bool {
        !self.view.unticked.contains(tick)
    }

    /// Tick or untick a proposed change of a Due row.
    pub fn set_ticked(&mut self, tick: &Tick, on: bool) {
        if on {
            self.view.unticked.remove(tick);
        } else {
            self.view.unticked.insert(tick.clone());
        }
    }

    /// The trackers' revision: it moves on at each settle, so a view keeps what it worked out
    /// from the values and definitions until it moves.
    pub fn revision(&self) -> crate::design::memo::Revision {
        self.revision.revision()
    }

    /// The problems list, each with the house rule note it belongs to.
    pub fn problems(&self) -> &[(Option<uuid::Uuid>, String)] {
        &self.problems
    }

    /// `Trackers.md` of the campaign, while one is open.
    pub fn ledger_path(&self) -> Option<std::path::PathBuf> {
        self.path()
    }

    /// `Try again` on a ledger that cannot be read: read it again now.
    pub fn read_again(&mut self) {
        self.view.pressed = Some(Act::ReadAgain);
    }

    /// `Done` pressed on this row, as a click in the view leaves it for the shell.
    #[cfg(feature = "test-support")]
    pub fn press_done_in_view(&mut self, notice: &super::Notice, ticked: Vec<bool>) {
        self.view.pressed = Some(Act::Done(notice.clone(), ticked));
    }
}

/// What the DM can press in the view's footer and in its placeholder.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrackerPress {
    /// `Undo: <label>`: the newest act of the session being played.
    UndoAct,
    AddToTimeline,
    /// `Undo add`: the event `Add to Timeline` made.
    UndoAdd,
    ReadAgain,
    ShowFile,
}

fn apply(trackers: &mut Trackers, act: Act) {
    match act {
        Act::Press {
            key,
            instance,
            label,
            at,
            asked,
        } => {
            let action = super::QuickRef {
                label: &label,
                at: Some(at),
            };
            trackers.press_at(&key, instance.as_deref(), action, asked);
        }
        Act::Roll(notice, table) => trackers.roll_notice(&notice, &table),
        Act::Done(notice, ticked) => {
            trackers.done(&notice, &ticked);
            if trackers.reason.is_none() {
                trackers.view.unticked.retain(|(id, _)| *id != notice.id);
            }
        }
        Act::Dismiss(notice) => trackers.dismiss(&notice),
        Act::DismissAll(notice) => trackers.dismiss_all(&notice),
        Act::Undo(entries) => trackers.undo(&entries),
        Act::AddToTimeline(entry, label) => trackers.wants_timeline = Some((entry, label)),
        Act::UndoAdd(event) => {
            trackers.added = None;
            trackers.wants_undo_add = Some(event);
        }
        Act::ReadAgain => {
            trackers.baseline = None;
            trackers.read_disk();
        }
    }
}

/// A Due row's parts, as `design::NoticeRow` lays them: its words, each roll's label with why
/// it is blocked, the rolls' results, and each proposed change's words.
pub struct DueParts {
    pub words: String,
    pub rolls: Vec<(String, Option<&'static str>)>,
    pub results: Option<String>,
    pub checks: Vec<String>,
    /// The notice's short name (the row's menu's), `Done`'s label and its accessible name.
    pub short: String,
    pub finish: String,
    pub done_name: String,
    /// Each check's place in the DM's unticked set: its notice and its change's own key.
    pub ticks: Vec<Tick>,
}

/// Why a roll is blocked: each name of its table has its roll.
const EACH_ROLLED: &str = "Each name has its roll.";

pub fn due_parts(rules: &Rules, notice: &super::Notice, mark: Says<'_>) -> DueParts {
    let rolls = mark
        .tables
        .iter()
        .map(|table| {
            let key = table.key();
            let rolled = notice.rolls.iter().filter(|r| r.table == key).count();
            let each_rolled = !table.names().is_empty() && rolled >= table.names().len();
            (
                format!("Roll {}", rules.table_name(key)),
                each_rolled.then_some(EACH_ROLLED),
            )
        })
        .collect();
    // The results are listed under the buttons, one line each, as one block of words.
    let results = (!mark.tables.is_empty() && !notice.rolls.is_empty()).then(|| {
        notice
            .rolls
            .iter()
            .map(|roll| {
                let name = roll
                    .name
                    .as_deref()
                    .unwrap_or_else(|| rules.table_name(&roll.table));
                format!("{name}: {} · {}", roll.value, roll.row)
            })
            .collect::<Vec<_>>()
            .join("\n")
    });
    let checks = mark
        .changes
        .iter()
        .map(|change| {
            let key = change.tracker.as_deref().unwrap_or(&notice.key);
            let name = rules.def(key).map_or(key, |def| def.name.as_str());
            let name = match &change.instance {
                Some(line) => format!("{name} ({line})"),
                None => name.to_owned(),
            };
            let amount = match (&change.add, change.set) {
                (_, Some(set)) => format!("set to {set}"),
                (Some(super::Amount::Number(n)), None) => crate::design::signed(*n),
                (Some(super::Amount::Words(words)), None) => format!("+{words}"),
                (None, None) => String::new(),
            };
            format!("{name} {amount}")
        })
        .collect();
    let short = notice.name(rules);
    DueParts {
        words: notice_words(rules, notice, mark),
        ticks: change_keys(notice, mark)
            .into_iter()
            .map(|key| (notice.id.clone(), key))
            .collect(),
        done_name: format!("Done: {short}"),
        finish: finish_label(notice),
        short,
        rolls,
        results,
        checks,
    }
}

/// A Due row's words. A mark at a value names its line: `Old Guard: First seal.` A day mark
/// names its day, and a row for several days says how many and which comes next, so an older
/// period says so before it runs: `End of month ×3. Next: Day 32. Move the rival powers.`
fn notice_words(rules: &Rules, notice: &super::Notice, mark: Says<'_>) -> String {
    match (&notice.id, notice.days.as_slice(), &notice.instance) {
        (NoticeRef::Day { day, .. }, [_], _) => format!("Day {day}: {}", mark.say),
        (NoticeRef::Day { mark: key, day }, days, _) => format!(
            "{} ×{}. Next: Day {day}. {}",
            rules.day_title(key),
            days.len(),
            mark.say
        ),
        (_, _, Some(name)) => format!("{name}: {}", mark.say),
        (_, _, None) => mark.say.to_owned(),
    }
}

/// The row's last action: `Done`, which applies the ticked changes and ends one notice. On a
/// day mark's row of several days it names the day it ends, the oldest: `Done: Day 32`.
fn finish_label(notice: &super::Notice) -> String {
    match (&notice.id, notice.days.len()) {
        (NoticeRef::Day { day, .. }, 2..) => format!("Done: Day {day}"),
        _ => "Done".to_owned(),
    }
}

/// One line of a tracker as words, before any layout: its name (the instance's, or the
/// tracker's), its value (with its goal), the actions it shows (by their place in `quick`, each
/// with its accessible name, `Pay: Ann` on an instance's line), its instance with how many lines
/// before it have the same one, and, of two amount fields with one owner (`Repeat`), the fault
/// the line that leaves one out says under it, once for the tracker.
pub struct LineWords {
    pub name: String,
    pub value: String,
    pub actions: Vec<(usize, String)>,
    pub instance: Option<String>,
    pub twin: usize,
    pub fault: Option<TrackerFault>,
}

/// Each line of a tracker as words (`LineWords`), in its order: one line, or one for each
/// instance. The GPUI view lays them out and draws them.
pub fn line_words(
    rules: &Rules,
    state: &State,
    key: &str,
    def: &super::Definition,
) -> Vec<LineWords> {
    let repeat = def.repeat.as_ref();
    let (mut word_seen, mut said) = (false, false);
    let instances = def.instances();
    let mut lines = Vec::with_capacity(instances.len());
    for (line, &instance) in instances.iter().enumerate() {
        // The line by its own identity: its instance, and how many lines before it have the same
        // one (`per` can name a line twice).
        let twin = instances[..line].iter().filter(|&&i| i == instance).count();
        // The first line of a repeated name keeps its fields; a later one leaves them out.
        let repeated_line = match repeat {
            Some(super::Repeat::Line { word, .. }) if instance == Some(word.as_str()) => {
                std::mem::replace(&mut word_seen, true)
            }
            _ => false,
        };
        let left_out = |at: usize| match repeat {
            Some(super::Repeat::Line { asking, .. }) => repeated_line && asking.contains(&at),
            Some(super::Repeat::Label { label, keep }) => {
                at != *keep && def.quick[at].label == *label
            }
            None => false,
        };
        let leaves_out = match repeat {
            Some(super::Repeat::Line { .. }) => repeated_line,
            Some(super::Repeat::Label { .. }) => true,
            None => false,
        };
        let fault = repeat
            .filter(|_| leaves_out && !said)
            .map(|repeat| TrackerFault::Problem(super::repeat_words(repeat)));
        said |= fault.is_some();
        let actions: Vec<(usize, String)> = (0..def.quick.len())
            .filter(|at| !left_out(*at))
            .map(|at| {
                let label = &def.quick[at].label;
                let named = match instance {
                    Some(name) => format!("{label}: {name}"),
                    None => label.clone(),
                };
                (at, named)
            })
            .collect();
        let value = match def.goal {
            Some(goal) => format!("{} / {goal}", state.value(rules, key, instance)),
            None => state.value(rules, key, instance).to_string(),
        };
        lines.push(LineWords {
            name: instance.unwrap_or(&def.name).to_owned(),
            value,
            actions,
            instance: instance.map(str::to_owned),
            twin,
            fault,
        });
    }
    lines
}

/// The view's footer messages, made before the body runs: why the last act recorded nothing,
/// then the last act of the session being played with `Undo: <label>` and `Add to Timeline`, or,
/// after `Add to Timeline`, `Added to the Timeline.` with `Undo add`. Nothing shows while
/// `Trackers.md` cannot be read.
pub fn messages<'f>(frame: &'f PaneFrame, trackers: &Trackers) -> Messages<'f, TrackerPress> {
    let mut messages = Messages::new();
    if trackers.unreadable.is_some() {
        return messages;
    }
    match &trackers.reason {
        Some(Unrecorded::Refused(refusal)) => {
            messages.outcome(FaultMessage::new(&TrackerFault::Refused(refusal.clone())));
        }
        Some(Unrecorded::NotSaved(error)) => {
            messages.outcome(FaultMessage::new(&TrackerFault::NotSaved(error.clone())));
        }
        None => {}
    }
    let entries = trackers.newest();
    let (Some(first), Some((label, words))) = (entries.first(), trackers.result.as_ref()) else {
        return messages;
    };
    let added = trackers
        .added
        .as_ref()
        .filter(|added| added.entry == first.id);
    let Some(shown) = trackers
        .played_owner()
        .and_then(|owner| trackers.results.get(&owner))
    else {
        return messages;
    };
    let line = shown.outcome(frame, |_| match added {
        // The event waits in the Timeline's draft when its save failed: say so, not success.
        Some(added) => match &added.not_saved {
            Some(words) => KeptOutcome::Fault(
                FaultMessage::new(&TrackerFault::TimelineNotSaved(words.clone())),
                UndoStep::of(Verb::Add, added.title.clone()).pressed_as(TrackerPress::UndoAdd),
            ),
            None => Said::new(crate::sentence!("Added to the Timeline."))
                .undo(
                    UndoStep::of(Verb::Add, added.title.clone()).pressed_as(TrackerPress::UndoAdd),
                )
                .into(),
        },
        None => Said::new(words.clone())
            .undo(UndoStep::act(label.as_str()).pressed_as(TrackerPress::UndoAct))
            .choice(Choice::new(
                MessageChoice::Choice(ADD_TO_TIMELINE),
                TrackerPress::AddToTimeline,
            ))
            .into(),
    });
    messages.kept(line);
    messages
}

/// The act a footer press asks for, from the view's state in this frame.
pub fn footer_press(trackers: &Trackers, pressed: Option<TrackerPress>, act: &mut Option<Act>) {
    let entries = trackers.newest();
    let first = entries.first();
    let added = first.and_then(|first| {
        trackers
            .added
            .as_ref()
            .filter(|added| added.entry == first.id)
    });
    match (pressed, first, added) {
        (Some(TrackerPress::UndoAdd), _, Some(added)) => {
            *act = Some(Act::UndoAdd(added.event.clone()));
        }
        (Some(TrackerPress::UndoAct), Some(_), None) => *act = Some(Act::Undo(entries.to_vec())),
        (Some(TrackerPress::AddToTimeline), Some(first), None) => {
            let title = super::timeline_title(&trackers.rules, &trackers.ledger, entries);
            *act = Some(Act::AddToTimeline(first.id.clone(), title));
        }
        _ => {}
    }
}

/// The result's choice that makes a Major event of the act.
const ADD_TO_TIMELINE: &str = "Add to Timeline";

/// The words of a failed save: the file, the reason, and that nothing was recorded.
pub fn not_saved(error: &str) -> Sentence {
    TrackerFault::NotSaved(error.to_owned()).cause()
}
