//! Rolls in a stat block or a sheet. Its actions are tiles: in a run a tile arms its action
//! against the roster's rows; outside a run it previews. Numbers in tables (an ability, a
//! save, a skill) also roll. A right-click on a tile or a number chooses how its d20 rolls. A
//! roll changes nothing: in a run it goes to the Log, and its damage goes into Amount.
use crate::design::{self, Action, Density, Type, colors};
use eframe::egui::{self, Id, text::LayoutJob};
use splat::content::dice_text::{self, D20, DamagePart, RollLink};
use splat::execution::dice::{Dice, Random, Roll};

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

/// One line of a right-click menu.
pub(crate) fn menu_item(ui: &mut egui::Ui, words: &str) -> bool {
    Action::quiet(words)
        .density(Density::Read)
        .show(ui)
        .clicked()
}

/// The right-click menu on a d20 roll. Returns how to roll it, when one was chosen.
fn mode_menu(response: &egui::Response, link: &RollLink) -> Option<(Mode, bool)> {
    if !matches!(link, RollLink::D20 { .. }) {
        return None;
    }
    let inspired = response
        .ctx
        .data(|data| data.get_temp::<bool>(inspired_id()))
        .unwrap_or(false);
    let mut chosen = None;
    egui::Popup::context_menu(response).show(|ui| {
        for (words, mode, inspired) in mode_choices("Roll", inspired) {
            if menu_item(ui, &words) {
                chosen = Some((mode, inspired));
                ui.close();
            }
        }
    });
    chosen
}

