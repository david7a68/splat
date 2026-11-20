//! Creature facts for finding creatures: habitat, type, size, alignment, challenge,
//! movement, senses, languages and damage or condition defences. The values come from
//! the creature's Stat block fields. A filter keeps the creatures that have one of the
//! chosen values for each fact, and search words can name these facts too.
use super::Entry;
use serde_yaml_ng::Value;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Facet {
    Habitat,
    Type,
    Size,
    Alignment,
    Challenge,
    Movement,
    Senses,
    Languages,
    Resistances,
    Immunities,
    Vulnerabilities,
}

impl Facet {
    pub const ALL: [Facet; 11] = [
        Facet::Habitat,
        Facet::Type,
        Facet::Size,
        Facet::Alignment,
        Facet::Challenge,
        Facet::Movement,
        Facet::Senses,
        Facet::Languages,
        Facet::Resistances,
        Facet::Immunities,
        Facet::Vulnerabilities,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Facet::Habitat => "Habitat",
            Facet::Type => "Type",
            Facet::Size => "Size",
            Facet::Alignment => "Alignment",
            Facet::Challenge => "Challenge",
            Facet::Movement => "Movement",
            Facet::Senses => "Senses",
            Facet::Languages => "Languages",
            Facet::Resistances => "Resistances",
            Facet::Immunities => "Immunities",
            Facet::Vulnerabilities => "Vulnerabilities",
        }
    }

    /// The words that name this fact in a search, such as `habitat:forest`.
    fn search_names(self) -> &'static [&'static str] {
        match self {
            Facet::Habitat => &["habitat", "environment", "env", "terrain"],
            Facet::Type => &["type"],
            Facet::Size => &["size"],
            Facet::Alignment => &["alignment", "align"],
            Facet::Challenge => &["cr", "challenge"],
            Facet::Movement => &["speed", "movement", "move"],
            Facet::Senses => &["senses", "sense"],
            Facet::Languages => &["languages", "language", "lang"],
            Facet::Resistances => &["resistances", "resistance", "resists", "resist"],
            Facet::Immunities => &["immunities", "immunity", "immune"],
            Facet::Vulnerabilities => &["vulnerabilities", "vulnerability", "vulnerable", "vuln"],
        }
    }

    fn named(word: &str) -> Option<Facet> {
        Facet::ALL
            .into_iter()
            .find(|facet| facet.search_names().contains(&word))
    }
}

const SIZES: [&str; 6] = ["Tiny", "Small", "Medium", "Large", "Huge", "Gargantuan"];
const TYPES: [&str; 14] = [
    "Aberration",
    "Beast",
    "Celestial",
    "Construct",
    "Dragon",
    "Elemental",
    "Fey",
    "Fiend",
    "Giant",
    "Humanoid",
    "Monstrosity",
    "Ooze",
    "Plant",
    "Undead",
];
const DEFENCES: [&str; 27] = [
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
    "Blinded",
    "Charmed",
    "Deafened",
    "Exhaustion",
    "Frightened",
    "Grappled",
    "Incapacitated",
    "Invisible",
    "Paralyzed",
    "Petrified",
    "Poisoned",
    "Prone",
    "Restrained",
    "Stunned",
];
const SENSES: [&str; 4] = ["Blindsight", "Darkvision", "Tremorsense", "Truesight"];
const MOVEMENT: [&str; 4] = ["Burrow", "Climb", "Fly", "Swim"];
/// A creature of any habitat is found under every habitat.
pub const ANY_HABITAT: &str = "Any";

/// The facts of one creature, as display words.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Facets {
    values: Vec<(Facet, String)>,
    /// Type tags, such as Devil, which a search can find but the filter does not list.
    tags: Vec<String>,
    challenge: Option<f64>,
}

