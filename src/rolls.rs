//! Rolls in a stat block or a sheet. A run's roster has an action strip under the selected
//! row; a reader outside a run has the same line under its stat head. Numbers in tables (an
//! ability, a save, a skill) also roll. A roll changes nothing: in a run it goes to the Log,
//! and its damage waits for a click on a target's HP. Outside a run it is a preview.
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
fn preview_id() -> Id {
    Id::new("roll_preview_random")
}
fn outcome_id(entry: &str) -> Id {
    Id::new(("roll_preview_outcome", entry))
}
fn bonus_id(entry: &str) -> Id {
    Id::new(("roll_preview_bonus", entry))
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

/// The entry whose numbers are drawn next, to name its rolls.
pub(crate) fn set_entry(ctx: &egui::Context, name: &str) {
    ctx.data_mut(|data| data.insert_temp(entry_id(), name.to_owned()));
}

/// Ask for `link`, from the roller and entry set for what is drawn now.
fn ask(ctx: &egui::Context, link: RollLink, mode: Mode) {
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
            },
        );
    });
}

/// What a press on this number does, in words, for hover.
pub(crate) fn hint(link: &RollLink) -> String {
    let keys = " Shift: advantage. Ctrl: disadvantage.";
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

/// `2d6+2 bludgeoning, 1d6 fire`: every part of a damage roll.
fn parts_text(parts: &[DamagePart]) -> String {
    parts.iter().map(part_text).collect::<Vec<_>>().join(", ")
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
    if !response.hovered() && !response.clicked() {
        return;
    }
    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    response.clone().on_hover_text(hint(link));
    if response.clicked() {
        ask(ui.ctx(), link.clone(), Mode::from_keys(ui, link));
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
    fn headline(&self) -> String {
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
    fn natural_words(&self) -> Option<&'static str> {
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

/// A situational bonus as dice: `1d4`, `+1d4`, `-2`. Spaces and book minus signs are fine.
pub(crate) fn bonus_dice(text: &str) -> Result<Option<String>, &'static str> {
    let plain = dice_text::plain_dice(text);
    let plain = plain.strip_prefix('+').unwrap_or(&plain).to_owned();
    if plain.is_empty() {
        return Ok(None);
    }
    Dice::parse(&plain)
        .map(|_| Some(plain))
        .map_err(|_| "Use dice or a number, such as 1d4 or -2.")
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
}

impl Chip {
    pub fn new(link: RollLink) -> Option<Self> {
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
        Some(Self { label, link })
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
        if let Some(chip) = link.cloned().and_then(Chip::new)
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

    /// Its damage waits for targets: once for an attack, once per target for a save.
    pub fn many_targets(&self) -> bool {
        matches!(self.link, RollLink::SaveDc { .. })
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
    let (first, damage) = match link {
        RollLink::D20 {
            what: D20::Attack,
            label,
            damage,
            ..
        } => {
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
        RollLink::Damage { .. } => (None, Some(roll(who, link, mode, damage_change, random)?)),
        RollLink::Dice { .. } => (Some(roll(who, link, mode, 0, random)?), None),
        RollLink::SaveDc { label, damage, .. } => (
            None,
            damage_roll(who, label, damage, Mode::Normal, damage_change, random)?,
        ),
    };
    Ok(Outcome {
        link: link.clone(),
        first,
        damage,
    })
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
    let bonus = ctx
        .data(|data| data.get_temp::<String>(bonus_id(&ask.entry)))
        .and_then(|text| bonus_dice(&text).ok().flatten());
    let outcome = make(
        &ask.entry,
        &ask.link,
        ask.mode,
        (0, 0),
        bonus.as_deref(),
        &mut random,
    );
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

/// What an action line asks for.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum LineEvent {
    /// A chip pressed, with its keys.
    Roll(RollLink, Mode),
    /// The last action again, this way.
    Again(Mode),
    /// Full or half damage for the next HP click.
    Half(bool),
}

/// What an action line shows under its chips.
pub(crate) struct LineResult<'a> {
    pub outcome: Option<&'a Outcome>,
    /// During a save's damage in a run: whether a HP click applies half.
    pub half: Option<bool>,
    /// In a run: what a click on a row's HP does now.
    pub apply: Option<String>,
}

/// The height of an action line: one control line.
pub(crate) const LINE_HEIGHT: f32 = 28.0;

/// One line: the chips, the last result and what can follow it, and a bonus field at the
/// right. After a roll only the rolled chip stays in view and the rest go in `More`, so the
/// result has the room. One fixed height, so nothing near it moves when a result arrives.
pub(crate) fn action_line(
    ui: &mut egui::Ui,
    salt: impl std::hash::Hash,
    chips: &[Chip],
    result: &LineResult<'_>,
    bonus: &mut String,
) -> Option<LineEvent> {
    let mut event = None;
    let id = ui.id().with(("action_line", salt));
    ui.horizontal(|ui| {
        ui.set_height(LINE_HEIGHT);
        ui.spacing_mut().item_spacing.x = design::S;
        // The bonus field keeps the right end; the chips and the result share what is left.
        let room = (ui.available_width() - BONUS - design::L).max(0.0);
        ui.allocate_ui_with_layout(
            egui::vec2(room, LINE_HEIGHT),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                ui.set_width(room);
                ui.spacing_mut().item_spacing.x = design::XS;
                let active = result.outcome.map(|outcome| &outcome.link);
                event = chip_row(ui, id, chips, active);
                ui.add_space(design::S);
                ui.spacing_mut().item_spacing.x = design::S;
                event = result_row(ui, result, bonus).or(event.take());
            },
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = design::XS;
            let field = design::text_input(ui, "Bonus", bonus, "1d4", BONUS - 12.0);
            field.on_hover_text(
                "A bonus for this creature's d20 rolls, such as 1d4 for Bless. It stays until you clear it.",
            );
            ui.label(Type::Body.rich(ui, "+").color(colors().muted));
        });
    });
    event
}

/// The width of the bonus field and its `+`.
const BONUS: f32 = 64.0;

/// The chips that fit, and `More` for the rest. With a result, only the rolled chip shows.
fn chip_row(
    ui: &mut egui::Ui,
    id: Id,
    chips: &[Chip],
    active: Option<&RollLink>,
) -> Option<LineEvent> {
    let mut event = None;
    let more = Action::quiet("More").density(Density::Read);
    // With no result, the chips may use the line; with one, the rolled chip alone.
    let shown: Vec<&Chip> = match active.and_then(|link| chips.iter().find(|c| &c.link == link)) {
        Some(chip) => vec![chip],
        None => {
            let room = (ui.available_width() - more.natural_width(ui)).max(0.0);
            let mut used = 0.0;
            let mut fit = Vec::new();
            for (at, chip) in chips.iter().enumerate() {
                let width = Action::secondary(&chip.label)
                    .density(Density::Read)
                    .natural_width(ui)
                    + design::XS;
                let limit = if at + 1 == chips.len() {
                    room + more.natural_width(ui)
                } else {
                    room
                };
                if used + width > limit {
                    break;
                }
                used += width;
                fit.push(chip);
            }
            fit
        }
    };
    for chip in &shown {
        let response = Action::secondary(&chip.label)
            .density(Density::Read)
            .selected(active == Some(&chip.link))
            .hint(&hint(&chip.link))
            .show(ui);
        if response.clicked() {
            event = Some(LineEvent::Roll(
                chip.link.clone(),
                Mode::from_keys(ui, &chip.link),
            ));
        }
    }
    let rest: Vec<&Chip> = chips
        .iter()
        .filter(|chip| !shown.iter().any(|shown| shown.link == chip.link))
        .collect();
    if !rest.is_empty() {
        let button = more.name("More actions").show(ui);
        egui::Popup::menu(&button).id(id.with("more")).show(|ui| {
            for chip in rest {
                if ui
                    .button(&chip.label)
                    .on_hover_text(hint(&chip.link))
                    .clicked()
                {
                    event = Some(LineEvent::Roll(
                        chip.link.clone(),
                        Mode::from_keys(ui, &chip.link),
                    ));
                    ui.close();
                }
            }
        });
    }
    if chips.is_empty() {
        ui.label(
            Type::Small
                .rich(ui, "No attacks or saves in this text.")
                .color(colors().muted),
        );
    }
    event
}

/// The last result: each roll's total, then `adv dis crit`, then Full or Half and how to
/// apply, while they fit. The dice are in hover.
fn result_row(ui: &mut egui::Ui, result: &LineResult<'_>, bonus: &str) -> Option<LineEvent> {
    let c = colors();
    let mut event = None;
    // A hint shows the longest of its forms that fits whole.
    let hint = |ui: &mut egui::Ui, forms: &[&str], color: egui::Color32| {
        let room = ui.available_width();
        if let Some(words) = forms
            .iter()
            .find(|words| design::number_width(ui, words, &Type::Small.font(ui)) <= room)
        {
            ui.label(Type::Small.rich(ui, *words).color(color));
        }
    };
    if let Err(problem) = bonus_dice(bonus) {
        hint(ui, &[problem, "Use dice, such as 1d4."], c.result.warning);
        return None;
    }
    let Some(outcome) = result.outcome else {
        hint(
            ui,
            &[
                "Click an action to roll it. Shift: advantage. Ctrl: disadvantage.",
                "Click an action to roll it.",
            ],
            c.muted,
        );
        return None;
    };
    let fits = |ui: &egui::Ui, words: &str, role: Type| {
        design::number_width(ui, words, &role.font(ui)) + design::S <= ui.available_width()
    };
    // Room kept for what can follow the totals: Full and Half, or adv, dis and crit.
    let keep = if result.half.is_some() {
        design::number_width(ui, "Full Half", &Type::Body.font(ui)) + design::L
    } else {
        design::number_width(ui, "adv dis crit", &Type::Small.font(ui)) + design::L
    };
    for rolled in [&outcome.first, &outcome.damage].into_iter().flatten() {
        // Damage by type when there is room; its total alone, with the types in hover, if not.
        let full = rolled.headline();
        let words = if design::number_width(ui, &full, &Type::Strong.font(ui)) + keep
            <= ui.available_width()
        {
            full
        } else {
            format!("{} {}", rolled.total, rolled.what())
        };
        if fits(ui, &words, Type::Strong) {
            let response = ui.label(Type::Strong.rich(ui, &words).color(c.text));
            design::named(ui, &response, &rolled.line());
            response.on_hover_ui(|ui| {
                ui.label(Type::Small.rich(ui, rolled.headline()).color(c.text));
                ui.add(egui::Label::new(rolled.parts_job(ui, c.muted)));
                if let Some(words) = rolled.changed_words() {
                    ui.label(Type::Small.rich(ui, words).color(c.participant.changed));
                }
            });
        }
        if let Some(words) = rolled.natural_words()
            && fits(ui, words, Type::Body)
        {
            ui.label(Type::Body.rich(ui, words).color(c.text));
        }
    }
    if let Some(half) = result.half
        && fits(ui, "Full Half", Type::Body)
    {
        let mut chosen = half;
        if design::segmented(ui, &mut chosen, &[(false, "Full"), (true, "Half")]) {
            event = Some(LineEvent::Half(chosen));
        }
    }
    let d20 = matches!(outcome.link, RollLink::D20 { .. });
    // A save's damage is never a critical hit.
    let damage = outcome.damage.is_some() && !outcome.many_targets();
    for (word, mode, offered, hint) in [
        ("adv", Mode::Advantage, d20, "Roll it again with advantage"),
        (
            "dis",
            Mode::Disadvantage,
            d20,
            "Roll it again with disadvantage",
        ),
        (
            "crit",
            Mode::Critical,
            damage,
            "Roll its damage again as a critical hit",
        ),
    ] {
        if offered && fits(ui, word, Type::Small) && again_link(ui, word, hint) {
            event = Some(LineEvent::Again(mode));
        }
    }
    if let Some(apply) = &result.apply
        && fits(ui, apply, Type::Small)
    {
        ui.label(Type::Small.rich(ui, apply).color(c.muted));
    }
    event
}

/// A short Small link, such as `adv`, in a target at least 24 px each way.
fn again_link(ui: &mut egui::Ui, word: &str, hint: &str) -> bool {
    let c = colors();
    let galley = design::one_line(
        ui,
        word,
        Type::Small.font(ui),
        egui::Color32::PLACEHOLDER,
        f32::INFINITY,
    );
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(galley.size().x.max(24.0), 24.0),
        egui::Sense::click(),
    );
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Link, true, hint));
    let color = if response.hovered() {
        c.focus
    } else {
        c.accent
    };
    let left = rect.center().x - galley.size().x / 2.0;
    let top = rect.center().y - galley.size().y / 2.0;
    ui.painter().hline(
        left..=left + galley.size().x,
        top + galley.size().y,
        egui::Stroke::new(1.0, color.gamma_multiply(0.6)),
    );
    ui.painter().galley(egui::pos2(left, top), galley, color);
    if response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    response.on_hover_text(hint).clicked()
}

