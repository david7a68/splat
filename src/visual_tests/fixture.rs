//! Synthetic worlds for the visual suite. Every file lives in a temporary folder,
//! every name is invented, and the only other content is the built-in SRD.
use crate::{
    Splat,
    campaigns::Campaigns,
    content_workspace::{CampaignDocument, EntryRef, PackageSelection, Selection},
    encounter_run::{Participant, write_in_progress_run},
    hermetic,
    persistence::{Campaign, Module, Store, UserData},
};
use serde_json::{Value, json};
use serde_yaml_ng::Mapping;
use splat::{
    character::{Cache, EntitySnapshot, stable_id},
    content::Entry,
};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub const REMOTE_CAMPAIGN: &str = "7700001";
const QUIET_REMOTE_CAMPAIGN: &str = "7700002";

pub const CAMPAIGN: &str = "The Emberfall Marches";
pub const QUIET_CAMPAIGN: &str = "Quiet Harbor";
pub const UNLINKED_CAMPAIGN: &str = "Hollow Keep";
pub const MODULE: &str = "Ashen Marches";
pub const RUN: &str = "Bridge Ambush";
pub const RUN_12: &str = "Border Skirmish";
pub const RUN_20: &str = "Siege of Emberfall";

/// What settings the app starts with.
#[derive(Clone, Copy, PartialEq)]
pub enum Seed {
    /// First launch: no settings file, campaigns or modules.
    Empty,
    /// One active campaign with content, a party and a run in progress, plus a
    /// linked campaign with no characters and an unlinked one.
    Campaign,
    /// `Campaign`, plus saved runs with rosters of 12 and 20.
    LargeRosters,
}

#[derive(Clone)]
pub struct World {
    settings: PathBuf,
}

impl World {
    /// `root` is unique to the scenario. `scenario` keeps its campaign identity
    /// apart from the others in the shared character cache folder.
    pub fn build(root: &Path, cache: &Path, scenario: &str, seed: Seed) -> Result<Self, String> {
        fs::create_dir_all(root).map_err(|e| e.to_string())?;
        let settings = root.join("settings").join("settings.json");
        if seed != Seed::Empty {
            seed_campaigns(root, cache, scenario, &settings, seed)?;
        }
        Ok(Self { settings })
    }

    pub fn app(&self) -> Splat {
        Splat {
            campaigns: Campaigns::open_at(self.settings.clone()),
            ui_store: crate::ui_state::Store::at(self.settings.with_file_name("ui-state.json")),
            ..Default::default()
        }
    }
}

fn seed_campaigns(
    root: &Path,
    cache: &Path,
    scenario: &str,
    settings: &Path,
    seed: Seed,
) -> Result<(), String> {
    let module = write_module(&root.join("modules").join("ashen-marches"))?;
    let active = campaign(root, scenario, CAMPAIGN, "emberfall", Some(REMOTE_CAMPAIGN))?;
    let quiet = campaign(
        root,
        scenario,
        QUIET_CAMPAIGN,
        "quiet-harbor",
        Some(QUIET_REMOTE_CAMPAIGN),
    )?;
    let unlinked = campaign(root, scenario, UNLINKED_CAMPAIGN, "hollow-keep", None)?;

    let mut document = CampaignDocument::load(&active)?;
    document.selections.push(PackageSelection {
        package: EntryRef::id(module.id),
        selection: Selection::Selected {
            entries: [
                "cinder_wolf",
                "ember_cultist",
                "ashfall",
                "veilglass_lantern",
            ]
            .map(|entry| EntryRef::id(module_entry_id(entry)))
            .to_vec(),
        },
    });
    document.save()?;

    let party = party(&active.path, cache, active.id)?;
    write_campaign_notes(&active.path)?;
    write_encounters(&active.path, &party)?;
    write_run(&active, &module.path, &party, RUN, &[(2, 17), (1, 9)])?;
    if seed == Seed::LargeRosters {
        write_run(&active, &module.path, &party, RUN_12, &[(5, 15), (2, 7)])?;
        write_run(&active, &module.path, &party, RUN_20, &[(12, 15), (3, 7)])?;
    }

    let data = UserData {
        version: 3,
        active_campaign_id: Some(active.id),
        campaigns: vec![active, quiet, unlinked],
        modules: vec![module],
    };
    let mut store = Store::at(settings.to_path_buf());
    store.load()?;
    store.save(&data)
}

