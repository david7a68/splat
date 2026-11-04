---
splat_schema: 2
splat_id: 877e3432-81e8-5958-9f61-1687c16c48c9
splat_kind: rule
splat_key: dnd2024:rules/sorcerer_sorcerer_class_features
splat_name: Sorcerer Class Features
splat_origin: dnd2024:reference/pages/064
splat_sources:
- SRD 5.2.1, pp. 64, https://www.dndbeyond.com/srd
---

# Sorcerer Class Features

## Source text

<!-- SRD page 64 -->

As a Sorcerer, you gain the following class features when you reach the specified Sorcerer levels. These features are listed in the Sorcerer Features table.

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

<!-- splat:table
path: splat_rule/parameters/feature_unlocks
key: Feature
order:
- sorcerer_level_19_epic_boon
- sorcerer_level_1_innate_sorcery
- sorcerer_level_1_spellcasting
- sorcerer_level_20_arcane_apotheosis
- sorcerer_level_2_font_of_magic
- sorcerer_level_2_metamagic
- sorcerer_level_3_sorcerer_subclass
- sorcerer_level_4_ability_score_improvement
- sorcerer_level_5_sorcerous_restoration
- sorcerer_level_7_sorcery_incarnate
columns:
  Feature:
    path: feature
    type: reference
  Class level:
    path: class_level
    type: integer
-->

| Feature | Class level |
| --- | --- |
| [Level 19: Epic Boon](<../class_features/sorcerer_level_19_epic_boon.md> "dnd2024:class_features/sorcerer_level_19_epic_boon") | ` 19 ` |
| [Level 1: Innate Sorcery](<../class_features/sorcerer_level_1_innate_sorcery.md> "dnd2024:class_features/sorcerer_level_1_innate_sorcery") | ` 1 ` |
| [Level 1: Spellcasting](<../class_features/sorcerer_level_1_spellcasting.md> "dnd2024:class_features/sorcerer_level_1_spellcasting") | ` 1 ` |
| [Level 20: Arcane Apotheosis](<../class_features/sorcerer_level_20_arcane_apotheosis.md> "dnd2024:class_features/sorcerer_level_20_arcane_apotheosis") | ` 20 ` |
| [Level 2: Font of Magic](<../class_features/sorcerer_level_2_font_of_magic.md> "dnd2024:class_features/sorcerer_level_2_font_of_magic") | ` 2 ` |
| [Level 2: Metamagic](<../class_features/sorcerer_level_2_metamagic.md> "dnd2024:class_features/sorcerer_level_2_metamagic") | ` 2 ` |
| [Level 3: Sorcerer Subclass](<../class_features/sorcerer_level_3_sorcerer_subclass.md> "dnd2024:class_features/sorcerer_level_3_sorcerer_subclass") | ` 3 ` |
| [Level 4: Ability Score Improvement](<../class_features/sorcerer_level_4_ability_score_improvement.md> "dnd2024:class_features/sorcerer_level_4_ability_score_improvement") | ` 4 ` |
| [Level 5: Sorcerous Restoration](<../class_features/sorcerer_level_5_sorcerous_restoration.md> "dnd2024:class_features/sorcerer_level_5_sorcerous_restoration") | ` 5 ` |
| [Level 7: Sorcery Incarnate](<../class_features/sorcerer_level_7_sorcery_incarnate.md> "dnd2024:class_features/sorcerer_level_7_sorcery_incarnate") | ` 7 ` |
