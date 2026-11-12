//! F1: a fight's difficulty, where the DM builds it and plans it. The word ends a line that is
//! there today, and shows nothing when an input is not known. Each surface is shown with Lookup
//! closed and open at the minimum window, and at the default one.
//!
//! The campaign's party is five characters at levels 5 and 6: Low 2,800, Moderate 4,500 and
//! High 6,400, from the XP budget table of the campaign's rules.
use super::*;
use crate::test_support::{CachedWork, Counts};
use serde_yaml_ng::{Mapping, Value};

/// Five characters, the Cinder Tyrant (CR 10, XP 5,900) and two Ember Cultists (XP 200 each).
const HIGH: &str = "Mill Road";
// iter-26-B FL2: the members in the group's form, each with its count, joined by commas.
const HIGH_SUMMARY: &str = "5 party · 1 × Cinder Tyrant, 2 × Ember Cultist";
const HIGH_NUMBERS: &str = "XP 6,300 of 6,400";
/// Two Cinder Tyrants and no character: the campaign's party stands in.
const ABOVE: &str = "Court of Cinders";
/// Five characters and a creature with no XP and no CR.
const UNKNOWN: &str = "Nameless Visitor";
/// The fixture's: five characters and two Ember Cultists, XP 400.
const LOW: &str = "Shrine of Cinders";
const BUDGET: &str = "Budget for 5 at levels 5 to 6: Low 2,800 · Moderate 4,500 · High 6,400";
const STAND_IN: &str = "Budget for the party of 5 at levels 5 to 6:";

/// A campaign creature with a CR, or with none.
fn write_creature(d: &Driver<'_>, name: &str, cr: Option<&str>) -> uuid::Uuid {
    let id = crate::hermetic::new_uuid();
    let relative = format!("Creatures/{name}.md");
    let challenge = cr.map_or_else(String::new, |cr| format!(", cr_text: '{cr}'"));
    let text = splat::content::formats::fixture(
        &relative,
        &format!(
            "splat_id: {id}\nsplat_kind: creature\nsplat_name: {name}\nsplat_creature: {{size: large, type: fiend{challenge}}}\nsplat_stats:\n  max_hp: {{value: 120}}\n  armor_class: {{value: 16}}\n  initiative: {{value: 2}}\n"
        ),
        &format!("\n# {name}\n"),
    )
    .unwrap();
    let path = d.root.join("campaigns/emberfall").join(relative);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, text).unwrap();
    id
}

/// The fixture's encounter with the party and two Ember Cultists, as saved.
fn shrine(d: &Driver<'_>) -> Value {
    let path = d
        .root
        .join(format!("campaigns/emberfall/Encounters/{LOW}.md"));
    let shrine = std::fs::read_to_string(path).unwrap();
    let shrine = splat::content::Entry::parse("Encounters/Shrine.md", &shrine).unwrap();
    shrine.fields["splat_encounter"].clone()
}

/// An encounter with the fixture's party, or with none, and these creatures against it.
fn write_encounter(d: &Driver<'_>, name: &str, party: bool, foes: &[uuid::Uuid]) {
    let folder = d.root.join("campaigns/emberfall/Encounters");
    let mut setup = shrine(d);
    let roster = setup["roster"].as_sequence_mut().unwrap();
    roster.retain(|slot| party && slot["side"] == "party");
    for (at, foe) in foes.iter().enumerate() {
        let slot = format!(
            "{{key: actor_foe_{at}, creature: {{id: {foe}}}, side: enemies, controller: {{kind: manual, role: gm}}}}"
        );
        roster.push(serde_yaml_ng::from_str(&slot).unwrap());
    }
    let mut fields = Mapping::new();
    for (key, value) in [
        (
            "splat_id",
            Value::from(crate::hermetic::new_uuid().to_string()),
        ),
        ("splat_kind", "encounter".into()),
        ("splat_name", name.into()),
        ("splat_encounter", setup),
    ] {
        fields.insert(key.into(), value);
    }
    let relative = format!("Encounters/{name}.md");
    let text = splat::content::formats::fixture(
        &relative,
        &serde_yaml_ng::to_string(&fields).unwrap(),
        "\nWritten for the difficulty scenario.\n",
    )
    .unwrap();
    std::fs::write(folder.join(format!("{name}.md")), text).unwrap();
}

/// The accessible name of a Ready to run row's second line.
fn ready_line(d: &Driver<'_>, name: &str) -> String {
    d.nodes()
        .into_iter()
        .find(|node| node.name.starts_with(&format!("{name} · ")) && node.in_view())
        .map(|node| node.name)
        .unwrap_or_default()
}

