//! The encounter ledger and its local editor controls.
#![warn(deprecated)] // Its parent allows old message functions; this file does not.
use super::*;
use crate::encounter_run::{FightSummary, SavedRunInfo};
use std::path::Path;

/// Why a Ready row's Start fight is blocked while its run is open (FJ7), and the link to it.
pub const FIGHT_OPEN: &str = "This fight is open.";
pub const GO_TO_RUN: &str = "Go to the run";

/// The design's reason line, which never justifies its words (M-P0.26): this list's own copy
/// justified them in a menu.
pub(super) use crate::design::blocked_reason;

/// The quiet `⋯` menu of a row or a header. The menu part places and sizes it (`menu_popup`):
/// inside the pane it opens from, clear of its splitter and scroll bar, measured at each opening.
pub(super) fn row_menu(ui: &mut egui::Ui, name: &str, add: impl FnOnce(&mut egui::Ui)) {
    let name = format!("More actions for {name}");
    // The pane the menu opens from.
    let inside = ui.clip_rect();
    let button = Action::quiet(icons::EXPAND)
        .density(Density::Read)
        .name(&name)
        .hint(&name)
        .show(ui);
    let id = egui::Popup::default_response_id(&button);
    // When the pane changes size under an open menu (the window shrinks), the pane brings the
    // menu's button back into view, so the menu does not hang from a button the DM cannot see.
    // Only a resize does this: the DM's own scrolling stays where the DM leaves it.
    let pane = id.with("pane size");
    if egui::Popup::is_id_open(ui.ctx(), id) {
        let last = ui.data_mut(|data| data.get_temp::<egui::Vec2>(pane));
        let resized = last.is_some_and(|last| last != inside.size());
        let pending = pane.with("bring back");
        // Review of PR 770: any scroll of the DM's in this column (a wheel, a drag of its content
        // or its bar, a key) ends the retry, so the column stays where the DM leaves it.
        let scrolled = design::reveal::dm_scrolls(ui);
        let bring_back = !scrolled
            && (resized
                || ui
                    .data_mut(|data| data.get_temp::<bool>(pending))
                    .unwrap_or(false));
        if bring_back && !inside.contains_rect(button.rect) {
            // The layout can settle a frame after the resize: keep asking until the button shows.
            ui.data_mut(|data| data.insert_temp(pending, true));
            // The least scroll that shows it, written by the pane's own column (M0259).
            design::reveal::bring_into_view(
                ui,
                button.rect,
                design::reveal::JumpTo::OpenMenuControl,
            );
        } else {
            ui.data_mut(|data| data.remove::<bool>(pending));
        }
        ui.data_mut(|data| data.insert_temp(pane, inside.size()));
    } else {
        ui.data_mut(|data| data.remove::<egui::Vec2>(pane));
    }
    design::menu_popup(ui, &button).show(add);
}

/// FM1: a menu whose first choice has no icon, but a later one has, says so before its first
/// choice: egui sizes a menu on its first pass, so a slot that came later would cut the choices
/// before it (`Edit encounter` above `Remove`, R12).
fn menu_has_icons(ui: &egui::Ui) {
    let mark = ui.layer_id().id.with("menu has icons");
    ui.data_mut(|data| data.insert_temp(mark, true));
}

fn menu_icon(word: &str) -> Option<&'static str> {
    match word {
        "Move up" => Some(icons::MOVE_UP),
        "Move down" => Some(icons::MOVE_DOWN),
        "Remove" | "Delete the saved run" => Some(icons::REMOVE),
        _ => None,
    }
}

/// A line under a menu choice: why it is blocked, or what it does, at the choice's words
/// (`design::choice_note`), after the icon slot when the menu keeps one (FM1).
pub(super) fn choice_note(ui: &mut egui::Ui, word: &str, note: Option<&str>) {
    let slot = menu_slot_icon(ui, word).is_some();
    design::choice_note(ui, slot, note);
}

pub(super) fn menu_action(ui: &mut egui::Ui, word: &str, blocked: Option<&str>) -> bool {
    let icon = menu_slot_icon(ui, word);
    let action = match icon {
        Some(icon) => Action::quiet(word).icon(icon),
        None => Action::quiet(word),
    };
    let pressed = action
        .density(Density::Read)
        .blocked(blocked)
        .show(ui)
        .clicked();
    choice_note(ui, word, blocked);
    if pressed {
        ui.close();
    }
    pressed
}

/// FM1: a choice's icon, or an empty icon slot when its menu has a choice with an icon, so every
/// choice's words start at one x. The menu is its popup's layer; a choice with an icon marks it,
/// and the menu draws first on a frame it is not shown, so the mark is there when it shows.
fn menu_slot_icon(ui: &egui::Ui, word: &str) -> Option<&'static str> {
    let mark = ui.layer_id().id.with("menu has icons");
    match menu_icon(word) {
        Some(icon) => {
            ui.data_mut(|data| data.insert_temp(mark, true));
            Some(icon)
        }
        None => ui
            .data(|data| data.get_temp::<bool>(mark))
            .unwrap_or(false)
            .then_some(""),
    }
}

/// A side picker's two fields: the choice of a named side, and the name of another side.
pub(super) const SIDE_CHOICE: design::FieldName = crate::field_id!("side_choice");
pub(super) const SIDE_OTHER: design::FieldName = crate::field_id!("side_other");

/// The words an other-side field holds while it has the keyboard: as typed, over the stored
/// side they were typed over, and the frame that last drew the field.
#[derive(Clone)]
struct HeldSide {
    typed: String,
    over: String,
    frame: u64,
}

/// Every other-side field's held words, by the field's id: one entry, pruned of fields not drawn.
type HeldSides = std::collections::HashMap<egui::Id, HeldSide>;
const HELD_SIDES: &str = "side_other_held";

/// What a side picker offers: a named side, or another side the DM types.
#[derive(Clone, Debug, PartialEq, Eq)]
enum SideChoice {
    Named(String),
    Other,
}

/// The same named side picker serves roster slots and groups. Other opens a text field. Lane E2's
/// pressure caller (round 31 SD11, chosen-value-weight): the choice is the `Dropdown` part, whose
/// closed value is regular `Body` weight, and the other side's name is the shared line field.
/// `choice` and `other` are its two fields, keyed by the object whose side it is.
pub(super) fn side_control(
    ui: &mut egui::Ui,
    (choice, other_field): (design::FieldId, design::FieldId),
    name: &str,
    side: &mut String,
    choices: &[String],
) {
    let id = ui.make_persistent_id((name, "other_side"));
    let custom = !matches!(side.as_str(), "party" | "enemies")
        && !choices.iter().any(|choice| choice == side);
    let mut other = ui.data(|data| data.get_temp::<bool>(id)).unwrap_or(false);
    ui.horizontal(|ui| {
        if other {
            // While the field has the keyboard it keeps the words as typed, so its Undo history
            // stays whole (review of PR 613); the side's stored form (`friends`, shown
            // `Friends`) comes back when the field lets go. The words belong to one field (its
            // id, which its owner's key makes) and to the stored side they were typed over: a side
            // changed from outside (Undo of a discard) or a field not drawn in the last frame
            // drops them (review of PR 695).
            let field_id = other_field.id();
            let frame = ui.ctx().cumulative_frame_nr();
            let held = ui.data_mut(|data| {
                let held = data.get_temp_mut_or_default::<HeldSides>(egui::Id::new(HELD_SIDES));
                held.retain(|_, words| words.frame + 1 >= frame);
                held.get(&field_id)
                    .filter(|words| words.over == *side)
                    .map(|words| words.typed.clone())
            });
            let mut text = held.unwrap_or_else(|| {
                if custom {
                    design::reader::human_label(side)
                } else {
                    String::new()
                }
            });
            let field = design::line_field(
                ui,
                other_field,
                &format!("Other side for {name}"),
                &mut text,
                "Side name",
                design::S * 10.0,
            );
            if field.changed() && !text.trim().is_empty() {
                *side = text.trim().to_lowercase();
            }
            // Read before the data lock: `has_focus` takes the context's lock too.
            let focused = field.has_focus();
            ui.data_mut(|data| {
                let held = data.get_temp_mut_or_default::<HeldSides>(egui::Id::new(HELD_SIDES));
                if focused {
                    held.insert(
                        field_id,
                        HeldSide {
                            typed: text,
                            over: side.clone(),
                            frame,
                        },
                    );
                } else {
                    held.remove(&field_id);
                }
            });
        }
        let mut named: Vec<&str> = ["party", "enemies"]
            .into_iter()
            .chain(
                choices
                    .iter()
                    .map(String::as_str)
                    .filter(|choice| !matches!(*choice, "party" | "enemies")),
            )
            .collect();
        // A side typed before (`rivals`) is the current value: the control shows it, and the menu
        // keeps it among the named sides.
        if custom && !other && !side.trim().is_empty() {
            named.push(side.as_str());
        }
        let mut chosen = if other {
            SideChoice::Other
        } else {
            SideChoice::Named(side.clone())
        };
        let mut dropdown = design::Dropdown::new(choice, name)
            .width(if other {
                Density::Read.control()
            } else {
                design::S * 13.0
            })
            .choice(
                SideChoice::Named(named[0].to_owned()),
                design::reader::human_label(named[0]),
            );
        for value in &named[1..] {
            dropdown = dropdown.choices([(
                SideChoice::Named((*value).to_owned()),
                design::reader::human_label(value),
            )]);
        }
        // While another side is typed, the control shows only its arrow.
        let shown = dropdown
            .choice_with_short_label(SideChoice::Other, "Other…", "")
            .show(ui, &mut chosen);
        if shown.changed {
            match chosen {
                SideChoice::Named(value) => {
                    *side = value;
                    other = false;
                }
                SideChoice::Other => other = true,
            }
        }
    });
    ui.data_mut(|data| data.insert_temp(id, other));
}

#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
)]
fn heading(ui: &mut egui::Ui, word: &str, count: usize) {
    design::group_heading(ui, word, count);
}

/// Q-o: the In progress heading says the rows' order after its count, in the count's muted
/// form, from `Arranged::in_progress_order` (the rows as arranged, the open run first).
fn in_progress_heading(ui: &mut egui::Ui, page: &history::Arranged<'_>) {
    let mut job = design::group_heading_text(ui, "IN PROGRESS", page.in_progress.len());
    if let Some(order) = page.in_progress_order()
        && let Some(format) = job.sections.last().map(|section| section.format.clone())
    {
        job.append(&format!(" · {order}"), 0.0, format);
    }
    ui.label(job);
}

