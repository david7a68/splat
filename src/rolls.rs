//! Rolls from dice links in a stat block or a sheet. A press asks for a roll; whoever owns
//! the dice (an open run, or the preview stream) makes it. A roll changes nothing, so its
//! result shows beside the link and, in a run, in the Log and the roll strip.
use crate::design::{self, Type, colors};
use eframe::egui::{self, Id, Pos2, text::LayoutJob};
use splat::content::dice_text::{D20, DamagePart, RollLink};
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

/// A press on a dice link, waiting for whoever owns the dice.
#[derive(Clone, Debug)]
pub(crate) struct Ask {
    pub link: RollLink,
    pub mode: Mode,
    /// Where the result shows: just below the link.
    pub at: Pos2,
    /// The run row whose stat block or sheet was drawn, if any.
    pub roller: Option<usize>,
    /// The entry or character the link is in.
    pub entry: String,
    /// A situational bonus from the menu, such as `1d4` for Bless or `-2`.
    pub extra: Option<String>,
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
fn bonus_id() -> Id {
    Id::new("roll_menu_bonus")
}
fn menu_id() -> Id {
    Id::new("roll_menu")
}
fn shown_id() -> Id {
    Id::new("roll_shown")
}
fn preview_id() -> Id {
    Id::new("roll_preview_random")
}
fn anchor_id() -> Id {
    Id::new("roll_anchor")
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

/// The entry whose links are drawn next, to name its rolls.
pub(crate) fn set_entry(ctx: &egui::Context, name: &str) {
    ctx.data_mut(|data| data.insert_temp(entry_id(), name.to_owned()));
}

/// A press on `link`, from the roller and entry set for what is drawn now.
fn ask(ui: &egui::Ui, link: &RollLink, mode: Mode, at: Pos2) -> Ask {
    let roller = ui.ctx().data(|data| data.get_temp::<usize>(roller_id()));
    let entry = ui
        .ctx()
        .data(|data| data.get_temp::<String>(entry_id()))
        .unwrap_or_default();
    Ask {
        link: link.clone(),
        mode,
        at,
        roller,
        entry,
        extra: None,
    }
}

/// What a press on this link does, in words, for hover.
pub(crate) fn hint(link: &RollLink) -> String {
    let keys = " Shift: advantage. Ctrl: disadvantage. Right-click for more.";
    match link {
        RollLink::D20 {
            what, bonus, label, ..
        } => {
            let what = match what {
                D20::Attack => format!("{label} attack"),
                _ => d20_words(*what, label),
            };
            format!("Roll 1d20 {}: {what}.{keys}", signed_spaced(*bonus))
        }
        RollLink::Damage { label, parts } => format!(
            "Roll {}: {label} damage. Right-click for critical damage.",
            parts_text(parts)
        ),
        RollLink::Dice { dice, .. } => format!("Roll {dice}."),
        RollLink::SaveDc { ability, dc, .. } => {
            format!("Call for a DC {dc} {ability} save from the targets.")
        }
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
pub(crate) fn parts_text(parts: &[DamagePart]) -> String {
    parts.iter().map(part_text).collect::<Vec<_>>().join(", ")
}

/// Handle the pointer on a dice link: hover words, a click with its keys, and the menu.
pub(crate) fn press(ui: &egui::Ui, response: &egui::Response, link: &RollLink) {
    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    let at = response
        .hover_pos()
        .map_or(response.rect.left_bottom(), |pos| {
            egui::pos2(pos.x, response.rect.bottom().min(pos.y + Type::Body.line()))
        });
    if response.clicked() || response.secondary_clicked() {
        // The result follows its link when the page moves, as the strip opens above it.
        let anchor = (response.id, at - response.rect.min);
        ui.ctx()
            .data_mut(|data| data.insert_temp(anchor_id(), anchor));
    }
    if response.secondary_clicked() {
        let asked = ask(ui, link, Mode::Normal, at);
        ui.ctx().data_mut(|data| {
            data.insert_temp(menu_id(), asked);
            // Each menu starts with no bonus, so one creature's Bless never rolls for another.
            data.remove::<String>(bonus_id());
        });
        return;
    }
    response.clone().on_hover_text(hint(link));
    if !response.clicked() {
        return;
    }
    let modifiers = ui.input(|i| i.modifiers);
    let d20 = matches!(link, RollLink::D20 { .. });
    let mode = if d20 && modifiers.shift {
        Mode::Advantage
    } else if d20 && modifiers.command {
        Mode::Disadvantage
    } else {
        Mode::Normal
    };
    let asked = ask(ui, link, mode, at);
    ui.ctx().data_mut(|data| data.insert_temp(ask_id(), asked));
}

/// Keep an open result or menu beside the link that `response` draws, where it is now.
pub(crate) fn follow(ui: &egui::Ui, response: &egui::Response) {
    let ctx = ui.ctx();
    let Some((id, offset)) = ctx.data(|data| data.get_temp::<(Id, egui::Vec2)>(anchor_id())) else {
        return;
    };
    if id != response.id {
        return;
    }
    let at = response.rect.min + offset;
    let mut moved = false;
    ctx.data_mut(|data| {
        if let Some(mut shown) = data.get_temp::<Shown>(shown_id())
            && shown.at != at
        {
            shown.at = at;
            data.insert_temp(shown_id(), shown);
            moved = true;
        }
        if let Some(mut asked) = data.get_temp::<Ask>(menu_id())
            && asked.at != at
        {
            asked.at = at;
            data.insert_temp(menu_id(), asked);
            moved = true;
        }
    });
    if moved {
        ctx.request_repaint();
    }
}

/// A dice link outside Markdown: a table cell, a skill, a sheet's bonus.
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
    // A pointer target is at least 24 px tall, as the reader's other links are.
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(
            galley.size().x.max(24.0),
            galley.size().y.max(role.line()).max(24.0),
        ),
        egui::Sense::hover(),
    );
    // An id from the link, so its result follows it when lines appear above it.
    let response = ui.interact(
        rect,
        ui.id().with(("dice_link", link.destination(), text)),
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

/// The pointer on a drawn dice link: its open result stays beside it, and a press rolls.
pub(crate) fn interact(ui: &egui::Ui, response: &egui::Response, link: &RollLink) {
    follow(ui, response);
    if response.hovered() || response.clicked() || response.secondary_clicked() {
        press(ui, response, link);
    }
}

/// Where a popup at `at` fits in the window: below its link, or above it when there is no
/// room below. Its size is from the frame before; a new popup starts below.
fn place(ctx: &egui::Context, id: Id, at: Pos2) -> Pos2 {
    let screen = ctx.content_rect();
    let Some(size) = ctx
        .memory(|memory| memory.area_rect(id))
        .map(|rect| rect.size())
    else {
        return at;
    };
    let x =
        at.x.min(screen.right() - size.x - design::S)
            .max(screen.left());
    let y = if at.y + size.y > screen.bottom() - design::S {
        // Above the link, clear of its whole line: `at` is a line below the pointer, which
        // can be anywhere on a line of prose. The link stays free for its next roll.
        let line = Type::Body.line().max(Type::Prose.line());
        (at.y - 2.0 * line - design::XS - size.y).max(screen.top())
    } else {
        at.y
    };
    egui::pos2(x, y)
}

/// The right-click menu of the last dice link, until the DM picks a way to roll.
/// What a menu rolls, at its head: `Bite · 1d20 + 4`.
fn menu_title(link: &RollLink) -> String {
    match link {
        RollLink::D20 {
            what, bonus, label, ..
        } => {
            let name = match what {
                D20::Attack => format!("{label} attack"),
                _ => d20_words(*what, label),
            };
            format!("{name} · 1d20 {}", signed_spaced(*bonus))
        }
        RollLink::Damage { label, parts } => format!("{label} · {}", parts_text(parts)),
        RollLink::Dice { label, dice } if label.is_empty() => dice.clone(),
        RollLink::Dice { label, dice } => format!("{label} · {dice}"),
        RollLink::SaveDc {
            label, ability, dc, ..
        } => format!("{label} · DC {dc} {ability} save"),
    }
}

/// The right-click menu of the last dice link: what it rolls, each way to roll it with its
/// click, an attack's damage, and a bonus for the roll that is picked.
pub(crate) fn show_menu(ctx: &egui::Context) {
    let Some(asked) = ctx.data(|data| data.get_temp::<Ask>(menu_id())) else {
        return;
    };
    let close = |ctx: &egui::Context| {
        ctx.data_mut(|data| {
            data.remove::<Ask>(menu_id());
            data.remove::<String>(bonus_id());
        });
    };
    let mut typed = ctx
        .data(|data| data.get_temp::<String>(bonus_id()))
        .unwrap_or_default();
    let bonus = bonus_dice(&typed);
    // Each choice: words, its click, and the roll it asks for.
    let mut choices: Vec<(&str, &str, RollLink, Mode)> = match &asked.link {
        RollLink::D20 { .. } => vec![
            ("Roll", "Click", asked.link.clone(), Mode::Normal),
            (
                "Roll with advantage",
                "Shift+click",
                asked.link.clone(),
                Mode::Advantage,
            ),
            (
                "Roll with disadvantage",
                "Ctrl+click",
                asked.link.clone(),
                Mode::Disadvantage,
            ),
        ],
        RollLink::Damage { .. } => vec![
            ("Roll", "Click", asked.link.clone(), Mode::Normal),
            (
                "Roll critical damage",
                "",
                asked.link.clone(),
                Mode::Critical,
            ),
        ],
        RollLink::Dice { .. } => vec![("Roll", "Click", asked.link.clone(), Mode::Normal)],
        RollLink::SaveDc { .. } => {
            vec![(
                "Call for the save",
                "Click",
                asked.link.clone(),
                Mode::Normal,
            )]
        }
    };
    // An attack's damage, without its attack: an automatic hit, or a roll again.
    let damage = match &asked.link {
        RollLink::D20 {
            what: D20::Attack,
            label,
            damage,
            ..
        } if !damage.is_empty() => Some(RollLink::Damage {
            label: label.clone(),
            parts: damage.clone(),
        }),
        _ => None,
    };
    let first_damage = choices.len();
    if let Some(damage) = damage {
        choices.push(("Roll damage", "", damage.clone(), Mode::Normal));
        choices.push(("Roll critical damage", "", damage, Mode::Critical));
    }
    let takes_bonus = !matches!(asked.link, RollLink::SaveDc { .. });
    let response = egui::Area::new(menu_id())
        .order(egui::Order::Foreground)
        .fixed_pos(place(ctx, menu_id(), asked.at))
        .show(ctx, |ui| {
            egui::Frame::menu(ui.style()).show(ui, |ui| {
                ui.set_min_width(240.0);
                ui.spacing_mut().item_spacing.y = 0.0;
                ui.label(
                    Type::Small
                        .rich(ui, menu_title(&asked.link))
                        .color(colors().muted),
                );
                ui.add_space(design::XS);
                let blocked = bonus.as_ref().err().copied();
                let mut picked = None;
                for (at, (word, click, link, mode)) in choices.iter().enumerate() {
                    if at == first_damage && at > 0 {
                        ui.add_space(design::XS);
                        design::rule(ui);
                        ui.add_space(design::XS);
                    }
                    let action = design::Action::quiet(word).density(design::Density::Read);
                    let action = if click.is_empty() {
                        action
                    } else {
                        action.key(click)
                    };
                    if action.blocked(blocked).show(ui).clicked() && blocked.is_none() {
                        picked = Some((link.clone(), *mode));
                    }
                }
                if takes_bonus {
                    ui.add_space(design::XS);
                    design::rule(ui);
                    ui.add_space(design::XS);
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = design::S;
                        ui.label(Type::Body.rich(ui, "Bonus").color(colors().text));
                        let field = design::text_input(ui, "Bonus", &mut typed, "1d4", 72.0);
                        // Enter rolls the first choice with the bonus.
                        if field.lost_focus()
                            && ui.input(|i| i.key_pressed(egui::Key::Enter))
                            && blocked.is_none()
                        {
                            picked = choices
                                .first()
                                .map(|(_, _, link, mode)| (link.clone(), *mode));
                        }
                    });
                    let words =
                        blocked.unwrap_or("Adds to the roll you pick, such as 1d4 for Bless.");
                    ui.add(
                        egui::Label::new(Type::Small.rich(ui, words).color(if blocked.is_some() {
                            colors().result.warning
                        } else {
                            colors().muted
                        }))
                        .wrap(),
                    );
                }
                picked
            })
        });
    ctx.data_mut(|data| data.insert_temp(bonus_id(), typed));
    if let Some((link, mode)) = response.inner.inner {
        let extra = bonus.ok().flatten();
        ctx.data_mut(|data| {
            data.insert_temp(
                ask_id(),
                Ask {
                    link,
                    mode,
                    extra,
                    ..asked
                },
            );
        });
        close(ctx);
    } else if ctx.input(|i| i.pointer.any_pressed())
        && !response.response.contains_pointer()
        && ctx.input(|i| !i.pointer.secondary_pressed())
    {
        close(ctx);
    }
}

/// One group of dice in a result: a label, each face and whether it was dropped.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Piece {
    pub label: String,
    pub faces: Vec<(i64, bool)>,
}

/// A made roll, with what the strip, the popover and the Log say about it.
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
    pub fn attack(&self) -> bool {
        matches!(
            self.link,
            RollLink::D20 {
                what: D20::Attack,
                ..
            }
        )
    }