impl Facets {
    /// The facts of a creature entry, or none for any other kind of entry.
    pub fn of(entry: &Entry) -> Option<Facets> {
        if entry.kind != "creature" {
            return None;
        }
        let creature = entry.fields.get("splat_creature");
        let text = |key: &str| {
            creature
                .and_then(|c| c.get(key))
                .and_then(Value::as_str)
                .map(str::trim)
                .filter(|s| !s.is_empty())
        };
        let mut facets = Facets::default();
        let size = text("size_options_text").or(text("size")).unwrap_or("");
        // `Large Swarm of Tiny Beasts` is a Large creature.
        let size = size.split(" Swarm").next().unwrap_or(size);
        facets.add_known(Facet::Size, size, &SIZES);
        let kind = text("type_options_text").or(text("type")).unwrap_or("");
        facets.add_known(Facet::Type, kind.split(" (").next().unwrap_or(kind), &TYPES);
        for tags in [text("tags_text"), kind.split_once(" (").map(|(_, t)| t)]
            .into_iter()
            .flatten()
        {
            for tag in split_top(tags.trim_matches(['(', ')']), &[',']) {
                if !tag.is_empty() && !tag.eq_ignore_ascii_case("your choice") {
                    facets.tags.push(title_words(&tag));
                }
            }
        }
        if let Some(alignment) = text("alignment") {
            facets.add(Facet::Alignment, title_words(alignment));
        }
        if let Some(cr) = text("cr_text").and_then(|t| t.split_whitespace().next())
            && let Some(value) = challenge_value(cr)
        {
            facets.challenge = Some(value);
            facets.add(Facet::Challenge, cr.to_owned());
        }
        if let Some(speed) = text("movement_text") {
            facets.add_words(Facet::Movement, speed, &MOVEMENT);
            if speed.to_lowercase().contains("hover") {
                facets.add(Facet::Movement, "Hover".into());
            }
        }
        if let Some(senses) = text("senses_text") {
            facets.add_words(Facet::Senses, senses, &SENSES);
        }
        if let Some(languages) = text("languages_text") {
            for language in languages_in(languages) {
                facets.add(Facet::Languages, language);
            }
        }
        for (facet, key) in [
            (Facet::Resistances, "resistances_text"),
            (Facet::Immunities, "immunities_text"),
            (Facet::Vulnerabilities, "vulnerabilities_text"),
        ] {
            for piece in text(key)
                .map(|t| split_top(t, &[',', ';']))
                .unwrap_or_default()
            {
                let word = piece.trim_start_matches("and ").split_whitespace().next();
                facets.add_known(facet, word.unwrap_or(""), &DEFENCES);
            }
        }
        if let Some(habitat) = text("habitat_text") {
            for place in habitats_in(habitat) {
                facets.add(Facet::Habitat, place);
            }
        }
        Some(facets)
    }

    pub fn values(&self, facet: Facet) -> impl Iterator<Item = &str> {
        self.values
            .iter()
            .filter(move |(f, _)| *f == facet)
            .map(|(_, v)| v.as_str())
    }

    pub fn challenge(&self) -> Option<f64> {
        self.challenge
    }

    fn has(&self, facet: Facet, value: &str) -> bool {
        self.values(facet).any(|v| v.eq_ignore_ascii_case(value))
            || (facet == Facet::Habitat && self.values(facet).any(|v| v == ANY_HABITAT))
    }

    /// A word from a search, found in any fact. `fire` finds fire immunity and the
    /// Elemental Plane of Fire.
    fn mentions(&self, word: &str) -> bool {
        self.values
            .iter()
            .map(|(_, v)| v)
            .chain(&self.tags)
            .any(|v| v.to_lowercase().contains(word))
    }

    fn add(&mut self, facet: Facet, value: String) {
        if !value.is_empty() && !self.has_exact(facet, &value) {
            self.values.push((facet, value));
        }
    }

    fn has_exact(&self, facet: Facet, value: &str) -> bool {
        self.values.iter().any(|(f, v)| *f == facet && v == value)
    }

    /// Each known word that the text names, such as `Large or Medium`.
    fn add_known(&mut self, facet: Facet, text: &str, known: &[&str]) {
        self.add_words(facet, text, known);
    }

