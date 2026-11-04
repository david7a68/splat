---
splat_schema: 2
splat_id: 54be02d7-b7b5-5e60-850c-892a8d7bca02
splat_kind: rule
splat_key: dnd2024:rules/fighter_as_a_multiclass_character
splat_name: As a Multiclass Character
splat_origin: dnd2024:reference/pages/047
splat_sources:
- SRD 5.2.1, pp. 47, https://www.dndbeyond.com/srd
---

# As a Multiclass Character

## Source text

<!-- SRD page 47 -->

•  Gain the following traits from the Core Fighter Traits table: Hit Point Die, proficiency with Martial weapons, and training with Light and Medium armor and Shields.

•  Gain the Fighter’s level 1 features, which are listed in the Fighter Features table.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/fighter_as_a_multiclass_character ` |
| ` implementation ` | ` dnd2024:rules_fighter_as_a_multiclass_character ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` entry ` | ` multiclass ` |
| ` hit_die_sides ` | ` 10 ` |
| ` armor_training ` | ` ["light", "medium", "shield"] ` |
| ` weapon_proficiencies ` | ` ["martial"] ` |

#### Class

<!-- splat:values path=splat_rule/parameters/class -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/fighter ` |

#### Features

<!-- splat:table
path: splat_rule/parameters/features
key: Feature
order:
- fighter_level_1_fighting_style
- fighter_level_1_second_wind
- fighter_level_1_weapon_mastery
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
| [Level 1: Fighting Style](<../class_features/fighter_level_1_fighting_style.md> "dnd2024:class_features/fighter_level_1_fighting_style") | ` 1 ` |
| [Level 1: Second Wind](<../class_features/fighter_level_1_second_wind.md> "dnd2024:class_features/fighter_level_1_second_wind") | ` 1 ` |
| [Level 1: Weapon Mastery](<../class_features/fighter_level_1_weapon_mastery.md> "dnd2024:class_features/fighter_level_1_weapon_mastery") | ` 1 ` |