    /// Damage from a feature that hits or forces a save, which must not land on its roller.
    pub fn feature_damage(&self) -> bool {
        matches!(self.link, RollLink::Damage { .. })
    }

    /// A total `Use` can put in Amount: damage or dice, not a d20.
    pub fn usable(&self) -> bool {
        !matches!(self.link, RollLink::D20 { .. })
    }

    pub fn damage(&self) -> &[DamagePart] {
        match &self.link {
            RollLink::D20 { damage, .. } => damage,
            _ => &[],
        }
    }

    /// The words after the total: `to hit`, `damage`, `Dexterity save`.
    pub fn what(&self) -> String {
        match &self.link {
            RollLink::D20 { what, label, .. } => d20_words(*what, label),
            RollLink::Damage { .. } => "damage".to_owned(),
            RollLink::Dice { dice, .. } => dice.clone(),
            RollLink::SaveDc { .. } => String::new(),
        }
    }

    /// The feature, when the words after the total do not already name it.
    pub fn feature(&self) -> Option<&str> {
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

    /// The roller, then the feature: `Wolf · Bite`.
    pub fn name(&self) -> String {
        match self.feature() {
            Some(feature) => format!("{} · {feature}", self.who),
            None => self.who.clone(),
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
    pub fn parts_job(&self, ui: &egui::Ui, color: egui::Color32) -> LayoutJob {
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

    pub fn changed_words(&self) -> Option<String> {
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
    roll_with(who, link, mode, change, None, random)
}

/// A situational bonus as dice: `1d4`, `+1d4`, `-2`. Spaces and book minus signs are fine.
pub(crate) fn bonus_dice(text: &str) -> Result<Option<String>, &'static str> {
    let plain = splat::content::dice_text::plain_dice(text);
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
    let (mut expression, mut total) = (expression, total);
    if let Some(extra) = extra {
        let rolled = Dice::parse(extra)?.roll(random, false)?;
        total += rolled.total;
        let shown = extra.strip_prefix('-').map_or_else(
            || format!(" + {extra}"),
            |minus| format!(" \u{2212} {minus}"),
        );
        expression.push_str(&shown);
        if !rolled.dice.is_empty() {
            pieces.push(Piece {
                label: format!("{extra} bonus"),
                faces: faces(&rolled),
            });
        }
        if !by_kind.is_empty() {
            by_kind.push((rolled.total, None));
        }
    }
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

/// A roll made outside a run, on its own stream. Tests use a fixed seed.
pub(crate) fn preview(ctx: &egui::Context, ask: &Ask) -> Result<Rolled, String> {
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
    let rolled = roll_with(
        &ask.entry,
        &ask.link,
        ask.mode,
        0,
        ask.extra.as_deref(),
        &mut random,
    );
    ctx.data_mut(|data| data.insert_temp(preview_id(), random));
    rolled
}

/// What the result beside a link can ask the run to do.
/// What a held result beside a link can ask the run to do.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Action {
    RollDamage,
    RollSaves,
    HalfMade,
}

/// A call for a save, beside its DC: what it is and what can follow.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CallCard {
    pub who: String,
    pub label: String,
    pub ability: String,
    pub dc: i64,
    pub targets: usize,
    pub can_roll: bool,
    pub can_half: bool,
    pub damage: bool,
}

/// The result shown beside its link: a roll, or a call for a save.
#[derive(Clone, Debug)]
pub(crate) struct Shown {
    pub rolled: Option<Rolled>,
    pub call: Option<CallCard>,
    pub at: Pos2,
    /// In a run, a result from a stat block holds the panel on its roller. It stays open
    /// across clicks on the roster, until Escape, the next roll or Next turn.
    pub held: bool,
    /// The selected row's AC beside an attack, such as `AC 15 Oren`.
    pub armor: Option<String>,
}

pub(crate) fn show_result(ctx: &egui::Context, shown: Shown) {
    ctx.data_mut(|data| data.insert_temp(shown_id(), shown));
}

pub(crate) fn shown(ctx: &egui::Context) -> Option<Shown> {
    ctx.data(|data| data.get_temp::<Shown>(shown_id()))
}

pub(crate) fn close_result(ctx: &egui::Context) {
    ctx.data_mut(|data| data.remove::<Shown>(shown_id()));
}

/// True when Escape belongs to an open result or menu. It closes the top one.
pub(crate) fn escape(ctx: &egui::Context) -> bool {
    let menu = ctx.data(|data| data.get_temp::<Ask>(menu_id()).is_some());
    let open = shown(ctx).is_some();
    if (menu || open) && ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
    {
        if menu {
            ctx.data_mut(|data| data.remove::<Ask>(menu_id()));
        } else {
            close_result(ctx);
        }
        return true;
    }
    false
}

/// The total in heading type, the parts in Small, and any actions. Shared by the popover and
/// the run's strip.
pub(crate) fn result_lines(ui: &mut egui::Ui, rolled: &Rolled, armor: Option<&str>) {
    let c = colors();
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = design::XS;
        ui.label(Type::Small.rich(ui, rolled.name()).color(c.muted));
    });
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = design::S;
        ui.label(Type::Heading.rich(ui, rolled.headline()).color(c.text));
        if let Some(armor) = armor {
            ui.label(Type::Body.rich(ui, armor).color(c.text));
        }
        if let Some(words) = rolled.natural_words() {
            ui.label(
                Type::Body
                    .rich(ui, format!("{} {words}", design::icons::INFO))
                    .color(c.text),
            );
        }
    });
    ui.add(egui::Label::new(rolled.parts_job(ui, c.muted)).wrap());
    if let Some(words) = rolled.changed_words() {
        ui.label(Type::Small.rich(ui, words).color(c.participant.changed));
    }
}

