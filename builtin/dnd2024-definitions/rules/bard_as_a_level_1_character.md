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

•  Gain all the traits in the Core Bard Traits table.

•  Gain the Bard’s level 1 features, which are listed in the Bard Features table.

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

<!-- splat:table
path: splat_rule/parameters/features
key: Feature
order:
- bard_level_1_bardic_inspiration
- bard_level_1_spellcasting
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
| [Level 1: Bardic Inspiration](<../class_features/bard_level_1_bardic_inspiration.md> "dnd2024:class_features/bard_level_1_bardic_inspiration") | ` 1 ` |
| [Level 1: Spellcasting](<../class_features/bard_level_1_spellcasting.md> "dnd2024:class_features/bard_level_1_spellcasting") | ` 1 ` |

#### Progression

<!-- splat:table
path: splat_rule/parameters/progression
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
  Bardic die sides:
    path: bardic_die_sides
    type: integer
  Details:
    type: section
    optional: true
-->

| Key | Level | Proficiency bonus | Bardic die sides | Details |
| --- | --- | --- | --- | --- |
| ` row_1 ` | ` 1 ` | ` 2 ` | ` 6 ` | [Rules](#^splat-af862744-row_1 "section:row_1") |
| ` row_2 ` | ` 2 ` | ` 2 ` | ` 6 ` | [Rules](#^splat-af862744-row_2 "section:row_2") |
| ` row_3 ` | ` 3 ` | ` 2 ` | ` 6 ` | [Rules](#^splat-af862744-row_3 "section:row_3") |
| ` row_4 ` | ` 4 ` | ` 2 ` | ` 6 ` | [Rules](#^splat-af862744-row_4 "section:row_4") |
| ` row_5 ` | ` 5 ` | ` 3 ` | ` 8 ` | [Rules](#^splat-af862744-row_5 "section:row_5") |
| ` row_6 ` | ` 6 ` | ` 3 ` | ` 8 ` | [Rules](#^splat-af862744-row_6 "section:row_6") |
| ` row_7 ` | ` 7 ` | ` 3 ` | ` 8 ` | [Rules](#^splat-af862744-row_7 "section:row_7") |
| ` row_8 ` | ` 8 ` | ` 3 ` | ` 8 ` | [Rules](#^splat-af862744-row_8 "section:row_8") |
| ` row_9 ` | ` 9 ` | ` 4 ` | ` 8 ` | [Rules](#^splat-af862744-row_9 "section:row_9") |
| ` row_10 ` | ` 10 ` | ` 4 ` | ` 10 ` | [Rules](#^splat-af862744-row_10 "section:row_10") |
| ` row_11 ` | ` 11 ` | ` 4 ` | ` 10 ` | [Rules](#^splat-af862744-row_11 "section:row_11") |
| ` row_12 ` | ` 12 ` | ` 4 ` | ` 10 ` | [Rules](#^splat-af862744-row_12 "section:row_12") |
| ` row_13 ` | ` 13 ` | ` 5 ` | ` 10 ` | [Rules](#^splat-af862744-row_13 "section:row_13") |
| ` row_14 ` | ` 14 ` | ` 5 ` | ` 10 ` | [Rules](#^splat-af862744-row_14 "section:row_14") |
| ` row_15 ` | ` 15 ` | ` 5 ` | ` 12 ` | [Rules](#^splat-af862744-row_15 "section:row_15") |
| ` row_16 ` | ` 16 ` | ` 5 ` | ` 12 ` | [Rules](#^splat-af862744-row_16 "section:row_16") |
| ` row_17 ` | ` 17 ` | ` 6 ` | ` 12 ` | [Rules](#^splat-af862744-row_17 "section:row_17") |
| ` row_18 ` | ` 18 ` | ` 6 ` | ` 12 ` | [Rules](#^splat-af862744-row_18 "section:row_18") |
| ` row_19 ` | ` 19 ` | ` 6 ` | ` 12 ` | [Rules](#^splat-af862744-row_19 "section:row_19") |
| ` row_20 ` | ` 20 ` | ` 6 ` | ` 12 ` | [Rules](#^splat-af862744-row_20 "section:row_20") |

##### Row 1

^splat-af862744-row_1

###### Features

<!-- splat:rows path=splat_rule/parameters/progression/@row_1/features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/bard_level_1_bardic_inspiration ` |
| ` 1 ` | ` dnd2024:class_features/bard_level_1_spellcasting ` |

##### Row 2

^splat-af862744-row_2

###### Features

<!-- splat:rows path=splat_rule/parameters/progression/@row_2/features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/bard_level_2_expertise ` |
| ` 1 ` | ` dnd2024:class_features/bard_level_2_jack_of_all_trades ` |

##### Row 3

^splat-af862744-row_3

###### Features

<!-- splat:rows path=splat_rule/parameters/progression/@row_3/features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/bard_level_3_bard_subclass ` |

##### Row 4

^splat-af862744-row_4

###### Features

<!-- splat:rows path=splat_rule/parameters/progression/@row_4/features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/bard_level_4_ability_score_improvement ` |

##### Row 5

^splat-af862744-row_5

###### Features

<!-- splat:rows path=splat_rule/parameters/progression/@row_5/features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/bard_level_5_font_of_inspiration ` |

##### Row 6

^splat-af862744-row_6

###### Features

<!-- splat:yaml path=splat_rule/parameters/progression/@row_6/features -->

```yaml
[]
```

##### Row 7

^splat-af862744-row_7

###### Features

<!-- splat:rows path=splat_rule/parameters/progression/@row_7/features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/bard_level_7_countercharm ` |

##### Row 8

^splat-af862744-row_8

###### Features

<!-- splat:yaml path=splat_rule/parameters/progression/@row_8/features -->

```yaml
[]
```

##### Row 9

^splat-af862744-row_9

###### Features

<!-- splat:yaml path=splat_rule/parameters/progression/@row_9/features -->

```yaml
[]
```

##### Row 10

^splat-af862744-row_10

###### Features

<!-- splat:rows path=splat_rule/parameters/progression/@row_10/features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/bard_level_10_magical_secrets ` |

##### Row 11

^splat-af862744-row_11

###### Features

<!-- splat:yaml path=splat_rule/parameters/progression/@row_11/features -->

```yaml
[]
```

##### Row 12

^splat-af862744-row_12

###### Features

<!-- splat:yaml path=splat_rule/parameters/progression/@row_12/features -->

```yaml
[]
```

##### Row 13

^splat-af862744-row_13

###### Features

<!-- splat:yaml path=splat_rule/parameters/progression/@row_13/features -->

```yaml
[]
```

##### Row 14

^splat-af862744-row_14

###### Features

<!-- splat:yaml path=splat_rule/parameters/progression/@row_14/features -->

```yaml
[]
```

##### Row 15

^splat-af862744-row_15

###### Features

<!-- splat:yaml path=splat_rule/parameters/progression/@row_15/features -->

```yaml
[]
```

##### Row 16

^splat-af862744-row_16

###### Features

<!-- splat:yaml path=splat_rule/parameters/progression/@row_16/features -->

```yaml
[]
```

##### Row 17

^splat-af862744-row_17

###### Features

<!-- splat:yaml path=splat_rule/parameters/progression/@row_17/features -->

```yaml
[]
```

##### Row 18

^splat-af862744-row_18

###### Features

<!-- splat:rows path=splat_rule/parameters/progression/@row_18/features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/bard_level_18_superior_inspiration ` |

##### Row 19

^splat-af862744-row_19

###### Features

<!-- splat:rows path=splat_rule/parameters/progression/@row_19/features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/bard_level_19_epic_boon ` |

##### Row 20

^splat-af862744-row_20

###### Features

<!-- splat:rows path=splat_rule/parameters/progression/@row_20/features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/bard_level_20_words_of_creation ` |
