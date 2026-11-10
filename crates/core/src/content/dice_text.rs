//! Rollable numbers in printed rules text: `Attack Roll: +4`, `9 (2d6 + 2) Bludgeoning
//! damage`, `Dexterity Saving Throw: DC 14`, and plain dice such as `1d4`. A roll made from
//! one of these only shows a result, so the printed numbers are enough to make it.
use serde::{Deserialize, Serialize};
use std::ops::Range;

/// The six abilities, in book order, as a save or check names them.
pub const ABILITIES: [&str; 6] = [
    "Strength",
    "Dexterity",
    "Constitution",
    "Intelligence",
    "Wisdom",
    "Charisma",
];

/// One typed part of a damage roll, such as `2d6 + 2` bludgeoning.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DamagePart {
    pub dice: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

/// What a d20 roll is for. Only an attack or a death save has a rule for a natural 20 or 1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum D20 {
    Attack,
    Check,
    Save,
    Skill,
    DeathSave,
}

/// What a press on a rollable number asks for. `label` names the feature, ability or skill.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RollLink {
    /// A d20 plus a bonus. An attack carries the damage of its hit, for `Roll damage`.
    D20 {
        what: D20,
        bonus: i64,
        label: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        damage: Vec<DamagePart>,
    },
    /// One damage roll, with every part in its sentence.
    Damage {
        label: String,
        parts: Vec<DamagePart>,
    },
    /// Dice with no attack or damage meaning.
    Dice { label: String, dice: String },
    /// A DC that targets save against. It is never rolled itself.
    SaveDc {
        label: String,
        ability: String,
        dc: i64,
        /// True when the text says a success takes half damage.
        half: bool,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        damage: Vec<DamagePart>,
    },
}

/// The scheme of a Markdown link that rolls.
pub const ROLL_LINK: &str = "splat-roll:";

impl RollLink {
    /// A Markdown link destination, inside `<…>`, so spaces are allowed.
    pub fn destination(&self) -> String {
        let json = serde_json::to_string(self).unwrap_or_default();
        let mut out = String::from(ROLL_LINK);
        for c in json.chars() {
            match c {
                '%' => out.push_str("%25"),
                '<' => out.push_str("%3C"),
                '>' => out.push_str("%3E"),
                '\\' => out.push_str("%5C"),
                '&' => out.push_str("%26"),
                '\n' | '\r' => out.push(' '),
                c => out.push(c),
            }
        }
        out
    }

    /// The link a destination names, if it is a roll.
    pub fn from_destination(destination: &str) -> Option<Self> {
        let encoded = destination.strip_prefix(ROLL_LINK)?;
        let mut json = String::with_capacity(encoded.len());
        let mut rest = encoded;
        while let Some(at) = rest.find('%') {
            json.push_str(&rest[..at]);
            let code = rest.get(at + 1..at + 3)?;
            json.push(char::from(u8::from_str_radix(code, 16).ok()?));
            rest = &rest[at + 3..];
        }
        json.push_str(rest);
        serde_json::from_str(&json).ok()
    }

    pub fn label(&self) -> &str {
        match self {
            Self::D20 { label, .. }
            | Self::Damage { label, .. }
            | Self::Dice { label, .. }
            | Self::SaveDc { label, .. } => label,
        }
    }
}

/// One rollable number in a text, by its byte range.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Span {
    pub range: Range<usize>,
    pub link: RollLink,
}

/// Book minus signs and dashes read as minus in a dice expression.
pub fn plain_dice(text: &str) -> String {
    text.chars()
        .map(|c| {
            if matches!(c, '\u{2212}' | '\u{2013}') {
                '-'
            } else {
                c
            }
        })
        .filter(|c| !c.is_whitespace())
        .collect()
}

fn signed_number(text: &str) -> Option<(i64, usize)> {
    let mut chars = text.char_indices();
    let (_, first) = chars.next()?;
    let (negative, start) = match first {
        '+' => (false, 1),
        '-' => (true, 1),
        '\u{2212}' => (true, '\u{2212}'.len_utf8()),
        _ => return None,
    };
    let digits = text[start..]
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(text.len() - start);
    let value: i64 = text[start..start + digits].parse().ok()?;
    Some((if negative { -value } else { value }, start + digits))
}

