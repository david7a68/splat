---
splat_schema: 1
splat_id: fbd3a3b4-5d1f-5725-b7a3-4b41d20ffdb5
splat_kind: rule
splat_key: dnd2024:glossary/difficult_terrain
splat_name: Difficult Terrain
splat_origin:
  key: dnd2024:reference/glossary/difficult_terrain
splat_sources:
- title: SRD 5.2.1
  locator: pp. 181
  url: https://www.dndbeyond.com/srd
---

# Difficult Terrain

## Source text

<!-- SRD page 181 -->

If a space is Difficult Terrain, every foot of move-\
ment in that space costs 1 extra foot. For example,\
moving 5 feet through Difficult Terrain costs 10 feet\
of movement. Difficult Terrain isn’t cumulative; ei-\
ther a space is Difficult Terrain or it isn’t.\
A space is Difficult Terrain if the space contains\
any of the following or something similar:\
• A creature that isn’t Tiny or your ally\
• Furniture that is sized for creatures of your size or\
larger\
• Heavy snow, ice, rubble, or undergrowth\
• Liquid that’s between shin- and waist-deep\
• A narrow opening sized for a creature one size\
smaller than you\
• A slope of 20 degrees or more\

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

<!-- splat:rows path=splat_rule/parameters/qualifying_any -->

| Row | creature_present | creature_not_tiny | creature_not_ally | furniture_size_at_least | terrain_any | liquid_depth_min | liquid_depth_max | narrow_opening_size_difference | slope_degrees_min |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` true ` | ` true ` | ` true ` |  |  |  |  |  |  |
| ` 1 ` |  |  |  | ` traveler_size ` |  |  |  |  |  |
| ` 2 ` |  |  |  |  | ` ["heavy_snow", "ice", "rubble", "undergrowth"] ` |  |  |  |  |
| ` 3 ` |  |  |  |  |  | ` shin ` | ` waist ` |  |  |
| ` 4 ` |  |  |  |  |  |  |  | ` -1 ` |  |
| ` 5 ` |  |  |  |  |  |  |  |  | ` 20 ` |

#### Similar terrain

<!-- splat:values path=splat_rule/parameters/similar_terrain -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` gm ` |
| ` result_type ` | ` boolean ` |
| ` similar_to_listed_terrain_required ` | ` true ` |
