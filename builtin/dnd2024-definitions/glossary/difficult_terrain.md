---
splat_schema: 2
splat_id: fbd3a3b4-5d1f-5725-b7a3-4b41d20ffdb5
splat_kind: rule
splat_key: dnd2024:glossary/difficult_terrain
splat_name: Difficult Terrain
splat_origin: dnd2024:reference/glossary/difficult_terrain
splat_sources:
- SRD 5.2.1, pp. 181, https://www.dndbeyond.com/srd
---

# Difficult Terrain

## Source text

<!-- SRD page 181 -->

If a space is Difficult Terrain, every foot of movement in that space costs 1 extra foot. For example, moving 5 feet through Difficult Terrain costs 10 feet of movement. Difficult Terrain isn’t cumulative; either a space is Difficult Terrain or it isn’t.

A space is Difficult Terrain if the space contains any of the following or something similar:

• A creature that isn’t Tiny or your ally

• Furniture that is sized for creatures of your size or larger

• Heavy snow, ice, rubble, or undergrowth

• Liquid that’s between shin- and waist-deep

• A narrow opening sized for a creature one size smaller than you

• A slope of 20 degrees or more

Original layout: [[../../srd-5.2.1/pages/181|page 181]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/difficult_terrain ` |
| ` implementation ` | ` dnd2024:glossary_difficult_terrain ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` additional_cost_feet_per_foot ` | ` 1 ` |
| ` stacks ` | ` false ` |

#### Qualifying any

<!-- splat:table
path: splat_rule/parameters/qualifying_any
key: Key
order:
- row_1
- row_2
- row_3
- row_4
- row_5
- row_6
columns:
  Key:
    type: key
  Creature present:
    path: creature_present
    type: boolean
    optional: true
  Creature not tiny:
    path: creature_not_tiny
    type: boolean
    optional: true
  Creature not ally:
    path: creature_not_ally
    type: boolean
    optional: true
  Furniture size at least:
    path: furniture_size_at_least
    type: symbol
    optional: true
  Terrain any:
    path: terrain_any
    type: yaml
    optional: true
  Liquid depth min:
    path: liquid_depth_min
    type: symbol
    optional: true
  Liquid depth max:
    path: liquid_depth_max
    type: symbol
    optional: true
  Narrow opening size difference:
    path: narrow_opening_size_difference
    type: integer
    optional: true
  Slope degrees min:
    path: slope_degrees_min
    type: integer
    optional: true
-->

| Key | Creature present | Creature not tiny | Creature not ally | Furniture size at least | Terrain any | Liquid depth min | Liquid depth max | Narrow opening size difference | Slope degrees min |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` row_1 ` | true | true | true |  |  |  |  |  |  |
| ` row_2 ` |  |  |  | ` Traveler size ` |  |  |  |  |  |
| ` row_3 ` |  |  |  |  | ` ["heavy_snow", "ice", "rubble", "undergrowth"] ` |  |  |  |  |
| ` row_4 ` |  |  |  |  |  | ` Shin ` | ` Waist ` |  |  |
| ` row_5 ` |  |  |  |  |  |  |  | ` -1 ` |  |
| ` row_6 ` |  |  |  |  |  |  |  |  | ` 20 ` |

#### Similar terrain

<!-- splat:values path=splat_rule/parameters/similar_terrain -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` gm ` |
| ` result_type ` | ` boolean ` |
| ` similar_to_listed_terrain_required ` | ` true ` |
