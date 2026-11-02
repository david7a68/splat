---
splat_schema: 1
splat_id: 7881a225-a35d-5332-b737-862468563258
splat_kind: rule
splat_key: dnd2024:class_features/oath_of_devotion_level_7_aura_of_devotion
splat_name: 'Level 7: Aura of Devotion'
splat_origin:
  key: dnd2024:reference/pages/056
splat_sources:
- title: SRD 5.2.1
  locator: pp. 56
  url: https://www.dndbeyond.com/srd
---

# Level 7: Aura of Devotion

## Source text

<!-- SRD page 56 -->

You and your allies have Immunity to the Charmed condition while in your Aura of Protection. If a Charmed ally enters the aura, that condition has no effect on that ally while there.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/oath_of_devotion_level_7_aura_of_devotion ` |
| ` implementation ` | ` dnd2024:class_features_oath_of_devotion_level_7_aura_of_devotion ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` aura ` | ` own_aura_of_protection ` |
| ` targets ` | ` ["self", "allies"] ` |
| ` condition_immunity ` | ` charmed ` |
| ` existing_condition ` | ` suppressed_while_in_aura ` |
