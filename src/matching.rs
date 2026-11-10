//! Typed words against names and note text: each word on its own, in any order, and with a
//! small typo allowed. "healing potion" and "potoin of healng" both find Potion of Healing.

/// Small words a search can leave out, so that word order does not matter.
const FILLER: [&str; 5] = ["a", "an", "and", "of", "the"];

/// The words in `text`, split where a letter or digit stops.
pub(crate) fn words(text: &str) -> impl Iterator<Item = &str> {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|word| !word.is_empty())
}

/// The rank `name_fit` gives a name found only with a typo.
pub(crate) const TYPO: u8 = 6;

/// How a lower-case name fits a lower-case search, best first: the exact name, a prefix, a
/// whole word or phrase, anywhere in the name (0 to 3, as `lookup::name_rank`), then each
/// typed word in any order, as whole words (4) or as the start of words (5), then a typo
/// (`TYPO`).
pub(crate) fn name_fit(name: &str, query: &str, typed: &Typed) -> Option<u8> {
    crate::lookup::name_rank(name, query).or_else(|| typed.in_name(words(name)).map(words_fit))
}

/// The rank to sort a row by when the row can also match by something other than its name,
/// such as its facts or its path: the name as typed (as `name_fit` ranks it), then the other
/// match, then a typo in the name. A rank above `TYPO` was found only with a typo.
pub(crate) fn row_rank(fit: Option<u8>, other: bool) -> u8 {
    match fit {
        Some(rank) if rank < TYPO => rank,
        _ if other => TYPO,
        Some(_) => TYPO + 1,
        None => TYPO,
    }
}

/// The rank of a name found by its words, as `name_fit` ranks it.
pub(crate) fn words_fit(fit: Fit) -> u8 {
    match fit {
        Fit::Exact => 4,
        Fit::Start => 5,
        Fit::Typo => TYPO,
    }
}

/// The different words in lower-case `text`, sorted, for a search of note text.
pub(crate) fn word_set(text: &str) -> Vec<String> {
    let mut set: Vec<String> = words(text).map(str::to_owned).collect();
    set.sort_unstable();
    set.dedup();
    set
}

/// How well a typed word fits a word. Better fits sort first.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum Fit {
    Exact,
    /// The typed word starts the word, as when the DM is still typing.
    Start,
    /// The typed word is a small typo away from the word.
    Typo,
}

/// The words of a lower-case search, prepared once for each search.
pub(crate) struct Typed {
    words: Vec<String>,
}

impl Typed {
    pub(crate) fn new(query: &str) -> Self {
        let all: Vec<&str> = words(query).collect();
        let last = all.len().saturating_sub(1);
        // The last word stays: "a" in "dragon a" can be the start of a word still being typed.
        let words = all
            .iter()
            .enumerate()
            .filter(|&(at, word)| at == last || !FILLER.contains(word))
            .map(|(_, word)| (*word).to_owned())
            .collect();
        Self { words }
    }

    /// The worst fit when each typed word fits a different word of the name, in any order.
    /// The assignment with the best worst fit wins.
    pub(crate) fn in_name<'a>(&self, name: impl IntoIterator<Item = &'a str>) -> Option<Fit> {
        if self.words.is_empty() {
            return None;
        }
        let name: Vec<&str> = name.into_iter().collect();
        if name.len() < self.words.len() {
            return None;
        }
        let fits: Vec<Vec<Option<Fit>>> = self
            .words
            .iter()
            .map(|typed| name.iter().map(|word| fit(word, typed)).collect())
            .collect();
        [Fit::Exact, Fit::Start, Fit::Typo]
            .into_iter()
            .find(|&worst| assign(&fits, worst))
    }

    /// The worst fit when each typed word fits some word of the text. `set` is sorted, as
    /// `word_set` makes it. A word of the text can fit more than one typed word.
    pub(crate) fn in_text(&self, set: &[String]) -> Option<Fit> {
        if self.words.is_empty() {
            return None;
        }
        self.words
            .iter()
            .map(|typed| best_in_set(set, typed))
            .try_fold(Fit::Exact, |worst, fit| Some(worst.max(fit?)))
    }
}