/// A dice term at `at`, such as `2d6` or `d20`, with ` + 2` or ` − 1` after it. Returns its end.
fn dice_at(text: &str, at: usize) -> Option<usize> {
    let bytes = text.as_bytes();
    let mut end = at;
    while end < bytes.len() && bytes[end].is_ascii_digit() {
        end += 1;
    }
    if bytes.get(end) != Some(&b'd') {
        return None;
    }
    end += 1;
    let sides = end;
    while end < bytes.len() && bytes[end].is_ascii_digit() {
        end += 1;
    }
    if end == sides || bytes.get(end).is_some_and(u8::is_ascii_alphabetic) {
        return None;
    }
    // A flat bonus after the dice, as books print it.
    let after = &text[end..];
    let spaced = after.trim_start_matches(' ');
    let gap = after.len() - spaced.len();
    if let Some(sign) = spaced.chars().next().filter(|c| "+-\u{2212}".contains(*c)) {
        let rest = &spaced[sign.len_utf8()..];
        let number = rest.trim_start_matches(' ');
        let digits = number
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(number.len());
        if digits > 0 {
            end += gap + sign.len_utf8() + (rest.len() - number.len()) + digits;
        }
    }
    Some(end)
}

/// The word just before `at`, without emphasis marks.
fn word_before(text: &str, at: usize) -> &str {
    let head = text[..at].trim_end_matches(['*', '_', ' ']);
    let start = head.rfind(|c: char| !c.is_alphabetic()).map_or(0, |at| {
        at + head[at..].chars().next().map_or(1, char::len_utf8)
    });
    &head[start..]
}

fn ability(word: &str) -> Option<&'static str> {
    ABILITIES
        .into_iter()
        .find(|ability| ability.eq_ignore_ascii_case(word))
}

/// The damage type and the word `damage` after a group, as in `) Fire damage`.
fn damage_kind(after: &str) -> Option<Option<String>> {
    let rest = after.trim_start();
    if rest.starts_with("damage") {
        return Some(None);
    }
    let word: String = rest.chars().take_while(|c| c.is_alphabetic()).collect();
    let tail = rest[word.len()..].trim_start();
    (!word.is_empty() && tail.starts_with("damage")).then(|| {
        let mut chars = word.chars();
        Some(chars.next().map_or_else(String::new, |first| {
            first
                .to_uppercase()
                .chain(chars.map(|c| c.to_ascii_lowercase()))
                .collect()
        }))
    })
}

/// Where each sentence of a paragraph ends, so a hit's damage is the sentence after it.
fn sentence_end(text: &str, from: usize) -> usize {
    let bytes = text.as_bytes();
    let mut depth = 0i32;
    for at in from..bytes.len() {
        match bytes[at] {
            b'(' => depth += 1,
            b')' => depth -= 1,
            b'.' if depth <= 0
                && bytes
                    .get(at + 1)
                    .is_none_or(|next| next.is_ascii_whitespace()) =>
            {
                return at + 1;
            }
            _ => {}
        }
    }
    text.len()
}

