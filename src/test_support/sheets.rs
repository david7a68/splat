//! Synthetic sheets for Party tests and the visual suite.
use crate::hermetic;
use serde_json::{Value, json};
use splat_character::character::EntitySnapshot;

pub const REMOTE_CAMPAIGN: &str = "7700001";

/// One party member's sheet, for a test that needs a sheet and not a whole campaign.
pub fn sample_sheet() -> EntitySnapshot {
    HEROES[0].snapshot().expect("The first hero makes a sheet")
}

impl Hero {
    pub fn portrait_color(&self) -> [u8; 3] {
        let seed = self.name.bytes().fold(7_u32, |sum, b| {
            sum.wrapping_mul(31).wrapping_add(u32::from(b))
        });
        [
            70 + (seed % 150) as u8,
            70 + (seed / 150 % 150) as u8,
            70 + (seed / 22500 % 150) as u8,
        ]
    }
}

pub struct Hero {
    pub id: &'static str,
    name: &'static str,
    species: &'static str,
    class: &'static str,
    level: u64,
    hp: (i64, i64),
    temp: i64,
    armor: i64,
    initiative: i64,
    /// STR, DEX, CON, INT, WIS, CHA
    scores: [i64; 6],
    proficient: &'static [&'static str],
    /// Name, to hit, damage dice, damage type, and where it lands: `Reach` or `Range` and its
    /// feet. A sheet can give none.
    #[allow(clippy::type_complexity)]
    attacks: &'static [(
        &'static str,
        i64,
        &'static str,
        &'static str,
        Option<(&'static str, u64)>,
    )],
    spells: &'static [(&'static str, u64, &'static str)],
    features: &'static [(&'static str, &'static str)],
    items: &'static [(&'static str, u64, bool, &'static str)],
    conditions: &'static [&'static str],
}