/// What a press on this number does, in words, for hover.
pub(crate) fn hint(link: &RollLink) -> String {
    let keys = " Shift: advantage. Ctrl: disadvantage. Right-click for more.";
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
        } => format!("{label}: roll its damage and each creature's DC {dc} {ability} save."),
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
        .map(|part| match &part.when {
            None => part_text(part),
            Some(when) if when.instead => format!("or {}", part_text(part)),
            Some(_) => format!("plus {}", part_text(part)),
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
    if let Some((mode, inspired)) = mode_menu(response, link) {
        ask(ui.ctx(), link.clone(), mode, inspired);
    }
}

/// One group of dice in a result: a label, each face and whether it was dropped.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Piece {
    pub label: String,
    pub faces: Vec<(i64, bool)>,
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
/// DC, or damage, in the order printed. Quotes, tables and headings are not rules.
pub(crate) fn chips(text: &str) -> Vec<Chip> {
    let mut chips: Vec<Chip> = Vec::new();
    for paragraph in text.split("\n\n").map(str::trim) {
        if paragraph.starts_with(['>', '|', '#', '<']) {
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
            && !chips.iter().any(|known| known.label == chip.label)
        {
            chips.push(chip);
        }
    }
    chips
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

/// Reroll an outcome's d20 with inspiration. A new die takes the place of the one that
/// counted, and the keep is made again; the new roll stands. On an attack the damage rolls
/// again only when a critical hit begins or ends. Returns the outcome, the old die and the new.
pub(crate) fn reroll_d20(
    who: &str,
    outcome: &Outcome,
    changes: (i64, i64),
    random: &mut Random,
) -> Result<(Outcome, i64, i64), String> {
    let mut first = outcome.first.clone().ok_or("There is no d20 to reroll.")?;
    let piece = first
        .pieces
        .iter_mut()
        .find(|piece| piece.label == "d20")
        .ok_or("There is no d20 to reroll.")?;
    let kept = piece
        .faces
        .iter()
        .position(|(_, dropped)| !dropped)
        .ok_or("There is no d20 to reroll.")?;
    let old = piece.faces[kept].0;
    let new = Dice::parse("1d20")?.roll(random, false)?.total;
    piece.faces[kept].0 = new;
    let faces: Vec<i64> = piece.faces.iter().map(|(face, _)| *face).collect();
    let pick = match first.mode {
        Mode::Disadvantage => faces.iter().enumerate().min_by_key(|(_, face)| **face),
        _ => faces.iter().enumerate().max_by_key(|(_, face)| **face),
    }
    .map_or(kept, |(at, _)| at);
    for (at, face) in piece.faces.iter_mut().enumerate() {
        face.1 = at != pick;
    }
    let counted = faces[pick];
    first.total = counted + first.flat;
    let was_critical = first.natural == Some(20);
    if first.natural.is_some() {
        first.natural = Some(counted);
    }
    let critical = first.natural == Some(20);
    let damage = match &outcome.link {
        RollLink::D20 {
            what: D20::Attack,
            label,
            damage,
            ..
        } if was_critical != critical => {
            let mode = if critical {
                Mode::Critical
            } else {
                Mode::Normal
            };
            damage_roll(who, label, damage, mode, changes.1, random)?
        }
        _ => outcome.damage.clone(),
    };
    Ok((
        Outcome {
            link: outcome.link.clone(),
            first: Some(first),
            damage,
        },
        old,
        new,
    ))
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

/// A tile's second line: `+4 · 2d4+2 piercing, 1d6 fire` or `DC 12 Dexterity · 2d6 fire`.
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

/// The width of an action tile.
const TILE_WIDTH: f32 = 208.0;

/// One action as a tile: its icon, name and hotkey number, and its numbers under them. The
/// rules text is in hover. It is painted, so the tile is one control with one name.
fn tile(ui: &mut egui::Ui, chip: &Chip, number: Option<usize>, armed: bool) -> egui::Response {
    let c = colors();
    let width = TILE_WIDTH.min(ui.available_width()).max(48.0);
    let height = Type::Strong.line() + Type::Small.line() + 2.0 * design::XS + 2.0;
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::click());
    let response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
    let keys = number.map_or_else(String::new, |n| format!(" Ctrl+{n}."));
    let label = format!(
        "{} · {}.{keys}",
        link_label(&chip.link),
        tile_detail(&chip.link)
    );
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, &label));
    if ui.is_rect_visible(rect) {
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
        let mark = if armed {
            design::icons::ARMED
        } else {
            icon(&chip.link)
        };
        let mark = design::one_line(
            ui,
            mark,
            Type::Body.font(ui),
            if armed { c.accent } else { c.muted },
            f32::INFINITY,
        );
        let mark_width = mark.size().x + design::XS;
        painter.galley(inner.min, mark, c.text);
        let digit = number.map(|n| {
            design::one_line(
                ui,
                &n.to_string(),
                Type::Small.font(ui),
                c.faint,
                f32::INFINITY,
            )
        });
        let digit_width = digit.as_ref().map_or(0.0, |g| g.size().x + design::XS);
        if let Some(digit) = digit {
            painter.galley(
                egui::pos2(inner.right() - digit.size().x, inner.top()),
                digit,
                c.faint,
            );
        }
        let room = (inner.width() - mark_width - digit_width).max(1.0);
        let name = design::words_that_fit(ui, link_label(&chip.link), &Type::Strong.font(ui), room);
        let name = design::one_line(ui, &name, Type::Strong.font(ui), c.text, room);
        painter.galley(inner.min + egui::vec2(mark_width, 0.0), name, c.text);
        let detail = tile_detail(&chip.link);
        let detail = design::words_that_fit(ui, &detail, &Type::Small.font(ui), inner.width());
        let detail = design::one_line(ui, &detail, Type::Small.font(ui), c.muted, inner.width());
        painter.galley(
            egui::pos2(inner.left(), inner.top() + Type::Strong.line()),
            detail,
            c.muted,
        );
    }
    if response.has_focus() {
        design::focus_ring(ui, rect);
    }
    response.on_hover_ui(|ui| {
        ui.set_max_width(360.0);
        ui.add(egui::Label::new(Type::Body.rich(ui, &chip.text).color(c.text)).wrap());
        ui.label(
            Type::Small
                .rich(ui, format!("{}{keys}", hint(&chip.link)))
                .color(c.muted),
        );
    })
}

/// An entry's actions as tiles. In a run a tile arms its action, and the line under the
/// tiles says how to use it; outside a run a tile previews, and the line shows the result.
pub(crate) fn tiles(ui: &mut egui::Ui, entry: &str, chips: &[Chip]) {
    if chips.is_empty() {
        return;
    }
    let ctx = ui.ctx().clone();
    let in_run = ctx.data(|data| data.get_temp::<usize>(roller_id()).is_some());
    let armed = ctx
        .data(|data| data.get_temp::<ArmedView>(armed_id()))
        .filter(|view| in_run && chips.iter().any(|chip| chip.link == view.link));
    set_entry(&ctx, entry);
    ui.add_space(design::XS);
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = egui::vec2(design::S, design::S);
        for (at, chip) in chips.iter().enumerate() {
            let number = (in_run && at < 9).then_some(at + 1);
            let lit = armed.as_ref().is_some_and(|view| view.link == chip.link);
            let response = tile(ui, chip, number, lit);
            press(ui, &response, &chip.link);
        }
    });
    ui.add_space(design::XS);
    if let Some(view) = &armed {
        // What to do now, then what can follow, each on its own line.
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = design::XS;
            ui.label(
                Type::Body
                    .rich(ui, design::icons::ARMED)
                    .color(colors().accent),
            );
            ui.add(
                egui::Label::new(Type::Small.rich(ui, &view.status).color(colors().text)).wrap(),
            );
        });
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = design::S;
            for (word, action) in &view.actions {
                if Action::quiet(word)
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                {
                    ctx.data_mut(|data| data.insert_temp(armed_action_id(), *action));
                }
            }
        });
    } else if !in_run
        && let Some(outcome) =
            preview_outcome(&ctx, entry).filter(|o| chips.iter().any(|c| c.link == o.link))
    {
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = design::S;
            outcome_line(ui, &outcome);
        });
    }
    ui.add_space(design::S);
}

