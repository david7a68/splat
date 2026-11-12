//! Loot in a run. The encounter's loot is its note's `## Loot` list as the run began, and a
//! creature's is what its roster row or group member gave it. A creature's card shows one line
//! of it until the creature drops to 0 HP, then the whole list. The ended view lists the
//! encounter's loot and each downed creature's. A roll's result takes the place of its button,
//! the saved run keeps it, and Undo takes it back.
use super::{Command, Participant, Run, State};
use crate::design::{self, Action, Density, Type, colors, icons};
use crate::roll_table::TableRoll;
use eframe::egui;
use serde::{Deserialize, Serialize};
use splat::content::loot::{self, Loot};

/// What one roll found.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Found {
    pub value: i64,
    pub dice: String,
    pub text: String,
}

/// One loot line to roll, with the table it rolls on.
#[derive(Clone, Debug)]
pub(crate) struct LootRoll {
    /// The line's place: see `place`.
    pub place: String,
    pub owner: String,
    pub roll: TableRoll,
    pub times: u32,
}

/// Whose loot: the encounter's, or one participant's.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct Holder {
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
pub(super) fn place(holder: &str, line: usize) -> String {
    format!("{holder}#{line}")
}

/// Make each roll on the run's dice, keep what it found, and give the log a line for each.
pub(super) fn roll(state: &mut State, rolls: Vec<LootRoll>) -> Result<Vec<String>, String> {
    let mut lines = Vec::new();
    for loot in rolls {
        if state.loot.contains_key(&loot.place) {
            return Err("This loot has its roll. Undo it to roll again.".into());
        }
        let mut found = Vec::new();
        for _ in 0..loot.times {
            let (value, text) = loot.roll.draw(&mut state.random)?;
            lines.push(format!(
                "Loot for {}, {}: {value} on {}, {text}",
                loot.owner, loot.roll.key, loot.roll.dice
            ));
            found.push(Found {
                value,
                dice: loot.roll.dice.clone(),
                text,
            });
        }
        state.loot.insert(loot.place, found);
    }
    Ok(lines)
}

/// The Undo words for these rolls. Short, because the ended view's header holds them.
pub(super) fn describe(rolls: &[LootRoll]) -> String {
    match rolls {
        [one] if one.times == 1 => format!("loot roll on {}", one.roll.key),
        [one] => format!("{} loot rolls on {}", one.times, one.roll.key),
        all => format!(
            "{} loot rolls",
            all.iter().map(|roll| roll.times).sum::<u32>()
        ),
    }
}

/// `3`, `3 and 4`, `3, 4 and 1`.
fn values(found: &[Found]) -> String {
    let words: Vec<String> = found.iter().map(|f| f.value.to_string()).collect();
    and_list(&words)
}

/// `a`, `a and b`, `a, b and c`.
fn and_list<S: std::borrow::Borrow<str>>(words: &[S]) -> String {
    match words.split_last() {
        Some((last, rest)) if !rest.is_empty() => {
            format!("{} and {}", rest.join(", "), last.borrow())
        }
        _ => words.concat(),
    }
}

/// One thing found, after its mark: a bullet, or the dice for a roll's result. A Library
/// item's name opens it. `source` follows, muted, on the same line.
fn loot_line(ui: &mut egui::Ui, mark: &str, line: &str, source: Option<&str>) {
    let c = colors();
    ui.horizontal_wrapped(|ui| {
        // A line of text, not a row of controls: it is as tall as its words.
        ui.spacing_mut().interact_size.y = 0.0;
        ui.spacing_mut().item_spacing.x = design::XS;
        ui.label(Type::Body.rich(ui, mark).color(c.muted));
        match (
            crate::loot_editor::shown(line),
            crate::loot_editor::link_target(line),
        ) {
            (Some(name), Some(target)) => {
                if design::link(ui, name).clicked() {
                    crate::markdown_view::send_note_link(ui.ctx(), target.to_owned(), None);
                }
            }
            (Some(name), None) => {
                ui.label(Type::Body.rich(ui, name).color(c.text));
            }
            _ => {
                ui.add(egui::Label::new(Type::Body.rich(ui, line).color(c.text)).wrap());
            }
        }
        if let Some(source) = source {
            ui.label(Type::Small.rich(ui, format!("  {source}")).color(c.muted));
        }
    });
}

