//! Who a character is, from the sheet: background, details, characteristics, notes, coins,
//! companions, custom items and custom actions. No rule reads these.
//!
//! They come from the private cache's character v5 record, so an old cache gains them when it is
//! next published, with no new read.
use super::content::named_key;
use super::*;
use crate::content::character::{
    Background, Characteristics, Coins, Companion, CustomItem, Details, Feature, Gap, Sheet, Story,
};
use crate::content::{body_text, inline_text};

// The names behind D&D Beyond's numeric IDs, from its `api/config/json`, read on 2026-11-09.
const ALIGNMENTS: [(u64, &str); 10] = [
    (1, "Lawful Good"),
    (2, "Neutral Good"),
    (3, "Chaotic Good"),
    (4, "Lawful Neutral"),
    (5, "Neutral"),
    (6, "Chaotic Neutral"),
    (7, "Lawful Evil"),
    (8, "Neutral Evil"),
    (9, "Chaotic Evil"),
    (10, "Unaligned"),
];
/// There is no ID 2.
const LIFESTYLES: [(u64, &str); 7] = [
    (1, "Wretched"),
    (3, "Squalid"),
    (4, "Poor"),
    (5, "Modest"),
    (6, "Comfortable"),
    (7, "Wealthy"),
    (8, "Aristocratic"),
];
const CREATURE_GROUPS: [(u64, &str); 11] = [
    (1, "Wild Shape (2014)"),
    (2, "Familiar"),
    (3, "Beast Companion (2014)"),
    (4, "Mount"),
    (5, "Pet"),
    (6, "Summoned"),
    (7, "Misc"),
    (10, "Battle Smith Defender"),
    (11, "Sidekick"),
    (12, "Infusion"),
    (13, "Wild Shape (2024)"),
];

/// Everything on the sheet about who the character is, ready for the campaign note and the viewer.
#[derive(Clone, Debug, Default)]
pub struct Profile {
    pub background: Option<Background>,
    /// The background's own feature, keyed as `background.feature` names it.
    pub background_feature: Option<(String, Feature)>,
    pub details: Details,
    pub characteristics: Characteristics,
    pub story: Story,
    pub coins: Option<Coins>,
    pub companions: BTreeMap<String, Companion>,
    pub custom_items: BTreeMap<String, CustomItem>,
    pub custom_actions: BTreeMap<String, Feature>,
    /// Source values that Splat cannot name. The source value stays in the cache.
    pub gaps: Vec<Gap>,
}

