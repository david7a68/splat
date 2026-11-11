//! Rolls in a stat block or a sheet. Its actions are tiles: in a run a tile arms its action
//! against the roster's rows; outside a run it previews. Numbers in tables (an ability, a
//! save, a skill) also roll. A right-click on a tile or a number chooses how its d20 rolls. A
//! roll changes nothing: in a run it goes to the Log, and its damage goes into Amount.
use crate::design::{self, Action, Density, Type, colors};
use eframe::egui::{self, Id, text::LayoutJob};
use splat::content::dice_text::{self, D20, DamagePart, RollLink};
use splat::execution::dice::{Dice, Random, Roll};
use splat::memo::Memo;
use std::sync::Arc;

/// How a d20 or a damage roll is made.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Mode {
    Normal,
    Advantage,
    Disadvantage,
    /// Damage dice twice over, for a critical hit.
    Critical,
}

impl Mode {
    /// Shift for advantage, Ctrl for disadvantage, on a d20.
    fn from_keys(ui: &egui::Ui, link: &RollLink) -> Self {
        let d20 = matches!(link, RollLink::D20 { .. });
        let keys = ui.input(|i| i.modifiers);
        if d20 && keys.shift {
            Self::Advantage
        } else if d20 && keys.command {
            Self::Disadvantage
        } else {
            Self::Normal
        }
    }
}

/// A press on a rollable number, waiting for whoever owns the dice.
#[derive(Clone, Debug)]
pub(crate) struct Ask {
    pub link: RollLink,
    pub mode: Mode,
    /// The run row whose stat block or sheet was drawn, if any.
    pub roller: Option<usize>,
    /// The entry or character the number is in.
    pub entry: String,
    /// The roller expends inspiration for advantage, by a rule that gives it before the roll.
    pub inspired: bool,
}

fn ask_id() -> Id {
    Id::new("roll_ask")
}
fn roller_id() -> Id {
    Id::new("roll_roller")
}
fn entry_id() -> Id {
    Id::new("roll_entry")
}
fn inspired_id() -> Id {
    Id::new("roll_inspired")
}
fn preview_id() -> Id {
    Id::new("roll_preview_random")
}
fn outcome_id(entry: &str) -> Id {
    Id::new(("roll_preview_outcome", entry))
}

/// The press of this frame, for the run or the preview stream.
pub(crate) fn take_ask(ctx: &egui::Context) -> Option<Ask> {
    ctx.data_mut(|data| {
        let ask = data.get_temp::<Ask>(ask_id());
        data.remove::<Ask>(ask_id());
        ask
    })
}

/// Who rolls from what is drawn next: a run row, or nobody. Set it around a run's reader.
pub(crate) fn set_roller(ctx: &egui::Context, roller: Option<usize>) {
    ctx.data_mut(|data| match roller {
        Some(row) => data.insert_temp(roller_id(), row),
        None => data.remove::<usize>(roller_id()),
    });
}

/// Whether the roller drawn next can expend inspiration for advantage before a roll. Set it
/// around a run's reader, by the campaign's inspiration rule and the character's sheet.
pub(crate) fn set_inspired(ctx: &egui::Context, inspired: bool) {
    ctx.data_mut(|data| data.insert_temp(inspired_id(), inspired));
}

/// The entry whose numbers are drawn next, to name its rolls.
pub(crate) fn set_entry(ctx: &egui::Context, name: &str) {
    ctx.data_mut(|data| data.insert_temp(entry_id(), name.to_owned()));
}

/// Ask for `link`, from the roller and entry set for what is drawn now.
fn ask(ctx: &egui::Context, link: RollLink, mode: Mode, inspired: bool) {
    let roller = ctx.data(|data| data.get_temp::<usize>(roller_id()));
    let entry = ctx
        .data(|data| data.get_temp::<String>(entry_id()))
        .unwrap_or_default();
    ctx.data_mut(|data| {
        data.insert_temp(
            ask_id(),
            Ask {
                link,
                mode,
                roller,
                entry,
                inspired,
            },
        );
    });
}

/// How a right-click on a d20 roll can roll it: the mode, and whether inspiration pays for
/// advantage. `verb` names the press: `Roll`, or `Roll for them`.
pub(crate) fn mode_choices(verb: &str, inspired: bool) -> Vec<(String, Mode, bool)> {
    let mut choices = vec![
        (verb.to_owned(), Mode::Normal, false),
        (format!("{verb} with advantage"), Mode::Advantage, false),
        (
            format!("{verb} with disadvantage"),
            Mode::Disadvantage,
            false,
        ),
    ];
    if inspired {
        choices.push((
            format!("{verb} with inspiration (advantage)"),
            Mode::Advantage,
            true,
        ));
    }
    choices
}

/// An action's name: `Bite`, `Ember Burst`.
pub(crate) fn link_label(link: &RollLink) -> &str {
    match link {
        RollLink::D20 { label, .. }
        | RollLink::Damage { label, .. }
        | RollLink::SaveDc { label, .. }
        | RollLink::Dice { label, .. } => label,
    }
}

/// Words in a menu: a heading or a question. A menu's layout is justified, which would stretch
/// the gaps of a wrapped line, so the words get a left-aligned layout of their own.
pub(crate) fn menu_text(ui: &mut egui::Ui, text: &str, color: egui::Color32) -> egui::Response {
    ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
        ui.add(egui::Label::new(Type::Small.rich(ui, text).color(color)).wrap())
    })
    .inner
}

/// One line of a menu under a heading or a question: its words start at the heading's text.
/// In a menu the keys opened, the first choice takes the focus.
pub(crate) fn menu_choice(ui: &mut egui::Ui, words: &str) -> bool {
    let response = Action::quiet(words)
        .density(Density::Read)
        .line_start()
        .show(ui);
    if ui
        .ctx()
        .data_mut(|data| data.remove_temp::<bool>(focus_first_choice_id()))
        .unwrap_or(false)
    {
        response.request_focus();
    }
    if response.has_focus() {
        let frame = ui.ctx().cumulative_frame_nr();
        ui.ctx().data_mut(|data| {
            data.insert_temp(menu_choice_focus_id(), response.id);
            data.insert_temp(menu_choice_focus_frame_id(), frame);
        });
    }
    response.clicked()
}

/// The frame in which a menu choice last had the keyboard focus.
fn menu_choice_focus_frame_id() -> egui::Id {
    egui::Id::new("roll_menu_focused_choice_frame")
}

/// The menu choice that had the keyboard focus last.
fn menu_choice_focus_id() -> egui::Id {
    egui::Id::new("roll_menu_focused_choice")
}

fn focus_first_choice_id() -> egui::Id {
    egui::Id::new("roll_menu_focus_first_choice")
}

fn menu_keys_owner_id() -> egui::Id {
    egui::Id::new("roll_menu_opened_by_keys")
}

/// At the end of each frame: when a menu the keys opened has closed, by any path (a choice, the
/// last choice going away, Escape, a click outside, or its control gone), it ends here, and the
/// focus goes back to the control that owned it. A click outside that gave the focus to another
/// control keeps it there. When the DM moves the focus out of the open menu (Tab to another
/// control or a field), the return is cancelled at once, so a later Escape never takes the
/// focus back from where the DM put it. The menu stays open, as a menu does when it loses the
/// focus. Run after everything that draws a menu.
pub(crate) fn end_menu_frame(ctx: &egui::Context) {
    let Some(owner) = ctx.data(|data| data.get_temp::<egui::Id>(menu_keys_owner_id())) else {
        return;
    };
    let focused = ctx.memory(|memory| memory.focused());
    let clear = |ctx: &egui::Context| {
        ctx.data_mut(|data| {
            data.remove::<egui::Id>(menu_keys_owner_id());
            data.remove::<bool>(focus_first_choice_id());
            data.remove::<egui::Id>(menu_choice_focus_id());
            data.remove::<u64>(menu_choice_focus_frame_id());
        });
    };
    if egui::Popup::is_id_open(ctx, owner.with("popup")) {
        let in_menu = ctx.data(|data| data.get_temp::<u64>(menu_choice_focus_frame_id()))
            == Some(ctx.cumulative_frame_nr());
        if focused.is_some_and(|focused| focused != owner) && !in_menu {
            clear(ctx);
        }
        return;
    }
    let choice = ctx.data(|data| data.get_temp::<egui::Id>(menu_choice_focus_id()));
    clear(ctx);
    if focused.is_none() || focused == choice {
        ctx.memory_mut(|memory| memory.request_focus(owner));
    }
}

/// Whether a right-click menu opens or closes this frame. A right-click opens it; so do
/// Shift+F10 on the focused control and a screen reader's `show context menu`, which put the
/// focus on its first choice and give it back to the control when the menu closes. A plain
/// click or Enter on the control closes it.
pub(crate) fn menu_open_command(response: &egui::Response) -> Option<egui::SetOpenCommand> {
    use egui::accesskit::Action as Access;
    let ctx = &response.ctx;
    ctx.accesskit_node_builder(response.id, |node| node.add_action(Access::ShowContextMenu));
    let by_keys = ctx
        .input(|i| i.has_accesskit_action_request(response.id, Access::ShowContextMenu))
        || (response.has_focus()
            && ctx.input_mut(|i| i.consume_key(egui::Modifiers::SHIFT, egui::Key::F10)));
    if by_keys {
        ctx.data_mut(|data| {
            data.insert_temp(menu_keys_owner_id(), response.id);
            data.insert_temp(focus_first_choice_id(), true);
        });
        return Some(egui::SetOpenCommand::Bool(true));
    }
    if response.secondary_clicked() {
        Some(egui::SetOpenCommand::Bool(true))
    } else if response.clicked() {
        Some(egui::SetOpenCommand::Bool(false))
    } else {
        None
    }
}

