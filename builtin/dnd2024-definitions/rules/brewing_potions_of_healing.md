---
splat_schema: 2
splat_id: 34dbfb32-3426-56f7-aac8-6fad5adc572e
splat_kind: rule
splat_key: dnd2024:rules/brewing_potions_of_healing
splat_name: Brewing Potions of Healing
splat_origin: dnd2024:reference/pages/103
splat_sources:
- SRD 5.2.1, pp. 103, https://www.dndbeyond.com/srd
---

# Brewing Potions of Healing

## Source text

<!-- SRD page 103 -->

A character who has proficiency with the Herbalism Kit can create a Potion of Healing. Doing so requires using that kit and 25 GP of raw material over the course of 1 day (8 hours of work).

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/brewing_potions_of_healing ` |
| ` implementation ` | ` dnd2024:rule_brewing_potions_of_healing ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` tool_proficiency_required ` | ` true ` |
| ` tool_use_required ` | ` true ` |
| ` quantity ` | ` 1 ` |
| ` raw_material_gp ` | ` 25 ` |
| ` days ` | ` 1 ` |
| ` work_hours ` | ` 8 ` |

#### Tool

<!-- splat:values path=splat_rule/parameters/tool -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/other_tools_herbalism_kit_5_gp ` |

#### Output

<!-- splat:values path=splat_rule/parameters/output -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/potion_of_healing_50_gp ` |
