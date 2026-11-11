//! F2 tables feed Library, encounter search, and the reader beside a run.
use super::*;

pub fn run(d: &mut Driver<'_>) {
    go_library(d);
    search_library(d, "Slate Wolf");
    d.click_containing("Slate Wolf · Creature");
    d.expect("22 (4d8 + 4)");
    d.expect("Bite · +");
    // FD2: a campaign creature's toolbar adds Make NPC note. Every toolbar action is a Read
    // row, so the title stays where a campaign item's title is (compared below).
    d.expect("Campaign · Editable");
    let creature_title = super::world_build::reader_title_tops(d, "Slate Wolf");
    for size in [SMALL, DEFAULT, (1920.0, 1080.0)] {
        let (make, read, more) = d.at_size(size, |d| {
            (
                d.place_of("Make NPC note"),
                d.place_of("Read"),
                d.place_of("More"),
            )
        });
        // Must stay true: shown, Make NPC note is a Read-density control on Read's line;
        // folded, More holds it.
        d.observe(
            "iter-18-E-FD2-row",
            match (make, read) {
                (Some(make), Some(read)) => {
                    (make.height() - crate::design::Density::Read.control()).abs() <= 0.5
                        && (make.center().y - read.center().y).abs() <= 1.0
                }
                (None, Some(_)) => more.is_some(),
                _ => false,
            },
            format!(
                "At {size:?}, Make NPC note {make:?} is a Read row on the toolbar line with Read {read:?} (or More {more:?} holds it)"
            ),
        );
    }
    d.state("schema-2-creature-library", Sizes::All);
    search_library(d, "Flagged Wolf");
    d.expect("1 value not read");
    d.click_containing("Flagged Wolf · ");
    d.expect(
        "Hit Points “about twenty” not read (line 19). Start with a number, such as 22 (4d8 + 4).",
    );
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
    d.expect("Level “many” not read (line 15). Use a number from 0 to 9.");
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
    d.expect("Campaign · Editable");
    assert!(
        !d.has("Make NPC note"),
        "fixture: an item has no Make NPC note"
    );
    let item_title = super::world_build::reader_title_tops(d, "Bound Charm");
    d.observe(
        "iter-18-E-FD2",
        creature_title.iter().all(Option::is_some) && creature_title == item_title,
        format!(
            "The campaign creature's title is at the campaign item's height (960, 1200, 1920): Slate Wolf {creature_title:?}, Bound Charm {item_title:?}"
        ),
    );
    d.expect("Keep the charm effect.");
    d.observe(
        "i19 item identity",
        !d.has("Attunement") && !d.visible_text("%% splat") && !d.visible_text("Field"),
        "The item keeps attunement in its identity line without a repeated fact",
    );
    d.state("item-facts", Sizes::All);

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
}
