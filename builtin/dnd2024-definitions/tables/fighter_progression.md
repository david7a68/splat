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

<!-- splat:rows path=splat_rule/parameters/rows -->

| Row | level | proficiency_bonus | features_text | second_wind | weapon_mastery |
| --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` 1 ` | ` 2 ` | ` Fighting Style, Second Wind, Weapon Mastery ` | ` 2 ` | ` 3 ` |
| ` 1 ` | ` 2 ` | ` 2 ` | ` Action Surge (one use), Tactical Mind ` | ` 2 ` | ` 3 ` |
| ` 2 ` | ` 3 ` | ` 2 ` | ` Fighter Subclass ` | ` 2 ` | ` 3 ` |
| ` 3 ` | ` 4 ` | ` 2 ` | ` Ability Score Improvement ` | ` 3 ` | ` 4 ` |
| ` 4 ` | ` 5 ` | ` 3 ` | ` Extra Attack, Tactical Shift ` | ` 3 ` | ` 4 ` |
| ` 5 ` | ` 6 ` | ` 3 ` | ` Ability Score Improvement ` | ` 3 ` | ` 4 ` |
| ` 6 ` | ` 7 ` | ` 3 ` | ` Subclass feature ` | ` 3 ` | ` 4 ` |
| ` 7 ` | ` 8 ` | ` 3 ` | ` Ability Score Improvement ` | ` 3 ` | ` 4 ` |
| ` 8 ` | ` 9 ` | ` 4 ` | ` Indomitable (one use), Tactical Master ` | ` 3 ` | ` 4 ` |
| ` 9 ` | ` 10 ` | ` 4 ` | ` Subclass feature ` | ` 4 ` | ` 5 ` |
| ` 10 ` | ` 11 ` | ` 4 ` | ` Two Extra Attacks ` | ` 4 ` | ` 5 ` |
| ` 11 ` | ` 12 ` | ` 4 ` | ` Ability Score Improvement ` | ` 4 ` | ` 5 ` |
| ` 12 ` | ` 13 ` | ` 5 ` | ` Indomitable (two uses), Studied Attacks ` | ` 4 ` | ` 5 ` |
| ` 13 ` | ` 14 ` | ` 5 ` | ` Ability Score Improvement ` | ` 4 ` | ` 5 ` |
| ` 14 ` | ` 15 ` | ` 5 ` | ` Subclass feature ` | ` 4 ` | ` 5 ` |
| ` 15 ` | ` 16 ` | ` 5 ` | ` Ability Score Improvement ` | ` 4 ` | ` 6 ` |
| ` 16 ` | ` 17 ` | ` 6 ` | ` Action Surge (two uses), Indomitable (three uses) ` | ` 4 ` | ` 6 ` |
| ` 17 ` | ` 18 ` | ` 6 ` | ` Subclass feature ` | ` 4 ` | ` 6 ` |
| ` 18 ` | ` 19 ` | ` 6 ` | ` Epic Boon ` | ` 4 ` | ` 6 ` |
| ` 19 ` | ` 20 ` | ` 6 ` | ` Three Extra Attacks ` | ` 4 ` | ` 6 ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/0/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 1 ` |
| ` 1 ` | ` grant_feature ` | ` 1 ` |
| ` 2 ` | ` grant_feature ` | ` 1 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/0/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_1_fighting_style ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/0/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_1_second_wind ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/0/grants/2/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_1_weapon_mastery ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/1/grants -->

| Row | operation | at_class_level | maximum_uses |
| --- | --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 2 ` | ` 1 ` |
| ` 1 ` | ` grant_feature ` | ` 2 ` |  |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/1/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_2_action_surge ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/1/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_2_tactical_mind ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/2/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` choose_subclass ` | ` 3 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/2/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_3_fighter_subclass ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/3/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 4 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/3/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_4_ability_score_improvement ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/4/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 5 ` |
| ` 1 ` | ` grant_feature ` | ` 5 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/4/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_5_extra_attack ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/4/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_5_tactical_shift ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/5/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 6 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/5/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_4_ability_score_improvement ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/6/grants -->

| Row | operation | class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_selected_subclass_features ` | ` 7 ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/7/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 8 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/7/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_4_ability_score_improvement ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/8/grants -->

| Row | operation | at_class_level | maximum_uses |
| --- | --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 9 ` | ` 1 ` |
| ` 1 ` | ` grant_feature ` | ` 9 ` |  |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/8/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_9_indomitable ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/8/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_9_tactical_master ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/9/grants -->

| Row | operation | class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_selected_subclass_features ` | ` 10 ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/10/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 11 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/10/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_11_two_extra_attacks ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/11/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 12 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/11/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_4_ability_score_improvement ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/12/grants -->

| Row | operation | at_class_level | maximum_uses |
| --- | --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 13 ` | ` 2 ` |
| ` 1 ` | ` grant_feature ` | ` 13 ` |  |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/12/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_9_indomitable ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/12/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_13_studied_attacks ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/13/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 14 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/13/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_4_ability_score_improvement ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/14/grants -->

| Row | operation | class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_selected_subclass_features ` | ` 15 ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/15/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 16 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/15/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_4_ability_score_improvement ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/16/grants -->

| Row | operation | at_class_level | maximum_uses |
| --- | --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 17 ` | ` 2 ` |
| ` 1 ` | ` grant_feature ` | ` 17 ` | ` 3 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/16/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_2_action_surge ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/16/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_9_indomitable ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/17/grants -->

| Row | operation | class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_selected_subclass_features ` | ` 18 ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/18/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 19 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/18/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_19_epic_boon ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/19/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 20 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/19/grants/0/feature -->

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
