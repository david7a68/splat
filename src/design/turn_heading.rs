//! The run's turn heading (mechanism M5, ruling R22): `Round 2`, the current turn's name,
//! `Add` and `Next turn` on one line, fitted and painted by this one part.
//!
//! A caller gives data: the run's name, the round, the full current name, the run's scale,
//! whether `Add` is selected, why `Next turn` is blocked, and whether the keys work. It gets
//! back presses. It cannot measure the controls, paint the heading's words, or give a width:
//! the part reads each control's width from the same [`Action`] that it paints, finds the
//! name's real room between the round and `Add`, chooses ONE complete candidate (the words
//! and the controls together) from a private ordered list, and paints that candidate.
//!
//! **Key policy of a name line (R22).** In a line that holds a name, the key letter `N` gives
//! up its room before the name is cut: a candidate with the letter is tried only while the
//! name is whole beside it. With the keyboard elsewhere (Lookup open) the letter is never a
//! candidate. A line with no name (the dock) keeps a fixed slot: [`super::KeySlot`].
//!
//! **Kept result.** [`KeptTurnHeading`] holds the one chosen candidate, its galleys and its
//! words. Its owner is the run view (`Run::turn_heading`); it ends with the run. A frame whose
//! data and metrics (the heading's width, the pixels per point, the fonts of both control
//! forms, the allowed name sizes and the colours) equal the kept key builds no string and
//! measures no candidate. Any change of one of them chooses again.
//!
//! The type proves construction and paint ownership (rung 1). It does not prove that the
//! glyphs are inside the room; the run's visual checks do (rung 4).
use std::sync::Arc;

use eframe::egui::{
    Align, Color32, FontId, Galley, Layout, Rect, Sense, Ui, UiBuilder, WidgetInfo, WidgetType,
    pos2, vec2,
};

use super::{
    Action, Density, HAIRLINE_WIDTH, KeyName, RunScale, S, Type, colors, icons, named,
    number_width, words_that_fit,
};

/// From this width of a run's line, `Add` reads `Add to fight`; below it, `Add`. The dock
/// below the roster uses the same width for its own words.
pub const ADD_WORDS_FROM: f32 = 500.0;

/// `Next turn`'s key, as the Keys list names it.
const NEXT_KEY: KeyName = crate::key!("N");

/// Whether the run's keys work while the heading shows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TurnKeys {
    /// The run has the keyboard: `N` works, and its letter may show.
    Work,
    /// Another part has the keyboard (Lookup): the letter shows nowhere on the line.
    Elsewhere,
}

/// What the DM pressed in the heading this frame.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[must_use]
pub struct TurnPresses {
    pub next_turn: bool,
    pub add: bool,
}

/// The run's turn heading. Make it with [`TurnHeading::new`], which takes every input; show it
/// with [`TurnHeading::show`].
#[must_use = "call show"]
pub struct TurnHeading<'a> {
    run: &'a str,
    round: u64,
    name: &'a str,
    scale: RunScale,
    add_selected: bool,
    next_blocked: Option<&'a str>,
    keys: TurnKeys,
}

impl<'a> TurnHeading<'a> {
    /// `run` is the run's name and `name` the current turn's whole name; the part shortens it.
    /// `round` is the live round (1 or more). `next_blocked` is why `Next turn` cannot be
    /// pressed, if it cannot.
    pub fn new(
        run: &'a str,
        round: u64,
        name: &'a str,
        scale: RunScale,
        add_selected: bool,
        next_blocked: Option<&'a str>,
        keys: TurnKeys,
    ) -> Self {
        Self {
            run,
            round,
            name,
            scale,
            add_selected,
            next_blocked,
            keys,
        }
    }

    /// The heading's height at this scale: it grows with the rows.
    pub fn height(scale: RunScale) -> f32 {
        (scale.hp * 1.6).max(36.0)
    }

    /// Fit (only when the data or metrics changed) and paint the heading in the full width
    /// left in `ui`.
    pub fn show(self, ui: &mut Ui, kept: &mut KeptTurnHeading) -> TurnPresses {
        let metrics = Metrics::of(ui, self.scale);
        if !kept
            .line
            .as_ref()
            .is_some_and(|line| line.key.matches(&self, &metrics))
        {
            kept.scans += 1;
            let fitted = fit(ui, &self, &metrics);
            kept.line = Some(Kept {
                key: Key::of(&self, metrics),
                fitted,
            });
        }
        let Some(line) = kept.line.as_ref() else {
            return TurnPresses::default();
        };
        paint(ui, &self, &line.key.metrics, &line.fitted)
    }
}

