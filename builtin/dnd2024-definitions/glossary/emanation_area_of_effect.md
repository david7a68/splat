---
splat_schema: 2
splat_id: e710403e-84d2-569a-9a6f-b30c5277b55c
splat_kind: rule
splat_key: dnd2024:glossary/emanation_area_of_effect
splat_name: Emanation [Area of Effect]
splat_origin: dnd2024:reference/glossary/emanation_area_of_effect
splat_sources:
- SRD 5.2.1, pp. 181, https://www.dndbeyond.com/srd
---

# Emanation [Area of Effect]

## Source text

<!-- SRD page 181 -->

An Emanation is an area of effect that extends in straight lines from a creature or an object in all directions. The effect that creates an Emanation specifies the distance it extends.

An Emanation moves with the creature or object that is its origin unless it is an instantaneous or a stationary effect.

An Emanation’s origin (creature or object) isn’t included in the area of effect unless its creator decides otherwise.

Original layout: [[../../srd-5.2.1/pages/181|page 181]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/emanation_area_of_effect ` |
| ` implementation ` | ` dnd2024:glossary_emanation_area_of_effect ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` shape ` | ` emanation ` |
| ` propagation ` | ` straight_lines ` |
| ` origin_types ` | ` ["creature", "object"] ` |
| ` direction ` | ` all ` |
| ` distance ` | ` effect_specified ` |
| ` moves_with_origin ` | ` true ` |
| ` movement_exceptions ` | ` ["instantaneous", "stationary"] ` |
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
