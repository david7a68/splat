---
splat_schema: 1
splat_id: 28b395b2-3cf1-51df-b7c9-4a3388036ea6
splat_kind: rule
splat_key: dnd2024:glossary/sphere_area_of_effect
splat_name: Sphere [Area of Effect]
splat_origin:
  key: dnd2024:reference/glossary/sphere_area_of_effect
splat_sources:
- title: SRD 5.2.1
  locator: pp. 188
  url: https://www.dndbeyond.com/srd
---

# Sphere [Area of Effect]

## Source text

<!-- SRD page 188 -->

A Sphere is an area of effect that extends in straight lines from a point of origin outward in all directions. The effect that creates a Sphere specifies the distance it extends as the radius of the Sphere.

A Sphere’s point of origin is included in the Sphere’s area of effect.

Original layout: [[../../srd-5.2.1/pages/188|page 188]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/sphere_area_of_effect ` |
| ` implementation ` | ` dnd2024:glossary_sphere_area_of_effect ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` shape ` | ` sphere ` |
| ` propagation ` | ` straight_lines ` |
| ` origin ` | ` center ` |
| ` direction ` | ` all ` |
| ` radius ` | ` effect_specified ` |
| ` origin_included ` | ` true ` |
| ` geometry_provider ` | ` external_geometry ` |

#### Base area rule

<!-- splat:values path=splat_rule/parameters/base_area_rule -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:glossary/area_of_effect ` |
