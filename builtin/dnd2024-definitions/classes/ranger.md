---
splat_schema: 2
splat_id: aed521b4-3fb7-58f7-a35e-bc8ab5848eef
splat_kind: rule_group
splat_key: dnd2024:classes/ranger
splat_name: Ranger
splat_origin: dnd2024:reference/pages/057
splat_sources:
- SRD 5.2.1, pp. 57, https://www.dndbeyond.com/srd
---

# Ranger

## Source text

<!-- SRD page 57 -->

**Core Ranger Traits**

|  |  |
| --- | --- |
| Primary Ability | Dexterity and Wisdom |
| Hit Point Die | D10 per Ranger level |
| Saving Throw | Strength and Dexterity |
| Proficiencies |  |
| Skill Proficiencies | Choose 3: Animal Handling, Athletics, Insight, Investigation, Nature, Perception, Stealth, or Survival |
| Weapon Proficiencies | Simple and Martial weapons |
| Armor Training | Light and Medium armor and Shields |
| Starting Equipment | Choose A or B: (A) Studded Leather Armor, Scimitar, Shortsword, Longbow, 20 Arrows, Quiver, Druidic Focus (sprig of mistletoe), Explorer’s Pack, and 7 GP; or (B) 150 GP |

**Ranger Features**

| Level | Proficiency Bonus | Class Features | Favored Enemy | Prepared Spells | Level 1 Slots | Level 2 Slots | Level 3 Slots | Level 4 Slots | Level 5 Slots |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | +2 | Spellcasting, Favored Enemy, Weapon Mastery | 2 | 2 | 2 | — | — | — | — |
| 2 | +2 | Deft Explorer, Fighting Style | 2 | 3 | 2 | — | — | — | — |
| 3 | +2 | Ranger Subclass | 2 | 4 | 3 | — | — | — | — |
| 4 | +2 | Ability Score Improvement | 2 | 5 | 3 | — | — | — | — |
| 5 | +3 | Extra Attack | 3 | 6 | 4 | 2 | — | — | — |
| 6 | +3 | Roving | 3 | 6 | 4 | 2 | — | — | — |
| 7 | +3 | Subclass feature | 3 | 7 | 4 | 3 | — | — | — |
| 8 | +3 | Ability Score Improvement | 3 | 7 | 4 | 3 | — | — | — |
| 9 | +4 | Expertise | 4 | 9 | 4 | 3 | 2 | — | — |
| 10 | +4 | Tireless | 4 | 9 | 4 | 3 | 2 | — | — |
| 11 | +4 | Subclass feature | 4 | 10 | 4 | 3 | 3 | — | — |
| 12 | +4 | Ability Score Improvement | 4 | 10 | 4 | 3 | 3 | — | — |
| 13 | +5 | Relentless Hunter | 5 | 11 | 4 | 3 | 3 | 1 | — |
| 14 | +5 | Nature’s Veil | 5 | 11 | 4 | 3 | 3 | 1 | — |
| 15 | +5 | Subclass feature | 5 | 12 | 4 | 3 | 3 | 2 | — |
| 16 | +5 | Ability Score Improvement | 5 | 12 | 4 | 3 | 3 | 2 | — |
| 17 | +6 | Precise Hunter | 6 | 14 | 4 | 3 | 3 | 3 | 1 |
| 18 | +6 | Feral Senses | 6 | 14 | 4 | 3 | 3 | 3 | 1 |
| 19 | +6 | Epic Boon | 6 | 15 | 4 | 3 | 3 | 3 | 2 |
| 20 | +6 | Foe Slayer | 6 | 15 | 4 | 3 | 3 | 3 | 2 |

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` class ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:classes/ranger ` |
| ` implementation ` | ` dnd2024:classes_ranger ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` ranger ` |
| ` primary_abilities ` | ` ["dexterity", "wisdom"] ` |

#### Hit point die

<!-- splat:values path=splat_rule/parameters/hit_point_die -->

| Field | Value |
| --- | --- |
| ` count_per_class_level ` | ` 1 ` |
| ` sides ` | ` 10 ` |

#### First class traits

<!-- splat:yaml path=splat_rule/parameters/first_class_traits -->

```yaml
save_proficiencies:
- strength
- dexterity
skill_proficiencies:
  count: 3
  choices:
  - animal_handling
  - athletics
  - insight
  - investigation
  - nature
  - perception
  - stealth
  - survival
weapon_proficiencies:
  simple: true
  martial: true
