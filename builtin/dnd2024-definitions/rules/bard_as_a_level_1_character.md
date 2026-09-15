---
splat_schema: 1
splat_id: 4df2c733-0a44-549c-8f55-e4f5870390ad
splat_kind: rule
splat_key: dnd2024:rules/bard_as_a_level_1_character
splat_name: As a Level 1 Character
splat_origin:
  key: dnd2024:reference/pages/031
splat_sources:
- title: SRD 5.2.1
  locator: pp. 31
  url: https://www.dndbeyond.com/srd
---

# As a Level 1 Character

## Source text

<!-- SRD page 31 -->

•  Gain all the traits in the Core Bard Traits table.\
Bard Features\
Proficiency Bardic\
Level Bonus Class Features Die\
1 +2 Bardic Inspiration, D6\
Spellcasting\
2 +2 Expertise, D6\
Jack of All Trades\
3 +2 Bard Subclass D6\
4 +2 Ability Score Improvement D6\
5 +3 Font of Inspiration D8\
6 +3 Subclass feature D8\
7 +3 Countercharm D8\
8 +3 Ability Score Improvement D8\
9 +4 Expertise D8\
10 +4 Magical Secrets D10\
11 +4 — D10\
12 +4 Ability Score Improvement D10\
13 +5 — D10\
14 +5 Subclass feature D10\
15 +5 — D12\
16 +5 Ability Score Improvement D12\
17 +6 — D12\
18 +6 Superior Inspiration D12\
19 +6 Epic Boon D12\
20 +6 Words of Creation D12\
•  Gain the Bard’s level 1 features, which are listed\
in the Bard Features table.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/bard_as_a_level_1_character ` |
| ` implementation ` | ` dnd2024:rules_bard_as_a_level_1_character ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` requires_total_character_level ` | ` 1 ` |
| ` grant_all_core_traits ` | ` true ` |
| ` subclass_feature_levels ` | ` [3, 6, 14] ` |

#### Core traits

<!-- splat:values path=splat_rule/parameters/core_traits -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/bard ` |

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
| ` key ` | ` dnd2024:class_features/bard_level_1_bardic_inspiration ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/features/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/bard_level_1_spellcasting ` |

#### Progression

<!-- splat:rows path=splat_rule/parameters/progression -->

| Row | level | proficiency_bonus | bardic_die_sides |
| --- | --- | --- | --- |
| ` 0 ` | ` 1 ` | ` 2 ` | ` 6 ` |
| ` 1 ` | ` 2 ` | ` 2 ` | ` 6 ` |
| ` 2 ` | ` 3 ` | ` 2 ` | ` 6 ` |
| ` 3 ` | ` 4 ` | ` 2 ` | ` 6 ` |
| ` 4 ` | ` 5 ` | ` 3 ` | ` 8 ` |
| ` 5 ` | ` 6 ` | ` 3 ` | ` 8 ` |
| ` 6 ` | ` 7 ` | ` 3 ` | ` 8 ` |
| ` 7 ` | ` 8 ` | ` 3 ` | ` 8 ` |
| ` 8 ` | ` 9 ` | ` 4 ` | ` 8 ` |
| ` 9 ` | ` 10 ` | ` 4 ` | ` 10 ` |
| ` 10 ` | ` 11 ` | ` 4 ` | ` 10 ` |
| ` 11 ` | ` 12 ` | ` 4 ` | ` 10 ` |
| ` 12 ` | ` 13 ` | ` 5 ` | ` 10 ` |
| ` 13 ` | ` 14 ` | ` 5 ` | ` 10 ` |
| ` 14 ` | ` 15 ` | ` 5 ` | ` 12 ` |
| ` 15 ` | ` 16 ` | ` 5 ` | ` 12 ` |
| ` 16 ` | ` 17 ` | ` 6 ` | ` 12 ` |
| ` 17 ` | ` 18 ` | ` 6 ` | ` 12 ` |
| ` 18 ` | ` 19 ` | ` 6 ` | ` 12 ` |
| ` 19 ` | ` 20 ` | ` 6 ` | ` 12 ` |

##### Features

<!-- splat:rows path=splat_rule/parameters/progression/0/features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/bard_level_1_bardic_inspiration ` |
| ` 1 ` | ` dnd2024:class_features/bard_level_1_spellcasting ` |

##### Features

<!-- splat:rows path=splat_rule/parameters/progression/1/features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/bard_level_2_expertise ` |
| ` 1 ` | ` dnd2024:class_features/bard_level_2_jack_of_all_trades ` |

##### Features

<!-- splat:rows path=splat_rule/parameters/progression/2/features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/bard_level_3_bard_subclass ` |

##### Features

<!-- splat:rows path=splat_rule/parameters/progression/3/features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/bard_level_4_ability_score_improvement ` |

##### Features

<!-- splat:rows path=splat_rule/parameters/progression/4/features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/bard_level_5_font_of_inspiration ` |

##### Features

<!-- splat:yaml path=splat_rule/parameters/progression/5/features -->

```yaml
[]
```

##### Features

<!-- splat:rows path=splat_rule/parameters/progression/6/features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/bard_level_7_countercharm ` |

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
| ` 0 ` | ` dnd2024:class_features/bard_level_10_magical_secrets ` |

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

<!-- splat:yaml path=splat_rule/parameters/progression/13/features -->

```yaml
[]
```

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

<!-- splat:rows path=splat_rule/parameters/progression/17/features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/bard_level_18_superior_inspiration ` |

##### Features

<!-- splat:rows path=splat_rule/parameters/progression/18/features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/bard_level_19_epic_boon ` |

##### Features

<!-- splat:rows path=splat_rule/parameters/progression/19/features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/bard_level_20_words_of_creation ` |