    fn add_words(&mut self, facet: Facet, text: &str, known: &[&str]) {
        let words: Vec<String> = text
            .split(|c: char| !c.is_alphanumeric())
            .map(str::to_lowercase)
            .collect();
        for word in known {
            if words.iter().any(|w| *w == word.to_lowercase()) {
                self.add(facet, (*word).to_owned());
            }
        }
    }
}

/// `1/4` is 0.25. A creature without a number has no challenge.
pub fn challenge_value(text: &str) -> Option<f64> {
    match text.split_once('/') {
        Some((top, bottom)) => {
            let bottom: f64 = bottom.parse().ok()?;
            (bottom > 0.0).then_some(top.parse::<f64>().ok()? / bottom)
        }
        None => text.parse().ok(),
    }
}

fn title_words(text: &str) -> String {
    text.split(' ')
        .map(|word| {
            let mut chars = word.chars();
            chars.next().map_or_else(String::new, |first| {
                first.to_uppercase().chain(chars).collect::<String>()
            })
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Pieces between separators that are outside parentheses.
fn split_top(text: &str, separators: &[char]) -> Vec<String> {
    let mut pieces = Vec::new();
    let mut depth = 0usize;
    let mut piece = String::new();
    for c in text.chars() {
        match c {
            '(' => depth += 1,
            ')' => depth = depth.saturating_sub(1),
            _ => {}
        }
        if depth == 0 && separators.contains(&c) {
            pieces.push(piece.trim().to_owned());
            piece.clear();
        } else {
            piece.push(c);
        }
    }
    pieces.push(piece.trim().to_owned());
    pieces.retain(|p| !p.is_empty());
    pieces
}

/// `Forest, Planar (Abyss, Nine Hells)` is Forest, Planar (Abyss) and Planar (Nine Hells).
fn habitats_in(text: &str) -> Vec<String> {
    let mut places = Vec::new();
    for piece in split_top(text, &[',', ';']) {
        match piece.split_once(" (") {
            Some((head, planes)) => {
                for plane in split_top(planes.trim_end_matches(')'), &[',']) {
                    places.push(format!("{} ({plane})", title_words(head)));
                }
            }
            None => places.push(title_words(&piece)),
        }
    }
    places
}

/// Language names from a printed Languages line. Telepathy counts as one language.
/// `Understands Common and Draconic but can’t speak` names Common and Draconic.
fn languages_in(text: &str) -> Vec<String> {
    let mut languages = Vec::new();
    for piece in split_top(text, &[',', ';']) {
        let lower = piece.to_lowercase();
        if lower.contains("telepathy") {
            languages.push("Telepathy".to_owned());
            continue;
        }
        let mut piece = piece.as_str();
        for prefix in ["and ", "Understands ", "understands "] {
            piece = piece.strip_prefix(prefix).unwrap_or(piece);
        }
        let piece = piece
            .split(" but ")
            .next()
            .unwrap_or(piece)
            .split(" plus ")
            .next()
            .unwrap_or(piece);
        for name in piece.split(" and ") {
            let name = name.split(" (").next().unwrap_or(name).trim();
            let words = name.split_whitespace().count();
            if name.chars().next().is_some_and(char::is_uppercase)
                && words <= 2
                && name != "None"
                && !languages.iter().any(|l| l == name)
            {
                languages.push(name.to_owned());
            }
        }
    }
    languages
}

/// The chosen values for each fact, and a challenge range.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Filter {
    pub chosen: BTreeMap<Facet, BTreeSet<String>>,
    pub challenge: (Option<f64>, Option<f64>),
}

impl Filter {
    pub fn is_empty(&self) -> bool {
        self.chosen.values().all(BTreeSet::is_empty)
            && self.challenge.0.is_none()
            && self.challenge.1.is_none()
    }

    pub fn count(&self) -> usize {
        self.chosen.values().map(BTreeSet::len).sum::<usize>()
            + usize::from(self.challenge != (None, None))
    }

    pub fn toggle(&mut self, facet: Facet, value: &str) {
        let values = self.chosen.entry(facet).or_default();
        if !values.remove(value) {
            values.insert(value.to_owned());
        }
        if values.is_empty() {
            self.chosen.remove(&facet);
        }
    }

    pub fn is_chosen(&self, facet: Facet, value: &str) -> bool {
        self.chosen.get(&facet).is_some_and(|v| v.contains(value))
    }

    /// One of the chosen values for each fact, and a challenge in range.
    pub fn matches(&self, facets: &Facets) -> bool {
        self.chosen
            .iter()
            .all(|(facet, values)| values.iter().any(|v| facets.has(*facet, v)))
            && in_range(facets.challenge, self.challenge)
    }
}

fn in_range(value: Option<f64>, (low, high): (Option<f64>, Option<f64>)) -> bool {
    if low.is_none() && high.is_none() {
        return true;
    }
    value.is_some_and(|v| low.is_none_or(|l| v >= l) && high.is_none_or(|h| v <= h))
}

/// The values that a set of creatures has, with how many creatures have each.
/// Challenge values sort by number; others alphabetically.
pub fn choices<'a>(
    creatures: impl IntoIterator<Item = &'a Facets>,
) -> BTreeMap<Facet, Vec<(String, usize)>> {
    let mut counts: BTreeMap<Facet, BTreeMap<String, usize>> = BTreeMap::new();
    for facets in creatures {
        for (facet, value) in &facets.values {
            *counts
                .entry(*facet)
                .or_default()
                .entry(value.clone())
                .or_default() += 1;
        }
    }
    counts
        .into_iter()
        .map(|(facet, values)| {
            let mut values: Vec<_> = values.into_iter().collect();
            match facet {
                Facet::Challenge => values.sort_by(|a, b| {
                    challenge_value(&a.0)
                        .partial_cmp(&challenge_value(&b.0))
                        .unwrap_or(std::cmp::Ordering::Equal)
                }),
                Facet::Size => values.sort_by_key(|(v, _)| SIZES.iter().position(|s| s == v)),
                _ => {}
            }
            (facet, values)
        })
        .collect()
}

/// Search text: plain words and `fact:value` terms, such as `dragon habitat:coastal cr:5-10`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Query {
    /// The plain words, lowercase and joined by single spaces.
    pub words: String,
    terms: Vec<(Facet, String)>,
    challenge: (Option<f64>, Option<f64>),
}

