//! Typed words against names and note text: each word on its own, in any order, and with a
//! small typo allowed. "healing potion" and "potoin of healng" both find Potion of Healing.

/// Small words a search can leave out, so that word order does not matter.
const FILLER: [&str; 5] = ["a", "an", "and", "of", "the"];

/// The words in `text`, split where a letter or digit stops.
pub(crate) fn words(text: &str) -> impl Iterator<Item = &str> {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|word| !word.is_empty())
}

/// Where a result sorts. Every result found as typed sorts before every result found only
/// with a typo. Inside each group, a lower tier sorts first.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct Rank {
    pub(crate) typo: bool,
    tier: u8,
}

impl Rank {
    pub(crate) const fn typed(tier: u8) -> Self {
        Self { typo: false, tier }
    }

    const fn typo(tier: u8) -> Self {
        Self { typo: true, tier }
    }

    /// FI5, M3: where the creature order puts a name rank: a name that holds the search as a
    /// whole word (2) sorts with any other name that holds it (3). Private, so that only
    /// [`Typed::creature`] and [`creature_tier`] rank a creature.
    const fn creature(self) -> Self {
        if !self.typo && self.tier == 3 {
            Self::typed(2)
        } else {
            self
        }
    }
}

/// FI5, M3: the one creature search order for the search as typed, Arrivals' order: the exact
/// name (0), then a name that starts with the search (1), then a name that holds it anywhere
/// (2). Names in one tier keep the list's order. [`Typed::creature`] adds the typed words and
/// typos after these tiers.
pub(crate) fn creature_tier(name: &str, query: &str) -> Option<u8> {
    name_rank(name, query).map(|tier| Rank::typed(tier).creature().tier)
}

/// Shown when every result was found only with a typo.
pub(crate) const CLOSE_NAMES: &str = "No name matches as typed. These names are close.";

/// How a lower-case name fits a lower-case search as typed: the exact name (0), a prefix (1),
/// a whole word or phrase (2), or anywhere in the name (3).
pub(crate) fn name_rank(name: &str, query: &str) -> Option<u8> {
    if name == query {
        Some(0)
    } else if name.starts_with(query) {
        Some(1)
    } else if names_lower_word(name, query) {
        Some(2)
    } else if name.contains(query) {
        Some(3)
    } else {
        None
    }
}

/// True if lower-case `name` appears in lower-case `text` as whole words.
pub(crate) fn names_lower_word(text: &str, name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    let boundary = |c: Option<char>| c.is_none_or(|c| !c.is_alphanumeric());
    text.match_indices(name).any(|(at, _)| {
        boundary(text[..at].chars().next_back()) && boundary(text[at + name.len()..].chars().next())
    })
}

