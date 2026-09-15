---
splat_schema: 1
splat_id: 54be02d7-b7b5-5e60-850c-892a8d7bca02
splat_kind: rule
splat_key: dnd2024:rules/fighter_as_a_multiclass_character
splat_name: As a Multiclass Character
splat_origin:
  key: dnd2024:reference/pages/047
splat_sources:
- title: SRD 5.2.1
  locator: pp. 47
  url: https://www.dndbeyond.com/srd
---

# As a Multiclass Character

## Source text

<!-- SRD page 47 -->

•  Gain the following traits from the Core Fighter\
Traits table: Hit Point Die, proficiency with Mar-\
tial weapons, and training with Light and Medium\
armor and Shields.\
•  Gain the Fighter’s level 1 features, which are\
listed in the Fighter Features table.\

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

<!-- splat:rows path=splat_rule/parameters/features -->

| Row | level |
| --- | --- |
| ` 0 ` | ` 1 ` |
| ` 1 ` | ` 1 ` |
| ` 2 ` | ` 1 ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/features/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_1_fighting_style ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/features/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_1_second_wind ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/features/2/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_1_weapon_mastery ` |
