//! What a spell rolls when a creature casts it, from the spell's printed rules: its attack or
//! its save, and its damage at the level it is cast. A stat block lists spells by name, and
//! the caster gives the numbers: its spell save DC and its bonus to hit with spell attacks.
use super::dice_text::{D20, DamagePart, RollLink, ability, damage_kind, dice_at, plain_dice};

/// A caster's numbers for its spells, as its Spellcasting gives them.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Caster {
    /// `spell save DC 13`.
    pub dc: Option<i64>,
    /// `+6 to hit with spell attacks`.
    pub attack: Option<i64>,
}

impl Caster {
    /// The bonus to hit in a Spellcasting paragraph's words: `+6 to hit with spell attacks`.
    pub fn attack_in(words: &str) -> Option<i64> {
        let at = words.find(" to hit with spell attack")?;
        let head = words[..at].trim_end();
        let start = head.rfind(['+', '-', '\u{2212}'])?;
        let digits = &head[start..]
            .trim_start_matches(['+', '-', '\u{2212}'])
            .trim();
        let value: i64 = digits.parse().ok()?;
        Some(if head[start..].starts_with('+') {
            value
        } else {
            -value
        })
    }
}

/// What `name` rolls when it is cast at level `cast`, from its rules text: an attack with the
/// damage of a hit, or a save with the damage of a failure. `level` is the spell's own level;
/// the paragraph `Using a Higher-Level Spell Slot.` adds the dice of each level above it.
/// `None` when the spell rolls nothing, when the caster lacks the number it needs, or when a
/// cantrip's damage grows by a level that a stat block does not give.
pub fn spell_roll(
    name: &str,
    rules: &str,
    level: u32,
    cast: u32,
    caster: &Caster,
) -> Option<RollLink> {
    let paragraphs: Vec<&str> = rules
        .split("\n\n")
        .flat_map(str::lines)
        .map(|line| line.trim().trim_end_matches('\\').trim_end())
        .filter(|line| !line.is_empty())
        .collect();
    let upcast = paragraphs
        .iter()
        .find_map(|p| p.strip_prefix("Using a Higher-Level Spell Slot."));
    let cantrip_upgrade = paragraphs.iter().any(|p| p.starts_with("Cantrip Upgrade."));
    let text = paragraphs
        .iter()
        .filter(|p| {
            !p.starts_with("Using a Higher-Level Spell Slot.") && !p.starts_with("Cantrip Upgrade.")
        })
        .copied()
        .collect::<Vec<_>>()
        .join("\n");
    let lower = text.to_lowercase();
    let attack = ["melee spell attack", "ranged spell attack"]
        .iter()
        .filter_map(|words| lower.find(words))
        .min();
    let save = save_at(&text);
    let (marker, link) = match (attack, save) {
        (Some(at), save) if save.is_none_or(|(save, _)| at < save) => (at, Mark::Attack),
        (_, Some((at, ability))) => (at, Mark::Save(ability)),
        _ => return None,
    };
    let mut damage = damage_after(&text, marker);
    if level == 0 && cantrip_upgrade && !damage.is_empty() {
        return None;
    }
    if let Some(upcast) = upcast {
        grow(&mut damage, upcast, level, cast);
    }
    match link {
        Mark::Attack => Some(RollLink::D20 {
            what: D20::Attack,
            bonus: caster.attack?,
            label: name.to_owned(),
            damage,
        }),
        Mark::Save(ability) => {
            let paragraph = paragraph_around(&text, marker);
            let half = paragraph.contains("half as much damage")
                || paragraph.contains("half the initial damage")
                || paragraph.contains("half that damage");
            Some(RollLink::SaveDc {
                label: name.to_owned(),
                ability: ability.to_owned(),
                dc: caster.dc?,
                half: half && !damage.is_empty(),
                damage,
            })
        }
    }
}