/// The totals of an outcome in Strong, with their dice in hover.
pub(crate) fn outcome_line(ui: &mut egui::Ui, outcome: &Outcome) {
    let c = colors();
    for rolled in [&outcome.first, &outcome.damage].into_iter().flatten() {
        let response = ui.label(Type::Strong.rich(ui, rolled.headline()).color(c.text));
        design::named(ui, &response, &rolled.line());
        response.on_hover_ui(|ui| {
            ui.add(egui::Label::new(rolled.parts_job(ui, c.muted)));
            if let Some(words) = rolled.changed_words() {
                ui.label(Type::Small.rich(ui, words).color(c.participant.changed));
            }
        });
        if let Some(words) = rolled.natural_words() {
            ui.label(Type::Body.rich(ui, words).color(c.text));
        }
    }
}

/// The run's actions on one line above the dock while Lookup or Notes hide the panel:
/// a quiet button for each tile, the armed one marked.
pub(crate) fn compact_tiles(ui: &mut egui::Ui, chips: &[Chip]) {
    let ctx = ui.ctx().clone();
    let armed = ctx.data(|data| data.get_temp::<ArmedView>(armed_id()));
    ui.horizontal(|ui| {
        ui.set_height(28.0);
        ui.spacing_mut().item_spacing.x = design::XS;
        for chip in chips {
            if ui.available_width() < 80.0 {
                break;
            }
            let lit = armed.as_ref().is_some_and(|view| view.link == chip.link);
            let words = format!("{} {}", icon(&chip.link), chip.label);
            let response = Action::secondary(&words)
                .name(&chip.label)
                .density(Density::Read)
                .selected(lit)
                .hint(&hint(&chip.link))
                .show(ui);
            press(ui, &response, &chip.link);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

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