/// Text the player wrote, or `None` when the field is empty.
fn words(value: &Value) -> Option<String> {
    let text = value.as_str()?.replace("\r\n", "\n").replace('\r', "\n");
    let text = text.trim();
    (!text.is_empty()).then(|| text.to_owned())
}
/// A name the player wrote, on one line.
fn one_line(value: &Value) -> Option<String> {
    words(value).map(|text| text.split_whitespace().collect::<Vec<_>>().join(" "))
}
/// Readable text from provider HTML, or `None` when there is none.
fn prose(value: &Value) -> Option<String> {
    let text = super::content::prose(value);
    (!text.trim().is_empty()).then_some(text)
}
/// A number or words, as the sheet shows it.
fn shown(value: &Value, unit: &str) -> Option<String> {
    match value {
        Value::Number(n) if n.as_f64().is_some_and(|n| n > 0.0) => Some(format!("{n}{unit}")),
        _ => words(value),
    }
}
/// The name of a D&D Beyond ID. An ID that Splat cannot name is a gap.
fn named(
    table: &[(u64, &str)],
    id: Option<u64>,
    path: &str,
    what: &str,
    gaps: &mut Vec<Gap>,
) -> Option<String> {
    let id = id.filter(|id| *id > 0)?;
    let name = table.iter().find(|(known, _)| *known == id);
    if name.is_none() {
        gaps.push(Gap {
            path: path.into(),
            reason: format!("{what} ID {id} is not one Splat can name; the source value is kept."),
        });
    }
    name.map(|(_, name)| (*name).to_owned())
}
/// The present values, with their labels, in order.
fn present<T>(rows: impl IntoIterator<Item = (&'static str, Option<T>)>) -> Vec<(&'static str, T)> {
    rows.into_iter()
        .filter_map(|(label, value)| Some((label, value?)))
        .collect()
}

impl EntitySnapshot {
    pub fn profile(&self) -> Profile {
        let raw = &self.source.raw;
        let (background, background_feature) = background(raw);
        let mut profile = Profile {
            background,
            background_feature,
            coins: coins(raw),
            ..Profile::default()
        };
        let gaps = &mut profile.gaps;
        let lifestyle = words(&raw["lifestyle"]["name"]).or_else(|| {
            let id = raw["lifestyleId"].as_u64();
            named(
                &LIFESTYLES,
                id,
                "splat_character/details/lifestyle",
                "Lifestyle",
                gaps,
            )
        });
        let alignment = named(
            &ALIGNMENTS,
            raw["alignmentId"].as_u64(),
            "splat_character/details/alignment",
            "Alignment",
            gaps,
        );
        profile.details = Details {
            player: words(&raw["username"]),
            alignment,
            faith: words(&raw["faith"]),
            lifestyle,
            gender: words(&raw["gender"]),
            age: shown(&raw["age"], ""),
            height: words(&raw["height"]),
            weight: shown(&raw["weight"], " lb"),
            eyes: words(&raw["eyes"]),
            hair: words(&raw["hair"]),
            skin: words(&raw["skin"]),
            experience: raw["currentXp"].as_u64().filter(|xp| *xp > 0),
        };
        let traits = &raw["traits"];
        profile.characteristics = Characteristics {
            personality_traits: words(&traits["personalityTraits"]),
            ideals: words(&traits["ideals"]),
            bonds: words(&traits["bonds"]),
            flaws: words(&traits["flaws"]),
            appearance: words(&traits["appearance"]),
        };
        let notes = &raw["notes"];
        profile.story = Story {
            backstory: words(&notes["backstory"]),
            allies: words(&notes["allies"]),
            enemies: words(&notes["enemies"]),
            organizations: words(&notes["organizations"]),
            personal_possessions: words(&notes["personalPossessions"]),
            other_holdings: words(&notes["otherHoldings"]),
            other_notes: words(&notes["otherNotes"]),
        };
        for creature in list(&raw["creatures"]) {
            let d = &creature["definition"];
            let kind = one_line(&d["name"]).unwrap_or_default();
            let Some(name) = one_line(&creature["name"])
                .or_else(|| Some(kind.clone()).filter(|k| !k.is_empty()))
            else {
                continue;
            };
            let group = named(
                &CREATURE_GROUPS,
                creature["groupId"].as_u64(),
                "splat_character/companions",
                &format!("{name}: creature group"),
                &mut profile.gaps,
            );
            let key = unique(&profile.companions, named_key("companion", creature));
            profile.companions.insert(
                key,
                Companion {
                    name,
                    creature: kind,
                    group,
                    active: creature["isActive"] == true,
                    description: prose(&creature["description"]).unwrap_or_default(),
                    armor_class: d["armorClass"].as_i64(),
                    hit_points: d["averageHitPoints"].as_i64(),
                    damage_taken: creature["removedHitPoints"].as_i64(),
                    temporary_hp: creature["temporaryHitPoints"].as_i64(),
                    source_id: id(&creature["id"]).unwrap_or_default(),
                },
            );
        }
        for item in list(&raw["customItems"]) {
            let Some(name) = one_line(&item["name"]) else {
                continue;
            };
            let key = unique(&profile.custom_items, named_key("custom_item", item));
            profile.custom_items.insert(
                key,
                CustomItem {
                    name,
                    quantity: item["quantity"].as_u64().unwrap_or(1),
                    weight_lb: item["weight"].as_f64().filter(|w| *w > 0.0),
                    cost: shown(&item["cost"], ""),
                    description: prose(&item["description"]).unwrap_or_default(),
                    notes: words(&item["notes"]).unwrap_or_default(),
                    source_id: id(&item["id"]).unwrap_or_default(),
                },
            );
        }
        for action in list(&raw["customActions"]) {
            let Some(name) = one_line(&action["name"]) else {
                continue;
            };
            let key = unique(&profile.custom_actions, named_key("custom_action", action));
            profile.custom_actions.insert(
                key,
                Feature {
                    name,
                    description: prose(&action["description"])
                        .or_else(|| prose(&action["snippet"]))
                        .unwrap_or_default(),
                    activation: super::content::activation(&action["activation"])
                        .or_else(|| super::content::activation(action)),
                    resource: None,
                    source_key: id(&action["id"]).unwrap_or_default(),
                },
            );
        }
        profile
    }
}

/// The sheet's coins, or `None` when it has none.
pub fn coins(raw: &Value) -> Option<Coins> {
    let coins = &raw["currencies"];
    let coin = |name: &str| coins[name].as_u64().unwrap_or(0);
    Some(Coins {
        cp: coin("cp"),
        sp: coin("sp"),
        ep: coin("ep"),
        gp: coin("gp"),
        pp: coin("pp"),
    })
    .filter(|coins| *coins != Coins::default())
}

/// The background, and its own feature keyed as `Background::feature` names it.
pub fn background(raw: &Value) -> (Option<Background>, Option<(String, Feature)>) {
    let b = &raw["background"];
    let custom = b["hasCustomBackground"] == true;
    let d = if custom {
        &b["customBackground"]
    } else {
        &b["definition"]
    };
    if !d.is_object() {
        return (None, None);
    }
    let Some(name) =
        one_line(&d["name"]).or_else(|| custom.then(|| "Custom background".to_owned()))
    else {
        return (None, None);
    };
    // A custom background can take its feature from a published one.
    let features = if custom { &d["featuresBackground"] } else { d };
    let granted = list(&features["grantedFeats"]);
    let feats = list(&raw["feats"])
        .iter()
        .filter(|feat| {
            granted
                .iter()
                .any(|g| g["id"].is_u64() && g["id"] == feat["componentId"])
        })
        .filter_map(|feat| one_line(&feat["definition"]["name"]))
        .collect::<Vec<_>>();
    // A 2024 background's feature is a feat, imported with the other feats.
    let feature = one_line(&features["featureName"])
        .filter(|_| granted.is_empty() && features["featureIsFeat"] != true)
        .map(|feature_name| {
            let identity = string(&features["definitionKey"]);
            let key = format!(
                "background_feature_{}",
                stable_id(&format!("background:{identity}:{feature_name}")).simple()
            );
            (
                key,
                Feature {
                    name: feature_name,
                    description: prose(&features["featureDescription"]).unwrap_or_default(),
                    activation: None,
                    resource: None,
                    source_key: identity.into(),
                },
            )
        });
    let background = Background {
        name,
        custom,
        source_key: words(&d["definitionKey"]),
        description: prose(&d["description"]).unwrap_or_default(),
        feature: feature.as_ref().map(|(key, _)| key.clone()),
        feats,
    };
    (Some(background), feature)
}

impl Profile {
    /// Put these parts into the sheet. The background's feature and custom actions join the
    /// other features, each with its conversion gap.
    pub(super) fn apply(self, sheet: &mut Sheet, gaps: &mut Vec<Gap>) {
        let mut add = |key: String, feature: Feature, what: &str| {
            gaps.push(Gap {
                path: format!("splat_character/features/{key}"),
                reason: format!("{}: {what}; rule conversion is pending.", feature.name),
            });
            sheet.features.insert(key, feature);
        };
        if let Some((key, feature)) = self.background_feature {
            add(key, feature, "background feature");
        }
        for (key, feature) in self.custom_actions {
            add(key, feature, "custom action written on the sheet");
        }
        if !self.companions.is_empty() {
            gaps.push(Gap {
                path: "splat_character/companions".into(),
                reason: "Companion stat blocks are not converted, so companions are not encounter participants.".into(),
            });
        }
        if !self.custom_items.is_empty() {
            gaps.push(Gap {
                path: "splat_character/custom_items".into(),
                reason:
                    "Custom items have no item definition, so they have no inventory placement."
                        .into(),
            });
        }
        gaps.extend(self.gaps);
        sheet.background = self.background;
        sheet.details = self.details;
        sheet.characteristics = self.characteristics;
        sheet.story = self.story;
        sheet.coins = self.coins;
        sheet.companions = self.companions;
        sheet.custom_items = self.custom_items;
    }
}

/// The details as label and value pairs, in the sheet's order.
pub fn detail_rows(d: &Details) -> Vec<(&'static str, String)> {
    present([
        ("Player", d.player.clone()),
        ("Alignment", d.alignment.clone()),
        ("Faith", d.faith.clone()),
        ("Lifestyle", d.lifestyle.clone()),
        ("Gender", d.gender.clone()),
        ("Age", d.age.clone()),
        ("Height", d.height.clone()),
        ("Weight", d.weight.clone()),
        ("Eyes", d.eyes.clone()),
        ("Hair", d.hair.clone()),
        ("Skin", d.skin.clone()),
        ("Experience", d.experience.map(|xp| format!("{xp} XP"))),
    ])
}
pub fn characteristic_rows(c: &Characteristics) -> Vec<(&'static str, &str)> {
    present([
        ("Personality traits", c.personality_traits.as_deref()),
        ("Ideals", c.ideals.as_deref()),
        ("Bonds", c.bonds.as_deref()),
        ("Flaws", c.flaws.as_deref()),
        ("Appearance", c.appearance.as_deref()),
    ])
}
pub fn story_rows(s: &Story) -> Vec<(&'static str, &str)> {
    present([
        ("Backstory", s.backstory.as_deref()),
        ("Allies", s.allies.as_deref()),
        ("Enemies", s.enemies.as_deref()),
        ("Organizations", s.organizations.as_deref()),
        ("Personal possessions", s.personal_possessions.as_deref()),
        ("Other holdings", s.other_holdings.as_deref()),
        ("Other notes", s.other_notes.as_deref()),
    ])
}
pub fn coin_line(coins: &Coins) -> String {
    format!(
        "{} cp · {} sp · {} ep · {} gp · {} pp",
        coins.cp, coins.sp, coins.ep, coins.gp, coins.pp
    )
}
/// The printed stat block values and what the sheet records about a companion now.
pub fn companion_line(c: &Companion) -> String {
    let mut parts = vec![c.creature.clone()];
    parts.extend(c.group.clone());
    parts.push(if c.active { "Active" } else { "Not active" }.into());
    if let Some(ac) = c.armor_class {
        parts.push(format!("AC {ac}"));
    }
    if let Some(hp) = c.hit_points {
        parts.push(format!("stat block HP {hp}"));
    }
    if let Some(taken) = c.damage_taken.filter(|n| *n > 0) {
        parts.push(format!("damage taken {taken}"));
    }
    if let Some(temp) = c.temporary_hp.filter(|n| *n > 0) {
        parts.push(format!("temporary HP {temp}"));
    }
    parts.retain(|p| !p.is_empty());
    parts.join(" · ")
}

/// The note's sections for these parts. `sheet.features` holds the background feature.
pub(super) fn markdown(sheet: &Sheet) -> String {
    let mut body = String::new();
    if let Some(b) = &sheet.background {
        body.push_str(&format!(
            "## Background\n\n**{}**{}\n\n",
            inline_text(&b.name),
            if b.custom { " (custom)" } else { "" }
        ));
        if !b.description.is_empty() {
            body.push_str(&format!("{}\n\n", body_text(&b.description)));
        }
        if let Some(feature) = b.feature.as_ref().and_then(|k| sheet.features.get(k)) {
            body.push_str(&format!(
                "- Feature: {} (see Features)\n",
                inline_text(&feature.name)
            ));
        }
        if !b.feats.is_empty() {
            body.push_str(&format!("- Feats: {}\n", inline_text(&b.feats.join(", "))));
        }
        body.push('\n');
    }
    let details = detail_rows(&sheet.details);
    if !details.is_empty() {
        body.push_str("## Details\n\n");
        for (label, value) in details {
            body.push_str(&format!("- {label}: {}\n", inline_text(&value)));
        }
        body.push('\n');
    }
    for (title, rows) in [
        (
            "Characteristics",
            characteristic_rows(&sheet.characteristics),
        ),
        ("Backstory and notes", story_rows(&sheet.story)),
    ] {
        if rows.is_empty() {
            continue;
        }
        body.push_str(&format!("## {title}\n\n"));
        for (label, value) in rows {
            body.push_str(&format!("### {label}\n\n{}\n\n", body_text(value)));
        }
    }
    if !sheet.companions.is_empty() {
        body.push_str("## Companions\n\n");
        for c in sheet.companions.values() {
            body.push_str(&format!(
                "### {}\n\n{}\n\n",
                inline_text(&c.name),
                inline_text(&companion_line(c))
            ));
            if !c.description.is_empty() {
                body.push_str(&format!("{}\n\n", body_text(&c.description)));
            }
        }
    }
    body
}

/// Coins and custom items, which follow the inventory tables.
pub(super) fn equipment_markdown(sheet: &Sheet) -> String {
    let mut body = String::new();
    if let Some(coins) = &sheet.coins {
        body.push_str(&format!("Coins: {}.\n\n", coin_line(coins)));
    }
    if !sheet.custom_items.is_empty() {
        body.push_str("### Custom items\n\n");
        for item in sheet.custom_items.values() {
            let mut facts = vec![format!("quantity {}", item.quantity)];
            if let Some(weight) = item.weight_lb {
                facts.push(format!("{weight} lb"));
            }
            if let Some(cost) = &item.cost {
                facts.push(format!("cost {cost}"));
            }
            body.push_str(&format!(
                "- **{}** ({})",
                inline_text(&item.name),
                facts.join(", ")
            ));
            for text in [&item.description, &item.notes] {
                if !text.is_empty() {
                    body.push_str(&format!(": {}", inline_text(text)));
                }
            }
            body.push('\n');
        }
        body.push('\n');
    }
    body
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::character::Sheet;

    fn with(patch: impl FnOnce(&mut Value)) -> EntitySnapshot {
        let sample = super::super::tests::sample();
        let mut raw = sample.source.raw.clone();
        patch(&mut raw);
        EntitySnapshot::from_beyond(raw, sample.source.computed.clone(), "456", "123", 10, 11)
            .unwrap()
    }
    fn sheet(record: &EntitySnapshot) -> (Sheet, crate::content::Entry) {
        let entry = record.entry().unwrap();
        let sheet = serde_yaml_ng::from_value(entry.fields["splat_character"].clone()).unwrap();
        (sheet, entry)
    }

    #[test]
    fn a_2014_background_brings_its_feature_details_characteristics_and_notes() {
        let record = with(|raw| {
            raw["background"] = json!({"hasCustomBackground": false, "definition": {
                "id": 10, "definitionKey": "1669830167:10", "name": "Noble",
                "description": "<p>You understand wealth.</p>", "grantedFeats": [],
                "featureIsFeat": false, "featureName": "Position of Privilege",
                "featureDescription": "<p>People think the best of you.</p>"}});
            raw["alignmentId"] = json!(7);
            raw["lifestyleId"] = json!(8);
            raw["faith"] = json!("Kord");
            raw["age"] = json!(46);
            raw["weight"] = json!(115);
            raw["height"] = json!("5'5");
            raw["username"] = json!("player");
            raw["currentXp"] = json!(0);
            raw["traits"] = json!({"personalityTraits": "Proud.\r\nLoud.", "ideals": "Power.", "bonds": null, "flaws": "", "appearance": null});
            raw["notes"] = json!({"backstory": "Exiled.\n\n## Import gaps\n<!-- splat-generated-sha256:x -->", "allies": "An army", "otherNotes": "  "});
            raw["currencies"] = json!({"cp": 8, "sp": 9, "ep": 0, "gp": 49, "pp": 0});
        });
        let (sheet, entry) = sheet(&record);
        let background = sheet.background.as_ref().unwrap();
        assert_eq!(background.name, "Noble");
        assert!(!background.custom);
        assert_eq!(background.source_key.as_deref(), Some("1669830167:10"));
        assert_eq!(background.description, "You understand wealth.");
        let feature = &sheet.features[background.feature.as_ref().unwrap()];
        assert_eq!(feature.name, "Position of Privilege");
        assert_eq!(feature.description, "People think the best of you.");
        assert_eq!(sheet.details.alignment.as_deref(), Some("Lawful Evil"));
        assert_eq!(sheet.details.lifestyle.as_deref(), Some("Aristocratic"));
        assert_eq!(sheet.details.faith.as_deref(), Some("Kord"));
        assert_eq!(sheet.details.age.as_deref(), Some("46"));
        assert_eq!(sheet.details.weight.as_deref(), Some("115 lb"));
        assert_eq!(sheet.details.player.as_deref(), Some("player"));
        assert_eq!(
            sheet.details.experience, None,
            "milestone sheets show no XP"
        );
        assert_eq!(
            sheet.characteristics.personality_traits.as_deref(),
            Some("Proud.\nLoud.")
        );
        assert_eq!(
            sheet.characteristics.flaws, None,
            "an empty field is not a value"
        );
        assert_eq!(sheet.story.allies.as_deref(), Some("An army"));
        assert_eq!(sheet.story.other_notes, None);
        assert_eq!(sheet.coins.as_ref().unwrap().gp, 49);
        // The player's words cannot end the generated part early or add a section.
        assert!(entry.body.contains("\\## Import gaps"));
        assert!(entry.body.contains("&lt;!-- splat-generated-sha256:x -->"));
        assert!(
            entry
                .body
                .contains("Background: Noble · Alignment: Lawful Evil.")
        );
        assert!(
            entry
                .body
                .contains("Coins: 8 cp · 9 sp · 0 ep · 49 gp · 0 pp.")
        );
        let gaps = serde_yaml_ng::to_string(&entry.fields["splat_import"]).unwrap();
        assert!(gaps.contains("Position of Privilege: background feature"));
    }

    #[test]
    fn sheet_text_cannot_change_the_note_structure() {
        let nasty = [
            "~~~",
            "```rust",
            "<pre>",
            "Allies",
            "------",
            "    indented",
            "> [!note]- Splat data",
            "<!-- splat:yaml path=splat_character -->",
            "| a | b |",
            "|---|---|",
            "## Import gaps",
            "-->",
        ]
        .join("\n");
        let record = with(|raw| {
            raw["notes"] = json!({"backstory": nasty, "allies": "~~~"});
            raw["traits"] = json!({"bonds": "<script>"});
            raw["customActions"] =
                json!([{"id": 4, "name": "<!-- splat:x --> ~~~", "snippet": "```"}]);
            raw["creatures"] = json!([{"id": 5, "name": "# Pip", "description": "~~~", "definition": {"name": "<pre>"}}]);
            raw["customItems"] = json!([{"id": 3, "name": "~~~", "notes": "<!-- x"}]);
        });
        let (sheet, entry) = sheet(&record);
        assert_eq!(sheet.story.backstory.as_deref(), Some(nasty.as_str()));
        assert_eq!(sheet.story.allies.as_deref(), Some("~~~"));
        assert_eq!(sheet.characteristics.bonds.as_deref(), Some("<script>"));
        assert_eq!(
            sheet.features["custom_action_4"].name,
            "<!-- splat:x --> ~~~"
        );
        assert_eq!(sheet.companions["companion_5"].name, "# Pip");
        let headings = entry
            .body
            .lines()
            .filter(|line| line.starts_with("## "))
            .collect::<Vec<_>>();
        for expected in [
            "## Backstory and notes",
            "## Companions",
            "## Conversion status",
        ] {
            assert_eq!(
                headings.iter().filter(|h| **h == expected).count(),
                1,
                "{headings:?}"
            );
        }
        assert!(!headings.contains(&"## Import gaps"), "{headings:?}");
    }

    #[test]
    fn a_sheet_with_no_coins_lists_none() {
        let record = with(|raw| {
            raw["currencies"] = json!({"cp": 0, "sp": 0, "ep": 0, "gp": 0, "pp": 0});
        });
        assert_eq!(sheet(&record).0.coins, None);
        assert!(!record.entry().unwrap().body.contains("Coins:"));
    }

    #[test]
    fn a_2024_background_names_its_feats_and_adds_no_second_feature() {
        let record = with(|raw| {
            raw["background"] = json!({"hasCustomBackground": false, "definition": {
                "id": 406477, "definitionKey": "1669830167:406477", "name": "Charlatan",
                "featureName": "Skilled", "featureIsFeat": false,
                "grantedFeats": [{"id": 16311, "featIds": [1], "name": "Skilled"}]}});
            raw["feats"] = json!([
                {"componentId": 16311, "componentTypeId": 67468084, "definition": {"id": 1, "name": "Skilled"}},
                {"componentId": 54574, "componentTypeId": 67468084, "definition": {"id": 2, "name": "Dark Bargain"}},
            ]);
        });
        let (sheet, _) = sheet(&record);
        let background = sheet.background.unwrap();
        assert_eq!(background.feats, ["Skilled"]);
        assert_eq!(background.feature, None);
        assert!(
            !sheet
                .features
                .keys()
                .any(|k| k.starts_with("background_feature_"))
        );
    }

    #[test]
    fn a_custom_background_takes_its_feature_from_the_chosen_one() {
        let record = with(|raw| {
            raw["background"] = json!({"hasCustomBackground": true, "definition": null,
                "customBackground": {"id": 1, "name": "Hedge Witch", "description": "Raised in the fens.",
                    "featuresBackground": {"definitionKey": "1669830167:3", "featureName": "Shelter of the Faithful",
                        "featureDescription": "<p>Temples help you.</p>", "grantedFeats": []}}});
        });
        let (sheet, _) = sheet(&record);
        let background = sheet.background.as_ref().unwrap();
        assert!(background.custom);
        assert_eq!(background.name, "Hedge Witch");
        assert_eq!(
            sheet.features[background.feature.as_ref().unwrap()].name,
            "Shelter of the Faithful"
        );
    }

    #[test]
    fn companions_custom_items_and_custom_actions_are_imported_with_their_gaps() {
        let record = with(|raw| {
            raw["creatures"] = json!([
                {"id": 5, "name": "Pip", "groupId": 2, "isActive": true, "removedHitPoints": 1,
                 "description": "<p>An owl.</p>", "definition": {"name": "Owl", "armorClass": 11, "averageHitPoints": 1}},
                {"id": 6, "name": "", "groupId": 99, "definition": {"name": "Riding Horse"}},
            ]);
            raw["customItems"] = json!([{"id": 3, "name": "Locket\nof Mother", "quantity": 2, "weight": 0.5, "description": "Silver.", "notes": "Never sold."}]);
            raw["customActions"] =
                json!([{"id": 4, "name": "Headbutt", "snippet": "Bonk.", "activationType": 3}]);
            raw["alignmentId"] = json!(99);
        });
        let (sheet, entry) = sheet(&record);
        let pip = &sheet.companions["companion_5"];
        assert_eq!(
            (
                pip.name.as_str(),
                pip.creature.as_str(),
                pip.group.as_deref(),
                pip.active
            ),
            ("Pip", "Owl", Some("Familiar"), true)
        );
        assert_eq!(
            (pip.armor_class, pip.hit_points, pip.damage_taken),
            (Some(11), Some(1), Some(1))
        );
        assert_eq!(pip.description, "An owl.");
        let horse = &sheet.companions["companion_6"];
        assert_eq!(
            (horse.name.as_str(), horse.group.as_deref()),
            ("Riding Horse", None)
        );
        let locket = &sheet.custom_items["custom_item_3"];
        assert_eq!(
            (locket.name.as_str(), locket.quantity, locket.weight_lb),
            ("Locket of Mother", 2, Some(0.5))
        );
        let headbutt = &sheet.features["custom_action_4"];
        assert_eq!(
            (
                headbutt.description.as_str(),
                headbutt.activation.as_deref()
            ),
            ("Bonk.", Some("bonus_action"))
        );
        assert_eq!(sheet.details.alignment, None);
        let gaps = serde_yaml_ng::to_string(&entry.fields["splat_import"]).unwrap();
        for words in [
            "Alignment ID 99",
            "creature group ID 99",
            "Companion stat blocks are not converted",
            "Custom items have no item definition",
            "Headbutt: custom action",
        ] {
            assert!(gaps.contains(words), "{words}");
        }
        assert!(entry.body.contains(
            "### Pip\n\nOwl · Familiar · Active · AC 11 · stat block HP 1 · damage taken 1"
        ));
        assert!(
            entry
                .body
                .contains("- **Locket of Mother** (quantity 2, 0.5 lb): Silver.: Never sold.")
        );
    }

    #[test]
    fn a_sheet_without_these_parts_adds_no_fields() {
        let record = with(|raw| {
            raw["notes"] = json!({});
        });
        let entry = record.entry().unwrap();
        let character = serde_yaml_ng::to_string(&entry.fields["splat_character"]).unwrap();
        for field in [
            "background:",
            "details:",
            "characteristics:",
            "story:",
            "coins:",
            "companions:",
            "custom_items:",
        ] {
            assert!(!character.contains(field), "{field}");
        }
        assert!(record.profile().background.is_none());
    }

    #[test]
    fn a_background_feature_must_exist() {
        let record = with(|raw| {
            raw["background"] =
                json!({"definition": {"name": "Noble", "featureName": "Position of Privilege"}});
        });
        let mut entry = record.entry().unwrap();
        let mut fields = (*entry.fields).clone();
        let character = fields.get_mut("splat_character").unwrap();
        character["features"] = serde_yaml_ng::Value::Mapping(Default::default());
        entry.fields = fields.into();
        let error = crate::content::character::validate(&entry).unwrap_err();
        assert!(
            error.contains("Background refers to missing feature"),
            "{error}"
        );
    }
}
