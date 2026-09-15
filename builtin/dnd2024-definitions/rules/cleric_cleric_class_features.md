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

<!-- splat:table
path: splat_rule/parameters/feature_unlocks
key: Feature
order:
- cleric_level_10_divine_intervention
- cleric_level_14_improved_blessed_strikes
- cleric_level_19_epic_boon
- cleric_level_1_divine_order
- cleric_level_1_spellcasting
- cleric_level_20_greater_divine_intervention
- cleric_level_2_channel_divinity
- cleric_level_3_cleric_subclass
- cleric_level_4_ability_score_improvement
- cleric_level_5_sear_undead
- cleric_level_7_blessed_strikes
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
| [Level 10: Divine Intervention](<../class_features/cleric_level_10_divine_intervention.md> "dnd2024:class_features/cleric_level_10_divine_intervention") | ` 10 ` |
| [Level 14: Improved Blessed Strikes](<../class_features/cleric_level_14_improved_blessed_strikes.md> "dnd2024:class_features/cleric_level_14_improved_blessed_strikes") | ` 14 ` |
| [Level 19: Epic Boon](<../class_features/cleric_level_19_epic_boon.md> "dnd2024:class_features/cleric_level_19_epic_boon") | ` 19 ` |
| [Level 1: Divine Order](<../class_features/cleric_level_1_divine_order.md> "dnd2024:class_features/cleric_level_1_divine_order") | ` 1 ` |
| [Level 1: Spellcasting](<../class_features/cleric_level_1_spellcasting.md> "dnd2024:class_features/cleric_level_1_spellcasting") | ` 1 ` |
| [Level 20: Greater Divine Intervention](<../class_features/cleric_level_20_greater_divine_intervention.md> "dnd2024:class_features/cleric_level_20_greater_divine_intervention") | ` 20 ` |
| [Level 2: Channel Divinity](<../class_features/cleric_level_2_channel_divinity.md> "dnd2024:class_features/cleric_level_2_channel_divinity") | ` 2 ` |
| [Level 3: Cleric Subclass](<../class_features/cleric_level_3_cleric_subclass.md> "dnd2024:class_features/cleric_level_3_cleric_subclass") | ` 3 ` |
| [Level 4: Ability Score Improvement](<../class_features/cleric_level_4_ability_score_improvement.md> "dnd2024:class_features/cleric_level_4_ability_score_improvement") | ` 4 ` |
| [Level 5: Sear Undead](<../class_features/cleric_level_5_sear_undead.md> "dnd2024:class_features/cleric_level_5_sear_undead") | ` 5 ` |
| [Level 7: Blessed Strikes](<../class_features/cleric_level_7_blessed_strikes.md> "dnd2024:class_features/cleric_level_7_blessed_strikes") | ` 7 ` |
