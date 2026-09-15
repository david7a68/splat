---
splat_schema: 1
splat_id: a8941e4b-da5a-5490-aed3-a271872d1611
splat_kind: rule
splat_key: dnd2024:tables/fighter_progression
splat_name: Fighter Progression
splat_origin:
  key: dnd2024:reference/pages/047
splat_sources:
- title: SRD 5.2.1
  locator: pp. 47
  url: https://www.dndbeyond.com/srd
---

# Fighter Progression

## Source text

From SRD 5.2.1 page 47.

     Fighter Features
                  Proficiency                                                                     Second        Weapon
       Level        Bonus          Class Features                                                  Wind         Mastery
         1             +2         Fighting Style, Second Wind, Weapon Mastery                        2              3
         2             +2         Action Surge (one use), Tactical Mind                              2              3
         3             +2         Fighter Subclass                                                   2              3
         4             +2         Ability Score Improvement                                          3              4
         5             +3         Extra Attack, Tactical Shift                                       3              4
         6             +3         Ability Score Improvement                                          3              4
         7             +3         Subclass feature                                                   3              4
         8             +3         Ability Score Improvement                                          3              4
         9             +4         Indomitable (one use), Tactical Master                             3              4
         10            +4         Subclass feature                                                   4              5
         11            +4         Two Extra Attacks                                                  4              5
         12            +4         Ability Score Improvement                                          4              5
         13            +5         Indomitable (two uses), Studied Attacks                            4              5
         14            +5         Ability Score Improvement                                          4              5
         15            +5         Subclass feature                                                   4              5
         16            +5         Ability Score Improvement                                          4              6
         17            +6         Action Surge (two uses), Indomitable (three uses)                  4              6
         18            +6         Subclass feature                                                   4              6
         19            +6         Epic Boon                                                          4              6
         20            +6         Three Extra Attacks                                                4              6



47   System Reference Document 5.2.1

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:tables/fighter_progression ` |
| ` implementation ` | ` dnd2024:class_progression_fighter_progression ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` fighter ` |

#### Rows

<!-- splat:table
path: splat_rule/parameters/rows
key: Key
order:
- row_1
- row_2
- row_3
- row_4
- row_5
- row_6
- row_7
- row_8
- row_9
- row_10
- row_11
- row_12
- row_13
- row_14
- row_15
- row_16
- row_17
- row_18
- row_19
- row_20
columns:
  Key:
    type: key
  Level:
    path: level
    type: integer
  Proficiency bonus:
    path: proficiency_bonus
    type: integer
  Features text:
    path: features_text
    type: string
  Second wind:
    path: second_wind
    type: integer
  Weapon mastery:
    path: weapon_mastery
    type: integer
  Details:
    type: section
    optional: true
-->