/// A row that can also match another way, such as by its facts, path or section: the name as
/// typed, then the other match (tier 6), then a typo in the name.
pub(crate) fn with_other(name: Option<Rank>, other: bool) -> Option<Rank> {
    match name {
        Some(rank) if !rank.typo => Some(rank),
        _ if other => Some(Rank::typed(6)),
        name => name,
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

/// A lower-case search, prepared once for each search.
pub(crate) struct Typed {
    phrase: String,
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
        Self {
            phrase: query.to_owned(),
            words,
        }
    }

    /// How a lower-case name fits: as `name_rank` ranks the search as typed, then each typed
    /// word in any order, as whole words (tier 4) or as the start of words (5), then with a
    /// typo.
    pub(crate) fn name(&self, name: &str) -> Option<Rank> {
        self.name_with_words(name, words(name))
    }

    /// M3: the one creature rank. Every search that lists creatures ranks a creature's
    /// lower-case name here (Lookup, the creature pickers, the encounter and group editors;
    /// Arrivals through [`creature_tier`]), so the DM meets one order on every screen: the exact
    /// name, then a name that starts with the search, then a name that holds it anywhere (a
    /// whole word or inside a word: `wolf` puts Werewolf before Winter Wolf, in the list's
    /// order), then the typed words in any order, then a typo. Other kinds keep
    /// [`Typed::name_with_words`], where a whole word ranks before a word that only holds it.
    pub(crate) fn creature<'a>(
        &self,
        name: &str,
        name_words: impl IntoIterator<Item = &'a str>,
    ) -> Option<Rank> {
        self.name_with_words(name, name_words).map(Rank::creature)
    }

    /// [`Typed::creature`] for a list that keeps only the names found as typed (the encounter
    /// search's `Use it`, the replacement picker): the same rank where the name fits as typed,
    /// and `None` where it fits only with a typo. No typo is worked out (review of PR 450).
    pub(crate) fn creature_as_typed<'a>(
        &self,
        name: &str,
        name_words: impl IntoIterator<Item = &'a str>,
    ) -> Option<Rank> {
        if let Some(tier) = name_rank(name, &self.phrase) {
            return Some(Rank::typed(tier).creature());
        }
        self.in_name_as_typed(name_words)
            .map(|fit| match fit {
                Fit::Exact => Rank::typed(4),
                _ => Rank::typed(5),
            })
            .map(Rank::creature)
    }

    /// `in_name` with the fits as typed only (`Exact`, `Start`): the same answer where it is
    /// one of those, and no typo comparison.
    fn in_name_as_typed<'a>(&self, name: impl IntoIterator<Item = &'a str>) -> Option<Fit> {
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
            .map(|typed| name.iter().map(|word| fit_as_typed(word, typed)).collect())
            .collect();
        [Fit::Exact, Fit::Start]
            .into_iter()
            .find(|&worst| assign(&fits, worst))
    }

    /// `name`, for a name whose words are already split.
    pub(crate) fn name_with_words<'a>(
        &self,
        name: &str,
        name_words: impl IntoIterator<Item = &'a str>,
    ) -> Option<Rank> {
        if let Some(tier) = name_rank(name, &self.phrase) {
            return Some(Rank::typed(tier));
        }
        self.in_name(name_words).map(|fit| match fit {
            Fit::Exact => Rank::typed(4),
            Fit::Start => Rank::typed(5),
            Fit::Typo => Rank::typo(0),
        })
    }

    /// A row found by its name or its lower-case text: the name as typed, then the text as
    /// typed (tier `from`), then each typed word in the text (`from + 1`) or the start of each
    /// (`from + 2`), then a typo in the name, then, last of all, a typo in the text.
    /// `text_words` is `word_set` of the text.
    pub(crate) fn with_text(
        &self,
        name: Option<Rank>,
        text: &str,
        text_words: &[String],
        from: u8,
    ) -> Option<Rank> {
        if name.is_some_and(|rank| !rank.typo) {
            return name;
        }
        if !self.phrase.is_empty() && text.contains(&self.phrase) {
            return Some(Rank::typed(from));
        }
        match self.in_text(text_words) {
            Some(Fit::Exact) => Some(Rank::typed(from + 1)),
            Some(Fit::Start) => Some(Rank::typed(from + 2)),
            _ if name.is_some() => name,
            Some(Fit::Typo) => Some(Rank::typo(1)),
            None => None,
        }
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

/// `fit` as typed: no typo.
fn fit_as_typed(word: &str, typed: &str) -> Option<Fit> {
    if word == typed {
        Some(Fit::Exact)
    } else if word.starts_with(typed) {
        Some(Fit::Start)
    } else {
        None
    }
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
    #[cfg(test)]
    TYPO_CHECKS.with(|checks| checks.set(checks.get() + 1));
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

// Typo comparisons (`within`) on this thread, for a test that a list as typed works out none.
#[cfg(test)]
thread_local! {
    pub(crate) static TYPO_CHECKS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
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
        let rank = |name: &str, query: &str| Typed::new(query).name(name);
        assert_eq!(
            rank("potion of healing", "potion of healing"),
            Some(Rank::typed(0))
        );
        assert_eq!(rank("potion of healing", "potion"), Some(Rank::typed(1)));
        assert_eq!(rank("potion of healing", "healing"), Some(Rank::typed(2)));
        assert_eq!(
            rank("potion of healing", "healing potion"),
            Some(Rank::typed(4))
        );
        assert_eq!(
            rank("potions of healing", "healing potion"),
            Some(Rank::typed(5))
        );
        assert_eq!(
            rank("potion of healing", "healng potion"),
            Some(Rank::typo(0))
        );
        assert_eq!(rank("potion of healing", "fire"), None);
        // A typo sorts after every tier found as typed.
        assert!(Rank::typed(9) < Rank::typo(0));
        // M3: a creature's name holds the search as a whole word or inside a word in one
        // tier, so the list's order decides Werewolf before Winter Wolf. Other kinds keep the
        // whole word first.
        let creature = |name: &str, query: &str| Typed::new(query).creature(name, words(name));
        assert_eq!(creature("wolf", "wolf"), Some(Rank::typed(0)));
        assert_eq!(creature("wolf spider", "wolf"), Some(Rank::typed(1)));
        assert_eq!(creature("winter wolf", "wolf"), Some(Rank::typed(2)));
        assert_eq!(creature("werewolf", "wolf"), Some(Rank::typed(2)));
        assert_eq!(rank("werewolf", "wolf"), Some(Rank::typed(3)));
        assert_eq!(
            creature("potion of healing", "healing potion"),
            Some(Rank::typed(4))
        );
        assert_eq!(creature("winter wolf", "wolf wintr"), Some(Rank::typo(0)));
        // Review of PR 450: the rank as typed is the creature rank where that fits as typed and
        // none where only a typo fits, and it compares no typo.
        let checks = TYPO_CHECKS.with(std::cell::Cell::get);
        for name in [
            "wolf",
            "wolf spider",
            "winter wolf",
            "werewolf",
            "potion of healing",
            "potions of healing",
            "dire bear",
        ] {
            for query in [
                "wolf",
                "healing potion",
                "wolf wintr",
                "healng potion",
                "bear",
            ] {
                let typed = Typed::new(query);
                assert_eq!(
                    typed.creature_as_typed(name, words(name)),
                    creature(name, query).filter(|rank| !rank.typo),
                    "{name} / {query}"
                );
            }
        }
        let after = TYPO_CHECKS.with(std::cell::Cell::get);
        let typo = Typed::new("wolf wintr");
        assert_eq!(
            typo.creature_as_typed("winter wolf", words("winter wolf")),
            None
        );
        assert_eq!(
            TYPO_CHECKS.with(std::cell::Cell::get),
            after,
            "a rank as typed compares no typo"
        );
        assert!(after > checks, "the creature rank above did compare typos");
        // Arrivals' tiers of the search as typed are the same order.
        for name in [
            "wolf",
            "wolf spider",
            "winter wolf",
            "werewolf",
            "dire bear",
        ] {
            assert_eq!(
                creature_tier(name, "wolf").map(Rank::typed),
                creature(name, "wolf").filter(|rank| rank.tier < 4),
                "{name}"
            );
        }
    }

    #[test]
    fn text_ranks_after_the_name_and_a_typo_in_text_comes_last() {
        let typed = Typed::new("smugglers");
        let text = "the smugglers hide here";
        let set = word_set(text);
        assert_eq!(
            typed.with_text(Some(Rank::typed(2)), text, &set, 7),
            Some(Rank::typed(2))
        );
        assert_eq!(typed.with_text(None, text, &set, 7), Some(Rank::typed(7)));
        let typed = Typed::new("hide smugglers");
        assert_eq!(typed.with_text(None, text, &set, 7), Some(Rank::typed(8)));
        assert_eq!(
            typed.with_text(Some(Rank::typo(0)), text, &set, 7),
            Some(Rank::typed(8))
        );
        let typed = Typed::new("smuglers");
        assert_eq!(
            typed.with_text(Some(Rank::typo(0)), text, &set, 7),
            Some(Rank::typo(0))
        );
        assert_eq!(typed.with_text(None, text, &set, 7), Some(Rank::typo(1)));
        assert_eq!(Typed::new("castle").with_text(None, text, &set, 7), None);
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
        assert_eq!(with_other(Some(Rank::typed(4)), true), Some(Rank::typed(4)));
        assert_eq!(with_other(Some(Rank::typo(0)), true), Some(Rank::typed(6)));
        assert_eq!(with_other(None, true), Some(Rank::typed(6)));
        assert_eq!(with_other(Some(Rank::typo(0)), false), Some(Rank::typo(0)));
        assert_eq!(with_other(None, false), None);
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