/// The heading of a d20 roll's menu: the action's name, `Bite`, or what the number rolls,
/// `Dexterity save`.
pub(crate) fn menu_heading(link: &RollLink) -> Option<String> {
    match link {
        RollLink::D20 {
            what: D20::Attack,
            label,
            ..
        } => Some(label.clone()),
        RollLink::D20 { what, label, .. } => Some(d20_words(*what, label)),
        _ => None,
    }
}

/// The right-click menu on a d20 roll, as a row's menu is: under the tile or number, so it
/// stays in view, headed in muted Small with what it rolls, and its choices start at the
/// heading's words. It stays inside its pane, whose right edge is `right`. Returns how to
/// roll it, when one was chosen.
fn mode_menu(response: &egui::Response, link: &RollLink, right: f32) -> Option<(Mode, bool)> {
    let heading = menu_heading(link)?;
    let inspired = response
        .ctx
        .data(|data| data.get_temp::<bool>(inspired_id()))
        .unwrap_or(false);
    let open = menu_open_command(response);
    // As wide as its tile, and never past the pane's right edge, which `right` is: a menu
    // that ran past the window would be pushed left, over the panel's splitter.
    let margin = egui::Frame::popup(&response.ctx.style())
        .total_margin()
        .sum()
        .x;
    let width = (response.rect.width().max(TILE_WIDTH) - margin)
        .min(right - response.rect.left() - margin)
        .max(1.0);
    let mut chosen = None;
    egui::Popup::menu(response)
        .open_memory(open)
        .width(width)
        .show(|ui| {
            // The area keeps the size it had; the content sets it each frame.
            ui.set_width(width);
            menu_text(ui, &heading, colors().muted);
            for (words, mode, inspired) in mode_choices("Roll", inspired) {
                if menu_choice(ui, &words) {
                    chosen = Some((mode, inspired));
                    ui.close();
                }
            }
        });
    chosen
}

/// What a press on this number does, in words, for hover.
pub(crate) fn hint(link: &RollLink) -> String {
    let keys = " Shift: advantage. Ctrl: disadvantage. Right-click or Shift+F10 for more.";
    match link {
        RollLink::D20 {
            what, bonus, label, ..
        } => {
            let what = match what {
                D20::Attack => format!("{label} attack and damage"),
                _ => d20_words(*what, label),
            };
            format!("Roll 1d20 {}: {what}.{keys}", signed_spaced(*bonus))
        }
        RollLink::Damage { label, parts } => {
            format!("Roll {}: {label} damage.", parts_text(parts))
        }
        RollLink::Dice { dice, .. } => format!("Roll {dice}."),
        RollLink::SaveDc {
            label, ability, dc, ..
        } => format!("{label}: roll its damage and each creature’s DC {dc} {ability} save."),
    }
}

fn signed_spaced(value: i64) -> String {
    if value < 0 {
        format!("\u{2212} {}", -value)
    } else {
        format!("+ {value}")
    }
}

/// What a d20 roll is for, in words: `to hit`, `Dexterity save`, `Stealth`.
fn d20_words(what: D20, label: &str) -> String {
    match what {
        D20::Attack => "to hit".to_owned(),
        D20::Check => format!("{label} check"),
        D20::Save => format!("{label} save"),
        D20::Skill => label.to_owned(),
        D20::DeathSave => "death save".to_owned(),
    }
}

/// `2d6+2 bludgeoning`: one damage part's dice and type.
fn part_text(part: &DamagePart) -> String {
    match &part.kind {
        Some(kind) => format!("{} {}", part.dice, kind.to_lowercase()),
        None => part.dice.clone(),
    }
}

/// `2d6+2 bludgeoning, 1d6 fire`: every part of a damage roll. A part with a condition
/// follows as `or 4d6+4 piercing` or `plus 1d4 slashing`; its condition is in the text.
pub(crate) fn parts_text(parts: &[DamagePart]) -> String {
    parts
        .iter()
        .enumerate()
        .map(|(at, part)| match &part.when {
            // With nothing before it, a part has nothing to replace or to add to.
            Some(when) if at > 0 && when.instead => format!("or {}", part_text(part)),
            Some(_) if at > 0 => format!("plus {}", part_text(part)),
            _ => part_text(part),
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// A number in a table that rolls: an ability's modifier, a save, a skill, a sheet's bonus.
/// Accent text with a quiet underline, at least 24 px each way.
pub(crate) fn link(ui: &mut egui::Ui, text: &str, role: Type, link: &RollLink) -> egui::Response {
    let c = colors();
    // Laid out without a colour, so the hover colour below paints the words too.
    let galley = design::one_line(
        ui,
        text,
        role.font(ui),
        egui::Color32::PLACEHOLDER,
        ui.available_width(),
    );
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(
            galley.size().x.max(24.0),
            galley.size().y.max(role.line()).max(24.0),
        ),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Link, ui.is_enabled(), hint(link))
    });
    if ui.is_rect_visible(rect) {
        let top = rect.center().y - galley.size().y / 2.0;
        // A short number sits in the middle of its target, so the words beside it keep even gaps.
        let left = rect.center().x - galley.size().x / 2.0;
        let color = if response.hovered() {
            c.focus
        } else {
            c.accent
        };
        ui.painter().hline(
            left..=left + galley.size().x,
            top + galley.size().y - 1.0,
            egui::Stroke::new(1.0, color.gamma_multiply(0.6)),
        );
        ui.painter().galley(egui::pos2(left, top), galley, color);
    }
    interact(ui, &response, link);
    response
}

/// Hover words and a press with its keys, on a drawn number that rolls.
pub(crate) fn interact(ui: &egui::Ui, response: &egui::Response, link: &RollLink) {
    press(ui, response, link);
    if !response.hovered() {
        return;
    }
    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    response.clone().on_hover_text(hint(link));
}

/// A click asks for `link` with the keys held; a choice in its right-click menu asks its way.
fn press(ui: &egui::Ui, response: &egui::Response, link: &RollLink) {
    if response.clicked() {
        ask(ui.ctx(), link.clone(), Mode::from_keys(ui, link), false);
    }
    if let Some((mode, inspired)) = mode_menu(response, link, ui.clip_rect().right()) {
        ask(ui.ctx(), link.clone(), mode, inspired);
    }
}

/// One group of dice in a result: a label, each face and whether it was dropped.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Piece {
    pub label: String,
    pub faces: Vec<(i64, bool)>,
    /// The dice rolled, such as `2d20kh1` or `2d6+2`, so that one die can roll again.
    pub dice: String,
}

/// One die of an outcome: in its damage or its first roll, in which piece, at which place.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Die {
    pub damage: bool,
    pub piece: usize,
    pub face: usize,
}

/// A made roll, with what the action line and the Log say about it.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Rolled {
    /// Who rolled: a participant, an entry, or the DM.
    pub who: String,
    pub link: RollLink,
    pub mode: Mode,
    pub pieces: Vec<Piece>,
    /// The flat part: a bonus, a damage bonus, or a run change.
    pub flat: i64,
    pub total: i64,
    /// Damage totals by type, for a damage roll with more than one part.
    pub by_kind: Vec<(i64, Option<String>)>,
    /// A run change in the flat part, with the printed bonus it replaced.
    pub changed_from: Option<i64>,
    /// The d20 that counted, on an attack or a death save, where a 20 or a 1 matters.
    pub natural: Option<i64>,
    /// The expression the Log names.
    pub expression: String,
}

impl Rolled {
    /// The words after the total: `to hit`, `damage`, `Dexterity save`.
    fn what(&self) -> String {
        match &self.link {
            RollLink::D20 { what, label, .. } => d20_words(*what, label),
            RollLink::Damage { .. } => "damage".to_owned(),
            RollLink::Dice { dice, .. } => dice.clone(),
            RollLink::SaveDc { .. } => String::new(),
        }
    }

    /// The feature, when the words after the total do not already name it.
    fn feature(&self) -> Option<&str> {
        match &self.link {
            RollLink::D20 {
                what: D20::Attack,
                label,
                ..
            }
            | RollLink::Damage { label, .. }
            | RollLink::Dice { label, .. } => Some(label.as_str()).filter(|l| !l.is_empty()),
            _ => None,
        }
    }

    /// The parts in order, each with whether a keep dropped it.
    fn each_part(&self, mut push: impl FnMut(&str, bool)) {
        for (at, piece) in self.pieces.iter().enumerate() {
            if at > 0 {
                push(" · ", false);
            }
            push(&format!("{} ", piece.label), false);
            for (index, (face, gone)) in piece.faces.iter().enumerate() {
                if index > 0 {
                    push(", ", false);
                }
                push(&face.to_string(), *gone);
            }
        }
        if self.flat != 0 {
            push(&format!(" {}", signed_spaced(self.flat)), false);
        }
    }

    /// The parts with dropped dice struck through, in Small type.
    fn parts_job(&self, ui: &egui::Ui, color: egui::Color32) -> LayoutJob {
        let font = Type::Small.font(ui);
        let mut job = LayoutJob::default();
        let plain = egui::TextFormat {
            font_id: font.clone(),
            color,
            ..Default::default()
        };
        let dropped = egui::TextFormat {
            strikethrough: egui::Stroke::new(1.0, color),
            ..plain.clone()
        };
        self.each_part(|text, gone| {
            let format = if gone { &dropped } else { &plain };
            job.append(text, 0.0, format.clone());
        });
        job
    }

