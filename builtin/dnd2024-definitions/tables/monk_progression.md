---
splat_schema: 2
splat_id: 235f10c4-25ec-5b36-8da9-344befa0f4c4
splat_kind: rule
splat_key: dnd2024:tables/monk_progression
splat_name: Monk Progression
splat_origin: dnd2024:reference/pages/050
splat_sources:
- SRD 5.2.1, pp. 50, https://www.dndbeyond.com/srd
---

# Monk Progression

## Source text

**Monk Features**

| Level | Proficiency Bonus | Class Features | Martial Arts | Focus Points | Unarmored Movement |
| --- | --- | --- | --- | --- | --- |
| 1 | +2 | Martial Arts, Unarmored Defense | 1d6 | — | — |
| 2 | +2 | Monk’s Focus, Unarmored Movement, Uncanny Metabolism | 1d6 | 2 | +10 ft. |
| 3 | +2 | Deflect Attacks, Monk Subclass | 1d6 | 3 | +10 ft. |
| 4 | +2 | Ability Score Improvement, Slow Fall | 1d6 | 4 | +10 ft. |
| 5 | +3 | Extra Attack, Stunning Strike | 1d8 | 5 | +10 ft. |
| 6 | +3 | Empowered Strikes, Subclass feature | 1d8 | 6 | +15 ft. |
| 7 | +3 | Evasion | 1d8 | 7 | +15 ft. |
| 8 | +3 | Ability Score Improvement | 1d8 | 8 | +15 ft. |
| 9 | +4 | Acrobatic Movement | 1d8 | 9 | +15 ft. |
| 10 | +4 | Heightened Focus, Self-Restoration | 1d8 | 10 | +20 ft. |
| 11 | +4 | Subclass feature | 1d10 | 11 | +20 ft. |
| 12 | +4 | Ability Score Improvement | 1d10 | 12 | +20 ft. |
| 13 | +5 | Deflect Energy | 1d10 | 13 | +20 ft. |
| 14 | +5 | Disciplined Survivor | 1d10 | 14 | +25 ft. |
| 15 | +5 | Perfect Focus | 1d10 | 15 | +25 ft. |
| 16 | +5 | Ability Score Improvement | 1d10 | 16 | +25 ft. |
| 17 | +6 | Subclass feature | 1d12 | 17 | +25 ft. |
| 18 | +6 | Superior Defense | 1d12 | 18 | +30 ft. |
| 19 | +6 | Epic Boon | 1d12 | 19 | +30 ft. |
| 20 | +6 | Body and Mind | 1d12 | 20 | +30 ft. |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:tables/monk_progression ` |
| ` implementation ` | ` dnd2024:class_progression_monk_progression ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` monk ` |

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
  Martial arts:
    path: martial_arts
    type: string
  Focus points:
    path: focus_points
    type: integer
  Unarmored movement:
    path: unarmored_movement
    type: integer
  Martial arts dice dice count:
    path: martial_arts_dice/dice/count
    type: integer
  Martial arts dice dice sides:
    path: martial_arts_dice/dice/sides
    type: integer
  Martial arts dice bonus:
    path: martial_arts_dice/bonus
    type: integer
  Details:
    type: section
    optional: true
-->

