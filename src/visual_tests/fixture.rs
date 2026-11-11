//! Synthetic worlds for the visual suite. Every file lives in a temporary folder,
//! every name is invented, and the only other content is the built-in SRD.
use crate::{
    Splat,
    campaigns::Campaigns,
    content_workspace::{CampaignDocument, EntryRef, PackageSelection, Selection},
    encounter_run::{
        Participant, write_ended_run, write_in_progress_run, write_in_progress_run_with,
    },
    hermetic,
    persistence::{Campaign, Module, Store, UserData},
};
use serde_json::{Value, json};
use serde_yaml_ng::Mapping;
use splat::{
    character::{Cache, EntitySnapshot, stable_id},
    content::{Entry, overrides::Overrides},
};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex, OnceLock},
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
pub const RUN_ARENA: &str = "Arena Final";
pub const RUN_VAULT: &str = "Sunken Vault";
pub const RUN_PAST: &str = "Goblin Crossing";
pub const RUN_OLD: &str = "Old Mill Raid";
pub const PAST_SESSION: &str = "The Road North";
pub const GROUP_RIVALS: &str = "Rival Team";
pub const GROUP_GOBLINS: &str = "Goblin Wave";
pub const GROUP_LOST: &str = "Lost Patrol";
pub const GROUP_WATCH: &str = "Watch Detail";
pub const GROUP_SHRINE: &str = "Shrine Reserve";
pub const GROUP_ARENA: &str = "Arena Beasts";
pub const GROUP_ORPHAN: &str = "Cellar Rats";

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
    /// `Campaign`, plus two encounter groups, and an encounter and a run with overrides.
    Groups,
    /// `Campaign`, plus plain world notes with links, headings and a roll table.
    World,
    /// World, with explicit old-layout conversion files and one malformed property.
    Formats,
    /// An old Timeline for conversion, plus schema-2 lists with an invalid cell.
    ListFormats,
    /// `Campaign`, plus ended runs: one on the timeline under a session, one with no event.
    History,
    /// `Groups`, plus groups that belong to an encounter, and one whose encounter is gone.
    OwnedGroups,
    /// `Campaign`, plus a run with the encounter's loot, a creature at 0 HP that carries loot,
    /// and the roll table its loot names.
    Loot,
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
        name: None,
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
    if matches!(seed, Seed::Groups | Seed::OwnedGroups | Seed::ListFormats) {
        write_groups(&active, &module.path, &party)?;
    }
    if seed == Seed::History {
        write_history(&active, &module.path)?;
    }
    if seed == Seed::OwnedGroups {
        write_owned_groups(&active.path)?;
    }
    if seed == Seed::Loot {
        write_loot(&active, &module.path, &party)?;
    }
    if matches!(seed, Seed::World | Seed::Formats) {
        write_world_notes(&active.path)?;
    }
    if seed == Seed::Formats {
        write_format_notes(&active.path)?;
    }

    seed_list_layouts(&active, &quiet, &unlinked, &module, scenario, seed)?;

    let mut modules = vec![module];
    if seed == Seed::Formats {
        let generated_root = root.join("modules/generated-book");
        let id = stable_id("visual:generated-book");
        write_note(
            &generated_root,
            "Package.md",
            &format!(
                "---\nsplat_schema: 1\nsplat_id: {id}\nsplat_kind: package\nsplat_name: Generated Book\nsplat_namespace: generated_book\nsplat_generated: true\n---\n\n# Generated Book\n"
            ),
        )?;
        write_note(
            &generated_root,
            "Creatures/Book Wolf.md",
            &format!(
                "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: creature\nsplat_key: generated_book:creatures/book_wolf\nsplat_name: Book Wolf\nsplat_creature: {{size: medium, type: beast}}\nsplat_stats: {{max_hp: {{value: 22}}, armor_class: {{value: 13}}}}\n---\n\n# Book Wolf\n\nA creature from an outside converter.\n",
                stable_id("visual:book-wolf")
            ),
        )?;
        old_layout(&generated_root, "Creatures/Book Wolf.md")?;
        old_layout(&modules[0].path, "Creatures/Cinder Wolf.md")?;
        old_layout(&active.path, "NPCs/Rook.md")?;
        old_layout(&active.path, "Notes/Emberfall Rumours.md")?;
        modules.push(Module {
            id,
            name: "Generated Book".into(),
            path: generated_root,
        });
    }
    let data = UserData {
        version: 3,
        active_campaign_id: Some(active.id),
        campaigns: vec![active, quiet, unlinked],
        modules,
        unregistered_modules: Vec::new(),
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
    let text = splat::content::formats::convert(relative, text)?;
    fs::write(path, &text).map_err(|e| e.to_string())?;
    Entry::parse(relative, &text).map(|_| ())
}