fn section(
    ui: &mut egui::Ui,
    id: &str,
    word: &str,
    count: usize,
    open: Option<bool>,
    add: impl FnOnce(&mut egui::Ui),
) {
    design::section_opened(
        ui,
        id,
        design::group_heading_text(ui, word, count),
        open,
        add,
    );
}

/// G118: a Ready row's difficulty line. It takes the roster's TYPED parts (`Roster::parts`),
/// never built words, so a member the summary shows is not named again after `XP not known`.
/// The line is kept in the page's `kept` under the roster's revision, so an unchanged frame
/// builds and measures nothing.
pub(super) struct ReadyLine<'a> {
    pub id: Uuid,
    pub difficulty: &'a crate::difficulty::Difficulty,
    pub roster: &'a super::history::Roster,
    pub kept: &'a mut crate::difficulty::Kept<Uuid>,
}

/// Layout PR 5a (I6, §4.1): one list's place in the ledger's columns. The facts column is
/// measured across In progress, Ready, Groups and History (`ledger`), so each list's actions
/// start `L` after the widest facts of the whole ledger, and the list's widest action recipe
/// (with its alternates) sets where its actions end. Spare width lies after the actions. Where
/// the facts, `L` and the actions do not fit, the actions keep the far edge and the facts give
/// way, as before.
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct Column {
    /// Where the ledger's rows start (the screen x), and its facts column's width from there:
    /// a list in a section starts one indent further in, and its facts are that much narrower.
    pub left: f32,
    pub facts: f32,
    pub actions: f32,
    /// The list, as the gate's record of its rows names it.
    #[cfg_attr(not(any(test, feature = "test-support")), allow(dead_code))]
    pub owner: &'static str,
}

/// The room a row's second line keeps for its icons: the run icon, and a leading warning's
/// icon after it.
fn icon_room(status: bool, detail: &str) -> f32 {
    if status {
        design::L
            + if detail.starts_with("Changed on disk") {
                design::L
            } else {
                0.0
            }
    } else {
        0.0
    }
}

/// A row's facts at their whole width: its name, or its second line with the line's icons.
/// `line` is a Ready row's whole difficulty line, kept with its roster (`Kept::widest`).
fn facts_width(ui: &egui::Ui, name: &str, detail: &str, status: bool, line: Option<f32>) -> f32 {
    let second = line.unwrap_or_else(|| design::number_width(ui, detail, &Type::Small.font(ui)));
    design::number_width(ui, name, &Type::Strong.font(ui)).max(icon_room(status, detail) + second)
}

/// A Ready row's whole line, kept with its roster: the facts column measures it.
#[cfg(any(test, feature = "test-support"))]
fn ready_line_width(ui: &egui::Ui, line: &mut ReadyLine<'_>) -> Option<f32> {
    let (difficulty, roster) = (line.difficulty, line.roster);
    line.kept
        .widest(ui, &line.id, roster.revision, &Type::Small.font(ui), || {
            crate::difficulty::Line::roster(difficulty, &roster.parts)
        })
}

/// The width of actions side by side, with the layout's item spacing between them.
fn spaced(ui: &egui::Ui, widths: impl IntoIterator<Item = f32>) -> f32 {
    let (sum, count) = widths
        .into_iter()
        .fold((0.0, 0_usize), |(sum, count), width| {
            (sum + width, count + 1)
        });
    sum + ui.spacing().item_spacing.x * count.saturating_sub(1) as f32
}

fn quiet_width(ui: &egui::Ui, words: &str) -> f32 {
    Action::quiet(words)
        .density(Density::Read)
        .natural_width(ui)
}

/// An action with the run icon: primary on the first row, secondary after it. The wider.
fn run_action_width(ui: &egui::Ui, words: &str) -> f32 {
    let width = |action: Action<'_>| {
        action
            .icon(icons::RUN)
            .density(Density::Read)
            .natural_width(ui)
    };
    width(Action::primary(words)).max(width(Action::secondary(words)))
}

/// The In progress row's Edit slot: as wide where the row has no Edit, so Resume keeps its place.
fn edit_slot_width(ui: &egui::Ui) -> f32 {
    design::number_width(ui, "Edit", &Type::Strong.font(ui)) + design::L
}

/// An In progress row's actions: its menu, the Edit slot, and Resume, or the open run's state.
fn in_progress_actions(ui: &egui::Ui, open_state: Option<&str>) -> f32 {
    let last = open_state.map_or_else(
        || run_action_width(ui, "Resume"),
        |word| design::number_width(ui, &format!("{} {word}", icons::RUN), &Type::Small.font(ui)),
    );
    spaced(
        ui,
        [
            quiet_width(ui, icons::EXPAND),
            edit_slot_width(ui).max(quiet_width(ui, "Edit")),
            last,
        ],
    )
}

/// A Ready row's actions: its menu, Edit, Start fight, and the short reason Start is blocked,
/// with the link to an open run. The reason keeps `L` before it, so it does not read as the
/// facts' second line.
fn ready_actions(ui: &egui::Ui, reason: Option<&str>, open: bool) -> f32 {
    let small = Type::Small.font(ui);
    let link = (open && reason.is_some())
        .then(|| design::number_width(ui, GO_TO_RUN, &small).max(design::XL));
    // The gap after the reason follows the layout's item spacing.
    let reason = reason.map(|reason| {
        design::number_width(ui, reason, &small) + ui.spacing().item_spacing.x + design::L
    });
    let fixed = [
        quiet_width(ui, icons::EXPAND),
        quiet_width(ui, "Edit"),
        run_action_width(ui, START_FIGHT),
    ];
    spaced(ui, fixed.into_iter().chain(link).chain(reason))
}

/// A History row's actions: its menu, and Open or Close (the wider, so opening keeps the column).
fn history_actions(ui: &egui::Ui) -> f32 {
    let open = quiet_width(ui, "Open").max(quiet_width(ui, "Close"));
    spaced(ui, [quiet_width(ui, icons::EXPAND), open])
}

/// A group row's actions: its menu and Edit.
pub(super) fn group_actions(ui: &egui::Ui) -> f32 {
    spaced(
        ui,
        [quiet_width(ui, icons::EXPAND), quiet_width(ui, "Edit")],
    )
}

/// The ledger's columns, measured before its first list is drawn: one facts column across all
/// its lists (also a closed section's, so opening it moves no action), and each list's widest
/// action recipe.
pub(super) struct Ledger {
    pub in_progress: Column,
    pub ready: Column,
    pub groups: Column,
    pub history: Column,
}

pub(super) fn ledger(
    ui: &egui::Ui,
    page: &history::Arranged<'_>,
    snapshot: &Snapshot,
    live: Option<&SavedRunInfo>,
    story: &Story<'_>,
    kept: &mut crate::difficulty::Kept<Uuid>,
) -> Ledger {
    // Groups and History are sections: their rows start one indent further in.
    let (left, indent) = (ui.cursor().left(), ui.spacing().indent);
    let small = Type::Small.font(ui);
    let (body, small_measure) = (
        crate::design::memo::Measure::whole(ui, &Type::Body.font(ui)),
        crate::design::memo::Measure::whole(ui, &small),
    );
    let minute = crate::hermetic::now() / 60;
    // Review of #875 (P2, #589): In progress is measured once per change of what its rows show
    // (the campaign's read and the run list's, the open run as its row reads it, and the story:
    // the Timeline's events and the Sessions folder's listing that name a paused run's session),
    // or of what they are measured in, and once a minute for "Saved N minutes ago". A story with
    // no revision (a test's fixed lists) is measured on each frame.
    let runs = snapshot.runs_read.revision();
    let progress_key = story.revision.map(|story| InProgressKey {
        snapshot: snapshot.revision,
        runs,
        open: live.map(open_run_words),
        story,
        minute,
        body: body.clone(),
        small: small_measure.clone(),
    });
    let progress_id = ui.id().with("encounters_ledger_in_progress");
    let kept_progress = progress_key.as_ref().and_then(|key| {
        ui.ctx()
            .data(|data| data.get_temp::<(InProgressKey, [f32; 2])>(progress_id))
            .filter(|(at, _)| at == key)
            .map(|(_, widths)| widths)
    });
    let [in_facts, in_progress] = kept_progress.unwrap_or_else(|| {
        #[cfg(any(test, feature = "test-support"))]
        LEDGER_MEASURED.with(|count| count.set(count.get() + 1));
        let mut in_facts = 0.0_f32;
        let mut in_progress = 0.0_f32;
        for run in &page.in_progress {
            let open = live.is_some_and(|live| live.id == run.id);
            let detail = paused_run_detail(run, &page.in_progress, open, story);
            in_facts = in_facts.max(facts_width(ui, &run.encounter, &detail, !open, None));
            let state = open.then(|| run_state_word(run, false));
            in_progress = in_progress.max(in_progress_actions(ui, state.as_deref()));
        }
        if let Some(key) = progress_key {
            ui.ctx().data_mut(|data| {
                data.insert_temp(progress_id, (key, [in_facts, in_progress]));
            });
        }
        [in_facts, in_progress]
    });
    // Review of PR 581 (#589): Ready, Groups and History are measured once per change of what
    // they show (the campaign's read, the run list's read: review of #875, a refresh or a
    // deleted save in the same minute; the groups' revisions, the open run) or of what they are
    // measured in (`memo::Measure`, the indent), and once a minute for History's saved times.
    let key = LedgerKey {
        snapshot: snapshot.revision,
        runs,
        groups: super::groups::revisions_issued(),
        live: live.map(|live| live.id),
        minute,
        body,
        small: small_measure,
        indent: indent.to_bits(),
    };
    let kept_id = ui.id().with("encounters_ledger_kept");
    let kept_lists = ui
        .ctx()
        .data(|data| data.get_temp::<(LedgerKey, [f32; 4])>(kept_id))
        .filter(|(at, _)| *at == key)
        .map(|(_, lists)| lists);
    let [rest, ready, group, ended] = match kept_lists {
        Some(lists) => lists,
        None => {
            let lists = rest_of_ledger(ui, page, snapshot, live, kept, indent, &small);
            ui.ctx()
                .data_mut(|data| data.insert_temp(kept_id, (key, lists)));
            lists
        }
    };
    let facts = in_facts.max(rest);
    let column = |actions: f32, owner: &'static str| Column {
        left,
        facts,
        actions,
        owner,
    };
    Ledger {
        in_progress: column(in_progress, "ledger-in-progress"),
        ready: column(ready, "ledger-ready"),
        groups: column(group, "ledger-groups"),
        history: column(ended, "ledger-history"),
    }
}

