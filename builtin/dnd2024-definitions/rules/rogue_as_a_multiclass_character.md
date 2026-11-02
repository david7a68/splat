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

•  Gain the following traits from the Core Rogue Traits table: Hit Point Die, proficiency in one skill of your choice from the Rogue’s skill list, proficiency with Thieves’ Tools, and training with Light armor.

•  Gain the Rogue’s level 1 features, which are listed in the Rogue Features table.

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

<!-- splat:table
path: splat_rule/parameters/features
key: Feature
order:
- rogue_level_1_expertise
- rogue_level_1_sneak_attack
- rogue_level_1_thieves_cant
- rogue_level_1_weapon_mastery
columns:
  Feature:
    path: feature
    type: reference
  Level:
    path: level
    type: integer
-->

| Feature | Level |
| --- | --- |
| [Level 1: Expertise](<../class_features/rogue_level_1_expertise.md> "dnd2024:class_features/rogue_level_1_expertise") | ` 1 ` |
| [Level 1: Sneak Attack](<../class_features/rogue_level_1_sneak_attack.md> "dnd2024:class_features/rogue_level_1_sneak_attack") | ` 1 ` |
| [Level 1: Thieves’ Cant](<../class_features/rogue_level_1_thieves_cant.md> "dnd2024:class_features/rogue_level_1_thieves_cant") | ` 1 ` |
| [Level 1: Weapon Mastery](<../class_features/rogue_level_1_weapon_mastery.md> "dnd2024:class_features/rogue_level_1_weapon_mastery") | ` 1 ` |

#### Skill choice

<!-- splat:values path=splat_rule/parameters/skill_choice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` options ` | ` ["acrobatics", "athletics", "deception", "insight", "intimidation", "investigation", "perception", "persuasion", "sleight_of_hand", "stealth"] ` |
