//! The party at a glance, above the Session page while a session is played. Each member is a
//! portrait, HP and a few numbers, with the parts the roster uses. The rest is on hover, and a
//! click opens the sheet.
use crate::design::Color;
use crate::design::{self, Density};
use splat_campaign::encounter_run::effect::Condition;
use std::path::PathBuf;
use uuid::Uuid;

/// One party member, as the bar shows them.
pub struct Glance {
    pub id: String,
    pub name: String,
    pub portrait: Option<PathBuf>,
    pub hp: Option<i64>,
    pub max_hp: Option<i64>,
    pub temp: i64,
    pub ac: Option<i64>,
    pub perception: Option<i64>,
    pub insight: Option<i64>,
    pub investigation: Option<i64>,
    /// The highest spell save DC, for a caster.
    pub spell_dc: Option<i64>,
    /// A run's entries while the member is in one, or the sheet's names as labels.
    pub conditions: Vec<Condition>,
}

/// Where the numbers come from, as a status: an icon, a word and a colour. `short` names the
/// same source in fewer words, for when the full words do not fit after the players.
pub struct Source {
    pub icon: &'static str,
    pub word: String,
    pub short: &'static str,
    pub color: Color,
}

/// The bar holds one line of items, so that what is below it does not move.
pub fn height() -> f32 {
    Density::Read.row()
}

const PORTRAIT: f32 = 24.0;

/// A number, or `?` when the sheet does not say.
pub fn known(n: Option<i64>) -> String {
    n.map_or_else(|| "?".to_owned(), |n| n.to_string())
}

impl Glance {
    /// The numbers after HP, in the order the Party list compares them. Only a caster has a DC.
    pub fn stats(&self) -> Vec<(&'static str, Option<i64>)> {
        let mut stats = vec![
            ("AC", self.ac),
            ("Per", self.perception),
            ("Ins", self.insight),
        ];
        if self.spell_dc.is_some() {
            stats.push(("DC", self.spell_dc));
        }
        stats
    }

    /// Everything the bar knows after the name, a fact to a line, for hover.
    pub fn facts(&self) -> Vec<String> {
        let mut hp = format!(
            "HP {}",
            crate::combat_sync::fraction(known(self.hp), known(self.max_hp))
        );
        if let Some((_, word, _)) = design::hp_word(self.hp, self.max_hp) {
            hp.push_str(&format!(" · {word}"));
        }
        if self.temp > 0 {
            hp.push_str(&format!(" · Temp HP {}", self.temp));
        }
        let mut armor = format!("AC {}", known(self.ac));
        if let Some(dc) = self.spell_dc {
            armor.push_str(&format!(" · Spell save DC {dc}"));
        }
        vec![
            hp,
            armor,
            format!(
                "Passive Perception {} · Insight {} · Investigation {}",
                known(self.perception),
                known(self.insight),
                known(self.investigation)
            ),
            if self.conditions.is_empty() {
                "No conditions".to_owned()
            } else {
                self.conditions
                    .iter()
                    .map(Condition::words)
                    .collect::<Vec<_>>()
                    .join(", ")
            },
        ]
    }
}

/// Which name each item draws.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Names {
    First,
    Initial,
}

/// One way to draw the bar: how many stats, with or without portraits, which names, and
/// whether the HP state keeps its word.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Step {
    pub shown: usize,
    pub portrait: bool,
    pub names: Names,
    pub state_word: bool,
}

/// From the fullest bar down. The optional stats (Per, Ins, DC) drop from the end, down to AC. Then the portraits go together, then AC.
/// HP and temporary HP always stay. Only when first names and HP do not fit do the names
/// shorten to an initial (`B.`). If that does not fit, initials stay and the HP state keeps its
/// icon and colour without its word. No item is ever cut off: when even that does not fit, the
/// items that fit show whole and `N more` names the rest. Every item shows the same parts, and
/// hover and the accessible name keep everything.
pub fn steps(most: usize) -> Vec<Step> {
    let step = |shown, portrait, names, state_word| Step {
        shown,
        portrait,
        names,
        state_word,
    };
    let mut steps: Vec<Step> = (most.min(1)..=most)
        .rev()
        .map(|shown| step(shown, true, Names::First, true))
        .collect();
    steps.extend([
        step(most.min(1), false, Names::First, true),
        step(0, false, Names::First, true),
        step(0, false, Names::Initial, true),
        step(0, false, Names::Initial, false),
    ]);
    steps
}