fn campaign(
    root: &Path,
    scenario: &str,
    name: &str,
    folder: &str,
    remote: Option<&str>,
) -> Result<Campaign, String> {
    let path = root.join("campaigns").join(folder);
    fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    let campaign = Campaign {
        id: stable_id(&format!("visual:{scenario}:{folder}")),
        name: name.into(),
        path,
        dndbeyond_campaign_id: remote.map(str::to_owned),
        dndbeyond_campaign_name: remote.map(|_| format!("{name} (D&D Beyond)")),
    };
    CampaignDocument::save_metadata(&campaign, None)?;
    Ok(campaign)
}

fn module_entry_id(name: &str) -> uuid::Uuid {
    stable_id(&format!("visual:module:{name}"))
}

fn write_note(root: &Path, relative: &str, text: &str) -> Result<(), String> {
    let path = root.join(relative);
    fs::create_dir_all(path.parent().ok_or("Missing folder")?).map_err(|e| e.to_string())?;
    fs::write(path, text).map_err(|e| e.to_string())?;
    Entry::parse(relative, text).map(|_| ())
}

fn write_module(root: &Path) -> Result<Module, String> {
    let id = stable_id("visual:module");
    write_note(
        root,
        "Module.md",
        &format!(
            "---\nsplat_schema: 1\nsplat_id: {id}\nsplat_kind: package\nsplat_name: {MODULE}\nsplat_namespace: ashen_marches\n---\n\n# {MODULE}\n\nHomebrew creatures, spells and hazards for a burned frontier.\n"
        ),
    )?;
    let creature = |slug: &str,
                    name: &str,
                    hp: i64,
                    ac: i64,
                    init: i64,
                    identity: &str,
                    prose: &str| {
        let requires = if slug == "ember_cultist" {
            "splat_requires:\n- key: ashen_marches:rule/ash_exposure\n"
        } else {
            ""
        };
        format!(
            "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: creature\nsplat_key: ashen_marches:creatures/{slug}\nsplat_name: {name}\n{requires}splat_creature: {{size: medium, type: {identity}}}\nsplat_stats:\n  max_hp: {{value: {hp}}}\n  armor_class: {{value: {ac}}}\n  initiative: {{value: {init}}}\n---\n\n# {name}\n\n## Source text\n\nMedium {identity}, Unaligned\\\nAC {ac}  Initiative +{init} ({})\\\nHP {hp} (4d8 + 4)\\\nSpeed 30 ft.\\\nCR 1 (XP 200; PB +2)\\\n\n### Actions\n\n{prose}\n",
            module_entry_id(slug),
            10 + init,
        )
    };
    let note = |slug: &str, kind: &str, name: &str, body: &str| {
        format!(
            "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: {kind}\nsplat_key: ashen_marches:{kind}/{slug}\nsplat_name: {name}\nsplat_{kind}: {{}}\n---\n\n# {name}\n\n{body}\n",
            module_entry_id(slug),
        )
    };
    for (path, text) in [
        (
            "Creatures/Cinder Wolf.md",
            creature(
                "cinder_wolf",
                "Cinder Wolf",
                22,
                13,
                2,
                "beast",
                "Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 7 (2d4 + 2) Piercing damage plus 3 (1d6) Fire damage.",
            ),
        ),
        (
            "Creatures/Ember Cultist.md",
            creature(
                "ember_cultist",
                "Ember Cultist",
                33,
                12,
                1,
                "humanoid",
                "Scorching Sickle. Melee Attack Roll: +3, reach 5 ft. Hit: 5 (1d6 + 2) Slashing damage.\n\nSpellcasting. The cultist casts Ashfall, using Wisdom as the spellcasting ability.",
            ),
        ),
        (
            "Creatures/Ash Wraith.md",
            creature(
                "ash_wraith",
                "Ash Wraith",
                45,
                14,
                3,
                "undead",
                "Choking Touch. Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6 + 3) Necrotic damage.",
            ),
        ),
        (
            "Spells/Ashfall.md",
            note(
                "ashfall",
                "spell",
                "Ashfall",
                "Hot ash falls in a 20-foot-radius Sphere. Each creature there makes a Constitution saving throw or has the Blinded condition until the end of its next turn.",
            ),
        ),
        (
            "Items/Veilglass Lantern.md",
            note(
                "veilglass_lantern",
                "item",
                "Veilglass Lantern",
                "This lantern of smoked glass sheds Dim Light in a 15-foot radius. Creatures in that light can see through ash and smoke.",
            ),
        ),
        (
            "Rules/Ash Exposure.md",
            note(
                "ash_exposure",
                "rule",
                "Ash Exposure",
                "At the end of each hour in the ash fields, a creature without a face covering makes a DC 12 Constitution saving throw or gains one level of Exhaustion.\n\n## Recovery\n\nA Long Rest in clean air removes one level of Exhaustion gained this way.",
            ),
        ),
    ] {
        write_note(root, path, &text)?;
    }
    Ok(Module {
        id,
        name: MODULE.into(),
        path: root.to_path_buf(),
    })
}