armor_training:
- light
- medium
- shield
```

#### Starting equipment

<!-- splat:values path=splat_rule/parameters/starting_equipment -->

| Field | Value |
| --- | --- |
| ` choose ` | ` 1 ` |

##### Options

###### Equipment

###### Items

<!-- splat:table
path: splat_rule/parameters/starting_equipment/options/equipment/items
key: Item
order:
- studded_leather_armor
- scimitar
- shortsword
- longbow
- ammunition_varies
- quiver_1_gp
- druidic_focus_varies
- explorer_s_pack_10_gp
columns:
  Item:
    path: definition
    type: reference
  Quantity:
    path: quantity
    type: integer
  Variant:
    path: variant
    type: symbol
    optional: true
-->

| Item | Quantity | Variant |
| --- | --- | --- |
| [Studded Leather Armor](<../equipment/studded_leather_armor.md> "dnd2024:equipment/studded_leather_armor") | ` 1 ` |  |
| [Scimitar](<../equipment/scimitar.md> "dnd2024:equipment/scimitar") | ` 1 ` |  |
| [Shortsword](<../equipment/shortsword.md> "dnd2024:equipment/shortsword") | ` 1 ` |  |
| [Longbow](<../equipment/longbow.md> "dnd2024:equipment/longbow") | ` 1 ` |  |
| [Ammunition (Varies)](<../equipment/ammunition_varies.md> "dnd2024:equipment/ammunition_varies") | ` 20 ` | ` Arrows ` |
| [Quiver (1 GP)](<../equipment/quiver_1_gp.md> "dnd2024:equipment/quiver_1_gp") | ` 1 ` |  |
| [Druidic Focus (Varies)](<../equipment/druidic_focus_varies.md> "dnd2024:equipment/druidic_focus_varies") | ` 1 ` | ` Sprig of mistletoe ` |
| [Explorer’s Pack (10 GP)](<../equipment/explorer_s_pack_10_gp.md> "dnd2024:equipment/explorer_s_pack_10_gp") | ` 1 ` |  |

###### Currency

<!-- splat:values path=splat_rule/parameters/starting_equipment/options/equipment/currency -->

| Field | Value |
| --- | --- |
| ` gp ` | ` 7 ` |

###### Currency

<!-- splat:yaml path=splat_rule/parameters/starting_equipment/options/currency -->

```yaml
currency:
  gp: 150
```

#### Progression

<!-- splat:values path=splat_rule/parameters/progression -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:tables/ranger_progression ` |

#### Feature grants

<!-- splat:table
path: splat_rule/parameters/feature_grants
key: Item
order:
- ranger_level_10_tireless
- ranger_level_13_relentless_hunter
- ranger_level_14_nature_s_veil
- ranger_level_17_precise_hunter
- ranger_level_18_feral_senses
- ranger_level_19_epic_boon
- ranger_level_1_favored_enemy
- ranger_level_1_spellcasting
- ranger_level_1_weapon_mastery
- ranger_level_20_foe_slayer
- ranger_level_2_deft_explorer
- ranger_level_2_fighting_style
- ranger_level_3_ranger_subclass
- ranger_level_4_ability_score_improvement
- ranger_level_5_extra_attack
- ranger_level_6_roving
- ranger_level_9_expertise
columns:
  Item:
    path: definition
    type: reference
  Minimum class level:
    path: minimum_class_level
    type: integer
-->

| Item | Minimum class level |
| --- | --- |
| [Level 10: Tireless](<../class_features/ranger_level_10_tireless.md> "dnd2024:class_features/ranger_level_10_tireless") | ` 10 ` |
| [Level 13: Relentless Hunter](<../class_features/ranger_level_13_relentless_hunter.md> "dnd2024:class_features/ranger_level_13_relentless_hunter") | ` 13 ` |
| [Level 14: Nature’s Veil](<../class_features/ranger_level_14_nature_s_veil.md> "dnd2024:class_features/ranger_level_14_nature_s_veil") | ` 14 ` |
| [Level 17: Precise Hunter](<../class_features/ranger_level_17_precise_hunter.md> "dnd2024:class_features/ranger_level_17_precise_hunter") | ` 17 ` |
| [Level 18: Feral Senses](<../class_features/ranger_level_18_feral_senses.md> "dnd2024:class_features/ranger_level_18_feral_senses") | ` 18 ` |
| [Level 19: Epic Boon](<../class_features/ranger_level_19_epic_boon.md> "dnd2024:class_features/ranger_level_19_epic_boon") | ` 19 ` |
| [Level 1: Favored Enemy](<../class_features/ranger_level_1_favored_enemy.md> "dnd2024:class_features/ranger_level_1_favored_enemy") | ` 1 ` |
| [Level 1: Spellcasting](<../class_features/ranger_level_1_spellcasting.md> "dnd2024:class_features/ranger_level_1_spellcasting") | ` 1 ` |
| [Level 1: Weapon Mastery](<../class_features/ranger_level_1_weapon_mastery.md> "dnd2024:class_features/ranger_level_1_weapon_mastery") | ` 1 ` |
| [Level 20: Foe Slayer](<../class_features/ranger_level_20_foe_slayer.md> "dnd2024:class_features/ranger_level_20_foe_slayer") | ` 20 ` |
| [Level 2: Deft Explorer](<../class_features/ranger_level_2_deft_explorer.md> "dnd2024:class_features/ranger_level_2_deft_explorer") | ` 2 ` |
| [Level 2: Fighting Style](<../class_features/ranger_level_2_fighting_style.md> "dnd2024:class_features/ranger_level_2_fighting_style") | ` 2 ` |
| [Level 3: Ranger Subclass](<../class_features/ranger_level_3_ranger_subclass.md> "dnd2024:class_features/ranger_level_3_ranger_subclass") | ` 3 ` |
| [Level 4: Ability Score Improvement](<../class_features/ranger_level_4_ability_score_improvement.md> "dnd2024:class_features/ranger_level_4_ability_score_improvement") | ` 4 ` |
| [Level 5: Extra Attack](<../class_features/ranger_level_5_extra_attack.md> "dnd2024:class_features/ranger_level_5_extra_attack") | ` 5 ` |
| [Level 6: Roving](<../class_features/ranger_level_6_roving.md> "dnd2024:class_features/ranger_level_6_roving") | ` 6 ` |
| [Level 9: Expertise](<../class_features/ranger_level_9_expertise.md> "dnd2024:class_features/ranger_level_9_expertise") | ` 9 ` |
