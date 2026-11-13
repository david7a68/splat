//! What the world's notes still need: the questions the DM left open, and the places where a
//! fact has no home or has two. Every check only reads. See `docs/design/world-data.md`.
#![warn(deprecated)] // Its parent allows old message functions; this file does not.
use crate::lookup::{ContentIndex, Item};
use std::collections::{BTreeMap, BTreeSet};

/// A sentence this long, said on two pages, is a copy and not a chance.
const SENTENCE_WORDS: usize = 8;

/// What one page still needs, and the heading to open it at.
#[derive(Debug, PartialEq, Eq)]
pub struct Found {
    pub page: usize,
    pub heading: Option<String>,
    pub words: String,
    /// An open question's box: its line in the page's body and that line as read.
    pub open_box: Option<(usize, String)>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Checks {
    /// Each unticked task line, in page order.
    pub questions: Vec<Found>,
    /// A link with no page: its name, how many lines want it, and the pages that do.
    pub wanted: Vec<(String, usize, Vec<usize>)>,
    /// Entries that do not start with a paragraph.
    pub no_summary: Vec<usize>,
    /// Entries that show on no other page: no line names them, they name no entry, nothing is
    /// in them or part of them, and they are in nothing and part of nothing.
    pub alone: Vec<usize>,
    /// One name or alias on two entries or more.
    pub shared_names: Vec<(String, Vec<usize>)>,
    /// One sentence on two pages or more.
    pub repeated: Vec<(String, Vec<usize>)>,
    /// A date in words on a page that is not an event.
    pub loose_dates: Vec<Found>,
}

impl Checks {
    pub fn problems(&self) -> usize {
        self.wanted.len()
            + self.no_summary.len()
            + self.alone.len()
            + self.shared_names.len()
            + self.repeated.len()
            + self.loose_dates.len()
    }
}

fn entry(index: &ContentIndex, item: &Item) -> bool {
    // Two files with one identity have their own repair.
    item.entry.kind == "world"
        && item.is_private()
        && !index.duplicate_world.contains_key(&item.entry.path)
}

pub fn run(index: &ContentIndex) -> Checks {
    let mut checks = Checks::default();
    let mut wanted: BTreeMap<String, (String, usize, BTreeSet<usize>)> = BTreeMap::new();
    let mut names: BTreeMap<String, BTreeSet<usize>> = BTreeMap::new();
    let mut sentences: BTreeMap<String, BTreeSet<usize>> = BTreeMap::new();
    let mut pointed_at = BTreeSet::new();
    for &page in index.world_pages() {
        let item = &index.items[page];
        // A copy in a package, or a second file with one identity, is not the DM's page.
        if item.entry.kind == "world" && !entry(index, item) {
            continue;
        }
        let dated = super::when(&item.entry).is_some();
        for line in &item.lines {
            let found = |words: String| Found {
                page,
                heading: line.heading.clone(),
                words,
                open_box: None,
            };
            if line.task == Some(false) {
                checks.questions.push(Found {
                    open_box: line.open_box.clone(),
                    ..found(line.words())
                });
            }
            for target in line.link_targets() {
                // A file that is not a note, such as a PDF, is not a page to write.
                let name = target.split('#').next().unwrap_or(target).trim();
                let name = name.strip_suffix(".md").unwrap_or(name);
                let other_file = name.rsplit_once('.').is_some_and(|(_, extension)| {
                    (2..=4).contains(&extension.len())
                        && extension.chars().all(|c| c.is_ascii_alphanumeric())
                });
                if !other_file && index.link_is_missing(target, item.entry.id) {
                    let want = wanted
                        .entry(name.to_lowercase())
                        .or_insert_with(|| (name.to_owned(), 0, BTreeSet::new()));
                    want.1 += 1;
                    want.2.insert(page);
                }
            }
            if line.kind == super::lines::Kind::Heading || line.in_log() {
                continue;
            }
            if !dated && line.has_date() {
                checks.loose_dates.push(found(line.words()));
            }
            if !line.bare && line.kind != super::lines::Kind::Row {
                for sentence in line.sentences(SENTENCE_WORDS) {
                    sentences.entry(sentence).or_default().insert(page);
                }
            }
        }
        if !entry(index, item) {
            continue;
        }
        if item.summary().is_empty() {
            checks.no_summary.push(page);
        }
        let mut own = vec![item.entry.name.to_lowercase()];
        own.extend(super::aliases(&item.entry).iter().map(|a| a.to_lowercase()));
        for name in own {
            names.entry(name).or_default().insert(page);
        }
        // A relation shows from both ends, so it leads to this entry and to its target.
        let targets: Vec<usize> = index
            .place_of(item.entry.id)
            .into_iter()
            .chain(
                index
                    .groups_of(&item.entry)
                    .iter()
                    .filter_map(|(_, group)| *group),
            )
            .collect();
        if !targets.is_empty() {
            pointed_at.insert(page);
        }
        pointed_at.extend(targets);
        // A page that names this one shows in this one's lists, so each leads to the other.
        pointed_at.extend(index.mentions_of(item.entry.id));
    }
    checks.alone = index
        .world_pages()
        .iter()
        .copied()
        .filter(|page| {
            let item = &index.items[*page];
            entry(index, item)
                && !pointed_at.contains(page)
                && index.lines_about(item.entry.id).is_empty()
        })
        .collect();
    checks.wanted = wanted
        .into_values()
        .map(|(name, lines, pages)| (name, lines, pages.into_iter().collect()))
        .collect();
    // The page that most lines want is the next one to write.
    checks
        .wanted
        .sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    let several = |found: BTreeMap<String, BTreeSet<usize>>| -> Vec<(String, Vec<usize>)> {
        found
            .into_iter()
            .filter(|(_, pages)| pages.len() > 1)
            .map(|(words, pages)| (words, pages.into_iter().collect()))
            .collect()
    };
    checks.shared_names = several(names);
    checks.repeated = several(sentences);
    checks
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence::Campaign;
    use uuid::Uuid;

    fn entry_note(name: &str, fields: &str, body: &str) -> String {
        format!(
            "---\nsplat_schema: 2\nsplat_id: {}\nsplat_kind: world\nsplat_name: {name}\n{fields}\n---\n\n# {name}\n\n{body}",
            Uuid::new_v4()
        )
    }

    #[test]
    fn the_checks_find_open_questions_and_facts_with_no_home_or_two() {
        let dir = tempfile::tempdir().unwrap();
        let said = "Verori settled Kabrigar for its coal and iron.";
        let notes = [
            (
                "Places/Kabrigar.md",
                entry_note(
                    "Kabrigar",
                    "splat_type: location",
                    &format!(
                        "The capital of [[Kaun]].\n\n- {said}\n- About 90 years ago, Kaun won its revolt.\n- [ ] Who rules it now?\n- [x] Is it cold?\n\n## Log\n\n- Session 1: Burned 2 days ago.\n"
                    ),
                ),
            ),
            (
                "History/The Revolt.md",
                entry_note(
                    "The Revolt",
                    "splat_type: event\nsplat_when: 90 years ago",
                    "Kaun won its revolt about 90 years ago, at [[Kabrigar]] and [[Kaun]].\n",
                ),
            ),
            (
                "NPCs/Rook.md",
                entry_note(
                    "Rook",
                    "splat_type: npc\naliases: [Ash]",
                    "- **Look:** A hat.\n",
                ),
            ),
            (
                "NPCs/Ash.md",
                entry_note(
                    "Ash",
                    "splat_type: npc\nsplat_in: '[[Kabrigar]]'",
                    "A goblin.\n",
                ),
            ),
            (
                "Setting.md",
                format!("{said} See [[Kabrigar]].\n\n- [ ] One treaty or several?\n"),
            ),
        ];
        for (path, text) in &notes {
            let path = dir.path().join(path);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, text).unwrap();
        }
        let campaign = Campaign {
            id: Uuid::new_v4(),
            name: "Sample".into(),
            path: dir.path().into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let checks = run(&index);
        let name = |page: &usize| index.items[*page].entry.name.as_str();
        let names = |pages: &[usize]| pages.iter().map(name).collect::<Vec<_>>();

        let questions: Vec<_> = checks
            .questions
            .iter()
            .map(|found| (name(&found.page), found.words.as_str()))
            .collect();
        assert_eq!(
            questions,
            [
                ("Kabrigar", "Who rules it now?"),
                ("Setting", "One treaty or several?")
            ]
        );
        // X45 of app round 2: a question knows the line of its note that holds its box, and a
        // tick sets that box only while the line is as read.
        let tick_file = super::super::page::tick_file;
        let (at, line) = checks.questions[0].open_box.clone().unwrap();
        let file = std::fs::read_to_string(dir.path().join("Places/Kabrigar.md")).unwrap();
        let ticked = tick_file(&file, at, &line, true).unwrap();
        assert_eq!(
            ticked,
            file.replacen("- [ ] Who rules it now?", "- [x] Who rules it now?", 1)
        );
        // An unrelated edit of the note after the tick stays when Undo opens the box again.
        let edited = format!("{ticked}- More words.\n");
        assert_eq!(
            tick_file(&edited, at, &line, false).unwrap(),
            format!("{file}- More words.\n")
        );
        assert_eq!(tick_file(&file, at, &line, false), None);
        let reworded = file.replace("Who rules it now?", "Who rules it?");
        assert_eq!(tick_file(&reworded, at, &line, true), None);
        // The question as read is the whole question: a changed continuation line is not it
        // (review round 1 of PR 287).
        let body = "- [ ] Who pays Rook\n  to open the gate?\n";
        let (at, question) = super::super::lines::lines(body, "Rook")
            .into_iter()
            .find_map(|line| line.open_box)
            .unwrap();
        assert_eq!(question, "- [ ] Who pays Rook\n  to open the gate?");
        assert_eq!(
            tick_file(body, at, &question, true).as_deref(),
            Some("- [x] Who pays Rook\n  to open the gate?\n")
        );
        let changed = body.replace("open the gate", "kill the king");
        assert_eq!(tick_file(&changed, at, &question, true), None);
        // Kaun has no page, and two lines want it.
        assert_eq!(checks.wanted.len(), 1);
        assert_eq!(
            (checks.wanted[0].0.as_str(), checks.wanted[0].1),
            ("Kaun", 2)
        );
        assert_eq!(names(&checks.wanted[0].2), ["Kabrigar", "The Revolt"]);
        assert_eq!(names(&checks.no_summary), ["Rook"]);
        // Ash and Rook share a name, so each names the other. The revolt names Kabrigar, so
        // it shows in that page's History.
        assert_eq!(names(&checks.alone), [] as [&str; 0]);
        assert_eq!(checks.shared_names.len(), 1);
        assert_eq!(checks.shared_names[0].0, "ash");
        assert_eq!(names(&checks.shared_names[0].1), ["Ash", "Rook"]);
        assert_eq!(checks.repeated.len(), 1);
        assert_eq!(names(&checks.repeated[0].1), ["Kabrigar", "Setting"]);
        // A date on an event's page has its home. A Log line records play, not history.
        let dates: Vec<_> = checks.loose_dates.iter().map(|f| name(&f.page)).collect();
        assert_eq!(dates, ["Kabrigar"]);
        assert_eq!(checks.problems(), 5);
    }
}
