//! Bounded dice expressions and a versioned, deterministic random stream.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct Dice {
    terms: Vec<Term>,
    constant: i64,
}

/// `count` dice of `sides`, added or taken away. `keep` counts only the highest (`kh`) or
/// the lowest (`kl`) of them, as advantage and disadvantage do.
#[derive(Clone, Copy, Debug)]
struct Term {
    sign: i64,
    count: u32,
    sides: u32,
    keep: Option<Keep>,
}

#[derive(Clone, Copy, Debug)]
enum Keep {
    Highest(u32),
    Lowest(u32),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Roll {
    pub dice: Vec<i64>,
    pub total: i64,
    /// The places in `dice` that a keep left out of the total.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dropped: Vec<usize>,
}

impl Dice {
    pub fn parse(expression: &str) -> Result<Self, String> {
        if expression.len() > 4096 {
            return Err("Dice expression exceeds 4096 bytes".into());
        }
        let text: String = expression
            .chars()
            .filter(|c| !c.is_ascii_whitespace())
            .collect();
        if text.is_empty() {
            return Err("Empty dice expression".into());
        }
        let mut terms = Vec::new();
        let mut constant = 0_i64;
        let mut count = 0_u32;
        let mut rest = text.as_str();
        while !rest.is_empty() {
            let sign = if let Some(s) = rest.strip_prefix('-') {
                rest = s;
                -1
            } else {
                rest = rest.strip_prefix('+').unwrap_or(rest);
                1
            };
            let end = rest.find(['+', '-']).unwrap_or(rest.len());
            let term = &rest[..end];
            rest = &rest[end..];
            if let Some((n, sides)) = term.split_once('d') {
                let n = n.parse::<u32>().map_err(|_| "Invalid dice count")?;
                let (sides, keep) = match sides.split_once('k') {
                    Some((sides, keep)) => (sides, Some(keep)),
                    None => (sides, None),
                };
                let sides = sides.parse::<u32>().map_err(|_| "Invalid die size")?;
                let keep = keep
                    .map(|keep| {
                        let (high, kept) = if let Some(kept) = keep.strip_prefix('h') {
                            (true, kept)
                        } else if let Some(kept) = keep.strip_prefix('l') {
                            (false, kept)
                        } else {
                            return Err("Keep needs h or l, as in 2d20kh1");
                        };
                        let kept = kept.parse::<u32>().map_err(|_| "Invalid keep count")?;
                        if kept == 0 || kept > n {
                            return Err("Keep count must be from 1 to the dice count");
                        }
                        Ok(if high {
                            Keep::Highest(kept)
                        } else {
                            Keep::Lowest(kept)
                        })
                    })
                    .transpose()?;
                count = count.checked_add(n).ok_or("Dice count overflow")?;
                if n == 0 || count > 1000 || sides == 0 || sides > 1_000_000 {
                    return Err("Dice limits exceeded".into());
                }
                terms.push(Term {
                    sign,
                    count: n,
                    sides,
                    keep,
                });
            } else {
                let n = term
                    .parse::<i64>()
                    .map_err(|_| "Invalid integer in dice expression")?;
                constant = constant
                    .checked_add(n.checked_mul(sign).ok_or("Dice integer overflow")?)
                    .ok_or("Dice integer overflow")?;
            }
        }
        Ok(Self { terms, constant })
    }

    /// The lowest and the highest total this expression can roll.
    pub fn range(&self) -> (i64, i64) {
        let (mut low, mut high) = (self.constant, self.constant);
        for term in &self.terms {
            let count = i64::from(term.kept(term.count));
            let sides = i64::from(term.sides);
            if term.sign > 0 {
                low += count;
                high += count * sides;
            } else {
                low -= count * sides;
                high -= count;
            }
        }
        (low, high)
    }