    /// `17 to hit`, `14 damage: 7 piercing, 7 fire`.
    pub fn headline(&self) -> String {
        match self.by_kind.as_slice() {
            [(_, Some(kind))] => format!("{} {} damage", self.total, kind.to_lowercase()),
            [_] | [] => format!("{} {}", self.total, self.what()),
            by_kind => {
                let each = by_kind
                    .iter()
                    .map(|(amount, kind)| match kind {
                        Some(kind) => format!("{amount} {}", kind.to_lowercase()),
                        None => amount.to_string(),
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{} {}: {each}", self.total, self.what())
            }
        }
    }

    /// `Natural 20` or `Natural 1`, where the rules give them a meaning.
    pub fn natural_words(&self) -> Option<&'static str> {
        match self.natural {
            Some(20) => Some("Natural 20"),
            Some(1) => Some("Natural 1"),
            _ => None,
        }
    }

    fn changed_words(&self) -> Option<String> {
        self.changed_from.map(|printed| {
            format!(
                "{} (changed from {})",
                design::signed(self.flat),
                design::signed(printed)
            )
        })
    }

    /// The Log line.
    pub fn line(&self) -> String {
        let faces: Vec<i64> = self
            .pieces
            .iter()
            .flat_map(|piece| piece.faces.iter().map(|(face, _)| *face))
            .collect();
        // The dock's own roll keeps the Log's older words.
        if let RollLink::Dice { label, dice } = &self.link
            && label.is_empty()
            && self.who == "DM"
        {
            return format!("DM roll {dice}: {faces:?} = {}.", self.total);
        }
        let mode = match self.mode {
            Mode::Normal => "",
            Mode::Advantage => " with advantage",
            Mode::Disadvantage => " with disadvantage",
            Mode::Critical => " as a critical hit",
        };
        let feature = self
            .feature()
            .map_or_else(String::new, |f| format!(", {f}"));
        let natural = self
            .natural_words()
            .map_or_else(String::new, |words| format!(" {words}."));
        format!(
            "{}{feature}: {} {}{mode}: {faces:?} = {}.{natural}",
            self.who,
            self.what(),
            self.expression,
            self.total
        )
    }
}

/// Make a roll. `change` is a run's attack or damage bonus for the roller.
pub(crate) fn roll(
    who: &str,
    link: &RollLink,
    mode: Mode,
    change: i64,
    random: &mut Random,
) -> Result<Rolled, String> {
    let mut pieces = Vec::new();
    let mut by_kind = Vec::new();
    let (expression, flat, total, natural, changed_from) = match link {
        RollLink::D20 { what, bonus, .. } => {
            let dice = match mode {
                Mode::Advantage => "2d20kh1",
                Mode::Disadvantage => "2d20kl1",
                Mode::Normal | Mode::Critical => "1d20",
            };
            let rolled = Dice::parse(dice)?.roll(random, false)?;
            let faces = faces(&rolled);
            let kept = faces
                .iter()
                .find(|(_, dropped)| !dropped)
                .map(|(face, _)| *face);
            pieces.push(Piece {
                label: "d20".into(),
                faces,
                dice: dice.to_owned(),
            });
            let flat = bonus + change;
            let natural = kept.filter(|_| matches!(what, D20::Attack | D20::DeathSave));
            (
                format!("{dice} {}", signed_spaced(flat)),
                flat,
                rolled.total + flat,
                natural,
                (change != 0).then_some(*bonus),
            )
        }
        RollLink::Damage { parts, .. } => {
            let mut sum = 0i64;
            for part in parts {
                let rolled = Dice::parse(&part.dice)?.roll(random, mode == Mode::Critical)?;
                sum += rolled.total;
                by_kind.push((rolled.total, part.kind.clone()));
                pieces.push(Piece {
                    label: part_text(part),
                    faces: rolled.dice.iter().map(|face| (*face, false)).collect(),
                    dice: part.dice.clone(),
                });
            }
            let mut expression = parts_text(parts);
            if change != 0 {
                if let Some(first) = by_kind.first_mut() {
                    first.0 += change;
                }
                expression.push_str(&format!(" {}", signed_spaced(change)));
            }
            (expression, change, sum + change, None, None)
        }
        RollLink::Dice { dice, .. } => {
            let rolled = Dice::parse(dice)?.roll(random, false)?;
            pieces.push(Piece {
                label: dice.clone(),
                faces: faces(&rolled),
                dice: dice.clone(),
            });
            (dice.clone(), 0, rolled.total, None, None)
        }
        RollLink::SaveDc { .. } => return Err("A save DC is not rolled.".into()),
    };
    Ok(Rolled {
        who: who.to_owned(),
        link: link.clone(),
        mode,
        pieces,
        flat,
        total,
        by_kind: if by_kind.len() > 1 || by_kind.first().is_some_and(|(_, k)| k.is_some()) {
            by_kind
        } else {
            Vec::new()
        },
        changed_from,
        natural,
        expression,
    })
}

/// Each face of a roll, and whether a keep dropped it.
fn faces(rolled: &Roll) -> Vec<(i64, bool)> {
    rolled
        .dice
        .iter()
        .enumerate()
        .map(|(at, face)| (*face, rolled.dropped.contains(&at)))
        .collect()
}

/// `roll` with a situational bonus added to the total, as its own part.
pub(crate) fn roll_with(
    who: &str,
    link: &RollLink,
    mode: Mode,
    change: i64,
    extra: Option<&str>,
    random: &mut Random,
) -> Result<Rolled, String> {
    let mut rolled = roll(who, link, mode, change, random)?;
    if let Some(extra) = extra {
        let bonus = Dice::parse(extra)?.roll(random, false)?;
        rolled.total += bonus.total;
        let shown = extra.strip_prefix('-').map_or_else(
            || format!(" + {extra}"),
            |minus| format!(" \u{2212} {minus}"),
        );
        rolled.expression.push_str(&shown);
        if bonus.dice.is_empty() {
            rolled.flat += bonus.total;
        } else {
            rolled.pieces.push(Piece {
                label: format!("{extra} bonus"),
                faces: faces(&bonus),
                dice: extra.to_owned(),
            });
        }
    }
    Ok(rolled)
}

/// One action of a creature or a character: a feature with an attack, a save or damage.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Chip {
    /// `Bite +4`, `Ember Burst DC 12`, `Claw 2d6`.
    pub label: String,
    pub link: RollLink,
    /// The rules text, for hover.
    pub text: String,
}

impl Chip {
    pub fn new(link: RollLink, text: String) -> Option<Self> {
        let label = match &link {
            RollLink::D20 {
                what: D20::Attack,
                bonus,
                label,
                ..
            } => format!("{label} {}", design::signed(*bonus)),
            RollLink::SaveDc { label, dc, .. } => format!("{label} DC {dc}"),
            RollLink::Damage { label, parts } => format!("{label} {}", parts_text(parts)),
            _ => return None,
        };
        Some(Self { label, link, text })
    }
}

/// A creature's actions, from its rules text: each named paragraph with an attack, a save
/// DC, or damage, in the order printed. Quotes, tables and headings are not rules. This list
/// is the one order of an entry's actions: a tile's Ctrl number is its place here.
pub(crate) fn chips(text: &str) -> Vec<Chip> {
    actions(text).iter().map(|(_, chip)| chip.clone()).collect()
}

/// The actions in `text`, each with the place of its paragraph among the blank-line
/// paragraphs. Remembered by the text, so a frame that draws the same text parses nothing.
pub(crate) fn actions(text: &str) -> Arc<Vec<(usize, Chip)>> {
    static ACTIONS: Memo<Arc<Vec<(usize, Chip)>>> = Memo::new(128);
    ACTIONS.get_or(&[text.as_bytes()], || Arc::new(parse_actions(text)))
}

fn parse_actions(text: &str) -> Vec<(usize, Chip)> {
    let mut chips: Vec<(usize, Chip)> = Vec::new();
    for (at, paragraph) in text.split("\n\n").map(str::trim).enumerate() {
        // Spellcasting stays prose: it lists spells, with links to them.
        if paragraph.starts_with(['>', '|', '#', '<']) || paragraph.starts_with("Spellcasting") {
            continue;
        }
        let Some(title) = design::reader::run_in_title(paragraph) else {
            continue;
        };
        let label = title.split(" (").next().unwrap_or(title);
        let spans = dice_text::find(paragraph, label);
        let pick = |test: fn(&RollLink) -> bool| spans.iter().map(|s| &s.link).find(|l| test(l));
        let link = pick(|l| {
            matches!(
                l,
                RollLink::D20 {
                    what: D20::Attack,
                    ..
                }
            )
        })
        .or_else(|| pick(|l| matches!(l, RollLink::SaveDc { .. })))
        .or_else(|| pick(|l| matches!(l, RollLink::Damage { .. })));
        if let Some(chip) = link
            .cloned()
            .and_then(|link| Chip::new(link, paragraph.replace(['*', '_'], "")))
            && !chips.iter().any(|(_, known)| known.label == chip.label)
        {
            chips.push((at, chip));
        }
    }
    chips
}

/// The place of an action in an entry's whole list, by which Ctrl+number arms it.
pub(crate) fn place_in(all: &[(usize, Chip)], chip: &Chip) -> Option<usize> {
    all.iter()
        .position(|(_, known)| known.label == chip.label && known.link == chip.link)
}

/// One press of an action and what it rolled: an attack and its damage, a save's damage,
/// or a single d20 or dice roll.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Outcome {
    pub link: RollLink,
    /// The d20, or plain dice.
    pub first: Option<Rolled>,
    pub damage: Option<Rolled>,
}

impl Outcome {
    /// The Log lines, in the order rolled.
    pub fn lines(&self) -> Vec<String> {
        [&self.first, &self.damage]
            .into_iter()
            .flatten()
            .map(Rolled::line)
            .collect()
    }
}

