---
splat_schema: 1
splat_id: 8c1a7c72-b2d4-5cae-90b9-f383b5ecd878
splat_kind: rule
splat_key: dnd2024:rules/wizard_as_a_level_1_character
splat_name: As a Level 1 Character
splat_origin:
  key: dnd2024:reference/pages/077
splat_sources:
- title: SRD 5.2.1
  locator: pp. 77
  url: https://www.dndbeyond.com/srd
---

# As a Level 1 Character

## Source text

<!-- SRD page 77 -->

•  Gain all the traits in the Core Wizard Traits table.\
•  Gain the Wizard’s level 1 features, which are\
listed in the Wizard Features table.\
Wizard Features\
Proficiency\
Level Bonus Class Features Cantrips\
1 +2 Spellcasting, Ritual Adept,\
Arcane Recovery\
2 +2 Scholar\
3 +2 Wizard Subclass\
4 +2 Ability Score Improvement\
5 +3 Memorize Spell\
6 +3 Subclass feature\
7 +3 —\
8 +3 Ability Score Improvement\
9 +4 —\
10 +4 Subclass feature\
11 +4 —\
12 +4 Ability Score Improvement\
13 +5 —\
14 +5 Subclass feature\
15 +5 —\
16 +5 Ability Score Improvement\
17 +6 —\
18 +6 Spell Mastery\
19 +6 Epic Boon\
20 +6 Signature Spells\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/wizard_as_a_level_1_character ` |
| ` implementation ` | ` dnd2024:rules_wizard_as_a_level_1_character ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` grant_core_traits ` | ` all ` |

#### Class definition

<!-- splat:values path=splat_rule/parameters/class_definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/wizard ` |

#### Grant features

<!-- splat:values path=splat_rule/parameters/grant_features -->

| Field | Value |
| --- | --- |
| ` class_level ` | ` 1 ` |
| ` from ` | ` class_definition ` |

#### Feature unlocks

<!-- splat:rows path=splat_rule/parameters/feature_unlocks -->

| Row | class_level |
| --- | --- |
| ` 0 ` | ` 18 ` |
| ` 1 ` | ` 19 ` |
| ` 2 ` | ` 1 ` |
| ` 3 ` | ` 1 ` |
| ` 4 ` | ` 1 ` |
| ` 5 ` | ` 20 ` |
| ` 6 ` | ` 2 ` |
| ` 7 ` | ` 3 ` |
| ` 8 ` | ` 4 ` |
| ` 9 ` | ` 5 ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/wizard_level_18_spell_mastery ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/wizard_level_19_epic_boon ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/2/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/wizard_level_1_arcane_recovery ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/3/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/wizard_level_1_ritual_adept ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/4/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/wizard_level_1_spellcasting ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/5/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/wizard_level_20_signature_spells ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/6/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/wizard_level_2_scholar ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/7/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/wizard_level_3_wizard_subclass ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/8/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/wizard_level_4_ability_score_improvement ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_unlocks/9/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/wizard_level_5_memorize_spell ` |

#### Recurring feature levels

<!-- splat:values path=splat_rule/parameters/recurring_feature_levels -->

| Field | Value |
| --- | --- |
| ` ability_score_improvement ` | ` [4, 8, 12, 16] ` |
| ` subclass ` | ` [3, 6, 10, 14] ` |
