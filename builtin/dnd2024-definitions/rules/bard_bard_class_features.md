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

As a Bard, you gain the following class features when you reach the specified Bard levels. These features are listed in the Bard Features table.

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

<!-- splat:table
path: splat_rule/parameters/feature_unlocks
key: Feature
order:
- bard_level_10_magical_secrets
- bard_level_18_superior_inspiration
- bard_level_19_epic_boon
- bard_level_1_bardic_inspiration
- bard_level_1_spellcasting
- bard_level_20_words_of_creation
- bard_level_2_expertise
- bard_level_2_jack_of_all_trades
- bard_level_3_bard_subclass
- bard_level_4_ability_score_improvement
- bard_level_5_font_of_inspiration
- bard_level_7_countercharm
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
| [Level 10: Magical Secrets](<../class_features/bard_level_10_magical_secrets.md> "dnd2024:class_features/bard_level_10_magical_secrets") | ` 10 ` |
| [Level 18: Superior Inspiration](<../class_features/bard_level_18_superior_inspiration.md> "dnd2024:class_features/bard_level_18_superior_inspiration") | ` 18 ` |
| [Level 19: Epic Boon](<../class_features/bard_level_19_epic_boon.md> "dnd2024:class_features/bard_level_19_epic_boon") | ` 19 ` |
| [Level 1: Bardic Inspiration](<../class_features/bard_level_1_bardic_inspiration.md> "dnd2024:class_features/bard_level_1_bardic_inspiration") | ` 1 ` |
| [Level 1: Spellcasting](<../class_features/bard_level_1_spellcasting.md> "dnd2024:class_features/bard_level_1_spellcasting") | ` 1 ` |
| [Level 20: Words of Creation](<../class_features/bard_level_20_words_of_creation.md> "dnd2024:class_features/bard_level_20_words_of_creation") | ` 20 ` |
| [Level 2: Expertise](<../class_features/bard_level_2_expertise.md> "dnd2024:class_features/bard_level_2_expertise") | ` 2 ` |
| [Level 2: Jack of All Trades](<../class_features/bard_level_2_jack_of_all_trades.md> "dnd2024:class_features/bard_level_2_jack_of_all_trades") | ` 2 ` |
| [Level 3: Bard Subclass](<../class_features/bard_level_3_bard_subclass.md> "dnd2024:class_features/bard_level_3_bard_subclass") | ` 3 ` |
| [Level 4: Ability Score Improvement](<../class_features/bard_level_4_ability_score_improvement.md> "dnd2024:class_features/bard_level_4_ability_score_improvement") | ` 4 ` |
| [Level 5: Font of Inspiration](<../class_features/bard_level_5_font_of_inspiration.md> "dnd2024:class_features/bard_level_5_font_of_inspiration") | ` 5 ` |
| [Level 7: Countercharm](<../class_features/bard_level_7_countercharm.md> "dnd2024:class_features/bard_level_7_countercharm") | ` 7 ` |
