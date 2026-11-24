//! The Party page and its view state.
use crate::design::{self, icons};
use splat_character::character::EntitySnapshot;
use splat_party::party::{Party as PartyState, SyncStopped, record_age, spell_dc, text, val};

#[derive(Default)]
pub struct PartyPage {
    pub wants_link_page: bool,
    pub wants_settings: bool,
    pub run_open: bool,
}

impl PartyPage {
    /// The page's footer lines (`page_footer`), for a UI that draws them itself.
    pub fn footer_lines(
        &mut self,
        party: &mut PartyState,
        connected: bool,
        linked: bool,
    ) -> Vec<design::outcome::plain::PlainLine<PartyPress>> {
        let frame = design::outcome::PaneFrame::new();
        Party { view: self, party }
            .page_footer(&frame, connected, linked)
            .into_plain()
    }

    /// Do a press of the page's footer; a file to show comes back as its URL.
    pub fn footer_press(
        &mut self,
        party: &mut PartyState,
        press: design::outcome::Press<PartyPress>,
    ) -> Option<String> {
        Party { view: self, party }.press(Some(press))
    }
}

/// The Party page's header: its state words and which actions it offers.
pub struct PartyHeader {
    /// `Not connected · Fetched 3 min ago`, with its icon first.
    pub context: String,
    pub icon: &'static str,
    /// Why Refresh party waits.
    pub refresh_reason: Option<String>,
    /// `Connect to refresh` shows.
    pub connect: bool,
    /// `Refresh party` shows.
    pub refresh: bool,
}

impl PartyHeader {
    pub fn of(party: &PartyState, connected: bool, linked: bool) -> Self {
        let reader = &party.reader;
        let ready = party.combat.ready();
        let fetched = reader
            .selected
            .as_ref()
            .and_then(|id| reader.sheets.get(id))
            .filter(|_| !ready)
            .map(|record| format!(" · Fetched {}", record_age(record)))
            .unwrap_or_default();
        let refresh_reason = if !connected {
            Some(format!("Not connected{fetched} · Connect to refresh"))
        } else if !linked {
            Some("Link this campaign to D&D Beyond first.".to_owned())
        } else if reader.loading() {
            Some("Already refreshing.".to_owned())
        } else {
            None
        };
        let checked =
            reader.selected.as_ref().is_some_and(|id| {
                reader.online_ids.contains(id) && !reader.errors.contains_key(id)
            }) && reader.error.is_none();
        let (icon, word) = if !connected {
            (icons::OFFLINE, "Not connected")
        } else if ready {
            (icons::ONLINE, "Connected · Sync reads this sheet")
        } else if !checked {
            (icons::ONLINE, "Connected")
        } else if reader.stream_open {
            (icons::ONLINE, "Live updates")
        } else {
            (icons::ONLINE, "Connected · Checking for updates")
        };
        // Another computer saves the sheets from automatic reads. Say so, because this
        // computer's notes change only through that computer and Obsidian Sync.
        let writer = reader
            .writer_elsewhere
            .as_ref()
            .map(|name| format!(" · Sheets save on {name}"))
            .unwrap_or_default();
        let refresh = linked && !reader.members.is_empty();
        Self {
            context: format!("{icon} {word}{fetched}{writer}"),
            icon,
            refresh_reason,
            connect: refresh && !connected && reader.conflicts.is_empty(),
            refresh,
        }
    }
}

/// `HP here is from tonight’s fights, as of …`, while tonight's fights count.
pub fn tonight_words(party: &PartyState) -> Option<String> {
    let carry = party.tonight.as_ref()?;
    let mut parts = Vec::new();
    if carry.full || !carry.values.is_empty() {
        parts.push(format!(
            "HP here is from tonight’s fights, as of {}.",
            carry.from
        ));
    }
    if !carry.missing.is_empty() {
        parts.push(format!(
            "Could not read the run of {}.",
            carry.missing.join(", ")
        ));
    }
    Some(parts.join(" "))
}

struct Party<'a> {
    view: &'a mut PartyPage,
    party: &'a mut PartyState,
}