| Key | Level | Proficiency bonus | Features text | Second wind | Weapon mastery | Details |
| --- | --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` 1 ` | ` 2 ` | ` Fighting Style, Second Wind, Weapon Mastery ` | ` 2 ` | ` 3 ` | [Rules](#^splat-0f5cfe33-row_1 "section:row_1") |
| ` row_2 ` | ` 2 ` | ` 2 ` | ` Action Surge (one use), Tactical Mind ` | ` 2 ` | ` 3 ` | [Rules](#^splat-0f5cfe33-row_2 "section:row_2") |
| ` row_3 ` | ` 3 ` | ` 2 ` | ` Fighter Subclass ` | ` 2 ` | ` 3 ` | [Rules](#^splat-0f5cfe33-row_3 "section:row_3") |
| ` row_4 ` | ` 4 ` | ` 2 ` | ` Ability Score Improvement ` | ` 3 ` | ` 4 ` | [Rules](#^splat-0f5cfe33-row_4 "section:row_4") |
| ` row_5 ` | ` 5 ` | ` 3 ` | ` Extra Attack, Tactical Shift ` | ` 3 ` | ` 4 ` | [Rules](#^splat-0f5cfe33-row_5 "section:row_5") |
| ` row_6 ` | ` 6 ` | ` 3 ` | ` Ability Score Improvement ` | ` 3 ` | ` 4 ` | [Rules](#^splat-0f5cfe33-row_6 "section:row_6") |
| ` row_7 ` | ` 7 ` | ` 3 ` | ` Subclass feature ` | ` 3 ` | ` 4 ` | [Rules](#^splat-0f5cfe33-row_7 "section:row_7") |
| ` row_8 ` | ` 8 ` | ` 3 ` | ` Ability Score Improvement ` | ` 3 ` | ` 4 ` | [Rules](#^splat-0f5cfe33-row_8 "section:row_8") |
| ` row_9 ` | ` 9 ` | ` 4 ` | ` Indomitable (one use), Tactical Master ` | ` 3 ` | ` 4 ` | [Rules](#^splat-0f5cfe33-row_9 "section:row_9") |
| ` row_10 ` | ` 10 ` | ` 4 ` | ` Subclass feature ` | ` 4 ` | ` 5 ` | [Rules](#^splat-0f5cfe33-row_10 "section:row_10") |
| ` row_11 ` | ` 11 ` | ` 4 ` | ` Two Extra Attacks ` | ` 4 ` | ` 5 ` | [Rules](#^splat-0f5cfe33-row_11 "section:row_11") |
| ` row_12 ` | ` 12 ` | ` 4 ` | ` Ability Score Improvement ` | ` 4 ` | ` 5 ` | [Rules](#^splat-0f5cfe33-row_12 "section:row_12") |
| ` row_13 ` | ` 13 ` | ` 5 ` | ` Indomitable (two uses), Studied Attacks ` | ` 4 ` | ` 5 ` | [Rules](#^splat-0f5cfe33-row_13 "section:row_13") |
| ` row_14 ` | ` 14 ` | ` 5 ` | ` Ability Score Improvement ` | ` 4 ` | ` 5 ` | [Rules](#^splat-0f5cfe33-row_14 "section:row_14") |
| ` row_15 ` | ` 15 ` | ` 5 ` | ` Subclass feature ` | ` 4 ` | ` 5 ` | [Rules](#^splat-0f5cfe33-row_15 "section:row_15") |
| ` row_16 ` | ` 16 ` | ` 5 ` | ` Ability Score Improvement ` | ` 4 ` | ` 6 ` | [Rules](#^splat-0f5cfe33-row_16 "section:row_16") |
| ` row_17 ` | ` 17 ` | ` 6 ` | ` Action Surge (two uses), Indomitable (three uses) ` | ` 4 ` | ` 6 ` | [Rules](#^splat-0f5cfe33-row_17 "section:row_17") |
| ` row_18 ` | ` 18 ` | ` 6 ` | ` Subclass feature ` | ` 4 ` | ` 6 ` | [Rules](#^splat-0f5cfe33-row_18 "section:row_18") |
| ` row_19 ` | ` 19 ` | ` 6 ` | ` Epic Boon ` | ` 4 ` | ` 6 ` | [Rules](#^splat-0f5cfe33-row_19 "section:row_19") |
| ` row_20 ` | ` 20 ` | ` 6 ` | ` Three Extra Attacks ` | ` 4 ` | ` 6 ` | [Rules](#^splat-0f5cfe33-row_20 "section:row_20") |

##### Row 1

^splat-0f5cfe33-row_1

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_1/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 1 ` |
| ` 1 ` | ` grant_feature ` | ` 1 ` |
| ` 2 ` | ` grant_feature ` | ` 1 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_1/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_1_fighting_style ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_1/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_1_second_wind ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_1/grants/2/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_1_weapon_mastery ` |

##### Row 2

^splat-0f5cfe33-row_2

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_2/grants -->

| Row | operation | at_class_level | maximum_uses |
| --- | --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 2 ` | ` 1 ` |
| ` 1 ` | ` grant_feature ` | ` 2 ` |  |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_2/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_2_action_surge ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_2/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_2_tactical_mind ` |

##### Row 3

^splat-0f5cfe33-row_3

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_3/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` choose_subclass ` | ` 3 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_3/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_3_fighter_subclass ` |

##### Row 4

^splat-0f5cfe33-row_4

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_4/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 4 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_4/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_4_ability_score_improvement ` |

##### Row 5

^splat-0f5cfe33-row_5

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_5/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 5 ` |
| ` 1 ` | ` grant_feature ` | ` 5 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_5/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_5_extra_attack ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_5/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_5_tactical_shift ` |

##### Row 6

^splat-0f5cfe33-row_6

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_6/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 6 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_6/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_4_ability_score_improvement ` |

##### Row 7

^splat-0f5cfe33-row_7

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_7/grants -->

| Row | operation | class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_selected_subclass_features ` | ` 7 ` |

##### Row 8

^splat-0f5cfe33-row_8

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_8/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 8 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_8/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_4_ability_score_improvement ` |

##### Row 9

^splat-0f5cfe33-row_9

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_9/grants -->

| Row | operation | at_class_level | maximum_uses |
| --- | --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 9 ` | ` 1 ` |
| ` 1 ` | ` grant_feature ` | ` 9 ` |  |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_9/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_9_indomitable ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_9/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_9_tactical_master ` |

##### Row 10

^splat-0f5cfe33-row_10

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_10/grants -->

| Row | operation | class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_selected_subclass_features ` | ` 10 ` |

##### Row 11

^splat-0f5cfe33-row_11

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_11/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 11 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_11/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_11_two_extra_attacks ` |

##### Row 12

^splat-0f5cfe33-row_12

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_12/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 12 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_12/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_4_ability_score_improvement ` |

##### Row 13

^splat-0f5cfe33-row_13

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_13/grants -->

| Row | operation | at_class_level | maximum_uses |
| --- | --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 13 ` | ` 2 ` |
| ` 1 ` | ` grant_feature ` | ` 13 ` |  |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_13/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_9_indomitable ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_13/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_13_studied_attacks ` |

##### Row 14

^splat-0f5cfe33-row_14

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_14/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 14 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_14/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_4_ability_score_improvement ` |

##### Row 15

^splat-0f5cfe33-row_15

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_15/grants -->

| Row | operation | class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_selected_subclass_features ` | ` 15 ` |

##### Row 16

^splat-0f5cfe33-row_16

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_16/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 16 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_16/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_4_ability_score_improvement ` |

##### Row 17

^splat-0f5cfe33-row_17

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_17/grants -->

| Row | operation | at_class_level | maximum_uses |
| --- | --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 17 ` | ` 2 ` |
| ` 1 ` | ` grant_feature ` | ` 17 ` | ` 3 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_17/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_2_action_surge ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_17/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_9_indomitable ` |

##### Row 18

^splat-0f5cfe33-row_18

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_18/grants -->

| Row | operation | class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_selected_subclass_features ` | ` 18 ` |

##### Row 19

^splat-0f5cfe33-row_19

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_19/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 19 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_19/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_19_epic_boon ` |

##### Row 20

^splat-0f5cfe33-row_20

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_20/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 20 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_20/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_20_three_extra_attacks ` |

#### Class definition

<!-- splat:values path=splat_rule/parameters/class_definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/fighter ` |

#### Lookup

<!-- splat:values path=splat_rule/parameters/lookup -->

| Field | Value |
| --- | --- |
| ` input ` | ` class_level ` |
| ` minimum ` | ` 1 ` |
| ` maximum ` | ` 20 ` |
| ` match ` | ` exact ` |
| ` numeric_columns ` | ` current_capacity_or_value ` |
| ` feature_grants ` | ` all_crossed_levels ` |
| ` repeat_application ` | ` idempotent_per_class_level_and_feature ` |

#### Multiclass

<!-- splat:yaml path=splat_rule/parameters/multiclass -->

```yaml
proficiency_bonus_source: total_character_level
spell_slot_override_rule:
  key: dnd2024:rules/multiclassing_spellcasting
pact_magic_pool_separate: true
```

#### Column semantics

<!-- splat:yaml path=splat_rule/parameters/column_semantics -->

```yaml
proficiency_bonus:
  kind: bonus
  applies_to: proficiency
  use_class_row_only_if_single_class: true
second_wind:
  kind: resource_capacity
  resource: second_wind
  recovery: owning_feature_contract
  level_change_refills: false
weapon_mastery:
  kind: selection_capacity
  selection: weapon_mastery
  eligibility: owning_feature_contract
```