/// A reader's action line outside a run: the entry's chips, its last preview, and its bonus.
pub(crate) fn preview_line(ui: &mut egui::Ui, entry: &str, chips: &[Chip]) {
    let ctx = ui.ctx().clone();
    let outcome = preview_outcome(&ctx, entry);
    let mut bonus = ctx
        .data(|data| data.get_temp::<String>(bonus_id(entry)))
        .unwrap_or_default();
    let result = LineResult {
        outcome: outcome.as_ref(),
        half: None,
        apply: None,
    };
    set_entry(&ctx, entry);
    let event = action_line(ui, ("preview", entry), chips, &result, &mut bonus);
    ctx.data_mut(|data| data.insert_temp(bonus_id(entry), bonus));
    match event {
        Some(LineEvent::Roll(link, mode)) => ask(&ctx, link, mode),
        Some(LineEvent::Again(mode)) => {
            if let Some(outcome) = outcome {
                ask(&ctx, outcome.link, mode);
            }
        }
        Some(LineEvent::Half(_)) | None => {}
    }
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
                },
                DamagePart {
                    dice: "1d6".into(),
                    kind: Some("Fire".into()),
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
        assert_eq!(bonus_dice(" +1d4 "), Ok(Some("1d4".into())));
        assert_eq!(bonus_dice("\u{2212}2"), Ok(Some("-2".into())));
        assert_eq!(bonus_dice(""), Ok(None));
        assert!(bonus_dice("bless").is_err());
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
