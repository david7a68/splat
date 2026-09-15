---
splat_schema: 1
splat_id: ed577dd9-19e9-5948-81fc-a74e88e4e5f6
splat_kind: rule
splat_key: dnd2024:rules/combat_difficult_terrain
splat_name: Difficult Terrain
splat_origin:
  key: dnd2024:reference/pages/014
splat_sources:
- title: SRD 5.2.1
  locator: pp. 14
  url: https://www.dndbeyond.com/srd
---

# Difficult Terrain

## Source text

<!-- SRD page 14 -->

Combatants are often slowed down by Difficult\
Terrain. Low furniture, rubble, undergrowth, steep\
stairs, snow, and shallow bogs are examples of Diffi-\
cult Terrain.\
Every foot of movement in Difficult Terrain costs\
1 extra foot, even if multiple things in a space count\
as Difficult Terrain.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/combat_difficult_terrain ` |
| ` implementation ` | ` dnd2024:rule_combat_difficult_terrain ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` movement_cost_extra_feet_per_foot ` | ` 1 ` |
| ` multiple_sources_stack ` | ` false ` |
