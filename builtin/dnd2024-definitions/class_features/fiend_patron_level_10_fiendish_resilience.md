---
splat_schema: 1
splat_id: d4d4815c-1dc1-5546-81a7-0429135dfedf
splat_kind: rule
splat_key: dnd2024:class_features/fiend_patron_level_10_fiendish_resilience
splat_name: 'Level 10: Fiendish Resilience'
splat_origin:
  key: dnd2024:reference/pages/076
splat_sources:
- title: SRD 5.2.1
  locator: pp. 76
  url: https://www.dndbeyond.com/srd
---

# Level 10: Fiendish Resilience

## Source text

<!-- SRD page 76 -->

Choose one damage type, other than Force, whenever you finish a Short or Long Rest. You have Resistance to that damage type until you choose a different one with this feature.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/fiend_patron_level_10_fiendish_resilience ` |
| ` implementation ` | ` dnd2024:class_features_fiend_patron_level_10_fiendish_resilience ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` grant ` | ` resistance ` |
| ` duration ` | ` until_different_choice ` |

#### Choose damage type

<!-- splat:values path=splat_rule/parameters/choose_damage_type -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` exclude ` | ` ["force"] ` |
| ` events ` | ` ["rest.short_completed", "rest.long_completed"] ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` warlock ` |
| ` minimum_level ` | ` 10 ` |
