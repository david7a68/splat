//! Rolls in a stat block or a sheet. Its actions are tiles: in a run a tile arms its action
//! against the roster's rows; outside a run it previews. Numbers in tables (an ability, a
//! save, a skill) also roll. A right-click on a tile or a number chooses how its d20 rolls. A
//! roll changes nothing: in a run it goes to the Log, and its damage goes into Amount.
use crate::design::{self};
use crate::state_context::{Slot, StateContext};
use splat::content::dice_text::{self, D20, DamagePart, RollLink};
use splat::execution::dice::{Dice, Random, Roll};
use splat::memo::Memo;
use std::sync::Arc;

/// How a d20 or a damage roll is made.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Advantage,
    Disadvantage,
    /// Damage dice twice over, for a critical hit.
    Critical,
}

impl Mode {
    /// The same, from the keys a toolkit says are held.
    fn from_held(link: &RollLink, shift: bool, command: bool) -> Self {
        let d20 = matches!(link, RollLink::D20 { .. });
        if d20 && shift {
            Self::Advantage
        } else if d20 && command {
            Self::Disadvantage
        } else {
            Self::Normal
        }
    }
}

/// A press on a rollable number, waiting for whoever owns the dice.
#[derive(Clone, Debug)]
pub struct Ask {
    pub link: RollLink,
    pub occurrence: Option<SpellOccurrence>,
    pub mode: Mode,
    /// The run row whose stat block or sheet was drawn, if any.
    pub roller: Option<usize>,
    /// The entry or character the number is in.
    pub entry: String,
    /// The roller expends inspiration for advantage, by a rule that gives it before the roll.
    pub inspired: bool,
}

/// The roll the DM asked for, for the run or the preview to roll.
const ASK: Slot<Ask> = Slot::new("roll_ask");
/// The run row whose stat block or sheet is drawn now.
const ROLLER: Slot<usize> = Slot::new("roll_roller");
/// The entry whose rolls are drawn now.
const ENTRY: Slot<String> = Slot::new("roll_entry");
/// Whether the roller drawn now has inspiration.
const INSPIRED: Slot<bool> = Slot::new("roll_inspired");
/// The preview's own dice.
const PREVIEW: Slot<Random> = Slot::new("roll_preview_random");

/// An entry's last preview.
fn outcome_slot(entry: &str) -> Slot<Outcome> {
    Slot::new("roll_preview_outcome").with(entry)
}

/// The press of this frame, for the run or the preview stream.
pub fn take_ask(ctx: &StateContext) -> Option<Ask> {
    ctx.take(ASK)
}

/// Who rolls from what is drawn next: a run row, or nobody. Set it around a run's reader.
pub fn set_roller(ctx: &StateContext, roller: Option<usize>) {
    ctx.put(ROLLER, roller);
}

/// Whether the roller drawn next can expend inspiration for advantage before a roll. Set it
/// around a run's reader, by the campaign's inspiration rule and the character's sheet.
pub fn set_inspired(ctx: &StateContext, inspired: bool) {
    ctx.set(INSPIRED, inspired);
}

/// The entry whose numbers are drawn next, to name its rolls.
pub fn set_entry(ctx: &StateContext, name: &str) {
    ctx.set(ENTRY, name.to_owned());
}

/// A press on a rollable number in the reader, with the keys
/// held: Shift rolls a d20 with advantage, Ctrl with disadvantage, as a click here does. The ask
/// waits in `ctx` for whoever owns the dice ([`take_ask`]).
pub fn ask_pressed(ctx: &StateContext, link: &RollLink, shift: bool, command: bool) {
    ask(
        ctx,
        link.clone(),
        None,
        Mode::from_held(link, shift, command),
        false,
    );
}

/// Ask for `link`, from the roller and entry set for what is drawn now.
fn ask(
    ctx: &StateContext,
    link: RollLink,
    occurrence: Option<SpellOccurrence>,
    mode: Mode,
    inspired: bool,
) {
    let roller = ctx.get(ROLLER);
    let entry = ctx.get(ENTRY).unwrap_or_default();
    ctx.set(
        ASK,
        Ask {
            link,
            occurrence,
            mode,
            roller,
            entry,
            inspired,
        },
    );
}

