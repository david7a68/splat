//! The run's turn heading (mechanism M5, ruling R22): `Round 2`, the current turn's name,
//! `Add` and `Next turn` on one line, fitted and painted by this one part.
//!
//! A caller gives data: the run's name, the round, the current turn's whole name (a
//! [`TurnName`], made from the participant's own name field), the run's scale,
//! whether `Add` is selected, why `Next turn` is blocked, and whether the keys work. It gets
//! back presses. It cannot measure the controls, paint the heading's words, or give a width:
//! the part reads each control's width from the same [`Action`] that it paints, finds the
//! name's real room between the round and `Add`, chooses ONE complete candidate (the words
//! and the controls together) from a private ordered list, and paints that candidate.
//!
//! **Key policy of a name line (R22).** In a line that holds a name, the key letter `N` gives
//! up its room before the name is cut: a candidate with the letter is tried only while every
//! name of the roster is whole beside it at the least size, so pressing `Next turn` never
//! changes the letter and no control moves under the pointer (G1b). With the keyboard elsewhere (Lookup open) the letter is never a
//! candidate. A line with no name keeps its controls' places by its own layout (the dock's keys are
//! plain `Action` keys).
//!
//! **Kept result.** [`KeptTurnHeading`] holds the one chosen candidate, its galleys and its
//! words. Its owner is the run view (`Run::turn_heading`, made with the run); it ends with the
//! run. A frame whose data and metrics (the heading's width, the pixels per point, the fonts of
//! both control forms, the theme's font generation, the allowed name sizes and the colours)
//! equal the kept key builds no string and measures no candidate in this part. Any change of
//! one of them chooses again. (The run's second header line and its `Strip` are outside this
//! claim.)
//!
//! **The name is never wholly hidden (lead's ruling 1).** Every shortened candidate keeps at
//! least the first letter of the name and its copy mark (`T… 12`, never `… 12`). Where not even
//! that fits, no candidate fits: the part paints that least form clipped to its room. The run
//! list keeps 454 px (`RUN_LIST_KEEPS`), and there a candidate always fits (the unit sweep
//! proves it for a three-digit round, the large scale and a long numbered name).
//!
//! The type proves construction and paint ownership (rung 1). It does not prove that the
//! glyphs are inside the room; the run's visual checks do (rung 4).
use std::sync::Arc;

use egui::{
    Align, Color32, FontId, Galley, Layout, Rect, Sense, Ui, UiBuilder, WidgetInfo, WidgetType,
    pos2, vec2,
};

use super::{
    Action, Density, HAIRLINE_WIDTH, KeyName, RunScale, S, Type, colors, icons, named,
    number_width, words_that_fit,
};

/// From this width of a run's line, `Add` reads `Add to fight`; below it, `Add`. The dock
/// below the roster uses the same width for its own words (`DOCK_ALL_WORDS_FROM`).
pub const ADD_WORDS_FROM: f32 = 500.0;

/// The gap between the heading's parts: the round, the name, `Add` and `Next turn`. As before
/// the part (`S` and the line's item spacing, `S`), so the line keeps its look.
const GAP: f32 = 2.0 * S;

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

/// The current turn's whole name, as the run holds it: made from the participant's own name
/// field, never from words a caller cut. Outside `src/design/`, its maker is counted by the
/// lint (`heading-owner`, one use), so a second maker is a visible change.
#[derive(Clone, Copy)]
pub struct TurnName<'a> {
    name: &'a str,
    roster: RosterNames<'a>,
    revision: RosterRevision,
}

/// Every name that can have the turn: a visitor over the participants' own name fields, in the
/// run's order. The key letter's choice reads all of them (see `TurnName::whole`).
pub type RosterNames<'a> = &'a dyn Fn(&mut dyn FnMut(&str));

/// The run's state revision (`Run::revision`: every step, Undo and sheet sync) and the roster's
/// length: fixed size, so an unchanged frame compares it without walking the roster.
pub type RosterRevision = (u64, usize);

impl<'a> TurnName<'a> {
    /// The current participant's name field itself (no current turn is `None`), the roster it
    /// is one of (absent participants too), and the run's state `revision`, which changes at
    /// every step, Undo and sheet sync (`Run::keep_step`, `Run::take_back`). `N` shows only
    /// while EVERY roster name fits whole beside it (a companion rule of R22), so pressing
    /// `Next turn` never brings the letter or takes it away: neither `Next turn` nor `Add`
    /// moves under the DM's pointer (G1b), and a cut name still never stands beside `N`. Its
    /// cost: one long name hides `N` for the whole roster at that width. A roster edit (a
    /// join, a rename, a copy, their Undo) may change the letter on the next draw (lead's
    /// ruling 2). The part walks the roster only when it fits again, never on a frame whose
    /// key is unchanged.
    pub fn whole(
        name: Option<&'a String>,
        roster: RosterNames<'a>,
        revision: RosterRevision,
    ) -> Self {
        Self {
            name: name.map_or("", String::as_str),
            roster,
            revision,
        }
    }
}