impl Query {
    pub fn parse(text: &str) -> Query {
        let mut query = Query::default();
        let mut words = Vec::new();
        for token in quoted_tokens(&text.to_lowercase()) {
            if let Some((name, value)) = token.split_once(':')
                && let Some(facet) = Facet::named(name)
                && !value.is_empty()
            {
                if facet == Facet::Challenge
                    && let Some(range) = challenge_range(value)
                {
                    query.challenge = range;
                } else {
                    query.terms.push((facet, value.to_owned()));
                }
            } else {
                words.push(token);
            }
        }
        query.words = words.join(" ");
        query
    }

    /// Terms that only a creature can satisfy.
    pub fn names_facts(&self) -> bool {
        !self.terms.is_empty() || self.challenge != (None, None)
    }

    /// Every term is in its fact, and every plain word is in the name or a fact.
    /// The caller decides how a name alone matches; this checks the facts.
    pub fn matches_facts(&self, facets: &Facets) -> bool {
        self.terms.iter().all(|(facet, term)| {
            facets
                .values(*facet)
                .any(|v| v.to_lowercase().contains(term.as_str()))
                || (*facet == Facet::Habitat && facets.values(*facet).any(|v| v == ANY_HABITAT))
                || (*facet == Facet::Type
                    && facets
                        .tags
                        .iter()
                        .any(|t| t.to_lowercase().contains(term.as_str())))
        }) && in_range(facets.challenge, self.challenge)
    }

