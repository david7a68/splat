---
splat_schema: 1
splat_id: 3dae3c2f-76fc-57ce-a4b1-3b629c7e6a0b
splat_kind: rule
splat_key: dnd2024:rules/ranger_as_a_level_1_character
splat_name: As a Level 1 Character
splat_origin:
  key: dnd2024:reference/pages/057
splat_sources:
- title: SRD 5.2.1
  locator: pp. 57
  url: https://www.dndbeyond.com/srd
---

# As a Level 1 Character

## Source text

<!-- SRD page 57 -->

•  Gain all the traits in the Core Ranger Traits table.

•  Gain the Ranger’s level 1 features, which are listed in the Ranger Features table.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/ranger_as_a_level_1_character ` |
| ` implementation ` | ` dnd2024:rules_ranger_as_a_level_1_character ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` requires_total_character_level ` | ` 1 ` |
| ` grant_all_core_traits ` | ` true ` |

#### Core traits

<!-- splat:values path=splat_rule/parameters/core_traits -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/ranger ` |

#### Features

<!-- splat:table
path: splat_rule/parameters/features
key: Feature
order:
- ranger_level_1_favored_enemy
- ranger_level_1_spellcasting
- ranger_level_1_weapon_mastery
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
| [Level 1: Favored Enemy](<../class_features/ranger_level_1_favored_enemy.md> "dnd2024:class_features/ranger_level_1_favored_enemy") | ` 1 ` |
| [Level 1: Spellcasting](<../class_features/ranger_level_1_spellcasting.md> "dnd2024:class_features/ranger_level_1_spellcasting") | ` 1 ` |
| [Level 1: Weapon Mastery](<../class_features/ranger_level_1_weapon_mastery.md> "dnd2024:class_features/ranger_level_1_weapon_mastery") | ` 1 ` |