    /// A critical hit rolls each term's dice twice over, and keeps twice as many.
    pub fn roll(&self, random: &mut Random, critical: bool) -> Result<Roll, String> {
        let times = if critical { 2 } else { 1 };
        let mut dice = Vec::new();
        let mut dropped = Vec::new();
        let mut total = self.constant;
        for term in &self.terms {
            let first = dice.len();
            let count = term.count * times;
            for _ in 0..count {
                dice.push(i64::from(random.die(term.sides)?));
            }
            let mut order: Vec<usize> = (first..dice.len()).collect();
            // Stable: of equal faces, the first rolled is the one kept.
            match term.keep {
                Some(Keep::Highest(_)) => order.sort_by_key(|&at| std::cmp::Reverse(dice[at])),
                Some(Keep::Lowest(_)) => order.sort_by_key(|&at| dice[at]),
                None => {}
            }
            let kept = term.kept(count) as usize;
            let mut left_out: Vec<usize> = order.split_off(kept.min(order.len()));
            left_out.sort_unstable();
            for &at in &order {
                total = total
                    .checked_add(dice[at] * term.sign)
                    .ok_or("Roll overflow")?;
            }
            for face in dice.iter_mut().skip(first) {
                *face *= term.sign;
            }
            dropped.extend(left_out);
        }
        Ok(Roll {
            dice,
            total,
            dropped,
        })
    }
}

impl Term {
    /// How many of `count` rolled dice count toward the total.
    fn kept(&self, count: u32) -> u32 {
        match self.keep {
            Some(Keep::Highest(kept) | Keep::Lowest(kept)) => kept * (count / self.count),
            None => count,
        }
    }
}

/// SplitMix64 v1, with rejection sampling for unbiased die faces. Tests can supply
/// explicit faces; those are validated against the requested die and never loop.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Random {
    state: u64,
    controlled: Option<std::collections::VecDeque<u32>>,
}
impl Random {
    pub const ID: &str = "splat:splitmix64-rejection-v1";
    pub fn seeded(seed: u64) -> Self {
        Self {
            state: seed,
            controlled: None,
        }
    }
    pub fn controlled(faces: impl IntoIterator<Item = u32>) -> Self {
        Self {
            state: 0,
            controlled: Some(faces.into_iter().collect()),
        }
    }
    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
    pub fn die(&mut self, sides: u32) -> Result<u32, String> {
        if sides == 0 {
            return Err("Zero-sided die".into());
        }
        if let Some(faces) = &mut self.controlled {
            return faces
                .pop_front()
                .filter(|n| (1..=sides).contains(n))
                .ok_or_else(|| "Missing or invalid controlled die face".into());
        }
        let range = u64::from(sides);
        let threshold = range.wrapping_neg() % range;
        loop {
            let n = self.next();
            if n >= threshold {
                return Ok((n % range + 1) as u32);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn critical_doubles_dice_but_keeps_flat_modifier() {
        let roll = Dice::parse("1d6 + 3")
            .unwrap()
            .roll(&mut Random::controlled([2, 5]), true)
            .unwrap();
        assert_eq!(roll.total, 10);
        assert_eq!(roll.dice, vec![2, 5]);
    }
    #[test]
    fn keep_highest_and_lowest_drop_the_other_d20() {
        let advantage = Dice::parse("2d20kh1 + 4")
            .unwrap()
            .roll(&mut Random::controlled([6, 13]), false)
            .unwrap();
        assert_eq!(advantage.total, 17);
        assert_eq!(advantage.dice, vec![6, 13]);
        assert_eq!(advantage.dropped, vec![0]);
        let disadvantage = Dice::parse("2d20kl1+4")
            .unwrap()
            .roll(&mut Random::controlled([6, 13]), false)
            .unwrap();
        assert_eq!(disadvantage.total, 10);
        assert_eq!(disadvantage.dropped, vec![1]);
        // Equal faces keep the first.
        let tie = Dice::parse("2d20kh1")
            .unwrap()
            .roll(&mut Random::controlled([9, 9]), false)
            .unwrap();
        assert_eq!(tie.dropped, vec![1]);
        assert_eq!(Dice::parse("2d20kh1+4").unwrap().range(), (5, 24));
        assert_eq!(Dice::parse("4d6kh3").unwrap().range(), (3, 18));
    }
    #[test]
    fn rejects_invalid_or_unbounded_expressions() {
        for text in [
            "",
            "1d0",
            "1001d6",
            "1d1000001",
            "1d6++2",
            "1d6junk",
            "1d6 * 2",
            "2d20k1",
            "2d20kh0",
            "2d20kh3",
            "2d20kx1",
            "9223372036854775807 + 1",
        ] {
            assert!(Dice::parse(text).is_err(), "{text}");
        }
    }
    #[test]
    fn seeded_stream_has_stable_known_values() {
        let mut rng = Random::seeded(0);
        assert_eq!(rng.next(), 0xe220a8397b1dcdaf);
        assert_eq!(rng.next(), 0x6e789e6aa1b965f4);
        let mut a = Random::seeded(42);
        let mut b = Random::seeded(42);
        for _ in 0..100 {
            assert_eq!(a.die(20).unwrap(), b.die(20).unwrap());
        }
    }
}