/// Each Ready to run row ends with its word, or with nothing when its XP is not known.
fn ready_rows_say_their_word(d: &mut Driver<'_>, id: &str) {
    for size in [SMALL, DEFAULT] {
        let lines = d.at_size(size, |d| {
            [HIGH, ABOVE, LOW, UNKNOWN].map(|name| ready_line(d, name))
        });
        let [high, above, low, unknown] = &lines;
        // Mill Road's long summary keeps its words: beside Lookup at 960x640 its word gives way,
        // and the accessible name keeps it (V7).
        d.observe(
            id,
            *high == format!("{HIGH} · {HIGH_SUMMARY} · High")
                && above == &format!("{ABOVE} · 2 × Cinder Tyrant · Above High")
                && low == &format!("{LOW} · 5 party · 2 × Ember Cultist · Low")
                && unknown == &format!("{UNKNOWN} · 5 party · 1 × Nameless Thing"),
            format!("At {size:?}, the Ready to run rows read {lines:?}"),
        );
    }
}

/// The blank before and after each `·` on the opened row's line that starts with
/// `Read encounter` (V9), between the glyphs' boxes as the font sets them. Empty when the line
/// is not in view.
fn separator_blanks(d: &Driver<'_>) -> Vec<(f32, f32)> {
    let mut link = None;
    d.each_painted_text(|text, clip| {
        let rect = text.visual_bounding_rect();
        if text.galley.job.text == "Read encounter" && clip.contains_rect(rect) {
            link = Some(rect);
        }
    });
    let Some(link) = link else {
        return Vec::new();
    };
    let mut glyphs = Vec::new();
    d.each_painted_text(|text, clip| {
        let rect = text.visual_bounding_rect();
        if clip.contains_rect(rect)
            && rect.left() >= link.left()
            && (rect.center().y - link.center().y).abs() <= 4.0
        {
            for row in &text.galley.rows {
                for glyph in &row.glyphs {
                    let left = text.pos.x + glyph.pos.x;
                    glyphs.push((glyph.chr, left, left + glyph.advance_width));
                }
            }
        }
    });
    glyphs.sort_by(|a, b| a.1.total_cmp(&b.1));
    let ink: Vec<_> = glyphs
        .iter()
        .filter(|glyph| !glyph.0.is_whitespace())
        .collect();
    (1..ink.len().saturating_sub(1))
        .filter(|at| ink[*at].0 == '·')
        .map(|at| (ink[at].1 - ink[at - 1].2, ink[at + 1].1 - ink[at].2))
        .collect()
}

/// V9: the same blank on each side of every separator on the opened row's line. Glyphs sit on
/// whole pixels, so the blanks agree to 1 px.
fn separators_have_one_blank(d: &mut Driver<'_>, what: &str, sizes: &[(f32, f32)]) {
    for size in sizes {
        let blanks = d.at_size(*size, separator_blanks);
        d.observe(
            "V9-separators",
            !blanks.is_empty()
                && blanks
                    .iter()
                    .all(|(before, after)| (before - after).abs() <= 1.0 && *before >= 2.0),
            format!("At {size:?}, {what}: the blank before and after each separator: {blanks:?}"),
        );
    }
}

/// The editor's line: the roster heading in a wide editor, the tab row in a narrow one.
fn editor_says(d: &mut Driver<'_>, id: &str, wide: &str, narrow: &str) {
    for size in [SMALL, DEFAULT, LARGE] {
        let (shown, words) = d.at_size(size, |d| {
            let words = d.painted_in(egui::Rect::EVERYTHING);
            (words.contains(wide) || words.contains(narrow), words)
        });
        d.observe(
            id,
            shown,
            format!("At {size:?} the editor says {wide:?} or {narrow:?}: {words}"),
        );
    }
}

/// A plan row's painted summary, which starts with these words, in the plain spaces it reads with.
fn plan_summary_line(d: &Driver<'_>, summary: &str) -> Option<String> {
    let mut found = None;
    d.each_painted_text(|text, clip| {
        // A line break stands for the separator it dropped (FI7).
        let words = text
            .galley
            .job
            .text
            .replace('\u{a0}', " ")
            .replace('\n', " · ");
        if found.is_none()
            && clip.intersects(text.visual_bounding_rect())
            && words.starts_with(summary)
        {
            found = Some(words);
        }
    });
    found
}