/// Roll an action. An attack rolls its damage too, as a critical hit on a natural 20; a
/// save DC rolls its damage once. `changes` are a run's attack and damage changes; `extra`
/// is a bonus for the d20.
pub(crate) fn make(
    who: &str,
    link: &RollLink,
    mode: Mode,
    changes: (i64, i64),
    extra: Option<&str>,
    random: &mut Random,
) -> Result<Outcome, String> {
    let (attack_change, damage_change) = changes;
    // A condition this roll settles counts; any other is false. A run settles the rest
    // before it calls this, by the target and by asking.
    let settled = |parts: &[DamagePart]| {
        dice_text::resolve(parts, |when| condition_by_mode(when, mode).unwrap_or(false))
    };
    let (first, damage) = match link {
        RollLink::D20 {
            what: D20::Attack,
            label,
            damage,
            ..
        } => {
            let damage = &settled(damage);
            let attack = roll_with(who, link, mode, attack_change, extra, random)?;
            let critical = if attack.natural == Some(20) || mode == Mode::Critical {
                Mode::Critical
            } else {
                Mode::Normal
            };
            let damage = damage_roll(who, label, damage, critical, damage_change, random)?;
            (Some(attack), damage)
        }
        RollLink::D20 { .. } => (Some(roll_with(who, link, mode, 0, extra, random)?), None),
        RollLink::Damage { label, parts } => {
            let link = RollLink::Damage {
                label: label.clone(),
                parts: settled(parts),
            };
            (None, Some(roll(who, &link, mode, damage_change, random)?))
        }
        RollLink::Dice { .. } => (Some(roll(who, link, mode, 0, random)?), None),
        RollLink::SaveDc { label, damage, .. } => (
            None,
            damage_roll(
                who,
                label,
                &settled(damage),
                Mode::Normal,
                damage_change,
                random,
            )?,
        ),
    };
    Ok(Outcome {
        link: link.clone(),
        first,
        damage,
    })
}

/// Whether a damage condition holds, when the roll's mode settles it: `if the attack roll had
/// Advantage`. `None` when the mode does not say.
pub(crate) fn condition_by_mode(when: &dice_text::When, mode: Mode) -> Option<bool> {
    let words = when.text.to_lowercase();
    (words.contains("attack roll")
        && words.contains("advantage")
        && !words.contains("disadvantage"))
    .then_some(mode == Mode::Advantage)
}

/// A feature's damage, if it has any.
fn damage_roll(
    who: &str,
    label: &str,
    parts: &[DamagePart],
    mode: Mode,
    change: i64,
    random: &mut Random,
) -> Result<Option<Rolled>, String> {
    if parts.is_empty() {
        return Ok(None);
    }
    let link = RollLink::Damage {
        label: label.to_owned(),
        parts: parts.to_vec(),
    };
    roll(who, &link, mode, change, random).map(Some)
}

impl Outcome {
    /// The dice that inspiration can roll again, with words for each: `3 on the d20`,
    /// `1 on 2d6+2 slashing`. Equal faces of one piece are one choice, lowest first.
    pub fn dice(&self) -> Vec<(String, Die)> {
        let mut all = Vec::new();
        for (damage, rolled) in [(false, &self.first), (true, &self.damage)] {
            for (piece, part) in rolled
                .iter()
                .flat_map(|rolled| rolled.pieces.iter().enumerate())
            {
                let mut faces: Vec<(i64, usize)> = part
                    .faces
                    .iter()
                    .enumerate()
                    .map(|(at, (face, _))| (face.abs(), at))
                    .collect();
                faces.sort();
                faces.dedup_by_key(|(face, _)| *face);
                let name = if part.label == "d20" {
                    "the d20"
                } else {
                    part.label.as_str()
                };
                all.extend(faces.into_iter().map(|(value, face)| {
                    (
                        format!("{value} on {name}"),
                        Die {
                            damage,
                            piece,
                            face,
                        },
                    )
                }));
            }
        }
        all
    }
}

/// Roll one die of an outcome again; the new die stands, and its keeps and totals are made
/// again. When an attack's d20 begins or ends a critical hit, its damage rolls again too.
/// Returns the outcome, the old face and the new.
pub(crate) fn reroll(
    who: &str,
    outcome: &Outcome,
    die: Die,
    changes: (i64, i64),
    random: &mut Random,
) -> Result<(Outcome, i64, i64), String> {
    const NONE: &str = "There is no such die to reroll.";
    let mut outcome = outcome.clone();
    let rolled = if die.damage {
        outcome.damage.as_mut()
    } else {
        outcome.first.as_mut()
    }
    .ok_or(NONE)?;
    let critical = rolled.mode == Mode::Critical && matches!(rolled.link, RollLink::Damage { .. });
    let piece = rolled.pieces.get_mut(die.piece).ok_or(NONE)?;
    let before: Vec<i64> = piece.faces.iter().map(|(face, _)| *face).collect();
    let dice = Dice::parse(&piece.dice)?;
    let old = dice.settle(&before, critical)?;
    let new = dice.reroll(&before, die.face, random, critical)?;
    piece.faces = faces(&new);
    let change = new.total - old.total;
    rolled.total += change;
    if let Some(kind) = rolled.by_kind.get_mut(die.piece) {
        kind.0 += change;
    }
    let (was, now) = (old.dice[die.face].abs(), new.dice[die.face].abs());
    // The d20 that counts can be another one now.
    let flips = if !die.damage && rolled.natural.is_some() {
        let counted = piece
            .faces
            .iter()
            .find(|(_, dropped)| !dropped)
            .map(|(face, _)| *face);
        let flips = (rolled.natural == Some(20)) != (counted == Some(20));
        rolled.natural = counted;
        flips.then_some(counted == Some(20))
    } else {
        None
    };
    if let (
        Some(critical),
        RollLink::D20 {
            what: D20::Attack,
            label,
            damage,
            ..
        },
    ) = (flips, &outcome.link)
    {
        let mode = if critical {
            Mode::Critical
        } else {
            Mode::Normal
        };
        outcome.damage = damage_roll(who, label, damage, mode, changes.1, random)?;
    }
    Ok((outcome, was, now))
}

/// A preview outside a run, on its own stream, kept for the entry's action line. Tests use a
/// fixed seed.
pub(crate) fn preview(ctx: &egui::Context, ask: &Ask) {
    let mut random = ctx
        .data(|data| data.get_temp::<Random>(preview_id()))
        .unwrap_or_else(|| {
            #[cfg(test)]
            let seed = 20_261_110;
            #[cfg(not(test))]
            let seed = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(1, |since| since.as_nanos() as u64);
            Random::seeded(seed)
        });
    let outcome = make(&ask.entry, &ask.link, ask.mode, (0, 0), None, &mut random);
    ctx.data_mut(|data| {
        data.insert_temp(preview_id(), random);
        if let Ok(outcome) = outcome {
            data.insert_temp(outcome_id(&ask.entry), outcome);
        }
    });
}

/// An entry's last preview, for its action line.
fn preview_outcome(ctx: &egui::Context, entry: &str) -> Option<Outcome> {
    ctx.data(|data| data.get_temp::<Outcome>(outcome_id(entry)))
}

/// The run's armed action, for the tiles to mark it and say what can follow.
#[derive(Clone, Debug)]
pub(crate) struct ArmedView {
    pub link: RollLink,
    /// What to do next, such as `Click a target. Escape stops.`
    pub status: String,
    pub actions: Vec<(&'static str, ArmedAction)>,
}

/// What the line under an armed tile can ask the run to do.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ArmedAction {
    /// For a player's attack: roll it for them against the target.
    RollForThem,
    /// For a save: roll its damage once, into Amount.
    RollDamage,
    /// For a save: mark the targets at or above the DC to take half.
    HalfMade,
    Stop,
}

fn armed_id() -> Id {
    Id::new("roll_armed")
}
fn armed_action_id() -> Id {
    Id::new("roll_armed_action")
}

/// Show the run's armed action on the tiles drawn next, or none.
pub(crate) fn set_armed(ctx: &egui::Context, view: Option<ArmedView>) {
    ctx.data_mut(|data| match view {
        Some(view) => data.insert_temp(armed_id(), view),
        None => data.remove::<ArmedView>(armed_id()),
    });
}

/// What the DM pressed under an armed tile this frame.
pub(crate) fn take_armed_action(ctx: &egui::Context) -> Option<ArmedAction> {
    ctx.data_mut(|data| {
        let action = data.get_temp::<ArmedAction>(armed_action_id());
        data.remove::<ArmedAction>(armed_action_id());
        action
    })
}

/// A tile's icon: a sword for an attack, a shield for a save, fire for damage.
fn icon(link: &RollLink) -> &'static str {
    match link {
        RollLink::D20 { .. } => design::icons::ATTACK,
        RollLink::SaveDc { .. } => design::icons::SAVE_DC,
        RollLink::Damage { .. } | RollLink::Dice { .. } => design::icons::AREA,
    }
}

fn changes_id() -> Id {
    Id::new("roll_changes")
}

/// The run row whose stat block or sheet is drawn now, if any.
pub(crate) fn roller(ctx: &egui::Context) -> Option<usize> {
    ctx.data(|data| data.get_temp::<usize>(roller_id()))
}

/// A run's attack and damage changes for `row`, so that row's tiles show the numbers Splat
/// rolls. Set it with the roller, around a run's reader.
pub(crate) fn set_changes(ctx: &egui::Context, row: usize, changes: (i64, i64)) {
    ctx.data_mut(|data| data.insert_temp(changes_id(), (row, changes)));
}

/// The changes of the roller drawn now: none outside a run, and none for another row.
fn roller_changes(ctx: &egui::Context) -> (i64, i64) {
    let roller = roller(ctx);
    ctx.data(|data| data.get_temp::<(usize, (i64, i64))>(changes_id()))
        .filter(|(row, _)| Some(*row) == roller)
        .map_or((0, 0), |(_, changes)| changes)
}