/// The damage types of the rules, as a growth sentence may name one.
const DAMAGE_TYPES: [&str; 13] = [
    "Acid",
    "Bludgeoning",
    "Cold",
    "Fire",
    "Force",
    "Lightning",
    "Necrotic",
    "Piercing",
    "Poison",
    "Psychic",
    "Radiant",
    "Slashing",
    "Thunder",
];

enum Mark {
    Attack,
    Save(&'static str),
}

/// The first `<Ability> saving throw`, and the ability.
fn save_at(text: &str) -> Option<(usize, &'static str)> {
    text.match_indices(" saving throw").find_map(|(at, _)| {
        let head = &text[..at];
        let word = &head[head.rfind(' ').map_or(0, |space| space + 1)..];
        ability(word).map(|ability| (at - word.len(), ability))
    })
}

/// The line of rules text that holds `at`.
fn paragraph_around(text: &str, at: usize) -> &str {
    let start = text[..at].rfind('\n').map_or(0, |n| n + 1);
    let end = text[at..].find('\n').map_or(text.len(), |n| at + n);
    &text[start..end]
}

/// The end of the sentence that holds `at`: after its full stop, or at the end of its line.
fn sentence_end(text: &str, at: usize) -> usize {
    let line = text[at..].find('\n').map_or(text.len(), |n| at + n);
    text[at..line]
        .match_indices(". ")
        .next()
        .map_or(line, |(n, _)| at + n + 1)
}

/// The damage that the attack's hit or the save's failure deals: each `2d6 Fire damage` in
/// the sentence of `marker` and the one after it. A part after `or` is another choice, and a
/// part that comes later (`at the end of its next turn`) or on a condition (`if it acts`) is
/// not part of this roll.
fn damage_after(text: &str, marker: usize) -> Vec<DamagePart> {
    let first = sentence_end(text, marker);
    let end = if text[first..].starts_with('\n') || first >= text.len() {
        first
    } else {
        sentence_end(text, (first + 1).min(text.len()))
    };
    let scope = &text[marker..end];
    let mut parts = Vec::new();
    let mut last = 0;
    let mut at = 0;
    while at < scope.len() {
        let starts_term = scope.as_bytes()[at].is_ascii_digit()
            && (at == 0 || !scope.as_bytes()[at - 1].is_ascii_alphanumeric());
        let Some(term_end) = starts_term.then(|| dice_at(scope, at)).flatten() else {
            at += 1;
            continue;
        };
        if let Some(kind) = damage_kind(&scope[term_end..]) {
            let between = &scope[last..at];
            // The clause of this part ends where the next part or sentence part starts.
            let rest = &scope[term_end..];
            let clause_end = [",", ";", ".", " and ", " plus ", " or "]
                .iter()
                .filter_map(|stop| rest.find(stop))
                .min()
                .map_or(scope.len(), |n| term_end + n);
            let clause = &scope[term_end..clause_end];
            let later = clause.contains(" at the end of") || clause.contains(" at the start of");
            let conditional = clause.contains(" if ");
            let another_choice = !parts.is_empty() && between.contains(" or ");
            if !later && !conditional && !another_choice {
                parts.push(DamagePart {
                    dice: plain_dice(&scope[at..term_end]),
                    kind,
                    when: None,
                });
            }
            last = term_end;
        }
        at = term_end;
    }
    parts
}

/// Add the dice that `Using a Higher-Level Spell Slot.` gives when the spell of `level` is
/// cast at `cast`: `The damage increases by 1d6 for each spell slot level above 2.` A sentence
/// that names a type (`The Cold damage increases`) adds only to that type.
fn grow(damage: &mut [DamagePart], upcast: &str, level: u32, cast: u32) {
    let Some((at, by)) = ["increases by ", "increase by "]
        .iter()
        .find_map(|words| upcast.find(words).map(|at| (at, words.len())))
    else {
        return;
    };
    let term = &upcast[at + by..];
    let Some(end) = dice_at(term, 0) else {
        return;
    };
    let Some((count, sides)) = plain_dice(&term[..end])
        .split_once('d')
        .and_then(|(c, s)| Some((c.parse::<u32>().ok()?, s.parse::<u32>().ok()?)))
    else {
        return;
    };
    let above = term[end..]
        .find("level above ")
        .and_then(|n| {
            let rest = &term[end + n + "level above ".len()..];
            rest[..rest
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(rest.len())]
                .parse::<u32>()
                .ok()
        })
        .unwrap_or(level);
    let extra = cast.saturating_sub(above) * count;
    if extra == 0 {
        return;
    }
    let head = upcast[..at].to_lowercase();
    let named: Vec<&str> = DAMAGE_TYPES
        .into_iter()
        .filter(|kind| head.contains(&format!("{} damage", kind.to_lowercase())))
        .collect();
    for part in damage.iter_mut().filter(|part| {
        named.is_empty()
            || part
                .kind
                .as_deref()
                .is_some_and(|kind| named.iter().any(|named| kind.eq_ignore_ascii_case(named)))
    }) {
        let Some((dice, flat)) = split_flat(&part.dice) else {
            continue;
        };
        let Some((n, s)) = dice
            .split_once('d')
            .and_then(|(n, s)| Some((n.parse::<u32>().ok()?, s.parse::<u32>().ok()?)))
        else {
            continue;
        };
        if s == sides {
            part.dice = format!("{}d{s}{flat}", n + extra);
        }
    }
}

/// `3d6+2` as `3d6` and `+2`.
fn split_flat(dice: &str) -> Option<(&str, &str)> {
    let at = dice[1..].find(['+', '-']).map_or(dice.len(), |n| n + 1);
    Some((&dice[..at], &dice[at..]))
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASTER: Caster = Caster {
        dc: Some(13),
        attack: Some(5),
    };

    fn part(dice: &str, kind: &str) -> DamagePart {
        DamagePart {
            dice: dice.into(),
            kind: Some(kind.into()),
            when: None,
        }
    }

    #[test]
    fn a_save_spell_rolls_its_failure_damage_with_the_casters_dc() {
        let moonbeam = "A silvery beam of pale light shines down in a 5-foot-radius, 40-foot-high Cylinder centered on a point within range.\n\nWhen the Cylinder appears, each creature in it makes a Constitution saving throw. On a failed save, a creature takes 2d10 Radiant damage, and if the creature is shape-shifted, it reverts to its true form. On a successful save, a creature takes half as much damage only.\n\nUsing a Higher-Level Spell Slot. The damage increases by 1d10 for each spell slot level above 2.";
        assert_eq!(
            spell_roll("Moonbeam", moonbeam, 2, 2, &CASTER),
            Some(RollLink::SaveDc {
                label: "Moonbeam".into(),
                ability: "Constitution".into(),
                dc: 13,
                half: true,
                damage: vec![part("2d10", "Radiant")],
            })
        );
        let Some(RollLink::SaveDc { damage, .. }) = spell_roll("Moonbeam", moonbeam, 2, 4, &CASTER)
        else {
            panic!("Moonbeam saves");
        };
        assert_eq!(damage, vec![part("4d10", "Radiant")]);
        // A caster with no save DC cannot cast it as a roll.
        assert_eq!(
            spell_roll("Moonbeam", moonbeam, 2, 2, &Caster::default()),
            None
        );
    }

    #[test]
    fn an_attack_spell_rolls_its_hit_at_the_level_cast() {
        let bolt = "A flash of light streaks toward a creature of your choice within range. Make a ranged spell attack against the target. On a hit, it takes 4d6 Radiant damage, and the next attack roll made against it before the end of your next turn has Advantage.\n\nUsing a Higher-Level Spell Slot. The damage increases by 1d6 for each spell slot level above 1.";
        assert_eq!(
            spell_roll("Guiding Bolt", bolt, 1, 2, &CASTER),
            Some(RollLink::D20 {
                what: D20::Attack,
                bonus: 5,
                label: "Guiding Bolt".into(),
                damage: vec![part("5d6", "Radiant")],
            })
        );
        let no_bonus = Caster {
            dc: Some(13),
            attack: None,
        };
        assert_eq!(spell_roll("Guiding Bolt", bolt, 1, 1, &no_bonus), None);
    }

    #[test]
    fn later_conditional_and_other_choice_damage_is_left_out() {
        let arrow = "Make a ranged spell attack against the target. On a hit, the target takes 4d4 Acid damage and 2d4 Acid damage at the end of its next turn. On a miss, the arrow splashes the target with acid for half as much of the initial damage only.";
        let Some(RollLink::D20 { damage, .. }) = spell_roll("Acid Arrow", arrow, 2, 2, &CASTER)
        else {
            panic!("Acid Arrow attacks");
        };
        assert_eq!(damage, vec![part("4d4", "Acid")]);
        let geas = "The target must succeed on a Wisdom saving throw or have the Charmed condition for the duration. The target automatically succeeds if it can’t understand your command.\nWhile Charmed, the creature takes 5d10 Psychic damage if it acts in a manner directly counter to your command.";
        let Some(RollLink::SaveDc { damage, half, .. }) = spell_roll("Geas", geas, 5, 5, &CASTER)
        else {
            panic!("Geas saves");
        };
        assert!(damage.is_empty() && !half);
        let guardians = "the creature must make a Wisdom saving throw. On a failed save, the creature takes 3d8 Radiant damage (if you are good or neutral) or 3d8 Necrotic damage (if you are evil). On a successful save, the creature takes half as much damage.";
        let Some(RollLink::SaveDc { damage, .. }) =
            spell_roll("Spirit Guardians", guardians, 3, 3, &CASTER)
        else {
            panic!("Spirit Guardians saves");
        };
        assert_eq!(damage, vec![part("3d8", "Radiant")]);
        // Two types on a failure both count; a growth that names one type adds to it alone.
        let strike = "Each creature in a 10-foot-radius, 40-foot-high Cylinder centered on a point within range makes a Dexterity saving throw. A creature takes 5d6 Fire damage and 5d6 Radiant damage on a failed save or half as much damage on a successful one.\n\nUsing a Higher-Level Spell Slot. The Fire damage and the Radiant damage increase by 1d6 for each spell slot level above 5.";
        let Some(RollLink::SaveDc { damage, half, .. }) =
            spell_roll("Flame Strike", strike, 5, 6, &CASTER)
        else {
            panic!("Flame Strike saves");
        };
        assert!(half);
        assert_eq!(damage, vec![part("6d6", "Fire"), part("6d6", "Radiant")]);
    }

    #[test]
    fn a_spell_with_no_attack_or_save_rolls_nothing() {
        let druidcraft = "Whispering to the spirits of nature, you create one of the following effects within range.";
        assert_eq!(spell_roll("Druidcraft", druidcraft, 0, 0, &CASTER), None);
        let hold = "Choose a Humanoid that you can see within range. The target must succeed on a Wisdom saving throw or have the Paralyzed condition for the duration.\n\nUsing a Higher-Level Spell Slot. You can target one additional Humanoid for each spell slot level above 2.";
        assert_eq!(
            spell_roll("Hold Person", hold, 2, 2, &CASTER),
            Some(RollLink::SaveDc {
                label: "Hold Person".into(),
                ability: "Wisdom".into(),
                dc: 13,
                half: false,
                damage: Vec::new(),
            })
        );
        // A cantrip's damage grows with a level a stat block does not give.
        let flame = "the target must succeed on a Dexterity saving throw or take 1d8 Radiant damage.\n\nCantrip Upgrade. The damage increases by 1d8 when you reach levels 5 (2d8), 11 (3d8), and 17 (4d8).";
        assert_eq!(spell_roll("Sacred Flame", flame, 0, 0, &CASTER), None);
    }

    #[test]
    fn a_casters_spell_attack_bonus_is_read_from_its_words() {
        assert_eq!(
            Caster::attack_in(
                "using Intelligence (spell save DC 14, +6 to hit with spell attacks):"
            ),
            Some(6)
        );
        assert_eq!(Caster::attack_in("(spell save DC 13):"), None);
    }
}