/// What the ledger's measure of Ready, Groups and History was made from (`ledger`).
#[derive(Clone, PartialEq)]
struct LedgerKey {
    snapshot: Uuid,
    runs: crate::design::memo::Revision,
    groups: u64,
    live: Option<Option<Uuid>>,
    minute: u64,
    body: crate::design::memo::Measure,
    small: crate::design::memo::Measure,
    indent: u32,
}

/// What the ledger's measure of In progress was made from (`ledger`).
#[derive(Clone, PartialEq)]
struct InProgressKey {
    snapshot: Uuid,
    runs: crate::design::memo::Revision,
    /// The open run's words as its row reads them (`open_run_words`): the open run's facts are
    /// made on each frame from the run itself, with no revision.
    open: Option<u64>,
    story: (crate::design::memo::Revision, u64),
    minute: u64,
    body: crate::design::memo::Measure,
    small: crate::design::memo::Measure,
}

/// The open run's facts that its In progress row reads (its name, file, words, state and save
/// time), as one number: a few short words hashed, never the run list.
fn open_run_words(run: &SavedRunInfo) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::hash::DefaultHasher::new();
    (
        run.id,
        &run.encounter,
        &run.path,
        &run.label,
        run.standing == crate::encounter_run::Standing::Ended,
        run.saved_at,
    )
        .hash(&mut hasher);
    hasher.finish()
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// How many times the ledger's lists were measured (tests, #589): In progress, and Ready,
    /// Groups and History together.
    pub(super) static LEDGER_MEASURED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// Ready's, Groups' and History's widest facts, and each list's widest action recipe:
/// `[facts, ready, groups, history]`.
fn rest_of_ledger(
    ui: &egui::Ui,
    page: &history::Arranged<'_>,
    snapshot: &Snapshot,
    live: Option<&SavedRunInfo>,
    kept: &mut crate::difficulty::Kept<Uuid>,
    indent: f32,
    small: &egui::FontId,
) -> [f32; 4] {
    #[cfg(any(test, feature = "test-support"))]
    LEDGER_MEASURED.with(|count| count.set(count.get() + 1));
    let mut facts = 0.0_f32;
    let mut ready = 0.0_f32;
    for row in &page.ready {
        let template = row.template;
        let (open, reason) = ready_reason(page, row, live);
        let line = template
            .summary
            .as_ref()
            .filter(|_| !row.unfinished_run)
            .and_then(|roster| {
                kept.widest(ui, &template.id, roster.revision, small, || {
                    crate::difficulty::Line::roster(&template.difficulty, &roster.parts)
                })
            });
        facts = facts.max(facts_width(
            ui,
            &template.name,
            ready_detail(template),
            false,
            line,
        ));
        ready = ready.max(ready_actions(ui, reason, open));
    }
    let mut group = 0.0_f32;
    for each in snapshot.groups.iter() {
        let width = facts_width(ui, &each.name, &groups::row_detail(each), false, None);
        facts = facts.max(indent + width);
        group = group_actions(ui);
    }
    let mut ended = 0.0_f32;
    for past in page.history.iter().flat_map(|session| &session.runs) {
        let width = facts_width(ui, &past.run.encounter, &history_detail(past), false, None);
        facts = facts.max(indent + width);
        ended = history_actions(ui);
    }
    [facts, ready, group, ended]
}

/// A Ready row's second line: its roster's words.
fn ready_detail(template: &Template) -> &str {
    template
        .summary
        .as_ref()
        .map(|roster| roster.words.as_str())
        .filter(|words| !words.is_empty())
        .unwrap_or("No participants yet")
}

/// FJ7: whether a Ready row's run is open, and the short reason its Start is blocked; the long
/// form is on Start's hover. An open run also gets its link, at the reason.
fn ready_reason(
    page: &history::Arranged<'_>,
    row: &history::ReadyRow<'_>,
    live: Option<&SavedRunInfo>,
) -> (bool, Option<&'static str>) {
    let template = row.template;
    let unfinished = page.in_progress.iter().find(|run| {
        run.source == Some(template.id) || (run.source.is_none() && run.encounter == template.name)
    });
    let open = unfinished.is_some_and(|run| live.is_some_and(|live| live.id == run.id));
    let reason = if open {
        Some(FIGHT_OPEN)
    } else if row.unfinished_run {
        Some("This fight is unfinished.")
    } else if template
        .summary
        .as_ref()
        .is_none_or(|roster| roster.words.is_empty())
    {
        Some("Add participants first.")
    } else {
        None
    };
    (open, reason)
}

/// A History row's second line: the outcome, the state, the date, and a deleted encounter.
fn history_detail(past: &history::PastRun<'_>) -> String {
    let run = past.run;
    let date = run
        .saved_at
        .map(|at| crate::encounter_run::saved_time(at, crate::hermetic::now()))
        .unwrap_or_else(|| "Date unknown".to_owned());
    let outcome = run
        .label
        .split(" · Ended")
        .next()
        .filter(|_| !run.label.starts_with("Ended"));
    format!(
        "{}{} · {date}{}",
        outcome.map_or_else(String::new, |outcome| format!("{outcome} · ")),
        run_state_word(run, false),
        if past.template.is_none() {
            " · Encounter deleted"
        } else {
            ""
        }
    )
}

/// A row of the ledger: its facts in the ledger's facts column, and its actions (`own` wide,
/// drawn from right to left in the existing order) in its list's action column.
#[allow(clippy::too_many_arguments)]
pub(super) fn list_row(
    ui: &mut egui::Ui,
    id: impl std::hash::Hash,
    name: &str,
    detail: &str,
    status: bool,
    line: Option<ReadyLine<'_>>,
    column: Column,
    own: f32,
    actions: impl FnOnce(&mut egui::Ui),
) {
    ui.push_id(id, |ui| {
        let height = Density::Scan.row() + design::S;
        // The gate's record names the row's own facts width (its gap is measured from them).
        #[cfg(any(test, feature = "test-support"))]
        let (whole, line) = {
            let mut line = line;
            let line_width = line.as_mut().and_then(|line| ready_line_width(ui, line));
            (facts_width(ui, name, detail, status, line_width), line)
        };
        let row = ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), height),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                let rect = ui.max_rect();
                ui.set_min_size(rect.size());
                let room = rect.width().max(0.0);
                let widest = (column.left + column.facts - rect.left()).max(0.0);
                let columns = design::Columns::measure(room, [widest], [column.actions]);
                let right = rect.left() + columns.width(room);
                // Where the list does not fit, the row keeps its old narrow geometry: the
                // actions at the far edge, and `L` and the layout's item spacing before them.
                let gap = if columns.wrapped {
                    design::L + ui.spacing().item_spacing.x
                } else {
                    design::L
                };
                let facts = (right - rect.left() - gap - own).clamp(0.0, widest);
                let start = rect.left() + facts + gap;
                let mut actions_ui = ui.new_child(
                    egui::UiBuilder::new()
                        .id_salt("actions")
                        .max_rect(egui::Rect::from_min_max(
                            egui::pos2(start, rect.top()),
                            egui::pos2(right.max(start), rect.bottom()),
                        ))
                        .layout(egui::Layout::right_to_left(egui::Align::Center)),
                );
                actions(&mut actions_ui);
                let text = Type::Strong.line() + Type::Small.line();
                let facts_rect = egui::Rect::from_min_size(
                    egui::pos2(rect.left(), rect.center().y - text / 2.0),
                    egui::vec2(facts, text),
                );
                let mut facts_ui = ui.new_child(
                    egui::UiBuilder::new()
                        .id_salt("facts")
                        .max_rect(facts_rect)
                        .layout(egui::Layout::top_down(egui::Align::Min)),
                );
                row_facts(&mut facts_ui, name, detail, status, line, facts);
                #[cfg(any(test, feature = "test-support"))]
                design::action_rows::keep(
                    ui,
                    design::action_rows::RowRecord {
                        owner: column.owner,
                        row: 0,
                        facts: egui::Rect::from_min_size(
                            facts_rect.min,
                            egui::vec2(whole.min(facts), text),
                        ),
                        line: egui::Rect::from_min_size(
                            facts_rect.min,
                            egui::vec2(whole.min(facts), text),
                        ),
                        action: actions_ui.min_rect(),
                        column: start,
                        edge: rect.right(),
                        wrapped: columns.wrapped,
                    },
                );
            },
        );
        // The row registers its rect, so its second line is known as a row detail, not help.
        design::register_row(ui, row.response.rect);
    });
}

