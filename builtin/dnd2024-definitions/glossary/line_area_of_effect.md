---
splat_schema: 1
splat_id: 4353cee4-d28f-5fc4-be6b-dcaa7c21305e
splat_kind: rule
splat_key: dnd2024:glossary/line_area_of_effect
splat_name: Line [Area of Effect]
splat_origin:
  key: dnd2024:reference/glossary/line_area_of_effect
splat_sources:
- title: SRD 5.2.1
  locator: pp. 184
  url: https://www.dndbeyond.com/srd
---

# Line [Area of Effect]

## Source text

<!-- SRD page 184 -->

A Line is an area of effect that extends from a point\
of origin in a straight path along its length and cov-\
ers an area defined by its width. The effect that cre-\
ates a Line specifies its length and width.\
A Line’s point of origin isn’t included in the area of\
effect unless its creator decides otherwise.\

Original layout: [[../../srd-5.2.1/pages/184|page 184]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/line_area_of_effect ` |
| ` implementation ` | ` dnd2024:glossary_line_area_of_effect ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` shape ` | ` line ` |
| ` propagation ` | ` straight_lines ` |
| ` origin ` | ` point ` |
| ` path ` | ` straight ` |
| ` length ` | ` effect_specified ` |
| ` width ` | ` effect_specified ` |
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
