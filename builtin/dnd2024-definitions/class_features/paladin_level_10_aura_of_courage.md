---
splat_schema: 1
splat_id: dd1a9d3d-72b4-56d0-8aa3-de6cee106087
splat_kind: rule
splat_key: dnd2024:class_features/paladin_level_10_aura_of_courage
splat_name: 'Level 10: Aura of Courage'
splat_origin:
  key: dnd2024:reference/pages/055
splat_sources:
- title: SRD 5.2.1
  locator: pp. 55
  url: https://www.dndbeyond.com/srd
---

# Level 10: Aura of Courage

## Source text

<!-- SRD page 55 -->

You and your allies have Immunity to the Frightened condition while in your Aura of Protection. If a Frightened ally enters the aura, that condition has no effect on that ally while there.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/paladin_level_10_aura_of_courage ` |
| ` implementation ` | ` dnd2024:class_features_paladin_level_10_aura_of_courage ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` aura ` | ` own_aura_of_protection ` |
| ` targets ` | ` ["self", "allies"] ` |
| ` condition_immunity ` | ` frightened ` |
| ` existing_condition ` | ` suppressed_while_in_aura ` |