impl Party<'_> {
    /// The page's one message footer (round 27 FA4, FH1), below the list and the sheet. Its
    /// order: the errors (the party's read, a character's read beside its kept edits), then the
    /// moved-aside sheets' results, the local-edit warnings with their actions, the storage
    /// warnings and the stopped-sync notice. Each keeps its owner and end event: a moved-aside
    /// sheet's result ends at its own close icon, a local-edit warning when its sheet is
    /// replaced, the stopped-sync notice at its close icon. The messages own their words, so the
    /// body can change the page; `page_pressed` does the press after the page is drawn.
    fn page_footer<'f>(
        &self,
        frame: &'f design::outcome::PaneFrame,
        connected: bool,
        linked: bool,
    ) -> design::outcome::Messages<'f, PartyPress> {
        use design::outcome::{Choice, FaultMessage, Lost, MessageChoice, Said};
        let reader = &self.party.reader;
        let mut messages = design::outcome::Messages::new();
        let show_file = |path: &std::path::Path| {
            Choice::new(
                MessageChoice::Link {
                    word: "Show file",
                    hover: None,
                },
                PartyPress::ShowFile(path.to_path_buf()),
            )
            .hover(path.display().to_string())
        };
        if linked {
            if let Some(error) = &reader.error {
                messages.outcome(FaultMessage::new(&PartyFault::Read(error)));
            }
            for id in reader.conflicts.keys() {
                if reader.selected.as_ref() != Some(id)
                    && let Some(error) = reader.errors.get(id)
                {
                    messages.outcome(FaultMessage::new(&PartyFault::Character {
                        name: reader.member_name(id),
                        error,
                    }));
                }
            }
            // Round 25 FD3: one dismiss form on the page, the close icon at the message end.
            for shown in reader.archived.newest_first(&()) {
                let line = shown.outcome(frame, |archived| {
                    let name = reader.member_name(&archived.owner);
                    Said::new(crate::sentence!(
                        "{}’s sheet now matches D&D Beyond. Your edited version is kept.",
                        name
                    ))
                    .choice(show_file(&archived.path))
                    .choice(Choice::new(
                        MessageChoice::Dismiss(Lost::Nothing),
                        PartyPress::DismissArchived,
                    ))
                    .into()
                });
                messages.kept(line);
            }
            let several = reader.conflicts.len() > 1;
            let waiting = reader.force_waiting();
            // `at` is the warning's place among the conflicts: its id is looked up at the press.
            for (at, (id, path)) in reader.conflicts.iter().enumerate() {
                let name = reader.member_name(id);
                let fault = FaultMessage::new(&PartyFault::LocalEdits { name, connected });
                let fault = if connected {
                    // Safe first: Show file, then Replace, which changes the active sheet (the
                    // part puts `L` before it). Its lock keeps its reason's room while the
                    // forced read waits and after, so nothing in the footer moves.
                    fault.choice(show_file(path)).choice(
                        Choice::new(
                            MessageChoice::Lose("Replace with D&D Beyond sheet"),
                            PartyPress::Replace(several.then_some(at)),
                        )
                        .locked_when(waiting, crate::sentence!("Waiting for D&D Beyond.")),
                    )
                } else {
                    // The warning asks the DM to connect, so the way to connect is first.
                    fault
                        .choice(
                            Choice::new(MessageChoice::Choice(CONNECT), PartyPress::Connect).hover(
                                "Opens Settings to connect your D&D Beyond account".to_owned(),
                            ),
                        )
                        .choice(show_file(path))
                };
                messages.outcome(fault);
            }
            let warnings: std::collections::BTreeSet<_> = reader.warnings.values().collect();
            for warning in warnings {
                messages.outcome(FaultMessage::new(&PartyFault::Storage(warning)));
            }
        }
        if self.party.sync_stopped {
            messages.outcome(SyncStopped::notice().choice(Choice::new(
                MessageChoice::Dismiss(Lost::Nothing),
                PartyPress::DismissStopped,
            )));
        }
        messages
    }

    /// Do a footer press; a file to show comes back as its URL.
    fn press(&mut self, pressed: Option<design::outcome::Press<PartyPress>>) -> Option<String> {
        let design::outcome::Press { id, ticket } = pressed?;
        match id {
            PartyPress::DismissStopped => self.party.sync_stopped = false,
            // Its own result only: a newer one keeps its line.
            PartyPress::DismissArchived => {
                if let Some(ticket) = ticket {
                    let _ = self.party.reader.archived.use_press_by(ticket);
                }
            }
            PartyPress::Connect => self.view.wants_settings = true,
            PartyPress::Replace(None) => self.party.reader.force_read(),
            PartyPress::Replace(Some(at)) => {
                if let Some(id) = self.party.reader.conflicts.keys().nth(at).cloned() {
                    self.party.reader.force_read_character(&id);
                }
            }
            PartyPress::ShowFile(path) => {
                if let Ok(url) = url::Url::from_file_path(&path) {
                    return Some(url.to_string());
                }
            }
        }
        None
    }
}