/// `1d6+2` with `change` added to its flat part: `1d6+3`. For what a tile shows only.
fn add_flat(dice: &str, change: i64) -> String {
    let dice = dice.trim();
    let digits = dice.len() - dice.trim_end_matches(|c: char| c.is_ascii_digit()).len();
    let (head, number) = dice.split_at(dice.len() - digits);
    let head = head.trim_end();
    // A fixed amount, such as `7`, has no dice.
    if head.is_empty()
        && let Ok(number) = number.parse::<i64>()
    {
        return design::signed(number + change)
            .trim_start_matches('+')
            .to_owned();
    }
    let sign = match head.chars().last() {
        Some('+') => 1,
        Some('-' | '\u{2212}') => -1,
        _ => 0,
    };
    let (base, flat) = match number.parse::<i64>() {
        Ok(number) if sign != 0 => {
            let sign_width = head.chars().last().map_or(0, char::len_utf8);
            (head[..head.len() - sign_width].trim_end(), sign * number)
        }
        _ => (dice, 0),
    };
    match flat + change {
        0 => base.to_owned(),
        total if total > 0 => format!("{base}+{total}"),
        total => format!("{base}\u{2212}{}", -total),
    }
}

/// `link` with a run's attack change in its bonus and its damage change in its first damage
/// part (or the part that replaces it), as `make` rolls it. For what a tile shows; a press still asks for the printed link.
fn with_changes(link: &RollLink, (attack, damage): (i64, i64)) -> RollLink {
    let mut link = link.clone();
    let parts = match &mut link {
        RollLink::D20 {
            what: D20::Attack,
            bonus,
            damage: parts,
            ..
        } => {
            *bonus += attack;
            Some(parts)
        }
        RollLink::SaveDc { damage: parts, .. } | RollLink::Damage { parts, .. } => Some(parts),
        RollLink::D20 { .. } | RollLink::Dice { .. } => None,
    };
    // The first part, and a part that takes its place when its condition holds, as
    // `or 2d6+2 slashing`: either one is first when it rolls.
    for (at, part) in parts.into_iter().flatten().enumerate() {
        if damage != 0 && (at == 0 || part.when.as_ref().is_some_and(|when| when.instead)) {
            part.dice = add_flat(&part.dice, damage);
        }
    }
    link
}

/// A tile's numbers: `+4 · 2d4+2 piercing, 1d6 fire` or `DC 12 Dexterity · 2d6 fire`.
fn tile_detail(link: &RollLink) -> String {
    match link {
        RollLink::D20 { bonus, damage, .. } if damage.is_empty() => design::signed(*bonus),
        RollLink::D20 { bonus, damage, .. } => {
            format!("{} · {}", design::signed(*bonus), parts_text(damage))
        }
        RollLink::SaveDc {
            ability,
            dc,
            damage,
            ..
        } if damage.is_empty() => format!("DC {dc} {ability}"),
        RollLink::SaveDc {
            ability,
            dc,
            damage,
            ..
        } => format!("DC {dc} {ability} · {}", parts_text(damage)),
        RollLink::Damage { parts, .. } => parts_text(parts),
        RollLink::Dice { dice, .. } => dice.clone(),
    }
}

/// A tile's numbers for painting: each damage part, such as `2d6+2 slashing`, joined by
/// no-break spaces, so a wrap falls only between parts and never parts a type from its dice.
fn keep_together(text: &str) -> String {
    text.split(" · ")
        .map(|group| {
            group
                .split(", ")
                .map(|part| part.replace(' ', "\u{a0}"))
                .collect::<Vec<_>>()
                .join(", ")
        })
        .collect::<Vec<_>>()
        .join(" · ")
}

/// The number before `ft` or `feet` right after `key`: `5` in `reach 5 ft.`, `80/320` in
/// `range 80/320 ft.`.
fn feet_after<'t>(text: &'t str, key: &str) -> Option<&'t str> {
    let at = text.find(key)? + key.len();
    let rest = &text[at..];
    let length = rest
        .find(|c: char| !(c.is_ascii_digit() || c == '/'))
        .unwrap_or(rest.len());
    let unit = rest[length..].trim_start();
    (length > 0 && (unit.starts_with("ft") || unit.starts_with("feet"))).then(|| &rest[..length])
}

/// An area in lower-case rules text, `10-foot emanation` or `20-foot-radius sphere`, as
/// `10-ft emanation`.
fn area(text: &str) -> Option<String> {
    const SHAPES: [&str; 7] = [
        "cone",
        "cube",
        "cylinder",
        "emanation",
        "line",
        "sphere",
        "square",
    ];
    for (at, _) in text.match_indices("-foot") {
        let size = &text[..at];
        let digits = size.len() - size.trim_end_matches(|c: char| c.is_ascii_digit()).len();
        if digits == 0 {
            continue;
        }
        let rest = &text[at + "-foot".len()..];
        let (radius, rest) = match rest
            .strip_prefix("-radius")
            .or_else(|| rest.strip_prefix(" radius"))
        {
            Some(rest) => ("-radius", rest),
            None => ("", rest),
        };
        let rest = rest.trim_start();
        if let Some(shape) = SHAPES.into_iter().find(|shape| {
            rest.starts_with(shape)
                && !rest[shape.len()..].starts_with(|c: char| c.is_ascii_alphabetic())
        }) {
            return Some(format!(
                "{}-ft{radius} {shape}",
                &size[size.len() - digits..]
            ));
        }
    }
    None
}

/// Where and how an action lands, from its rules text, for a tile's last line: `Reach 5 ft`,
/// `Range 80/320 ft`, `10-ft emanation · Half on a success`. `None` when the text says none.
fn tile_facts(chip: &Chip) -> Option<String> {
    let text = chip.text.to_lowercase();
    let mut facts = Vec::new();
    if let Some(reach) = feet_after(&text, "reach ") {
        facts.push(format!("reach {reach} ft"));
    }
    if let Some(range) = feet_after(&text, "range ") {
        facts.push(format!("range {range} ft"));
    }
    if let Some(area) = area(&text) {
        facts.push(area);
    } else if let Some(within) = feet_after(&text, "within ") {
        facts.push(format!("within {within} ft"));
    }
    if matches!(chip.link, RollLink::SaveDc { half: true, .. }) {
        facts.push("half on a success".to_owned());
    }
    let mut facts = facts.join(" · ");
    if let Some(first) = facts.get_mut(..1) {
        first.make_ascii_uppercase();
    }
    (!facts.is_empty()).then_some(facts)
}

/// The width of an action tile, when two fit on a line.
const TILE_WIDTH: f32 = 208.0;

/// A tile's words, laid out for its width: its name on one line, then its numbers and its
/// facts, each wrapped, so no damage type or range is cut.
struct TileText {
    mark: std::sync::Arc<egui::Galley>,
    digit: Option<std::sync::Arc<egui::Galley>>,
    name: std::sync::Arc<egui::Galley>,
    detail: std::sync::Arc<egui::Galley>,
    facts: Option<std::sync::Arc<egui::Galley>>,
}

impl TileText {
    fn new(
        ui: &egui::Ui,
        chip: &Chip,
        shown: &RollLink,
        width: f32,
        number: Option<usize>,
        armed: bool,
    ) -> Self {
        let c = colors();
        let inner = (width - 2.0 * design::S).max(1.0);
        let mark = design::one_line(
            ui,
            if armed {
                design::icons::ARMED
            } else {
                icon(&chip.link)
            },
            Type::Body.font(ui),
            if armed { c.accent } else { c.muted },
            f32::INFINITY,
        );
        let digit = number.map(|n| {
            design::one_line(
                ui,
                &n.to_string(),
                Type::Small.font(ui),
                c.faint,
                f32::INFINITY,
            )
        });
        let room = (inner
            - mark.size().x
            - design::XS
            - digit.as_ref().map_or(0.0, |g| g.size().x + design::XS))
        .max(1.0);
        let name = design::words_that_fit(ui, link_label(&chip.link), &Type::Strong.font(ui), room);
        let name = design::one_line(ui, &name, Type::Strong.font(ui), c.text, room);
        let wrapped = |text: String| {
            ui.painter()
                .layout(text, Type::Small.font(ui), c.muted, inner)
        };
        Self {
            mark,
            digit,
            name,
            detail: wrapped(keep_together(&tile_detail(shown))),
            facts: tile_facts(chip).map(wrapped),
        }
    }

    /// The height of a tile with these words.
    fn height(&self) -> f32 {
        Type::Strong.line()
            + self.detail.size().y
            + self.facts.as_ref().map_or(0.0, |facts| facts.size().y)
            + 2.0 * design::XS
            + 2.0
    }
}

/// The hover and focus text of a tile: its rules, its printed numbers when a run changes
/// them, and its keys.
fn tile_rules(ui: &mut egui::Ui, chip: &Chip, shown: &RollLink, keys: &str) {
    let c = colors();
    ui.set_max_width(360.0);
    ui.add(egui::Label::new(Type::Body.rich(ui, &chip.text).color(c.text)).wrap());
    if *shown != chip.link {
        ui.label(
            Type::Small
                .rich(ui, format!("Printed {}", tile_detail(&chip.link)))
                .color(c.muted),
        );
    }
    ui.label(
        Type::Small
            .rich(ui, format!("{}{keys}", hint(shown)))
            .color(c.muted),
    );
}