/// `Add to fight`, which opens Arrivals: `Add` with the icon on a narrow line. The run's setup
/// header uses it too, so both say the same.
pub fn add_to_fight(words: bool, density: Density, selected: bool) -> Action<'static> {
    let action = if words {
        Action::secondary("Add to fight").icon(icons::ADD)
    } else {
        Action::secondary("Add").icon(icons::ADD)
    };
    action
        .name("Add to fight")
        .hint("Add an encounter group or a creature (Ctrl+G)")
        .density(density)
        .selected(selected)
}

fn next_turn(letter: Letter, blocked: Option<&str>) -> Action<'_> {
    let action = Action::primary("Next turn")
        .icon(icons::NEXT)
        .density(Density::Glance)
        .blocked(blocked);
    match letter {
        Letter::Shown => action.key(NEXT_KEY.as_str()),
        Letter::Released => action,
    }
}

/// The run view's one kept heading. See the module's words for its key and its end.
#[derive(Default)]
pub struct KeptTurnHeading {
    line: Option<Kept>,
    scans: u64,
}

impl KeptTurnHeading {
    /// How many times the part chose a candidate: once per change of the data or the metrics.
    #[cfg(test)]
    pub(crate) fn scans(&self) -> u64 {
        self.scans
    }

    /// The words the heading paints for the name, and whether `N` shows (for tests).
    #[cfg(test)]
    pub(crate) fn shown(&self) -> Option<(&str, bool)> {
        self.line.as_ref().map(|line| {
            (
                line.fitted.name.job.text.as_str(),
                line.fitted.letter == Letter::Shown,
            )
        })
    }
}

struct Kept {
    key: Key,
    fitted: Fitted,
}

/// Everything outside the data that the chosen candidate depends on.
#[derive(Clone, PartialEq)]
struct Metrics {
    width: f32,
    pixels_per_point: f32,
    /// The largest and the least name size.
    turn: f32,
    least: f32,
    /// `Next turn`'s words, its key letter, `Add`'s words, the round, and the name.
    heading: FontId,
    small: FontId,
    strong: FontId,
    round: FontId,
    name: FontId,
    text: Color32,
    current: Color32,
}

impl Metrics {
    fn of(ui: &Ui, scale: RunScale) -> Self {
        // As large as the largest HP, in the row's own face: nothing outweighs the header.
        let turn = scale.hp;
        let c = colors();
        Self {
            width: ui.available_width(),
            pixels_per_point: ui.ctx().pixels_per_point(),
            turn,
            least: (scale.name + HAIRLINE_WIDTH).min(turn),
            heading: Type::Heading.font(ui),
            small: Type::Small.font(ui),
            strong: Type::Strong.font(ui),
            round: Type::Round.sized(ui, turn),
            name: Type::Name.font(ui),
            text: c.text,
            current: c.turn.current,
        }
    }
}

struct Key {
    run: String,
    round: u64,
    name: String,
    keys: TurnKeys,
    scale: RunScale,
    metrics: Metrics,
}

impl Key {
    fn of(heading: &TurnHeading<'_>, metrics: Metrics) -> Self {
        Self {
            run: heading.run.to_owned(),
            round: heading.round,
            name: heading.name.to_owned(),
            keys: heading.keys,
            scale: heading.scale,
            metrics,
        }
    }

    /// Compares in place: no string is made.
    fn matches(&self, heading: &TurnHeading<'_>, metrics: &Metrics) -> bool {
        self.run == heading.run
            && self.round == heading.round
            && self.name == heading.name
            && self.keys == heading.keys
            && self.scale == heading.scale
            && self.metrics == *metrics
    }
}

/// The chosen candidate, ready to paint.
struct Fitted {
    letter: Letter,
    add_words: bool,
    next_width: f32,
    add_width: f32,
    round: Arc<Galley>,
    name: Arc<Galley>,
    /// `Run · Ember Road · Round 2`, the round's accessible name.
    access: String,
    /// `Iselle Marrowwind’s turn`, the name's hover and accessible name.
    full: String,
}

/// Which words of the name a candidate paints.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Words {
    /// `Iselle Marrowwind’s turn`, from the largest size down to the least.
    Turn,
    /// `Iselle Marrowwind`, from the largest size down to the least.
    Name,
    /// Whole words at the least size, then `…` (`Iselle…`, `Ember Cultist… 12`).
    Short,
    /// The first word's letters that fit, then `…`: only when not even one whole word fits.
    Letters,
}

/// Whether `Next turn` shows its key letter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Letter {
    Shown,
    /// The letter and its room are gone; `Next turn` is narrower by them.
    Released,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Candidate {
    words: Words,
    letter: Letter,
    /// The key availability this candidate needs; `None` for any.
    keys: Option<TurnKeys>,
}

const fn candidate(words: Words, letter: Letter, keys: Option<TurnKeys>) -> Candidate {
    Candidate {
        words,
        letter,
        keys,
    }
}