/// What the DM pressed in the Party page's footer, done after the page is drawn.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PartyPress {
    DismissStopped,
    /// The close icon of a moved-aside sheet's result (the press's ticket names which).
    DismissArchived,
    Connect,
    /// The local-edit warning whose Replace the DM pressed: `None` when there is one, which
    /// replaces every one. The warning's place among the conflicts, looked up at the press.
    Replace(Option<usize>),
    ShowFile(std::path::PathBuf),
}

/// What the Party page's footer says is wrong.
enum PartyFault<'a> {
    /// The party's read failed (D&D Beyond's or the system's words).
    Read(&'a str),
    /// One character's read failed, beside its kept edits.
    Character { name: &'a str, error: &'a str },
    /// The character's sheet note has the DM's own edits, so sync stopped writing it.
    LocalEdits { name: &'a str, connected: bool },
    /// The local copy could not be stored (the system's words).
    Storage(&'a str),
}

impl design::outcome::Fault for PartyFault<'_> {
    fn severity(&self) -> design::outcome::Severity {
        match self {
            Self::Read(_) | Self::Character { .. } => design::outcome::Severity::CannotRead,
            Self::LocalEdits { .. } | Self::Storage(_) => design::outcome::Severity::NeedsFixing,
        }
    }

    fn place(&self) -> Option<design::outcome::FaultPlace<'_>> {
        match self {
            Self::Character { name, .. } => Some(design::outcome::FaultPlace {
                name: (*name).into(),
                line: None,
            }),
            _ => None,
        }
    }

    #[allow(
        clippy::disallowed_methods,
        reason = "a failed read or store says D&D Beyond’s or the system’s own words"
    )]
    fn cause(&self) -> design::outcome::Sentence {
        use design::outcome::Sentence;
        match self {
            Self::Read(error) | Self::Character { error, .. } => {
                Sentence::outside(*error).unwrap_or(crate::sentence!("The read did not finish."))
            }
            Self::LocalEdits { name, connected } => {
                let first = crate::sentence!(
                    "{}’s sheet file has your own edits, so Splat stopped updating it.",
                    name
                );
                if *connected {
                    first.then(crate::sentence!("Replacing keeps your version as a copy."))
                } else {
                    first.then(crate::sentence!(
                        "Connect, then replace it. Replacing keeps your version as a copy."
                    ))
                }
            }
            Self::Storage(warning) => Sentence::outside(format!("Local storage: {warning}"))
                .unwrap_or(crate::sentence!("Local storage did not save.")),
        }
    }
}

pub fn most_recent_sheet<'a>(party: &'a PartyState, id: &str) -> Option<&'a EntitySnapshot> {
    match (party.combat.record(), party.reader.sheets.get(id)) {
        (Some(live), Some(saved)) if saved.source.fetched_at > live.source.fetched_at => {
            Some(saved)
        }
        (live, saved) => live.or(saved),
    }
}

