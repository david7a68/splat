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
    d.click_containing("Flagged Wolf · Creature");
    d.expect("Hit Points \"about twenty\" needs a number first (line 19).");
    d.expect("about twenty");
    d.expect("HIT POINTS");
    d.observe(
        "F2 rejected HP in vitals",
        !d.visible_text("| Hit Points | about twenty |"),
        "Rejected HP uses the reader's vital row, without raw table text",
    );
    d.state("flagged-hp-cell", Sizes::All);

    search_library(d, "Flagged Spark");
    d.click_containing("Flagged Spark · Spell");
    d.expect("Level needs a number from 0 to 9");
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
    d.state("schema-2-creature-beside-run", Sizes::All);
    d.click("Close Lookup");
    close_run(d);
    go_campaign(d);
    d.click("Convert files…");
    d.expect("Generated package; its converter writes the layout");
    d.expect("4 files can use the new layout.");
    d.expect("1 encounter file · 1 rule file · 1 timeline file · 1 World note");
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