/// What a fight holds from frame to frame, in the bar's own state, keyed to the run. The level
/// may drop when the words need it, but it never rises, and no item narrows, until the fight
/// ends. A new bar width or a new party measures again.
#[derive(Clone, Debug, PartialEq)]
pub struct Hold {
    run: Uuid,
    width: f32,
    party: Vec<String>,
    steps: usize,
    step: usize,
    kept: usize,
    widths: Vec<f32>,
}

/// One item's parts, measured by whichever toolkit draws the bar: each width in points. The fit
/// ([`fit`]) chooses the parts every item draws.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Measured {
    /// The first name, and its initial (`B.`).
    pub first: Option<f32>,
    pub initial: Option<f32>,
    /// The HP readout's slots: the current HP and a slash with three digits of maximum.
    pub hp: f32,
    /// `+5`, when the member has temporary HP.
    pub temp: Option<f32>,
    /// The HP state's icon and word, and its icon alone.
    pub full_word: Option<f32>,
    pub icon_word: Option<f32>,
    /// Each stat as one run: its label, a space, and its value.
    pub stats: Vec<f32>,
}

impl Measured {
    /// The item's width when drawn as `step` says.
    pub fn width(&self, step: Step) -> f32 {
        let name = match step.names {
            Names::First => self.first,
            Names::Initial => self.initial,
        };
        let word = if step.state_word {
            self.full_word
        } else {
            self.icon_word
        };
        design::XS
            + if step.portrait {
                PORTRAIT + design::XS
            } else {
                0.0
            }
            + name.map_or(0.0, |w| w + design::S)
            + self.hp
            + self.temp.map_or(0.0, |w| design::XS + w)
            + word.map_or(0.0, |w| design::S + w)
            + self
                .stats
                .iter()
                .take(step.shown)
                .map(|w| design::S + w)
                .sum::<f32>()
            + design::XS
    }
}

/// The room the items have, and what the hold of a fight is keyed to.
pub struct Room {
    /// The width the items may take.
    pub room: f32,
    /// The gap between two items.
    pub gap: f32,
    /// The bar's whole width: a new width measures again.
    pub bar_width: f32,
    /// The run being fought, if any.
    pub fight: Option<Uuid>,
    /// The members' ids, in order: a new party measures again.
    pub party: Vec<String>,
}

/// How the bar is drawn: the step, each item's width, how many items show whole, and what a
/// fight holds for the next frame (`None` outside a fight).
#[derive(Clone, Debug, PartialEq)]
pub struct Fitted {
    pub step: Step,
    pub widths: Vec<f32>,
    pub kept: usize,
    pub hold: Option<Hold>,
}