pub fn run(d: &mut Driver<'_>) {
    go_encounters(d);
    d.expect(READY_TO_RUN);
    let tyrant = write_creature(d, "Cinder Tyrant", Some("10"));
    let nameless = write_creature(d, "Nameless Thing", None);
    let cultist = {
        let shrine = shrine(d);
        let slot = shrine["roster"]
            .as_sequence()
            .unwrap()
            .iter()
            .find(|slot| slot["side"] == "enemies")
            .unwrap();
        slot["creature"]["id"].as_str().unwrap().parse().unwrap()
    };
    write_encounter(d, HIGH, true, &[tyrant, cultist, cultist]);
    write_encounter(d, ABOVE, false, &[tyrant, tyrant]);
    write_encounter(d, UNKNOWN, true, &[nameless]);
    d.focus_window(false);
    d.focus_window(true);
    d.expect(UNKNOWN);

    // T5.7: the Ready to run rows. Lookup closed, then open.
    ready_rows_say_their_word(d, "T5.7-ready");
    d.state("ready-to-run", Sizes::All);
    let stand_in = d.painted_tooltip_shows("Above High", STAND_IN);
    d.observe(
        "T5.7-hover",
        stand_in,
        "With no character in the roster, the hover names the campaign's party",
    );
    look_up(d, "wolf");
    ready_rows_say_their_word(d, "T5.7-ready-lookup");
    // V7: beside Lookup at 960x640 Mill Road's word gives way on its row, and the row's hover
    // keeps it, with the whole detail. The row is found by the start of its detail, which
    // stays painted also where the rest of the detail is cut.
    const HIGH_START: &str = "5 party · 1 × Cinder Tyrant";
    let (line, hover) = d.act_at_size(SMALL, |d| {
        let line = plan_summary_line(d, HIGH_START);
        let hover = d.painted_tooltip_shows(HIGH_START, &format!("{HIGH_SUMMARY} · High"));
        (line, hover)
    });
    let gave_way = line
        .as_deref()
        .is_some_and(|line| !line.contains(" · High"));
    d.observe(
        "V7-ready-hover",
        gave_way && hover,
        format!(
            "Beside Lookup at 960x640 the word gives way {gave_way} (the line: {line:?}), and the hover keeps it {hover}"
        ),
    );
    d.state("ready-to-run-beside-lookup", Sizes::Small);
    d.press_ctrl(Key::K);

    // The editor: the roster heading, or the tab row below 760 px. Its answer is worked out
    // when the draft changes, never on a frame that changes nothing.
    let work = CachedWork::start("Roster difficulty", d, |d: &mut Driver<'_>| {
        let (reached, work) = d.app_mut().encounters.difficulty_counts();
        Counts { reached, work }
    });
    open_saved_encounter(d, HIGH);
    d.expect(SAVE_ENCOUNTER);
    let heading = format!("ROSTER 8 · {HIGH_NUMBERS} · High");
    let shown = d.visible_text(&heading);
    let warm = work.warm(d, shown, 1);
    d.observe(
        "F1-cache-warm",
        warm.is_ok(),
        format!("The editor worked out the roster's difficulty: {warm:?}"),
    );
    let steady = work.steady(d, |d| {
        d.settle();
        d.at_size(LARGE, |_| ());
        d.settle();
        d.visible_text(&heading)
    });
    d.observe(
        "F1-cache-steady",
        steady.is_ok(),
        format!("Unchanged frames and a resize do not work it out again: {steady:?}"),
    );
    editor_says(
        d,
        "T5.7-editor",
        &heading,
        &format!("High · {HIGH_NUMBERS}"),
    );
    d.state("editor-high", Sizes::All);
    let budget = d.painted_tooltip_shows(HIGH_NUMBERS, BUDGET);
    d.observe(
        "T5.7-editor-hover",
        budget,
        "The hover gives the party's budgets",
    );
    look_up(d, "wolf");
    editor_says(
        d,
        "T5.7-editor-lookup",
        &heading,
        &format!("High · {HIGH_NUMBERS}"),
    );
    // X51: beside Lookup at the minimum window, Save encounter and Start fight keep their
    // place on the name field's line; the field gives up width.
    let line_one = d.at_size(SMALL, |d| {
        let field = d
            .nodes()
            .into_iter()
            .find(|node| node.name.starts_with("Encounter name"))?
            .rect;
        let on_line = |name: &str| {
            d.place_of(name)
                .map(|action| field.y_range().contains(action.center().y))
        };
        Some((on_line(SAVE_ENCOUNTER)?, on_line(RUN_ENCOUNTER)?))
    });
    d.observe(
        "X51-editor-actions-line-one",
        line_one == Some((true, true)),
        format!("Beside Lookup at 960x640, Save encounter and Start fight on the name's line: {line_one:?}"),
    );
    d.state("editor-high-beside-lookup", Sizes::Small);
    d.press_ctrl(Key::K);
    // One more Ember Cultist takes the fight past High: the line follows the draft at once.
    let refresh = work.refresh(d, 1, |d| {
        d.type_into(FIND_CREATURE, "ember cultist");
        d.click_first(ADD);
        d.expect("9. Ember Cultist");
        d.visible_text("ROSTER 9 · XP 6,500 · Above High")
    });
    d.observe(
        "F1-cache-refresh",
        refresh.is_ok(),
        format!("A changed roster gives its new word at once: {refresh:?}"),
    );
    d.click("Discard");
    d.click("Back to encounters");
    open_saved_encounter(d, ABOVE);
    d.expect(SAVE_ENCOUNTER);
    editor_says(
        d,
        "T5.7-editor-above",
        "ROSTER 2 · XP 11,800 · Above High",
        "Above High · XP 11,800",
    );
    d.click("Back to encounters");
    open_saved_encounter(d, UNKNOWN);
    d.expect(SAVE_ENCOUNTER);
    // X2 (by the lead's ruling after app round 2): the editor is where the DM can repair the
    // cause, so its line ends with whose XP is not known, and shows no word.
    for size in [SMALL, DEFAULT] {
        let words = d.at_size(size, |d| d.painted_in(egui::Rect::EVERYTHING));
        d.observe(
            "T5.7-editor-unknown",
            words.contains("ROSTER 6 · XP not known: Nameless Thing") && !words.contains("Low"),
            format!("At {size:?}, with an XP not known, the heading says whose: {words}"),
        );
    }
    d.state("editor-unknown", Sizes::Default);
    // Below 760 px the tab row says it, where the word would be; the name shortens from its
    // end before it goes.
    look_up(d, "wolf");
    editor_says(
        d,
        "X2-editor-unknown-lookup",
        "ROSTER 6 · XP not known: Nameless Thing",
        "XP not known: Nameless",
    );
    d.state("editor-unknown-beside-lookup", Sizes::Small);
    d.press_ctrl(Key::K);
    d.click("Back to encounters");

    // The session's plan: each planned fight's summary, or its opened row.
    d.click(NAV_SESSIONS);
    new_session(d, "The Toll Road");
    show_plan(d);
    for name in [HIGH, ABOVE, UNKNOWN, LOW] {
        d.click("Add a fight");
        d.click(&format!("Plan {name}"));
        d.expect(&format!("Start fight: {name}"));
    }
    for (lookup, sizes) in [(false, [SMALL, DEFAULT]), (true, [SMALL, SMALL])] {
        if lookup {
            look_up(d, "wolf");
            show_plan(d);
        }
        for size in sizes {
            let [low, unknown, high] = d.at_size(size, |d| {
                [
                    "5 party · 2 × Ember Cultist",
                    "5 party · 1 × Nameless Thing",
                    HIGH_SUMMARY,
                ]
                .map(|summary| plan_summary_line(d, summary))
            });
            // Mill Road's word is on its row only where the summary keeps its lines with it.
            d.observe(
                "T12.1-difficulty",
                low.as_deref() == Some("5 party · 2 × Ember Cultist · Low")
                    && unknown.as_deref() == Some("5 party · 1 × Nameless Thing")
                    && high.as_deref().is_some_and(|high| {
                        high == HIGH_SUMMARY || high == format!("{HIGH_SUMMARY} · High")
                    }),
                format!(
                    "At {size:?}, Lookup open {lookup}: the plan's summaries read {low:?}, \
                     {unknown:?}, {high:?}"
                ),
            );
        }
    }
    d.state("plan-beside-lookup", Sizes::Small);
    d.press_ctrl(Key::K);
    show_plan(d);
    d.state("plan", Sizes::All);
    // The opened row's line holds the word and the numbers, also where the summary has no room.
    d.click(HIGH);
    for size in [SMALL, DEFAULT, LARGE] {
        let words = d.at_size(size, |d| d.painted_in(egui::Rect::EVERYTHING));
        d.observe(
            "T12.1-difficulty-opened",
            words.contains("Read encounter") && words.contains(HIGH_NUMBERS),
            format!("At {size:?} the opened row says {HIGH_NUMBERS}: {words}"),
        );
    }
    separators_have_one_blank(d, "the opened row with its word", &[SMALL, DEFAULT, LARGE]);
    d.state("plan-opened", Sizes::All);
    // W1-D2: a fight with no word says why on its opened row, and whose. Its closed row is as
    // it was (above).
    d.click(HIGH);
    d.click(UNKNOWN);
    // Where the line has no room for the name, it says `XP not known`, and its hover and its
    // accessible name keep the name.
    const WHY: &str = "XP not known: Nameless Thing";
    for (lookup, sizes) in [(false, &[SMALL, DEFAULT, LARGE][..]), (true, &[SMALL][..])] {
        if lookup {
            look_up(d, "wolf");
            show_plan(d);
        }
        for size in sizes {
            let (words, named) = d.at_size(*size, |d| {
                let named = d
                    .nodes()
                    .into_iter()
                    .any(|node| node.name.ends_with(WHY) && node.fully_in_view());
                (d.painted_in(egui::Rect::EVERYTHING), named)
            });
            // X1: at every size, also at 1200x800 with the note beside the plan, the name
            // shortens from its end and keeps at least its first letter.
            let name_shown = words.contains(" · XP not known: N");
            d.observe(
                "W1-D2-why",
                words.contains("Read encounter")
                    && words.contains(" · XP not known")
                    && named
                    && name_shown,
                format!(
                    "At {size:?}, Lookup open {lookup}, the opened row says why, and whose                      {named}: {words}"
                ),
            );
        }
        if lookup {
            separators_have_one_blank(d, "the opened row with its reason beside Lookup", &[SMALL]);
            d.state("plan-opened-unknown-beside-lookup", Sizes::Small);
            d.press_ctrl(Key::K);
            show_plan(d);
        } else {
            separators_have_one_blank(d, "the opened row with its reason", &[SMALL, DEFAULT]);
            d.state("plan-opened-unknown", Sizes::All);
        }
    }
    d.click(UNKNOWN);
    // A paused fight's row shows its run, not a word, also when it is opened: its roster may
    // have changed in the run. The fixture's Bridge Ambush is paused, with XP 600 of 2,800.
    d.click("Add a fight");
    d.click(&format!("Plan {RUN}"));
    d.expect(&format!("Resume: {RUN}"));
    d.click(RUN);
    d.expect("Read encounter");
    let words = d.painted_in(egui::Rect::EVERYTHING);
    d.observe(
        "T12.1-difficulty-paused",
        !words.contains("XP 600 of 2,800") && !words.contains("Read encounter · "),
        format!("The opened paused fight's line has no difficulty: {words}"),
    );
    // App round 2, Sol's W1-D1: from the word on the plan to the roster and back. The last line
    // of a planned fight's menu opens that encounter's own file in the editor, whose way back
    // is this session; the saved roster's summary and word follow on its row.
    let session = d
        .app()
        .sessions
        .selected_path()
        .map(std::path::Path::to_owned);
    d.click(&format!("More actions for {LOW}"));
    d.click("Edit encounter");
    let opened = d.app().encounters.draft_path().map(str::to_owned);
    d.observe(
        "W1-D1-edit",
        opened.as_deref() == Some(&format!("Encounters/{LOW}.md")[..])
            && d.has(&format!("Encounter name: {LOW}"))
            && d.has("Back to session"),
        format!(
            "Edit encounter opens {LOW}'s own file with the way back to the session: {opened:?}"
        ),
    );
    d.click_first("More actions for Ember Cultist");
    d.click("Duplicate");
    d.click(SAVE_ENCOUNTER);
    d.click("Back to session");
    show_plan(d);
    let line = plan_summary_line(d, "5 party · 3 × Ember Cultist");
    d.observe(
        "W1-D1-edit",
        d.app().sessions.selected_path() == session.as_deref()
            && line.as_deref() == Some("5 party · 3 × Ember Cultist · Low"),
        format!("Back in the same session, the row reads the saved roster: {line:?}"),
    );
    // Astra's review: the act names the encounter by its ID. When its file now holds another
    // encounter (Mill Road's), the editor does not open it, and the plan says why.
    let folder = d.root.join("campaigns/emberfall/Encounters");
    let mill = std::fs::read_to_string(folder.join(format!("{HIGH}.md"))).unwrap();
    std::fs::write(folder.join(format!("{LOW}.md")), &mill).unwrap();
    d.click(&format!("More actions for {LOW}"));
    d.click("Edit encounter");
    const GONE: &str = "This encounter is no longer in the campaign. Open it from Encounters.";
    d.observe(
        "W1-D1-edit",
        d.app().encounters.draft_path().is_none()
            && d.page_title().as_deref() == Some("Session")
            && d.has_containing(GONE),
        format!(
            "A file that now holds another encounter opens nothing, and the plan says why: {:?}",
            d.app().encounters.draft_path()
        ),
    );
    // Review round 1: a file deleted since the campaign was read opens nothing either, and the
    // plan gives the same words with their next step, not the file system's error.
    std::fs::remove_file(folder.join(format!("{ABOVE}.md"))).unwrap();
    d.click(&format!("More actions for {ABOVE}"));
    d.click("Edit encounter");
    d.observe(
        "W1-D1-edit",
        d.app().encounters.draft_path().is_none()
            && d.page_title().as_deref() == Some("Session")
            && d.has_containing(GONE)
            && !d.has_containing("No such file"),
        "A deleted encounter file opens nothing, and the plan says why with its next step",
    );
    // Review round 1 (REVIEW-291-return-owner): the way back belongs to the route. After a
    // detour by the rail to a new session, the editor no longer offers `Back to session`, so no
    // press can land on the other session.
    d.click(&format!("More actions for {HIGH}"));
    d.click("Edit encounter");
    d.expect("Back to session");
    d.click(NAV_SESSIONS);
    d.click("New session");
    d.type_into("Session title", "Review detour session");
    d.click("Create session");
    d.expect("Session 2: Review detour session");
    let detour = d
        .app()
        .sessions
        .selected_path()
        .map(std::path::Path::to_owned);
    d.click(NAV_ENCOUNTERS);
    d.observe(
        "REVIEW-291-return-owner",
        detour.is_some()
            && detour != session
            && d.has("Back to encounters")
            && !d.has("Back to session"),
        format!(
            "Opened from {session:?}; after the rail made {detour:?}, Back to session shows {}",
            d.has("Back to session")
        ),
    );
    // X2, review 290: a roster member whose saved level an outside edit removed. Their sheet
    // cannot be read, and the heading still says whose level is not known, at each size and
    // in the tab row beside Lookup; a roster with no character names the campaign's copy too.
    go_encounters(d);
    let campaign = d.root.join("campaigns/emberfall");
    fn notes(at: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
        for entry in std::fs::read_dir(at).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                notes(&path, out);
            } else if path.extension().is_some_and(|e| e == "md") {
                out.push(path);
            }
        }
    }
    let mut paths = Vec::new();
    notes(&campaign, &mut paths);
    let path = paths
        .into_iter()
        .find(|p| {
            let source = std::fs::read_to_string(p).unwrap();
            splat::content::Entry::parse(p.to_str().unwrap(), &source).is_ok_and(|entry| {
                entry.fields.contains_key("splat_character") && source.contains("Brannoch Vale")
            })
        })
        .expect("Brannoch's parsed saved sheet");
    let source = std::fs::read_to_string(&path).unwrap();
    let (mut removed, mut in_sheet) = (0, false);
    let revised = source
        .lines()
        .filter(|line| {
            let raw = line.strip_prefix("> ").unwrap_or(line);
            if raw == "splat_character:" {
                in_sheet = true;
            } else if !raw.starts_with(' ') && !raw.trim().is_empty() {
                in_sheet = false;
            }
            if in_sheet && raw.starts_with("  level:") {
                removed += 1;
                false
            } else {
                true
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    assert_eq!(removed, 1, "remove exactly the sheet's saved level");
    std::fs::write(&path, revised).unwrap();
    d.focus_window(false);
    d.focus_window(true);
    for (encounter, id) in [
        (LOW, "review290-level-unknown"),
        (ABOVE, "review290-level-stand-in"),
    ] {
        open_saved_encounter(d, encounter);
        for size in [SMALL, DEFAULT] {
            let words = d.at_size(size, |d| d.painted_in(egui::Rect::EVERYTHING));
            d.observe(
                id,
                words.contains("Level not known: Brannoch Vale"),
                format!("{encounter} at {size:?}: {words}"),
            );
        }
        look_up(d, "wolf");
        let words = d.at_size(SMALL, |d| d.painted_in(egui::Rect::EVERYTHING));
        d.observe(
            id,
            words.contains("Level not known: Brannoch"),
            format!("{encounter} beside Lookup at {SMALL:?}: {words}"),
        );
        d.press_ctrl(Key::K);
        d.click("Back to encounters");
    }
}