/// How a right-click on a d20 roll can roll it: the mode, and whether inspiration pays for
/// advantage. `verb` names the press: `Roll`, or `Roll for them`.
pub fn mode_choices(verb: &str, inspired: bool) -> Vec<(String, Mode, bool)> {
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
pub fn link_label(link: &RollLink) -> &str {
    match link {
        RollLink::D20 { label, .. }
        | RollLink::Damage { label, .. }
        | RollLink::SaveDc { label, .. }
        | RollLink::Dice { label, .. } => label,
    }
}

/// What a press on this number does, in words, for hover.
pub fn hint(link: &RollLink) -> String {
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
pub fn parts_text(parts: &[DamagePart]) -> String {
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

/// One group of dice in a result: a label, each face and whether it was dropped.
#[derive(Clone, Debug, PartialEq)]
pub struct Piece {
    pub label: String,
    pub faces: Vec<(i64, bool)>,
    /// The dice rolled, such as `2d20kh1` or `2d6+2`, so that one die can roll again.
    pub dice: String,
}

/// One die of an outcome: in its damage or its first roll, in which piece, at which place.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Die {
    pub damage: bool,
    pub piece: usize,
    pub face: usize,
}

/// A made roll, with what the action line and the Log say about it.
#[derive(Clone, Debug, PartialEq)]
pub struct Rolled {
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
pub fn roll(
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

/// A spell's printed position stays the same when a reload shifts its shortcut.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpellOccurrence {
    pub paragraph: usize,
    pub line: usize,
    pub item: usize,
}

/// One action of a creature or a character: a feature with an attack, a save or damage.
#[derive(Clone, Debug, PartialEq)]
pub struct Chip {
    /// `Bite +4`, `Ember Burst DC 12`, `Claw 2d6`.
    pub label: String,
    pub link: RollLink,
    /// The rules text, for hover.
    pub text: String,
    /// A tile's last line when the rules text does not give it: a spell's level, casting time
    /// and range, which come from its entry, not from the words of the stat block.
    pub facts: Option<String>,
    pub spell_occurrence: Option<SpellOccurrence>,
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
        Some(Self {
            label,
            link,
            text,
            facts: None,
            spell_occurrence: None,
        })
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
        // Spellcasting stays prose; its lists of spells are tiles of their own (`library::spells`).
        if paragraph.starts_with(['>', '|', '#', '<'])
            || paragraph.starts_with("Spellcasting")
            || crate::library::spells::spellcasting(paragraph)
        {
            continue;
        }
        // A book may print the parts' labels in italics (`*Melee Attack Roll:* +5`), and one
        // action may list several as numbered items (`1. **Fear Ray.** *Wisdom Saving Throw:*`).
        let plain = list_number(paragraph).replace('*', "");
        let Some(title) = design::reader::run_in_title(&plain) else {
            continue;
        };
        let label = title.split(" (").next().unwrap_or(title);
        let spans = dice_text::find(&plain, label);
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
            .and_then(|link| Chip::new(link, plain.replace('_', "")))
            && !chips.iter().any(|(_, known)| known.label == chip.label)
        {
            chips.push((at, chip));
        }
    }
    chips
}

/// A paragraph without the number of a list item: `1. **Fear Ray.** ...` is
/// `**Fear Ray.** ...`.
fn list_number(paragraph: &str) -> &str {
    let digits = paragraph.len()
        - paragraph
            .trim_start_matches(|c: char| c.is_ascii_digit())
            .len();
    if digits > 0
        && let Some(rest) = paragraph[digits..].strip_prefix(". ")
    {
        rest
    } else {
        paragraph
    }
}

/// The place of an action in an entry's whole list, by which Ctrl+number arms it.
pub(crate) fn place_in(all: &[(usize, Chip)], chip: &Chip) -> Option<usize> {
    all.iter()
        .position(|(_, known)| known.label == chip.label && known.link == chip.link)
}

/// One press of an action and what it rolled: an attack and its damage, a save's damage,
/// or a single d20 or dice roll.
#[derive(Clone, Debug, PartialEq)]
pub struct Outcome {
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
pub fn make(
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
pub fn condition_by_mode(when: &dice_text::When, mode: Mode) -> Option<bool> {
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
pub fn reroll(
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
pub fn preview(ctx: &StateContext, ask: &Ask) {
    let mut random = ctx.get(PREVIEW).unwrap_or_else(|| {
        #[cfg(any(test, feature = "test-support"))]
        let seed = 20_261_110;
        #[cfg(not(any(test, feature = "test-support")))]
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(1, |since| since.as_nanos() as u64);
        Random::seeded(seed)
    });
    let outcome = make(&ask.entry, &ask.link, ask.mode, (0, 0), None, &mut random);
    ctx.set(PREVIEW, random);
    if let Ok(outcome) = outcome {
        ctx.set(outcome_slot(&ask.entry), outcome);
    }
}

/// An entry's last preview, for its action line.
fn preview_outcome(ctx: &StateContext, entry: &str) -> Option<Outcome> {
    ctx.get(outcome_slot(entry))
}

/// The run's armed action, for the tiles to mark it and say what can follow.
#[derive(Clone, Debug)]
pub struct ArmedView {
    pub link: RollLink,
    pub occurrence: Option<SpellOccurrence>,
    /// What to do next, such as `Click a target. Escape stops.`
    pub status: String,
    /// The longest words `status` can say for any armed action and roll mode. The hint keeps
    /// the lines these take, so `Stop` below it keeps its y in every mode (FK4).
    pub reserve: String,
    pub actions: Vec<(&'static str, ArmedAction)>,
}

impl ArmedView {
    fn marks(&self, chip: &Chip) -> bool {
        self.link == chip.link
            && self
                .occurrence
                .is_none_or(|armed| chip.spell_occurrence == Some(armed))
    }
}

/// What the line under an armed tile can ask the run to do.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmedAction {
    /// For a player's attack: roll it for them against the target.
    RollForThem,
    /// For a save: roll its damage once, into Amount.
    RollDamage,
    /// For a save: mark the targets at or above the DC to take half.
    HalfMade,
    Stop,
}

impl ArmedAction {}

/// The run's armed action, for the tiles.
const ARMED: Slot<ArmedView> = Slot::new("roll_armed");
/// What the DM pressed under an armed tile.
const ARMED_ACTION: Slot<ArmedAction> = Slot::new("roll_armed_action");

/// Show the run's armed action on the tiles drawn next, or none.
pub fn set_armed(ctx: &StateContext, view: Option<ArmedView>) {
    ctx.put(ARMED, view);
}

/// What the DM pressed under an armed tile this frame.
pub fn take_armed_action(ctx: &StateContext) -> Option<ArmedAction> {
    ctx.take(ARMED_ACTION)
}

/// A tile's icon: a sword for an attack, a shield for a save, fire for damage.
fn icon(link: &RollLink) -> &'static str {
    match link {
        RollLink::D20 { .. } => design::icons::ATTACK,
        RollLink::SaveDc { .. } => design::icons::SAVE_DC,
        RollLink::Damage { .. } | RollLink::Dice { .. } => design::icons::AREA,
    }
}

/// A run row's attack and damage changes.
const CHANGES: Slot<(usize, (i64, i64))> = Slot::new("roll_changes");

/// The run row whose stat block or sheet is drawn now, if any.
pub fn roller(ctx: &StateContext) -> Option<usize> {
    ctx.get(ROLLER)
}

/// A run's attack and damage changes for `row`, so that row's tiles show the numbers Splat
/// rolls. Set it with the roller, around a run's reader.
pub fn set_changes(ctx: &StateContext, row: usize, changes: (i64, i64)) {
    ctx.set(CHANGES, (row, changes));
}

/// The changes of the roller drawn now: none outside a run, and none for another row.
fn roller_changes(ctx: &StateContext) -> (i64, i64) {
    let roller = roller(ctx);
    ctx.get(CHANGES)
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

/// A tile's two lines of words, its numbers and its facts (the GPUI tiles, and a check
/// outside the app).
pub fn tile_words(chip: &Chip) -> (String, Option<String>) {
    (tile_detail(&chip.link), tile_facts(chip))
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
pub(crate) fn area(text: &str) -> Option<String> {
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
        // A line gives its length first, then its width: `90-foot-long, 5-foot-wide line`.
        if let Some(after) = rest.strip_prefix("-long")
            && let Some(words) = after.split(['.', ';']).next()
            && words.split_whitespace().any(|w| w.starts_with("line"))
        {
            let length = &size[size.len() - digits..];
            // The digits just before `-foot-wide`.
            let wide = words
                .split_once("-foot-wide")
                .and_then(|(head, _)| head.rsplit(|c: char| !c.is_ascii_digit()).next())
                .filter(|wide| !wide.is_empty());
            return Some(match wide {
                Some(wide) => format!("{length}-ft line, {wide} ft wide"),
                None => format!("{length}-ft line"),
            });
        }
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
    if chip.facts.is_some() {
        return chip.facts.clone();
    }
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

/// The height of a line of buttons above the dock.
pub const BAR_BUTTONS: f32 = 28.0;

#[cfg(test)]
mod tests {
    use super::*;

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
        // Numbered items under one action are tiles of their own, each with its own number.
        let rays = "Gaze. The golem shoots one of these rays:\n\n1. **Frost Ray.** *Dexterity Saving Throw:* DC 13. *Failure:* 9 (2d8) Cold damage. *Success:* Half damage.\n\n2. **Slow Ray.** *Wisdom Saving Throw:* DC 13. *Failure:* The target is slowed.";
        let labels: Vec<String> = chips(rays).into_iter().map(|chip| chip.label).collect();
        assert_eq!(labels, ["Frost Ray DC 13", "Slow Ray DC 13"]);
        assert_eq!(actions(rays)[0].0, 1, "the tile stands in its item's place");
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
        // A book may print the parts' labels in italics; the tile reads the same.
        let italic =
            "Bite. *Melee Attack Roll:* +4, reach 5 ft. *Hit:* 7 (2d4 + 2) Piercing damage.";
        assert_eq!(chips(italic)[0].label, "Bite +4");
        assert_eq!(facts(italic), "Reach 5 ft");
        let italic_save = "Burst. *Dexterity Saving Throw:* DC 12, each creature in a 10-foot Emanation. *Failure:* 7 (2d6) Fire damage. *Success:* Half damage.";
        assert_eq!(facts(italic_save), "10-ft emanation · half on a success");
        // A line gives its length first.
        assert_eq!(
            facts(
                "Lightning Breath. Dexterity Saving Throw: DC 19, each creature in a 90-foot-long, 5-foot-wide Line. Failure: 55 (10d10) Lightning damage. Success: Half damage."
            ),
            "90-ft line, 5 ft wide · half on a success"
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

/// Action tiles for the reader: the words and state of each tile, without painting them. Presses go back through [`ask_pressed_at`].
pub mod plain {
    use super::*;

    /// An entry's actions in printed order, each with its paragraph's place in `text`.
    pub fn actions_of(text: &str) -> Arc<Vec<(usize, Chip)>> {
        actions(text)
    }

    /// A chip's place in an entry's whole list of actions: the number that arms it.
    pub fn place_of(all: &[(usize, Chip)], chip: &Chip) -> Option<usize> {
        place_in(all, chip)
    }

    /// The words of one tile and its state.
    #[derive(Clone, Debug)]
    pub struct TileView {
        /// Its icon (an attack, a save, an area, a spell, or the armed mark).
        pub icon: &'static str,
        pub name: String,
        /// `Ctrl+1`, in a run.
        pub keys: Option<String>,
        /// Its numbers (with a run's changes) or an open tile's facts.
        pub detail: String,
        pub facts: Option<String>,
        /// Armed in the run: its frame and fill are the accent's.
        pub armed: bool,
        /// The hover lines: its rules text, then what a press does.
        pub tip: Vec<String>,
        /// Its accessible name.
        pub label: String,
    }

    /// The width of a tile, when two fit on a line.
    pub const TILE_WIDTH: f32 = super::TILE_WIDTH;

    /// The tiles of an entry's actions: each with its place in the whole list (its number).
    /// `entry` names the rolls, as `tile_row` sets it.
    pub fn roll_tiles(
        ctx: &StateContext,
        entry: &str,
        chips: &[(Option<usize>, Chip)],
    ) -> Vec<TileView> {
        set_entry(ctx, entry);
        let in_run = roller(ctx).is_some();
        let armed = ctx.get(ARMED).filter(|_| in_run);
        let changes = if in_run { roller_changes(ctx) } else { (0, 0) };
        chips
            .iter()
            .map(|(place, chip)| {
                let shown = with_changes(&chip.link, changes);
                let number = place
                    .filter(|place| in_run && *place < 9)
                    .map(|place| place + 1);
                let lit = armed.as_ref().is_some_and(|view| view.marks(chip));
                let keys = number.map_or_else(String::new, |n| format!(" Ctrl+{n}."));
                let facts = tile_facts(chip);
                let mut tip = vec![chip.text.clone()];
                if shown != chip.link {
                    tip.push(format!("Printed {}", tile_detail(&chip.link)));
                }
                tip.push(format!("{}{keys}", hint(&shown)));
                TileView {
                    icon: if lit {
                        design::icons::ARMED
                    } else {
                        icon(&chip.link)
                    },
                    name: link_label(&chip.link).to_owned(),
                    keys: number.map(|n| format!("Ctrl+{n}")),
                    detail: keep_together(&tile_detail(&shown)),
                    label: format!(
                        "{} · {}.{}{keys}",
                        link_label(&chip.link),
                        tile_detail(&shown),
                        facts
                            .as_ref()
                            .map_or_else(String::new, |facts| format!(" {facts}."))
                    ),
                    facts,
                    armed: lit,
                    tip,
                }
            })
            .collect()
    }

    /// A tile that opens an entry (a spell with no attack or save): its icon, name and facts.
    pub fn open_tile(name: &str, facts: &str, text: &str) -> TileView {
        let mut tip = Vec::new();
        if !text.is_empty() {
            tip.push(text.to_owned());
        }
        tip.push(format!("Click to read {name}."));
        TileView {
            icon: design::icons::SPELL,
            name: name.to_owned(),
            keys: None,
            detail: facts.to_owned(),
            facts: None,
            armed: false,
            tip,
            label: if facts.is_empty() {
                format!("{name}. Opens its description.")
            } else {
                format!("{name} · {facts}. Opens its description.")
            },
        }
    }

    /// A press on a tile or a dice link, with the keys held and the spell it casts, if any.
    pub fn ask_pressed_at(
        ctx: &StateContext,
        link: &RollLink,
        occurrence: Option<SpellOccurrence>,
        shift: bool,
        command: bool,
    ) {
        ask(
            ctx,
            link.clone(),
            occurrence,
            Mode::from_held(link, shift, command),
            false,
        );
    }

    /// Under an entry's tiles in a run: the armed action's status and what can follow it, when
    /// one of these tiles is armed.
    pub fn armed_under(ctx: &StateContext, chips: &[&Chip]) -> Option<ArmedView> {
        roller(ctx)?;
        ctx.get(ARMED)
            .filter(|view| chips.iter().any(|chip| view.marks(chip)))
    }

    /// A press on one of the armed action's follow-up buttons.
    pub fn send_armed_action(ctx: &StateContext, action: ArmedAction) {
        ctx.set(ARMED_ACTION, action);
    }

    /// Outside a run, the last preview's result under the tiles that rolled it: its totals,
    /// each part's words, joined by a muted dot.
    pub fn preview_words(
        ctx: &StateContext,
        entry: &str,
        links: &[&RollLink],
    ) -> Option<Vec<String>> {
        if roller(ctx).is_some() {
            return None;
        }
        let outcome = preview_outcome(ctx, entry).filter(|o| links.contains(&&o.link))?;
        let mut parts = Vec::new();
        for rolled in [&outcome.first, &outcome.damage].into_iter().flatten() {
            parts.push(rolled.headline());
            if let Some(words) = rolled.natural_words() {
                parts.push(words.to_owned());
            }
        }
        Some(parts)
    }
}