/// One action as a tile: its icon, name and hotkey number, then its numbers and where it
/// lands. The rules text shows on hover and on keyboard focus, and is the tile's accessible
/// description. It is painted, so the tile is one control with one name.
fn tile(
    ui: &mut egui::Ui,
    chip: &Chip,
    shown: &RollLink,
    size: egui::Vec2,
    number: Option<usize>,
    armed: bool,
) -> egui::Response {
    let c = colors();
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    let response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
    let keys = number.map_or_else(String::new, |n| format!(" Ctrl+{n}."));
    let facts = tile_facts(chip).map_or_else(String::new, |facts| format!(" {facts}."));
    let label = format!(
        "{} · {}.{facts}{keys}",
        link_label(&chip.link),
        tile_detail(shown)
    );
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, &label));
    if ui.is_rect_visible(rect) {
        let text = TileText::new(ui, chip, shown, rect.width(), number, armed);
        let painter = ui.painter_at(rect);
        let edge = if armed {
            c.accent
        } else if response.hovered() {
            c.focus
        } else {
            c.rule
        };
        painter.rect(
            rect,
            design::RADIUS_CONTROL,
            if armed { c.selected } else { c.page },
            egui::Stroke::new(design::HAIRLINE_WIDTH, edge),
            egui::StrokeKind::Inside,
        );
        let inner = rect.shrink2(egui::vec2(design::S, design::XS));
        let mark_width = text.mark.size().x + design::XS;
        painter.galley(inner.min, text.mark, c.text);
        if let Some(digit) = text.digit {
            painter.galley(
                egui::pos2(inner.right() - digit.size().x, inner.top()),
                digit,
                c.faint,
            );
        }
        painter.galley(inner.min + egui::vec2(mark_width, 0.0), text.name, c.text);
        let detail_top = inner.top() + Type::Strong.line();
        let facts_top = detail_top + text.detail.size().y;
        painter.galley(egui::pos2(inner.left(), detail_top), text.detail, c.muted);
        if let Some(facts) = text.facts {
            painter.galley(egui::pos2(inner.left(), facts_top), facts, c.muted);
        }
    }
    if response.has_focus() {
        design::focus_ring(ui, rect);
    }
    rules_on_hover_and_focus(ui, response, chip, shown, &keys)
}

/// A tile's rules text, and its printed numbers and keys, on hover and on keyboard focus (a
/// click gives no focus), and as the control's accessible description.
fn rules_on_hover_and_focus(
    ui: &egui::Ui,
    response: egui::Response,
    chip: &Chip,
    shown: &RollLink,
    keys: &str,
) -> egui::Response {
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_description(chip.text.clone());
    });
    if response.has_focus() {
        response.show_tooltip_ui(|ui| tile_rules(ui, chip, shown, keys));
        response
    } else {
        response.on_hover_ui(|ui| tile_rules(ui, chip, shown, keys))
    }
}

/// How `count` tiles share lines `line` wide: each line's number of tiles and their width.
/// As many 208 px tiles as fit go on a line; a tile that stands alone on its line, by itself
/// or as the last of an odd row, takes the line's width up to the reader measure, so it wraps
/// nothing it need not and its edge meets the reader's.
fn tile_lines(line: f32, count: usize) -> Vec<(usize, f32)> {
    let per = if line >= 2.0 * TILE_WIDTH + design::S {
        (((line + design::S) / (TILE_WIDTH + design::S)) as usize).max(2)
    } else {
        1
    };
    let mut lines = Vec::new();
    let mut left = count;
    while left > 0 {
        let here = left.min(per);
        let width = if here == 1 {
            line.clamp(48.0, design::READER_MEASURE)
        } else {
            TILE_WIDTH
        };
        lines.push((here, width));
        left -= here;
    }
    lines
}

/// Quiet actions on a line under a message, wrapped by whole actions. The first action of
/// each line starts its words at the text edge.
fn line_start_actions<T: Copy>(ui: &mut egui::Ui, actions: &[(&str, T)]) -> Option<T> {
    let mut chosen = None;
    let width = ui.available_width();
    let mut lines: Vec<Vec<(&str, T)>> = Vec::new();
    let mut used = width;
    for (word, action) in actions {
        let needed = Action::quiet(word).density(Density::Read).natural_width(ui);
        // The first action hangs its padding in the gutter, so it takes less of the line.
        if lines.is_empty() || used + design::S + needed > width {
            lines.push(Vec::new());
            used = needed;
        } else {
            used += design::S + needed;
        }
        if let Some(line) = lines.last_mut() {
            line.push((word, *action));
        }
    }
    for line in lines {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = design::S;
            for (at, (word, action)) in line.into_iter().enumerate() {
                let button = Action::quiet(word).density(Density::Read);
                let button = if at == 0 { button.line_start() } else { button };
                if button.show(ui).clicked() {
                    chosen = Some(action);
                }
            }
        });
    }
    chosen
}

/// The armed action's next step under its tiles: a crosshair, then the words in Small. The
/// crosshair stays on the first line when the words wrap, centred on it.
fn armed_hint(ui: &mut egui::Ui, status: &str) {
    let icon_row = ui.fonts_mut(|fonts| fonts.row_height(&Type::Body.font(ui)));
    let text_row = ui.fonts_mut(|fonts| fonts.row_height(&Type::Small.font(ui)));
    // Each starts lower by half of what the other's first line is taller.
    let icon_drop = ((text_row - icon_row) / 2.0).max(0.0);
    let text_drop = ((icon_row - text_row) / 2.0).max(0.0);
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = design::XS;
        ui.vertical(|ui| {
            ui.add_space(icon_drop);
            ui.label(
                Type::Body
                    .rich(ui, design::icons::ARMED)
                    .color(colors().accent),
            );
        });
        ui.vertical(|ui| {
            ui.add_space(text_drop);
            ui.add(egui::Label::new(Type::Small.rich(ui, status).color(colors().text)).wrap());
        });
    });
}

/// An entry's actions as tiles. In a run a tile arms its action, and the line under the
/// tiles says how to use it; outside a run a tile previews, and the line shows the result.
///
/// Each tile comes with its place in the entry's whole list of actions, if it has one: the
/// number it shows is the number that arms it.
pub(crate) fn tiles(ui: &mut egui::Ui, entry: &str, chips: &[(Option<usize>, Chip)]) {
    if chips.is_empty() {
        return;
    }
    let ctx = ui.ctx().clone();
    let in_run = roller(&ctx).is_some();
    let armed = ctx
        .data(|data| data.get_temp::<ArmedView>(armed_id()))
        .filter(|view| in_run && chips.iter().any(|(_, chip)| chip.link == view.link));
    let changes = if in_run { roller_changes(&ctx) } else { (0, 0) };
    // A tile's number is its place in the entry's whole list, which Ctrl+number arms from.
    let number = |place: &Option<usize>| {
        place
            .filter(|place| in_run && *place < 9)
            .map(|place| place + 1)
    };
    set_entry(&ctx, entry);
    ui.add_space(design::XS);
    let lines = tile_lines(ui.available_width(), chips.len());
    let shown: Vec<RollLink> = chips
        .iter()
        .map(|(_, chip)| with_changes(&chip.link, changes))
        .collect();
    let mut at = 0;
    for (line, (count, width)) in lines.into_iter().enumerate() {
        if line > 0 {
            ui.add_space(design::S);
        }
        let range = at..at + count;
        at += count;
        // One height for the tiles of a line, so the line keeps one edge.
        let height = range
            .clone()
            .map(|at| {
                let (place, chip) = &chips[at];
                TileText::new(ui, chip, &shown[at], width, number(place), false).height()
            })
            .fold(0.0, f32::max);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(design::S, design::S);
            for at in range {
                let (place, chip) = &chips[at];
                let lit = armed.as_ref().is_some_and(|view| view.link == chip.link);
                let response = tile(
                    ui,
                    chip,
                    &shown[at],
                    egui::vec2(width, height),
                    number(place),
                    lit,
                );
                press(ui, &response, &chip.link);
            }
        });
    }
    ui.add_space(design::XS);
    if let Some(view) = &armed {
        // What to do now, then what can follow, each on its own line.
        armed_hint(ui, &view.status);
        if let Some(action) = line_start_actions(ui, &view.actions) {
            ctx.data_mut(|data| data.insert_temp(armed_action_id(), action));
        }
    } else if !in_run
        && let Some(outcome) =
            preview_outcome(&ctx, entry).filter(|o| chips.iter().any(|(_, c)| c.link == o.link))
    {
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = design::S;
            outcome_line(ui, &outcome);
        });
    }
    ui.add_space(design::S);
}

/// The totals of an outcome in Strong, with their dice in hover. A muted `·` comes between
/// the parts, as in a run's `11 vs AC 13 · 10 damage`; it never ends a line, so it wraps with
/// the part after it.
pub(crate) fn outcome_line(ui: &mut egui::Ui, outcome: &Outcome) {
    let c = colors();
    let mut first = true;
    let mut part = |ui: &mut egui::Ui, words: &str, kind: Type| {
        if !std::mem::take(&mut first) {
            let gap = ui.spacing().item_spacing.x;
            let width = design::number_width(ui, "·", &Type::Body.font(ui))
                + gap
                + design::number_width(ui, words, &kind.font(ui));
            // A wrapping line's available width is the whole line, so measure from the cursor.
            let (cursor, line) = (ui.cursor().left(), ui.max_rect());
            if cursor > line.left() && line.right() - cursor < width {
                ui.end_row();
            }
            ui.label(Type::Body.rich(ui, "·").color(c.muted));
        }
        ui.label(kind.rich(ui, words).color(c.text))
    };
    for rolled in [&outcome.first, &outcome.damage].into_iter().flatten() {
        let response = part(ui, &rolled.headline(), Type::Strong);
        design::named(ui, &response, &rolled.line());
        response.on_hover_ui(|ui| {
            ui.add(egui::Label::new(rolled.parts_job(ui, c.muted)));
            if let Some(words) = rolled.changed_words() {
                ui.label(Type::Small.rich(ui, words).color(c.participant.changed));
            }
        });
        if let Some(words) = rolled.natural_words() {
            part(ui, words, Type::Body);
        }
    }
}

/// The height of a line of buttons above the dock.
pub(crate) const BAR_BUTTONS: f32 = 28.0;

