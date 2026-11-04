---
splat_schema: 2
splat_id: b48abbf4-267e-521b-9df5-36abfef6f063
splat_kind: rule
splat_key: dnd2024:glossary/cylinder_area_of_effect
splat_name: Cylinder [Area of Effect]
splat_origin: dnd2024:reference/glossary/cylinder_area_of_effect
splat_sources:
- SRD 5.2.1, pp. 180, https://www.dndbeyond.com/srd
---

# Cylinder [Area of Effect]

## Source text

<!-- SRD page 180 -->

A Cylinder is an area of effect that extends in straight lines from a point of origin located at the center of the circular top or bottom of the Cylinder. The effect that creates a Cylinder specifies the radius of the Cylinder’s base and the Cylinder’s height.

A Cylinder’s point of origin is included in the area of effect.

Original layout: [[../../srd-5.2.1/pages/180|page 180]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/cylinder_area_of_effect ` |
| ` implementation ` | ` dnd2024:glossary_cylinder_area_of_effect ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` shape ` | ` cylinder ` |
| ` propagation ` | ` straight_lines ` |
| ` origin ` | ` center_of_top_or_bottom ` |
| ` base_radius ` | ` effect_specified ` |
| ` height ` | ` effect_specified ` |
| ` origin_included ` | ` true ` |
| ` geometry_provider ` | ` external_geometry ` |

#### Base area rule

<!-- splat:values path=splat_rule/parameters/base_area_rule -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:glossary/area_of_effect ` |

#### Origin face

<!-- splat:values path=splat_rule/parameters/origin_face -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` creator ` |
| ` options ` | ` ["top", "bottom"] ` |