pub const HEROES: [Hero; 5] = [
    Hero {
        id: "9100001",
        name: "Brannoch Vale",
        species: "Dwarf",
        class: "Fighter",
        level: 6,
        hp: (58, 62),
        temp: 5,
        armor: 19,
        initiative: 1,
        scores: [18, 12, 16, 10, 13, 8],
        proficient: &["Athletics", "Intimidation", "Perception", "Survival"],
        attacks: &[
            ("Longsword", 7, "1d8", "Slashing", Some(("Reach", 5))),
            (
                "Heavy Crossbow",
                4,
                "1d10",
                "Piercing",
                Some(("Range", 100)),
            ),
        ],
        spells: &[],
        features: &[
            (
                "Second Wind",
                "Regain Hit Points as a Bonus Action. Short Rest recovers one use.",
            ),
            ("Action Surge", "Take one additional action on your turn."),
            (
                "Extra Attack",
                "Attack twice when you take the Attack action.",
            ),
            (
                "Tactical Mind",
                "Spend a use of Second Wind to add 1d10 to a failed ability check.",
            ),
        ],
        items: &[
            ("Plate Armor", 1, true, "Heavy armor. AC 18."),
            ("Shield", 1, true, "+2 to Armor Class."),
            ("Longsword", 1, true, "Versatile (1d10). Sap mastery."),
            ("Rations", 5, false, "One day of food."),
        ],
        conditions: &[],
    },
    Hero {
        id: "9100002",
        name: "Iselle Marrowind",
        species: "Elf",
        class: "Wizard",
        level: 6,
        hp: (27, 38),
        temp: 0,
        armor: 13,
        initiative: 3,
        scores: [8, 16, 14, 18, 12, 10],
        proficient: &["Arcana", "History", "Investigation", "Insight"],
        attacks: &[
            ("Dagger", 6, "1d4", "Piercing", Some(("Reach", 5))),
            ("Fire Bolt", 7, "2d10", "Fire", Some(("Range", 120))),
        ],
        spells: &[
            (
                "Fire Bolt",
                0,
                "Hurl a mote of fire at a creature or object.",
            ),
            ("Mage Hand", 0, "A spectral hand appears within range."),
            // Nine spells keep the long-list filter covered by the navigation scenarios.
            ("Light", 0, "A small object gives light."),
            (
                "Shield",
                1,
                "Reaction. +5 to AC until the start of your next turn.",
            ),
            (
                "Magic Missile",
                1,
                "Three glowing darts strike creatures of your choice.",
            ),
            (
                "Misty Step",
                2,
                "Teleport up to 30 feet to an unoccupied space you can see.",
            ),
            (
                "Web",
                2,
                "Conjure a mass of sticky webbing in a 20-foot Cube.",
            ),
            (
                "Fireball",
                3,
                "A bright streak blossoms into an explosion in a 20-foot-radius Sphere.",
            ),
            (
                "Counterspell",
                3,
                "Attempt to interrupt a creature casting a spell.",
            ),
        ],
        features: &[
            (
                "Arcane Recovery",
                "Recover expended spell slots on a Short Rest.",
            ),
            (
                "Ritual Adept",
                "Cast any Wizard spell with the Ritual tag from your spellbook.",
            ),
            ("Scholar", "Gain Expertise in one skill."),
        ],
        items: &[
            ("Spellbook", 1, true, "Holds your Wizard spells."),
            ("Quarterstaff", 1, true, "Versatile (1d8). Topple mastery."),
        ],
        conditions: &[],
    },
    Hero {
        id: "9100003",
        name: "Kestrel Dunmoor",
        species: "Halfling",
        class: "Rogue",
        level: 5,
        hp: (11, 41),
        temp: 0,
        armor: 15,
        initiative: 4,
        scores: [10, 18, 14, 13, 12, 14],
        proficient: &["Acrobatics", "Deception", "Sleight of Hand", "Stealth"],
        attacks: &[
            ("Shortsword", 7, "1d6", "Piercing", Some(("Reach", 5))),
            ("Shortbow", 7, "1d6", "Piercing", Some(("Range", 80))),
        ],
        spells: &[],
        features: &[
            (
                "Sneak Attack",
                "Once per turn, deal an extra 3d6 damage to one creature you hit.",
            ),
            (
                "Cunning Action",
                "Dash, Disengage or Hide as a Bonus Action.",
            ),
            (
                "Uncanny Dodge",
                "Reaction. Halve the damage of an attack you can see.",
            ),
        ],
        items: &[
            (
                "Studded Leather Armor",
                1,
                true,
                "Light armor. AC 12 + Dexterity.",
            ),
            ("Thieves' Tools", 1, false, "Pick locks and disarm traps."),
        ],
        conditions: &["Poisoned"],
    },
    Hero {
        id: "9100004",
        name: "Oren Thistlewick",
        species: "Human",
        class: "Cleric",
        level: 6,
        hp: (45, 45),
        temp: 0,
        armor: 18,
        initiative: 0,
        scores: [14, 10, 14, 11, 18, 13],
        proficient: &["Insight", "Medicine", "Religion", "Persuasion"],
        attacks: &[("Mace", 5, "1d6", "Bludgeoning", None)],
        spells: &[
            ("Guidance", 0, "Add 1d4 to one ability check."),
            ("Cure Wounds", 1, "Touch a creature to restore Hit Points."),
            (
                "Bless",
                1,
                "Up to three creatures add 1d4 to attack rolls and saving throws.",
            ),
            (
                "Spiritual Weapon",
                2,
                "Create a floating spectral weapon within range.",
            ),
            (
                "Spirit Guardians",
                3,
                "Protective spirits flit around you in a 15-foot Emanation.",
            ),
        ],
        features: &[
            (
                "Channel Divinity",
                "Channel divine energy to fuel magical effects.",
            ),
            (
                "Sear Undead",
                "Add Wisdom-based Radiant damage when you Turn Undead.",
            ),
        ],
        items: &[
            ("Chain Mail", 1, true, "Heavy armor. AC 16."),
            ("Holy Symbol", 1, true, "A spellcasting focus."),
        ],
        conditions: &[],
    },
    Hero {
        id: "9100005",
        name: "Sable Harrowgate",
        species: "Tiefling",
        class: "Warlock",
        level: 5,
        hp: (32, 38),
        temp: 3,
        armor: 14,
        initiative: 2,
        scores: [8, 14, 14, 12, 10, 18],
        proficient: &["Deception", "Intimidation", "Arcana", "Nature"],
        attacks: &[("Eldritch Blast", 7, "1d10", "Force", Some(("Range", 120)))],
        spells: &[
            (
                "Eldritch Blast",
                0,
                "A beam of crackling energy streaks toward a creature.",
            ),
            ("Hex", 1, "Curse a creature you can see within range."),
            (
                "Armor of Agathys",
                1,
                "Spectral frost grants Temporary Hit Points.",
            ),
            (
                "Hold Person",
                2,
                "A Humanoid makes a Wisdom save or has the Paralyzed condition.",
            ),
            (
                "Hunger of Hadar",
                3,
                "Open a gateway to the darkness between the stars.",
            ),
        ],
        features: &[
            (
                "Eldritch Invocations",
                "Fragments of forbidden knowledge grant lasting abilities.",
            ),
            ("Pact of the Tome", "Stitch together a Book of Shadows."),
        ],
        items: &[("Leather Armor", 1, true, "Light armor. AC 11 + Dexterity.")],
        conditions: &[],
    },
];