| Key | Level | Proficiency bonus | Features text | Martial arts | Focus points | Unarmored movement | Martial arts dice dice count | Martial arts dice dice sides | Martial arts dice bonus | Details |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` 1 ` | ` 2 ` | ` Martial Arts, Unarmored Defense ` | ` 1d6 ` | ` 0 ` | ` 0 ` | ` 1 ` | ` 6 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_1 "section:row_1") |
| ` row_2 ` | ` 2 ` | ` 2 ` | ` Monk’s Focus, Unarmored Movement, Uncanny Metabolism ` | ` 1d6 ` | ` 2 ` | ` 10 ` | ` 1 ` | ` 6 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_2 "section:row_2") |
| ` row_3 ` | ` 3 ` | ` 2 ` | ` Deflect Attacks, Monk Subclass ` | ` 1d6 ` | ` 3 ` | ` 10 ` | ` 1 ` | ` 6 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_3 "section:row_3") |
| ` row_4 ` | ` 4 ` | ` 2 ` | ` Ability Score Improvement, Slow Fall ` | ` 1d6 ` | ` 4 ` | ` 10 ` | ` 1 ` | ` 6 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_4 "section:row_4") |
| ` row_5 ` | ` 5 ` | ` 3 ` | ` Extra Attack, Stunning Strike ` | ` 1d8 ` | ` 5 ` | ` 10 ` | ` 1 ` | ` 8 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_5 "section:row_5") |
| ` row_6 ` | ` 6 ` | ` 3 ` | ` Empowered Strikes, Subclass feature ` | ` 1d8 ` | ` 6 ` | ` 15 ` | ` 1 ` | ` 8 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_6 "section:row_6") |
| ` row_7 ` | ` 7 ` | ` 3 ` | ` Evasion ` | ` 1d8 ` | ` 7 ` | ` 15 ` | ` 1 ` | ` 8 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_7 "section:row_7") |
| ` row_8 ` | ` 8 ` | ` 3 ` | ` Ability Score Improvement ` | ` 1d8 ` | ` 8 ` | ` 15 ` | ` 1 ` | ` 8 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_8 "section:row_8") |
| ` row_9 ` | ` 9 ` | ` 4 ` | ` Acrobatic Movement ` | ` 1d8 ` | ` 9 ` | ` 15 ` | ` 1 ` | ` 8 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_9 "section:row_9") |
| ` row_10 ` | ` 10 ` | ` 4 ` | ` Heightened Focus, Self-Restoration ` | ` 1d8 ` | ` 10 ` | ` 20 ` | ` 1 ` | ` 8 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_10 "section:row_10") |
| ` row_11 ` | ` 11 ` | ` 4 ` | ` Subclass feature ` | ` 1d10 ` | ` 11 ` | ` 20 ` | ` 1 ` | ` 10 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_11 "section:row_11") |
| ` row_12 ` | ` 12 ` | ` 4 ` | ` Ability Score Improvement ` | ` 1d10 ` | ` 12 ` | ` 20 ` | ` 1 ` | ` 10 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_12 "section:row_12") |
| ` row_13 ` | ` 13 ` | ` 5 ` | ` Deflect Energy ` | ` 1d10 ` | ` 13 ` | ` 20 ` | ` 1 ` | ` 10 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_13 "section:row_13") |
| ` row_14 ` | ` 14 ` | ` 5 ` | ` Disciplined Survivor ` | ` 1d10 ` | ` 14 ` | ` 25 ` | ` 1 ` | ` 10 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_14 "section:row_14") |
| ` row_15 ` | ` 15 ` | ` 5 ` | ` Perfect Focus ` | ` 1d10 ` | ` 15 ` | ` 25 ` | ` 1 ` | ` 10 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_15 "section:row_15") |
| ` row_16 ` | ` 16 ` | ` 5 ` | ` Ability Score Improvement ` | ` 1d10 ` | ` 16 ` | ` 25 ` | ` 1 ` | ` 10 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_16 "section:row_16") |
| ` row_17 ` | ` 17 ` | ` 6 ` | ` Subclass feature ` | ` 1d12 ` | ` 17 ` | ` 25 ` | ` 1 ` | ` 12 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_17 "section:row_17") |
| ` row_18 ` | ` 18 ` | ` 6 ` | ` Superior Defense ` | ` 1d12 ` | ` 18 ` | ` 30 ` | ` 1 ` | ` 12 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_18 "section:row_18") |
| ` row_19 ` | ` 19 ` | ` 6 ` | ` Epic Boon ` | ` 1d12 ` | ` 19 ` | ` 30 ` | ` 1 ` | ` 12 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_19 "section:row_19") |
| ` row_20 ` | ` 20 ` | ` 6 ` | ` Body and Mind ` | ` 1d12 ` | ` 20 ` | ` 30 ` | ` 1 ` | ` 12 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_20 "section:row_20") |

