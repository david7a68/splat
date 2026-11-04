---
splat_schema: 2
splat_id: e29ea5b8-5087-51fb-aa95-c7e475fb5152
splat_kind: rule
splat_key: dnd2024:rules/travel_pace
splat_name: Travel Pace
splat_origin: dnd2024:reference/pages/192
splat_sources:
- SRD 5.2.1, pp. 192, https://www.dndbeyond.com/srd
---

# Travel Pace

## Source text

<!-- SRD page 192 -->

A group of characters can travel overland at a Normal, Fast, or Slow pace, as described in “Playing the Game.” During any journey stage, the predominant terrain determines the characters’ maximum travel pace, as shown in the Maximum Pace column of the Travel Terrain table. Certain factors can affect a group’s travel pace.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/travel_pace ` |
| ` implementation ` | ` dnd2024:rules_travel_pace ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` paces ` | ` ["normal", "fast", "slow"] ` |
| ` maximum_determined_by ` | ` predominant_terrain_per_journey_stage ` |
| ` arctic_fast_pace_requires_appropriate_equipment ` | ` true ` |
| ` waterborne_uses_vehicle_speed ` | ` true ` |

#### Terrain table

<!-- splat:table
path: splat_rule/parameters/terrain_table
key: Key
order:
- row_1
- row_2
- row_3
- row_4
- row_5
- row_6
- row_7
- row_8
- row_9
- row_10
- row_11
columns:
  Key:
    type: key
  Terrain:
    path: terrain
    type: symbol
  Maximum pace:
    path: maximum_pace
    type: symbol
  Encounter distance ft dice count:
    path: encounter_distance_ft/dice/count
    type: integer
  Encounter distance ft dice sides:
    path: encounter_distance_ft/dice/sides
    type: integer
  Encounter distance ft multiplier:
    path: encounter_distance_ft/multiplier
    type: integer
  Foraging dc:
    path: foraging_dc
    type: integer
  Navigation dc:
    path: navigation_dc
    type: integer
  Search dc:
    path: search_dc
    type: integer
-->

| Key | Terrain | Maximum pace | Encounter distance ft dice count | Encounter distance ft dice sides | Encounter distance ft multiplier | Foraging dc | Navigation dc | Search dc |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` Arctic ` | ` Fast ` | ` 6 ` | ` 6 ` | ` 10 ` | ` 20 ` | ` 10 ` | ` 10 ` |
| ` row_2 ` | ` Coastal ` | ` Normal ` | ` 2 ` | ` 10 ` | ` 10 ` | ` 10 ` | ` 5 ` | ` 15 ` |
| ` row_3 ` | ` Desert ` | ` Normal ` | ` 6 ` | ` 6 ` | ` 10 ` | ` 20 ` | ` 10 ` | ` 10 ` |
| ` row_4 ` | ` Forest ` | ` Normal ` | ` 2 ` | ` 8 ` | ` 10 ` | ` 10 ` | ` 15 ` | ` 15 ` |
| ` row_5 ` | ` Grassland ` | ` Fast ` | ` 6 ` | ` 6 ` | ` 10 ` | ` 15 ` | ` 5 ` | ` 15 ` |
| ` row_6 ` | ` Hill ` | ` Normal ` | ` 2 ` | ` 10 ` | ` 10 ` | ` 15 ` | ` 10 ` | ` 15 ` |
| ` row_7 ` | ` Mountain ` | ` Slow ` | ` 4 ` | ` 10 ` | ` 10 ` | ` 20 ` | ` 15 ` | ` 20 ` |
| ` row_8 ` | ` Swamp ` | ` Slow ` | ` 2 ` | ` 8 ` | ` 10 ` | ` 10 ` | ` 15 ` | ` 20 ` |
| ` row_9 ` | ` Underdark ` | ` Normal ` | ` 2 ` | ` 6 ` | ` 10 ` | ` 20 ` | ` 10 ` | ` 20 ` |
| ` row_10 ` | ` Urban ` | ` Normal ` | ` 2 ` | ` 6 ` | ` 10 ` | ` 20 ` | ` 15 ` | ` 15 ` |
| ` row_11 ` | ` Waterborne ` | ` Vehicle speed ` | ` 6 ` | ` 6 ` | ` 10 ` | ` 15 ` | ` 10 ` | ` 15 ` |