    /// Each plain word is in the name or in a fact.
    pub fn words_in(&self, name: &str, facets: &Facets) -> bool {
        let name = name.to_lowercase();
        self.words
            .split(' ')
            .filter(|w| !w.is_empty())
            .all(|word| name.contains(word) || facets.mentions(word))
    }
}

/// Whitespace-separated tokens. Double quotes keep spaces: `habitat:"nine hells"`.
fn quoted_tokens(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut token = String::new();
    let mut quoted = false;
    for c in text.chars() {
        match c {
            '"' => quoted = !quoted,
            c if c.is_whitespace() && !quoted => {
                if !token.is_empty() {
                    tokens.push(std::mem::take(&mut token));
                }
            }
            c => token.push(c),
        }
    }
    if !token.is_empty() {
        tokens.push(token);
    }
    tokens
}

/// `5`, `1/4`, `5-10`, `5+` and `-2` (2 or less).
fn challenge_range(text: &str) -> Option<(Option<f64>, Option<f64>)> {
    if let Some(low) = text.strip_suffix('+') {
        return Some((Some(challenge_value(low)?), None));
    }
    if let Some(high) = text.strip_prefix('-') {
        return Some((None, Some(challenge_value(high)?)));
    }
    if let Some((low, high)) = text.split_once('-') {
        return Some((Some(challenge_value(low)?), Some(challenge_value(high)?)));
    }
    let value = challenge_value(text)?;
    Some((Some(value), Some(value)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn creature(fields: &str) -> Facets {
        let text = format!(
            "---\nsplat_schema: 2\nsplat_id: 8dc91c9b-4e6d-51e8-97a0-6f23bafeb1ef\nsplat_kind: creature\nsplat_name: Test\n---\n\n# Test\n\n## Stat block\n\n| Stat | Value |\n| --- | --- |\n{fields}\n"
        );
        let entry = Entry::parse("creatures/test.md", &text).expect("creature parses");
        Facets::of(&entry).expect("creature facets")
    }

    fn aboleth() -> Facets {
        creature(
            "| Size | Large |\n| Type | Aberration |\n| Alignment | Lawful Evil |\n| Speed | 10 ft., Swim 40 ft. |\n| Challenge | 10 (XP 5,900, or 7,200 in lair; PB +4) |\n| Senses | Darkvision 120 ft.; Passive Perception 20 |\n| Languages | Deep Speech; telepathy 120 ft. |\n| Habitat | Underdark, Underwater |",
        )
    }

    #[test]
    fn stat_block_rows_become_facts() {
        let facets = aboleth();
        let values = |facet| facets.values(facet).collect::<Vec<_>>();
        assert_eq!(values(Facet::Size), ["Large"]);
        assert_eq!(values(Facet::Type), ["Aberration"]);
        assert_eq!(values(Facet::Alignment), ["Lawful Evil"]);
        assert_eq!(values(Facet::Challenge), ["10"]);
        assert_eq!(values(Facet::Movement), ["Swim"]);
        assert_eq!(values(Facet::Senses), ["Darkvision"]);
        assert_eq!(values(Facet::Languages), ["Deep Speech", "Telepathy"]);
        assert_eq!(values(Facet::Habitat), ["Underdark", "Underwater"]);
    }

    #[test]
    fn printed_lists_name_only_their_values() {
        let facets = creature(
            "| Size | Medium |\n| Type | Fiend (Devil) |\n| Alignment | Lawful Evil |\n| Speed | 30 ft., Fly 40 ft. (hover) |\n| Challenge | 1/4 (XP 50; PB +2) |\n| Languages | Understands Common and Draconic but can’t speak |\n| Resistances | Cold; Piercing damage from weapons wielded by creatures under the effect of a Bless spell |\n| Immunities | Fire, Poison; Charmed (except from its vampire master), Poisoned |\n| Habitat | Forest, Planar (Abyss, Nine Hells) |",
        );
        let values = |facet| facets.values(facet).collect::<Vec<_>>();
        assert_eq!(values(Facet::Movement), ["Fly", "Hover"]);
        assert_eq!(values(Facet::Languages), ["Common", "Draconic"]);
        assert_eq!(values(Facet::Resistances), ["Cold", "Piercing"]);
        assert_eq!(
            values(Facet::Immunities),
            ["Fire", "Poison", "Charmed", "Poisoned"]
        );
        assert_eq!(
            values(Facet::Habitat),
            ["Forest", "Planar (Abyss)", "Planar (Nine Hells)"]
        );
        assert_eq!(facets.challenge(), Some(0.25));
        assert!(Query::parse("type:devil").matches_facts(&facets));
    }

    #[test]
    fn a_filter_needs_one_chosen_value_of_each_fact() {
        let facets = aboleth();
        let mut filter = Filter::default();
        filter.toggle(Facet::Habitat, "Forest");
        assert!(!filter.matches(&facets));
        filter.toggle(Facet::Habitat, "Underwater");
        assert!(filter.matches(&facets));
        filter.toggle(Facet::Size, "Huge");
        assert!(!filter.matches(&facets));
        filter.toggle(Facet::Size, "Huge");
        filter.challenge = (Some(11.0), None);
        assert!(!filter.matches(&facets));
        filter.challenge = (Some(5.0), Some(10.0));
        assert!(filter.matches(&facets));
        assert_eq!(filter.count(), 3);
    }

    #[test]
    fn any_habitat_is_found_under_every_habitat() {
        let facets = creature("| Habitat | Any |");
        let mut filter = Filter::default();
        filter.toggle(Facet::Habitat, "Arctic");
        assert!(filter.matches(&facets));
        assert!(Query::parse("habitat:desert").matches_facts(&facets));
    }

    #[test]
    fn search_terms_name_facts_and_plain_words_search_them() {
        let facets = aboleth();
        let query = Query::parse("Swim env:underdark cr:5-10");
        assert_eq!(query.words, "swim");
        assert!(query.names_facts());
        assert!(query.matches_facts(&facets));
        assert!(query.words_in("Aboleth", &facets));
        assert!(!Query::parse("cr:11+").matches_facts(&facets));
        assert!(!Query::parse("immune:fire").matches_facts(&facets));
        assert!(Query::parse("darkvision deep").words_in("Aboleth", &facets));
        assert!(!Query::parse("darkvision fire").words_in("Aboleth", &facets));
        assert!(Query::parse("language:\"deep speech\"").matches_facts(&facets));
        // An unknown fact name is a plain word.
        assert_eq!(Query::parse("note:x").words, "note:x");
    }

    #[test]
    fn every_builtin_creature_has_its_core_facts() {
        let entries = crate::content::definitions::entries().expect("built-in definitions");
        let creatures: Vec<_> = entries.iter().filter_map(Facets::of).collect();
        assert_eq!(creatures.len(), 336);
        for (facet, least) in [
            (Facet::Size, 336),
            (Facet::Type, 336),
            (Facet::Alignment, 336),
            (Facet::Habitat, 330),
        ] {
            let with = creatures
                .iter()
                .filter(|c| c.values(facet).next().is_some())
                .count();
            assert!(with >= least, "{} on {with} creatures", facet.label());
        }
        if std::env::var_os("SPLAT_PRINT_FACETS").is_some() {
            for (facet, values) in choices(&creatures) {
                println!("{}: {values:?}", facet.label());
            }
        }
    }

    #[test]
    fn choices_count_creatures_and_sort_challenge_by_number() {
        let low = creature("| Size | Tiny |\n| Challenge | 1/2 (XP 100; PB +2) |");
        let high = aboleth();
        let choices = choices([&high, &low]);
        assert_eq!(
            choices[&Facet::Challenge],
            [("1/2".to_owned(), 1), ("10".to_owned(), 1)]
        );
        assert_eq!(
            choices[&Facet::Size],
            [("Tiny".to_owned(), 1), ("Large".to_owned(), 1)]
        );
    }
}