##### Row 1

^splat-0f5cfe33-row_1

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_1/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 1 ` |
| ` 1 ` | ` grant_feature ` | ` 1 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_1/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_1_martial_arts ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_1/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_1_unarmored_defense ` |

##### Row 2

^splat-0f5cfe33-row_2

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_2/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 2 ` |
| ` 1 ` | ` grant_feature ` | ` 2 ` |
| ` 2 ` | ` grant_feature ` | ` 2 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_2/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_2_monk_s_focus ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_2/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_2_unarmored_movement ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_2/grants/2/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_2_uncanny_metabolism ` |

##### Row 3

^splat-0f5cfe33-row_3

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_3/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 3 ` |
| ` 1 ` | ` choose_subclass ` | ` 3 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_3/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_3_deflect_attacks ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_3/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_3_monk_subclass ` |

##### Row 4

^splat-0f5cfe33-row_4

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_4/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 4 ` |
| ` 1 ` | ` grant_feature ` | ` 4 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_4/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_4_ability_score_improvement ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_4/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_4_slow_fall ` |

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
| ` key ` | ` dnd2024:class_features/monk_level_5_extra_attack ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_5/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_5_stunning_strike ` |

##### Row 6

^splat-0f5cfe33-row_6

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_6/grants -->

| Row | operation | at_class_level | class_level |
| --- | --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 6 ` |  |
| ` 1 ` | ` grant_selected_subclass_features ` |  | ` 6 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_6/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_6_empowered_strikes ` |

##### Row 7

^splat-0f5cfe33-row_7

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_7/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 7 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_7/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_7_evasion ` |

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
| ` key ` | ` dnd2024:class_features/monk_level_4_ability_score_improvement ` |

##### Row 9

^splat-0f5cfe33-row_9

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_9/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 9 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_9/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_9_acrobatic_movement ` |

##### Row 10

^splat-0f5cfe33-row_10

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_10/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 10 ` |
| ` 1 ` | ` grant_feature ` | ` 10 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_10/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_10_heightened_focus ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_10/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_10_self_restoration ` |

##### Row 11

^splat-0f5cfe33-row_11

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_11/grants -->

| Row | operation | class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_selected_subclass_features ` | ` 11 ` |

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
| ` key ` | ` dnd2024:class_features/monk_level_4_ability_score_improvement ` |

##### Row 13

^splat-0f5cfe33-row_13

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_13/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 13 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_13/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_13_deflect_energy ` |

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
| ` key ` | ` dnd2024:class_features/monk_level_14_disciplined_survivor ` |

##### Row 15

^splat-0f5cfe33-row_15

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_15/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 15 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_15/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_15_perfect_focus ` |

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
| ` key ` | ` dnd2024:class_features/monk_level_4_ability_score_improvement ` |

##### Row 17

^splat-0f5cfe33-row_17

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_17/grants -->

| Row | operation | class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_selected_subclass_features ` | ` 17 ` |

##### Row 18

^splat-0f5cfe33-row_18

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_18/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 18 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_18/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_18_superior_defense ` |

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
| ` key ` | ` dnd2024:class_features/monk_level_19_epic_boon ` |

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
| ` key ` | ` dnd2024:class_features/monk_level_20_body_and_mind ` |

#### Class definition

<!-- splat:values path=splat_rule/parameters/class_definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/monk ` |

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
focus_points:
  kind: resource_capacity
  resource: focus_points
  recovery: owning_feature_contract
  level_change_refills: false
martial_arts:
  kind: dice_expression
  applies_to: martial_arts
proficiency_bonus:
  kind: bonus
  applies_to: proficiency
  use_class_row_only_if_single_class: true
unarmored_movement:
  kind: speed_bonus
  unit: ft
  requires: unarmored_movement_feature_eligibility
```
