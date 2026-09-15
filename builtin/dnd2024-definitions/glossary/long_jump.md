---
splat_schema: 1
splat_id: 7137802e-4cb8-559f-a7fa-3bf5429c7007
splat_kind: rule
splat_key: dnd2024:glossary/long_jump
splat_name: Long Jump
splat_origin:
  key: dnd2024:reference/glossary/long_jump
splat_sources:
- title: SRD 5.2.1
  locator: pp. 184, 185
  url: https://www.dndbeyond.com/srd
---

# Long Jump

## Source text

<!-- SRD page 184 -->

When you make a Long Jump, you leap horizontally\
a number of feet up to your Strength score if you\

<!-- SRD page 185 -->

move at least 10 feet immediately before the jump.\
When you make a standing Long Jump, you can leap\
only half that distance. Either way, each foot you\
jump costs a foot of movement.\
If you land in Difficult Terrain, you must succeed\
on a DC 10 Dexterity (Acrobatics) check or have the\
Prone condition.\
This Long Jump rule assumes that the height of\
the jump doesn’t matter, such as a jump across a\
stream or chasm. At your GM’s option, you must suc-\
ceed on a DC 10 Strength (Athletics) check to clear a\
low obstacle (no taller than a quarter of the jump’s\
distance), such as a hedge or low wall. Otherwise,\
you hit the obstacle.\

Original layout: [[../../srd-5.2.1/pages/184|page 184]], [[../../srd-5.2.1/pages/185|page 185]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/long_jump ` |
| ` implementation ` | ` dnd2024:glossary_long_jump ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` direction ` | ` horizontal ` |
| ` standing_multiplier ` | ` 0.5 ` |
| ` movement_cost_per_foot ` | ` 1 ` |
| ` default_geometry_assumption ` | ` jump_height_irrelevant ` |

#### Running start

<!-- splat:values path=splat_rule/parameters/running_start -->

| Field | Value |
| --- | --- |
| ` minimum_feet ` | ` 10 ` |
| ` immediately_before ` | ` true ` |

#### Running maximum feet

<!-- splat:values path=splat_rule/parameters/running_maximum_feet -->

| Field | Value |
| --- | --- |
| ` var ` | ` strength_score ` |

#### Difficult terrain landing

<!-- splat:values path=splat_rule/parameters/difficult_terrain_landing -->

| Field | Value |
| --- | --- |
| ` ability ` | ` dexterity ` |
| ` skill ` | ` acrobatics ` |
| ` dc ` | ` 10 ` |
| ` failure_condition ` | ` prone ` |

#### Low obstacle

<!-- splat:yaml path=splat_rule/parameters/low_obstacle -->

```yaml
height_maximum_jump_distance_fraction: 0.25
check_required:
  chooser: gm
  options:
  - true
  - false
ability: strength
skill: athletics
dc: 10
failure: hit_obstacle
```