/// The result beside its link, until Escape, a click elsewhere or the next roll. A held
/// result stays open across clicks elsewhere.
pub(crate) fn show_popover(ctx: &egui::Context) -> Option<Action> {
    let shown = shown(ctx)?;
    let mut action = None;
    let response = egui::Area::new(shown_id())
        .order(egui::Order::Foreground)
        .fixed_pos(place(ctx, shown_id(), shown.at))
        .show(ctx, |ui| {
            egui::Frame::popup(ui.style()).show(ui, |ui| {
                ui.set_max_width(320.0);
                ui.spacing_mut().item_spacing.y = design::XS;
                if let Some(rolled) = &shown.rolled {
                    result_lines(ui, rolled, shown.armor.as_deref());
                    if shown.held {
                        action = roll_actions(ui, rolled);
                    }
                }
                if let Some(call) = &shown.call {
                    action = call_lines(ui, call);
                }
            });
        });
    let pressed_outside = ctx.input(|i| i.pointer.any_pressed())
        && !response.response.contains_pointer()
        && !response.response.hovered();
    if pressed_outside && !shown.held {
        close_result(ctx);
    }
    action
}

/// `Roll damage` after an attack. Damage itself goes into Amount, so it has no action.
fn roll_actions(ui: &mut egui::Ui, rolled: &Rolled) -> Option<Action> {
    if !rolled.attack() || rolled.damage().is_empty() {
        return None;
    }
    let word = if rolled.natural == Some(20) {
        "Roll critical damage"
    } else {
        "Roll damage"
    };
    design::Action::secondary(word)
        .density(design::Density::Read)
        .hint("Roll this attack's damage into Amount. D also does it.")
        .show(ui)
        .clicked()
        .then_some(Action::RollDamage)
}

