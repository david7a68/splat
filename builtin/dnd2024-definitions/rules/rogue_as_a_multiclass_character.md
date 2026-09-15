---
splat_schema: 1
splat_id: d1ea6b55-eb25-5dcb-915e-b5b8c0ffb000
splat_kind: rule
splat_key: dnd2024:rules/rogue_as_a_multiclass_character
splat_name: As a Multiclass Character
splat_origin:
  key: dnd2024:reference/pages/061
splat_sources:
- title: SRD 5.2.1
  locator: pp. 61
  url: https://www.dndbeyond.com/srd
---

# As a Multiclass Character

## Source text

<!-- SRD page 61 -->

•  Gain the following traits from the Core Rogue\
Traits table: Hit Point Die, proficiency in one skill\
of your choice from the Rogue’s skill list, pro-\
ficiency with Thieves’ Tools, and training with\
Light armor.\
•  Gain the Rogue’s level 1 features, which are listed\
in the Rogue Features table.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/rogue_as_a_multiclass_character ` |
| ` implementation ` | ` dnd2024:rules_rogue_as_a_multiclass_character ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` entry ` | ` multiclass ` |
| ` hit_die_sides ` | ` 8 ` |
| ` armor_training ` | ` ["light"] ` |
| ` tool_proficiencies ` | ` ["thieves_tools"] ` |

#### Class

<!-- splat:values path=splat_rule/parameters/class -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/rogue ` |

#### Features

<!-- splat:rows path=splat_rule/parameters/features -->

| Row | level |
| --- | --- |
| ` 0 ` | ` 1 ` |
| ` 1 ` | ` 1 ` |
| ` 2 ` | ` 1 ` |
| ` 3 ` | ` 1 ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/features/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_1_expertise ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/features/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_1_sneak_attack ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/features/2/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_1_thieves_cant ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/features/3/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_1_weapon_mastery ` |

#### Skill choice

<!-- splat:values path=splat_rule/parameters/skill_choice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` options ` | ` ["acrobatics", "athletics", "deception", "insight", "intimidation", "investigation", "perception", "persuasion", "sleight_of_hand", "stealth"] ` |
