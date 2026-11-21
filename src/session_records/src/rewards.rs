use splat::content::loot::{self, Loot};
use splat_campaign::encounter_run::{State, read_saved};
/// Whose loot: the encounter's, or one participant's.
#[derive(Clone, Debug, PartialEq)]
pub struct Holder {
    /// `encounter`, or the participant's key, which a name change does not move.
    pub key: String,
    pub name: String,
    pub lines: Vec<String>,
    /// The encounter, or a creature at 0 HP. The ended view lists these first.
    pub open: bool,
}

/// The owner key of the encounter's own loot. A participant's key is never this word alone.
const ENCOUNTER: &str = "encounter";

/// Where one line's results are kept. Two lines with the same table keep their own.
pub fn place(holder: &str, line: usize) -> String {
    format!("{holder}#{line}")
}

/// `a`, `a and b`, `a, b and c`.
pub fn and_list<S: std::borrow::Borrow<str>>(words: &[S]) -> String {
    match words.split_last() {
        Some((last, rest)) if !rest.is_empty() => {
            format!("{} and {}", rest.join(", "), last.borrow())
        }
        _ => words.concat(),
    }
}

/// `gems`, or `2 × gems`, as the roster and Arrivals count creatures.
pub fn plain_roll(times: u32, key: &str) -> String {
    if times == 1 {
        key.to_owned()
    } else {
        format!("{times} × {key}")
    }
}

/// Everyone with loot: the encounter first, then participants in turn order. `prep` is the
/// run's read-aloud text and notes, whose `## Loot` list is the encounter's.
pub fn holders<C>(name: &str, prep: &str, state: &State<C>) -> Vec<Holder> {
    let mut holders = Vec::new();
    let own = loot::items(prep);
    if !own.is_empty() {
        holders.push(Holder {
            key: ENCOUNTER.into(),
            name: name.to_owned(),
            lines: own,
            open: true,
        });
    }
    for &index in &state.order {
        let p = &state.participants[index];
        let lines = p.loot_lines();
        if !lines.is_empty() {
            holders.push(Holder {
                key: p.key.clone(),
                name: p.name.clone(),
                lines,
                open: p.hp == Some(0),
            });
        }
    }
    holders
}

/// One holder's loot as lines for a session note: each roll's results, or the roll with
/// `(not rolled)`, and words as the note has them. The flag marks a roll not made.
pub fn note_lines<C>(state: &State<C>, holder: &Holder) -> Vec<(String, bool)> {
    let mut out = Vec::new();
    for (at, line) in holder.lines.iter().enumerate() {
        match (Loot::parse(line), state.loot.get(&place(&holder.key, at))) {
            (Loot::Roll { .. }, Some(found)) => {
                out.extend(found.iter().map(|one| (one.text.clone(), false)));
            }
            (Loot::Roll { times, key }, None) => {
                out.push((format!("{} (not rolled)", plain_roll(times, &key)), true));
            }
            (Loot::Text(text), _) => out.push((text, false)),
        }
    }
    out
}

/// What an ended fight gives its session note (F7), read from its saved run: the creatures
/// against the party at 0 HP, also one marked absent, and what still stood, each with its XP
/// when Splat knows it; the characters who were there; and the encounter's loot and each
/// downed creature's.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Rewards {
    pub downed: Vec<(String, Option<u64>)>,
    pub standing: Vec<Option<u64>>,
    pub characters: usize,
    pub loot: Vec<String>,
    /// How many loot rolls were not made.
    pub not_rolled: usize,
}

/// The rewards of the run saved at `path`, read now.
pub fn rewards(path: &std::path::Path, campaign: uuid::Uuid) -> Result<Rewards, String> {
    let saved = read_saved::<()>(path, campaign)?;
    let state = &saved.state;
    let mut out = Rewards::default();
    for p in &state.participants {
        let xp = crate::difficulty::creature_xp(&p.definition);
        if p.side.trim() == "party" {
            let character = crate::difficulty::character_level(&p.definition).is_some();
            out.characters += usize::from(character && p.present);
        } else if p.hp == Some(0) {
            out.downed.push((p.name.clone(), xp));
        } else if p.present {
            out.standing.push(xp);
        }
    }
    let prep = crate::encounter_run::prep_parts(&saved.setup_details).0;
    for holder in holders(&saved.name, prep, state)
        .iter()
        .filter(|holder| holder.open)
    {
        for (line, not_rolled) in note_lines(state, holder) {
            out.loot.push(line);
            out.not_rolled += usize::from(not_rolled);
        }
    }
    Ok(out)
}

impl Rewards {
    /// The XP line, from the one XP answer (`Scale::weigh`): what it counted, each character's
    /// share and what still stood, as `XP 400 for 2 at 0 HP · 80 each for 5 · 1 still stood
    /// (XP 200)`. A creature whose XP Splat does not know is named, and its amount is left out
    /// of every sum, never counted as zero. With the line: its XP, and the count of those
    /// awards not known.
    pub fn xp_line(&self) -> (String, u64, u64) {
        let scale = crate::difficulty::Scale::default();
        let downed = scale.weigh(&[], self.downed.iter().map(|(_, xp)| *xp));
        let grouped = crate::encounter_run::grouped_number;
        let known = self.downed.len() as u64 - downed.unknown;
        // Round 27 FM4: with nothing counted, the line says so in words, not `XP 0 for 0 at 0 HP`.
        let mut line = if known == 0 && downed.unknown == 0 {
            "XP 0 · no creature at 0 HP".to_owned()
        } else if known == 0 {
            "XP 0".to_owned()
        } else {
            format!("XP {} for {known} at 0 HP", grouped(downed.xp))
        };
        if downed.unknown > 0 {
            let names: Vec<&str> = self
                .downed
                .iter()
                .filter(|(_, xp)| xp.is_none())
                .map(|(name, _)| name.as_str())
                .collect();
            line.push_str(&format!(
                ". XP not known for {}: enter an award here.",
                and_list(&names)
            ));
            return (line, downed.xp, downed.unknown);
        }
        if self.characters > 0 && downed.xp > 0 {
            line.push_str(&format!(
                " · {} each for {}",
                grouped(downed.xp / self.characters as u64),
                self.characters
            ));
        }
        if !self.standing.is_empty() {
            let standing = scale.weigh(&[], self.standing.iter().copied());
            line.push_str(&format!(" · {} still stood", self.standing.len()));
            if standing.unknown == 0 {
                line.push_str(&format!(" (XP {})", grouped(standing.xp)));
            }
        }
        (line, downed.xp, 0)
    }
}
