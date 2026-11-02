---
splat_schema: 1
splat_id: f8d05a63-ed18-577b-858e-18cd212df9df
splat_kind: rule
splat_key: dnd2024:glossary/cone_area_of_effect
splat_name: Cone [Area of Effect]
splat_origin:
  key: dnd2024:reference/glossary/cone_area_of_effect
splat_sources:
- title: SRD 5.2.1
  locator: pp. 179
  url: https://www.dndbeyond.com/srd
---

# Cone [Area of Effect]

## Source text

<!-- SRD page 179 -->

A Cone is an area of effect that extends in straight lines from a point of origin in a direction its creator chooses. A Cone’s width at any point along its length is equal to that point’s distance from the point of origin. For example, a Cone is 15 feet wide at a point along its length that is 15 feet from the point of origin. The effect that creates a Cone specifies its maximum length.

A Cone’s point of origin isn’t included in the area of effect unless its creator decides otherwise.

Original layout: [[../../srd-5.2.1/pages/179|page 179]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/cone_area_of_effect ` |
| ` implementation ` | ` dnd2024:glossary_cone_area_of_effect ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` shape ` | ` cone ` |
| ` propagation ` | ` straight_lines ` |
| ` origin ` | ` point ` |
| ` length ` | ` effect_specified ` |
| ` geometry_provider ` | ` external_geometry ` |

#### Base area rule

<!-- splat:values path=splat_rule/parameters/base_area_rule -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:glossary/area_of_effect ` |

#### Direction

<!-- splat:values path=splat_rule/parameters/direction -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` creator ` |

#### Width at distance

<!-- splat:values path=splat_rule/parameters/width_at_distance -->

| Field | Value |
| --- | --- |
| ` var ` | ` distance_from_origin ` |

#### Origin included

<!-- splat:values path=splat_rule/parameters/origin_included -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` creator ` |
| ` options ` | ` [false, true] ` |
| ` default ` | ` false ` |