/// A row's name and its second line in `room`.
fn row_facts(
    ui: &mut egui::Ui,
    name: &str,
    detail: &str,
    status: bool,
    line: Option<ReadyLine<'_>>,
    room: f32,
) {
    ui.spacing_mut().item_spacing.y = 0.0;
    ui.set_width(room);
    ui.add(egui::Label::new(Type::Strong.rich(ui, name).color(colors().text)).truncate())
        .on_hover_text(name);
    // The run icon, and a leading warning's icon after it.
    let room = room - icon_room(status, detail);
    // F1, FB2, FB6: the difficulty ends the line in its one form. N7, FG1:
    // `difficulty::Line` decides what gives way: the word's limit first,
    // then the detail from its end by whole facts and members, then the XP
    // number. Hover and the accessible name keep the whole line (V7).
    let font = Type::Small.font(ui);
    let difficulty = line.as_ref().map(|line| line.difficulty);
    // G118: the Ready row's whole line is kept with its roster; other rows
    // say their detail and form.
    let kept_hover = line.as_ref().map(|line| line.roster.hover.as_str());
    let ready = line.is_some();
    let fitted = line.and_then(|line| {
        let ReadyLine {
            id,
            difficulty,
            roster,
            kept,
        } = line;
        // Where the fight has no form, the kept answer is the words alone,
        // so no frame fits the summary again.
        kept.answer(ui, &id, roster.revision, room, &font, || {
            crate::difficulty::Line::roster(difficulty, &roster.parts)
                .fit(|text| design::number_width(ui, text, &font) <= room)
                .or_else(|| {
                    Some(crate::difficulty::Fitted {
                        lead: fit_detail(ui, detail, &font, room),
                        form: Vec::new(),
                        cut: false,
                        wrap: false,
                    })
                })
        })
    });
    let (visible_detail, parts) = match fitted {
        Some(fitted) if ready && fitted.form.is_empty() => (fitted.lead, None),
        Some(fitted) => (fitted.lead, Some(fitted.form)),
        None => (fit_detail(ui, detail, &font, room), None),
    };
    let whole = row_whole(kept_hover, detail, difficulty);
    let warning = visible_detail
        .split(" · ")
        .position(|part| matches!(part, "Changed on disk" | "Offered in no fight"));
    let mut job = egui::text::LayoutJob::default();
    for (index, part) in visible_detail.split(" · ").enumerate() {
        if index > 0 {
            job.append(
                " · ",
                0.0,
                egui::TextFormat {
                    font_id: Type::Small.font(ui),
                    color: colors().muted,
                    ..Default::default()
                },
            );
        }
        // FJ6: a run row's line two always starts with the run icon,
        // also when its first part is a warning.
        if index == 0 && status && warning == Some(0) {
            job.append(
                &format!("{} ", icons::RUN),
                0.0,
                egui::TextFormat {
                    font_id: Type::Small.font(ui),
                    color: colors().muted,
                    ..Default::default()
                },
            );
        }
        let words = if warning == Some(index) {
            format!("{} {part}", icons::WARN)
        } else if index == 0 && status {
            format!("{} {part}", icons::RUN)
        } else {
            part.to_owned()
        };
        job.append(
            &words,
            0.0,
            egui::TextFormat {
                font_id: Type::Small.font(ui),
                color: if warning == Some(index) {
                    colors().result.warning
                } else {
                    colors().muted
                },
                ..Default::default()
            },
        );
    }
    let Some(parts) = parts else {
        let response = ui
            .add(egui::Label::new(job).extend())
            .on_hover_text(&*whole);
        design::row_detail(ui, &response);
        design::named(ui, &response, &format!("{name} · {whole}"));
        return;
    };
    crate::difficulty::append(&mut job, ui, Type::Small, " · ", colors().muted);
    crate::difficulty::append_form(&mut job, ui, Type::Small, &parts);
    let response = ui.add(egui::Label::new(job).extend());
    design::row_detail(ui, &response);
    design::named(ui, &response, &format!("{name} · {whole}"));
    let response = if visible_detail == detail {
        response
    } else {
        response.on_hover_text(&*whole)
    };
    if let Some(rating) = difficulty.and_then(|d| d.rating.as_ref()) {
        crate::difficulty::describe(ui, response, rating);
    }
}

/// A row's whole second line, for its hover and its accessible name: a Ready row's kept hover,
/// else the shown detail and the form. An empty kept hover (a roster with no participants)
/// gives way to the shown detail (`No participants yet`), so the status is never lost.
fn row_whole<'a>(
    kept: Option<&'a str>,
    detail: &'a str,
    difficulty: Option<&crate::difficulty::Difficulty>,
) -> std::borrow::Cow<'a, str> {
    match kept.filter(|hover| !hover.is_empty()) {
        Some(hover) => hover.into(),
        None => match difficulty.and_then(crate::difficulty::Difficulty::form) {
            Some(form) => format!("{detail} · {form}").into(),
            None => detail.into(),
        },
    }
}

/// A row's second line in `width`. A paused run's last change gives up its words first, by whole
/// words, so that the save time after it stays whole. With no room for one of its words, the last
/// change goes. Hover keeps the full line.
fn fit_detail(ui: &egui::Ui, detail: &str, font: &egui::FontId, width: f32) -> String {
    let parts = detail.split(" · ").collect::<Vec<_>>();
    let last = parts
        .iter()
        .position(|part| part.starts_with(crate::encounter_run::LAST_CHANGE));
    let Some(at) = last.filter(|_| design::number_width(ui, detail, font) > width) else {
        return design::words_that_fit(ui, detail, font, width);
    };
    let mut others = parts.clone();
    others.remove(at);
    let room = width - design::number_width(ui, &format!("{} · ", others.join(" · ")), font);
    let change = design::words_that_fit(ui, parts[at], font, room);
    let has_words = change.trim_end_matches('…').trim() != crate::encounter_run::LAST_CHANGE.trim();
    if has_words && design::number_width(ui, &change, font) <= room {
        others.insert(at, &change);
    }
    design::words_that_fit(ui, &others.join(" · "), font, width)
}

pub use splat_session_records::encounters::run_state_word;

fn row_menu_for_run(
    ui: &mut egui::Ui,
    run: &SavedRunInfo,
    template: Option<&Template>,
    open: bool,
    blocked: Option<&str>,
    loading: bool,
    action: &mut Option<ListAction>,
) {
    row_menu(ui, &run.encounter, |ui| {
        if let Some(template) = template
            && menu_action(ui, "Start another fight", blocked)
        {
            *action = Some(ListAction::Run(template.path.clone()));
        }
        if !open
            && menu_action(
                ui,
                "Delete the saved run",
                loading.then_some("Still reading this campaign."),
            )
        {
            *action = Some(ListAction::Delete(run.path.clone()));
        }
    });
}

/// A duplicate name uses its session first, and has only one save time.
fn paused_run_detail(
    run: &SavedRunInfo,
    runs: &[&SavedRunInfo],
    open: bool,
    story: &Story<'_>,
) -> String {
    let session_of = |run: &SavedRunInfo| {
        story
            .events
            .iter()
            .find(|event| event.run == run.id && run.id.is_some())
            .and_then(|event| event.session)
            .and_then(|id| story.sessions.iter().find(|session| session.id == id))
    };
    let duplicates = runs
        .iter()
        .filter(|other| other.encounter == run.encounter)
        .collect::<Vec<_>>();
    let session = session_of(run);
    let mut parts = if open {
        Vec::new()
    } else {
        run.label
            .trim_end_matches('.')
            .split(" · ")
            .filter(|part| !part.is_empty() && !part.starts_with("Saved "))
            .map(str::to_owned)
            .collect::<Vec<_>>()
    };
    let needs_date = duplicates.len() > 1
        && (session.is_none()
            || duplicates.iter().any(|other| {
                other.path != run.path
                    && session_of(other).map(|session| session.id)
                        == session.map(|session| session.id)
            }));
    if duplicates.len() > 1
        && let Some(session) = session
    {
        parts.push(session.label.clone());
    }
    if needs_date {
        let time = run
            .saved_at
            .map(|at| {
                let same_day = duplicates.iter().any(|other| {
                    other.path != run.path
                        && other
                            .saved_at
                            .is_some_and(|other| other / 86_400 == at / 86_400)
                });
                if same_day {
                    splat_party::party::calendar_time(at)
                } else {
                    crate::encounter_run::calendar_date(at)
                }
            })
            .unwrap_or_else(|| "Save time unknown".into());
        parts.push(time);
    } else if let Some(at) = run.saved_at {
        parts.push(format!(
            "Saved {}",
            crate::encounter_run::saved_time(at, crate::hermetic::now())
        ));
    } else if open {
        parts.push("Save time unknown".into());
    }
    parts.join(" · ")
}