/// Every rollable number in one paragraph, in order. `label` names the paragraph's feature.
pub fn find(text: &str, label: &str) -> Vec<Span> {
    let mut groups: Vec<(Range<usize>, DamagePart, bool)> = Vec::new();
    // Damage groups first: an average, then its dice in brackets.
    let mut at = 0;
    while let Some(open) = text[at..].find(" (").map(|i| at + i + 1) {
        at = open + 1;
        let average_start = text[..open - 1]
            .rfind(|c: char| !c.is_ascii_digit())
            .map_or(0, |i| i + 1);
        if average_start == open - 1 {
            continue;
        }
        let Some(end) = dice_at(text, open + 1) else {
            continue;
        };
        if text.as_bytes().get(end) != Some(&b')') {
            continue;
        }
        let dice = plain_dice(&text[open + 1..end]);
        let kind = damage_kind(&text[end + 1..]);
        groups.push((
            average_start..end + 1,
            DamagePart {
                dice,
                kind: kind.clone().flatten(),
            },
            kind.is_some(),
        ));
        at = end + 1;
    }
    let damage_in = |range: Range<usize>| -> Vec<DamagePart> {
        groups
            .iter()
            .filter(|(group, _, damage)| *damage && range.contains(&group.start))
            .map(|(_, part, _)| part.clone())
            .collect()
    };
    let mut spans = Vec::new();
    for (range, part, damage) in &groups {
        let sentence =
            text[..range.start].rfind(". ").map_or(0, |at| at + 2)..sentence_end(text, range.start);
        spans.push(Span {
            range: range.clone(),
            link: if *damage {
                RollLink::Damage {
                    label: label.to_owned(),
                    parts: damage_in(sentence),
                }
            } else {
                RollLink::Dice {
                    label: label.to_owned(),
                    dice: part.dice.clone(),
                }
            },
        });
    }
    // Attack bonuses: the 2024 `Attack Roll: +4`, and the older `+4 to hit`.
    for (index, _) in text.match_indices("Attack Roll:") {
        let start = index + "Attack Roll:".len();
        let rest = &text[start..];
        let offset = rest.len() - rest.trim_start().len();
        if let Some((bonus, length)) = signed_number(&text[start + offset..]) {
            let range = start + offset..start + offset + length;
            let hit = text[range.end..]
                .find("Hit:")
                .map(|at| range.end + at)
                .unwrap_or(range.end);
            spans.push(Span {
                range,
                link: RollLink::D20 {
                    what: D20::Attack,
                    bonus,
                    label: label.to_owned(),
                    damage: damage_in(hit..sentence_end(text, hit)),
                },
            });
        }
    }
    for (index, _) in text.match_indices(" to hit") {
        let start = text[..index].rfind(['+', '-', '\u{2212}']).unwrap_or(index);
        if let Some((bonus, length)) = signed_number(&text[start..])
            && start + length == index
        {
            spans.push(Span {
                range: start..index,
                link: RollLink::D20 {
                    what: D20::Attack,
                    bonus,
                    label: label.to_owned(),
                    damage: damage_in(index..sentence_end(text, index)),
                },
            });
        }
    }
    // Save DCs: the 2024 `Dexterity Saving Throw: DC 14`, and the older `DC 14 Dexterity
    // saving throw`. A DC with no save, such as an escape DC, stays text.
    let half = {
        let lower = text.to_lowercase();
        lower.contains("success: half damage") || lower.contains("half as much damage")
    };
    let mut add_dc = |range: Range<usize>, ability: &str, dc: i64| {
        spans.push(Span {
            range,
            link: RollLink::SaveDc {
                label: label.to_owned(),
                ability: ability.to_owned(),
                dc,
                half,
                damage: damage_in(0..text.len()),
            },
        });
    };
    for (index, _) in text.match_indices("Saving Throw") {
        let Some(ability) = ability(word_before(text, index)) else {
            continue;
        };
        let after = &text[index + "Saving Throw".len()..];
        let skipped = after.len() - after.trim_start_matches(['*', '_', ':', ' ']).len();
        let start = index + "Saving Throw".len() + skipped;
        if let Some(digits) = text[start..].strip_prefix("DC ") {
            let length = digits
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(digits.len());
            if let Ok(dc) = digits[..length].parse() {
                add_dc(start..start + 3 + length, ability, dc);
            }
        }
    }
    for (index, _) in text.match_indices("DC ") {
        let digits = &text[index + 3..];
        let length = digits
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(digits.len());
        let after = digits[length..].trim_start();
        let word: String = after.chars().take_while(|c| c.is_alphabetic()).collect();
        if length > 0
            && let Some(ability) = ability(&word)
            && after[word.len()..].trim_start().starts_with("saving throw")
            && let Ok(dc) = digits[..length].parse()
        {
            add_dc(index..index + 3 + length, ability, dc);
        }
    }
    // Any other dice, such as `1d4` or `regains 2d8 + 5`.
    let bytes = text.as_bytes();
    let mut at = 0;
    while at < bytes.len() {
        let starts = bytes[at].is_ascii_digit() || bytes[at] == b'd';
        let boundary = at == 0 || !bytes[at - 1].is_ascii_alphanumeric();
        if starts
            && boundary
            && !spans.iter().any(|span| span.range.contains(&at))
            && let Some(end) = dice_at(text, at)
            && !spans.iter().any(|span| span.range.contains(&(end - 1)))
        {
            spans.push(Span {
                range: at..end,
                link: RollLink::Dice {
                    label: label.to_owned(),
                    dice: plain_dice(&text[at..end]),
                },
            });
            at = end;
            continue;
        }
        at += 1;
    }
    spans.sort_by_key(|span| span.range.start);
    spans
}

