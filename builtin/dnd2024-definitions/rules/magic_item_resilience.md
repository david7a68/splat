---
splat_schema: 2
splat_id: a60ce9cd-15e0-5c4f-8ef5-3bff5d68721f
splat_kind: rule
splat_key: dnd2024:rules/magic_item_resilience
splat_name: Magic Item Resilience
splat_origin: dnd2024:reference/pages/206
splat_sources:
- SRD 5.2.1, pp. 206, https://www.dndbeyond.com/srd
---

# Magic Item Resilience

## Source text

<!-- SRD page 206 -->

A magic item is at least as durable as a nonmagical item of its kind. Most magic items, other than Potions and Scrolls, have Resistance to all damage.

An Artifact can be destroyed only in some special way. Otherwise, it is impervious to damage. Learning how to destroy an Artifact usually requires research or the completion of a quest.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/magic_item_resilience ` |
| ` implementation ` | ` dnd2024:rules_magic_item_resilience ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` durability_minimum ` | ` nonmagical_equivalent ` |
| ` default_resistance ` | ` all_damage ` |
| ` default_resistance_exceptions ` | ` ["potions", "scrolls"] ` |

#### Artifact

<!-- splat:values path=splat_rule/parameters/artifact -->

| Field | Value |
| --- | --- |
| ` damage_immunity ` | ` true ` |
| ` destruction_requires ` | ` artifact_specific_method ` |