#[allow(clippy::too_many_arguments)]
pub(super) fn in_progress_list(
    ui: &mut egui::Ui,
    page: &history::Arranged<'_>,
    templates: &[Template],
    live: Option<&SavedRunInfo>,
    availability: (Option<&'static str>, bool),
    story: &Story<'_>,
    column: Column,
    action: &mut Option<ListAction>,
) {
    let (blocked, loading) = availability;
    in_progress_heading(ui, page);
    // AB1, R41: while the run list cannot be read, the list says why in the fault's words
    // (`cannot find` against `cannot read`); the footer names the folder and offers Choose folder.
    if let Some(fault) = page.runs_unread {
        blocked_reason(ui, Some(fault.list_words(false)));
    } else if page.in_progress.is_empty() {
        blocked_reason(
            ui,
            Some("No unfinished runs. Start a fight from Ready to run."),
        );
    }
    for (index, run) in page.in_progress.iter().enumerate() {
        let open = live.is_some_and(|live| live.id == run.id);
        let template = history::template_of(run, templates);
        let detail = paused_run_detail(run, &page.in_progress, open, story);
        let word = run_state_word(run, false);
        let own = in_progress_actions(ui, open.then_some(word.as_str()));
        list_row(
            ui,
            &run.path,
            &run.encounter,
            &detail,
            !open,
            None,
            column,
            own,
            |ui| {
                row_menu_for_run(ui, run, template, open, blocked, loading, action);
                let edit_width = edit_slot_width(ui);
                ui.allocate_ui_with_layout(
                    egui::vec2(edit_width, Density::Read.control()),
                    egui::Layout::right_to_left(egui::Align::Center),
                    |ui| {
                        ui.set_min_width(edit_width);
                        if let Some(template) = template
                            && Action::quiet("Edit")
                                .density(Density::Read)
                                .show(ui)
                                .clicked()
                        {
                            *action = Some(ListAction::Edit(template.path.clone()));
                        }
                    },
                );
                if open {
                    if design::status_link(
                        ui,
                        Status {
                            icon: icons::RUN,
                            word: &word,
                            color: colors().muted,
                        },
                    )
                    .clicked()
                    {
                        *action = Some(ListAction::BackToRun);
                    }
                } else {
                    let button = if index == 0 && live.is_none() {
                        Action::primary("Resume")
                    } else {
                        Action::secondary("Resume")
                    };
                    if button
                        .icon(icons::RUN)
                        .density(Density::Read)
                        .blocked(blocked)
                        .show(ui)
                        .clicked()
                    {
                        *action = Some(ListAction::Resume(run.path.clone()));
                    }
                }
            },
        );
    }
    design::gap(ui, design::Gap::L);
}

/// Whether a run of this encounter is saved: unfinished, ended, or in a file that cannot be read
/// but names it. Only an encounter with none can be removed.
fn has_saved_run(page: &history::Arranged<'_>, row: &history::ReadyRow<'_>) -> bool {
    let template = row.template;
    row.unfinished_run
        || page
            .history
            .iter()
            .flat_map(|session| &session.runs)
            .any(|past| {
                past.template
                    .is_some_and(|other| other.path == template.path)
            })
        || page.unreadable.iter().any(|run| {
            run.source == Some(template.id) || run.encounter.eq_ignore_ascii_case(&template.name)
        })
}

pub(super) fn ready_list(
    ui: &mut egui::Ui,
    page: &history::Arranged<'_>,
    live: Option<&SavedRunInfo>,
    blocked: Option<&'static str>,
    kept: &mut crate::difficulty::Kept<Uuid>,
    column: Column,
    action: &mut Option<ListAction>,
) {
    heading(ui, "READY TO RUN", page.ready.len());
    blocked_reason(ui, blocked);
    if page.ready.is_empty() {
        blocked_reason(ui, Some("None ready. Select New encounter."));
    }
    for (index, row) in page.ready.iter().enumerate() {
        let template = row.template;
        let (open, short_reason) = ready_reason(page, row, live);
        let start_blocked = if row.unfinished_run {
            Some(if open {
                "This encounter has an open run. Go back to the run."
            } else {
                "This encounter has an unfinished run. Resume it in In progress."
            })
        } else if template
            .summary
            .as_ref()
            .is_none_or(|roster| roster.words.is_empty())
        {
            Some("Add participants first.")
        } else {
            blocked
        };
        let own = ready_actions(ui, short_reason, open);
        list_row(
            ui,
            &template.path,
            &template.name,
            ready_detail(template),
            false,
            // A fight with an unfinished run is not ready, so it has no word (F1).
            template
                .summary
                .as_ref()
                .filter(|_| !row.unfinished_run)
                .map(|roster| ReadyLine {
                    id: template.id,
                    difficulty: &template.difficulty,
                    roster,
                    kept: &mut *kept,
                }),
            column,
            own,
            |ui| {
                row_menu(ui, &template.name, |ui| {
                    let remove = !has_saved_run(page, row);
                    if remove {
                        menu_has_icons(ui);
                    }
                    if menu_action(ui, "Edit encounter", None) {
                        *action = Some(ListAction::Edit(template.path.clone()));
                    }
                    if remove && menu_action(ui, "Remove", None) {
                        *action = Some(ListAction::RemoveEncounter(template.path.clone()));
                    }
                });
                if Action::quiet("Edit")
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                {
                    *action = Some(ListAction::Edit(template.path.clone()));
                }
                let button = if index == 0 && page.in_progress.is_empty() && live.is_none() {
                    Action::primary(START_FIGHT)
                } else {
                    Action::secondary(START_FIGHT)
                };
                if button
                    .icon(icons::RUN)
                    .density(Density::Read)
                    .blocked(start_blocked)
                    .show(ui)
                    .clicked()
                {
                    *action = Some(ListAction::Run(template.path.clone()));
                }
                if let Some(reason) = short_reason {
                    if open && design::small_link(ui, GO_TO_RUN).clicked() {
                        *action = Some(ListAction::BackToRun);
                    }
                    ui.add(
                        egui::Label::new(Type::Small.rich(ui, reason).color(colors().muted))
                            .extend(),
                    );
                    design::gap(ui, design::Gap::L);
                }
            },
        );
    }
}

pub(super) fn group_list(
    ui: &mut egui::Ui,
    snapshot: &Snapshot,
    blocked: Option<&'static str>,
    column: Column,
    action: &mut Option<ListAction>,
) {
    if snapshot.groups.is_empty() {
        heading(ui, "GROUPS", 0);
        blocked_reason(ui, Some("No groups yet. Select New group."));
    } else {
        section(
            ui,
            "encounter_groups",
            "GROUPS",
            snapshot.groups.len(),
            None,
            |ui| {
                groups::list(ui, snapshot, blocked, column, action);
            },
        );
    }
}

/// The ended run read in History.
pub(super) struct Reading<'a> {
    pub path: &'a Path,
    pub summary: &'a FightSummary,
    /// Why it cannot go on, when it is not the fight that was played last.
    pub not_last: Option<&'static str>,
}

/// History. Returns, when a run is open there, the screen span from the opened run's clean top
/// edge to the end of its Final state: what `Open` brings into view (R2).
pub(super) fn history_list(
    ui: &mut egui::Ui,
    page: &history::Arranged<'_>,
    blocked: Option<&'static str>,
    live: Option<&SavedRunInfo>,
    read: Option<Reading<'_>>,
    column: Column,
    action: &mut Option<ListAction>,
) -> Option<egui::Rangef> {
    design::gap(ui, design::Gap::L);
    if let Some(fault) = page.runs_unread {
        heading(ui, "HISTORY", 0);
        blocked_reason(ui, Some(fault.list_words(true)));
        return None;
    }
    if page.ended_count() == 0 && page.unreadable.is_empty() {
        heading(ui, "HISTORY", 0);
        blocked_reason(ui, Some("No ended fights yet."));
        return None;
    }
    // The opened run: the top of its row's gap above it, to the end of its Final state (R2).
    let mut opened = None;
    section(
        ui,
        "encounter_history",
        "HISTORY",
        page.ended_count(),
        read.as_ref().map(|_| true),
        |ui| {
            for session in &page.history {
                design::heading(
                    ui,
                    design::Heading::Group {
                        title: session.label,
                        count: session.runs.len(),
                    },
                );
                for past in &session.runs {
                    let run = past.run;
                    let detail = history_detail(past);
                    let top = ui.cursor().top();
                    let own = history_actions(ui);
                    list_row(
                        ui,
                        &run.path,
                        &run.encounter,
                        &detail,
                        false,
                        None,
                        column,
                        own,
                        |ui| {
                            row_menu(ui, &run.encounter, |ui| {
                                if menu_action(
                                    ui,
                                    "Delete the saved run",
                                    live.filter(|live| live.id == run.id)
                                        .map(|_| "Close this run before deleting its save."),
                                ) {
                                    *action = Some(ListAction::Delete(run.path.clone()));
                                }
                                if let Some(template) = past.template {
                                    let start_blocked = blocked.or(template
                                        .summary
                                        .as_ref()
                                        .is_none_or(|roster| roster.words.is_empty())
                                        .then_some("Add participants first."));
                                    if menu_action(ui, START_FIGHT, start_blocked) {
                                        *action = Some(ListAction::Run(template.path.clone()));
                                    }
                                    if menu_action(ui, "Edit", None) {
                                        *action = Some(ListAction::Edit(template.path.clone()));
                                    }
                                }
                            });
                            let opened = read.as_ref().is_some_and(|read| read.path == run.path);
                            if Action::quiet(if opened { "Close" } else { "Open" })
                                .density(Density::Read)
                                .name(&format!(
                                    "{} the run of {}, {}",
                                    if opened { "Close" } else { "Open" },
                                    run.encounter,
                                    run.label
                                ))
                                .show(ui)
                                .clicked()
                            {
                                *action = Some(ListAction::ReadHistory(run.path.clone()));
                            }
                        },
                    );
                    if let Some(read) = &read
                        && read.path == run.path
                    {
                        // The open run is this fight, so nothing stands in its way.
                        let here = live.is_some_and(|live| live.id == run.id);
                        // Continue fight is for the fight that was played last.
                        // A deleted encounter cannot start a new run, so its reason gives only the cause.
                        let not_last = read.not_last.map(|reason| {
                            crate::encounter_run::continue_reason(reason, past.template.is_some())
                        });
                        let (go_on, final_end) =
                            history_summary(ui, read.summary, blocked.filter(|_| !here), not_last);
                        if go_on {
                            *action = Some(ListAction::ContinueFight(run.path.clone()));
                        }
                        // FH4: the open run ends with a gap, so the next row does not read as
                        // a line of its log.
                        design::gap(ui, design::Gap::L);
                        // The top of the row's own gap above it: the line above ends there.
                        opened = Some(egui::Rangef::new(
                            top - ui.spacing().item_spacing.y,
                            final_end,
                        ));
                    }
                }
            }
            if !page.unreadable.is_empty() {
                design::heading(
                    ui,
                    design::Heading::Label(&format!("Unreadable {}", page.unreadable.len())),
                );
                for run in &page.unreadable {
                    blocked_reason(ui, Some(&run.label));
                }
            }
        },
    );
    opened
}

