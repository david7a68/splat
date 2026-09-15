---
splat_schema: 1
splat_id: f2ba5c21-ec3d-517e-9038-6e49be5d389f
splat_kind: rule
splat_key: dnd2024:rules/cleric_as_a_level_1_character
splat_name: As a Level 1 Character
splat_origin:
  key: dnd2024:reference/pages/036
splat_sources:
- title: SRD 5.2.1
  locator: pp. 36
  url: https://www.dndbeyond.com/srd
---

# As a Level 1 Character

## Source text

<!-- SRD page 36 -->

•  Gain all the traits in the Core Cleric Traits table.\
•  Gain the Cleric’s level 1 features, which are listed\
in the Cleric Features table.\
Cleric Features\
Proficiency Channel\
Level Bonus Class Features Divinity\
1 +2 Spellcasting, Divine Order —\
2 +2 Channel Divinity 2\
3 +2 Cleric Subclass 2\
4 +2 Ability Score Improvement 2\
5 +3 Sear Undead 2\
6 +3 Subclass feature 3\
7 +3 Blessed Strikes 3\
8 +3 Ability Score Improvement 3\
9 +4 — 3\
10 +4 Divine Intervention 3\
11 +4 — 3\
12 +4 Ability Score Improvement 3\
13 +5 — 3\
14 +5 Improved Blessed Strikes 3\
15 +5 — 3\
16 +5 Ability Score Improvement 3\
17 +6 Subclass feature 3\
18 +6 — 4\
19 +6 Epic Boon 4\
20 +6 Greater Divine Intervention 4\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/cleric_as_a_level_1_character ` |
| ` implementation ` | ` dnd2024:rules_cleric_as_a_level_1_character ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` requires_total_character_level ` | ` 1 ` |
| ` grant_all_core_traits ` | ` true ` |
| ` subclass_feature_levels ` | ` [3, 6, 17] ` |

#### Core traits

<!-- splat:values path=splat_rule/parameters/core_traits -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/cleric ` |

#### Features

<!-- splat:rows path=splat_rule/parameters/features -->

| Row | level |
| --- | --- |
| ` 0 ` | ` 1 ` |
| ` 1 ` | ` 1 ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/features/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/cleric_level_1_divine_order ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/features/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/cleric_level_1_spellcasting ` |

#### Progression

<!-- splat:rows path=splat_rule/parameters/progression -->

| Row | level | proficiency_bonus | channel_divinity_uses |
| --- | --- | --- | --- |
| ` 0 ` | ` 1 ` | ` 2 ` | ` 0 ` |
| ` 1 ` | ` 2 ` | ` 2 ` | ` 2 ` |
| ` 2 ` | ` 3 ` | ` 2 ` | ` 2 ` |
| ` 3 ` | ` 4 ` | ` 2 ` | ` 2 ` |
| ` 4 ` | ` 5 ` | ` 3 ` | ` 2 ` |
| ` 5 ` | ` 6 ` | ` 3 ` | ` 3 ` |
| ` 6 ` | ` 7 ` | ` 3 ` | ` 3 ` |
| ` 7 ` | ` 8 ` | ` 3 ` | ` 3 ` |
| ` 8 ` | ` 9 ` | ` 4 ` | ` 3 ` |
| ` 9 ` | ` 10 ` | ` 4 ` | ` 3 ` |
| ` 10 ` | ` 11 ` | ` 4 ` | ` 3 ` |
| ` 11 ` | ` 12 ` | ` 4 ` | ` 3 ` |
| ` 12 ` | ` 13 ` | ` 5 ` | ` 3 ` |
| ` 13 ` | ` 14 ` | ` 5 ` | ` 3 ` |
| ` 14 ` | ` 15 ` | ` 5 ` | ` 3 ` |
| ` 15 ` | ` 16 ` | ` 5 ` | ` 3 ` |
| ` 16 ` | ` 17 ` | ` 6 ` | ` 3 ` |
| ` 17 ` | ` 18 ` | ` 6 ` | ` 4 ` |
| ` 18 ` | ` 19 ` | ` 6 ` | ` 4 ` |
| ` 19 ` | ` 20 ` | ` 6 ` | ` 4 ` |

##### Features

<!-- splat:rows path=splat_rule/parameters/progression/0/features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/cleric_level_1_divine_order ` |
| ` 1 ` | ` dnd2024:class_features/cleric_level_1_spellcasting ` |

##### Features

<!-- splat:rows path=splat_rule/parameters/progression/1/features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/cleric_level_2_channel_divinity ` |

##### Features

<!-- splat:rows path=splat_rule/parameters/progression/2/features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/cleric_level_3_cleric_subclass ` |

##### Features

<!-- splat:rows path=splat_rule/parameters/progression/3/features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/cleric_level_4_ability_score_improvement ` |

##### Features

<!-- splat:rows path=splat_rule/parameters/progression/4/features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/cleric_level_5_sear_undead ` |

##### Features

<!-- splat:yaml path=splat_rule/parameters/progression/5/features -->

```yaml
[]
```

##### Features

<!-- splat:rows path=splat_rule/parameters/progression/6/features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/cleric_level_7_blessed_strikes ` |

##### Features

<!-- splat:yaml path=splat_rule/parameters/progression/7/features -->

```yaml
[]
```

##### Features

<!-- splat:yaml path=splat_rule/parameters/progression/8/features -->

```yaml
[]
```

##### Features

<!-- splat:rows path=splat_rule/parameters/progression/9/features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/cleric_level_10_divine_intervention ` |

##### Features

<!-- splat:yaml path=splat_rule/parameters/progression/10/features -->

```yaml
[]
```

##### Features

<!-- splat:yaml path=splat_rule/parameters/progression/11/features -->

```yaml
[]
```

##### Features

<!-- splat:yaml path=splat_rule/parameters/progression/12/features -->

```yaml
[]
```

##### Features

<!-- splat:rows path=splat_rule/parameters/progression/13/features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/cleric_level_14_improved_blessed_strikes ` |

##### Features

<!-- splat:yaml path=splat_rule/parameters/progression/14/features -->

```yaml
[]
```

##### Features

<!-- splat:yaml path=splat_rule/parameters/progression/15/features -->

```yaml
[]
```

##### Features

<!-- splat:yaml path=splat_rule/parameters/progression/16/features -->

```yaml
[]
```

##### Features

<!-- splat:yaml path=splat_rule/parameters/progression/17/features -->

```yaml
[]
```

##### Features

<!-- splat:rows path=splat_rule/parameters/progression/18/features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/cleric_level_19_epic_boon ` |

##### Features

<!-- splat:rows path=splat_rule/parameters/progression/19/features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/cleric_level_20_greater_divine_intervention ` |