/// True if each typed word can have its own word with a fit no worse than `worst`. This is a
/// bipartite matching (Kuhn's augmenting paths), so repeated words cannot make it slow.
fn assign(fits: &[Vec<Option<Fit>>], worst: Fit) -> bool {
    let words = fits.first().map_or(0, Vec::len);
    let mut owner = vec![None; words];
    (0..fits.len()).all(|typed| augment(fits, worst, typed, &mut vec![false; words], &mut owner))
}

/// Give `typed` a word, moving other typed words to other words if that makes room.
fn augment(
    fits: &[Vec<Option<Fit>>],
    worst: Fit,
    typed: usize,
    seen: &mut [bool],
    owner: &mut [Option<usize>],
) -> bool {
    for word in 0..owner.len() {
        if seen[word] || fits[typed][word].is_none_or(|fit| fit > worst) {
            continue;
        }
        seen[word] = true;
        if owner[word].is_none_or(|other| augment(fits, worst, other, seen, owner)) {
            owner[word] = Some(typed);
            return true;
        }
    }
    false
}

fn best_in_set(set: &[String], typed: &str) -> Option<Fit> {
    let from = set.partition_point(|word| word.as_str() < typed);
    match set.get(from) {
        Some(word) if word == typed => return Some(Fit::Exact),
        Some(word) if word.starts_with(typed) => return Some(Fit::Start),
        _ => {}
    }
    let limit = typos_allowed(typed);
    (limit > 0 && set.iter().any(|word| within(word, typed, limit))).then_some(Fit::Typo)
}

fn fit(word: &str, typed: &str) -> Option<Fit> {
    if word == typed {
        Some(Fit::Exact)
    } else if word.starts_with(typed) {
        Some(Fit::Start)
    } else {
        within(word, typed, typos_allowed(typed)).then_some(Fit::Typo)
    }
}

/// Short words get no typo, because too many other short words are one change away.
fn typos_allowed(typed: &str) -> usize {
    match typed.chars().count() {
        0..=3 => 0,
        4..=7 => 1,
        _ => 2,
    }
}

/// True if no more than `limit` changes turn `a` into `b`. A change adds, removes or replaces
/// one letter, or swaps two letters beside each other.
fn within(a: &str, b: &str, limit: usize) -> bool {
    if limit == 0 {
        return a == b;
    }
    let (a_len, b_len) = (a.chars().count(), b.chars().count());
    if a_len.abs_diff(b_len) > limit {
        return false;
    }
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    // Three rows of the edit table: two rows back, the last row, and this row.
    let mut before: Vec<usize> = vec![0; b.len() + 1];
    let mut last: Vec<usize> = (0..=b.len()).collect();
    let mut row: Vec<usize> = vec![0; b.len() + 1];
    for i in 1..=a.len() {
        row[0] = i;
        let mut least = row[0];
        for j in 1..=b.len() {
            let cost = usize::from(a[i - 1] != b[j - 1]);
            let mut value = (last[j] + 1).min(row[j - 1] + 1).min(last[j - 1] + cost);
            if i > 1 && j > 1 && a[i - 1] == b[j - 2] && a[i - 2] == b[j - 1] {
                value = value.min(before[j - 2] + 1);
            }
            row[j] = value;
            least = least.min(value);
        }
        if least > limit {
            return false;
        }
        std::mem::swap(&mut before, &mut last);
        std::mem::swap(&mut last, &mut row);
    }
    last[b.len()] <= limit
}

#[cfg(test)]
mod tests {
    use super::*;

    fn name(text: &str) -> Vec<&str> {
        words(text).collect()
    }

    #[test]
    fn words_match_in_any_order_without_small_words() {
        let potion = name("potion of healing (50 gp)");
        for typed in [
            "potion of healing",
            "healing potion",
            "potion healing",
            "heal pot",
        ] {
            assert!(
                Typed::new(typed).in_name(potion.iter().copied()) <= Some(Fit::Start),
                "{typed}"
            );
        }
        assert_eq!(
            Typed::new("of").in_name(potion.iter().copied()),
            Some(Fit::Exact)
        );
    }