/// The skills of a Skills row, `Stealth +6, Survival +2`, with each bonus's byte range.
pub fn skills(text: &str) -> Vec<(Range<usize>, String, i64)> {
    let mut out = Vec::new();
    let mut start = 0;
    for part in text.split([',', ';']) {
        let trimmed = part.trim_end();
        if let Some(sign) = trimmed.rfind(['+', '-', '\u{2212}'])
            && let Some((bonus, length)) = signed_number(&trimmed[sign..])
            && sign + length == trimmed.len()
        {
            let name = trimmed[..sign].trim();
            if !name.is_empty() {
                out.push((start + sign..start + trimmed.len(), name.to_owned(), bonus));
            }
        }
        start += part.len() + 1;
    }
    out
}

/// A Markdown paragraph with each rollable number as a link that rolls.
pub fn with_links(text: &str, label: &str) -> String {
    let mut out = String::with_capacity(text.len() * 2);
    let mut at = 0;
    for span in find(text, label) {
        if span.range.start < at {
            continue;
        }
        out.push_str(&text[at..span.range.start]);
        out.push('[');
        out.push_str(&text[span.range.clone()]);
        out.push_str("](<");
        out.push_str(&span.link.destination());
        out.push_str(">)");
        at = span.range.end;
    }
    out.push_str(&text[at..]);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn part(dice: &str, kind: &str) -> DamagePart {
        DamagePart {
            dice: dice.into(),
            kind: Some(kind.into()),
        }
    }

    #[test]
    fn an_attack_carries_the_damage_of_its_hit_and_each_group_is_a_link() {
        let text = "Melee Attack Roll: +4, reach 10 ft. Hit: 9 (2d6 + 2) Bludgeoning damage plus 3 (1d6) Fire damage. If the target is Grappled, it takes 7 (2d6) Poison damage.";
        let spans = find(text, "Grab");
        let shown: Vec<&str> = spans.iter().map(|s| &text[s.range.clone()]).collect();
        assert_eq!(shown, ["+4", "9 (2d6 + 2)", "3 (1d6)", "7 (2d6)"]);
        assert_eq!(
            spans[0].link,
            RollLink::D20 {
                what: D20::Attack,
                bonus: 4,
                label: "Grab".into(),
                damage: vec![part("2d6+2", "Bludgeoning"), part("1d6", "Fire")],
            }
        );
        // Both groups of one sentence are one roll, from either link.
        assert_eq!(spans[1].link, spans[2].link);
        assert_eq!(
            spans[3].link,
            RollLink::Damage {
                label: "Grab".into(),
                parts: vec![part("2d6", "Poison")],
            }
        );
    }

    #[test]
    fn a_save_dc_knows_its_ability_damage_and_half_but_an_escape_dc_is_text() {
        let text = "Dexterity Saving Throw: DC 21, each creature in a 60-foot Cone. Failure: 66 (12d10) Fire damage. Success: Half damage.";
        let spans = find(text, "Fire Breath");
        assert_eq!(&text[spans[0].range.clone()], "DC 21");
        assert_eq!(
            spans[0].link,
            RollLink::SaveDc {
                label: "Fire Breath".into(),
                ability: "Dexterity".into(),
                dc: 21,
                half: true,
                damage: vec![part("12d10", "Fire")],
            }
        );
        let grab = "Hit: 9 (2d6 + 2) Bludgeoning damage, and it has the Grappled condition (escape DC 12).";
        assert!(
            find(grab, "Grab")
                .iter()
                .all(|span| !matches!(span.link, RollLink::SaveDc { .. }))
        );
        let old = "The target must make a DC 13 Dexterity saving throw, taking 21 (6d6) fire damage on a failed save, or half as much damage on a successful one.";
        let spans = find(old, "Fireball");
        assert!(matches!(
            &spans[0].link,
            RollLink::SaveDc { dc: 13, half: true, ability, .. } if ability == "Dexterity"
        ));
    }

    #[test]
    fn book_minus_signs_older_to_hit_and_plain_dice_are_rollable() {
        let text = "Bite. Melee Weapon Attack: +5 to hit, reach 5 ft. Hit: 1 (1d4 \u{2212} 1) piercing damage. It regains 2d8 + 5 Hit Points, and rolls a d20.";
        let spans = find(text, "Bite");
        let shown: Vec<&str> = spans.iter().map(|s| &text[s.range.clone()]).collect();
        assert_eq!(shown, ["+5", "1 (1d4 \u{2212} 1)", "2d8 + 5", "d20"]);
        assert!(matches!(
            &spans[1].link,
            RollLink::Damage { parts, .. } if parts == &[part("1d4-1", "Piercing")]
        ));
        assert!(matches!(&spans[2].link, RollLink::Dice { dice, .. } if dice == "2d8+5"));
        // Words that hold a d are not dice.
        assert!(find("The dragon's 3rd head and d6x", "x").is_empty());
    }

    #[test]
    fn links_survive_markdown_and_come_back_whole() {
        let link = RollLink::Dice {
            label: "Odd <name> & 50% \\ (1)".into(),
            dice: "1d4".into(),
        };
        let markdown = format!("[1d4](<{}>)", link.destination());
        let parser = pulldown_cmark::Parser::new(&markdown);
        let destination = parser
            .filter_map(|event| match event {
                pulldown_cmark::Event::Start(pulldown_cmark::Tag::Link { dest_url, .. }) => {
                    Some(dest_url.to_string())
                }
                _ => None,
            })
            .next()
            .unwrap();
        assert_eq!(RollLink::from_destination(&destination), Some(link));
    }

    #[test]
    fn skills_give_each_bonus_its_place() {
        let text = "Stealth +6, Survival \u{2212}1";
        let found = skills(text);
        assert_eq!(found[0].1, "Stealth");
        assert_eq!(found[0].2, 6);
        assert_eq!(&text[found[0].0.clone()], "+6");
        assert_eq!(found[1].2, -1);
        assert_eq!(&text[found[1].0.clone()], "\u{2212}1");
    }

    #[test]
    fn every_builtin_attack_roll_and_damage_group_gets_a_link() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../builtin/dnd2024-definitions/creatures");
        let mut checked = 0;
        for file in std::fs::read_dir(dir).unwrap() {
            let text = std::fs::read_to_string(file.unwrap().path()).unwrap();
            for paragraph in text.split("\n\n").filter(|p| !p.starts_with('|')) {
                let spans = find(paragraph, "x");
                let attacks = paragraph
                    .matches("Attack Roll: +")
                    .chain(paragraph.matches("Attack Roll: \u{2212}"))
                    .count()
                    // A spellcasting line's `+9 to hit with spell attacks` rolls too.
                    + paragraph
                        .match_indices(" to hit")
                        .filter(|(at, _)| paragraph[..*at].ends_with(|c: char| c.is_ascii_digit()))
                        .count();
                let found = spans
                    .iter()
                    .filter(|s| matches!(s.link, RollLink::D20 { .. }))
                    .count();
                assert_eq!(found, attacks, "{paragraph}");
                for (index, _) in paragraph.match_indices(") ") {
                    let after = &paragraph[index + 2..];
                    let typed = damage_kind(after).is_some();
                    let open = paragraph[..index].rfind('(');
                    if typed && open.is_some_and(|open| dice_at(paragraph, open + 1) == Some(index))
                    {
                        assert!(
                            spans.iter().any(|s| s.range.end == index + 1
                                && matches!(s.link, RollLink::Damage { .. })),
                            "{paragraph}"
                        );
                        checked += 1;
                    }
                }
            }
        }
        assert!(checked > 300, "{checked}");
    }
}
