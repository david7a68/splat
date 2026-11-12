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
const HIGH_SUMMARY: &str = "5 party · Cinder Tyrant · 2 × Ember Cultist";
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
                && unknown == &format!("{UNKNOWN} · 5 party · Nameless Thing"),
            format!("At {size:?}, the Ready to run rows read {lines:?}"),
        );
    }
}

/// The ink blank before and after each `·` on the opened row's line that starts with
/// `Read encounter` (V9). Empty when the line is not in view.
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
                    let left = text.pos.x + glyph.pos.x + glyph.uv_rect.offset.x;
                    glyphs.push((glyph.chr, left, left + glyph.uv_rect.size.x));
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

/// V9: the same blank on each side of every separator on the opened row's line. A letter's
/// own shape moves its ink by up to a pixel.
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
        let words = text.galley.job.text.replace('\u{a0}', " ");
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
    // keeps it.
    let (gave_way, hover) = d.act_at_size(SMALL, |d| {
        let gave_way = plan_summary_line(d, HIGH_SUMMARY).as_deref() == Some(HIGH_SUMMARY);
        let hover = d.painted_tooltip_shows(HIGH_SUMMARY, &format!("{HIGH_SUMMARY} · High"));
        (gave_way, hover)
    });
    d.observe(
        "V7-ready-hover",
        gave_way && hover,
        format!(
            "Beside Lookup at 960x640 the word gives way {gave_way}, and the hover keeps it {hover}"
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
    for size in [SMALL, DEFAULT] {
        let words = d.at_size(size, |d| d.painted_in(egui::Rect::EVERYTHING));
        d.observe(
            "T5.7-editor-unknown",
            words.contains("ROSTER 6") || words.contains("Roster"),
            format!("At {size:?} the roster's line is there: {words}"),
        );
        d.observe(
            "T5.7-editor-unknown",
            !words.contains("XP ") && !words.contains("Low"),
            format!("At {size:?}, with an XP not known, the line is as today: {words}"),
        );
    }
    d.state("editor-unknown", Sizes::Default);
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
                    "5 party · Nameless Thing",
                    HIGH_SUMMARY,
                ]
                .map(|summary| plan_summary_line(d, summary))
            });
            // Mill Road's word is on its row only where the summary keeps its lines with it.
            d.observe(
                "T12.1-difficulty",
                low.as_deref() == Some("5 party · 2 × Ember Cultist · Low")
                    && unknown.as_deref() == Some("5 party · Nameless Thing")
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
    const WHY: &str = " · XP not known: Nameless Thing";
    for (lookup, sizes) in [(false, &[SMALL, DEFAULT, LARGE][..]), (true, &[SMALL][..])] {
        if lookup {
            look_up(d, "wolf");
            show_plan(d);
        }
        for size in sizes {
            let words = d.at_size(*size, |d| d.painted_in(egui::Rect::EVERYTHING));
            d.observe(
                "W1-D2-why",
                words.contains("Read encounter") && words.contains(WHY),
                format!("At {size:?}, Lookup open {lookup}, the opened row says why: {words}"),
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
}
