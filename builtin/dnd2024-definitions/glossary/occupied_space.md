---
splat_schema: 2
splat_id: 1db9676a-f1d1-57d3-a524-129ff6e037ab
splat_kind: rule
splat_key: dnd2024:glossary/occupied_space
splat_name: Occupied Space
splat_origin: dnd2024:reference/glossary/occupied_space
splat_sources:
- SRD 5.2.1, pp. 185, https://www.dndbeyond.com/srd
---

# Occupied Space

## Source text

<!-- SRD page 185 -->

A space is occupied if a creature is in it or if it is completely filled by objects.

Original layout: [[../../srd-5.2.1/pages/185|page 185]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/occupied_space ` |
| ` implementation ` | ` dnd2024:glossary_occupied_space ` |

### Parameters

#### Predicate

<!-- splat:values path=splat_rule/parameters/predicate -->

| Field | Value |
| --- | --- |
| ` any ` | ` ["creature_in_space", "space_completely_filled_by_objects"] ` |
