---
splat_schema: 1
splat_id: 71d76934-1fc8-50be-a15c-cca568917292
splat_kind: rule
splat_key: dnd2024:rules/monk_monk_class_features
splat_name: Monk Class Features
splat_origin:
  key: dnd2024:reference/pages/049
splat_sources:
- title: SRD 5.2.1
  locator: pp. 49, 50
  url: https://www.dndbeyond.com/srd
---

# Monk Class Features

## Source text

<!-- SRD page 49 -->

As a Monk, you gain the following class features\
when you reach the specified Monk levels. These\
features are listed in the Monk Features table.\

<!-- SRD page 50 -->

Monk Features\
Proficiency\
Level Bonus Class Features\
1 +2 Martial Arts, Unarmored Defense\
2 +2 Monk’s Focus, Unarmored Movement,\
Uncanny Metabolism\
3 +2 Deflect Attacks, Monk Subclass\
4 +2 Ability Score Improvement, Slow Fall\
5 +3 Extra Attack, Stunning Strike\
6 +3 Empowered Strikes, Subclass feature\
7 +3 Evasion\
8 +3 Ability Score Improvement\
9 +4 Acrobatic Movement\
10 +4 Heightened Focus, Self-Restoration\
11 +4 Subclass feature\
12 +4 Ability Score Improvement\
13 +5 Deflect Energy\
14 +5 Disciplined Survivor\
15 +5 Perfect Focus\
16 +5 Ability Score Improvement\
17 +6 Subclass feature\
18 +6 Superior Defense\
19 +6 Epic Boon\
20 +6 Body and Mind\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/monk_monk_class_features ` |
| ` implementation ` | ` dnd2024:rules_monk_monk_class_features ` |

### Parameters

#### Class definition

<!-- splat:values path=splat_rule/parameters/class_definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/monk ` |

#### Grant features

<!-- splat:values path=splat_rule/parameters/grant_features -->

| Field | Value |
| --- | --- |
| ` level_basis ` | ` class_level ` |
| ` unlock_when ` | ` class_level_gte_feature_level ` |
| ` from ` | ` class_progression ` |

#### Feature unlocks

<!-- splat:rows path=splat_rule/parameters/feature_unlocks -->

| Row | class_level |
| --- | --- |
| ` 0 ` | ` 10 ` |
| ` 1 ` | ` 10 ` |
| ` 2 ` | ` 13 ` |
| ` 3 ` | ` 14 ` |
| ` 4 ` | ` 15 ` |
| ` 5 ` | ` 18 ` |
| ` 6 ` | ` 19 ` |
| ` 7 ` | ` 1 ` |
| ` 8 ` | ` 1 ` |
| ` 9 ` | ` 20 ` |
| ` 10 ` | ` 2 ` |
| ` 11 ` | ` 2 ` |
| ` 12 ` | ` 2 ` |
| ` 13 ` | ` 3 ` |
| ` 14 ` | ` 3 ` |
| ` 15 ` | ` 4 ` |
| ` 16 ` | ` 4 ` |
| ` 17 ` | ` 5 ` |
| ` 18 ` | ` 5 ` |
| ` 19 ` | ` 6 ` |
| ` 20 ` | ` 7 ` |
| ` 21 ` | ` 9 ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_10_heightened_focus ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_10_self_restoration ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/2/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_13_deflect_energy ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/3/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_14_disciplined_survivor ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/4/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_15_perfect_focus ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/5/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_18_superior_defense ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/6/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_19_epic_boon ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/7/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_1_martial_arts ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/8/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_1_unarmored_defense ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/9/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_20_body_and_mind ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/10/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_2_monk_s_focus ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/11/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_2_unarmored_movement ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/12/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_2_uncanny_metabolism ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/13/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_3_deflect_attacks ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/14/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_3_monk_subclass ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/15/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_4_ability_score_improvement ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/16/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_4_slow_fall ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/17/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_5_extra_attack ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/18/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_5_stunning_strike ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/19/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_6_empowered_strikes ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/20/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_7_evasion ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/21/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_9_acrobatic_movement ` |

#### Recurring feature levels

<!-- splat:values path=splat_rule/parameters/recurring_feature_levels -->

| Field | Value |
| --- | --- |
| ` ability_score_improvement ` | ` [4, 8, 12, 16] ` |
| ` subclass ` | ` [3, 6, 11, 17] ` |