/// The way to connect, wherever the party asks the DM to.
const CONNECT: &str = "Connect D&D Beyond";
/// `Fighter · Level 6`: the one form of a member's classes and level, in the sheet's kind line
/// and in the member's row (round 28 FB12).
pub fn classes_and_level(classes: &str, level: Option<impl std::fmt::Display>) -> String {
    [
        classes.to_owned(),
        level.map(|n| format!("Level {n}")).unwrap_or_default(),
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join(" · ")
}

pub fn class_line(record: &EntitySnapshot) -> String {
    format!(
        "{} · {}",
        record.character.species,
        classes_and_level(
            &record
                .character
                .classes
                .iter()
                .map(|c| text(&c["name"]))
                .collect::<Vec<_>>()
                .join(" / "),
            Some(record.character.level),
        )
    )
}

pub fn compare_values(record: Option<&EntitySnapshot>) -> [String; 3] {
    let Some(record) = record else {
        return ["—"; 3].map(str::to_owned);
    };
    let dc = spell_dc(record).map_or_else(|| "—".to_owned(), |dc| dc.to_string());
    [
        val(record, "passive_perception"),
        val(record, "passive_insight"),
        dc,
    ]
}

/// The technical details' words, free of any toolkit: egui's `source_and_parity` and the GPUI
/// sheet (`src/gui/pages/party/sheet.rs`) both show them.
pub mod technical {
    use splat_character::character;
    use splat_content::content;

    /// A line of the character check: a group's title, or its words.
    #[derive(Clone, Debug, PartialEq)]
    pub enum CheckLine {
        Title(&'static str),
        Words(String),
    }

    /// The help under the optional character rules.
    pub const OPTIONS_HELP: &str = "Selections follow the D&D Beyond sheet. Active choices can need further details. Simulation support is separate.";

    /// `Character check`: what matches the SRD rules, then each kind of finding under its title.
    pub fn check(review: &Result<character::check::Review, String>) -> Vec<CheckLine> {
        use character::check::FindingKind;
        let review = match review {
            Ok(review) => review,
            Err(message) => {
                return vec![CheckLine::Words(format!(
                    "The check could not run: {message}"
                ))];
            }
        };
        let mut lines = vec![CheckLine::Words(if review.matched.is_empty() {
            "Nothing could be compared with the SRD rules.".to_owned()
        } else {
            format!("Matches the SRD rules: {}.", review.matched.join(", "))
        })];
        for (kind, title) in [
            (FindingKind::Differs, "Differs from the SRD rules"),
            (FindingKind::MissingChoice, "Missing choices"),
            (FindingKind::InvalidChoice, "Choices the rules do not allow"),
            (FindingKind::NotChecked, "Not compared"),
        ] {
            if review.count(kind) > 0 {
                lines.push(CheckLine::Title(title));
                lines.extend(
                    review
                        .findings
                        .iter()
                        .filter(|finding| finding.kind == kind)
                        .map(|finding| CheckLine::Words(finding.message.clone())),
                );
            }
        }
        lines
    }

    /// `Optional character rules`: each rule's state on the sheet, or `None` when the sheet
    /// says nothing of them.
    pub fn options(entry: &content::Entry) -> Option<Vec<String>> {
        use content::character::{CampaignOptionState, Sheet};
        let value = entry.fields.get("splat_character")?;
        let sheet = serde_yaml_ng::from_value::<Sheet>(value.clone()).ok()?;
        let mut lines = Vec::new();
        for (key, name) in [
            ("dark_bargain", "Dark Bargain"),
            ("character_threads", "Character Threads"),
            ("runestones", "Runestones"),
        ] {
            match sheet.campaign_options.get(key) {
                Some(CampaignOptionState::Inactive) => {
                    lines.push(format!("{name}: inactive (not granted on the sheet)."));
                }
                Some(CampaignOptionState::Unselected) => {
                    lines.push(format!("{name}: no effect selected. No effect imported."));
                }
                Some(CampaignOptionState::Unknown) => {
                    lines.push(format!("{name}: source selection data is unavailable."));
                }
                Some(CampaignOptionState::Active { selections }) => {
                    for selected in selections {
                        let status = if selected.rule.is_some() {
                            "converted"
                        } else {
                            "needs rule conversion"
                        };
                        lines.push(format!("{name}: {} ({status}).", selected.name));
                    }
                }
                None => (),
            }
        }
        Some(lines)
    }

    /// `Campaign content: 12 inventory rows, 4 action definitions.`
    pub fn content_counts(entry: &content::Entry) -> String {
        format!(
            "Campaign content: {} inventory rows, {} action definitions.",
            entry.inventory.len(),
            entry
                .fields
                .get("splat_actions")
                .and_then(serde_yaml_ng::Value::as_mapping)
                .map_or(0, |v| v.len())
        )
    }

    /// What the import left to do: its summary, and each list's reasons under its title.
    pub struct Import {
        pub summary: String,
        pub lists: Vec<(&'static str, Vec<String>)>,
    }

    /// The import's rules not read yet and its required inputs, when the entry has an import.
    pub fn import(entry: &content::Entry) -> Option<Import> {
        let import = entry.fields.get("splat_import")?;
        let list = |key: &str| {
            import
                .get(key)
                .and_then(serde_yaml_ng::Value::as_sequence)
                .cloned()
                .unwrap_or_default()
        };
        let summary = format!(
            "{} not read yet · {} not supported in Splat",
            crate::design::count(list("gaps").len(), "rule", "rules"),
            list("engine_gaps")
                .iter()
                .filter_map(serde_yaml_ng::Value::as_str)
                .filter(|gap| !content::adapters::is_implemented(gap))
                .count()
        );
        let lists = [
            ("Rules still to read", "gaps"),
            ("Required character or encounter inputs", "required_inputs"),
        ]
        .into_iter()
        .map(|(title, key)| {
            (
                title,
                list(key)
                    .iter()
                    .filter_map(|item| {
                        item.get("reason")
                            .and_then(serde_yaml_ng::Value::as_str)
                            .map(str::to_owned)
                    })
                    .collect(),
            )
        })
        .collect();
        Some(Import { summary, lists })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn iter_3_b_storage_actions_stay_with_their_message() {
        use crate::design::outcome::plain::{ChoiceForm, PlainLine};
        // Two characters with local edits, A and B, in the page's footer as the GPUI page
        // builds it: B's Replace forces a read of B alone, and A stays as it was (the press
        // keeps its notice's place among the conflicts, not a copy of its id).
        let mut party = PartyState::default();
        for (id, path) in [("a", "/campaign/a.md"), ("b", "/campaign/b.md")] {
            party
                .reader
                .conflicts
                .insert(id.into(), std::path::Path::new(path).into());
        }
        let mut page = PartyPage::default();
        const REPLACE: &str = "Replace with D&D Beyond sheet";
        fn replaces(lines: &[PlainLine<PartyPress>]) -> Vec<&PlainLine<PartyPress>> {
            lines
                .iter()
                .filter(|line| line.choices.iter().any(|choice| choice.words == REPLACE))
                .collect()
        }
        let lines = page.footer_lines(&mut party, true, true);
        let warnings = replaces(&lines);
        assert_eq!(warnings.len(), 2, "each local-edit notice has its Replace");
        // Review of PR 330: safe first. Each warning has Show file, then Replace (the
        // destructive choice, which the kit puts `L` after it).
        for line in warnings.iter().copied() {
            let words: Vec<_> = line.choices.iter().map(|c| c.words.as_str()).collect();
            assert_eq!(words, ["Show file", REPLACE]);
            assert_eq!(line.choices[1].form, ChoiceForm::Lose);
        }
        let press = warnings[1].press_choice(1).expect("B's Replace");
        page.footer_press(&mut party, press);
        assert_eq!(
            party.reader.forced(),
            ["b"],
            "Replace on the second notice forces a read of B only; A is unchanged"
        );
        // While the forced read waits, Replace is locked with its reason: it takes no press.
        assert!(party.reader.force_waiting());
        let lines = page.footer_lines(&mut party, true, true);
        let warnings = replaces(&lines);
        assert!(
            warnings
                .iter()
                .all(|line| line.choices[1].locked.is_some() && line.press_choice(1).is_none()),
            "a locked Replace takes no press"
        );
        party.reader.end_forced_read();
        let lines = page.footer_lines(&mut party, true, true);
        assert!(
            replaces(&lines)
                .iter()
                .all(|line| line.choices[1].locked.is_none())
        );
        // Wave 2 d (C): two moved-aside sheets, A then B, each a kept result with its own close
        // icon. They are the page's, not the selected sheet's: they stay when the DM selects
        // another character, and each acts on its own file. With B selected, A's Show file
        // opens A's file; B's close icon (the newest, first in the footer) ends B's result only.
        party.reader.conflicts.clear();
        party.reader.select("a".into());
        for (owner, path) in [("a", "/campaign/a-old.md"), ("b", "/campaign/b-old.md")] {
            party.reader.archived.record(crate::party_sync::Archived {
                path: std::path::Path::new(path).into(),
                owner: owner.into(),
            });
        }
        party.reader.select("b".into());
        let lines = page.footer_lines(&mut party, true, true);
        let kept: Vec<_> = lines
            .iter()
            .filter(|line| line.choices.iter().any(|c| c.words == "Show file"))
            .collect();
        assert_eq!(kept.len(), 2, "both results stay when B is selected");
        let url = page.footer_press(&mut party, kept[1].press_choice(0).expect("A's Show file"));
        assert!(
            url.as_deref()
                .is_some_and(|url| url.ends_with("/campaign/a-old.md")),
            "A’s Show file opens A’s file only: {url:?}"
        );
        let close = kept[0]
            .choices
            .iter()
            .position(|choice| choice.form == ChoiceForm::Dismiss)
            .expect("each moved-aside sheet has its close icon");
        page.footer_press(&mut party, kept[0].press_choice(close).unwrap());
        let left: Vec<_> = party
            .reader
            .archived
            .newest_first(&())
            .map(|shown| shown.value().owner.clone())
            .collect();
        assert_eq!(left, ["a"], "B’s close icon ends B’s result; A’s stays");
    }
}