const ABILITIES: [&str; 6] = ["str", "dex", "con", "int", "wis", "cha"];
const SKILLS: [(&str, usize); 18] = [
    ("Athletics", 0),
    ("Acrobatics", 1),
    ("Sleight of Hand", 1),
    ("Stealth", 1),
    ("Arcana", 3),
    ("History", 3),
    ("Investigation", 3),
    ("Nature", 3),
    ("Religion", 3),
    ("Animal Handling", 4),
    ("Insight", 4),
    ("Medicine", 4),
    ("Perception", 4),
    ("Survival", 4),
    ("Deception", 5),
    ("Intimidation", 5),
    ("Performance", 5),
    ("Persuasion", 5),
];

impl Hero {
    pub fn snapshot(&self) -> Result<EntitySnapshot, String> {
        let bonus = 2 + (self.level as i64 - 1) / 4;
        let modifier = |index: usize| (self.scores[index] - 10).div_euclid(2);
        let abilities: Vec<Value> = ABILITIES
            .iter()
            .enumerate()
            .map(|(index, name)| {
                json!({
                    "name": name,
                    "score": self.scores[index],
                    "modifier": modifier(index),
                    "save": modifier(index) + if index == 0 || index == 2 { bonus } else { 0 },
                })
            })
            .collect();
        let skills: Vec<Value> = SKILLS
            .iter()
            .map(|(name, ability)| {
                let proficient = self.proficient.contains(name);
                json!({
                    "name": name,
                    "modifier": modifier(*ability) + if proficient { bonus } else { 0 },
                    "isProficient": proficient,
                    "isExpert": false,
                    "isHalfProficient": false,
                    "isAdvantage": false,
                    "isDisadvantage": false,
                })
            })
            .collect();
        let perception = 10
            + modifier(4)
            + if self.proficient.contains(&"Perception") {
                bonus
            } else {
                0
            };
        let attacks: Vec<Value> = self
            .attacks
            .iter()
            .map(|(name, hit, dice, kind, lands)| {
                let range = match lands {
                    Some(("Reach", feet)) => json!({"label": "Reach", "value": feet}),
                    Some((_, feet)) => json!({"origin": "Ranged", "value": feet}),
                    None => Value::Null,
                };
                json!({
                    "name": name,
                    "toHit": hit,
                    "damage": {"value": {"diceString": dice, "fixedValue": modifier(0).max(modifier(1))}, "type": kind},
                    "range": range,
                })
            })
            .collect();
        let casting = if self.spells.is_empty() {
            json!({})
        } else {
            json!({
                "saveDcs": [{"value": 8 + bonus + self.scores.iter().max().map_or(0, |s| (s - 10) / 2), "sources": [self.class]}],
                "spellAttacks": [{"value": bonus + self.scores.iter().max().map_or(0, |s| (s - 10) / 2), "sources": [self.class]}],
            })
        };
        let spells: Vec<Value> = self
            .spells
            .iter()
            .map(|(name, level, text)| {
                json!({"prepared": true, "definition": {"name": name, "level": level, "description": format!("<p>{text}</p>")}})
            })
            .collect();
        let features: Vec<Value> = self
            .features
            .iter()
            .enumerate()
            .map(|(index, (name, text))| {
                json!({"definition": {"id": index + 1, "name": name, "requiredLevel": 1, "description": format!("<p>{text}</p>")}})
            })
            .collect();
        let inventory: Vec<Value> = self
            .items
            .iter()
            .enumerate()
            .map(|(index, (name, quantity, equipped, text))| {
                json!({
                    "id": index + 1,
                    "quantity": quantity,
                    "equipped": equipped,
                    "definition": {"id": index + 1, "entityTypeId": 1, "name": name, "description": format!("<p>{text}</p>")},
                })
            })
            .collect();
        let conditions: Vec<Value> = self
            .conditions
            .iter()
            .map(|name| json!({"name": name}))
            .collect();
        // One hero has no portrait, as many D&D Beyond characters have none.
        let portrait = (self.name != "Oren Thistlewick")
            .then(|| format!("https://www.dndbeyond.com/avatars/visual/{}.png", self.id));
        let raw = json!({
            "id": self.id.parse::<u64>().map_err(|e| e.to_string())?,
            "name": self.name,
            "decorations": {"avatarUrl": portrait},
            "campaign": {"id": REMOTE_CAMPAIGN.parse::<u64>().map_err(|e| e.to_string())?},
            "race": {"sizeId": 4, "creatureTypeId": 11, "racialTraits": [
                {"definition": {"name": format!("{} Resilience", self.species), "description": "<p>Your ancestry grants advantage against one kind of harm.</p>"}},
            ]},
            "classes": [{
                "id": 1,
                "level": self.level,
                "hitDiceUsed": 1,
                "definition": {"name": self.class},
                "classFeatures": features,
            }],
            "inventory": inventory,
            "currencies": {"cp": 12, "sp": 40, "ep": 0, "gp": 215, "pp": 1},
            "spells": {"class": spells},
            "actions": {"class": [{
                "id": 10,
                "name": format!("{} Reserve", self.class),
                "limitedUse": {"maxUses": 3, "numberUsed": 1},
            }]},
            "background": {"hasCustomBackground": false, "definition": {
                "id": 1, "definitionKey": "1669830167:1",
                "name": if self.spells.is_empty() { "Soldier" } else { "Sage" },
                "description": "<p>Years before the road, this life shaped how you see a fight and a friend.</p>",
                "grantedFeats": [], "featureIsFeat": false,
                "featureName": "Old Contacts",
                "featureDescription": "<p>Someone in most towns remembers you and will share news.</p>",
            }},
            "alignmentId": 2,
            "lifestyleId": 5,
            "faith": "The Lantern Keeper",
            "gender": "Any",
            "age": 30 + self.level,
            "height": "5'8\"",
            "weight": 160,
            "eyes": "Grey",
            "hair": "Brown",
            "username": "player_one",
            "traits": {
                "personalityTraits": format!("{} checks every exit before sitting down.", self.name),
                "ideals": "Loyalty. The party comes home together.",
                "bonds": "A debt owed to the ferry captain at Salt Road.",
                "flaws": "Trusts a coin toss more than a plan.",
            },
            "notes": {
                "otherNotes": "",
                "backstory": format!("{} left home after the floods.\n\nThe road has been kinder than the river.", self.name),
                "allies": "The ferry captain at Salt Road",
            },
        });
        let computed = json!({
            "characterId": self.id.parse::<u64>().map_err(|e| e.to_string())?,
            "name": self.name,
            "campaign": {"id": REMOTE_CAMPAIGN.parse::<u64>().map_err(|e| e.to_string())?},
            "level": self.level,
            "armorClass": self.armor,
            "proficiencyBonus": bonus,
            "initiativeBonus": self.initiative,
            "passivePerception": perception,
            "passiveInsight": 10 + modifier(4),
            "passiveInvestigation": 10 + modifier(3),
            "abilities": abilities,
            "skills": skills,
            "hitPointInfo": {"maximum": self.hp.1, "current": self.hp.0, "temp": self.temp},
            "deathSaveInfo": {"successCount": 0, "failCount": 0},
            "speeds": [{"name": "Walking", "distance": 30}],
            "race": {"name": self.species},
            "classes": [{"name": self.class, "level": self.level}],
            "attacks": attacks,
            "castingInfo": casting,
            "senses": [{"name": "Darkvision", "distance": "60 ft."}],
            "proficiencyGroups": [
                {"group": "Armor", "values": "Light, Medium, Shields"},
                {"group": "Weapons", "values": "Simple, Martial"},
                {"group": "Languages", "values": "Common, Dwarvish, Sylvan"},
            ],
            "resistances": [{"name": "Fire"}],
            "conditions": conditions,
            "inspiration": self.temp > 0,
        });
        // Fetched three hours before the fixed test clock.
        let fetched = hermetic::closed::FIXED_NOW - 3 * 3600;
        EntitySnapshot::from_beyond(raw, computed, REMOTE_CAMPAIGN, self.id, fetched, fetched)
    }
}