/// An ended fight, read in place. Returns true when the DM chooses to go on with it, and the
/// screen y where its Final state ends. `not_last` says why this fight cannot go on, when it is
/// not the fight that was played last: then its sentence stands where the control would be.
fn history_summary(
    ui: &mut egui::Ui,
    summary: &FightSummary,
    blocked: Option<&str>,
    not_last: Option<&str>,
) -> (bool, f32) {
    let mut go_on = false;
    let mut final_end = ui.cursor().top();
    ui.indent("history_summary", |ui| {
        // The line under Read-only is `S` below it and `M` above Final state, so it reads as
        // the run's own line and not as part of Final state.
        // The line is as tall as its words, so the gap below it is the gap that shows. A
        // wrapped row takes its height from the spacing when it starts.
        ui.scope(|ui| {
            ui.spacing_mut().interact_size.y = Type::Small.line();
            ui.horizontal_wrapped(|ui| {
                design::status(
                    ui,
                    Status {
                        icon: icons::READ_ONLY,
                        word: "Read-only",
                        color: colors().document.read_only,
                    },
                );
                ui.label(
                    Type::Small
                        .rich(ui, &summary.end_reason)
                        .color(colors().muted),
                );
            });
        });
        design::gap_to_next(ui, design::Gap::S);
        // The one way on from a fight that ended by mistake. Undo puts the ending back.
        match not_last {
            Some(reason) => blocked_reason(ui, Some(reason)),
            None => {
                go_on = crate::encounter_run::view::continue_action()
                    .density(Density::Read)
                    .blocked(blocked)
                    .show(ui)
                    .clicked();
            }
        }
        // The heading part owns the space above it (`L`, more than the `XS` under it).
        design::heading(ui, design::Heading::Label("Final state"));
        // AI1, FI6, SI1: shared columns. The name column is as wide as the longest name, up to
        // half the room (a longer name wraps); then HP and temporary HP, each as wide as its
        // widest value, kept where a row has none; then Absent and the conditions, which wrap
        // inside the last column. Every value sits on its name's baseline.
        let strong = Type::Strong.font(ui);
        let small = Type::Small.font(ui);
        let names = summary
            .rows
            .iter()
            .map(|row| design::number_width(ui, &row.name, &strong))
            .fold(0.0, f32::max)
            .min(ui.available_width() / 2.0);
        let hp_words = |row: &crate::encounter_run::FightRow| {
            let hp = row.hp.map_or_else(|| "?".to_owned(), |hp| hp.to_string());
            let max = row
                .max_hp
                .map_or_else(|| "?".to_owned(), |hp| hp.to_string());
            format!("HP {}", crate::combat_sync::fraction(hp, max))
        };
        let temp_words = |row: &crate::encounter_run::FightRow| {
            (row.temporary_hp > 0).then(|| format!("{} +{}", icons::TEMP_HP, row.temporary_hp))
        };
        let hp_width = summary
            .rows
            .iter()
            .map(|row| design::number_width(ui, &hp_words(row), &small))
            .fold(0.0, f32::max);
        let temp_width = summary
            .rows
            .iter()
            .filter_map(&temp_words)
            .map(|words| design::number_width(ui, &words, &small))
            .fold(0.0, f32::max);
        // A fixed cell: its words at the cell's left, the cell as wide as its column.
        let cell = |ui: &mut egui::Ui, width: f32, words: Option<egui::RichText>| {
            ui.allocate_ui_with_layout(
                egui::vec2(width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    ui.set_width(width);
                    if let Some(words) = words {
                        ui.add(egui::Label::new(words).extend());
                    }
                },
            );
        };
        for row in &summary.rows {
            ui.horizontal_top(|ui| {
                ui.allocate_ui_with_layout(
                    egui::vec2(names, 0.0),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        ui.set_width(names);
                        ui.add(
                            egui::Label::new(Type::Strong.rich(ui, &row.name).color(colors().text))
                                .wrap(),
                        );
                    },
                );
                design::gap_to_next(ui, design::Gap::M);
                // The values are one line of cells, each a top-down block, so they share the
                // name's first line (a wrapping flow put them 4 px below it).
                ui.vertical(|ui| {
                    ui.horizontal_top(|ui| {
                        let hp = Type::Small.rich(ui, hp_words(row)).color(colors().text);
                        cell(ui, hp_width, Some(hp));
                        if temp_width > 0.0 {
                            let temp = temp_words(row)
                                .map(|words| Type::Small.rich(ui, words).color(colors().hp.temp));
                            cell(ui, temp_width, temp);
                        }
                        // The last column: one paragraph that wraps inside it, on the line's
                        // baseline.
                        if let Some(rest) = final_state_rest(ui, row) {
                            let width = ui.available_width();
                            ui.allocate_ui_with_layout(
                                egui::vec2(width, 0.0),
                                egui::Layout::top_down(egui::Align::Min),
                                |ui| ui.add(egui::Label::new(rest).wrap()),
                            );
                        }
                    });
                });
            });
        }
        final_end = ui.min_rect().bottom();
        // X66: the log's heading has the gap that Final state's has: the heading part's.
        design::heading(ui, design::Heading::Label("Combat log"));
        for line in &summary.log {
            blocked_reason(ui, Some(line));
        }
    });
    (go_on, final_end)
}

/// A Final state row's last column: the HP state's icon and word in its colour, as the run's
/// rows show it (FK5: `0 HP`, `Bloodied`), Absent, the saved concentration in the mark that the
/// run's rows use (X70), then the conditions. `None` when the row has none of them.
fn final_state_rest(
    ui: &egui::Ui,
    row: &crate::encounter_run::FightRow,
) -> Option<egui::text::LayoutJob> {
    let conditions = row
        .conditions
        .iter()
        .map(|c| c.words())
        .collect::<Vec<_>>()
        .join(", ");
    let muted = colors().muted;
    let parts = [
        design::hp_word(row.hp, row.max_hp)
            .map(|(icon, word, color)| (format!("{icon}\u{a0}{word}"), color)),
        (!row.present).then(|| ("Absent".to_owned(), muted)),
        row.concentration
            .then(crate::encounter_run::view::concentration_mark),
        (!conditions.is_empty()).then_some((conditions, muted)),
    ];
    let mut job = egui::text::LayoutJob::default();
    for (words, color) in parts.into_iter().flatten() {
        if !job.is_empty() {
            crate::difficulty::append(&mut job, ui, Type::Small, " · ", muted);
        }
        crate::difficulty::append(&mut job, ui, Type::Small, &words, color);
    }
    (!job.is_empty()).then_some(job)
}

#[cfg(test)]
mod iter_2_c_tests {
    use super::*;
    use crate::timeline::{Book, Kind, SessionLink};
    use std::path::PathBuf;