/// `gems`, or `2 × gems`, as the roster and Arrivals count creatures.
fn plain_roll(times: u32, key: &str) -> String {
    if times == 1 {
        key.to_owned()
    } else {
        format!("{times} × {key}")
    }
}

impl Participant {
    /// What this creature carries, one line each.
    pub(super) fn loot_lines(&self) -> Vec<String> {
        self.changes
            .as_ref()
            .and_then(|changes| changes.overrides.loot.as_deref())
            .map(loot::parts)
            .unwrap_or_default()
    }

    /// The creature is down and has loot to take.
    pub(super) fn loot_ready(&self) -> bool {
        self.hp == Some(0)
            && self
                .changes
                .as_ref()
                .is_some_and(|changes| changes.overrides.loot.is_some())
    }
}

/// Everyone with loot: the encounter first, then participants in turn order. `prep` is the
/// run's read-aloud text and notes, whose `## Loot` list is the encounter's.
fn holders(name: &str, prep: &str, state: &State) -> Vec<Holder> {
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
fn note_lines(state: &State, holder: &Holder) -> Vec<(String, bool)> {
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
pub(crate) struct Rewards {
    pub downed: Vec<(String, Option<u64>)>,
    pub standing: Vec<Option<u64>>,
    pub characters: usize,
    pub loot: Vec<String>,
    /// How many loot rolls were not made.
    pub not_rolled: usize,
}

/// The rewards of the run saved at `path`, read now.
pub(crate) fn rewards(path: &std::path::Path, campaign: uuid::Uuid) -> Result<Rewards, String> {
    let saved = super::read_saved(path, campaign)?;
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
    let prep = super::prep_parts(&saved.setup_details).0;
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
    pub(crate) fn xp_line(&self) -> (String, u64, u64) {
        let scale = crate::difficulty::Scale::default();
        let downed = scale.weigh(&[], self.downed.iter().map(|(_, xp)| *xp));
        let grouped = crate::encounter_run::grouped_number;
        let known = self.downed.len() as u64 - downed.unknown;
        let mut line = format!("XP {} for {known} at 0 HP", grouped(downed.xp));
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

impl Run {
    /// Everyone with loot: the encounter first, then participants in turn order.
    pub(super) fn holders(&self) -> Vec<Holder> {
        holders(&self.name, self.prep().0, &self.state)
    }

    /// The roll lines of these holders with no result yet, as `(holder, line)`.
    pub(super) fn unrolled<'a>(
        &self,
        holders: impl IntoIterator<Item = &'a Holder>,
    ) -> Vec<(&'a Holder, usize)> {
        let sources = self.content.as_ref().map(|index| index.loot_sources());
        holders
            .into_iter()
            .flat_map(|holder| {
                holder
                    .lines
                    .iter()
                    .enumerate()
                    .filter(|(at, line)| match Loot::parse(line) {
                        // A line whose table cannot roll waits for the DM to fix it. It never
                        // stops the others, and it is not counted as left to roll.
                        Loot::Roll { key, .. } => {
                            !self.state.loot.contains_key(&place(&holder.key, *at))
                                && sources
                                    .as_ref()
                                    .is_none_or(|sources| sources.problem(&key).is_none())
                        }
                        Loot::Text(_) => false,
                    })
                    .map(move |(at, _)| (holder, at))
            })
            .collect()
    }

    /// The rolls for these lines, on the campaign's tables. None if one cannot roll.
    pub(super) fn loot_rolls(&self, picks: &[(Holder, usize)]) -> Result<Vec<LootRoll>, String> {
        picks
            .iter()
            .filter_map(|(holder, at)| match Loot::parse(&holder.lines[*at]) {
                Loot::Roll { times, key } => Some((holder, *at, times, key)),
                Loot::Text(_) => None,
            })
            .map(|(holder, at, times, key)| {
                let roll = match self.content.as_ref() {
                    Some(index) => index.roll_table(&key),
                    None => Err(
                        "The campaign’s notes are still loading. Roll again in a moment.".into(),
                    ),
                }?;
                Ok(LootRoll {
                    place: place(&holder.key, at),
                    owner: holder.name.clone(),
                    roll,
                    times,
                })
            })
            .collect()
    }

    /// Roll these lines as one step.
    pub(super) fn roll_loot(&mut self, picks: &[(Holder, usize)]) {
        match self.loot_rolls(picks) {
            Ok(rolls) if !rolls.is_empty() => self.run_command(Command::Loot(rolls)),
            Ok(_) => {}
            Err(error) => self.error = Some(error),
        }
    }

    /// One holder's loot: words as the note has them, then each roll, with its button until it
    /// has a result, and then the result in its place. Returns the line the DM asked to roll.
    pub(super) fn loot_list(&self, ui: &mut egui::Ui, holder: &Holder) -> Option<usize> {
        let c = colors();
        let mut request = None;
        let sources = self.content.as_ref().map(|index| index.loot_sources());
        // One list: its lines sit as close as lines of a paragraph.
        ui.scope(|ui| {
            ui.spacing_mut().item_spacing.y = design::XS;
            for (at, line) in holder.lines.iter().enumerate() {
                let Loot::Roll { times, key } = Loot::parse(line) else {
                    loot_line(ui, "•", line, None);
                    continue;
                };
                match self.state.loot.get(&place(&holder.key, at)) {
                    // One result reads as one line, with where it came from at its end.
                    Some(found) if found.len() == 1 => {
                        let source = format!("{key} · {} on {}", values(found), found[0].dice);
                        loot_line(ui, icons::DICE, &found[0].text, Some(&source));
                    }
                    // Several share one muted heading, above them.
                    Some(found) => {
                        let dice = found.first().map_or("", |one| one.dice.as_str());
                        ui.label(
                            Type::Small
                                .rich(
                                    ui,
                                    format!(
                                        "{} · {} on {dice}",
                                        plain_roll(times, &key),
                                        values(found)
                                    ),
                                )
                                .color(c.muted),
                        );
                        for one in found {
                            loot_line(ui, icons::DICE, &one.text, None);
                        }
                    }
                    None => {
                        let problem = sources.as_ref().and_then(|sources| sources.problem(&key));
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = design::XS;
                            ui.label(Type::Body.rich(ui, icons::DICE).color(c.muted));
                            ui.label(Type::Body.rich(ui, plain_roll(times, &key)).color(c.text));
                            if let Some(table) =
                                sources.as_ref().and_then(|sources| sources.table(&key))
                                && problem.is_none()
                            {
                                ui.label(
                                    Type::Small
                                        .rich(ui, format!("· {}", table.dice))
                                        .color(c.muted),
                                );
                            }
                            if problem.is_some() {
                                design::status(
                                    ui,
                                    design::Status {
                                        icon: icons::WARN,
                                        word: "Cannot roll",
                                        color: c.result.warning,
                                    },
                                );
                            }
                            ui.add_space(design::S);
                            let name =
                                format!("Roll {} for {}", plain_roll(times, &key), holder.name);
                            if Action::secondary("Roll")
                                .icon(icons::DICE)
                                .name(&name)
                                .density(Density::Read)
                                .show(ui)
                                .clicked()
                            {
                                request = Some(at);
                            }
                        });
                        if let Some(problem) = problem {
                            ui.add(
                                egui::Label::new(Type::Small.rich(ui, problem).color(c.muted))
                                    .wrap(),
                            );
                        }
                    }
                }
            }
        });
        request
    }

    /// The card's loot. One quiet line while the creature stands, and all of it at 0 HP.
    pub(super) fn card_loot(&mut self, ui: &mut egui::Ui, index: usize) {
        let p = &self.state.participants[index];
        let lines = p.loot_lines();
        if lines.is_empty() {
            return;
        }
        let holder = Holder {
            key: p.key.clone(),
            name: p.name.clone(),
            lines,
            open: p.hp == Some(0),
        };
        if !holder.open {
            let summary: Vec<String> = holder
                .lines
                .iter()
                .map(|line| crate::loot_editor::plain(line))
                .collect();
            ui.add(
                egui::Label::new(
                    Type::Small
                        .rich(ui, format!("{} Loot · {}", icons::LOOT, summary.join(", ")))
                        .color(colors().muted),
                )
                .wrap(),
            );
            ui.add_space(design::S);
            return;
        }
        design::group_label(ui, "Loot");
        if let Some(at) = self.loot_list(ui, &holder) {
            self.roll_loot(&[(holder, at)]);
        }
        ui.add_space(design::S);
    }

    /// The ended view's loot: the encounter's and each downed creature's, with what is left to
    /// roll, Roll all and Copy loot. Loot on creatures still standing waits behind one line.
    pub(super) fn loot_section(&mut self, ui: &mut egui::Ui) {
        let holders = self.holders();
        if holders.is_empty() {
            return;
        }
        let c = colors();
        let (open, standing): (Vec<Holder>, Vec<Holder>) =
            holders.into_iter().partition(|holder| holder.open);
        let left = self.unrolled(&open).len();
        let mut picks: Vec<(Holder, usize)> = Vec::new();
        // The copy owns its result: Undo and any new step end it, whatever the Log's length.
        let copied = self.loot_copied == Some(self.revision);
        ui.horizontal(|ui| {
            design::group_label(ui, "Loot");
            if left > 0 {
                ui.label(
                    Type::Small
                        .rich(ui, format!("{left} to roll"))
                        .color(c.muted),
                );
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if Action::quiet("Copy loot")
                    .icon(icons::COPY)
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                {
                    // Standing creatures too, once the DM has rolled their loot.
                    let copied: Vec<Holder> = open
                        .iter()
                        .chain(standing.iter().filter(|holder| {
                            (0..holder.lines.len())
                                .any(|at| self.state.loot.contains_key(&place(&holder.key, at)))
                        }))
                        .cloned()
                        .collect();
                    ui.ctx().copy_text(self.loot_markdown(&copied));
                    self.loot_copied = Some(self.revision);
                }
                if left > 1
                    && Action::secondary(&format!("Roll all {left}"))
                        .icon(icons::DICE)
                        .density(Density::Read)
                        .show(ui)
                        .clicked()
                {
                    picks = self
                        .unrolled(&open)
                        .into_iter()
                        .map(|(holder, at)| (holder.clone(), at))
                        .collect();
                }
                // A result never moves what the DM works on: it goes after Roll all, in the
                // room on its left, so neither button moves when it shows.
                if copied {
                    ui.label(Type::Small.rich(ui, "Loot copied.").color(c.muted));
                }
            });
        });
        for holder in &open {
            ui.label(Type::Strong.rich(ui, &holder.name).color(c.text));
            if let Some(at) = self.loot_list(ui, holder) {
                picks = vec![(holder.clone(), at)];
            }
            ui.add_space(design::S);
        }
        if !standing.is_empty() {
            let title = if standing.len() == 1 {
                "1 more creature carries loot".to_owned()
            } else {
                format!("{} more creatures carry loot", standing.len())
            };
            let title = Type::Body.rich(ui, title).color(c.muted);
            design::section(ui, title, |ui| {
                for holder in &standing {
                    ui.label(Type::Strong.rich(ui, &holder.name).color(c.text));
                    if let Some(at) = self.loot_list(ui, holder) {
                        picks = vec![(holder.clone(), at)];
                    }
                    ui.add_space(design::S);
                }
            });
        }
        ui.add_space(design::M);
        if !picks.is_empty() {
            self.roll_loot(&picks);
        }
    }

    /// These holders' loot as a Markdown list, with each roll's result, for a session note.
    pub(super) fn loot_markdown(&self, holders: &[Holder]) -> String {
        let mut out = String::new();
        for holder in holders {
            out.push_str(&format!("**{}**\n", holder.name));
            for (line, _) in note_lines(&self.state, holder) {
                out.push_str(&format!("- {line}\n"));
            }
            out.push('\n');
        }
        out.trim_end().to_owned()
    }
}