/// The run's actions on one line above the dock while Lookup or Notes hide the panel: a
/// quiet button for each tile, the armed one marked, with a run's changes in its numbers.
/// The first button starts at the dock's content edge.
pub(crate) fn compact_tiles(ui: &mut egui::Ui, chips: &[Chip]) {
    let ctx = ui.ctx().clone();
    let armed = ctx.data(|data| data.get_temp::<ArmedView>(armed_id()));
    let changes = roller_changes(&ctx);
    ui.horizontal(|ui| {
        ui.set_height(BAR_BUTTONS);
        ui.spacing_mut().item_spacing.x = design::XS;
        ui.add_space(design::S);
        for (at, chip) in chips.iter().enumerate() {
            let lit = armed.as_ref().is_some_and(|view| view.link == chip.link);
            let shown = with_changes(&chip.link, changes);
            let label = Chip::new(shown.clone(), chip.text.clone())
                .map_or_else(|| chip.label.clone(), |shown| shown.label);
            let words = format!("{} {label}", icon(&chip.link));
            let action = Action::secondary(&words)
                .name(&label)
                .density(Density::Read)
                .selected(lit);
            // A button shows whole or not at all. The actions with no room are counted, and
            // Ctrl+number still arms them.
            let left = chips.len() - at;
            let more = format!("+{left}");
            let more_width = design::number_width(ui, &more, &Type::Small.font(ui)) + design::S;
            let room = ui.available_width() - if left > 1 { more_width } else { 0.0 };
            if action.natural_width(ui) > room {
                let hint = format!(
                    "{}. Close Lookup to see them, or press Ctrl and an action’s number.",
                    design::count(left, "more action", "more actions")
                );
                ui.label(Type::Small.rich(ui, more).color(colors().muted))
                    .on_hover_text(hint);
                break;
            }
            let response = action.show(ui);
            let response = rules_on_hover_and_focus(ui, response, chip, &shown, "");
            press(ui, &response, &chip.link);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The text's actions, each with its place in that list.
    fn placed(text: &str) -> Vec<(Option<usize>, Chip)> {
        chips(text)
            .into_iter()
            .enumerate()
            .map(|(at, chip)| (Some(at), chip))
            .collect()
    }

    #[test]
    fn a_tiles_number_is_its_place_in_the_whole_list_of_actions() {
        // A part of the body, as the reader draws one section, keeps the whole body's numbers.
        let source =
            splat::content::definitions::source_file("definitions/creatures/aboleth.md").unwrap();
        let entry = splat::content::Entry::parse("Aboleth.md", source).unwrap();
        let all = actions(&entry.body);
        let labels: Vec<&str> = all.iter().map(|(_, chip)| chip.label.as_str()).collect();
        let tentacle = labels
            .iter()
            .position(|label| label.starts_with("Tentacle"))
            .expect("a Tentacle action");
        assert!(tentacle > 0, "a trait with a save comes first: {labels:?}");
        let section = &entry.body[entry.body.find("Tentacle.").unwrap()..];
        let here = actions(section);
        assert!(here[0].1.label.starts_with("Tentacle"));
        assert_eq!(place_in(&all, &here[0].1), Some(tentacle));
        assert_eq!(
            chips(&entry.body)[tentacle],
            here[0].1,
            "Ctrl+number arms that tile"
        );
        // Spellcasting is prose in the reader, so it takes no number.
        let caster = "Dagger. Melee Attack Roll: +4, reach 5 ft. Hit: 4 (1d4 + 2) Piercing damage.\n\nSpellcasting. The mage casts one of the following spells, using Intelligence as the spellcasting ability (spell save DC 14, +6 to hit with spell attacks).\n\nArcane Burst. Ranged Attack Roll: +6, range 120 ft. Hit: 14 (3d8 + 1) Force damage.";
        let labels: Vec<String> = chips(caster).into_iter().map(|chip| chip.label).collect();
        assert_eq!(labels, ["Dagger +4", "Arcane Burst +6"]);
        // A rider that is no longer prose is in its tile's hover text.
        let claw = "Claw. Melee Attack Roll: +4, reach 5 ft. Hit: 4 (1d4 + 2) Slashing damage. If the target is a creature that isn't an Undead or elf, it is subjected to the following effect. Constitution Saving Throw: DC 10. Failure: The target has the Paralyzed condition until the end of its next turn.";
        assert!(chips(claw)[0].text.contains("Paralyzed condition"));
        // The same text again is not parsed again.
        assert!(Arc::ptr_eq(&actions(caster), &actions(caster)));
    }

    fn attack() -> RollLink {
        RollLink::D20 {
            what: D20::Attack,
            bonus: 4,
            label: "Grab".into(),
            damage: vec![DamagePart {
                dice: "2d6+2".into(),
                kind: Some("Bludgeoning".into()),
                when: None,
            }],
        }
    }

    #[test]
    fn advantage_keeps_the_higher_d20_and_the_log_names_both() {
        let rolled = roll(
            "Bugbear Warrior 2",
            &attack(),
            Mode::Advantage,
            0,
            &mut Random::controlled([6, 13]),
        )
        .unwrap();
        assert_eq!(rolled.total, 17);
        assert_eq!(rolled.natural, Some(13));
        assert_eq!(rolled.headline(), "17 to hit");
        assert_eq!(
            rolled.line(),
            "Bugbear Warrior 2, Grab: to hit 2d20kh1 + 4 with advantage: [6, 13] = 17."
        );
    }

    #[test]
    fn a_run_change_adds_to_the_bonus_and_says_what_it_replaced() {
        let rolled = roll(
            "Rival",
            &attack(),
            Mode::Normal,
            1,
            &mut Random::controlled([20]),
        )
        .unwrap();
        assert_eq!(rolled.total, 25);
        assert_eq!(rolled.changed_words().unwrap(), "+5 (changed from +4)");
        assert_eq!(rolled.natural_words(), Some("Natural 20"));
    }

    #[test]
    fn critical_damage_doubles_dice_and_keeps_each_type() {
        let link = RollLink::Damage {
            label: "Bite".into(),
            parts: vec![
                DamagePart {
                    dice: "1d6+4".into(),
                    kind: Some("Piercing".into()),
                    when: None,
                },
                DamagePart {
                    dice: "1d6".into(),
                    kind: Some("Fire".into()),
                    when: None,
                },
            ],
        };
        let rolled = roll(
            "Wolf",
            &link,
            Mode::Critical,
            0,
            &mut Random::controlled([1, 2, 3, 4]),
        )
        .unwrap();
        assert_eq!(rolled.total, 1 + 2 + 4 + 3 + 4);
        assert_eq!(rolled.headline(), "14 damage: 7 piercing, 7 fire");
    }

    #[test]
    fn a_stat_block_gives_one_chip_per_action_in_order_and_skips_the_printed_quote() {
        let text = "### Actions\n\n> Bite. Melee Attack Roll: +9, reach 5 ft.\n\nGrab. Melee Attack Roll: +4, reach 10 ft. Hit: 9 (2d6 + 2) Bludgeoning damage.\n\nFire Breath (Recharge 5\u{2013}6). Dexterity Saving Throw: DC 21, each creature in a 60-foot Cone. Failure: 66 (12d10) Fire damage.\n\nRegeneration. The troll regains 10 Hit Points.";
        let labels: Vec<String> = chips(text).into_iter().map(|chip| chip.label).collect();
        assert_eq!(labels, ["Grab +4", "Fire Breath DC 21"]);
    }

    #[test]
    fn an_attack_rolls_its_damage_critical_on_a_natural_20() {
        let outcome = make(
            "Wolf",
            &attack(),
            Mode::Normal,
            (0, 1),
            None,
            &mut Random::controlled([20, 1, 2, 3, 4]),
        )
        .unwrap();
        assert_eq!(outcome.first.as_ref().unwrap().total, 24);
        assert_eq!(
            outcome.damage.as_ref().unwrap().total,
            1 + 2 + 3 + 4 + 2 + 1
        );
        assert_eq!(outcome.lines().len(), 2);
    }

    #[test]
    fn a_preview_has_one_muted_separator_between_parts_and_never_ends_a_line_with_it() {
        // iter-21-T-FT3: `24 to hit · Natural 20 · 12 damage`, and a narrow line moves each
        // `·` to the next line with the part after it.
        let outcome = make(
            "Wolf",
            &attack(),
            Mode::Normal,
            (0, 0),
            None,
            &mut Random::controlled([20, 1, 2, 3, 4]),
        )
        .unwrap();
        let (first, damage) = (outcome.first.as_ref(), outcome.damage.as_ref());
        let expected = [
            first.unwrap().headline(),
            "·".into(),
            "Natural 20".into(),
            "·".into(),
            damage.unwrap().headline(),
        ];
        let mut wrapped = false;
        // From the narrowest line that holds the damage words whole.
        for width in [180.0, 220.0, 260.0, 320.0, 600.0] {
            let ctx = egui::Context::default();
            design::theme::install(&ctx);
            // The second frame, once egui knows its sizes.
            let mut output = None;
            for _ in 0..2 {
                let input = egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(width, 400.0),
                    )),
                    ..Default::default()
                };
                output = Some(ctx.run(input, |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.spacing_mut().item_spacing.x = design::S;
                            outcome_line(ui, &outcome);
                        });
                    });
                }));
            }
            let output = output.unwrap();
            let texts: Vec<(String, egui::Rect, egui::Color32)> = output
                .shapes
                .iter()
                .filter_map(|clipped| match &clipped.shape {
                    egui::Shape::Text(text) => Some((
                        text.galley.job.text.clone(),
                        // The first row: a long part may wrap inside its own label.
                        text.galley.rows[0]
                            .rect_without_leading_space()
                            .translate(text.pos.to_vec2()),
                        text.galley.job.sections[0].format.color,
                    )),
                    _ => None,
                })
                .collect();
            let words: Vec<&str> = texts.iter().map(|(words, ..)| words.as_str()).collect();
            assert_eq!(words, expected, "at {width}");
            for pair in texts.windows(2).filter(|pair| pair[0].0 == "·") {
                let (dot, next) = (&pair[0], &pair[1]);
                assert_eq!(dot.2, colors().muted, "at {width}");
                assert!(
                    (next.1.top()..next.1.bottom()).contains(&dot.1.center().y)
                        && dot.1.right() < next.1.left(),
                    "at {width}, `·` {:?} sits on the line of {:?} {:?}",
                    dot.1,
                    next.0,
                    next.1
                );
            }
            wrapped |= texts
                .iter()
                .any(|(words, rect, _)| words == "·" && rect.top() > texts[0].1.bottom());
        }
        assert!(wrapped, "a narrow line moves a `·` to the next line");
    }

    #[test]
    fn a_bonus_is_its_own_part_and_bad_words_are_refused() {
        let rolled = roll_with(
            "Guard",
            &attack(),
            Mode::Normal,
            0,
            Some("1d4"),
            &mut Random::controlled([10, 3]),
        )
        .unwrap();
        assert_eq!(rolled.total, 17);
        assert!(
            rolled.line().contains("1d20 + 4 + 1d4"),
            "{}",
            rolled.line()
        );
    }

    #[test]
    fn a_tile_shows_a_runs_changes_in_the_numbers_it_rolls() {
        assert_eq!(add_flat("1d6+2", 1), "1d6+3");
        assert_eq!(add_flat("1d6", 1), "1d6+1");
        assert_eq!(add_flat("2d6+2", -2), "2d6");
        assert_eq!(add_flat("1d4 + 2", -3), "1d4\u{2212}1");
        assert_eq!(add_flat("1d8-1", 2), "1d8+1");
        assert_eq!(add_flat("7", 1), "8");
        let sickle = RollLink::D20 {
            what: D20::Attack,
            bonus: 3,
            label: "Scorching Sickle".into(),
            damage: vec![
                DamagePart {
                    dice: "1d6+2".into(),
                    kind: Some("Slashing".into()),
                    when: None,
                },
                DamagePart {
                    dice: "1d6".into(),
                    kind: Some("Fire".into()),
                    when: None,
                },
            ],
        };
        // iter-19-R-FT3: the numbers Splat rolls, with the printed ones kept for hover.
        assert_eq!(
            tile_detail(&with_changes(&sickle, (1, 1))),
            "+4 · 1d6+3 slashing, 1d6 fire"
        );
        assert_eq!(with_changes(&sickle, (0, 0)), sickle);
        // `make` rolls the same: the bonus, and the damage change on the first part.
        let mut random = Random::controlled([10, 1, 1]);
        let outcome = make("Guard", &sickle, Mode::Normal, (1, 1), None, &mut random).unwrap();
        assert_eq!(outcome.first.unwrap().total, 14);
        assert_eq!(outcome.damage.unwrap().total, 1 + 2 + 1 + 1);
        // A part that takes the first one's place shows the change too, as it rolls with it.
        let charge = RollLink::D20 {
            what: D20::Attack,
            bonus: 3,
            label: "Gore".into(),
            damage: vec![
                DamagePart {
                    dice: "1d6+2".into(),
                    kind: Some("Piercing".into()),
                    when: None,
                },
                DamagePart {
                    dice: "2d6+2".into(),
                    kind: Some("Piercing".into()),
                    when: Some(dice_text::When {
                        text: "if the attack roll had Advantage".into(),
                        instead: true,
                    }),
                },
            ],
        };
        assert_eq!(
            tile_detail(&with_changes(&charge, (0, 1))),
            "+3 · 1d6+3 piercing, or 2d6+3 piercing"
        );
        let mut random = Random::controlled([10, 10, 1, 1]);
        let outcome = make("Goat", &charge, Mode::Advantage, (0, 1), None, &mut random).unwrap();
        assert_eq!(outcome.damage.unwrap().total, 1 + 1 + 2 + 1);
    }

    #[test]
    fn a_tile_says_where_its_action_lands() {
        let facts = |text: &str| chips(text).first().and_then(tile_facts).unwrap_or_default();
        assert_eq!(
            facts("Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 7 (2d4 + 2) Piercing damage."),
            "Reach 5 ft"
        );
        assert_eq!(
            facts(
                "Longbow. Ranged Attack Roll: +4, range 150/600 ft. Hit: 6 (1d8 + 2) Piercing damage."
            ),
            "Range 150/600 ft"
        );
        assert_eq!(
            facts(
                "Dagger. Melee or Ranged Attack Roll: +4, reach 5 ft. or range 20/60 ft. Hit: 4 (1d4 + 2) Piercing damage."
            ),
            "Reach 5 ft · range 20/60 ft"
        );
        assert_eq!(
            facts(
                "Ember Burst. Dexterity Saving Throw: DC 12, each creature in a 10-foot Emanation originating from the cultist. Failure: 7 (2d6) Fire damage. Success: Half damage."
            ),
            "10-ft emanation · half on a success"
        );
        assert_eq!(
            facts(
                "Fire Breath. The dragon exhales fire in a 15-foot cone. Each creature in that area must make a DC 13 Dexterity saving throw, taking 22 (4d10) fire damage on a failed save, or half as much damage on a successful one."
            ),
            "15-ft cone · half on a success"
        );
        assert_eq!(
            facts(
                "Fireball. Each creature in a 20-foot-radius Sphere must make a DC 15 Dexterity saving throw, taking 28 (8d6) fire damage on a failed save, or half as much damage on a successful one."
            ),
            "20-ft-radius sphere · half on a success"
        );
        // Feet that are no area, and a save with no half, say nothing they do not know.
        assert_eq!(
            facts(
                "Charge. If the goat moved 20+ feet straight toward a target, it takes 7 (2d6) damage."
            ),
            ""
        );
    }

    #[test]
    fn a_tile_alone_on_its_line_takes_the_line_and_two_share_a_wide_one() {
        // A narrow line holds one tile, at the line's width.
        assert_eq!(tile_lines(416.0, 2), vec![(1, 416.0), (1, 416.0)]);
        assert_eq!(tile_lines(300.0, 1), vec![(1, 300.0)]);
        assert_eq!(tile_lines(423.0, 1), vec![(1, 423.0)]);
        // iter-20-R-ST2: a lone tile on a line that holds two takes the line, up to the
        // measure; the old rule gave it 208 px.
        let pair = 2.0 * TILE_WIDTH + design::S;
        assert_eq!(tile_lines(pair, 2), vec![(2, TILE_WIDTH)]);
        assert_eq!(tile_lines(488.0, 1), vec![(1, 488.0)]);
        assert_eq!(
            tile_lines(488.0, 3),
            vec![(2, TILE_WIDTH), (1, 488.0)],
            "the last of an odd row stands alone and takes the line"
        );
        assert_eq!(tile_lines(2000.0, 1), vec![(1, design::READER_MEASURE)]);
        // Must stay true: three fit on a wide line, at 208 px each.
        assert_eq!(
            tile_lines(3.0 * TILE_WIDTH + 2.0 * design::S, 3),
            vec![(3, TILE_WIDTH)]
        );
        assert!(tile_lines(488.0, 0).is_empty());
    }

    #[test]
    fn a_tiles_rules_text_is_its_accessible_description() {
        use egui_kittest::Harness;
        use egui_kittest::kittest::{NodeT, Queryable};
        let text = "Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 7 (2d4 + 2) Piercing damage.";
        let harness = Harness::builder()
            .with_size(egui::vec2(600.0, 300.0))
            .build_ui(move |ui| {
                design::theme::install(ui.ctx());
                tiles(ui, "Wolf", &placed(text));
            });
        let tile = harness.get_by_label_contains("Bite · +4");
        assert_eq!(
            tile.accesskit_node().description().as_deref(),
            Some(text),
            "iter-19-R-ST3: the rules text is the tile's description"
        );
        assert!(
            tile.accesskit_node()
                .label()
                .is_some_and(|label| label.contains("Reach 5 ft.")),
            "its name has where it lands"
        );
    }

    #[test]
    fn keyboard_focus_shows_a_tiles_rules_on_the_tiles_and_on_the_action_bar() {
        use egui_kittest::Harness;
        use egui_kittest::kittest::{NodeT, Queryable};
        let text = "Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 7 (2d4 + 2) Piercing damage.";
        for bar in [false, true] {
            let mut harness = Harness::builder()
                .with_size(egui::vec2(600.0, 300.0))
                .build_ui(move |ui| {
                    design::theme::install(ui.ctx());
                    if bar {
                        compact_tiles(ui, &chips(text));
                    } else {
                        tiles(ui, "Wolf", &placed(text));
                    }
                });
            let rules_shown = |harness: &Harness<'_>| {
                harness
                    .query_all_by_label_contains("Melee Attack Roll: +4, reach 5 ft.")
                    .any(|node| node.accesskit_node().role() == egui::accesskit::Role::Label)
            };
            assert!(!rules_shown(&harness), "no rules before focus (bar {bar})");
            // The tile's name, `Bite · +4 · …`, or the bar's button, `Bite +4`.
            let name = if bar { "Bite +4" } else { "Bite · +4" };
            harness.get_by_label_contains(name).focus();
            harness.run_steps(3);
            assert!(
                rules_shown(&harness),
                "iter-19-R-ST3: keyboard focus shows the rules (bar {bar})"
            );
            assert_eq!(
                harness
                    .get_by_label_contains(name)
                    .accesskit_node()
                    .description()
                    .as_deref(),
                Some(text),
                "the rules are its description (bar {bar})"
            );
        }
    }

    #[test]
    fn a_check_has_no_natural_mark() {
        let link = RollLink::D20 {
            what: D20::Save,
            bonus: 2,
            label: "Dexterity".into(),
            damage: Vec::new(),
        };
        let rolled = roll(
            "Goblin",
            &link,
            Mode::Normal,
            0,
            &mut Random::controlled([20]),
        )
        .unwrap();
        assert_eq!(rolled.natural_words(), None);
        assert_eq!(rolled.headline(), "22 Dexterity save");
    }
}