/// R22's order. The suffix gives way first, then the key letter, then the name's words. A
/// letter needs working keys; with the keys elsewhere, the suffix comes back as the first
/// candidate without a letter.
const ORDER: [Candidate; 6] = [
    candidate(Words::Turn, Letter::Shown, Some(TurnKeys::Work)),
    candidate(Words::Name, Letter::Shown, Some(TurnKeys::Work)),
    candidate(Words::Turn, Letter::Released, Some(TurnKeys::Elsewhere)),
    candidate(Words::Name, Letter::Released, None),
    candidate(Words::Short, Letter::Released, None),
    candidate(Words::Letters, Letter::Released, None),
];

/// The private ordered selector.
mod select {
    /// The first candidate of a closed `order` that `legal` allows and `measure` finds a fit
    /// for, with that fit. Linear and in order: each legal candidate is measured at most once,
    /// and never by bisection, because a later candidate can be wider than an earlier one.
    pub(super) fn first_fit<C: Copy, R>(
        order: &[C],
        legal: impl Fn(C) -> bool,
        mut measure: impl FnMut(C) -> Option<R>,
    ) -> Option<(C, R)> {
        order
            .iter()
            .copied()
            .filter(|candidate| legal(*candidate))
            .find_map(|candidate| measure(candidate).map(|fit| (candidate, fit)))
    }
}

/// Choose the heading's one candidate, on a change only.
fn fit(ui: &Ui, heading: &TurnHeading<'_>, metrics: &Metrics) -> Fitted {
    let add_words = metrics.width >= ADD_WORDS_FROM;
    let add_width = add_to_fight(add_words, Density::Glance, false)
        .natural_width(ui)
        .ceil();
    let next_width = |letter| next_turn(letter, None).natural_width(ui).ceil();
    let round = layout(
        ui,
        format!("Round {}", heading.round.max(1)),
        metrics.round.clone(),
        metrics.text,
    );
    let full = format!("{}’s turn", heading.name);
    // The name's room: between the round and `Add`, as `paint` places them.
    let room = |letter| {
        metrics.width - next_width(letter) - S - add_width - S - round.size().x - S - HAIRLINE_WIDTH
    };
    let sized = |size: f32| {
        let mut font = metrics.name.clone();
        font.size = size;
        font
    };
    // A whole text from the largest size down: the largest size at which it is inside `room`,
    // if that is not below the least.
    let scaled = |text: &str, room: f32| -> Option<(String, f32)> {
        let width = number_width(ui, text, &sized(metrics.turn));
        let mut size = (metrics.turn * room / width.max(1.0)).min(metrics.turn);
        while size >= metrics.least {
            if number_width(ui, text, &sized(size)) <= room {
                return Some((text.to_owned(), size));
            }
            size -= 0.25;
        }
        None
    };
    let least = sized(metrics.least);
    let (chosen, (text, size)) = select::first_fit(
        &ORDER,
        |candidate| candidate.keys.is_none_or(|keys| keys == heading.keys),
        |candidate| {
            let room = room(candidate.letter);
            match candidate.words {
                Words::Turn => scaled(&full, room),
                Words::Name => scaled(heading.name, room),
                Words::Short => {
                    let short = whole_words_name(ui, heading.name, &least, room);
                    (number_width(ui, &short, &least) <= room).then_some((short, metrics.least))
                }
                Words::Letters => Some((letters(ui, heading.name, &least, room), metrics.least)),
            }
        },
    )
    .unwrap_or_else(|| {
        // `Letters` always fits, so this is not reached; paint the ellipsis.
        (ORDER[ORDER.len() - 1], ("…".to_owned(), metrics.least))
    });
    let name = layout(ui, text, sized(size), metrics.current);
    Fitted {
        letter: chosen.letter,
        add_words,
        next_width: next_width(chosen.letter),
        add_width,
        round,
        name,
        access: format!("Run · {} · Round {}", heading.run, heading.round),
        full,
    }
}

fn layout(ui: &Ui, text: String, font: FontId, color: Color32) -> Arc<Galley> {
    ui.fonts_mut(|fonts| fonts.layout_no_wrap(text, font, color))
}

/// The first word's letters that fit in `width`, then `…`; `…` alone when no letter fits.
/// Only for a name whose first word is wider than its room (R15's lone word).
fn letters(ui: &Ui, name: &str, font: &FontId, width: f32) -> String {
    let first: Vec<char> = name
        .split_whitespace()
        .next()
        .unwrap_or(name)
        .chars()
        .collect();
    (1..first.len())
        .rev()
        .map(|count| format!("{}…", first[..count].iter().collect::<String>()))
        .find(|cut| number_width(ui, cut, font) <= width)
        .unwrap_or_else(|| "…".to_owned())
}

