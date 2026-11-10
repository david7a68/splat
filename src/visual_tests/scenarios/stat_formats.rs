//! F2 tables feed Library, encounter search, and the reader beside a run.
use super::*;
use std::fs;

pub fn run(d: &mut Driver<'_>) {
    go_library(d);
    search_library(d, "Slate Wolf");
    d.click_containing("Slate Wolf · Creature");
    d.expect("22 (4d8 + 4)");
    d.expect("Bite.");
    d.state("schema-2-creature-library", Sizes::All);
    search_library(d, "Flagged Wolf");
    d.expect("1 value not read");
    d.click_containing("Flagged Wolf · ");
    d.expect("Hit Points \"about twenty\" not read (line 19). Start with a number, such as 22 (4d8 + 4).");
    d.expect("about twenty");
    d.expect("HIT POINTS");
    d.observe(
        "F2 rejected HP in vitals",
        !d.visible_text("| Hit Points | about twenty |"),
        "Rejected HP uses the reader's vital row, without raw table text",
    );
    d.state("flagged-hp-cell", Sizes::All);

    search_library(d, "Flagged Spark");
    d.expect("1 value not read");
    d.click_containing("Flagged Spark · ");
    d.expect("Level \"many\" not read (line 15). Use a number from 0 to 9.");
    d.expect("Keep the spark effect.");
    d.observe(
        "i19 valid spell fact",
        d.visible_text("Range"),
        "The valid Range fact stays visible beside the bad Level warning",
    );
    d.expect("Self");
    d.observe(
        "i19 bad spell cell",
        !d.visible_text("%% splat") && !d.visible_text("Field"),
        "The bad cell has a line warning; the effect stays readable without raw data",
    );
    d.state("flagged-spell-cell", Sizes::All);
    search_library(d, "Bound Charm");
    d.click_containing("Bound Charm · ");
    d.expect("Wondrous Item, Uncommon (Requires Attunement)");
    d.expect("Keep the charm effect.");
    d.observe(
        "i19 converted item",
        !d.has("Attunement") && !d.visible_text("%% splat") && !d.visible_text("Field"),
        "The converted item keeps attunement in its identity line without a repeated fact",
    );
    d.state("converted-item-facts", Sizes::All);

    go_encounters(d);
    open_saved_encounter(d, RUN);
    d.type_into(FIND_CREATURE, "Slate Wolf");
    d.expect("HP 22");
    d.expect("AC 13");
    d.click("Back to encounters");
    resume_run_from_cold_start(d);
    look_up(d, "Slate Wolf");
    d.expect("22 (4d8 + 4)");
    d.expect(NEXT_TURN);
    d.state_prepared("schema-2-creature-beside-run", Sizes::All, |d| {
        d.observe(
            "iter-6-C-FD10",
            d.visible_text("Close the run to convert these files.") && !d.has("Convert files…"),
            "Lookup states why conversion waits, with no disabled conversion action",
        );
    });
    d.click("Close Lookup");
    close_run(d);
    go_campaign(d);
    d.click("Convert files…");
    d.expect("Its converter will overwrite these files on its next run.");
    d.expect("2 files can use the new layout.");
    d.expect("1 rule · 1 World note");
    d.state("grouped-conversion-preview", Sizes::All);
    d.click("Ashen Marches");
    d.click("File to convert");
    d.click("Creatures/Cinder Wolf.md");
    d.expect("BEFORE");
    d.expect("AFTER");
    for _ in 0..8 {
        if d.in_view_containing("| Stat | Value |") {
            break;
        }
        d.click("Next difference");
    }
    d.expect("| Stat | Value |");
    d.scroll_at(pos2(640.0, 360.0), 2);
    d.expect("| Hit Points | 22 (4d8 + 4) |");
    d.expect("| Size | Medium |");
    d.expect("| Type | Beast |");
    d.expect("| Alignment | Unaligned |");
    d.state("creature-conversion-preview", Sizes::All);
    d.click("Cancel");
    let text = fs::read_to_string(
        d.root
            .join("modules/ashen-marches/Creatures/Cinder Wolf.md"),
    )
    .unwrap();
    assert!(text.contains("splat_schema: 1"), "Preview never writes");
}
