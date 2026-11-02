---
splat_schema: 1
splat_id: 2e69a4b7-6bb3-5f25-8df7-ddcd6301a929
splat_kind: rule
splat_key: dnd2024:class_features/bard_level_20_words_of_creation
splat_name: 'Level 20: Words of Creation'
splat_origin:
  key: dnd2024:reference/pages/033
splat_sources:
- title: SRD 5.2.1
  locator: pp. 33
  url: https://www.dndbeyond.com/srd
---

# Level 20: Words of Creation

## Source text

<!-- SRD page 33 -->

You have mastered two of the Words of Creation: the words of life and death. You therefore always have the Power Word Heal and Power Word Kill spells prepared. When you cast either spell, you can target a second creature with it if that creature is within 10 feet of the first target.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/bard_level_20_words_of_creation ` |
| ` implementation ` | ` dnd2024:class_features_bard_level_20_words_of_creation ` |

### Parameters

#### Always prepared

<!-- splat:rows path=splat_rule/parameters/always_prepared -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/power_word_heal ` |
| ` 1 ` | ` dnd2024:spells/power_word_kill ` |

#### When casting these spells

<!-- splat:values path=splat_rule/parameters/when_casting_these_spells -->

| Field | Value |
| --- | --- |
| ` optional_additional_creature_targets ` | ` 1 ` |
| ` additional_target_within_feet_of_first ` | ` 10 ` |
