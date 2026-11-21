//! The rule contracts the engine implements. Only Rust code can add to this list, so content can
//! never claim support for itself. A contract's generated `engine_gaps` name what it requires. An
//! open gap is a required contract that is not on this list.
use std::collections::BTreeSet;
use std::sync::OnceLock;

pub const CLASSES: [&str; 12] = [
    "barbarian",
    "bard",
    "cleric",
    "druid",
    "fighter",
    "monk",
    "paladin",
    "ranger",
    "rogue",
    "sorcerer",
    "warlock",
    "wizard",
];

pub const SUBCLASSES: [&str; 12] = [
    "barbarian_path_of_the_berserker",
    "bard_college_of_lore",
    "cleric_cleric_subclass_life_domain",
    "druid_circle_of_the_land",
    "fighter_fighter_subclass_champion",
    "monk_monk_subclass_warrior_of_the_open_hand",
    "paladin_paladin_subclass_oath_of_devotion",
    "ranger_ranger_subclass_hunter",
    "rogue_rogue_subclass_thief",
    "sorcerer_sorcerer_subclass_draconic_sorcery",
    "warlock_warlock_subclass_fiend_patron",
    "wizard_wizard_subclass_evoker",
];

/// Every contract `crate::character::build` handles in full. A contract that needs anything more
/// stays open, even when the build reads part of it:
/// - Class notes also specify inventory changes and tool choices.
/// - Ability Score Improvement and Epic Boon also specify feat grants and their effects.
/// - Wizard Subclass also specifies copying spells into a spellbook.
pub fn implemented() -> &'static BTreeSet<String> {
    static IMPLEMENTED: OnceLock<BTreeSet<String>> = OnceLock::new();
    IMPLEMENTED.get_or_init(|| {
        let mut ids = BTreeSet::new();
        for class in CLASSES {
            ids.insert(format!("dnd2024:class_progression_{class}_progression"));
            if class != "wizard" {
                ids.insert(format!(
                    "dnd2024:class_features_{class}_level_3_{class}_subclass"
                ));
            }
        }
        ids.extend(SUBCLASSES.map(|subclass| format!("dnd2024:subclasses_{subclass}")));
        ids
    })
}

pub fn is_implemented(contract: &str) -> bool {
    implemented().contains(contract)
}
