---
splat_schema: 1
splat_id: d5841c57-96b9-5db8-a39b-317142aa3f25
splat_kind: rule
splat_key: dnd2024:glossary/cube_area_of_effect
splat_name: Cube [Area of Effect]
splat_origin:
  key: dnd2024:reference/glossary/cube_area_of_effect
splat_sources:
- title: SRD 5.2.1
  locator: pp. 179
  url: https://www.dndbeyond.com/srd
---

# Cube [Area of Effect]

## Source text

<!-- SRD page 179 -->

A Cube is an area of effect that extends in straight\
lines from a point of origin located anywhere on a\
face of the Cube. The effect that creates a Cube spec-\
ifies its size, which is the length of each side.\
A Cube’s point of origin isn’t included in the area\
of effect unless its creator decides otherwise.\

Original layout: [[../../srd-5.2.1/pages/179|page 179]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/cube_area_of_effect ` |
| ` implementation ` | ` dnd2024:glossary_cube_area_of_effect ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` shape ` | ` cube ` |
| ` propagation ` | ` straight_lines ` |
| ` origin ` | ` anywhere_on_face ` |
| ` side_length ` | ` effect_specified ` |
| ` geometry_provider ` | ` external_geometry ` |

#### Base area rule

<!-- splat:values path=splat_rule/parameters/base_area_rule -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:glossary/area_of_effect ` |

#### Origin included

<!-- splat:values path=splat_rule/parameters/origin_included -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` creator ` |
| ` options ` | ` [false, true] ` |
| ` default ` | ` false ` |
