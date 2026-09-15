---
splat_schema: 1
splat_id: ab1a5798-8113-5d7d-9f4c-b409d5b76886
splat_kind: rule
splat_key: dnd2024:rules/cleric_cleric_class_features
splat_name: Cleric Class Features
splat_origin:
  key: dnd2024:reference/pages/036
splat_sources:
- title: SRD 5.2.1
  locator: pp. 36
  url: https://www.dndbeyond.com/srd
---

# Cleric Class Features

## Source text

<!-- SRD page 36 -->

As a Cleric, you gain the following class features\
when you reach the specified Cleric levels. These\
features are listed in the Cleric Features table.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/cleric_cleric_class_features ` |
| ` implementation ` | ` dnd2024:rules_cleric_cleric_class_features ` |

### Parameters

#### Class definition

<!-- splat:values path=splat_rule/parameters/class_definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/cleric ` |

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
| ` 1 ` | ` 14 ` |
| ` 2 ` | ` 19 ` |
| ` 3 ` | ` 1 ` |
| ` 4 ` | ` 1 ` |
| ` 5 ` | ` 20 ` |
| ` 6 ` | ` 2 ` |
| ` 7 ` | ` 3 ` |
| ` 8 ` | ` 4 ` |
| ` 9 ` | ` 5 ` |
| ` 10 ` | ` 7 ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/cleric_level_10_divine_intervention ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/cleric_level_14_improved_blessed_strikes ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/2/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/cleric_level_19_epic_boon ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/3/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/cleric_level_1_divine_order ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/4/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/cleric_level_1_spellcasting ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/5/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/cleric_level_20_greater_divine_intervention ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/6/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/cleric_level_2_channel_divinity ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/7/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/cleric_level_3_cleric_subclass ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/8/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/cleric_level_4_ability_score_improvement ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/9/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/cleric_level_5_sear_undead ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/10/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/cleric_level_7_blessed_strikes ` |
