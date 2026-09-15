---
splat_schema: 1
splat_id: 877e3432-81e8-5958-9f61-1687c16c48c9
splat_kind: rule
splat_key: dnd2024:rules/sorcerer_sorcerer_class_features
splat_name: Sorcerer Class Features
splat_origin:
  key: dnd2024:reference/pages/064
splat_sources:
- title: SRD 5.2.1
  locator: pp. 64
  url: https://www.dndbeyond.com/srd
---

# Sorcerer Class Features

## Source text

<!-- SRD page 64 -->

As a Sorcerer, you gain the following class features\
when you reach the specified Sorcerer levels. These\
features are listed in the Sorcerer Features table.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/sorcerer_sorcerer_class_features ` |
| ` implementation ` | ` dnd2024:rules_sorcerer_sorcerer_class_features ` |

### Parameters

#### Class definition

<!-- splat:values path=splat_rule/parameters/class_definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/sorcerer ` |

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
| ` 0 ` | ` 19 ` |
| ` 1 ` | ` 1 ` |
| ` 2 ` | ` 1 ` |
| ` 3 ` | ` 20 ` |
| ` 4 ` | ` 2 ` |
| ` 5 ` | ` 2 ` |
| ` 6 ` | ` 3 ` |
| ` 7 ` | ` 4 ` |
| ` 8 ` | ` 5 ` |
| ` 9 ` | ` 7 ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/sorcerer_level_19_epic_boon ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/sorcerer_level_1_innate_sorcery ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/2/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/sorcerer_level_1_spellcasting ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/3/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/sorcerer_level_20_arcane_apotheosis ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/4/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/sorcerer_level_2_font_of_magic ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/5/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/sorcerer_level_2_metamagic ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/6/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/sorcerer_level_3_sorcerer_subclass ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/7/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/sorcerer_level_4_ability_score_improvement ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/8/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/sorcerer_level_5_sorcerous_restoration ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/9/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/sorcerer_level_7_sorcery_incarnate ` |