fn write_campaign_notes(root: &Path) -> Result<(), String> {
    write_note(
        root,
        "Notes/Captain Varra.md",
        &format!(
            "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: creature\nsplat_name: Captain Varra\nsplat_creature: {{size: medium, type: humanoid}}\nsplat_stats:\n  max_hp: {{value: 52}}\n  armor_class: {{value: 16}}\n  initiative: {{value: 2}}\n---\n\n# Captain Varra\n\nThe last commander of the Emberfall watch. She trusts the party until they side with the cult.\n\n## Actions\n\nLongsword. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3) Slashing damage.\n",
            stable_id("visual:note:varra"),
        ),
    )?;
    write_note(
        root,
        "Notes/Emberfall Rumours.md",
        &format!(
            "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: rule\nsplat_name: Emberfall Rumours\nsplat_rule: {{}}\n---\n\n# Emberfall Rumours\n\n- The bridge toll collectors have not been paid in a month.\n- Someone lights signal fires on the old shrine at night.\n- Wolves with burning eyes follow the ash line south.\n",
            stable_id("visual:note:rumours"),
        ),
    )
}

fn slot(key: &str, creature: uuid::Uuid, side: &str, role: &str) -> String {
    format!(
        "  - key: {key}\n    creature: {{id: {creature}}}\n    side: {side}\n    controller: {{kind: manual, role: {role}}}\n"
    )
}

fn write_encounters(root: &Path, party: &[EntitySnapshot]) -> Result<(), String> {
    let heroes: String = party
        .iter()
        .map(|hero| {
            slot(
                &format!("actor_hero_{}", hero.source.character_id),
                hero.definition.splat_id,
                "party",
                "player",
            )
        })
        .collect();
    for (file, name, enemies, note) in [
        (
            "Encounters/Bridge Ambush.md",
            "Bridge Ambush",
            vec![
                slot(
                    "actor_wolf_1",
                    module_entry_id("cinder_wolf"),
                    "enemies",
                    "gm",
                ),
                slot(
                    "actor_wolf_2",
                    module_entry_id("cinder_wolf"),
                    "enemies",
                    "gm",
                ),
                slot(
                    "actor_cultist",
                    module_entry_id("ember_cultist"),
                    "enemies",
                    "gm",
                ),
            ],
            "Wolves hold the far bank while the cultist burns the ropes.",
        ),
        (
            "Encounters/Shrine of Cinders.md",
            "Shrine of Cinders",
            vec![
                slot(
                    "actor_cultist_1",
                    module_entry_id("ember_cultist"),
                    "enemies",
                    "gm",
                ),
                slot(
                    "actor_cultist_2",
                    module_entry_id("ember_cultist"),
                    "enemies",
                    "gm",
                ),
            ],
            "The cultists retreat once the signal fire goes out.",
        ),
    ] {
        write_note(
            root,
            file,
            &format!(
                "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: encounter\nsplat_name: {name}\nsplat_encounter:\n  roster:\n{heroes}{}  sides:\n    party: {{hostile_to: [enemies]}}\n    enemies: {{hostile_to: [party]}}\n  assumptions: []\n  completion: {{mode: manual}}\n  limits: {{rounds: 20, resolution_events: 10000}}\n---\n\n{note}\n",
                stable_id(&format!("visual:encounter:{name}")),
                enemies.concat(),
            ),
        )?;
    }
    Ok(())
}

