---
splat_schema: 1
splat_id: 87aea1b6-3224-5ccb-8dba-46af7a6ca1ae
splat_kind: rule
splat_key: dnd2024:rules/bard_bard_class_features
splat_name: Bard Class Features
splat_origin:
  key: dnd2024:reference/pages/031
splat_sources:
- title: SRD 5.2.1
  locator: pp. 31
  url: https://www.dndbeyond.com/srd
---

# Bard Class Features

## Source text

<!-- SRD page 31 -->

As a Bard, you gain the following class features\
when you reach the specified Bard levels. These fea-\
tures are listed in the Bard Features table.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/bard_bard_class_features ` |
| ` implementation ` | ` dnd2024:rules_bard_bard_class_features ` |

### Parameters

#### Class definition

<!-- splat:values path=splat_rule/parameters/class_definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/bard ` |

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
| ` 1 ` | ` 18 ` |
| ` 2 ` | ` 19 ` |
| ` 3 ` | ` 1 ` |
| ` 4 ` | ` 1 ` |
| ` 5 ` | ` 20 ` |
| ` 6 ` | ` 2 ` |
| ` 7 ` | ` 2 ` |
| ` 8 ` | ` 3 ` |
| ` 9 ` | ` 4 ` |
| ` 10 ` | ` 5 ` |
| ` 11 ` | ` 7 ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/bard_level_10_magical_secrets ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/bard_level_18_superior_inspiration ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/2/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/bard_level_19_epic_boon ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/3/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/bard_level_1_bardic_inspiration ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/4/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/bard_level_1_spellcasting ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/5/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/bard_level_20_words_of_creation ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/6/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/bard_level_2_expertise ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/7/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/bard_level_2_jack_of_all_trades ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/8/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/bard_level_3_bard_subclass ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/9/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/bard_level_4_ability_score_improvement ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/10/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/bard_level_5_font_of_inspiration ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/11/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/bard_level_7_countercharm ` |