    /// Review of PR 613 (P2): the other side's field keeps the words as typed while it has the
    /// keyboard, so Ctrl+Z gives back the side it replaced; when it lets go, it shows the
    /// side's stored form.
    #[test]
    fn the_other_side_field_keeps_its_undo_while_typing() {
        use crate::design::test_harness::Themed as _;
        use egui_kittest::kittest::Queryable;
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(600.0, 200.0))
            .themed_ui_state(
                |ui, (side, shown): &mut (String, bool)| {
                    let id = ui.make_persistent_id(("Side", "other_side"));
                    ui.data_mut(|data| data.insert_temp(id, true));
                    if !*shown {
                        ui.label("Elsewhere");
                        return;
                    }
                    side_control(
                        ui,
                        (
                            SIDE_CHOICE.of(uuid::Uuid::nil()),
                            SIDE_OTHER.of(uuid::Uuid::nil()),
                        ),
                        "Side",
                        side,
                        &[],
                    );
                    ui.label("Elsewhere");
                },
                ("rivals".to_owned(), true),
            );
        harness.run_steps(2);
        let field = |harness: &egui_kittest::Harness<'_, (String, bool)>| {
            harness
                .get_by_role_and_label(egui::accesskit::Role::TextInput, "Other side for Side")
                .value()
                .unwrap_or_default()
        };
        assert_eq!(field(&harness), "Rivals");
        harness
            .get_by_role_and_label(egui::accesskit::Role::TextInput, "Other side for Side")
            .focus();
        harness.run_steps(2);
        let key = |key, modifiers| egui::Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers,
        };
        harness.input_mut().events.extend([
            key(egui::Key::A, egui::Modifiers::COMMAND),
            egui::Event::Text("friends".into()),
        ]);
        harness.run_steps(3);
        assert_eq!(harness.state().0, "friends");
        assert_eq!(
            field(&harness),
            "friends",
            "the words stay as typed while typing"
        );
        harness
            .input_mut()
            .events
            .push(key(egui::Key::Z, egui::Modifiers::COMMAND));
        harness.run_steps(3);
        assert_eq!(
            field(&harness),
            "Rivals",
            "Ctrl+Z gives back the side it replaced"
        );
        assert_eq!(harness.state().0, "rivals");
        // When the field lets go, it shows the side's stored form.
        harness
            .input_mut()
            .events
            .push(egui::Event::Text("x".into()));
        harness.run_steps(2);
        harness.get_by_label("Elsewhere").click();
        harness
            .input_mut()
            .events
            .push(key(egui::Key::Escape, egui::Modifiers::NONE));
        harness.run_steps(3);
        assert_eq!(harness.state().0, "rivalsx");
        assert_eq!(field(&harness), "Rivalsx");
        // Review of PR 695 (P1): the held words belong to the side they were typed over. A side
        // changed from outside while the field keeps the keyboard (Undo of a discard, pressed
        // through accessibility) shows in the field, and the next letter goes after it.
        let focus = |harness: &mut egui_kittest::Harness<'_, (String, bool)>| {
            harness
                .get_by_role_and_label(egui::accesskit::Role::TextInput, "Other side for Side")
                .focus();
            harness.run_steps(2);
        };
        focus(&mut harness);
        assert_eq!(field(&harness), "Rivalsx");
        harness.state_mut().0 = "friends".into();
        harness.run_steps(2);
        assert_eq!(
            field(&harness),
            "Friends",
            "the outside change shows in the field"
        );
        harness.input_mut().events.extend([
            key(egui::Key::End, egui::Modifiers::NONE),
            egui::Event::Text("x".into()),
        ]);
        harness.run_steps(2);
        assert_eq!(
            harness.state().0,
            "friendsx",
            "typing goes after the side as it is"
        );
        // P2: a field not drawn in a frame loses its held words; drawn again, it shows the
        // stored form, not the words typed before it went.
        harness.input_mut().events.extend([
            key(egui::Key::A, egui::Modifiers::COMMAND),
            egui::Event::Text("foes".into()),
        ]);
        harness.run_steps(2);
        assert_eq!(field(&harness), "foes");
        harness.state_mut().1 = false;
        harness.run_steps(2);
        harness.state_mut().1 = true;
        harness.step();
        assert_eq!(
            field(&harness),
            "Foes",
            "a hidden field’s held words are gone"
        );
        assert_eq!(harness.state().0, "foes");
    }

    /// F1, FB2, FB6: a Ready row's line ends with the difficulty in its one form at every width;
    /// the detail gives way first, by whole words, and a line with room keeps it whole.
    #[test]
    fn the_difficulty_form_never_gives_way_to_the_detail() {
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let rating = crate::difficulty::above_high();
        let rating = crate::difficulty::Difficulty {
            xp: rating.xp,
            rating: Some(rating),
            unknown: 0,
            missing: None,
        };
        // The Ready row's roster as `roster_summary` gives it: typed parts (G118).
        let roster = |members: &[(usize, &str)]| crate::difficulty::Summary {
            party: 5,
            members: members
                .iter()
                // The fixture's creatures with no XP.
                .map(|&(count, name)| {
                    crate::difficulty::Member::new(
                        count,
                        name,
                        ["Nameless Thing", "Wisp"].contains(&name),
                    )
                })
                .collect(),
            facts: Vec::new(),
        };
        let summary = roster(&[(1, "Cinder Tyrant"), (2, "Ember Cultist")]);
        let detail = &summary.text();
        assert_eq!(detail, "5 party · 1 × Cinder Tyrant, 2 × Ember Cultist");
        // The counts in `design::count`'s grouped form, as the summary always said them.
        assert_eq!(roster(&[(999, "Wolf")]).text(), "5 party · 999 × Wolf");
        assert_eq!(roster(&[(1000, "Wolf")]).text(), "5 party · 1,000 × Wolf");
        // P3 (final review of PR 390): an encounter with no participants keeps its status in
        // its hover and accessible name.
        let empty = super::super::history::Roster::new(Default::default(), &Default::default());
        assert_eq!(
            row_whole(Some(&empty.hover), "No participants yet", None),
            "No participants yet"
        );
        let widths: Vec<f32> = (200..=520).step_by(4).map(|width| width as f32).collect();
        let lines = |rating: &crate::difficulty::Difficulty,
                     summary: &crate::difficulty::Summary,
                     ends: &str| {
            let roster = super::super::history::Roster::new(summary.clone(), rating);
            let detail = roster.words.clone();
            let mut kept = crate::difficulty::Kept::default();
            let mut lines = Vec::new();
            for width in &widths {
                let output = ctx.run(Default::default(), |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.scope(|ui| {
                            ui.set_max_width(*width);
                            let line = ReadyLine {
                                id: Uuid::nil(),
                                difficulty: rating,
                                roster: &roster,
                                kept: &mut kept,
                            };
                            let column = Column {
                                left: 0.0,
                                facts: f32::INFINITY,
                                actions: 0.0,
                                owner: "test",
                            };
                            list_row(
                                ui,
                                "row",
                                "Mill Road",
                                &detail,
                                false,
                                Some(line),
                                column,
                                0.0,
                                |_| {},
                            );
                        });
                    });
                });
                let line = output.shapes.iter().find_map(|shape| match &shape.shape {
                    egui::Shape::Text(text) if text.galley.job.text.contains(ends) => {
                        Some(text.galley.job.text.clone())
                    }
                    _ => None,
                });
                lines.push(line.unwrap());
            }
            lines
        };
        let rated = lines(&rating, &summary, "Above High");
        const FORM: &str = " · XP 6,300 · Above High";
        // Q-s (iter-29 G120): the word never gives way; `5 party` goes first, then the XP number,
        // then the creatures from the end. So a line that lost a creature lost those two too.
        for line in &rated {
            assert!(line.ends_with("Above High"), "{line:?}");
            if !line.contains("Cinder Tyrant") || !line.contains("Ember Cultist") {
                assert!(
                    !line.contains("XP 6,300") && !line.contains("5 party"),
                    "{line:?}"
                );
            }
            // What is left of the summary is the summary's own start, in order: whole, or cut
            // with `…` where it gave way, never corrupt partial words.
            let shown = line
                .strip_suffix(" · Above High")
                .map(|rest| rest.strip_suffix(" · XP 6,300").unwrap_or(rest))
                .unwrap_or("");
            let shown = shown.strip_prefix("5 party · ").unwrap_or(shown);
            let tail = detail.strip_prefix("5 party · ").unwrap();
            assert!(
                shown.is_empty()
                    || shown == tail
                    || shown.strip_suffix('…').is_some_and(|cut| {
                        tail.starts_with(cut.trim_end_matches([' ', '·', ',']))
                    }),
                "{line:?}"
            );
        }
        // The detail is whole where it fits, and gives way where it does not.
        let whole = format!("{detail}{FORM}");
        assert!(
            rated.contains(&whole) && rated.iter().any(|line| *line != whole),
            "{rated:?}"
        );
        // G118 (AB4, FB5), ruling R31: a creature with no XP that the summary shows alone is
        // named once: the line ends with `XP not known`. Only where the summary gave way so
        // that it no longer shows the creature do the names follow `XP not known`.
        let nameless = crate::difficulty::Difficulty {
            unknown: 1,
            missing: Some(crate::difficulty::Missing {
                what: crate::difficulty::Unknown::Xp,
                at: vec![0],
                names: vec!["Nameless Thing".into()],
            }),
            ..Default::default()
        };
        let unknown = lines(&nameless, &roster(&[(1, "Nameless Thing")]), "XP not known");
        let mut named_after = 0;
        for line in &unknown {
            assert!(line.matches("Nameless Thing").count() <= 1, "{line:?}");
            if line.contains("1 × Nameless Thing") {
                assert!(line.ends_with(" · XP not known"), "{line:?}");
            } else {
                assert!(line.contains("XP not known: Na"), "{line:?}");
                named_after += 1;
            }
        }
        // The sweep reaches the width where the summary gave way and the name follows.
        assert!(named_after > 0, "{unknown:?}");
        assert!(
            unknown.contains(&"5 party · 1 × Nameless Thing · XP not known".to_owned()),
            "{unknown:?}"
        );
        // R31: two creatures that both lack their XP: the summary says whose. Beside a creature
        // with a known XP, the reason keeps the name: it says whose input is missing.
        let missing = |names: &[&str]| crate::difficulty::Difficulty {
            unknown: names.len() as u64,
            missing: Some(crate::difficulty::Missing {
                what: crate::difficulty::Unknown::Xp,
                at: (0..names.len()).collect(),
                names: names.iter().map(|name| (*name).to_owned()).collect(),
            }),
            ..Default::default()
        };
        let two = lines(
            &missing(&["Nameless Thing", "Wisp"]),
            &roster(&[(1, "Nameless Thing"), (1, "Wisp")]),
            "XP not known",
        );
        assert!(
            two.contains(&"5 party · 1 × Nameless Thing, 1 × Wisp · XP not known".to_owned()),
            "{two:?}"
        );
        let mixed = lines(
            &nameless,
            &roster(&[(1, "Wolf"), (1, "Nameless Thing")]),
            "XP not known",
        );
        assert!(
            mixed.contains(
                &"5 party · 1 × Wolf, 1 × Nameless Thing · XP not known: Nameless Thing".to_owned()
            ),
            "{mixed:?}"
        );
        // R31 by the member, never by a name (final review of PR 390): Nameless Thing renamed
        // `Wolf` beside a known Wolf; both are called `Wolf`. A known creature is shown, so the
        // reason keeps the name; where the unknown one gave way, the reason names it.
        let twins = crate::difficulty::Summary {
            party: 5,
            members: vec![
                crate::difficulty::Member::new(1, "Wolf", false),
                crate::difficulty::Member::renamed(1, "Wolf", "Nameless Thing", true),
            ],
            facts: Vec::new(),
        };
        let twins = lines(&missing(&["Wolf"]), &twins, "XP not known");
        for line in &twins {
            if line.contains(" × ") {
                assert!(line.contains("XP not known: W"), "{line:?}");
            }
        }
        assert!(
            twins.contains(
                &"5 party · 1 × Wolf, 1 × Wolf (as Nameless Thing) · XP not known: Wolf".to_owned()
            ) && twins.contains(&"1 × Wolf, … · XP not known: Wolf".to_owned()),
            "{twins:?}"
        );
    }

    /// Astra review of PR 231, on the menu part since round 28: a row menu opens below or above
    /// its button, never beside it, and stays inside the pane it opens from, one scroll lane
    /// clear of the pane's right edge (a splitter or a scroll bar): at the pane's right edge
    /// near the window's bottom, and near the pane's left edge.
    #[test]
    fn iter22_b_row_menu_opens_below_or_above_inside_its_pane() {
        use crate::design::test_harness::ThemedPage as _;
        use egui::{pos2, vec2};
        use egui_kittest::kittest::Queryable;
        let pane = egui::Rect::from_min_max(pos2(150.0, 90.0), pos2(590.0, 632.0));
        let size = vec2(24.0, 28.0);
        for (case, button) in [
            (
                // A row's `⋯` ends at its list's scroll lane, as every row's action does.
                "right edge, near the bottom",
                egui::Rect::from_min_size(pos2(548.0, 600.0), size),
            ),
            (
                "left edge, near the top",
                egui::Rect::from_min_size(pos2(160.0, 100.0), size),
            ),
        ] {
            let mut harness = egui_kittest::Harness::builder()
                .with_size(vec2(960.0, 640.0))
                .themed(move |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.scope_builder(egui::UiBuilder::new().max_rect(pane), |ui| {
                            ui.set_clip_rect(pane);
                            ui.scope_builder(egui::UiBuilder::new().max_rect(button), |ui| {
                                row_menu(ui, "Bridge Ambush", |ui| {
                                    let _ = menu_action(ui, "Edit encounter", None);
                                    let _ = menu_action(ui, "Delete the saved run", None);
                                });
                            });
                        });
                    });
                });
            harness.run_steps(2);
            harness
                .get_by_label("More actions for Bridge Ambush")
                .click();
            harness.run_steps(4);
            let shown = harness
                .get_by_label("More actions for Bridge Ambush")
                .rect();
            let lane = design::lane_width().max(design::S);
            let rows: Vec<egui::Rect> = ["Edit encounter", "Delete the saved run"]
                .iter()
                .map(|name| {
                    harness
                        .query_all_by_label(name)
                        .map(|node| node.rect())
                        .reduce(|a, b| a.union(b))
                        .unwrap_or(egui::Rect::NOTHING)
                })
                .collect();
            let inside = rows.iter().all(|row| {
                row.is_positive()
                    && row.left() >= pane.left() + lane - 0.5
                    && row.right() <= pane.right() - lane + 0.5
                    && (row.bottom() <= shown.top() + 0.5 || row.top() >= shown.bottom() - 0.5)
            });
            assert!(
                inside,
                "{case}: the menu’s rows {rows:?} lie below or above the button {shown:?}, inside the pane {pane:?} less a lane of {lane}"
            );
        }
        // M0259 (M-P0.25): when the pane shrinks under an open row menu, the pane's own column
        // brings the menu's button back with the least scroll (`reveal::bring_into_view`), with
        // no press: its bottom comes to the view's bottom, not its top to the view's top.
        let mut harness = egui_kittest::Harness::builder()
            .with_size(vec2(400.0, 600.0))
            .themed(|ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    design::ScrollColumn::new("rows").show(ui, |ui| {
                        let _ = ui.allocate_exact_size(vec2(1.0, 400.0), egui::Sense::hover());
                        row_menu(ui, "Bridge Ambush", |ui| {
                            let _ = menu_action(ui, "Edit encounter", None);
                        });
                        let _ = ui.allocate_exact_size(vec2(1.0, 600.0), egui::Sense::hover());
                    });
                });
            });
        harness.run_steps(2);
        harness
            .get_by_label("More actions for Bridge Ambush")
            .click();
        harness.run_steps(4);
        let before = harness
            .get_by_label("More actions for Bridge Ambush")
            .rect();
        harness.set_size(vec2(400.0, 300.0));
        harness.run_steps(8);
        let after = harness
            .get_by_label("More actions for Bridge Ambush")
            .rect();
        assert!(
            before.top() > 300.0
                && after.top() >= 0.0
                && after.bottom() <= 300.0
                && after.top() > 150.0,
            "m0259-least-scroll: shrunk under the open menu, its button came back from {before:?} \
             to {after:?}, at the view's bottom"
        );
        // M-P0.26: a reason under a menu choice that wraps, at 960 in a narrow pane. It starts at
        // the choice's words (after its icon slot), ends `S` before the menu's edge, and its words are never justified (the menu's
        // own layout is justified; `blocked_reason` lays its words out in a column that is not).
        const REASON: &str =
            "This saved run is open in another window. Close it there first, then delete it.";
        const MENU_LINE: &str =
            "Runs that are open elsewhere wait for you to close them before they can change.";
        let mut harness = egui_kittest::Harness::builder()
            .with_size(vec2(960.0, 640.0))
            .themed(|ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    // A 300 px pane, as a narrow list column: the menu's room, so the reason
                    // must wrap.
                    let pane = egui::Rect::from_min_max(pos2(300.0, 40.0), pos2(600.0, 600.0));
                    ui.scope_builder(egui::UiBuilder::new().max_rect(pane), |ui| {
                        ui.set_clip_rect(pane);
                        row_menu(ui, "Bridge Ambush", |ui| {
                            let _ = menu_action(ui, "Edit encounter", None);
                            let _ = menu_action(ui, "Delete the saved run", Some(REASON));
                            // A reason line straight in the menu's justified layout.
                            blocked_reason(ui, Some(MENU_LINE));
                        });
                    });
                });
            });
        harness.run_steps(2);
        harness
            .get_by_label("More actions for Bridge Ambush")
            .click();
        harness.run_steps(4);
        // Where each text is painted: its galley and its origin, also inside shape lists.
        fn find(
            shape: &egui::Shape,
            words: &str,
        ) -> Option<(egui::Pos2, std::sync::Arc<egui::Galley>)> {
            match shape {
                egui::Shape::Text(text) if text.galley.job.text == words => {
                    Some((text.pos, text.galley.clone()))
                }
                egui::Shape::Vec(shapes) => shapes.iter().find_map(|shape| find(shape, words)),
                _ => None,
            }
        }
        let output = harness.output();
        let painted = |words: &str| {
            output
                .shapes
                .iter()
                .find_map(|clipped| find(&clipped.shape, words))
                .unwrap_or_else(|| panic!("{words:?} is painted"))
        };
        let (_, line) = painted(MENU_LINE);
        assert!(
            line.rows.len() >= 2 && !line.job.justify,
            "m-p0-blocked-reason: a wrapped reason line in a justified menu is not justified \
             ({} rows, justified {})",
            line.rows.len(),
            line.job.justify
        );
        // Review of PR 780: the note's first letter against the choice's own painted words, as
        // both are painted (an independent reference, not the part's inset again).
        let (at, galley) = painted(REASON);
        let (choice_at, choice) = painted("Delete the saved run");
        let note_x = at.x + galley.rows[0].rect().left();
        let words_x = choice_at.x + choice.rows[0].rect().left();
        let menu_right = harness.get_by_label("Edit encounter").rect().right();
        assert!(
            galley.rows.len() >= 2
                && !galley.job.justify
                && (note_x - words_x).abs() <= 1.0
                && at.x + galley.size().x <= menu_right - design::S + 0.5,
            "m-p0-choice-note: at 960 the reason wraps ({} rows), unjustified ({}), its words at \
             the painted words of the choice {words_x} (they start at {note_x}), ending S before \
             the edge of the menu {menu_right} ({})",
            galley.rows.len(),
            !galley.job.justify,
            at.x + galley.size().x
        );
    }

    /// T12.12: Remove is offered only for an encounter with no saved run: none unfinished,
    /// none ended, and no unreadable run that names it.
    #[test]
    fn remove_is_offered_only_for_an_encounter_with_no_saved_run() {
        use crate::encounter_run::Standing;
        let template = |name: &str| Template {
            path: format!("Encounters/{name}.md"),
            id: Uuid::new_v4(),
            name: name.into(),
            summary: Some(super::super::history::Roster::new(
                crate::difficulty::Summary {
                    members: vec![crate::difficulty::Member::new(1, "Wolf", false)],
                    ..Default::default()
                },
                &Default::default(),
            )),
            difficulty: Default::default(),
            repeatable: true,
        };
        let templates = ["Fresh", "Paused", "Ended", "Unreadable"].map(template);
        let run = |at: usize, standing| SavedRunInfo {
            path: PathBuf::from(format!("run-{at}.json")),
            id: Some(Uuid::new_v4()),
            source: Some(templates[at].id),
            encounter: templates[at].name.clone(),
            label: String::new(),
            standing,
            setup: false,
            saved_at: None,
            unsent: vec![],
        };
        let mut unreadable = run(3, Standing::Unreadable);
        unreadable.source = None;
        let runs = [
            run(1, Standing::Unfinished),
            run(2, Standing::Ended),
            unreadable,
        ];
        let page = history::arrange(&templates, &runs, None, &[], &[]);
        let offered: Vec<_> = page
            .ready
            .iter()
            .filter(|row| !has_saved_run(&page, row))
            .map(|row| row.template.name.as_str())
            .collect();
        assert_eq!(offered, ["Fresh"]);
    }

    #[test]
    fn paused_duplicates_use_session_then_one_save_time() {
        let runs = [1, 2].map(|n| SavedRunInfo {
            path: PathBuf::from(format!("run-{n}.json")),
            id: Some(Uuid::from_u128(n)),
            source: None,
            encounter: "Bridge Ambush".into(),
            label: "Setup · Paused · Last: Added a wolf · Saved just now".into(),
            standing: crate::encounter_run::Standing::Unfinished,
            setup: true,
            saved_at: Some(1_767_229_200 + n as u64 * 60),
            unsent: vec![],
        });
        let refs = runs.iter().collect::<Vec<_>>();
        let sessions = [1, 2].map(|n| SessionLink {
            id: Uuid::from_u128(n),
            label: format!("Session {n}"),
            mentions: vec![],
        });
        let mut book = Book::default();
        for (run, session) in runs.iter().zip(&sessions) {
            book.add(Kind::Encounter, Some(session.id));
            book.events.last_mut().unwrap().run = run.id;
        }
        let detail = paused_run_detail(
            &runs[0],
            &refs,
            false,
            &Story {
                events: &book.events,
                sessions: &sessions,
                revision: None,
            },
        );
        assert!(detail.contains("Session 1"));
        assert_eq!(detail.matches("Saved ").count(), 1);
        assert!(!detail.contains("2026-01-01"));
        assert!(detail.contains("Last: Added a wolf"));
        book.events[1].session = Some(sessions[0].id);
        let detail = paused_run_detail(
            &runs[0],
            &refs,
            false,
            &Story {
                events: &book.events,
                sessions: &sessions,
                revision: None,
            },
        );
        assert!(detail.contains("Session 1 · 2026-01-01"));
        assert!(!detail.contains("Saved "));
        let detail = paused_run_detail(
            &runs[0],
            &refs,
            false,
            &Story {
                events: &[],
                sessions: &[],
                revision: None,
            },
        );
        assert_eq!(detail.matches("2026-01-01").count(), 1);
        assert!(!detail.contains("Saved "));
        let detail = paused_run_detail(
            &runs[0],
            &[&runs[0]],
            false,
            &Story {
                events: &[],
                sessions: &[],
                revision: None,
            },
        );
        assert_eq!(detail.matches("Saved ").count(), 1);
        assert!(!detail.contains("2026-01-01"));
    }

    /// FT2: a paused copy's line gives up its last change before its save time.
    #[test]
    fn a_paused_copy_shortens_its_last_change_and_keeps_its_save_time() {
        let context = egui::Context::default();
        design::theme::install(&context);
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| check(ui));
        });
        fn check(ui: &egui::Ui) {
            let font = Type::Small.font(ui);
            let width = |text: &str| design::number_width(ui, text, &font);
            assert!(width("Last: 1") > 0.0, "the test measures text");
            let detail = "Copy 2 · Round 2 · Paused · Last: 1 damage to Kestrel Dunmoor (6 HP left) · 2026-01-01 03:00";
            // It fits: nothing changes.
            assert_eq!(fit_detail(ui, detail, &font, width(detail)), detail);
            // Room for some of the last change: it loses whole words, and the time stays whole.
            let shortened =
                "Copy 2 · Round 2 · Paused · Last: 1 damage to Kestrel… · 2026-01-01 03:00";
            let fitted = fit_detail(ui, detail, &font, width(shortened));
            assert!(fitted.ends_with(" · 2026-01-01 03:00"), "{fitted}");
            assert!(fitted.contains(" · Last: 1 damage to Kestrel…"), "{fitted}");
            assert!(width(&fitted) <= width(shortened));
            // Room for no word of it: the last change goes, and the time stays whole.
            let without = "Copy 2 · Round 2 · Paused · 2026-01-01 03:00";
            let fitted = fit_detail(
                ui,
                detail,
                &font,
                width(without) + width(" · Last: …") / 2.0,
            );
            assert_eq!(fitted, without);
            // A line with a warning keeps that part whole too.
            let changed = "Changed on disk · Round 2 · Paused · Last: 3 damage to Kestrel Dunmoor (8 HP left) · 2026-01-01 03:00";
            let fitted = fit_detail(ui, changed, &font, width(changed) - 1.0);
            assert!(
                fitted.starts_with("Changed on disk · Round 2 · Paused · Last: 3"),
                "{fitted}"
            );
            assert!(fitted.ends_with(" · 2026-01-01 03:00"), "{fitted}");
            // A line without a last change still shortens from its end.
            let history = "Victory · Round 4 · 2026-01-01 03:00";
            let fitted = fit_detail(ui, history, &font, width("Victory · Round 4 · 2026-01-01"));
            assert_eq!(
                fitted,
                design::words_that_fit(ui, history, &font, width("Victory · Round 4 · 2026-01-01"))
            );
            assert!(fitted.ends_with('…'));
        }
    }
}