    #[test]
    fn a_word_of_the_name_fits_only_one_typed_word() {
        assert_eq!(Typed::new("wolf wolf").in_name(name("wolf")), None);
        assert_eq!(Typed::new("wolf wolf").in_name(name("dire wolf")), None);
        // The second "pot" needs its own word, and "potion" is the only one.
        assert_eq!(
            Typed::new("pot pot").in_name(name("potion of healing")),
            None
        );
    }

    #[test]
    fn a_small_typo_fits_only_a_long_enough_word() {
        let potion = name("potion of healing");
        assert_eq!(
            Typed::new("potoin of healng").in_name(potion.iter().copied()),
            Some(Fit::Typo)
        );
        assert_eq!(
            Typed::new("potions of healing").in_name(potion.iter().copied()),
            Some(Fit::Typo)
        );
        // Two changes in a short word, and one change in a word of three letters, are not typos.
        assert_eq!(Typed::new("ptoino").in_name(name("potion")), None);
        assert_eq!(Typed::new("rat").in_name(name("bat")), None);
        assert_eq!(
            Typed::new("tarasquee").in_name(name("tarrasque")),
            Some(Fit::Typo)
        );
    }

    #[test]
    fn the_best_assignment_wins() {
        // "pot" taking "potion" first would leave "potion" only "pots", which does not fit.
        assert_eq!(
            Typed::new("pot potion").in_name(name("potion pots")),
            Some(Fit::Start)
        );
    }

    #[test]
    fn text_needs_each_typed_word_somewhere() {
        let set = word_set("the smugglers hide in the harbor at night");
        assert_eq!(
            Typed::new("harbor smugglers").in_text(&set),
            Some(Fit::Exact)
        );
        assert_eq!(Typed::new("smug harb").in_text(&set), Some(Fit::Start));
        assert_eq!(Typed::new("smuglers").in_text(&set), Some(Fit::Typo));
        assert_eq!(Typed::new("smugglers castle").in_text(&set), None);
    }

    #[test]
    fn a_name_ranks_as_typed_then_by_words_then_with_a_typo() {
        let rank = |name: &str, query: &str| name_fit(name, query, &Typed::new(query));
        assert_eq!(rank("potion of healing", "potion of healing"), Some(0));
        assert_eq!(rank("potion of healing", "potion"), Some(1));
        assert_eq!(rank("potion of healing", "healing"), Some(2));
        assert_eq!(rank("potion of healing", "healing potion"), Some(4));
        assert_eq!(rank("potions of healing", "healing potion"), Some(5));
        assert_eq!(rank("potion of healing", "healng potion"), Some(TYPO));
        assert_eq!(rank("potion of healing", "fire"), None);
    }

    #[test]
    fn many_repeated_words_are_matched_quickly() {
        // Backtracking took seconds here. A matching takes no time.
        let query = format!("{}zzzz", "xa ".repeat(30));
        let name = "xab ".repeat(40);
        assert_eq!(Typed::new(&query).in_name(words(&name)), None);
        let query = "xa ".repeat(30);
        assert_eq!(Typed::new(&query).in_name(words(&name)), Some(Fit::Start));
    }

    #[test]
    fn a_small_word_stays_while_it_is_the_last_word() {
        let dragon = name("ancient red dragon");
        // "a" can start "ancient", so the search does not widen while the DM types.
        assert_eq!(
            Typed::new("dragon a").in_name(dragon.iter().copied()),
            Some(Fit::Start)
        );
        assert_eq!(Typed::new("dragon b").in_name(dragon.iter().copied()), None);
        assert_eq!(
            Typed::new("the red dragon").in_name(dragon.iter().copied()),
            Some(Fit::Exact)
        );
    }

    #[test]
    fn a_row_found_another_way_is_not_a_typo() {
        assert_eq!(row_rank(Some(4), false), 4);
        assert_eq!(row_rank(Some(TYPO), true), TYPO);
        assert_eq!(row_rank(None, true), TYPO);
        assert_eq!(row_rank(Some(TYPO), false), TYPO + 1);
    }

    #[test]
    fn swapped_letters_are_one_change() {
        assert!(within("potion", "potoin", 1));
        assert!(within("healing", "healng", 1));
        assert!(!within("healing", "heeling", 0));
        assert!(within("tarrasque", "tarasquee", 2));
        assert!(!within("tarrasque", "tarasqu", 1));
    }
}