/// Whole words of a creature's name that fit in `width`, then `…`. A name does not end on a
/// joining word, and a copy number or letter stays after the `…` (`Ember Cultist… 12`). The
/// dock and the run's Undo shorten names with it too.
pub fn whole_words_name(ui: &Ui, name: &str, font: &FontId, width: f32) -> String {
    let fitted = words_that_fit(ui, name, font, width);
    if fitted == name {
        return fitted;
    }
    let fitted = fitted.trim_end_matches('…');
    let last = name.split_whitespace().last().unwrap_or(name);
    if !last.chars().all(|c| c.is_ascii_digit()) && last.chars().count() != 1 {
        let mut words: Vec<_> = fitted.split_whitespace().collect();
        while words
            .last()
            .is_some_and(|word| ["of", "the", "de", "von"].contains(word))
        {
            words.pop();
        }
        while words.len() > 1 && number_width(ui, &format!("{}…", words.join(" ")), font) > width
        {
            words.pop();
        }
        return format!("{}…", words.join(" "));
    }
    let prefix_room = width - number_width(ui, &format!("… {last}"), font);
    let prefix = words_that_fit(ui, name, font, prefix_room);
    let prefix = prefix.trim_end_matches('…');
    let candidate = format!("{prefix}… {last}");
    if number_width(ui, &candidate, font) <= width {
        candidate
    } else {
        format!("… {last}")
    }
}

/// One child `Ui` on `rect`, its widgets at its right edge.
fn at_right<R>(ui: &mut Ui, rect: Rect, add: impl FnOnce(&mut Ui) -> R) -> R {
    ui.scope_builder(
        UiBuilder::new()
            .max_rect(rect)
            .layout(Layout::right_to_left(Align::Center)),
        add,
    )
    .inner
}

/// Paint the kept candidate: `Next turn` at the right edge, `S`, `Add`, and the round and the
/// name from the left edge.
fn paint(
    ui: &mut Ui,
    heading: &TurnHeading<'_>,
    metrics: &Metrics,
    fitted: &Fitted,
) -> TurnPresses {
    let height = TurnHeading::height(heading.scale);
    let width = metrics.width;
    ui.allocate_ui_with_layout(
        vec2(width, height),
        Layout::left_to_right(Align::Center),
        |ui| {
            ui.set_min_size(vec2(width, height));
            let area = Rect::from_min_size(ui.max_rect().min, vec2(width, height));
            // One point of slack on the left of each control's child, so the control's own
            // layout never cuts its words; the control paints at the child's right edge.
            let next_left = area.right() - fitted.next_width;
            let next = at_right(ui, area.with_min_x(next_left - 1.0), |ui| {
                next_turn(fitted.letter, heading.next_blocked)
                    .min_width(fitted.next_width)
                    .show(ui)
            });
            let add_right = next_left - S;
            let add_left = add_right - fitted.add_width;
            let add = at_right(
                ui,
                Rect::from_min_max(
                    pos2(add_left - 1.0, area.top()),
                    pos2(add_right, area.bottom()),
                ),
                |ui| {
                    add_to_fight(fitted.add_words, Density::Glance, heading.add_selected)
                        .min_width(fitted.add_width)
                        .show(ui)
                },
            );
            let middle = area.center().y;
            let round = Rect::from_min_size(
                pos2(area.left(), middle - fitted.round.size().y / 2.0),
                fitted.round.size(),
            );
            ui.painter()
                .galley(round.min, fitted.round.clone(), metrics.text);
            let response = ui.interact(round, ui.id().with("turn_heading_round"), Sense::hover());
            response.widget_info(|| {
                WidgetInfo::labeled(WidgetType::Label, true, fitted.round.job.text.as_str())
            });
            named(ui, &response, &fitted.access);
            let room = Rect::from_min_max(
                pos2(round.right() + S, area.top()),
                pos2(add_left - S - HAIRLINE_WIDTH, area.bottom()),
            );
            let name = Rect::from_min_size(
                pos2(room.left(), middle - fitted.name.size().y / 2.0),
                fitted.name.size(),
            );
            // The candidate is inside `room`; the clip only keeps the last fallback's `…`
            // off `Add` in a heading too narrow for one letter.
            ui.painter()
                .with_clip_rect(room.intersect(ui.clip_rect()))
                .galley(name.min, fitted.name.clone(), metrics.current);
            let response = ui.interact(
                name.intersect(room),
                ui.id().with("turn_heading_name"),
                Sense::hover(),
            );
            response.widget_info(|| {
                WidgetInfo::labeled(WidgetType::Label, true, fitted.name.job.text.as_str())
            });
            named(ui, &response, &fitted.full);
            response.on_hover_text(&fitted.full);
            TurnPresses {
                next_turn: next.clicked(),
                add: add.clicked(),
            }
        },
    )
    .inner
}

#[cfg(test)]
mod tests;