/// A run with the whole party, and `(count, initiative)` wolves then cultists.
fn write_run(
    campaign: &Campaign,
    module: &Path,
    party: &[EntitySnapshot],
    name: &str,
    enemies: &[(usize, i64)],
) -> Result<(), String> {
    let mut participants = Vec::new();
    for (index, hero) in party.iter().enumerate() {
        participants.push((
            Participant::new(
                format!("actor_hero_{}", hero.source.character_id),
                "party".into(),
                hero.entry()?,
                &Mapping::new(),
                true,
            )?,
            18 - index as i64 * 2,
        ));
    }
    let files = ["Creatures/Cinder Wolf.md", "Creatures/Ember Cultist.md"];
    for (kind, (&(count, total), file)) in enemies.iter().zip(files).enumerate() {
        let text = fs::read_to_string(module.join(file)).map_err(|e| e.to_string())?;
        for index in 0..count {
            participants.push((
                Participant::new(
                    format!("actor_enemy_{kind}_{index}"),
                    "enemies".into(),
                    Entry::parse(file, &text)?,
                    &Mapping::new(),
                    true,
                )?,
                total,
            ));
        }
    }
    write_in_progress_run(
        campaign.id,
        &campaign.path.join("Encounter Runs"),
        name,
        participants,
    )
}

/// The cached sheets a player sees without a connection, and their campaign copies.
fn party(
    campaign_path: &Path,
    cache: &Path,
    campaign: uuid::Uuid,
) -> Result<Vec<EntitySnapshot>, String> {
    let cache = Cache::at(cache.to_path_buf());
    let mut party = Vec::new();
    for hero in HEROES {
        let record = hero.snapshot()?;
        cache.save(campaign, &record)?;
        record.save_campaign_copy(campaign_path)?;
        party.push(record);
    }
    cache.save_roster(
        campaign,
        REMOTE_CAMPAIGN,
        &HEROES.map(|hero| hero.id.to_string()),
    )?;
    Ok(party)
}

struct Hero {
    id: &'static str,
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
    attacks: &'static [(&'static str, i64, &'static str, &'static str)],
    spells: &'static [(&'static str, u64, &'static str)],
    features: &'static [(&'static str, &'static str)],
    items: &'static [(&'static str, u64, bool, &'static str)],
    conditions: &'static [&'static str],
}

const HEROES: [Hero; 5] = [
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
            ("Longsword", 7, "1d8", "Slashing"),
            ("Heavy Crossbow", 4, "1d10", "Piercing"),
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
            ("Dagger", 6, "1d4", "Piercing"),
            ("Fire Bolt", 7, "2d10", "Fire"),
        ],
        spells: &[
            (
                "Fire Bolt",
                0,
                "Hurl a mote of fire at a creature or object.",
            ),
            ("Mage Hand", 0, "A spectral hand appears within range."),
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
            ("Shortsword", 7, "1d6", "Piercing"),
            ("Shortbow", 7, "1d6", "Piercing"),
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
        attacks: &[("Mace", 5, "1d6", "Bludgeoning")],
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
        attacks: &[("Eldritch Blast", 7, "1d10", "Force")],
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
    fn snapshot(&self) -> Result<EntitySnapshot, String> {
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
            .map(|(name, hit, dice, kind)| {
                json!({
                    "name": name,
                    "toHit": hit,
                    "damage": {"value": {"diceString": dice, "fixedValue": modifier(0).max(modifier(1))}, "type": kind},
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
        let raw = json!({
            "id": self.id.parse::<u64>().map_err(|e| e.to_string())?,
            "name": self.name,
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
            "notes": {"otherNotes": ""},
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
