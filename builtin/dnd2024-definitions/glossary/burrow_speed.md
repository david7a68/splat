---
splat_schema: 1
splat_id: 1eb79bfb-fa78-5a62-8292-7a5776bc557c
splat_kind: rule
splat_key: dnd2024:glossary/burrow_speed
splat_name: Burrow Speed
splat_origin:
  key: dnd2024:reference/glossary/burrow_speed
splat_sources:
- title: SRD 5.2.1
  locator: pp. 178
  url: https://www.dndbeyond.com/srd
---

# Burrow Speed

## Source text

<!-- SRD page 178 -->

A creature that has a Burrow Speed can use that speed to move through sand, earth, mud, or ice. The creature can’t burrow through solid rock unless the creature has a trait that allows it to do so. See also “Speed.”

Original layout: [[../../srd-5.2.1/pages/178|page 178]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/burrow_speed ` |
| ` implementation ` | ` dnd2024:glossary_burrow_speed ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` movement_mode ` | ` burrow ` |
| ` allowed_materials ` | ` ["sand", "earth", "mud", "ice"] ` |
| ` distance_budget ` | ` burrow_speed ` |

#### Solid rock

<!-- splat:values path=splat_rule/parameters/solid_rock -->

| Field | Value |
| --- | --- |
| ` allowed ` | ` false ` |
| ` exception ` | ` trait_allows_rock_burrowing ` |
