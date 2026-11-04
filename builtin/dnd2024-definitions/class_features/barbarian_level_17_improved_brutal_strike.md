---
splat_schema: 2
splat_id: 31ef89c0-c86f-5759-903b-47d89c3ff98f
splat_kind: rule
splat_key: dnd2024:class_features/barbarian_level_17_improved_brutal_strike
splat_name: 'Level 17: Improved Brutal Strike'
splat_origin: dnd2024:reference/pages/030
splat_sources:
- SRD 5.2.1, pp. 30, https://www.dndbeyond.com/srd
---

# Level 17: Improved Brutal Strike

## Source text

<!-- SRD page 30 -->

The extra damage of your Brutal Strike increases to 2d10. In addition, you can use two different Brutal Strike effects whenever you use your Brutal Strike feature.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/barbarian_level_17_improved_brutal_strike ` |
| ` implementation ` | ` dnd2024:class_features_barbarian_level_17_improved_brutal_strike ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` effects_chosen ` | ` 2 ` |
| ` effects_distinct ` | ` true ` |

#### Modifies

<!-- splat:values path=splat_rule/parameters/modifies -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/barbarian_level_9_brutal_strike ` |

#### Extra damage dice

<!-- splat:values path=splat_rule/parameters/extra_damage_dice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 2 ` |
| ` sides ` | ` 10 ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` barbarian ` |
| ` minimum_level ` | ` 17 ` |