/// The run's turn heading. Make it with [`TurnHeading::new`], which takes every input; show it
/// with [`TurnHeading::show`].
#[must_use = "call show"]
pub struct TurnHeading<'a> {
    run: &'a str,
    round: u64,
    name: &'a str,
    roster: RosterNames<'a>,
    revision: RosterRevision,
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
        name: TurnName<'a>,
        scale: RunScale,
        add_selected: bool,
        next_blocked: Option<&'a str>,
        keys: TurnKeys,
    ) -> Self {
        Self {
            run,
            round,
            name: name.name,
            roster: name.roster,
            revision: name.revision,
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
/// header uses it too (`Run::add_to_fight`), so both say the same.
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

/// The run view's one kept heading. See the module's words for its key and its end. It has no
/// `Default`: the run makes it once with [`KeptTurnHeading::new`] (counted outside
/// `src/design/` by the lint's `heading-owner` rule), never a draw path.
pub struct KeptTurnHeading {
    line: Option<Kept>,
    scans: u64,
}

impl KeptTurnHeading {
    #[allow(
        clippy::new_without_default,
        reason = "the source gate counts each kept heading constructor"
    )]
    pub fn new() -> Self {
        Self {
            line: None,
            scans: 0,
        }
    }

    /// How many times the part chose a candidate: once per change of the data or the metrics.
    #[cfg(any(test, feature = "test-support"))]
    pub fn scans(&self) -> u64 {
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

    /// Whether the kept candidate fits its room (false: the clipped least form).
    #[cfg(test)]
    pub(crate) fn fits(&self) -> bool {
        self.line.as_ref().is_some_and(|line| line.fitted.fits)
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
    /// The theme's font generation: font data replaced under the same `FontId` (E1).
    fonts: u64,
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
            fonts: super::theme::font_generation(ui.ctx()),
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
    /// The run's state revision: a roster that changes also changes it (F2).
    revision: RosterRevision,
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
            revision: heading.revision,
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
            && self.revision == heading.revision
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
    /// False: no candidate fits, and the least form is painted clipped.
    fits: bool,
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
    /// The first word's letters that fit, then `…` and the copy mark: only when not even one
    /// whole word fits. Fails when not even one letter fits.
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

/// The ordered selector (`design::fitting`), shared with the plan row's recipe.
use super::fitting as select;

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
        metrics.width
            - next_width(letter)
            - GAP
            - add_width
            - GAP
            - round.size().x
            - GAP
            - HAIRLINE_WIDTH
    };
    let sized = |size: f32| {
        let mut font = metrics.name.clone();
        font.size = size;
        font
    };
    // A whole text from the largest size down: the largest size at which it is inside `room`,
    // if that is not below the least. The proportional estimate only picks where to start;
    // the least size is always measured exactly, so an estimate never skips a fit.
    let scaled = |text: &str, room: f32| -> Option<(String, f32)> {
        let width = |size: f32| number_width(ui, text, &sized(size));
        let estimate = metrics.turn * room / width(metrics.turn).max(1.0);
        let mut size = (estimate + 0.5).min(metrics.turn);
        while size > metrics.least {
            if width(size) <= room {
                return Some((text.to_owned(), size));
            }
            size -= 0.25;
        }
        (width(metrics.least) <= room).then(|| (text.to_owned(), metrics.least))
    };
    let least = sized(metrics.least);
    // The letter may show only while every roster name fits whole beside it (G1b, R22). This
    // decision measures the round as at least three of the font's widest digits, so the round
    // 9 → 10 and 99 → 100 does not change it (lead's ruling 1); from round 1000 the real label
    // is wider, and the letter may change once there (the named overflow).
    let roster_fits = {
        let widest = ('0'..='9')
            .map(|digit| number_width(ui, &digit.to_string(), &metrics.round))
            .fold(0.0, f32::max);
        let allowance = number_width(ui, "Round ", &metrics.round) + 3.0 * widest;
        let room = room(Letter::Shown) + round.size().x - round.size().x.max(allowance);
        let mut fits = true;
        (heading.roster)(&mut |name| fits &= number_width(ui, name, &least) <= room);
        fits
    };
    let inside = |text: String, room: f32| {
        (number_width(ui, &text, &least) <= room).then_some((text, metrics.least))
    };
    let chosen = select::first_fit(
        ORDER,
        |candidate| {
            candidate.keys.is_none_or(|keys| keys == heading.keys)
                && (candidate.letter == Letter::Released || roster_fits)
        },
        |candidate| {
            let room = room(candidate.letter);
            match candidate.words {
                Words::Turn => scaled(&full, room),
                Words::Name => scaled(heading.name, room),
                // Ruling 1: a shortened name starts with its own letters (`… 12` is no name).
                Words::Short => Some(whole_words_name(ui, heading.name, &least, room))
                    .filter(|short| !short.starts_with('…'))
                    .and_then(|short| inside(short, room)),
                Words::Letters => {
                    letters(heading.name, |cut| number_width(ui, cut, &least) <= room)
                        .and_then(|cut| inside(cut, room))
                }
            }
        },
    );
    let fits = chosen.is_some();
    let (chosen, (text, size)) = chosen.unwrap_or_else(|| {
        // No candidate fits: the least form (one letter and the copy mark), clipped to its
        // room. Narrower than the run list keeps (454 px) only.
        let least_form = letters(heading.name, |cut| {
            cut.chars().take_while(|c| *c != '…').count() <= 1
        })
        .unwrap_or_default();
        (ORDER[ORDER.len() - 1], (least_form, metrics.least))
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
        fits,
    }
}

fn layout(ui: &Ui, text: String, font: FontId, color: Color32) -> Arc<Galley> {
    ui.fonts_mut(|fonts| fonts.layout_no_wrap(text, font, color))
}

/// The most letters of the name's first word that `fit` allows, then `…`, then the copy mark
/// of a numbered name (`Seraph… 12`, `S… 12`). At least one letter: `None` when not even that
/// fits. Only for a name whose first word is wider than its room (R15's lone word).
fn letters(name: &str, fit: impl Fn(&str) -> bool) -> Option<String> {
    let words: Vec<&str> = name.split_whitespace().collect();
    let first: Vec<char> = words.first().copied().unwrap_or(name).chars().collect();
    let mark = words
        .last()
        .filter(|last| {
            words.len() > 1
                && (last.chars().all(|c| c.is_ascii_digit()) || last.chars().count() == 1)
        })
        .map_or_else(String::new, |last| format!(" {last}"));
    (1..=first.len())
        .rev()
        .map(|count| format!("{}…{mark}", first[..count].iter().collect::<String>()))
        .find(|cut| fit(cut))
}

/// The run strip's line on the other pages (M0274, M0275), fitted as one candidate as the turn
/// heading is. Its title leads (`Round 3`, or the run's state and name while it is not in
/// combat), then the current turn (`Kaya’s turn`, in the turn colour) and the next (`Next:
/// Wolf`, in the next colour); `end` draws the actions and statuses that end the line, right to
/// left. The next goes first; then the current turn's name gives up whole words, then letters,
/// never all of them; a title that is still too wide gives up whole words; the end never
/// shortens. A shortened piece keeps its whole words as its accessible name and its hover. The
/// line measures its end with the last frame's width (a new width draws the frame again), so
/// the line reads in its order, the title first. The host draws the band around it (its panel,
/// fill and height).
#[must_use = "call show"]
pub struct TurnStrip<'a> {
    title: &'a str,
    turn: Option<&'a str>,
    next: Option<&'a str>,
}

impl<'a> TurnStrip<'a> {
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            turn: None,
            next: None,
        }
    }

    /// Whose turn it is: its name, as `<name>’s turn`.
    pub fn turn(mut self, name: Option<&'a str>) -> Self {
        self.turn = name;
        self
    }

    /// Who comes next: its name, as `Next: <name>`.
    pub fn next(mut self, name: Option<&'a str>) -> Self {
        self.next = name;
        self
    }

    /// Draw the line in `ui` (the band), its end drawn by `end`, right to left.
    pub fn show(self, ui: &mut Ui, end: impl FnOnce(&mut Ui)) {
        let c = colors();
        let key = ui.id().with("turn_strip_end");
        let end_width = ui.data(|data| data.get_temp::<f32>(key)).unwrap_or(0.0);
        let gap = ui.spacing().item_spacing.x;
        let room = (ui.available_width() - end_width - super::L).max(0.0);
        let (heading, name, small) = (
            Type::Heading.font(ui),
            Type::Name.font(ui),
            Type::Small.font(ui),
        );
        let title_width = number_width(ui, self.title, &heading);
        let title = if title_width <= room {
            self.title.to_owned()
        } else {
            words_that_fit(ui, self.title, &heading, room)
        };
        let after_title = (room - number_width(ui, &title, &heading) - gap).max(0.0);
        let turn = self.turn.map(|who| {
            let whole = format!("{who}\u{2019}s turn");
            let tail = number_width(ui, "\u{2019}s turn", &name);
            let shown = if number_width(ui, &whole, &name) <= after_title {
                whole.clone()
            } else {
                let fitted = whole_words_name(ui, who, &name, after_title - tail);
                if fitted.chars().take_while(|c| *c != '\u{2026}').count() > 0
                    && number_width(ui, &fitted, &name) + tail <= after_title
                {
                    format!("{fitted}\u{2019}s turn")
                } else {
                    // Letters, never all of them: at least the first and the copy mark.
                    let fit = |cut: &str| number_width(ui, cut, &name) + tail <= after_title;
                    let letters = letters(who, fit).unwrap_or_else(|| {
                        letters(who, |cut| {
                            cut.chars().take_while(|c| *c != '\u{2026}').count() <= 1
                        })
                        .unwrap_or_default()
                    });
                    format!("{letters}\u{2019}s turn")
                }
            };
            (whole, shown)
        });
        let after_turn = turn.as_ref().map_or(after_title, |(_, shown)| {
            after_title - number_width(ui, shown, &name) - gap
        });
        let next = self
            .next
            .map(|who| format!("Next: {who}"))
            .filter(|_| turn.as_ref().is_none_or(|(whole, shown)| whole == shown))
            .filter(|words| number_width(ui, words, &small) <= after_turn);
        let piece = |ui: &mut Ui, shown: &str, whole: &str, font: Type, color: Color32| {
            let response = ui.label(font.rich(ui, shown).color(color));
            if shown != whole {
                named(ui, &response, whole);
                let _ = response.on_hover_text(whole);
            }
        };
        ui.horizontal_centered(|ui| {
            if title == self.title {
                super::heading(ui, super::Heading::Panel(self.title));
            } else {
                piece(ui, &title, self.title, Type::Heading, c.text);
            }
            if let Some((whole, shown)) = &turn {
                piece(ui, shown, whole, Type::Name, c.turn.current);
            }
            if let Some(next) = &next {
                piece(ui, next, next, Type::Small, c.turn.next);
            }
            let used = ui
                .with_layout(Layout::right_to_left(Align::Center), end)
                .response
                .rect
                .width();
            if (used - end_width).abs() > 0.5 {
                ui.data_mut(|data| data.insert_temp(key, used));
                ui.ctx().request_discard("the run strip measured its end");
            }
        });
    }
}