/// Fit the items to the room (see [`steps`]). No item is ever cut off: when even the narrowest
/// step does not fit, the items that fit show whole and `N more` (`more(hidden)` wide) names the
/// rest. In a fight (`room.fight`), `held` is the last frame's hold: the level never rises, and
/// at the same level no item narrows, so nothing after an item moves back while HP changes.
pub fn fit(
    items: &[Measured],
    room: Room,
    more: impl Fn(usize) -> f32,
    held: Option<&Hold>,
) -> Fitted {
    let Room {
        room,
        gap,
        bar_width,
        fight,
        party,
    } = room;
    let steps = steps(items.iter().map(|item| item.stats.len()).max().unwrap_or(0));
    let held = fight.and_then(|run| {
        held.filter(|hold| {
            hold.run == run
                && hold.width == bar_width
                && hold.party == party
                && hold.steps == steps.len()
                && hold.widths.len() == items.len()
        })
    });
    let needed = |step: Step| {
        items.iter().map(|item| item.width(step)).sum::<f32>()
            + gap * items.len().saturating_sub(1) as f32
    };
    let fitted = steps
        .iter()
        .position(|&step| needed(step) <= room)
        .unwrap_or_else(|| {
            // Below every step, the narrowest one, so that the most items show whole.
            (0..steps.len())
                .min_by(|&a, &b| needed(steps[a]).total_cmp(&needed(steps[b])))
                .unwrap_or(0)
        });
    // How many items show whole, from the first, with these widths.
    let fit = |widths: &[f32]| {
        (0..=widths.len())
            .rev()
            .find(|&kept| {
                let shown =
                    widths[..kept].iter().sum::<f32>() + gap * kept.saturating_sub(1) as f32;
                let hidden = widths.len() - kept;
                let count = if hidden == 0 {
                    0.0
                } else {
                    gap + more(hidden) + 2.0 * design::XS
                };
                shown + count <= room
            })
            .unwrap_or(0)
    };
    // Each item's width at this step, with or without the widths a fight held.
    let measure = |index: usize, hold: Option<&Hold>| {
        items
            .iter()
            .enumerate()
            .map(|(at, item)| {
                hold.map_or(0.0, |hold| hold.widths[at])
                    .max(item.width(steps[index]))
            })
            .collect::<Vec<f32>>()
    };
    let fresh = fit(&measure(fitted, None));
    // In a fight the level never rises, and at the same level no item narrows. The held widths
    // are measured again at the held level each frame. When they would hide a player who shows
    // without the hold, they are dropped, and the held level stays: its items are no wider than
    // a fuller level's, so the same players fit.
    let (index, widths, kept) = match held {
        Some(hold) if fitted <= hold.step => {
            let widths = measure(hold.step, Some(hold));
            let kept = fit(&widths);
            if kept >= fresh {
                (hold.step, widths, kept.min(hold.kept).max(fresh))
            } else {
                let widths = measure(hold.step, None);
                let kept = fit(&widths);
                (hold.step, widths, kept)
            }
        }
        _ => (fitted, measure(fitted, None), fresh),
    };
    let hold = fight.map(|run| Hold {
        run,
        width: bar_width,
        party,
        steps: steps.len(),
        step: index,
        kept,
        widths: widths.clone(),
    });
    Fitted {
        step: steps[index],
        widths,
        kept,
        hold,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// One member's measured parts: Bloodied members have the state's word.
    fn member(bloodied: bool) -> Measured {
        Measured {
            first: Some(60.0),
            initial: Some(14.0),
            hp: 40.0,
            temp: None,
            full_word: bloodied.then_some(50.0),
            icon_word: bloodied.then_some(12.0),
            // AC, Per, Ins, DC.
            stats: vec![34.0, 38.0, 36.0, 32.0],
        }
    }

    fn party(count: usize, bloodied: usize) -> Vec<Measured> {
        (0..count).map(|at| member(at < bloodied)).collect()
    }

    fn room(width: f32, fight: Option<Uuid>) -> Room {
        Room {
            room: width,
            gap: design::M,
            bar_width: width,
            fight,
            party: (0..5).map(|at| at.to_string()).collect(),
        }
    }

    /// `N more`'s width.
    fn more(_: usize) -> f32 {
        50.0
    }

    /// Where a step stands in the order, fullest first.
    fn level(fitted: &Fitted) -> usize {
        steps(4)
            .iter()
            .position(|step| *step == fitted.step)
            .unwrap()
    }

    /// The items that show and `N more` fit the room.
    fn whole(fitted: &Fitted, items: usize, width: f32) -> bool {
        let shown = fitted.widths[..fitted.kept].iter().sum::<f32>()
            + design::M * fitted.kept.saturating_sub(1) as f32;
        let count = if fitted.kept == items {
            0.0
        } else {
            design::M + more(items - fitted.kept) + 2.0 * design::XS
        };
        shown + count <= width
    }

    /// FT3 and its repair: the optional stats drop first, then the portraits go together, then
    /// AC. Then first names shorten to an initial, then the state word to its icon.
    #[test]
    fn no_item_is_cut_off_and_ac_stays_until_the_portraits_have_gone() {
        let step = |shown, portrait, names, state_word| Step {
            shown,
            portrait,
            names,
            state_word,
        };
        assert_eq!(
            steps(4),
            [
                step(4, true, Names::First, true),
                step(3, true, Names::First, true),
                step(2, true, Names::First, true),
                step(1, true, Names::First, true),
                step(1, false, Names::First, true),
                step(0, false, Names::First, true),
                step(0, false, Names::Initial, true),
                step(0, false, Names::Initial, false),
            ]
        );
        // No item is ever cut off: at every width, the items that show are whole, and a wider
        // bar never shows a lower level or fewer players.
        for bloodied in [0, 3, 5] {
            let items = party(5, bloodied);
            let mut last: Option<Fitted> = None;
            for width in (30..=200).map(|step| step as f32 * 10.0) {
                let fitted = fit(&items, room(width, None), more, None);
                assert!(whole(&fitted, 5, width), "{bloodied} Bloodied, {width}");
                if let Some(last) = &last {
                    assert!(level(&fitted) <= level(last) && fitted.kept >= last.kept);
                }
                last = Some(fitted);
            }
            let wide = last.unwrap();
            assert!(
                level(&wide) == 0 && wide.kept == 5,
                "a wide bar shows everything"
            );
            let narrow = fit(&items, room(300.0, None), more, None);
            assert!(
                narrow.kept < 5,
                "the narrowest bar keeps whole items and counts the rest"
            );
        }
    }

    /// Round 13 FT1, the lead's ruling: in a fight, the level may drop when the words need it,
    /// but it never rises, and no item narrows, until the fight ends or the bar's width
    /// changes. The hold never hides a player who shows without it (review of PR 185).
    #[test]
    fn a_fight_holds_the_level_and_places_until_it_ends() {
        let (calm, hurt) = (party(5, 0), party(5, 1));
        let fight = Some(Uuid::from_u128(7));
        let mut held = 0;
        for width in (30..=200).map(|step| step as f32 * 10.0) {
            let fresh = |items: &[Measured], width: f32| fit(items, room(width, None), more, None);
            let at = |items: &[Measured], hold: Option<&Hold>, fight: Option<Uuid>| {
                fit(items, room(width, fight), more, hold)
            };
            let before = at(&calm, None, fight);
            let bloodied = at(&hurt, before.hold.as_ref(), fight);
            let healed = at(&calm, bloodied.hold.as_ref(), fight);
            for (fitted, items) in [(&bloodied, &hurt), (&healed, &calm)] {
                assert!(whole(fitted, 5, width), "{width}: {fitted:?}");
                assert!(
                    fitted.kept >= fresh(items, width).kept,
                    "{width}: the hold hid a player"
                );
            }
            if level(&fresh(&hurt, width)) == level(&fresh(&calm, width))
                || fresh(&hurt, width).kept < 5
            {
                continue;
            }
            held += 1;
            assert_ne!(
                level(&before),
                level(&bloodied),
                "the level drops at {width}"
            );
            assert_eq!(
                level(&healed),
                level(&bloodied),
                "the level holds at {width}"
            );
            assert!(
                healed
                    .widths
                    .iter()
                    .zip(&bloodied.widths)
                    .all(|(healed, bloodied)| healed >= bloodied),
                "no item narrows at {width}"
            );
            // Another fight, another width, or the end of the run measures again.
            let next = at(&calm, healed.hold.as_ref(), Some(Uuid::from_u128(8)));
            assert_eq!(
                level(&next),
                level(&before),
                "a new fight starts fresh at {width}"
            );
            let wider = fit(&calm, room(width + 10.0, fight), more, healed.hold.as_ref());
            assert_eq!(
                level(&wider),
                level(&fresh(&calm, width + 10.0)),
                "resize at {width}"
            );
            let ended = at(&calm, healed.hold.as_ref(), None);
            assert!(ended.hold.is_none() && level(&ended) == level(&before));
        }
        assert!(
            held > 2,
            "some widths need a lower level for one Bloodied player"
        );
    }
}