/// A call's card: the save, how to choose targets, and the steps that can follow.
fn call_lines(ui: &mut egui::Ui, call: &CallCard) -> Option<Action> {
    let c = colors();
    ui.label(
        Type::Small
            .rich(ui, format!("{} · {}", call.who, call.label))
            .color(c.muted),
    );
    ui.label(
        Type::Heading
            .rich(ui, format!("DC {} {} save", call.dc, call.ability))
            .color(c.text),
    );
    let targets = match call.targets {
        0 => "Click a row to make it a target.".to_owned(),
        1 => "1 target. Click more rows to add them.".to_owned(),
        n => format!("{n} targets. Click a row to add or remove it."),
    };
    ui.add(egui::Label::new(Type::Small.rich(ui, targets).color(c.muted)).wrap());
    let mut action = None;
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = design::S;
        for (shown, word, hint, picked) in [
            (
                call.can_roll,
                "Roll saves",
                "Roll each creature target's save. Results show on their rows.",
                Action::RollSaves,
            ),
            (
                call.can_half,
                "Half for those at or above DC",
                "Mark the targets whose save is at or above the DC to take half.",
                Action::HalfMade,
            ),
            (
                call.damage && call.targets > 0,
                "Roll damage",
                "Roll the damage once for every target, into Amount. D also does it.",
                Action::RollDamage,
            ),
        ] {
            if shown
                && design::Action::secondary(word)
                    .density(design::Density::Read)
                    .hint(hint)
                    .show(ui)
                    .clicked()
            {
                action = Some(picked);
            }
        }
    });
    action
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
        assert!(rolled.feature_damage());
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
        assert_eq!(
            rolled.natural,
            Some(10),
            "the bonus die is never the natural d20"
        );
        assert!(
            rolled.line().contains("1d20 + 4 + 1d4"),
            "{}",
            rolled.line()
        );
        let flat = roll_with(
            "Guard",
            &attack(),
            Mode::Normal,
            0,
            Some("-2"),
            &mut Random::controlled([10]),
        )
        .unwrap();
        assert_eq!(flat.total, 12);
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