/// Whole words of a creature's name that fit in `width`, then `…`. A name does not end on a
/// joining word, and a copy number or letter stays after the `…` (`Ember Cultist… 12`). The
/// Legacy users outside the part (counted by the lint's `fitting-escape` rule): the run's
/// `dock_name`, for the dock's conflict words and the run's Undo.
pub fn whole_words_name(ui: &Ui, name: &str, font: &FontId, width: f32) -> String {
    whole_words_name_by(name, width, |words| number_width(ui, words, font))
}

/// [`whole_words_name`] with any measure of a line's width (a view that does not draw with
/// egui).
pub fn whole_words_name_by(name: &str, width: f32, measure: impl Fn(&str) -> f32) -> String {
    let fitted = super::components::words_that_fit_by(name, width, &measure);
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
        while words.len() > 1 && measure(&format!("{}…", words.join(" "))) > width {
            words.pop();
        }
        return format!("{}…", words.join(" "));
    }
    let prefix_room = width - measure(&format!("… {last}"));
    let prefix = super::components::words_that_fit_by(name, prefix_room, &measure);
    let prefix = prefix.trim_end_matches('…');
    let candidate = format!("{prefix}… {last}");
    if measure(&candidate) <= width {
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
            let add_right = next_left - GAP;
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
                pos2(round.right() + GAP, area.top()),
                pos2(add_left - GAP - HAIRLINE_WIDTH, area.bottom()),
            );
            let name = Rect::from_min_size(
                pos2(room.left(), middle - fitted.name.size().y / 2.0),
                fitted.name.size(),
            );
            // A candidate that fits is inside `room` by its measure; only the failure form (a
            // heading narrower than the run list keeps) is clipped there, off `Add`.
            let clip = if fitted.fits {
                ui.clip_rect()
            } else {
                room.intersect(ui.clip_rect())
            };
            ui.painter().with_clip_rect(clip).galley(
                name.min,
                fitted.name.clone(),
                metrics.current,
            );
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