/// Conversion scenarios alone keep selected old files on disk.
fn old_layout(root: &Path, relative: &str) -> Result<(), String> {
    let path = root.join(relative);
    let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let entry = Entry::parse(relative, &text)?;
    let body = splat::content::stat_blocks::reading_body(&entry, &entry.body)
        .split("\n> [!note]- Splat data")
        .next()
        .unwrap_or(&entry.body)
        .to_owned();
    let mut fields = (*entry.fields).clone();
    fields.insert("splat_schema".into(), 1.into());
    fs::write(
        path,
        format!(
            "---\n{}---\n{body}",
            serde_yaml_ng::to_string(&fields).map_err(|e| e.to_string())?
        ),
    )
    .map_err(|e| e.to_string())
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
            "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: creature\nsplat_key: ashen_marches:creatures/{slug}\nsplat_name: {name}\n{requires}splat_creature: {{size: medium, type: {identity}, cr_text: '1 (XP 200; PB +2)', movement_text: '30 ft.', hit_points_text: '{hp} (4d8 + 4)'}}\nsplat_stats:\n  speed: {{value: 30, unit: ft}}\n  max_hp: {{value: {hp}}}\n  armor_class: {{value: {ac}}}\n  initiative: {{value: {init}}}\n---\n\n# {name}\n\n## Source text\n\nMedium {identity}, Unaligned\\\nAC {ac}  Initiative +{init} ({})\\\nHP {hp} (4d8 + 4)\\\nSpeed 30 ft.\\\nCR 1 (XP 200; PB +2)\\\n\n### Actions\n\n{prose}\n",
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
            // A full attribute table, so its checks and saves roll.
            creature(
                "cinder_wolf",
                "Cinder Wolf",
                22,
                13,
                2,
                "beast",
                "Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 7 (2d4 + 2) Piercing damage plus 3 (1d6) Fire damage.",
            )
            .replace(
                "  initiative: {value: 2}\n",
                "  initiative: {value: 2}\n  strength: {value: 13}\n  dexterity: {value: 15}\n  constitution: {value: 14}\n  intelligence: {value: 3}\n  wisdom: {value: 12}\n  charisma: {value: 6}\n",
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
                "Scorching Sickle. Melee Attack Roll: +3, reach 5 ft. Hit: 5 (1d6 + 2) Slashing damage, or 9 (2d6 + 2) Slashing damage if the cultist moved 10+ feet straight toward the target immediately before the hit.\n\nEmber Burst. Dexterity Saving Throw: DC 12, each creature in a 10-foot Emanation originating from the cultist. Failure: 7 (2d6) Fire damage. Success: Half damage.\n\nSpellcasting. The cultist casts Ashfall, using Wisdom as the spellcasting ability.",
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
    write_art(root)?;
    Ok(Module {
        id,
        name: MODULE.into(),
        path: root.to_path_buf(),
    })
}

/// A plain picture, drawn here, so that the scenes use no published art.
fn picture_bytes(width: u32, height: u32, color: [u8; 3]) -> Result<Vec<u8>, String> {
    let (cx, cy) = (width as f32 / 2.0, height as f32 * 0.4);
    let radius = width.min(height) as f32 * 0.32;
    let image = image::RgbImage::from_fn(width, height, |x, y| {
        let distance = (x as f32 - cx).hypot(y as f32 - cy);
        let shade = if distance < radius {
            1.0
        } else {
            0.35 + 0.25 * (y as f32 / height as f32)
        };
        image::Rgb(color.map(|part| (part as f32 * shade) as u8))
    });
    let mut bytes = Vec::new();
    image
        .write_to(
            &mut std::io::Cursor::new(&mut bytes),
            image::ImageFormat::Png,
        )
        .map_err(|e| e.to_string())?;
    Ok(bytes)
}

/// Cache only immutable encoded bytes, keyed by every input to the generator.
/// Each scenario still writes and owns its own module art and portrait files.
fn cached_picture_bytes(width: u32, height: u32, color: [u8; 3]) -> Result<Arc<[u8]>, String> {
    type Pictures = std::collections::BTreeMap<(u32, u32, [u8; 3]), Arc<[u8]>>;
    static PICTURES: OnceLock<Mutex<Pictures>> = OnceLock::new();
    let mut pictures = PICTURES
        .get_or_init(Mutex::default)
        .lock()
        .map_err(|e| e.to_string())?;
    let key = (width, height, color);
    if let Some(bytes) = pictures.get(&key) {
        return Ok(Arc::clone(bytes));
    }
    let bytes: Arc<[u8]> = picture_bytes(width, height, color)?.into();
    pictures.insert(key, Arc::clone(&bytes));
    Ok(bytes)
}

/// Art for the module: one creature of its own, and a default picture for one creature type.
fn write_art(root: &Path) -> Result<(), String> {
    let art = root.join("Art");
    fs::create_dir_all(&art).map_err(|e| e.to_string())?;
    let wolf = cached_picture_bytes(480, 640, [190, 96, 52])?;
    let humanoid = cached_picture_bytes(320, 320, [84, 118, 170])?;
    fs::write(art.join("cinder-wolf.png"), wolf).map_err(|e| e.to_string())?;
    fs::write(art.join("type-humanoid.png"), humanoid).map_err(|e| e.to_string())?;
    let index = json!({
        "splat_art": 1,
        "entries": {"ashen_marches:creatures/cinder_wolf": "Art/cinder-wolf.png"},
        "creature_types": {"humanoid": "Art/type-humanoid.png"},
    });
    fs::write(
        root.join(splat::content::art::ART_FILE),
        format!("# Art Index\n\n```json\n{index}\n```\n"),
    )
    .map_err(|e| e.to_string())
}

/// Plain notes, as a DM writes them in Obsidian, with no Splat fields.
fn write_world_notes(root: &Path) -> Result<(), String> {
    for (path, text) in [
        (
            "Setting.md",
            "# Setting\n\nA cold war around an inner sea.\n\n[[Verori|The home country]]\n",
        ),
        (
            "Powers/Verori.md",
            "# Verori\n\nNo ruler, only a law code.\n",
        ),
        (
            "Gate Mechanics.md",
            "# Gate Mechanics\n\n## Forced trips\n\nEvery forced trip rolls here.\n\n\
             <!-- splat:rolls key=forced_trip dice=1d20 -->\n\n\
             | d20 | Result |\n| --- | --- |\n| 1-5 | No connection. |\n\
             | 6-10 | Rough arrival. |\n| 11-20 | Clean arrival. |\n\n\
             ## Reliability\n\nA field gate fails on a low roll.\n",
        ),
    ] {
        let path = root.join(path);
        fs::create_dir_all(path.parent().ok_or("A note needs a folder.")?)
            .map_err(|e| e.to_string())?;
        fs::write(path, text).map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn write_format_notes(root: &Path) -> Result<(), String> {
    let old = format!(
        "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: creature\nsplat_name: Slate Wolf\nsplat_creature: {{size: medium, type: beast, alignment: Unaligned, hit_points_text: '22 (4d8 + 4)', cr_text: '1 (XP 200; PB +2)'}}\nsplat_stats: {{max_hp: {{value: 22}}, armor_class: {{value: 13}}, initiative: {{value: 2}}, speed: {{value: 40, unit: ft}}, strength: {{value: 12}}, dexterity: {{value: 15}}}}\n---\n\n# Slate Wolf\n\n## Actions\n\nBite. Melee Attack Roll: +4, reach 5 ft. Hit: 7 Piercing damage.\n",
        stable_id("slate-wolf")
    );
    let new = splat::content::formats::convert("Creatures/Slate Wolf.md", &old)?;
    write_note(root, "Creatures/Slate Wolf.md", &new)?;
    let bad = new
        .replace(
            &stable_id("slate-wolf").to_string(),
            &stable_id("flagged-wolf").to_string(),
        )
        .replace("Slate Wolf", "Flagged Wolf")
        .replace(
            "| Hit Points | 22 (4d8 + 4) |",
            "| Hit Points | about twenty |",
        );
    write_note(root, "Creatures/Flagged Wolf.md", &bad)?;
    let spell = format!(
        "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: spell\nsplat_name: Flagged Spark\nsplat_spell: {{level: 1, range_text: Self}}\n---\n\n# Flagged Spark\n\nKeep the spark effect.\n",
        stable_id("flagged-spark")
    );
    let spell = splat::content::formats::convert("Spells/Flagged Spark.md", &spell)?
        .replace("| Level | 1 |", "| Level | many |");
    write_note(root, "Spells/Flagged Spark.md", &spell)?;
    let item = format!(
        "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: item\nsplat_name: Bound Charm\nsplat_item: {{category: wondrous_item, rarity: uncommon, equipment: {{attunement: required}}}}\n---\n\n# Bound Charm\n\nKeep the charm effect.\n",
        stable_id("bound-charm")
    );
    write_note(
        root,
        "Items/Bound Charm.md",
        &splat::content::formats::convert("Items/Bound Charm.md", &item)?,
    )?;
    for (path, schema, name, data) in [
        (
            "NPCs/Rook.md",
            1,
            "Rook",
            "splat_world: {type: npc, status: alive, aliases: [The Veteran]}",
        ),
        (
            "Locations/Undercroft.md",
            2,
            "Undercroft",
            "splat_type: location\naliases: [The Cellar]",
        ),
        (
            "NPCs/Malformed.md",
            2,
            "Malformed",
            "splat_type: npc\nsplat_status: [alive]",
        ),
    ] {
        write_note(
            root,
            path,
            &format!(
                "---\nsplat_schema: {schema}\nsplat_id: {}\nsplat_kind: world\nsplat_name: {name}\n{data}\n---\n\n# {name}\n\nA note for tonight.\n",
                stable_id(path)
            ),
        )?;
    }
    Ok(())
}

fn write_campaign_notes(root: &Path) -> Result<(), String> {
    // Native creatures use the current template layout. Format scenarios cover
    // legacy conversion; ordinary saves keep testing a stable editor.
    write_note(
        root,
        "Notes/Captain Varra.md",
        &splat::content::formats::convert(
            "Notes/Captain Varra.md",
            &format!(
                "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: creature\nsplat_name: Captain Varra\nsplat_creature: {{size: medium, type: humanoid}}\nsplat_stats:\n  max_hp: {{value: 52}}\n  armor_class: {{value: 16}}\n  initiative: {{value: 2}}\n---\n\n# Captain Varra\n\nThe last commander of the Emberfall watch. She trusts the party until they side with the cult.\n\n## Actions\n\nLongsword. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3) Slashing damage.\n",
                stable_id("visual:note:varra"),
            ),
        )?,
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

/// Groups that belong to one encounter, and one whose encounter this campaign no longer has.
fn write_owned_groups(root: &Path) -> Result<(), String> {
    for (name, encounter, creature) in [
        (
            GROUP_SHRINE,
            stable_id("visual:encounter:Shrine of Cinders"),
            "{key: 'dnd2024:creatures/goblin_warrior'}".to_owned(),
        ),
        (
            GROUP_ARENA,
            stable_id("visual:encounter:arena"),
            format!("{{id: {}}}", module_entry_id("cinder_wolf")),
        ),
        (
            GROUP_ORPHAN,
            stable_id("visual:encounter:gone"),
            format!("{{id: {}}}", module_entry_id("cinder_wolf")),
        ),
    ] {
        write_note(
            root,
            &format!("Encounter Groups/{name}.md"),
            &format!(
                "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: encounter_group\nsplat_name: {name}\nsplat_encounter_group:\n  side: enemies\n  encounter: {{id: {encounter}}}\n  members:\n  - creature: {creature}\n    count: 2\n---\n",
                stable_id(&format!("visual:group:{name}")),
            ),
        )?;
    }
    Ok(())
}

/// Two groups, and the same changes built in to an encounter's slot and to a run in progress.
fn write_groups(
    campaign: &Campaign,
    module: &Path,
    party: &[EntitySnapshot],
) -> Result<(), String> {
    const RIVAL_CHANGES: &str = "{name: Rival Captain, max_hp: {add: 10}, armor_class: {set: 15}, attack_bonus: {add: 1}, damage_bonus: {add: 1}, speed: {set: 40}, conditions: [Blessed], note: Gained a seal in the last round}";
    let root = &campaign.path;
    write_note(
        root,
        &format!("Encounter Groups/{GROUP_RIVALS}.md"),
        &format!(
            "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: encounter_group\nsplat_name: {GROUP_RIVALS}\nsplat_encounter_group:\n  side: rivals\n  members:\n  - creature: {{id: {}}}\n    count: 1\n    overrides: {RIVAL_CHANGES}\n  - creature: {{id: {}}}\n    count: 2\n---\n\nThey come through the east gate at the horn.\n",
            stable_id("visual:group:rivals"),
            module_entry_id("ember_cultist"),
            module_entry_id("cinder_wolf"),
        ),
    )?;
    write_note(
        root,
        &format!("Encounter Groups/{GROUP_GOBLINS}.md"),
        &format!(
            "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: encounter_group\nsplat_name: {GROUP_GOBLINS}\nsplat_encounter_group:\n  side: enemies\n  members:\n  - creature: {{key: 'dnd2024:creatures/goblin_warrior'}}\n    count: 3\n---\n",
            stable_id("visual:group:goblins"),
        ),
    )?;
    // A group that cannot be added: a creature gone, and a change this one cannot take.
    write_note(
        root,
        &format!("Encounter Groups/{GROUP_LOST}.md"),
        &format!(
            "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: encounter_group\nsplat_name: {GROUP_LOST}\nsplat_encounter_group:\n  side: enemies\n  members:\n  - creature: {{id: {}}}\n  - creature: {{id: {}}}\n    overrides: {{max_hp: {{add: -50}}}}\n  - creature: {{key: 'dnd2024:creatures/goblin_warrior'}}\n---\n",
            stable_id("visual:group:lost"),
            stable_id("visual:group:lost:creature"),
            module_entry_id("cinder_wolf"),
        ),
    )?;
    // A creature whose text gives no XP, which must not count as none.
    write_note(
        root,
        &format!("Encounter Groups/{GROUP_WATCH}.md"),
        &format!(
            "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: encounter_group\nsplat_name: {GROUP_WATCH}\nsplat_encounter_group:\n  side: party\n  members:\n  - creature: {{id: {}}}\n---\n",
            stable_id("visual:group:watch"),
            stable_id("visual:note:varra"),
        ),
    )?;
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
    let rival = slot(
        "actor_rival",
        module_entry_id("ember_cultist"),
        "enemies",
        "gm",
    );
    write_note(
        root,
        &format!("Encounters/{RUN_ARENA}.md"),
        &format!(
            "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: encounter\nsplat_name: {RUN_ARENA}\nsplat_encounter:\n  roster:\n{heroes}{rival}    overrides: {RIVAL_CHANGES}\n  sides:\n    party: {{hostile_to: [enemies]}}\n    enemies: {{hostile_to: [party]}}\n  assumptions: []\n  completion: {{mode: manual}}\n  limits: {{rounds: 20, resolution_events: 10000}}\n---\n\nThe last fight of the season.\n",
            stable_id("visual:encounter:arena"),
        ),
    )?;
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
    let file = "Creatures/Ember Cultist.md";
    let text = fs::read_to_string(module.join(file)).map_err(|e| e.to_string())?;
    let changes: Mapping = serde_yaml_ng::from_str(RIVAL_CHANGES).map_err(|e| e.to_string())?;
    participants.push((
        Participant::with_overrides(
            "actor_rival".into(),
            "enemies".into(),
            Entry::parse(file, &text)?,
            &Mapping::new(),
            &Overrides::parse(&changes)?,
            true,
        )?,
        15,
    ));
    write_in_progress_run(
        campaign.id,
        &root.join("Encounter Runs"),
        RUN_ARENA,
        participants,
    )
}

/// A run in progress whose encounter has loot, with a guard at 0 HP that carries some, and the
/// campaign table both roll on.
fn write_loot(campaign: &Campaign, module: &Path, party: &[EntitySnapshot]) -> Result<(), String> {
    // A plain world note, as Obsidian writes one.
    fs::write(
        campaign.path.join("Treasure.md"),
        "# Treasure\n\n<!-- splat:rolls key=gems dice=1d4 -->\n\n| d4 | Gem |\n| --- | --- |\n| 1 | A cracked garnet (10 gp) |\n| 2 | A river pearl (50 gp) |\n| 3 | A smoky quartz (50 gp) |\n| 4 | A star ruby (100 gp) |\n",
    )
    .map_err(|e| e.to_string())?;
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
    let file = "Creatures/Ember Cultist.md";
    let text = fs::read_to_string(module.join(file)).map_err(|e| e.to_string())?;
    let guard: Mapping = serde_yaml_ng::from_str("{name: Vault Guard, loot: Vault key; Roll gems}")
        .map_err(|e| e.to_string())?;
    let down: Mapping = serde_yaml_ng::from_str("{hp: 0}").map_err(|e| e.to_string())?;
    participants.push((
        Participant::with_overrides(
            "actor_guard".into(),
            "enemies".into(),
            Entry::parse(file, &text)?,
            &down,
            &Overrides::parse(&guard)?,
            true,
        )?,
        15,
    ));
    let file = "Creatures/Cinder Wolf.md";
    let text = fs::read_to_string(module.join(file)).map_err(|e| e.to_string())?;
    participants.push((
        Participant::new(
            "actor_hound".into(),
            "enemies".into(),
            Entry::parse(file, &text)?,
            &Mapping::new(),
            true,
        )?,
        9,
    ));
    write_in_progress_run_with(
        campaign.id,
        &campaign.path.join("Encounter Runs"),
        RUN_VAULT,
        participants,
        &format!(
            "# {RUN_VAULT}\n\nWater to the knees.\n\n## Loot\n\n- 120 gp in a locked coffer\n- Roll 2× gems\n"
        ),
    )
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
                    {
                        let mut entry = Entry::parse(file, &text)?;
                        entry.fields.insert(
                            "splat_run_source_path".into(),
                            module.join(file).to_string_lossy().to_string().into(),
                        );
                        entry
                            .fields
                            .insert("splat_run_source_owner".into(), MODULE.into());
                        entry
                    },
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

/// An encounter a run has ended, its event on the timeline under a session, and an ended run
/// from before the timeline, whose encounter is gone.
fn write_history(campaign: &Campaign, module: &Path) -> Result<(), String> {
    let id = stable_id(&format!("visual:encounter:{RUN_PAST}"));
    let root = &campaign.path;
    write_note(
        root,
        &format!("Encounters/{RUN_PAST}.md"),
        &format!(
            "---\nsplat_schema: 1\nsplat_id: {id}\nsplat_kind: encounter\nsplat_name: {RUN_PAST}\nsplat_encounter:\n  roster: []\n  sides:\n    party: {{hostile_to: [enemies]}}\n    enemies: {{hostile_to: [party]}}\n  assumptions: []\n  completion: {{mode: manual}}\n  limits: {{rounds: 20, resolution_events: 10000}}\n---\n\nThey held the ford.\n"
        ),
    )?;
    let file = "Creatures/Cinder Wolf.md";
    let text = fs::read_to_string(module.join(file)).map_err(|e| e.to_string())?;
    let wolf = |key: &str| {
        Participant::new(
            key.into(),
            "enemies".into(),
            Entry::parse(file, &text)?,
            &Mapping::new(),
            true,
        )
    };
    let directory = root.join("Encounter Runs");
    let past = write_ended_run(
        campaign.id,
        &directory,
        RUN_PAST,
        Some(id),
        "Party won",
        wolf("actor_past")?,
    )?;
    write_ended_run(
        campaign.id,
        &directory,
        RUN_OLD,
        None,
        "Party fled",
        wolf("actor_old")?,
    )?;
    let session = stable_id("visual:session:past");
    // A session note is not a content entry, so `write_note` would refuse it.
    let sessions = crate::sessions::folder(campaign)?;
    fs::create_dir_all(&sessions).map_err(|e| e.to_string())?;
    fs::write(
        sessions.join("Session 7.md"),
        format!(
            "---\nsession_id: {session}\nsession_number: 7\nsession_title: {PAST_SESSION}\nsession_date: 2026-01-01\n---\n\nNotes.\n"
        ),
    )
    .map_err(|e| e.to_string())?;
    let mut book = crate::timeline::Book::default();
    book.open(Some(&sessions));
    book.record_run(&past, Some(session), false);
    book.save();
    Ok(())
}

/// One party member's sheet, for a test that needs a sheet and not a whole campaign.
pub(crate) fn sample_sheet() -> EntitySnapshot {
    HEROES[0].snapshot().expect("The first hero makes a sheet")
}

/// The cached sheets a player sees without a connection, and their campaign copies.
fn party(
    campaign_path: &Path,
    cache: &Path,
    campaign: uuid::Uuid,
) -> Result<Vec<EntitySnapshot>, String> {
    let cache = Cache::at(cache.to_path_buf());
    // This computer has saved these sheets for a while, so its turn has settled. The test
    // clock does not move, so a new claim would never settle.
    let (device, name) = crate::hermetic::device();
    let now = crate::hermetic::now();
    crate::sheet_writer::hold(
        campaign_path,
        &crate::sheet_writer::Holder {
            device,
            name,
            since: now - 120,
            until: now + 300,
        },
    );
    let mut party = Vec::new();
    for hero in HEROES {
        let record = hero.snapshot()?;
        cache.save(campaign, &record)?;
        if let Some(url) = record.portrait_url() {
            save_portrait(&cache, campaign, &hero, url)?;
        }
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

/// Save a hero's portrait as the cache keeps it. The cache resizes each picture; doing that
/// once, not in every scenario, saves about a tenth of the suite's fixture time.
fn save_portrait(
    cache: &Cache,
    campaign: uuid::Uuid,
    hero: &Hero,
    url: &str,
) -> Result<(), String> {
    type Stored = std::collections::BTreeMap<&'static str, Arc<[u8]>>;
    static STORED: OnceLock<Mutex<Stored>> = OnceLock::new();
    let stored = STORED.get_or_init(Mutex::default);
    let path = cache.portrait_path(campaign, REMOTE_CAMPAIGN, hero.id, url)?;
    let known = stored
        .lock()
        .map_err(|e| e.to_string())?
        .get(hero.id)
        .cloned();
    if let Some(bytes) = known {
        let parent = path.parent().ok_or("A portrait path has no folder.")?;
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        return fs::write(&path, bytes).map_err(|e| e.to_string());
    }
    let picture = cached_picture_bytes(320, 320, hero.portrait_color())?;
    cache.save_portrait(campaign, REMOTE_CAMPAIGN, hero.id, url, &picture)?;
    let bytes: Arc<[u8]> = fs::read(&path).map_err(|e| e.to_string())?.into();
    stored
        .lock()
        .map_err(|e| e.to_string())?
        .insert(hero.id, bytes);
    Ok(())
}

impl Hero {
    fn portrait_color(&self) -> [u8; 3] {
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

#[test]
fn cached_art_is_byte_identical_and_world_files_remain_independent() {
    let first = tempfile::tempdir().unwrap();
    let second = tempfile::tempdir().unwrap();
    write_art(first.path()).unwrap();
    let wolf = picture_bytes(480, 640, [190, 96, 52]).unwrap();
    let humanoid = picture_bytes(320, 320, [84, 118, 170]).unwrap();
    assert_eq!(
        fs::read(first.path().join("Art/cinder-wolf.png")).unwrap(),
        wolf
    );
    assert_eq!(
        fs::read(first.path().join("Art/type-humanoid.png")).unwrap(),
        humanoid
    );
    for hero in HEROES {
        let color = hero.portrait_color();
        assert_eq!(
            cached_picture_bytes(320, 320, color).unwrap().as_ref(),
            picture_bytes(320, 320, color).unwrap()
        );
    }
    fs::write(
        first.path().join("Art/cinder-wolf.png"),
        b"edited by one scenario",
    )
    .unwrap();
    write_art(second.path()).unwrap();
    assert_eq!(
        fs::read(second.path().join("Art/cinder-wolf.png")).unwrap(),
        wolf
    );
    assert_eq!(
        fs::read(second.path().join("Art/type-humanoid.png")).unwrap(),
        humanoid
    );
    assert_eq!(
        fs::read(first.path().join("Art/cinder-wolf.png")).unwrap(),
        b"edited by one scenario"
    );
}

/// Seed explicit conversion fixtures and normal schema-2 lists.
fn seed_list_layouts(
    active: &Campaign,
    quiet: &Campaign,
    unlinked: &Campaign,
    _module: &Module,
    _scenario: &str,
    seed: Seed,
) -> Result<(), String> {
    // Conversion scenarios retain one old campaign for preview and Undo.
    if seed == Seed::Formats {
        let campaign = unlinked;
        let path = campaign.path.join("Campaign.md");
        let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let entry = Entry::parse("Campaign.md", &text)?;
        let mut fields = (*entry.fields).clone();
        fields.insert("splat_schema".into(), 1.into());
        fs::write(
            path,
            format!(
                "---\n{}---\n\n# {}\n",
                serde_yaml_ng::to_string(&fields).map_err(|e| e.to_string())?,
                entry.name
            ),
        )
        .map_err(|e| e.to_string())?;
    }
    // Quiet campaigns also use version 2 timelines.
    let quiet_folder = crate::sessions::open_folder(quiet)?;
    fs::write(
        quiet_folder.join(crate::timeline::FILE),
        crate::timeline::formats::render(&[], &Default::default()),
    )
    .map_err(|e| e.to_string())?;
    if seed == Seed::ListFormats {
        let folder = crate::sessions::open_folder(active)?;
        fs::write(folder.join(crate::timeline::FILE),"---\ntimeline_version: 1\ntimeline:\n- id: t-f3-day\n  kind: day\n  title: The crossing\n- id: t-f3-road\n  kind: travel\n  title: Old bridge\n  duration: 10 minutes\n  notes: The party argued about the toll.\n---\n\n# Timeline\n").map_err(|e|e.to_string())?;
        let text = format!(
            "---\nsplat_schema: 2\nsplat_id: {}\nsplat_kind: encounter_group\nsplat_name: Bridge guards\nsplat_side: enemies\n---\n\nThe guards demand a toll.\n\n## Members\n\n| Count | Creature | Name | Changes | Note |\n| --- | --- | --- | --- | --- |\n| 1 | [Goblin](splat:dnd2024:creatures/goblin_warrior) | Scout | HP lots | Keep this row |\n| 2 | [Goblin](splat:dnd2024:creatures/goblin_warrior) | | | |\n",
            stable_id("visual:f3:guards")
        );
        write_note(&active.path, "Encounter Groups/Bridge guards.md", &text)?;
    }
    Ok(())
}
